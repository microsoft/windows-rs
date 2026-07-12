#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_ESCAPE(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const UChar, length: i32, codepoint: UChar32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_FROM_U_CALLBACK_ESCAPE(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const UChar, length : i32, codepoint : UChar32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_FROM_U_CALLBACK_ESCAPE(context, fromuargs as _, codeunits, length, codepoint, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_SKIP(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const UChar, length: i32, codepoint: UChar32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_FROM_U_CALLBACK_SKIP(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const UChar, length : i32, codepoint : UChar32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_FROM_U_CALLBACK_SKIP(context, fromuargs as _, codeunits, length, codepoint, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_STOP(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const UChar, length: i32, codepoint: UChar32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_FROM_U_CALLBACK_STOP(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const UChar, length : i32, codepoint : UChar32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_FROM_U_CALLBACK_STOP(context, fromuargs as _, codeunits, length, codepoint, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_SUBSTITUTE(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const UChar, length: i32, codepoint: UChar32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_FROM_U_CALLBACK_SUBSTITUTE(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const UChar, length : i32, codepoint : UChar32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_FROM_U_CALLBACK_SUBSTITUTE(context, fromuargs as _, codeunits, length, codepoint, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_ESCAPE(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: *const i8, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_TO_U_CALLBACK_ESCAPE(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : *const i8, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_TO_U_CALLBACK_ESCAPE(context, touargs as _, codeunits, length, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_SKIP(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: *const i8, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_TO_U_CALLBACK_SKIP(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : *const i8, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_TO_U_CALLBACK_SKIP(context, touargs as _, codeunits, length, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_STOP(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: *const i8, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_TO_U_CALLBACK_STOP(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : *const i8, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_TO_U_CALLBACK_STOP(context, touargs as _, codeunits, length, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_SUBSTITUTE(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: *const i8, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_TO_U_CALLBACK_SUBSTITUTE(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : *const i8, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_TO_U_CALLBACK_SUBSTITUTE(context, touargs as _, codeunits, length, reason, err as _) }
}
#[inline]
pub unsafe fn u_UCharsToChars(us: *const UChar, cs: *mut i8, length: i32) {
    windows_core::link!("icuuc.dll" "C" fn u_UCharsToChars(us : *const UChar, cs : *mut i8, length : i32));
    unsafe { u_UCharsToChars(us, cs as _, length) }
}
#[inline]
pub unsafe fn u_austrcpy(dst: *mut i8, src: *const UChar) -> *mut i8 {
    windows_core::link!("icuuc.dll" "C" fn u_austrcpy(dst : *mut i8, src : *const UChar) -> *mut i8);
    unsafe { u_austrcpy(dst as _, src) }
}
#[inline]
pub unsafe fn u_austrncpy(dst: *mut i8, src: *const UChar, n: i32) -> *mut i8 {
    windows_core::link!("icuuc.dll" "C" fn u_austrncpy(dst : *mut i8, src : *const UChar, n : i32) -> *mut i8);
    unsafe { u_austrncpy(dst as _, src, n) }
}
#[inline]
pub unsafe fn u_catclose() -> UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn u_catclose(catd : *mut UResourceBundle));
    unsafe {
        let mut result__ = core::mem::zeroed();
        u_catclose(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn u_catgets(catd: *mut UResourceBundle, set_num: i32, msg_num: i32, s: *const UChar, len: *mut i32, ec: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuuc.dll" "C" fn u_catgets(catd : *mut UResourceBundle, set_num : i32, msg_num : i32, s : *const UChar, len : *mut i32, ec : *mut UErrorCode) -> *const UChar);
    unsafe { u_catgets(catd as _, set_num, msg_num, s, len as _, ec as _) }
}
#[inline]
pub unsafe fn u_catopen(name: *const i8, locale: *const i8, ec: *mut UErrorCode) -> u_nl_catd {
    windows_core::link!("icuuc.dll" "C" fn u_catopen(name : *const i8, locale : *const i8, ec : *mut UErrorCode) -> u_nl_catd);
    unsafe { u_catopen(name, locale, ec as _) }
}
#[inline]
pub unsafe fn u_charAge(c: UChar32) -> u8 {
    windows_core::link!("icuuc.dll" "C" fn u_charAge(c : UChar32, versionarray : *mut u8));
    unsafe {
        let mut result__ = core::mem::zeroed();
        u_charAge(c, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn u_charDigitValue(c: UChar32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_charDigitValue(c : UChar32) -> i32);
    unsafe { u_charDigitValue(c) }
}
#[inline]
pub unsafe fn u_charDirection(c: UChar32) -> UCharDirection {
    windows_core::link!("icuuc.dll" "C" fn u_charDirection(c : UChar32) -> UCharDirection);
    unsafe { u_charDirection(c) }
}
#[inline]
pub unsafe fn u_charFromName(namechoice: UCharNameChoice, name: *const i8, perrorcode: *mut UErrorCode) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_charFromName(namechoice : UCharNameChoice, name : *const i8, perrorcode : *mut UErrorCode) -> UChar32);
    unsafe { u_charFromName(namechoice, name, perrorcode as _) }
}
#[inline]
pub unsafe fn u_charMirror(c: UChar32) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_charMirror(c : UChar32) -> UChar32);
    unsafe { u_charMirror(c) }
}
#[inline]
pub unsafe fn u_charName(code: UChar32, namechoice: UCharNameChoice, buffer: *mut i8, bufferlength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_charName(code : UChar32, namechoice : UCharNameChoice, buffer : *mut i8, bufferlength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_charName(code, namechoice, buffer as _, bufferlength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_charType(c: UChar32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_charType(c : UChar32) -> i8);
    unsafe { u_charType(c) }
}
#[inline]
pub unsafe fn u_charsToUChars(cs: *const i8, us: *mut UChar, length: i32) {
    windows_core::link!("icuuc.dll" "C" fn u_charsToUChars(cs : *const i8, us : *mut UChar, length : i32));
    unsafe { u_charsToUChars(cs, us as _, length) }
}
#[inline]
pub unsafe fn u_cleanup() {
    windows_core::link!("icuuc.dll" "C" fn u_cleanup());
    unsafe { u_cleanup() }
}
#[inline]
pub unsafe fn u_countChar32(s: *const UChar, length: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_countChar32(s : *const UChar, length : i32) -> i32);
    unsafe { u_countChar32(s, length) }
}
#[inline]
pub unsafe fn u_digit(ch: UChar32, radix: i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_digit(ch : UChar32, radix : i8) -> i32);
    unsafe { u_digit(ch, radix) }
}
#[inline]
pub unsafe fn u_enumCharNames(start: UChar32, limit: UChar32, r#fn: UEnumCharNamesFn, context: *mut core::ffi::c_void, namechoice: UCharNameChoice, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn u_enumCharNames(start : UChar32, limit : UChar32, r#fn : UEnumCharNamesFn, context : *mut core::ffi::c_void, namechoice : UCharNameChoice, perrorcode : *mut UErrorCode));
    unsafe { u_enumCharNames(start, limit, r#fn, context as _, namechoice, perrorcode as _) }
}
#[inline]
pub unsafe fn u_enumCharTypes(enumrange: UCharEnumTypeRange, context: *const core::ffi::c_void) {
    windows_core::link!("icuuc.dll" "C" fn u_enumCharTypes(enumrange : UCharEnumTypeRange, context : *const core::ffi::c_void));
    unsafe { u_enumCharTypes(enumrange, context) }
}
#[inline]
pub unsafe fn u_errorName(code: UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn u_errorName(code : UErrorCode) -> *const i8);
    unsafe { u_errorName(code) }
}
#[inline]
pub unsafe fn u_foldCase(c: UChar32, options: u32) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_foldCase(c : UChar32, options : u32) -> UChar32);
    unsafe { u_foldCase(c, options) }
}
#[inline]
pub unsafe fn u_forDigit(digit: i32, radix: i8) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_forDigit(digit : i32, radix : i8) -> UChar32);
    unsafe { u_forDigit(digit, radix) }
}
#[inline]
pub unsafe fn u_formatMessage(locale: *const i8, pattern: *const UChar, patternlength: i32, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn u_formatMessage(locale : *const i8, pattern : *const UChar, patternlength : i32, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { u_formatMessage(locale, pattern, patternlength, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn u_formatMessageWithError(locale: *const i8, pattern: *const UChar, patternlength: i32, result: *mut UChar, resultlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn u_formatMessageWithError(locale : *const i8, pattern : *const UChar, patternlength : i32, result : *mut UChar, resultlength : i32, parseerror : *mut UParseError, status : *mut UErrorCode) -> i32);
    unsafe { u_formatMessageWithError(locale, pattern, patternlength, result as _, resultlength, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn u_getBidiPairedBracket(c: UChar32) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_getBidiPairedBracket(c : UChar32) -> UChar32);
    unsafe { u_getBidiPairedBracket(c) }
}
#[inline]
pub unsafe fn u_getBinaryPropertySet(property: UProperty, perrorcode: *mut UErrorCode) -> *const USet {
    windows_core::link!("icu.dll" "C" fn u_getBinaryPropertySet(property : UProperty, perrorcode : *mut UErrorCode) -> *const USet);
    unsafe { u_getBinaryPropertySet(property, perrorcode as _) }
}
#[inline]
pub unsafe fn u_getCombiningClass(c: UChar32) -> u8 {
    windows_core::link!("icuuc.dll" "C" fn u_getCombiningClass(c : UChar32) -> u8);
    unsafe { u_getCombiningClass(c) }
}
#[inline]
pub unsafe fn u_getDataVersion(dataversionfillin: *mut u8, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn u_getDataVersion(dataversionfillin : *mut u8, status : *mut UErrorCode));
    unsafe { u_getDataVersion(dataversionfillin as _, status as _) }
}
#[inline]
pub unsafe fn u_getFC_NFKC_Closure(c: UChar32, dest: *mut UChar, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_getFC_NFKC_Closure(c : UChar32, dest : *mut UChar, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_getFC_NFKC_Closure(c, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn u_getIntPropertyMap(property: UProperty, perrorcode: *mut UErrorCode) -> *const UCPMap {
    windows_core::link!("icu.dll" "C" fn u_getIntPropertyMap(property : UProperty, perrorcode : *mut UErrorCode) -> *const UCPMap);
    unsafe { u_getIntPropertyMap(property, perrorcode as _) }
}
#[inline]
pub unsafe fn u_getIntPropertyMaxValue(which: UProperty) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_getIntPropertyMaxValue(which : UProperty) -> i32);
    unsafe { u_getIntPropertyMaxValue(which) }
}
#[inline]
pub unsafe fn u_getIntPropertyMinValue(which: UProperty) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_getIntPropertyMinValue(which : UProperty) -> i32);
    unsafe { u_getIntPropertyMinValue(which) }
}
#[inline]
pub unsafe fn u_getIntPropertyValue(c: UChar32, which: UProperty) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_getIntPropertyValue(c : UChar32, which : UProperty) -> i32);
    unsafe { u_getIntPropertyValue(c, which) }
}
#[inline]
pub unsafe fn u_getNumericValue(c: UChar32) -> f64 {
    windows_core::link!("icuuc.dll" "C" fn u_getNumericValue(c : UChar32) -> f64);
    unsafe { u_getNumericValue(c) }
}
#[inline]
pub unsafe fn u_getPropertyEnum(alias: *const i8) -> UProperty {
    windows_core::link!("icuuc.dll" "C" fn u_getPropertyEnum(alias : *const i8) -> UProperty);
    unsafe { u_getPropertyEnum(alias) }
}
#[inline]
pub unsafe fn u_getPropertyName(property: UProperty, namechoice: UPropertyNameChoice) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn u_getPropertyName(property : UProperty, namechoice : UPropertyNameChoice) -> *const i8);
    unsafe { u_getPropertyName(property, namechoice) }
}
#[inline]
pub unsafe fn u_getPropertyValueEnum(property: UProperty, alias: *const i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_getPropertyValueEnum(property : UProperty, alias : *const i8) -> i32);
    unsafe { u_getPropertyValueEnum(property, alias) }
}
#[inline]
pub unsafe fn u_getPropertyValueName(property: UProperty, value: i32, namechoice: UPropertyNameChoice) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn u_getPropertyValueName(property : UProperty, value : i32, namechoice : UPropertyNameChoice) -> *const i8);
    unsafe { u_getPropertyValueName(property, value, namechoice) }
}
#[inline]
pub unsafe fn u_getUnicodeVersion() -> u8 {
    windows_core::link!("icuuc.dll" "C" fn u_getUnicodeVersion(versionarray : *mut u8));
    unsafe {
        let mut result__ = core::mem::zeroed();
        u_getUnicodeVersion(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn u_getVersion() -> u8 {
    windows_core::link!("icuuc.dll" "C" fn u_getVersion(versionarray : *mut u8));
    unsafe {
        let mut result__ = core::mem::zeroed();
        u_getVersion(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn u_hasBinaryProperty(c: UChar32, which: UProperty) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_hasBinaryProperty(c : UChar32, which : UProperty) -> UBool);
    unsafe { u_hasBinaryProperty(c, which) }
}
#[inline]
pub unsafe fn u_init() -> UErrorCode {
    windows_core::link!("icuuc.dll" "C" fn u_init(status : *mut UErrorCode));
    unsafe {
        let mut result__ = core::mem::zeroed();
        u_init(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn u_isIDIgnorable(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isIDIgnorable(c : UChar32) -> UBool);
    unsafe { u_isIDIgnorable(c) }
}
#[inline]
pub unsafe fn u_isIDPart(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isIDPart(c : UChar32) -> UBool);
    unsafe { u_isIDPart(c) }
}
#[inline]
pub unsafe fn u_isIDStart(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isIDStart(c : UChar32) -> UBool);
    unsafe { u_isIDStart(c) }
}
#[inline]
pub unsafe fn u_isISOControl(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isISOControl(c : UChar32) -> UBool);
    unsafe { u_isISOControl(c) }
}
#[inline]
pub unsafe fn u_isJavaIDPart(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isJavaIDPart(c : UChar32) -> UBool);
    unsafe { u_isJavaIDPart(c) }
}
#[inline]
pub unsafe fn u_isJavaIDStart(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isJavaIDStart(c : UChar32) -> UBool);
    unsafe { u_isJavaIDStart(c) }
}
#[inline]
pub unsafe fn u_isJavaSpaceChar(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isJavaSpaceChar(c : UChar32) -> UBool);
    unsafe { u_isJavaSpaceChar(c) }
}
#[inline]
pub unsafe fn u_isMirrored(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isMirrored(c : UChar32) -> UBool);
    unsafe { u_isMirrored(c) }
}
#[inline]
pub unsafe fn u_isUAlphabetic(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isUAlphabetic(c : UChar32) -> UBool);
    unsafe { u_isUAlphabetic(c) }
}
#[inline]
pub unsafe fn u_isULowercase(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isULowercase(c : UChar32) -> UBool);
    unsafe { u_isULowercase(c) }
}
#[inline]
pub unsafe fn u_isUUppercase(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isUUppercase(c : UChar32) -> UBool);
    unsafe { u_isUUppercase(c) }
}
#[inline]
pub unsafe fn u_isUWhiteSpace(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isUWhiteSpace(c : UChar32) -> UBool);
    unsafe { u_isUWhiteSpace(c) }
}
#[inline]
pub unsafe fn u_isWhitespace(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isWhitespace(c : UChar32) -> UBool);
    unsafe { u_isWhitespace(c) }
}
#[inline]
pub unsafe fn u_isalnum(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isalnum(c : UChar32) -> UBool);
    unsafe { u_isalnum(c) }
}
#[inline]
pub unsafe fn u_isalpha(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isalpha(c : UChar32) -> UBool);
    unsafe { u_isalpha(c) }
}
#[inline]
pub unsafe fn u_isbase(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isbase(c : UChar32) -> UBool);
    unsafe { u_isbase(c) }
}
#[inline]
pub unsafe fn u_isblank(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isblank(c : UChar32) -> UBool);
    unsafe { u_isblank(c) }
}
#[inline]
pub unsafe fn u_iscntrl(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_iscntrl(c : UChar32) -> UBool);
    unsafe { u_iscntrl(c) }
}
#[inline]
pub unsafe fn u_isdefined(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isdefined(c : UChar32) -> UBool);
    unsafe { u_isdefined(c) }
}
#[inline]
pub unsafe fn u_isdigit(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isdigit(c : UChar32) -> UBool);
    unsafe { u_isdigit(c) }
}
#[inline]
pub unsafe fn u_isgraph(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isgraph(c : UChar32) -> UBool);
    unsafe { u_isgraph(c) }
}
#[inline]
pub unsafe fn u_islower(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_islower(c : UChar32) -> UBool);
    unsafe { u_islower(c) }
}
#[inline]
pub unsafe fn u_isprint(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isprint(c : UChar32) -> UBool);
    unsafe { u_isprint(c) }
}
#[inline]
pub unsafe fn u_ispunct(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_ispunct(c : UChar32) -> UBool);
    unsafe { u_ispunct(c) }
}
#[inline]
pub unsafe fn u_isspace(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isspace(c : UChar32) -> UBool);
    unsafe { u_isspace(c) }
}
#[inline]
pub unsafe fn u_istitle(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_istitle(c : UChar32) -> UBool);
    unsafe { u_istitle(c) }
}
#[inline]
pub unsafe fn u_isupper(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isupper(c : UChar32) -> UBool);
    unsafe { u_isupper(c) }
}
#[inline]
pub unsafe fn u_isxdigit(c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_isxdigit(c : UChar32) -> UBool);
    unsafe { u_isxdigit(c) }
}
#[inline]
pub unsafe fn u_memcasecmp(s1: *const UChar, s2: *const UChar, length: i32, options: u32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_memcasecmp(s1 : *const UChar, s2 : *const UChar, length : i32, options : u32) -> i32);
    unsafe { u_memcasecmp(s1, s2, length, options) }
}
#[inline]
pub unsafe fn u_memchr(s: *const UChar, c: UChar, count: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_memchr(s : *const UChar, c : UChar, count : i32) -> *mut UChar);
    unsafe { u_memchr(s, c, count) }
}
#[inline]
pub unsafe fn u_memchr32(s: *const UChar, c: UChar32, count: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_memchr32(s : *const UChar, c : UChar32, count : i32) -> *mut UChar);
    unsafe { u_memchr32(s, c, count) }
}
#[inline]
pub unsafe fn u_memcmp(buf1: *const UChar, buf2: *const UChar, count: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_memcmp(buf1 : *const UChar, buf2 : *const UChar, count : i32) -> i32);
    unsafe { u_memcmp(buf1, buf2, count) }
}
#[inline]
pub unsafe fn u_memcmpCodePointOrder(s1: *const UChar, s2: *const UChar, count: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_memcmpCodePointOrder(s1 : *const UChar, s2 : *const UChar, count : i32) -> i32);
    unsafe { u_memcmpCodePointOrder(s1, s2, count) }
}
#[inline]
pub unsafe fn u_memcpy(dest: *mut UChar, src: *const UChar, count: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_memcpy(dest : *mut UChar, src : *const UChar, count : i32) -> *mut UChar);
    unsafe { u_memcpy(dest as _, src, count) }
}
#[inline]
pub unsafe fn u_memmove(dest: *mut UChar, src: *const UChar, count: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_memmove(dest : *mut UChar, src : *const UChar, count : i32) -> *mut UChar);
    unsafe { u_memmove(dest as _, src, count) }
}
#[inline]
pub unsafe fn u_memrchr(s: *const UChar, c: UChar, count: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_memrchr(s : *const UChar, c : UChar, count : i32) -> *mut UChar);
    unsafe { u_memrchr(s, c, count) }
}
#[inline]
pub unsafe fn u_memrchr32(s: *const UChar, c: UChar32, count: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_memrchr32(s : *const UChar, c : UChar32, count : i32) -> *mut UChar);
    unsafe { u_memrchr32(s, c, count) }
}
#[inline]
pub unsafe fn u_memset(dest: *mut UChar, c: UChar, count: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_memset(dest : *mut UChar, c : UChar, count : i32) -> *mut UChar);
    unsafe { u_memset(dest as _, c, count) }
}
#[inline]
pub unsafe fn u_parseMessage(locale: *const i8, pattern: *const UChar, patternlength: i32, source: *const UChar, sourcelength: i32) -> UErrorCode {
    windows_core::link!("icuin.dll" "C" fn u_parseMessage(locale : *const i8, pattern : *const UChar, patternlength : i32, source : *const UChar, sourcelength : i32, status : *mut UErrorCode));
    unsafe {
        let mut result__ = core::mem::zeroed();
        u_parseMessage(locale, pattern, patternlength, source, sourcelength, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn u_parseMessageWithError(locale: *const i8, pattern: *const UChar, patternlength: i32, source: *const UChar, sourcelength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn u_parseMessageWithError(locale : *const i8, pattern : *const UChar, patternlength : i32, source : *const UChar, sourcelength : i32, parseerror : *mut UParseError, status : *mut UErrorCode));
    unsafe { u_parseMessageWithError(locale, pattern, patternlength, source, sourcelength, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn u_setMemoryFunctions(context: *const core::ffi::c_void, a: UMemAllocFn, r: UMemReallocFn, f: UMemFreeFn) -> UErrorCode {
    windows_core::link!("icuuc.dll" "C" fn u_setMemoryFunctions(context : *const core::ffi::c_void, a : UMemAllocFn, r : UMemReallocFn, f : UMemFreeFn, status : *mut UErrorCode));
    unsafe {
        let mut result__ = core::mem::zeroed();
        u_setMemoryFunctions(context, a, r, f, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn u_shapeArabic(source: *const UChar, sourcelength: i32, dest: *mut UChar, destsize: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_shapeArabic(source : *const UChar, sourcelength : i32, dest : *mut UChar, destsize : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_shapeArabic(source, sourcelength, dest as _, destsize, options, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strCaseCompare(s1: *const UChar, length1: i32, s2: *const UChar, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strCaseCompare(s1 : *const UChar, length1 : i32, s2 : *const UChar, length2 : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_strCaseCompare(s1, length1, s2, length2, options, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strCompare(s1: *const UChar, length1: i32, s2: *const UChar, length2: i32, codepointorder: UBool) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strCompare(s1 : *const UChar, length1 : i32, s2 : *const UChar, length2 : i32, codepointorder : UBool) -> i32);
    unsafe { u_strCompare(s1, length1, s2, length2, codepointorder) }
}
#[inline]
pub unsafe fn u_strCompareIter(iter1: *mut UCharIterator, iter2: *mut UCharIterator, codepointorder: UBool) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strCompareIter(iter1 : *mut UCharIterator, iter2 : *mut UCharIterator, codepointorder : UBool) -> i32);
    unsafe { u_strCompareIter(iter1 as _, iter2 as _, codepointorder) }
}
#[inline]
pub unsafe fn u_strFindFirst(s: *const UChar, length: i32, substring: *const UChar, sublength: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strFindFirst(s : *const UChar, length : i32, substring : *const UChar, sublength : i32) -> *mut UChar);
    unsafe { u_strFindFirst(s, length, substring, sublength) }
}
#[inline]
pub unsafe fn u_strFindLast(s: *const UChar, length: i32, substring: *const UChar, sublength: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strFindLast(s : *const UChar, length : i32, substring : *const UChar, sublength : i32) -> *mut UChar);
    unsafe { u_strFindLast(s, length, substring, sublength) }
}
#[inline]
pub unsafe fn u_strFoldCase(dest: *mut UChar, destcapacity: i32, src: *const UChar, srclength: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strFoldCase(dest : *mut UChar, destcapacity : i32, src : *const UChar, srclength : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_strFoldCase(dest as _, destcapacity, src, srclength, options, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromJavaModifiedUTF8WithSub(dest: *mut UChar, destcapacity: i32, pdestlength: *mut i32, src: *const i8, srclength: i32, subchar: UChar32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strFromJavaModifiedUTF8WithSub(dest : *mut UChar, destcapacity : i32, pdestlength : *mut i32, src : *const i8, srclength : i32, subchar : UChar32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut UChar);
    unsafe { u_strFromJavaModifiedUTF8WithSub(dest as _, destcapacity, pdestlength as _, src, srclength, subchar, pnumsubstitutions as _, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromUTF32(dest: *mut UChar, destcapacity: i32, pdestlength: *mut i32, src: *const UChar32, srclength: i32, perrorcode: *mut UErrorCode) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strFromUTF32(dest : *mut UChar, destcapacity : i32, pdestlength : *mut i32, src : *const UChar32, srclength : i32, perrorcode : *mut UErrorCode) -> *mut UChar);
    unsafe { u_strFromUTF32(dest as _, destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromUTF32WithSub(dest: *mut UChar, destcapacity: i32, pdestlength: *mut i32, src: *const UChar32, srclength: i32, subchar: UChar32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strFromUTF32WithSub(dest : *mut UChar, destcapacity : i32, pdestlength : *mut i32, src : *const UChar32, srclength : i32, subchar : UChar32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut UChar);
    unsafe { u_strFromUTF32WithSub(dest as _, destcapacity, pdestlength as _, src, srclength, subchar, pnumsubstitutions as _, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromUTF8(dest: *mut UChar, destcapacity: i32, pdestlength: *mut i32, src: *const i8, srclength: i32, perrorcode: *mut UErrorCode) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strFromUTF8(dest : *mut UChar, destcapacity : i32, pdestlength : *mut i32, src : *const i8, srclength : i32, perrorcode : *mut UErrorCode) -> *mut UChar);
    unsafe { u_strFromUTF8(dest as _, destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromUTF8Lenient(dest: *mut UChar, destcapacity: i32, pdestlength: *mut i32, src: *const i8, srclength: i32, perrorcode: *mut UErrorCode) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strFromUTF8Lenient(dest : *mut UChar, destcapacity : i32, pdestlength : *mut i32, src : *const i8, srclength : i32, perrorcode : *mut UErrorCode) -> *mut UChar);
    unsafe { u_strFromUTF8Lenient(dest as _, destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromUTF8WithSub(dest: *mut UChar, destcapacity: i32, pdestlength: *mut i32, src: *const i8, srclength: i32, subchar: UChar32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strFromUTF8WithSub(dest : *mut UChar, destcapacity : i32, pdestlength : *mut i32, src : *const i8, srclength : i32, subchar : UChar32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut UChar);
    unsafe { u_strFromUTF8WithSub(dest as _, destcapacity, pdestlength as _, src, srclength, subchar, pnumsubstitutions as _, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromWCS(dest: *mut UChar, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strFromWCS(dest : *mut UChar, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> *mut UChar);
    unsafe { u_strFromWCS(dest as _, destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strHasMoreChar32Than(s: *const UChar, length: i32, number: i32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn u_strHasMoreChar32Than(s : *const UChar, length : i32, number : i32) -> UBool);
    unsafe { u_strHasMoreChar32Than(s, length, number) }
}
#[inline]
pub unsafe fn u_strToJavaModifiedUTF8(dest: *mut i8, destcapacity: i32, pdestlength: *mut i32, src: *const UChar, srclength: i32, perrorcode: *mut UErrorCode) -> *mut i8 {
    windows_core::link!("icuuc.dll" "C" fn u_strToJavaModifiedUTF8(dest : *mut i8, destcapacity : i32, pdestlength : *mut i32, src : *const UChar, srclength : i32, perrorcode : *mut UErrorCode) -> *mut i8);
    unsafe { u_strToJavaModifiedUTF8(dest as _, destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToLower(dest: *mut UChar, destcapacity: i32, src: *const UChar, srclength: i32, locale: *const i8, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strToLower(dest : *mut UChar, destcapacity : i32, src : *const UChar, srclength : i32, locale : *const i8, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_strToLower(dest as _, destcapacity, src, srclength, locale, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToTitle(dest: *mut UChar, destcapacity: i32, src: *const UChar, srclength: i32, titleiter: *mut UBreakIterator, locale: *const i8, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strToTitle(dest : *mut UChar, destcapacity : i32, src : *const UChar, srclength : i32, titleiter : *mut UBreakIterator, locale : *const i8, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_strToTitle(dest as _, destcapacity, src, srclength, titleiter as _, locale, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToUTF32(dest: *mut UChar32, destcapacity: i32, pdestlength: *mut i32, src: *const UChar, srclength: i32, perrorcode: *mut UErrorCode) -> *mut UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_strToUTF32(dest : *mut UChar32, destcapacity : i32, pdestlength : *mut i32, src : *const UChar, srclength : i32, perrorcode : *mut UErrorCode) -> *mut UChar32);
    unsafe { u_strToUTF32(dest as _, destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToUTF32WithSub(dest: *mut UChar32, destcapacity: i32, pdestlength: *mut i32, src: *const UChar, srclength: i32, subchar: UChar32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_strToUTF32WithSub(dest : *mut UChar32, destcapacity : i32, pdestlength : *mut i32, src : *const UChar, srclength : i32, subchar : UChar32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut UChar32);
    unsafe { u_strToUTF32WithSub(dest as _, destcapacity, pdestlength as _, src, srclength, subchar, pnumsubstitutions as _, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToUTF8(dest: *mut i8, destcapacity: i32, pdestlength: *mut i32, src: *const UChar, srclength: i32, perrorcode: *mut UErrorCode) -> *mut i8 {
    windows_core::link!("icuuc.dll" "C" fn u_strToUTF8(dest : *mut i8, destcapacity : i32, pdestlength : *mut i32, src : *const UChar, srclength : i32, perrorcode : *mut UErrorCode) -> *mut i8);
    unsafe { u_strToUTF8(dest as _, destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToUTF8WithSub(dest: *mut i8, destcapacity: i32, pdestlength: *mut i32, src: *const UChar, srclength: i32, subchar: UChar32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut i8 {
    windows_core::link!("icuuc.dll" "C" fn u_strToUTF8WithSub(dest : *mut i8, destcapacity : i32, pdestlength : *mut i32, src : *const UChar, srclength : i32, subchar : UChar32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut i8);
    unsafe { u_strToUTF8WithSub(dest as _, destcapacity, pdestlength as _, src, srclength, subchar, pnumsubstitutions as _, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToUpper(dest: *mut UChar, destcapacity: i32, src: *const UChar, srclength: i32, locale: *const i8, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strToUpper(dest : *mut UChar, destcapacity : i32, src : *const UChar, srclength : i32, locale : *const i8, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_strToUpper(dest as _, destcapacity, src, srclength, locale, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToWCS(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const UChar, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strToWCS(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : *const UChar, srclength : i32, perrorcode : *mut UErrorCode) -> *mut u16);
    unsafe { u_strToWCS(dest as _, destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strcasecmp(s1: *const UChar, s2: *const UChar, options: u32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strcasecmp(s1 : *const UChar, s2 : *const UChar, options : u32) -> i32);
    unsafe { u_strcasecmp(s1, s2, options) }
}
#[inline]
pub unsafe fn u_strcat(dst: *mut UChar, src: *const UChar) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strcat(dst : *mut UChar, src : *const UChar) -> *mut UChar);
    unsafe { u_strcat(dst as _, src) }
}
#[inline]
pub unsafe fn u_strchr(s: *const UChar, c: UChar) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strchr(s : *const UChar, c : UChar) -> *mut UChar);
    unsafe { u_strchr(s, c) }
}
#[inline]
pub unsafe fn u_strchr32(s: *const UChar, c: UChar32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strchr32(s : *const UChar, c : UChar32) -> *mut UChar);
    unsafe { u_strchr32(s, c) }
}
#[inline]
pub unsafe fn u_strcmp(s1: *const UChar, s2: *const UChar) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strcmp(s1 : *const UChar, s2 : *const UChar) -> i32);
    unsafe { u_strcmp(s1, s2) }
}
#[inline]
pub unsafe fn u_strcmpCodePointOrder(s1: *const UChar, s2: *const UChar) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strcmpCodePointOrder(s1 : *const UChar, s2 : *const UChar) -> i32);
    unsafe { u_strcmpCodePointOrder(s1, s2) }
}
#[inline]
pub unsafe fn u_strcpy(dst: *mut UChar, src: *const UChar) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strcpy(dst : *mut UChar, src : *const UChar) -> *mut UChar);
    unsafe { u_strcpy(dst as _, src) }
}
#[inline]
pub unsafe fn u_strcspn(string: *const UChar, matchset: *const UChar) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strcspn(string : *const UChar, matchset : *const UChar) -> i32);
    unsafe { u_strcspn(string, matchset) }
}
#[inline]
pub unsafe fn u_stringHasBinaryProperty(s: *const UChar, length: i32, which: UProperty) -> UBool {
    windows_core::link!("icu.dll" "C" fn u_stringHasBinaryProperty(s : *const UChar, length : i32, which : UProperty) -> UBool);
    unsafe { u_stringHasBinaryProperty(s, length, which) }
}
#[inline]
pub unsafe fn u_strlen(s: *const UChar) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strlen(s : *const UChar) -> i32);
    unsafe { u_strlen(s) }
}
#[inline]
pub unsafe fn u_strncasecmp(s1: *const UChar, s2: *const UChar, n: i32, options: u32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strncasecmp(s1 : *const UChar, s2 : *const UChar, n : i32, options : u32) -> i32);
    unsafe { u_strncasecmp(s1, s2, n, options) }
}
#[inline]
pub unsafe fn u_strncat(dst: *mut UChar, src: *const UChar, n: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strncat(dst : *mut UChar, src : *const UChar, n : i32) -> *mut UChar);
    unsafe { u_strncat(dst as _, src, n) }
}
#[inline]
pub unsafe fn u_strncmp(ucs1: *const UChar, ucs2: *const UChar, n: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strncmp(ucs1 : *const UChar, ucs2 : *const UChar, n : i32) -> i32);
    unsafe { u_strncmp(ucs1, ucs2, n) }
}
#[inline]
pub unsafe fn u_strncmpCodePointOrder(s1: *const UChar, s2: *const UChar, n: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strncmpCodePointOrder(s1 : *const UChar, s2 : *const UChar, n : i32) -> i32);
    unsafe { u_strncmpCodePointOrder(s1, s2, n) }
}
#[inline]
pub unsafe fn u_strncpy(dst: *mut UChar, src: *const UChar, n: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strncpy(dst : *mut UChar, src : *const UChar, n : i32) -> *mut UChar);
    unsafe { u_strncpy(dst as _, src, n) }
}
#[inline]
pub unsafe fn u_strpbrk(string: *const UChar, matchset: *const UChar) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strpbrk(string : *const UChar, matchset : *const UChar) -> *mut UChar);
    unsafe { u_strpbrk(string, matchset) }
}
#[inline]
pub unsafe fn u_strrchr(s: *const UChar, c: UChar) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strrchr(s : *const UChar, c : UChar) -> *mut UChar);
    unsafe { u_strrchr(s, c) }
}
#[inline]
pub unsafe fn u_strrchr32(s: *const UChar, c: UChar32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strrchr32(s : *const UChar, c : UChar32) -> *mut UChar);
    unsafe { u_strrchr32(s, c) }
}
#[inline]
pub unsafe fn u_strrstr(s: *const UChar, substring: *const UChar) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strrstr(s : *const UChar, substring : *const UChar) -> *mut UChar);
    unsafe { u_strrstr(s, substring) }
}
#[inline]
pub unsafe fn u_strspn(string: *const UChar, matchset: *const UChar) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strspn(string : *const UChar, matchset : *const UChar) -> i32);
    unsafe { u_strspn(string, matchset) }
}
#[inline]
pub unsafe fn u_strstr(s: *const UChar, substring: *const UChar) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strstr(s : *const UChar, substring : *const UChar) -> *mut UChar);
    unsafe { u_strstr(s, substring) }
}
#[inline]
pub unsafe fn u_strtok_r(src: *mut UChar, delim: *const UChar, savestate: *mut *mut UChar) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_strtok_r(src : *mut UChar, delim : *const UChar, savestate : *mut *mut UChar) -> *mut UChar);
    unsafe { u_strtok_r(src as _, delim, savestate as _) }
}
#[inline]
pub unsafe fn u_tolower(c: UChar32) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_tolower(c : UChar32) -> UChar32);
    unsafe { u_tolower(c) }
}
#[inline]
pub unsafe fn u_totitle(c: UChar32) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_totitle(c : UChar32) -> UChar32);
    unsafe { u_totitle(c) }
}
#[inline]
pub unsafe fn u_toupper(c: UChar32) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_toupper(c : UChar32) -> UChar32);
    unsafe { u_toupper(c) }
}
#[inline]
pub unsafe fn u_uastrcpy(dst: *mut UChar, src: *const i8) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_uastrcpy(dst : *mut UChar, src : *const i8) -> *mut UChar);
    unsafe { u_uastrcpy(dst as _, src) }
}
#[inline]
pub unsafe fn u_uastrncpy(dst: *mut UChar, src: *const i8, n: i32) -> *mut UChar {
    windows_core::link!("icuuc.dll" "C" fn u_uastrncpy(dst : *mut UChar, src : *const i8, n : i32) -> *mut UChar);
    unsafe { u_uastrncpy(dst as _, src, n) }
}
#[inline]
pub unsafe fn u_unescape(src: *const i8, dest: *mut UChar, destcapacity: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_unescape(src : *const i8, dest : *mut UChar, destcapacity : i32) -> i32);
    unsafe { u_unescape(src, dest as _, destcapacity) }
}
#[inline]
pub unsafe fn u_unescapeAt(charat: UNESCAPE_CHAR_AT, offset: *mut i32, length: i32, context: *mut core::ffi::c_void) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn u_unescapeAt(charat : UNESCAPE_CHAR_AT, offset : *mut i32, length : i32, context : *mut core::ffi::c_void) -> UChar32);
    unsafe { u_unescapeAt(charat, offset as _, length, context as _) }
}
#[inline]
pub unsafe fn u_versionFromString(versionarray: *mut u8, versionstring: *const i8) {
    windows_core::link!("icuuc.dll" "C" fn u_versionFromString(versionarray : *mut u8, versionstring : *const i8));
    unsafe { u_versionFromString(versionarray as _, versionstring) }
}
#[inline]
pub unsafe fn u_versionFromUString(versionarray: *mut u8, versionstring: *const UChar) {
    windows_core::link!("icuuc.dll" "C" fn u_versionFromUString(versionarray : *mut u8, versionstring : *const UChar));
    unsafe { u_versionFromUString(versionarray as _, versionstring) }
}
#[inline]
pub unsafe fn u_versionToString(versionarray: *mut u8, versionstring: *mut i8) {
    windows_core::link!("icuuc.dll" "C" fn u_versionToString(versionarray : *mut u8, versionstring : *mut i8));
    unsafe { u_versionToString(versionarray as _, versionstring as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn u_vformatMessage(locale: *const i8, pattern: *const UChar, patternlength: i32, result: *mut UChar, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn u_vformatMessage(locale : *const i8, pattern : *const UChar, patternlength : i32, result : *mut UChar, resultlength : i32, ap : *mut i8, status : *mut UErrorCode) -> i32);
    unsafe { u_vformatMessage(locale, pattern, patternlength, result as _, resultlength, ap as _, status as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn u_vformatMessage(locale: *const i8, pattern: *const UChar, patternlength: i32, result: *mut UChar, resultlength: i32, ap: super::vadefs::va_list, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn u_vformatMessage(locale : *const i8, pattern : *const UChar, patternlength : i32, result : *mut UChar, resultlength : i32, ap : super::vadefs::va_list, status : *mut UErrorCode) -> i32);
    unsafe { u_vformatMessage(locale, pattern, patternlength, result as _, resultlength, ap, status as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn u_vformatMessageWithError(locale: *const i8, pattern: *const UChar, patternlength: i32, result: *mut UChar, resultlength: i32, parseerror: *mut UParseError, ap: *mut i8, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn u_vformatMessageWithError(locale : *const i8, pattern : *const UChar, patternlength : i32, result : *mut UChar, resultlength : i32, parseerror : *mut UParseError, ap : *mut i8, status : *mut UErrorCode) -> i32);
    unsafe { u_vformatMessageWithError(locale, pattern, patternlength, result as _, resultlength, parseerror as _, ap as _, status as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn u_vformatMessageWithError(locale: *const i8, pattern: *const UChar, patternlength: i32, result: *mut UChar, resultlength: i32, parseerror: *mut UParseError, ap: super::vadefs::va_list, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn u_vformatMessageWithError(locale : *const i8, pattern : *const UChar, patternlength : i32, result : *mut UChar, resultlength : i32, parseerror : *mut UParseError, ap : super::vadefs::va_list, status : *mut UErrorCode) -> i32);
    unsafe { u_vformatMessageWithError(locale, pattern, patternlength, result as _, resultlength, parseerror as _, ap, status as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn u_vparseMessage(locale: *const i8, pattern: *const UChar, patternlength: i32, source: *const UChar, sourcelength: i32, ap: *mut i8, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn u_vparseMessage(locale : *const i8, pattern : *const UChar, patternlength : i32, source : *const UChar, sourcelength : i32, ap : *mut i8, status : *mut UErrorCode));
    unsafe { u_vparseMessage(locale, pattern, patternlength, source, sourcelength, ap as _, status as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn u_vparseMessage(locale: *const i8, pattern: *const UChar, patternlength: i32, source: *const UChar, sourcelength: i32, ap: super::vadefs::va_list) -> UErrorCode {
    windows_core::link!("icuin.dll" "C" fn u_vparseMessage(locale : *const i8, pattern : *const UChar, patternlength : i32, source : *const UChar, sourcelength : i32, ap : super::vadefs::va_list, status : *mut UErrorCode));
    unsafe {
        let mut result__ = core::mem::zeroed();
        u_vparseMessage(locale, pattern, patternlength, source, sourcelength, ap, &mut result__);
        result__
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn u_vparseMessageWithError(locale: *const i8, pattern: *const UChar, patternlength: i32, source: *const UChar, sourcelength: i32, ap: *mut i8, parseerror: *mut UParseError, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn u_vparseMessageWithError(locale : *const i8, pattern : *const UChar, patternlength : i32, source : *const UChar, sourcelength : i32, ap : *mut i8, parseerror : *mut UParseError, status : *mut UErrorCode));
    unsafe { u_vparseMessageWithError(locale, pattern, patternlength, source, sourcelength, ap as _, parseerror as _, status as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn u_vparseMessageWithError(locale: *const i8, pattern: *const UChar, patternlength: i32, source: *const UChar, sourcelength: i32, ap: super::vadefs::va_list, parseerror: *mut UParseError, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn u_vparseMessageWithError(locale : *const i8, pattern : *const UChar, patternlength : i32, source : *const UChar, sourcelength : i32, ap : super::vadefs::va_list, parseerror : *mut UParseError, status : *mut UErrorCode));
    unsafe { u_vparseMessageWithError(locale, pattern, patternlength, source, sourcelength, ap, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn ubidi_close() -> UBiDi {
    windows_core::link!("icuuc.dll" "C" fn ubidi_close(pbidi : *mut UBiDi));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ubidi_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ubidi_countParagraphs(pbidi: *mut UBiDi) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_countParagraphs(pbidi : *mut UBiDi) -> i32);
    unsafe { ubidi_countParagraphs(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_countRuns(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_countRuns(pbidi : *mut UBiDi, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_countRuns(pbidi as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getBaseDirection(text: *const UChar, length: i32) -> UBiDiDirection {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getBaseDirection(text : *const UChar, length : i32) -> UBiDiDirection);
    unsafe { ubidi_getBaseDirection(text, length) }
}
#[inline]
pub unsafe fn ubidi_getClassCallback(pbidi: *mut UBiDi, r#fn: *mut UBiDiClassCallback, context: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getClassCallback(pbidi : *mut UBiDi, r#fn : *mut UBiDiClassCallback, context : *mut *mut core::ffi::c_void));
    unsafe { ubidi_getClassCallback(pbidi as _, r#fn as _, context as _) }
}
#[inline]
pub unsafe fn ubidi_getCustomizedClass(pbidi: *mut UBiDi, c: UChar32) -> UCharDirection {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getCustomizedClass(pbidi : *mut UBiDi, c : UChar32) -> UCharDirection);
    unsafe { ubidi_getCustomizedClass(pbidi as _, c) }
}
#[inline]
pub unsafe fn ubidi_getDirection(pbidi: *const UBiDi) -> UBiDiDirection {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getDirection(pbidi : *const UBiDi) -> UBiDiDirection);
    unsafe { ubidi_getDirection(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getLength(pbidi: *const UBiDi) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLength(pbidi : *const UBiDi) -> i32);
    unsafe { ubidi_getLength(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getLevelAt(pbidi: *const UBiDi, charindex: i32) -> UBiDiLevel {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLevelAt(pbidi : *const UBiDi, charindex : i32) -> UBiDiLevel);
    unsafe { ubidi_getLevelAt(pbidi, charindex) }
}
#[inline]
pub unsafe fn ubidi_getLevels(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> *const UBiDiLevel {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLevels(pbidi : *mut UBiDi, perrorcode : *mut UErrorCode) -> *const UBiDiLevel);
    unsafe { ubidi_getLevels(pbidi as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getLogicalIndex(pbidi: *mut UBiDi, visualindex: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLogicalIndex(pbidi : *mut UBiDi, visualindex : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_getLogicalIndex(pbidi as _, visualindex, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getLogicalMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLogicalMap(pbidi : *mut UBiDi, indexmap : *mut i32, perrorcode : *mut UErrorCode));
    unsafe { ubidi_getLogicalMap(pbidi as _, indexmap as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getLogicalRun(pbidi: *const UBiDi, logicalposition: i32, plogicallimit: *mut i32, plevel: *mut UBiDiLevel) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLogicalRun(pbidi : *const UBiDi, logicalposition : i32, plogicallimit : *mut i32, plevel : *mut UBiDiLevel));
    unsafe { ubidi_getLogicalRun(pbidi, logicalposition, plogicallimit as _, plevel as _) }
}
#[inline]
pub unsafe fn ubidi_getParaLevel(pbidi: *const UBiDi) -> UBiDiLevel {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getParaLevel(pbidi : *const UBiDi) -> UBiDiLevel);
    unsafe { ubidi_getParaLevel(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getParagraph(pbidi: *const UBiDi, charindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut UBiDiLevel, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getParagraph(pbidi : *const UBiDi, charindex : i32, pparastart : *mut i32, pparalimit : *mut i32, pparalevel : *mut UBiDiLevel, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_getParagraph(pbidi, charindex, pparastart as _, pparalimit as _, pparalevel as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getParagraphByIndex(pbidi: *const UBiDi, paraindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut UBiDiLevel, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getParagraphByIndex(pbidi : *const UBiDi, paraindex : i32, pparastart : *mut i32, pparalimit : *mut i32, pparalevel : *mut UBiDiLevel, perrorcode : *mut UErrorCode));
    unsafe { ubidi_getParagraphByIndex(pbidi, paraindex, pparastart as _, pparalimit as _, pparalevel as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getProcessedLength(pbidi: *const UBiDi) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getProcessedLength(pbidi : *const UBiDi) -> i32);
    unsafe { ubidi_getProcessedLength(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getReorderingMode(pbidi: *mut UBiDi) -> UBiDiReorderingMode {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getReorderingMode(pbidi : *mut UBiDi) -> UBiDiReorderingMode);
    unsafe { ubidi_getReorderingMode(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_getReorderingOptions(pbidi: *mut UBiDi) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getReorderingOptions(pbidi : *mut UBiDi) -> u32);
    unsafe { ubidi_getReorderingOptions(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_getResultLength(pbidi: *const UBiDi) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getResultLength(pbidi : *const UBiDi) -> i32);
    unsafe { ubidi_getResultLength(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getText(pbidi: *const UBiDi) -> *const UChar {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getText(pbidi : *const UBiDi) -> *const UChar);
    unsafe { ubidi_getText(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getVisualIndex(pbidi: *mut UBiDi, logicalindex: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getVisualIndex(pbidi : *mut UBiDi, logicalindex : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_getVisualIndex(pbidi as _, logicalindex, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getVisualMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getVisualMap(pbidi : *mut UBiDi, indexmap : *mut i32, perrorcode : *mut UErrorCode));
    unsafe { ubidi_getVisualMap(pbidi as _, indexmap as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getVisualRun(pbidi: *mut UBiDi, runindex: i32, plogicalstart: *mut i32, plength: *mut i32) -> UBiDiDirection {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getVisualRun(pbidi : *mut UBiDi, runindex : i32, plogicalstart : *mut i32, plength : *mut i32) -> UBiDiDirection);
    unsafe { ubidi_getVisualRun(pbidi as _, runindex, plogicalstart as _, plength as _) }
}
#[inline]
pub unsafe fn ubidi_invertMap(srcmap: *const i32, destmap: *mut i32, length: i32) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_invertMap(srcmap : *const i32, destmap : *mut i32, length : i32));
    unsafe { ubidi_invertMap(srcmap, destmap as _, length) }
}
#[inline]
pub unsafe fn ubidi_isInverse(pbidi: *mut UBiDi) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn ubidi_isInverse(pbidi : *mut UBiDi) -> UBool);
    unsafe { ubidi_isInverse(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_isOrderParagraphsLTR(pbidi: *mut UBiDi) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn ubidi_isOrderParagraphsLTR(pbidi : *mut UBiDi) -> UBool);
    unsafe { ubidi_isOrderParagraphsLTR(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_open() -> *mut UBiDi {
    windows_core::link!("icuuc.dll" "C" fn ubidi_open() -> *mut UBiDi);
    unsafe { ubidi_open() }
}
#[inline]
pub unsafe fn ubidi_openSized(maxlength: i32, maxruncount: i32, perrorcode: *mut UErrorCode) -> *mut UBiDi {
    windows_core::link!("icuuc.dll" "C" fn ubidi_openSized(maxlength : i32, maxruncount : i32, perrorcode : *mut UErrorCode) -> *mut UBiDi);
    unsafe { ubidi_openSized(maxlength, maxruncount, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_orderParagraphsLTR(pbidi: *mut UBiDi, orderparagraphsltr: UBool) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_orderParagraphsLTR(pbidi : *mut UBiDi, orderparagraphsltr : UBool));
    unsafe { ubidi_orderParagraphsLTR(pbidi as _, orderparagraphsltr) }
}
#[inline]
pub unsafe fn ubidi_reorderLogical(levels: *const UBiDiLevel, length: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_reorderLogical(levels : *const UBiDiLevel, length : i32, indexmap : *mut i32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ubidi_reorderLogical(levels, length, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ubidi_reorderVisual(levels: *const UBiDiLevel, length: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_reorderVisual(levels : *const UBiDiLevel, length : i32, indexmap : *mut i32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ubidi_reorderVisual(levels, length, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ubidi_setClassCallback(pbidi: *mut UBiDi, newfn: UBiDiClassCallback, newcontext: *const core::ffi::c_void, oldfn: *mut UBiDiClassCallback, oldcontext: *mut *mut core::ffi::c_void, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setClassCallback(pbidi : *mut UBiDi, newfn : UBiDiClassCallback, newcontext : *const core::ffi::c_void, oldfn : *mut UBiDiClassCallback, oldcontext : *mut *mut core::ffi::c_void, perrorcode : *mut UErrorCode));
    unsafe { ubidi_setClassCallback(pbidi as _, newfn, newcontext, oldfn as _, oldcontext as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_setContext(pbidi: *mut UBiDi, prologue: *const UChar, prolength: i32, epilogue: *const UChar, epilength: i32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setContext(pbidi : *mut UBiDi, prologue : *const UChar, prolength : i32, epilogue : *const UChar, epilength : i32, perrorcode : *mut UErrorCode));
    unsafe { ubidi_setContext(pbidi as _, prologue, prolength, epilogue, epilength, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_setInverse(pbidi: *mut UBiDi, isinverse: UBool) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setInverse(pbidi : *mut UBiDi, isinverse : UBool));
    unsafe { ubidi_setInverse(pbidi as _, isinverse) }
}
#[inline]
pub unsafe fn ubidi_setLine(pparabidi: *const UBiDi, start: i32, limit: i32, plinebidi: *mut UBiDi, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setLine(pparabidi : *const UBiDi, start : i32, limit : i32, plinebidi : *mut UBiDi, perrorcode : *mut UErrorCode));
    unsafe { ubidi_setLine(pparabidi, start, limit, plinebidi as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_setPara(pbidi: *mut UBiDi, text: *const UChar, length: i32, paralevel: UBiDiLevel, embeddinglevels: *mut UBiDiLevel, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setPara(pbidi : *mut UBiDi, text : *const UChar, length : i32, paralevel : UBiDiLevel, embeddinglevels : *mut UBiDiLevel, perrorcode : *mut UErrorCode));
    unsafe { ubidi_setPara(pbidi as _, text, length, paralevel, embeddinglevels as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_setReorderingMode(pbidi: *mut UBiDi, reorderingmode: UBiDiReorderingMode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setReorderingMode(pbidi : *mut UBiDi, reorderingmode : UBiDiReorderingMode));
    unsafe { ubidi_setReorderingMode(pbidi as _, reorderingmode) }
}
#[inline]
pub unsafe fn ubidi_setReorderingOptions(pbidi: *mut UBiDi, reorderingoptions: u32) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setReorderingOptions(pbidi : *mut UBiDi, reorderingoptions : u32));
    unsafe { ubidi_setReorderingOptions(pbidi as _, reorderingoptions) }
}
#[inline]
pub unsafe fn ubidi_writeReordered(pbidi: *mut UBiDi, dest: *mut UChar, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_writeReordered(pbidi : *mut UBiDi, dest : *mut UChar, destsize : i32, options : u16, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_writeReordered(pbidi as _, dest as _, destsize, options, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_writeReverse(src: *const UChar, srclength: i32, dest: *mut UChar, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_writeReverse(src : *const UChar, srclength : i32, dest : *mut UChar, destsize : i32, options : u16, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_writeReverse(src, srclength, dest as _, destsize, options, perrorcode as _) }
}
#[inline]
pub unsafe fn ubiditransform_close() -> UBiDiTransform {
    windows_core::link!("icuuc.dll" "C" fn ubiditransform_close(pbiditransform : *mut UBiDiTransform));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ubiditransform_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ubiditransform_open(perrorcode: *mut UErrorCode) -> *mut UBiDiTransform {
    windows_core::link!("icuuc.dll" "C" fn ubiditransform_open(perrorcode : *mut UErrorCode) -> *mut UBiDiTransform);
    unsafe { ubiditransform_open(perrorcode as _) }
}
#[inline]
pub unsafe fn ubiditransform_transform(pbiditransform: *mut UBiDiTransform, src: *const UChar, srclength: i32, dest: *mut UChar, destsize: i32, inparalevel: UBiDiLevel, inorder: UBiDiOrder, outparalevel: UBiDiLevel, outorder: UBiDiOrder, domirroring: UBiDiMirroring, shapingoptions: u32, perrorcode: *mut UErrorCode) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn ubiditransform_transform(pbiditransform : *mut UBiDiTransform, src : *const UChar, srclength : i32, dest : *mut UChar, destsize : i32, inparalevel : UBiDiLevel, inorder : UBiDiOrder, outparalevel : UBiDiLevel, outorder : UBiDiOrder, domirroring : UBiDiMirroring, shapingoptions : u32, perrorcode : *mut UErrorCode) -> u32);
    unsafe { ubiditransform_transform(pbiditransform as _, src, srclength, dest as _, destsize, inparalevel, inorder, outparalevel, outorder, domirroring, shapingoptions, perrorcode as _) }
}
#[inline]
pub unsafe fn ublock_getCode(c: UChar32) -> UBlockCode {
    windows_core::link!("icuuc.dll" "C" fn ublock_getCode(c : UChar32) -> UBlockCode);
    unsafe { ublock_getCode(c) }
}
#[inline]
pub unsafe fn ubrk_clone(bi: *const UBreakIterator, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_core::link!("icu.dll" "C" fn ubrk_clone(bi : *const UBreakIterator, status : *mut UErrorCode) -> *mut UBreakIterator);
    unsafe { ubrk_clone(bi, status as _) }
}
#[inline]
pub unsafe fn ubrk_close() -> UBreakIterator {
    windows_core::link!("icuuc.dll" "C" fn ubrk_close(bi : *mut UBreakIterator));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ubrk_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ubrk_countAvailable() -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_countAvailable() -> i32);
    unsafe { ubrk_countAvailable() }
}
#[inline]
pub unsafe fn ubrk_current(bi: *const UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_current(bi : *const UBreakIterator) -> i32);
    unsafe { ubrk_current(bi) }
}
#[inline]
pub unsafe fn ubrk_first(bi: *mut UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_first(bi : *mut UBreakIterator) -> i32);
    unsafe { ubrk_first(bi as _) }
}
#[inline]
pub unsafe fn ubrk_following(bi: *mut UBreakIterator, offset: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_following(bi : *mut UBreakIterator, offset : i32) -> i32);
    unsafe { ubrk_following(bi as _, offset) }
}
#[inline]
pub unsafe fn ubrk_getAvailable(index: i32) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_getAvailable(index : i32) -> *const i8);
    unsafe { ubrk_getAvailable(index) }
}
#[inline]
pub unsafe fn ubrk_getBinaryRules(bi: *mut UBreakIterator, binaryrules: *mut u8, rulescapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_getBinaryRules(bi : *mut UBreakIterator, binaryrules : *mut u8, rulescapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ubrk_getBinaryRules(bi as _, binaryrules as _, rulescapacity, status as _) }
}
#[inline]
pub unsafe fn ubrk_getLocaleByType(bi: *const UBreakIterator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_getLocaleByType(bi : *const UBreakIterator, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> *const i8);
    unsafe { ubrk_getLocaleByType(bi, r#type, status as _) }
}
#[inline]
pub unsafe fn ubrk_getRuleStatus(bi: *mut UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_getRuleStatus(bi : *mut UBreakIterator) -> i32);
    unsafe { ubrk_getRuleStatus(bi as _) }
}
#[inline]
pub unsafe fn ubrk_getRuleStatusVec(bi: *mut UBreakIterator, fillinvec: *mut i32, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_getRuleStatusVec(bi : *mut UBreakIterator, fillinvec : *mut i32, capacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ubrk_getRuleStatusVec(bi as _, fillinvec as _, capacity, status as _) }
}
#[inline]
pub unsafe fn ubrk_isBoundary(bi: *mut UBreakIterator, offset: i32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn ubrk_isBoundary(bi : *mut UBreakIterator, offset : i32) -> UBool);
    unsafe { ubrk_isBoundary(bi as _, offset) }
}
#[inline]
pub unsafe fn ubrk_last(bi: *mut UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_last(bi : *mut UBreakIterator) -> i32);
    unsafe { ubrk_last(bi as _) }
}
#[inline]
pub unsafe fn ubrk_next(bi: *mut UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_next(bi : *mut UBreakIterator) -> i32);
    unsafe { ubrk_next(bi as _) }
}
#[inline]
pub unsafe fn ubrk_open(r#type: UBreakIteratorType, locale: *const i8, text: *const UChar, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_core::link!("icuuc.dll" "C" fn ubrk_open(r#type : UBreakIteratorType, locale : *const i8, text : *const UChar, textlength : i32, status : *mut UErrorCode) -> *mut UBreakIterator);
    unsafe { ubrk_open(r#type, locale, text, textlength, status as _) }
}
#[inline]
pub unsafe fn ubrk_openBinaryRules(binaryrules: *const u8, ruleslength: i32, text: *const UChar, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_core::link!("icuuc.dll" "C" fn ubrk_openBinaryRules(binaryrules : *const u8, ruleslength : i32, text : *const UChar, textlength : i32, status : *mut UErrorCode) -> *mut UBreakIterator);
    unsafe { ubrk_openBinaryRules(binaryrules, ruleslength, text, textlength, status as _) }
}
#[inline]
pub unsafe fn ubrk_openRules(rules: *const UChar, ruleslength: i32, text: *const UChar, textlength: i32, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_core::link!("icuuc.dll" "C" fn ubrk_openRules(rules : *const UChar, ruleslength : i32, text : *const UChar, textlength : i32, parseerr : *mut UParseError, status : *mut UErrorCode) -> *mut UBreakIterator);
    unsafe { ubrk_openRules(rules, ruleslength, text, textlength, parseerr as _, status as _) }
}
#[inline]
pub unsafe fn ubrk_preceding(bi: *mut UBreakIterator, offset: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_preceding(bi : *mut UBreakIterator, offset : i32) -> i32);
    unsafe { ubrk_preceding(bi as _, offset) }
}
#[inline]
pub unsafe fn ubrk_previous(bi: *mut UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_previous(bi : *mut UBreakIterator) -> i32);
    unsafe { ubrk_previous(bi as _) }
}
#[inline]
pub unsafe fn ubrk_refreshUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubrk_refreshUText(bi : *mut UBreakIterator, text : *mut UText, status : *mut UErrorCode));
    unsafe { ubrk_refreshUText(bi as _, text as _, status as _) }
}
#[inline]
pub unsafe fn ubrk_safeClone(bi: *const UBreakIterator, stackbuffer: *mut core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_core::link!("icuuc.dll" "C" fn ubrk_safeClone(bi : *const UBreakIterator, stackbuffer : *mut core::ffi::c_void, pbuffersize : *mut i32, status : *mut UErrorCode) -> *mut UBreakIterator);
    unsafe { ubrk_safeClone(bi, stackbuffer as _, pbuffersize as _, status as _) }
}
#[inline]
pub unsafe fn ubrk_setText(bi: *mut UBreakIterator, text: *const UChar, textlength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubrk_setText(bi : *mut UBreakIterator, text : *const UChar, textlength : i32, status : *mut UErrorCode));
    unsafe { ubrk_setText(bi as _, text, textlength, status as _) }
}
#[inline]
pub unsafe fn ubrk_setUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubrk_setUText(bi : *mut UBreakIterator, text : *mut UText, status : *mut UErrorCode));
    unsafe { ubrk_setUText(bi as _, text as _, status as _) }
}
#[inline]
pub unsafe fn ucal_add(cal: *mut UCalendar, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_add(cal : *mut UCalendar, field : UCalendarDateFields, amount : i32, status : *mut UErrorCode));
    unsafe { ucal_add(cal as _, field, amount, status as _) }
}
#[inline]
pub unsafe fn ucal_clear() -> UCalendar {
    windows_core::link!("icuin.dll" "C" fn ucal_clear(calendar : *mut UCalendar));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucal_clear(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucal_clearField(cal: *mut UCalendar, field: UCalendarDateFields) {
    windows_core::link!("icuin.dll" "C" fn ucal_clearField(cal : *mut UCalendar, field : UCalendarDateFields));
    unsafe { ucal_clearField(cal as _, field) }
}
#[inline]
pub unsafe fn ucal_clone(cal: *const UCalendar, status: *mut UErrorCode) -> *mut UCalendar {
    windows_core::link!("icuin.dll" "C" fn ucal_clone(cal : *const UCalendar, status : *mut UErrorCode) -> *mut UCalendar);
    unsafe { ucal_clone(cal, status as _) }
}
#[inline]
pub unsafe fn ucal_close() -> UCalendar {
    windows_core::link!("icuin.dll" "C" fn ucal_close(cal : *mut UCalendar));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucal_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucal_countAvailable() -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_countAvailable() -> i32);
    unsafe { ucal_countAvailable() }
}
#[inline]
pub unsafe fn ucal_equivalentTo(cal1: *const UCalendar, cal2: *const UCalendar) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ucal_equivalentTo(cal1 : *const UCalendar, cal2 : *const UCalendar) -> UBool);
    unsafe { ucal_equivalentTo(cal1, cal2) }
}
#[inline]
pub unsafe fn ucal_get(cal: *const UCalendar, field: UCalendarDateFields, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_get(cal : *const UCalendar, field : UCalendarDateFields, status : *mut UErrorCode) -> i32);
    unsafe { ucal_get(cal, field, status as _) }
}
#[inline]
pub unsafe fn ucal_getAttribute(cal: *const UCalendar, attr: UCalendarAttribute) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getAttribute(cal : *const UCalendar, attr : UCalendarAttribute) -> i32);
    unsafe { ucal_getAttribute(cal, attr) }
}
#[inline]
pub unsafe fn ucal_getAvailable(localeindex: i32) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn ucal_getAvailable(localeindex : i32) -> *const i8);
    unsafe { ucal_getAvailable(localeindex) }
}
#[inline]
pub unsafe fn ucal_getCanonicalTimeZoneID(id: *const UChar, len: i32, result: *mut UChar, resultcapacity: i32, issystemid: *mut UBool, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getCanonicalTimeZoneID(id : *const UChar, len : i32, result : *mut UChar, resultcapacity : i32, issystemid : *mut UBool, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getCanonicalTimeZoneID(id, len, result as _, resultcapacity, issystemid as _, status as _) }
}
#[inline]
pub unsafe fn ucal_getDSTSavings(zoneid: *const UChar, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getDSTSavings(zoneid : *const UChar, ec : *mut UErrorCode) -> i32);
    unsafe { ucal_getDSTSavings(zoneid, ec as _) }
}
#[inline]
pub unsafe fn ucal_getDayOfWeekType(cal: *const UCalendar, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> UCalendarWeekdayType {
    windows_core::link!("icuin.dll" "C" fn ucal_getDayOfWeekType(cal : *const UCalendar, dayofweek : UCalendarDaysOfWeek, status : *mut UErrorCode) -> UCalendarWeekdayType);
    unsafe { ucal_getDayOfWeekType(cal, dayofweek, status as _) }
}
#[inline]
pub unsafe fn ucal_getDefaultTimeZone(result: *mut UChar, resultcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getDefaultTimeZone(result : *mut UChar, resultcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { ucal_getDefaultTimeZone(result as _, resultcapacity, ec as _) }
}
#[inline]
pub unsafe fn ucal_getFieldDifference(cal: *mut UCalendar, target: f64, field: UCalendarDateFields, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getFieldDifference(cal : *mut UCalendar, target : f64, field : UCalendarDateFields, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getFieldDifference(cal as _, target, field, status as _) }
}
#[inline]
pub unsafe fn ucal_getGregorianChange(cal: *const UCalendar, perrorcode: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn ucal_getGregorianChange(cal : *const UCalendar, perrorcode : *mut UErrorCode) -> f64);
    unsafe { ucal_getGregorianChange(cal, perrorcode as _) }
}
#[inline]
pub unsafe fn ucal_getHostTimeZone(result: *mut UChar, resultcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucal_getHostTimeZone(result : *mut UChar, resultcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { ucal_getHostTimeZone(result as _, resultcapacity, ec as _) }
}
#[inline]
pub unsafe fn ucal_getKeywordValuesForLocale(key: *const i8, locale: *const i8, commonlyused: UBool, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucal_getKeywordValuesForLocale(key : *const i8, locale : *const i8, commonlyused : UBool, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucal_getKeywordValuesForLocale(key, locale, commonlyused, status as _) }
}
#[inline]
pub unsafe fn ucal_getLimit(cal: *const UCalendar, field: UCalendarDateFields, r#type: UCalendarLimitType, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getLimit(cal : *const UCalendar, field : UCalendarDateFields, r#type : UCalendarLimitType, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getLimit(cal, field, r#type, status as _) }
}
#[inline]
pub unsafe fn ucal_getLocaleByType(cal: *const UCalendar, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn ucal_getLocaleByType(cal : *const UCalendar, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> *const i8);
    unsafe { ucal_getLocaleByType(cal, r#type, status as _) }
}
#[inline]
pub unsafe fn ucal_getMillis(cal: *const UCalendar, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn ucal_getMillis(cal : *const UCalendar, status : *mut UErrorCode) -> f64);
    unsafe { ucal_getMillis(cal, status as _) }
}
#[inline]
pub unsafe fn ucal_getNow() -> f64 {
    windows_core::link!("icuin.dll" "C" fn ucal_getNow() -> f64);
    unsafe { ucal_getNow() }
}
#[inline]
pub unsafe fn ucal_getTZDataVersion(status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn ucal_getTZDataVersion(status : *mut UErrorCode) -> *const i8);
    unsafe { ucal_getTZDataVersion(status as _) }
}
#[inline]
pub unsafe fn ucal_getTimeZoneDisplayName(cal: *const UCalendar, r#type: UCalendarDisplayNameType, locale: *const i8, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getTimeZoneDisplayName(cal : *const UCalendar, r#type : UCalendarDisplayNameType, locale : *const i8, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getTimeZoneDisplayName(cal, r#type, locale, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn ucal_getTimeZoneID(cal: *const UCalendar, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getTimeZoneID(cal : *const UCalendar, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getTimeZoneID(cal, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn ucal_getTimeZoneIDForWindowsID(winid: *const UChar, len: i32, region: *const i8, id: *mut UChar, idcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getTimeZoneIDForWindowsID(winid : *const UChar, len : i32, region : *const i8, id : *mut UChar, idcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getTimeZoneIDForWindowsID(winid, len, region, id as _, idcapacity, status as _) }
}
#[inline]
pub unsafe fn ucal_getTimeZoneOffsetFromLocal(cal: *const UCalendar, nonexistingtimeopt: UTimeZoneLocalOption, duplicatedtimeopt: UTimeZoneLocalOption, rawoffset: *mut i32, dstoffset: *mut i32, status: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucal_getTimeZoneOffsetFromLocal(cal : *const UCalendar, nonexistingtimeopt : UTimeZoneLocalOption, duplicatedtimeopt : UTimeZoneLocalOption, rawoffset : *mut i32, dstoffset : *mut i32, status : *mut UErrorCode));
    unsafe { ucal_getTimeZoneOffsetFromLocal(cal, nonexistingtimeopt, duplicatedtimeopt, rawoffset as _, dstoffset as _, status as _) }
}
#[inline]
pub unsafe fn ucal_getTimeZoneTransitionDate(cal: *const UCalendar, r#type: UTimeZoneTransitionType, transition: *mut f64, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ucal_getTimeZoneTransitionDate(cal : *const UCalendar, r#type : UTimeZoneTransitionType, transition : *mut f64, status : *mut UErrorCode) -> UBool);
    unsafe { ucal_getTimeZoneTransitionDate(cal, r#type, transition as _, status as _) }
}
#[inline]
pub unsafe fn ucal_getType(cal: *const UCalendar, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn ucal_getType(cal : *const UCalendar, status : *mut UErrorCode) -> *const i8);
    unsafe { ucal_getType(cal, status as _) }
}
#[inline]
pub unsafe fn ucal_getWeekendTransition(cal: *const UCalendar, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getWeekendTransition(cal : *const UCalendar, dayofweek : UCalendarDaysOfWeek, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getWeekendTransition(cal, dayofweek, status as _) }
}
#[inline]
pub unsafe fn ucal_getWindowsTimeZoneID(id: *const UChar, len: i32, winid: *mut UChar, winidcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getWindowsTimeZoneID(id : *const UChar, len : i32, winid : *mut UChar, winidcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getWindowsTimeZoneID(id, len, winid as _, winidcapacity, status as _) }
}
#[inline]
pub unsafe fn ucal_inDaylightTime(cal: *const UCalendar, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ucal_inDaylightTime(cal : *const UCalendar, status : *mut UErrorCode) -> UBool);
    unsafe { ucal_inDaylightTime(cal, status as _) }
}
#[inline]
pub unsafe fn ucal_isSet(cal: *const UCalendar, field: UCalendarDateFields) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ucal_isSet(cal : *const UCalendar, field : UCalendarDateFields) -> UBool);
    unsafe { ucal_isSet(cal, field) }
}
#[inline]
pub unsafe fn ucal_isWeekend(cal: *const UCalendar, date: f64, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ucal_isWeekend(cal : *const UCalendar, date : f64, status : *mut UErrorCode) -> UBool);
    unsafe { ucal_isWeekend(cal, date, status as _) }
}
#[inline]
pub unsafe fn ucal_open(zoneid: *const UChar, len: i32, locale: *const i8, r#type: UCalendarType, status: *mut UErrorCode) -> *mut UCalendar {
    windows_core::link!("icuin.dll" "C" fn ucal_open(zoneid : *const UChar, len : i32, locale : *const i8, r#type : UCalendarType, status : *mut UErrorCode) -> *mut UCalendar);
    unsafe { ucal_open(zoneid, len, locale, r#type, status as _) }
}
#[inline]
pub unsafe fn ucal_openCountryTimeZones(country: *const i8, ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucal_openCountryTimeZones(country : *const i8, ec : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucal_openCountryTimeZones(country, ec as _) }
}
#[inline]
pub unsafe fn ucal_openTimeZoneIDEnumeration(zonetype: USystemTimeZoneType, region: *const i8, rawoffset: *const i32, ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucal_openTimeZoneIDEnumeration(zonetype : USystemTimeZoneType, region : *const i8, rawoffset : *const i32, ec : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucal_openTimeZoneIDEnumeration(zonetype, region, rawoffset, ec as _) }
}
#[inline]
pub unsafe fn ucal_openTimeZones(ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucal_openTimeZones(ec : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucal_openTimeZones(ec as _) }
}
#[inline]
pub unsafe fn ucal_roll(cal: *mut UCalendar, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_roll(cal : *mut UCalendar, field : UCalendarDateFields, amount : i32, status : *mut UErrorCode));
    unsafe { ucal_roll(cal as _, field, amount, status as _) }
}
#[inline]
pub unsafe fn ucal_set(cal: *mut UCalendar, field: UCalendarDateFields, value: i32) {
    windows_core::link!("icuin.dll" "C" fn ucal_set(cal : *mut UCalendar, field : UCalendarDateFields, value : i32));
    unsafe { ucal_set(cal as _, field, value) }
}
#[inline]
pub unsafe fn ucal_setAttribute(cal: *mut UCalendar, attr: UCalendarAttribute, newvalue: i32) {
    windows_core::link!("icuin.dll" "C" fn ucal_setAttribute(cal : *mut UCalendar, attr : UCalendarAttribute, newvalue : i32));
    unsafe { ucal_setAttribute(cal as _, attr, newvalue) }
}
#[inline]
pub unsafe fn ucal_setDate(cal: *mut UCalendar, year: i32, month: i32, date: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setDate(cal : *mut UCalendar, year : i32, month : i32, date : i32, status : *mut UErrorCode));
    unsafe { ucal_setDate(cal as _, year, month, date, status as _) }
}
#[inline]
pub unsafe fn ucal_setDateTime(cal: *mut UCalendar, year: i32, month: i32, date: i32, hour: i32, minute: i32, second: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setDateTime(cal : *mut UCalendar, year : i32, month : i32, date : i32, hour : i32, minute : i32, second : i32, status : *mut UErrorCode));
    unsafe { ucal_setDateTime(cal as _, year, month, date, hour, minute, second, status as _) }
}
#[inline]
pub unsafe fn ucal_setDefaultTimeZone(zoneid: *const UChar) -> UErrorCode {
    windows_core::link!("icuin.dll" "C" fn ucal_setDefaultTimeZone(zoneid : *const UChar, ec : *mut UErrorCode));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucal_setDefaultTimeZone(zoneid, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucal_setGregorianChange(cal: *mut UCalendar, date: f64, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setGregorianChange(cal : *mut UCalendar, date : f64, perrorcode : *mut UErrorCode));
    unsafe { ucal_setGregorianChange(cal as _, date, perrorcode as _) }
}
#[inline]
pub unsafe fn ucal_setMillis(cal: *mut UCalendar, datetime: f64, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setMillis(cal : *mut UCalendar, datetime : f64, status : *mut UErrorCode));
    unsafe { ucal_setMillis(cal as _, datetime, status as _) }
}
#[inline]
pub unsafe fn ucal_setTimeZone(cal: *mut UCalendar, zoneid: *const UChar, len: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setTimeZone(cal : *mut UCalendar, zoneid : *const UChar, len : i32, status : *mut UErrorCode));
    unsafe { ucal_setTimeZone(cal as _, zoneid, len, status as _) }
}
#[inline]
pub unsafe fn ucasemap_close() -> UCaseMap {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_close(csm : *mut UCaseMap));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucasemap_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucasemap_getBreakIterator(csm: *const UCaseMap) -> *const UBreakIterator {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_getBreakIterator(csm : *const UCaseMap) -> *const UBreakIterator);
    unsafe { ucasemap_getBreakIterator(csm) }
}
#[inline]
pub unsafe fn ucasemap_getLocale(csm: *const UCaseMap) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_getLocale(csm : *const UCaseMap) -> *const i8);
    unsafe { ucasemap_getLocale(csm) }
}
#[inline]
pub unsafe fn ucasemap_getOptions(csm: *const UCaseMap) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_getOptions(csm : *const UCaseMap) -> u32);
    unsafe { ucasemap_getOptions(csm) }
}
#[inline]
pub unsafe fn ucasemap_open(locale: *const i8, options: u32, perrorcode: *mut UErrorCode) -> *mut UCaseMap {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_open(locale : *const i8, options : u32, perrorcode : *mut UErrorCode) -> *mut UCaseMap);
    unsafe { ucasemap_open(locale, options, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_setBreakIterator(csm: *mut UCaseMap, itertoadopt: *mut UBreakIterator, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_setBreakIterator(csm : *mut UCaseMap, itertoadopt : *mut UBreakIterator, perrorcode : *mut UErrorCode));
    unsafe { ucasemap_setBreakIterator(csm as _, itertoadopt as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_setLocale(csm: *mut UCaseMap, locale: *const i8, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_setLocale(csm : *mut UCaseMap, locale : *const i8, perrorcode : *mut UErrorCode));
    unsafe { ucasemap_setLocale(csm as _, locale, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_setOptions(csm: *mut UCaseMap, options: u32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_setOptions(csm : *mut UCaseMap, options : u32, perrorcode : *mut UErrorCode));
    unsafe { ucasemap_setOptions(csm as _, options, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_toTitle(csm: *mut UCaseMap, dest: *mut UChar, destcapacity: i32, src: *const UChar, srclength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_toTitle(csm : *mut UCaseMap, dest : *mut UChar, destcapacity : i32, src : *const UChar, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucasemap_toTitle(csm as _, dest as _, destcapacity, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_utf8FoldCase(csm: *const UCaseMap, dest: *mut i8, destcapacity: i32, src: *const i8, srclength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_utf8FoldCase(csm : *const UCaseMap, dest : *mut i8, destcapacity : i32, src : *const i8, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucasemap_utf8FoldCase(csm, dest as _, destcapacity, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_utf8ToLower(csm: *const UCaseMap, dest: *mut i8, destcapacity: i32, src: *const i8, srclength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_utf8ToLower(csm : *const UCaseMap, dest : *mut i8, destcapacity : i32, src : *const i8, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucasemap_utf8ToLower(csm, dest as _, destcapacity, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_utf8ToTitle(csm: *mut UCaseMap, dest: *mut i8, destcapacity: i32, src: *const i8, srclength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_utf8ToTitle(csm : *mut UCaseMap, dest : *mut i8, destcapacity : i32, src : *const i8, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucasemap_utf8ToTitle(csm as _, dest as _, destcapacity, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_utf8ToUpper(csm: *const UCaseMap, dest: *mut i8, destcapacity: i32, src: *const i8, srclength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_utf8ToUpper(csm : *const UCaseMap, dest : *mut i8, destcapacity : i32, src : *const i8, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucasemap_utf8ToUpper(csm, dest as _, destcapacity, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucfpos_close() -> UConstrainedFieldPosition {
    windows_core::link!("icu.dll" "C" fn ucfpos_close(ucfpos : *mut UConstrainedFieldPosition));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucfpos_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucfpos_constrainCategory(ucfpos: *mut UConstrainedFieldPosition, category: i32, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_constrainCategory(ucfpos : *mut UConstrainedFieldPosition, category : i32, ec : *mut UErrorCode));
    unsafe { ucfpos_constrainCategory(ucfpos as _, category, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_constrainField(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_constrainField(ucfpos : *mut UConstrainedFieldPosition, category : i32, field : i32, ec : *mut UErrorCode));
    unsafe { ucfpos_constrainField(ucfpos as _, category, field, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_getCategory(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucfpos_getCategory(ucfpos : *const UConstrainedFieldPosition, ec : *mut UErrorCode) -> i32);
    unsafe { ucfpos_getCategory(ucfpos, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_getField(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucfpos_getField(ucfpos : *const UConstrainedFieldPosition, ec : *mut UErrorCode) -> i32);
    unsafe { ucfpos_getField(ucfpos, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_getIndexes(ucfpos: *const UConstrainedFieldPosition, pstart: *mut i32, plimit: *mut i32, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_getIndexes(ucfpos : *const UConstrainedFieldPosition, pstart : *mut i32, plimit : *mut i32, ec : *mut UErrorCode));
    unsafe { ucfpos_getIndexes(ucfpos, pstart as _, plimit as _, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_getInt64IterationContext(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i64 {
    windows_core::link!("icu.dll" "C" fn ucfpos_getInt64IterationContext(ucfpos : *const UConstrainedFieldPosition, ec : *mut UErrorCode) -> i64);
    unsafe { ucfpos_getInt64IterationContext(ucfpos, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_matchesField(ucfpos: *const UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode) -> UBool {
    windows_core::link!("icu.dll" "C" fn ucfpos_matchesField(ucfpos : *const UConstrainedFieldPosition, category : i32, field : i32, ec : *mut UErrorCode) -> UBool);
    unsafe { ucfpos_matchesField(ucfpos, category, field, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_open(ec: *mut UErrorCode) -> *mut UConstrainedFieldPosition {
    windows_core::link!("icu.dll" "C" fn ucfpos_open(ec : *mut UErrorCode) -> *mut UConstrainedFieldPosition);
    unsafe { ucfpos_open(ec as _) }
}
#[inline]
pub unsafe fn ucfpos_reset(ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_reset(ucfpos : *mut UConstrainedFieldPosition, ec : *mut UErrorCode));
    unsafe { ucfpos_reset(ucfpos as _, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_setInt64IterationContext(ucfpos: *mut UConstrainedFieldPosition, context: i64, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_setInt64IterationContext(ucfpos : *mut UConstrainedFieldPosition, context : i64, ec : *mut UErrorCode));
    unsafe { ucfpos_setInt64IterationContext(ucfpos as _, context, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_setState(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, start: i32, limit: i32, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_setState(ucfpos : *mut UConstrainedFieldPosition, category : i32, field : i32, start : i32, limit : i32, ec : *mut UErrorCode));
    unsafe { ucfpos_setState(ucfpos as _, category, field, start, limit, ec as _) }
}
#[inline]
pub unsafe fn ucnv_cbFromUWriteBytes(args: *mut UConverterFromUnicodeArgs, source: *const i8, length: i32, offsetindex: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_cbFromUWriteBytes(args : *mut UConverterFromUnicodeArgs, source : *const i8, length : i32, offsetindex : i32, err : *mut UErrorCode));
    unsafe { ucnv_cbFromUWriteBytes(args as _, source, length, offsetindex, err as _) }
}
#[inline]
pub unsafe fn ucnv_cbFromUWriteSub(args: *mut UConverterFromUnicodeArgs, offsetindex: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_cbFromUWriteSub(args : *mut UConverterFromUnicodeArgs, offsetindex : i32, err : *mut UErrorCode));
    unsafe { ucnv_cbFromUWriteSub(args as _, offsetindex, err as _) }
}
#[inline]
pub unsafe fn ucnv_cbFromUWriteUChars(args: *mut UConverterFromUnicodeArgs, source: *mut *mut UChar, sourcelimit: *const UChar, offsetindex: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_cbFromUWriteUChars(args : *mut UConverterFromUnicodeArgs, source : *mut *mut UChar, sourcelimit : *const UChar, offsetindex : i32, err : *mut UErrorCode));
    unsafe { ucnv_cbFromUWriteUChars(args as _, source as _, sourcelimit, offsetindex, err as _) }
}
#[inline]
pub unsafe fn ucnv_cbToUWriteSub(args: *mut UConverterToUnicodeArgs, offsetindex: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_cbToUWriteSub(args : *mut UConverterToUnicodeArgs, offsetindex : i32, err : *mut UErrorCode));
    unsafe { ucnv_cbToUWriteSub(args as _, offsetindex, err as _) }
}
#[inline]
pub unsafe fn ucnv_cbToUWriteUChars(args: *mut UConverterToUnicodeArgs, source: *const UChar, length: i32, offsetindex: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_cbToUWriteUChars(args : *mut UConverterToUnicodeArgs, source : *const UChar, length : i32, offsetindex : i32, err : *mut UErrorCode));
    unsafe { ucnv_cbToUWriteUChars(args as _, source, length, offsetindex, err as _) }
}
#[inline]
pub unsafe fn ucnv_clone(cnv: *const UConverter, status: *mut UErrorCode) -> *mut UConverter {
    windows_core::link!("icu.dll" "C" fn ucnv_clone(cnv : *const UConverter, status : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_clone(cnv, status as _) }
}
#[inline]
pub unsafe fn ucnv_close() -> UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_close(converter : *mut UConverter));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucnv_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucnv_compareNames(name1: *const i8, name2: *const i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_compareNames(name1 : *const i8, name2 : *const i8) -> i32);
    unsafe { ucnv_compareNames(name1, name2) }
}
#[inline]
pub unsafe fn ucnv_convert(toconvertername: *const i8, fromconvertername: *const i8, target: *mut i8, targetcapacity: i32, source: *const i8, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_convert(toconvertername : *const i8, fromconvertername : *const i8, target : *mut i8, targetcapacity : i32, source : *const i8, sourcelength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucnv_convert(toconvertername, fromconvertername, target as _, targetcapacity, source, sourcelength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_convertEx(targetcnv: *mut UConverter, sourcecnv: *mut UConverter, target: *mut *mut i8, targetlimit: *const i8, source: *mut *mut i8, sourcelimit: *const i8, pivotstart: *mut UChar, pivotsource: *mut *mut UChar, pivottarget: *mut *mut UChar, pivotlimit: *const UChar, reset: UBool, flush: UBool, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_convertEx(targetcnv : *mut UConverter, sourcecnv : *mut UConverter, target : *mut *mut i8, targetlimit : *const i8, source : *mut *mut i8, sourcelimit : *const i8, pivotstart : *mut UChar, pivotsource : *mut *mut UChar, pivottarget : *mut *mut UChar, pivotlimit : *const UChar, reset : UBool, flush : UBool, perrorcode : *mut UErrorCode));
    unsafe { ucnv_convertEx(targetcnv as _, sourcecnv as _, target as _, targetlimit, source as _, sourcelimit, pivotstart as _, pivotsource as _, pivottarget as _, pivotlimit, reset, flush, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_countAliases(alias: *const i8, perrorcode: *mut UErrorCode) -> u16 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_countAliases(alias : *const i8, perrorcode : *mut UErrorCode) -> u16);
    unsafe { ucnv_countAliases(alias, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_countAvailable() -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_countAvailable() -> i32);
    unsafe { ucnv_countAvailable() }
}
#[inline]
pub unsafe fn ucnv_countStandards() -> u16 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_countStandards() -> u16);
    unsafe { ucnv_countStandards() }
}
#[inline]
pub unsafe fn ucnv_detectUnicodeSignature(source: *const i8, sourcelength: i32, signaturelength: *mut i32, perrorcode: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_detectUnicodeSignature(source : *const i8, sourcelength : i32, signaturelength : *mut i32, perrorcode : *mut UErrorCode) -> *const i8);
    unsafe { ucnv_detectUnicodeSignature(source, sourcelength, signaturelength as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_fixFileSeparator(cnv: *const UConverter, source: *mut UChar, sourcelen: i32) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_fixFileSeparator(cnv : *const UConverter, source : *mut UChar, sourcelen : i32));
    unsafe { ucnv_fixFileSeparator(cnv, source as _, sourcelen) }
}
#[inline]
pub unsafe fn ucnv_flushCache() -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_flushCache() -> i32);
    unsafe { ucnv_flushCache() }
}
#[inline]
pub unsafe fn ucnv_fromAlgorithmic(cnv: *mut UConverter, algorithmictype: UConverterType, target: *mut i8, targetcapacity: i32, source: *const i8, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_fromAlgorithmic(cnv : *mut UConverter, algorithmictype : UConverterType, target : *mut i8, targetcapacity : i32, source : *const i8, sourcelength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucnv_fromAlgorithmic(cnv as _, algorithmictype, target as _, targetcapacity, source, sourcelength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_fromUChars(cnv: *mut UConverter, dest: *mut i8, destcapacity: i32, src: *const UChar, srclength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_fromUChars(cnv : *mut UConverter, dest : *mut i8, destcapacity : i32, src : *const UChar, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucnv_fromUChars(cnv as _, dest as _, destcapacity, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_fromUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_fromUCountPending(cnv : *const UConverter, status : *mut UErrorCode) -> i32);
    unsafe { ucnv_fromUCountPending(cnv, status as _) }
}
#[inline]
pub unsafe fn ucnv_fromUnicode(converter: *mut UConverter, target: *mut *mut i8, targetlimit: *const i8, source: *mut *mut UChar, sourcelimit: *const UChar, offsets: *mut i32, flush: UBool, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_fromUnicode(converter : *mut UConverter, target : *mut *mut i8, targetlimit : *const i8, source : *mut *mut UChar, sourcelimit : *const UChar, offsets : *mut i32, flush : UBool, err : *mut UErrorCode));
    unsafe { ucnv_fromUnicode(converter as _, target as _, targetlimit, source as _, sourcelimit, offsets as _, flush, err as _) }
}
#[inline]
pub unsafe fn ucnv_getAlias(alias: *const i8, n: u16, perrorcode: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getAlias(alias : *const i8, n : u16, perrorcode : *mut UErrorCode) -> *const i8);
    unsafe { ucnv_getAlias(alias, n, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_getAliases(alias: *const i8, aliases: *mut *mut i8, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getAliases(alias : *const i8, aliases : *mut *mut i8, perrorcode : *mut UErrorCode));
    unsafe { ucnv_getAliases(alias, aliases as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_getAvailableName(n: i32) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getAvailableName(n : i32) -> *const i8);
    unsafe { ucnv_getAvailableName(n) }
}
#[inline]
pub unsafe fn ucnv_getCCSID(converter: *const UConverter, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getCCSID(converter : *const UConverter, err : *mut UErrorCode) -> i32);
    unsafe { ucnv_getCCSID(converter, err as _) }
}
#[inline]
pub unsafe fn ucnv_getCanonicalName(alias: *const i8, standard: *const i8, perrorcode: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getCanonicalName(alias : *const i8, standard : *const i8, perrorcode : *mut UErrorCode) -> *const i8);
    unsafe { ucnv_getCanonicalName(alias, standard, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_getDefaultName() -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getDefaultName() -> *const i8);
    unsafe { ucnv_getDefaultName() }
}
#[inline]
pub unsafe fn ucnv_getDisplayName(converter: *const UConverter, displaylocale: *const i8, displayname: *mut UChar, displaynamecapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getDisplayName(converter : *const UConverter, displaylocale : *const i8, displayname : *mut UChar, displaynamecapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { ucnv_getDisplayName(converter, displaylocale, displayname as _, displaynamecapacity, err as _) }
}
#[inline]
pub unsafe fn ucnv_getFromUCallBack(converter: *const UConverter, action: *mut UConverterFromUCallback, context: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getFromUCallBack(converter : *const UConverter, action : *mut UConverterFromUCallback, context : *mut *mut core::ffi::c_void));
    unsafe { ucnv_getFromUCallBack(converter, action as _, context as _) }
}
#[inline]
pub unsafe fn ucnv_getInvalidChars(converter: *const UConverter, errbytes: *mut i8, len: *mut i8, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getInvalidChars(converter : *const UConverter, errbytes : *mut i8, len : *mut i8, err : *mut UErrorCode));
    unsafe { ucnv_getInvalidChars(converter, errbytes as _, len as _, err as _) }
}
#[inline]
pub unsafe fn ucnv_getInvalidUChars(converter: *const UConverter, erruchars: *mut UChar, len: *mut i8, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getInvalidUChars(converter : *const UConverter, erruchars : *mut UChar, len : *mut i8, err : *mut UErrorCode));
    unsafe { ucnv_getInvalidUChars(converter, erruchars as _, len as _, err as _) }
}
#[inline]
pub unsafe fn ucnv_getMaxCharSize(converter: *const UConverter) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getMaxCharSize(converter : *const UConverter) -> i8);
    unsafe { ucnv_getMaxCharSize(converter) }
}
#[inline]
pub unsafe fn ucnv_getMinCharSize(converter: *const UConverter) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getMinCharSize(converter : *const UConverter) -> i8);
    unsafe { ucnv_getMinCharSize(converter) }
}
#[inline]
pub unsafe fn ucnv_getName(converter: *const UConverter, err: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getName(converter : *const UConverter, err : *mut UErrorCode) -> *const i8);
    unsafe { ucnv_getName(converter, err as _) }
}
#[inline]
pub unsafe fn ucnv_getNextUChar(converter: *mut UConverter, source: *mut *mut i8, sourcelimit: *const i8, err: *mut UErrorCode) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getNextUChar(converter : *mut UConverter, source : *mut *mut i8, sourcelimit : *const i8, err : *mut UErrorCode) -> UChar32);
    unsafe { ucnv_getNextUChar(converter as _, source as _, sourcelimit, err as _) }
}
#[inline]
pub unsafe fn ucnv_getPlatform(converter: *const UConverter, err: *mut UErrorCode) -> UConverterPlatform {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getPlatform(converter : *const UConverter, err : *mut UErrorCode) -> UConverterPlatform);
    unsafe { ucnv_getPlatform(converter, err as _) }
}
#[inline]
pub unsafe fn ucnv_getStandard(n: u16, perrorcode: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getStandard(n : u16, perrorcode : *mut UErrorCode) -> *const i8);
    unsafe { ucnv_getStandard(n, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_getStandardName(name: *const i8, standard: *const i8, perrorcode: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getStandardName(name : *const i8, standard : *const i8, perrorcode : *mut UErrorCode) -> *const i8);
    unsafe { ucnv_getStandardName(name, standard, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_getStarters(converter: *const UConverter, starters: *mut UBool, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getStarters(converter : *const UConverter, starters : *mut UBool, err : *mut UErrorCode));
    unsafe { ucnv_getStarters(converter, starters as _, err as _) }
}
#[inline]
pub unsafe fn ucnv_getSubstChars(converter: *const UConverter, subchars: *mut i8, len: *mut i8, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getSubstChars(converter : *const UConverter, subchars : *mut i8, len : *mut i8, err : *mut UErrorCode));
    unsafe { ucnv_getSubstChars(converter, subchars as _, len as _, err as _) }
}
#[inline]
pub unsafe fn ucnv_getToUCallBack(converter: *const UConverter, action: *mut UConverterToUCallback, context: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getToUCallBack(converter : *const UConverter, action : *mut UConverterToUCallback, context : *mut *mut core::ffi::c_void));
    unsafe { ucnv_getToUCallBack(converter, action as _, context as _) }
}
#[inline]
pub unsafe fn ucnv_getType(converter: *const UConverter) -> UConverterType {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getType(converter : *const UConverter) -> UConverterType);
    unsafe { ucnv_getType(converter) }
}
#[inline]
pub unsafe fn ucnv_getUnicodeSet(cnv: *const UConverter, setfillin: *mut USet, whichset: UConverterUnicodeSet, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getUnicodeSet(cnv : *const UConverter, setfillin : *mut USet, whichset : UConverterUnicodeSet, perrorcode : *mut UErrorCode));
    unsafe { ucnv_getUnicodeSet(cnv, setfillin as _, whichset, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_isAmbiguous(cnv: *const UConverter) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn ucnv_isAmbiguous(cnv : *const UConverter) -> UBool);
    unsafe { ucnv_isAmbiguous(cnv) }
}
#[inline]
pub unsafe fn ucnv_isFixedWidth(cnv: *mut UConverter, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn ucnv_isFixedWidth(cnv : *mut UConverter, status : *mut UErrorCode) -> UBool);
    unsafe { ucnv_isFixedWidth(cnv as _, status as _) }
}
#[inline]
pub unsafe fn ucnv_open(convertername: *const i8, err: *mut UErrorCode) -> *mut UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_open(convertername : *const i8, err : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_open(convertername, err as _) }
}
#[inline]
pub unsafe fn ucnv_openAllNames(perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn ucnv_openAllNames(perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucnv_openAllNames(perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_openCCSID(codepage: i32, platform: UConverterPlatform, err: *mut UErrorCode) -> *mut UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_openCCSID(codepage : i32, platform : UConverterPlatform, err : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_openCCSID(codepage, platform, err as _) }
}
#[inline]
pub unsafe fn ucnv_openPackage(packagename: *const i8, convertername: *const i8, err: *mut UErrorCode) -> *mut UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_openPackage(packagename : *const i8, convertername : *const i8, err : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_openPackage(packagename, convertername, err as _) }
}
#[inline]
pub unsafe fn ucnv_openStandardNames(convname: *const i8, standard: *const i8, perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn ucnv_openStandardNames(convname : *const i8, standard : *const i8, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucnv_openStandardNames(convname, standard, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_openU(name: *const UChar, err: *mut UErrorCode) -> *mut UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_openU(name : *const UChar, err : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_openU(name, err as _) }
}
#[inline]
pub unsafe fn ucnv_reset() -> UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_reset(converter : *mut UConverter));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucnv_reset(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucnv_resetFromUnicode() -> UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_resetFromUnicode(converter : *mut UConverter));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucnv_resetFromUnicode(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucnv_resetToUnicode() -> UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_resetToUnicode(converter : *mut UConverter));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucnv_resetToUnicode(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucnv_safeClone(cnv: *const UConverter, stackbuffer: *mut core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_safeClone(cnv : *const UConverter, stackbuffer : *mut core::ffi::c_void, pbuffersize : *mut i32, status : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_safeClone(cnv, stackbuffer as _, pbuffersize as _, status as _) }
}
#[inline]
pub unsafe fn ucnv_setDefaultName(name: *const i8) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_setDefaultName(name : *const i8));
    unsafe { ucnv_setDefaultName(name) }
}
#[inline]
pub unsafe fn ucnv_setFallback(cnv: *mut UConverter, usesfallback: UBool) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_setFallback(cnv : *mut UConverter, usesfallback : UBool));
    unsafe { ucnv_setFallback(cnv as _, usesfallback) }
}
#[inline]
pub unsafe fn ucnv_setFromUCallBack(converter: *mut UConverter, newaction: UConverterFromUCallback, newcontext: *const core::ffi::c_void, oldaction: *mut UConverterFromUCallback, oldcontext: *mut *mut core::ffi::c_void, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_setFromUCallBack(converter : *mut UConverter, newaction : UConverterFromUCallback, newcontext : *const core::ffi::c_void, oldaction : *mut UConverterFromUCallback, oldcontext : *mut *mut core::ffi::c_void, err : *mut UErrorCode));
    unsafe { ucnv_setFromUCallBack(converter as _, newaction, newcontext, oldaction as _, oldcontext as _, err as _) }
}
#[inline]
pub unsafe fn ucnv_setSubstChars(converter: *mut UConverter, subchars: *const i8, len: i8, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_setSubstChars(converter : *mut UConverter, subchars : *const i8, len : i8, err : *mut UErrorCode));
    unsafe { ucnv_setSubstChars(converter as _, subchars, len, err as _) }
}
#[inline]
pub unsafe fn ucnv_setSubstString(cnv: *mut UConverter, s: *const UChar, length: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_setSubstString(cnv : *mut UConverter, s : *const UChar, length : i32, err : *mut UErrorCode));
    unsafe { ucnv_setSubstString(cnv as _, s, length, err as _) }
}
#[inline]
pub unsafe fn ucnv_setToUCallBack(converter: *mut UConverter, newaction: UConverterToUCallback, newcontext: *const core::ffi::c_void, oldaction: *mut UConverterToUCallback, oldcontext: *mut *mut core::ffi::c_void, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_setToUCallBack(converter : *mut UConverter, newaction : UConverterToUCallback, newcontext : *const core::ffi::c_void, oldaction : *mut UConverterToUCallback, oldcontext : *mut *mut core::ffi::c_void, err : *mut UErrorCode));
    unsafe { ucnv_setToUCallBack(converter as _, newaction, newcontext, oldaction as _, oldcontext as _, err as _) }
}
#[inline]
pub unsafe fn ucnv_toAlgorithmic(algorithmictype: UConverterType, cnv: *mut UConverter, target: *mut i8, targetcapacity: i32, source: *const i8, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_toAlgorithmic(algorithmictype : UConverterType, cnv : *mut UConverter, target : *mut i8, targetcapacity : i32, source : *const i8, sourcelength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucnv_toAlgorithmic(algorithmictype, cnv as _, target as _, targetcapacity, source, sourcelength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_toUChars(cnv: *mut UConverter, dest: *mut UChar, destcapacity: i32, src: *const i8, srclength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_toUChars(cnv : *mut UConverter, dest : *mut UChar, destcapacity : i32, src : *const i8, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucnv_toUChars(cnv as _, dest as _, destcapacity, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_toUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_toUCountPending(cnv : *const UConverter, status : *mut UErrorCode) -> i32);
    unsafe { ucnv_toUCountPending(cnv, status as _) }
}
#[inline]
pub unsafe fn ucnv_toUnicode(converter: *mut UConverter, target: *mut *mut UChar, targetlimit: *const UChar, source: *mut *mut i8, sourcelimit: *const i8, offsets: *mut i32, flush: UBool, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_toUnicode(converter : *mut UConverter, target : *mut *mut UChar, targetlimit : *const UChar, source : *mut *mut i8, sourcelimit : *const i8, offsets : *mut i32, flush : UBool, err : *mut UErrorCode));
    unsafe { ucnv_toUnicode(converter as _, target as _, targetlimit, source as _, sourcelimit, offsets as _, flush, err as _) }
}
#[inline]
pub unsafe fn ucnv_usesFallback(cnv: *const UConverter) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn ucnv_usesFallback(cnv : *const UConverter) -> UBool);
    unsafe { ucnv_usesFallback(cnv) }
}
#[inline]
pub unsafe fn ucnvsel_close() -> UConverterSelector {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_close(sel : *mut UConverterSelector));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucnvsel_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucnvsel_open(converterlist: *const *const i8, converterlistsize: i32, excludedcodepoints: *const USet, whichset: UConverterUnicodeSet, status: *mut UErrorCode) -> *mut UConverterSelector {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_open(converterlist : *const *const i8, converterlistsize : i32, excludedcodepoints : *const USet, whichset : UConverterUnicodeSet, status : *mut UErrorCode) -> *mut UConverterSelector);
    unsafe { ucnvsel_open(converterlist, converterlistsize, excludedcodepoints, whichset, status as _) }
}
#[inline]
pub unsafe fn ucnvsel_openFromSerialized(buffer: *const core::ffi::c_void, length: i32, status: *mut UErrorCode) -> *mut UConverterSelector {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_openFromSerialized(buffer : *const core::ffi::c_void, length : i32, status : *mut UErrorCode) -> *mut UConverterSelector);
    unsafe { ucnvsel_openFromSerialized(buffer, length, status as _) }
}
#[inline]
pub unsafe fn ucnvsel_selectForString(sel: *const UConverterSelector, s: *const UChar, length: i32, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_selectForString(sel : *const UConverterSelector, s : *const UChar, length : i32, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucnvsel_selectForString(sel, s, length, status as _) }
}
#[inline]
pub unsafe fn ucnvsel_selectForUTF8(sel: *const UConverterSelector, s: *const i8, length: i32, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_selectForUTF8(sel : *const UConverterSelector, s : *const i8, length : i32, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucnvsel_selectForUTF8(sel, s, length, status as _) }
}
#[inline]
pub unsafe fn ucnvsel_serialize(sel: *const UConverterSelector, buffer: *mut core::ffi::c_void, buffercapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_serialize(sel : *const UConverterSelector, buffer : *mut core::ffi::c_void, buffercapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucnvsel_serialize(sel, buffer as _, buffercapacity, status as _) }
}
#[inline]
pub unsafe fn ucol_clone(coll: *const UCollator, status: *mut UErrorCode) -> *mut UCollator {
    windows_core::link!("icu.dll" "C" fn ucol_clone(coll : *const UCollator, status : *mut UErrorCode) -> *mut UCollator);
    unsafe { ucol_clone(coll, status as _) }
}
#[inline]
pub unsafe fn ucol_cloneBinary(coll: *const UCollator, buffer: *mut u8, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_cloneBinary(coll : *const UCollator, buffer : *mut u8, capacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucol_cloneBinary(coll, buffer as _, capacity, status as _) }
}
#[inline]
pub unsafe fn ucol_close() -> UCollator {
    windows_core::link!("icuin.dll" "C" fn ucol_close(coll : *mut UCollator));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucol_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucol_closeElements() -> UCollationElements {
    windows_core::link!("icuin.dll" "C" fn ucol_closeElements(elems : *mut UCollationElements));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucol_closeElements(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucol_countAvailable() -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_countAvailable() -> i32);
    unsafe { ucol_countAvailable() }
}
#[inline]
pub unsafe fn ucol_equal(coll: *const UCollator, source: *const UChar, sourcelength: i32, target: *const UChar, targetlength: i32) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ucol_equal(coll : *const UCollator, source : *const UChar, sourcelength : i32, target : *const UChar, targetlength : i32) -> UBool);
    unsafe { ucol_equal(coll, source, sourcelength, target, targetlength) }
}
#[inline]
pub unsafe fn ucol_getAttribute(coll: *const UCollator, attr: UColAttribute, status: *mut UErrorCode) -> UColAttributeValue {
    windows_core::link!("icuin.dll" "C" fn ucol_getAttribute(coll : *const UCollator, attr : UColAttribute, status : *mut UErrorCode) -> UColAttributeValue);
    unsafe { ucol_getAttribute(coll, attr, status as _) }
}
#[inline]
pub unsafe fn ucol_getAvailable(localeindex: i32) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn ucol_getAvailable(localeindex : i32) -> *const i8);
    unsafe { ucol_getAvailable(localeindex) }
}
#[inline]
pub unsafe fn ucol_getBound(source: *const u8, sourcelength: i32, boundtype: UColBoundMode, nooflevels: u32, result: *mut u8, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getBound(source : *const u8, sourcelength : i32, boundtype : UColBoundMode, nooflevels : u32, result : *mut u8, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucol_getBound(source, sourcelength, boundtype, nooflevels, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn ucol_getContractionsAndExpansions(coll: *const UCollator, contractions: *mut USet, expansions: *mut USet, addprefixes: UBool, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_getContractionsAndExpansions(coll : *const UCollator, contractions : *mut USet, expansions : *mut USet, addprefixes : UBool, status : *mut UErrorCode));
    unsafe { ucol_getContractionsAndExpansions(coll, contractions as _, expansions as _, addprefixes, status as _) }
}
#[inline]
pub unsafe fn ucol_getDisplayName(objloc: *const i8, disploc: *const i8, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getDisplayName(objloc : *const i8, disploc : *const i8, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucol_getDisplayName(objloc, disploc, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn ucol_getEquivalentReorderCodes(reordercode: i32, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getEquivalentReorderCodes(reordercode : i32, dest : *mut i32, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucol_getEquivalentReorderCodes(reordercode, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn ucol_getFunctionalEquivalent(result: *mut i8, resultcapacity: i32, keyword: *const i8, locale: *const i8, isavailable: *mut UBool, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getFunctionalEquivalent(result : *mut i8, resultcapacity : i32, keyword : *const i8, locale : *const i8, isavailable : *mut UBool, status : *mut UErrorCode) -> i32);
    unsafe { ucol_getFunctionalEquivalent(result as _, resultcapacity, keyword, locale, isavailable as _, status as _) }
}
#[inline]
pub unsafe fn ucol_getKeywordValues(keyword: *const i8, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucol_getKeywordValues(keyword : *const i8, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucol_getKeywordValues(keyword, status as _) }
}
#[inline]
pub unsafe fn ucol_getKeywordValuesForLocale(key: *const i8, locale: *const i8, commonlyused: UBool, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucol_getKeywordValuesForLocale(key : *const i8, locale : *const i8, commonlyused : UBool, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucol_getKeywordValuesForLocale(key, locale, commonlyused, status as _) }
}
#[inline]
pub unsafe fn ucol_getKeywords(status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucol_getKeywords(status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucol_getKeywords(status as _) }
}
#[inline]
pub unsafe fn ucol_getLocaleByType(coll: *const UCollator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn ucol_getLocaleByType(coll : *const UCollator, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> *const i8);
    unsafe { ucol_getLocaleByType(coll, r#type, status as _) }
}
#[inline]
pub unsafe fn ucol_getMaxExpansion(elems: *const UCollationElements, order: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getMaxExpansion(elems : *const UCollationElements, order : i32) -> i32);
    unsafe { ucol_getMaxExpansion(elems, order) }
}
#[inline]
pub unsafe fn ucol_getMaxVariable(coll: *const UCollator) -> UColReorderCode {
    windows_core::link!("icuin.dll" "C" fn ucol_getMaxVariable(coll : *const UCollator) -> UColReorderCode);
    unsafe { ucol_getMaxVariable(coll) }
}
#[inline]
pub unsafe fn ucol_getOffset(elems: *const UCollationElements) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getOffset(elems : *const UCollationElements) -> i32);
    unsafe { ucol_getOffset(elems) }
}
#[inline]
pub unsafe fn ucol_getReorderCodes(coll: *const UCollator, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getReorderCodes(coll : *const UCollator, dest : *mut i32, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucol_getReorderCodes(coll, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn ucol_getRules(coll: *const UCollator, length: *mut i32) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn ucol_getRules(coll : *const UCollator, length : *mut i32) -> *const UChar);
    unsafe { ucol_getRules(coll, length as _) }
}
#[inline]
pub unsafe fn ucol_getRulesEx(coll: *const UCollator, delta: UColRuleOption, buffer: *mut UChar, bufferlen: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getRulesEx(coll : *const UCollator, delta : UColRuleOption, buffer : *mut UChar, bufferlen : i32) -> i32);
    unsafe { ucol_getRulesEx(coll, delta, buffer as _, bufferlen) }
}
#[inline]
pub unsafe fn ucol_getSortKey(coll: *const UCollator, source: *const UChar, sourcelength: i32, result: *mut u8, resultlength: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getSortKey(coll : *const UCollator, source : *const UChar, sourcelength : i32, result : *mut u8, resultlength : i32) -> i32);
    unsafe { ucol_getSortKey(coll, source, sourcelength, result as _, resultlength) }
}
#[inline]
pub unsafe fn ucol_getStrength(coll: *const UCollator) -> UCollationStrength {
    windows_core::link!("icuin.dll" "C" fn ucol_getStrength(coll : *const UCollator) -> UCollationStrength);
    unsafe { ucol_getStrength(coll) }
}
#[inline]
pub unsafe fn ucol_getTailoredSet(coll: *const UCollator, status: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuin.dll" "C" fn ucol_getTailoredSet(coll : *const UCollator, status : *mut UErrorCode) -> *mut USet);
    unsafe { ucol_getTailoredSet(coll, status as _) }
}
#[inline]
pub unsafe fn ucol_getUCAVersion(coll: *const UCollator) -> u8 {
    windows_core::link!("icuin.dll" "C" fn ucol_getUCAVersion(coll : *const UCollator, info : *mut u8));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucol_getUCAVersion(coll, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucol_getVariableTop(coll: *const UCollator, status: *mut UErrorCode) -> u32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getVariableTop(coll : *const UCollator, status : *mut UErrorCode) -> u32);
    unsafe { ucol_getVariableTop(coll, status as _) }
}
#[inline]
pub unsafe fn ucol_getVersion(coll: *const UCollator) -> u8 {
    windows_core::link!("icuin.dll" "C" fn ucol_getVersion(coll : *const UCollator, info : *mut u8));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucol_getVersion(coll, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucol_greater(coll: *const UCollator, source: *const UChar, sourcelength: i32, target: *const UChar, targetlength: i32) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ucol_greater(coll : *const UCollator, source : *const UChar, sourcelength : i32, target : *const UChar, targetlength : i32) -> UBool);
    unsafe { ucol_greater(coll, source, sourcelength, target, targetlength) }
}
#[inline]
pub unsafe fn ucol_greaterOrEqual(coll: *const UCollator, source: *const UChar, sourcelength: i32, target: *const UChar, targetlength: i32) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ucol_greaterOrEqual(coll : *const UCollator, source : *const UChar, sourcelength : i32, target : *const UChar, targetlength : i32) -> UBool);
    unsafe { ucol_greaterOrEqual(coll, source, sourcelength, target, targetlength) }
}
#[inline]
pub unsafe fn ucol_keyHashCode(key: *const u8, length: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_keyHashCode(key : *const u8, length : i32) -> i32);
    unsafe { ucol_keyHashCode(key, length) }
}
#[inline]
pub unsafe fn ucol_mergeSortkeys(src1: *const u8, src1length: i32, src2: *const u8, src2length: i32, dest: *mut u8, destcapacity: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_mergeSortkeys(src1 : *const u8, src1length : i32, src2 : *const u8, src2length : i32, dest : *mut u8, destcapacity : i32) -> i32);
    unsafe { ucol_mergeSortkeys(src1, src1length, src2, src2length, dest as _, destcapacity) }
}
#[inline]
pub unsafe fn ucol_next(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_next(elems : *mut UCollationElements, status : *mut UErrorCode) -> i32);
    unsafe { ucol_next(elems as _, status as _) }
}
#[inline]
pub unsafe fn ucol_nextSortKeyPart(coll: *const UCollator, iter: *mut UCharIterator, state: *mut u32, dest: *mut u8, count: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_nextSortKeyPart(coll : *const UCollator, iter : *mut UCharIterator, state : *mut u32, dest : *mut u8, count : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucol_nextSortKeyPart(coll, iter as _, state as _, dest as _, count, status as _) }
}
#[inline]
pub unsafe fn ucol_open(loc: *const i8, status: *mut UErrorCode) -> *mut UCollator {
    windows_core::link!("icuin.dll" "C" fn ucol_open(loc : *const i8, status : *mut UErrorCode) -> *mut UCollator);
    unsafe { ucol_open(loc, status as _) }
}
#[inline]
pub unsafe fn ucol_openAvailableLocales(status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucol_openAvailableLocales(status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucol_openAvailableLocales(status as _) }
}
#[inline]
pub unsafe fn ucol_openBinary(bin: *const u8, length: i32, base: *const UCollator, status: *mut UErrorCode) -> *mut UCollator {
    windows_core::link!("icuin.dll" "C" fn ucol_openBinary(bin : *const u8, length : i32, base : *const UCollator, status : *mut UErrorCode) -> *mut UCollator);
    unsafe { ucol_openBinary(bin, length, base, status as _) }
}
#[inline]
pub unsafe fn ucol_openElements(coll: *const UCollator, text: *const UChar, textlength: i32, status: *mut UErrorCode) -> *mut UCollationElements {
    windows_core::link!("icuin.dll" "C" fn ucol_openElements(coll : *const UCollator, text : *const UChar, textlength : i32, status : *mut UErrorCode) -> *mut UCollationElements);
    unsafe { ucol_openElements(coll, text, textlength, status as _) }
}
#[inline]
pub unsafe fn ucol_openRules(rules: *const UChar, ruleslength: i32, normalizationmode: UColAttributeValue, strength: UCollationStrength, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut UCollator {
    windows_core::link!("icuin.dll" "C" fn ucol_openRules(rules : *const UChar, ruleslength : i32, normalizationmode : UColAttributeValue, strength : UCollationStrength, parseerror : *mut UParseError, status : *mut UErrorCode) -> *mut UCollator);
    unsafe { ucol_openRules(rules, ruleslength, normalizationmode, strength, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn ucol_previous(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_previous(elems : *mut UCollationElements, status : *mut UErrorCode) -> i32);
    unsafe { ucol_previous(elems as _, status as _) }
}
#[inline]
pub unsafe fn ucol_primaryOrder(order: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_primaryOrder(order : i32) -> i32);
    unsafe { ucol_primaryOrder(order) }
}
#[inline]
pub unsafe fn ucol_reset() -> UCollationElements {
    windows_core::link!("icuin.dll" "C" fn ucol_reset(elems : *mut UCollationElements));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucol_reset(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucol_safeClone(coll: *const UCollator, stackbuffer: *mut core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UCollator {
    windows_core::link!("icuin.dll" "C" fn ucol_safeClone(coll : *const UCollator, stackbuffer : *mut core::ffi::c_void, pbuffersize : *mut i32, status : *mut UErrorCode) -> *mut UCollator);
    unsafe { ucol_safeClone(coll, stackbuffer as _, pbuffersize as _, status as _) }
}
#[inline]
pub unsafe fn ucol_secondaryOrder(order: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_secondaryOrder(order : i32) -> i32);
    unsafe { ucol_secondaryOrder(order) }
}
#[inline]
pub unsafe fn ucol_setAttribute(coll: *mut UCollator, attr: UColAttribute, value: UColAttributeValue, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_setAttribute(coll : *mut UCollator, attr : UColAttribute, value : UColAttributeValue, status : *mut UErrorCode));
    unsafe { ucol_setAttribute(coll as _, attr, value, status as _) }
}
#[inline]
pub unsafe fn ucol_setMaxVariable(coll: *mut UCollator, group: UColReorderCode, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_setMaxVariable(coll : *mut UCollator, group : UColReorderCode, perrorcode : *mut UErrorCode));
    unsafe { ucol_setMaxVariable(coll as _, group, perrorcode as _) }
}
#[inline]
pub unsafe fn ucol_setOffset(elems: *mut UCollationElements, offset: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_setOffset(elems : *mut UCollationElements, offset : i32, status : *mut UErrorCode));
    unsafe { ucol_setOffset(elems as _, offset, status as _) }
}
#[inline]
pub unsafe fn ucol_setReorderCodes(coll: *mut UCollator, reordercodes: *const i32, reordercodeslength: i32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_setReorderCodes(coll : *mut UCollator, reordercodes : *const i32, reordercodeslength : i32, perrorcode : *mut UErrorCode));
    unsafe { ucol_setReorderCodes(coll as _, reordercodes, reordercodeslength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucol_setStrength(coll: *mut UCollator, strength: UCollationStrength) {
    windows_core::link!("icuin.dll" "C" fn ucol_setStrength(coll : *mut UCollator, strength : UCollationStrength));
    unsafe { ucol_setStrength(coll as _, strength) }
}
#[inline]
pub unsafe fn ucol_setText(elems: *mut UCollationElements, text: *const UChar, textlength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_setText(elems : *mut UCollationElements, text : *const UChar, textlength : i32, status : *mut UErrorCode));
    unsafe { ucol_setText(elems as _, text, textlength, status as _) }
}
#[inline]
pub unsafe fn ucol_strcoll(coll: *const UCollator, source: *const UChar, sourcelength: i32, target: *const UChar, targetlength: i32) -> UCollationResult {
    windows_core::link!("icuin.dll" "C" fn ucol_strcoll(coll : *const UCollator, source : *const UChar, sourcelength : i32, target : *const UChar, targetlength : i32) -> UCollationResult);
    unsafe { ucol_strcoll(coll, source, sourcelength, target, targetlength) }
}
#[inline]
pub unsafe fn ucol_strcollIter(coll: *const UCollator, siter: *mut UCharIterator, titer: *mut UCharIterator, status: *mut UErrorCode) -> UCollationResult {
    windows_core::link!("icuin.dll" "C" fn ucol_strcollIter(coll : *const UCollator, siter : *mut UCharIterator, titer : *mut UCharIterator, status : *mut UErrorCode) -> UCollationResult);
    unsafe { ucol_strcollIter(coll, siter as _, titer as _, status as _) }
}
#[inline]
pub unsafe fn ucol_strcollUTF8(coll: *const UCollator, source: *const i8, sourcelength: i32, target: *const i8, targetlength: i32, status: *mut UErrorCode) -> UCollationResult {
    windows_core::link!("icuin.dll" "C" fn ucol_strcollUTF8(coll : *const UCollator, source : *const i8, sourcelength : i32, target : *const i8, targetlength : i32, status : *mut UErrorCode) -> UCollationResult);
    unsafe { ucol_strcollUTF8(coll, source, sourcelength, target, targetlength, status as _) }
}
#[inline]
pub unsafe fn ucol_tertiaryOrder(order: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_tertiaryOrder(order : i32) -> i32);
    unsafe { ucol_tertiaryOrder(order) }
}
#[inline]
pub unsafe fn ucpmap_get(map: *const UCPMap, c: UChar32) -> u32 {
    windows_core::link!("icu.dll" "C" fn ucpmap_get(map : *const UCPMap, c : UChar32) -> u32);
    unsafe { ucpmap_get(map, c) }
}
#[inline]
pub unsafe fn ucpmap_getRange(map: *const UCPMap, start: UChar32, option: UCPMapRangeOption, surrogatevalue: u32, filter: UCPMapValueFilter, context: *const core::ffi::c_void, pvalue: *mut u32) -> UChar32 {
    windows_core::link!("icu.dll" "C" fn ucpmap_getRange(map : *const UCPMap, start : UChar32, option : UCPMapRangeOption, surrogatevalue : u32, filter : UCPMapValueFilter, context : *const core::ffi::c_void, pvalue : *mut u32) -> UChar32);
    unsafe { ucpmap_getRange(map, start, option, surrogatevalue, filter, context, pvalue as _) }
}
#[inline]
pub unsafe fn ucptrie_close(trie: *mut UCPTrie) {
    windows_core::link!("icu.dll" "C" fn ucptrie_close(trie : *mut UCPTrie));
    unsafe { ucptrie_close(trie as _) }
}
#[inline]
pub unsafe fn ucptrie_get(trie: *const UCPTrie, c: UChar32) -> u32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_get(trie : *const UCPTrie, c : UChar32) -> u32);
    unsafe { ucptrie_get(trie, c) }
}
#[inline]
pub unsafe fn ucptrie_getRange(trie: *const UCPTrie, start: UChar32, option: UCPMapRangeOption, surrogatevalue: u32, filter: UCPMapValueFilter, context: *const core::ffi::c_void, pvalue: *mut u32) -> UChar32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_getRange(trie : *const UCPTrie, start : UChar32, option : UCPMapRangeOption, surrogatevalue : u32, filter : UCPMapValueFilter, context : *const core::ffi::c_void, pvalue : *mut u32) -> UChar32);
    unsafe { ucptrie_getRange(trie, start, option, surrogatevalue, filter, context, pvalue as _) }
}
#[inline]
pub unsafe fn ucptrie_getType(trie: *const UCPTrie) -> UCPTrieType {
    windows_core::link!("icu.dll" "C" fn ucptrie_getType(trie : *const UCPTrie) -> UCPTrieType);
    unsafe { ucptrie_getType(trie) }
}
#[inline]
pub unsafe fn ucptrie_getValueWidth(trie: *const UCPTrie) -> UCPTrieValueWidth {
    windows_core::link!("icu.dll" "C" fn ucptrie_getValueWidth(trie : *const UCPTrie) -> UCPTrieValueWidth);
    unsafe { ucptrie_getValueWidth(trie) }
}
#[inline]
pub unsafe fn ucptrie_internalSmallIndex(trie: *const UCPTrie, c: UChar32) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_internalSmallIndex(trie : *const UCPTrie, c : UChar32) -> i32);
    unsafe { ucptrie_internalSmallIndex(trie, c) }
}
#[inline]
pub unsafe fn ucptrie_internalSmallU8Index(trie: *const UCPTrie, lt1: i32, t2: u8, t3: u8) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_internalSmallU8Index(trie : *const UCPTrie, lt1 : i32, t2 : u8, t3 : u8) -> i32);
    unsafe { ucptrie_internalSmallU8Index(trie, lt1, t2, t3) }
}
#[inline]
pub unsafe fn ucptrie_internalU8PrevIndex(trie: *const UCPTrie, c: UChar32, start: *const u8, src: *const u8) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_internalU8PrevIndex(trie : *const UCPTrie, c : UChar32, start : *const u8, src : *const u8) -> i32);
    unsafe { ucptrie_internalU8PrevIndex(trie, c, start, src) }
}
#[inline]
pub unsafe fn ucptrie_openFromBinary(r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, data: *const core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut UCPTrie {
    windows_core::link!("icu.dll" "C" fn ucptrie_openFromBinary(r#type : UCPTrieType, valuewidth : UCPTrieValueWidth, data : *const core::ffi::c_void, length : i32, pactuallength : *mut i32, perrorcode : *mut UErrorCode) -> *mut UCPTrie);
    unsafe { ucptrie_openFromBinary(r#type, valuewidth, data, length, pactuallength as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ucptrie_toBinary(trie: *const UCPTrie, data: *mut core::ffi::c_void, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_toBinary(trie : *const UCPTrie, data : *mut core::ffi::c_void, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucptrie_toBinary(trie, data as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn ucsdet_close() -> UCharsetDetector {
    windows_core::link!("icuin.dll" "C" fn ucsdet_close(ucsd : *mut UCharsetDetector));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ucsdet_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ucsdet_detect(ucsd: *mut UCharsetDetector, status: *mut UErrorCode) -> *const UCharsetMatch {
    windows_core::link!("icuin.dll" "C" fn ucsdet_detect(ucsd : *mut UCharsetDetector, status : *mut UErrorCode) -> *const UCharsetMatch);
    unsafe { ucsdet_detect(ucsd as _, status as _) }
}
#[inline]
pub unsafe fn ucsdet_detectAll(ucsd: *mut UCharsetDetector, matchesfound: *mut i32, status: *mut UErrorCode) -> *const *const UCharsetMatch {
    windows_core::link!("icuin.dll" "C" fn ucsdet_detectAll(ucsd : *mut UCharsetDetector, matchesfound : *mut i32, status : *mut UErrorCode) -> *const *const UCharsetMatch);
    unsafe { ucsdet_detectAll(ucsd as _, matchesfound as _, status as _) }
}
#[inline]
pub unsafe fn ucsdet_enableInputFilter(ucsd: *mut UCharsetDetector, filter: UBool) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ucsdet_enableInputFilter(ucsd : *mut UCharsetDetector, filter : UBool) -> UBool);
    unsafe { ucsdet_enableInputFilter(ucsd as _, filter) }
}
#[inline]
pub unsafe fn ucsdet_getAllDetectableCharsets(ucsd: *const UCharsetDetector, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucsdet_getAllDetectableCharsets(ucsd : *const UCharsetDetector, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucsdet_getAllDetectableCharsets(ucsd, status as _) }
}
#[inline]
pub unsafe fn ucsdet_getConfidence(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucsdet_getConfidence(ucsm : *const UCharsetMatch, status : *mut UErrorCode) -> i32);
    unsafe { ucsdet_getConfidence(ucsm, status as _) }
}
#[inline]
pub unsafe fn ucsdet_getLanguage(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn ucsdet_getLanguage(ucsm : *const UCharsetMatch, status : *mut UErrorCode) -> *const i8);
    unsafe { ucsdet_getLanguage(ucsm, status as _) }
}
#[inline]
pub unsafe fn ucsdet_getName(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn ucsdet_getName(ucsm : *const UCharsetMatch, status : *mut UErrorCode) -> *const i8);
    unsafe { ucsdet_getName(ucsm, status as _) }
}
#[inline]
pub unsafe fn ucsdet_getUChars(ucsm: *const UCharsetMatch, buf: *mut UChar, cap: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucsdet_getUChars(ucsm : *const UCharsetMatch, buf : *mut UChar, cap : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucsdet_getUChars(ucsm, buf as _, cap, status as _) }
}
#[inline]
pub unsafe fn ucsdet_isInputFilterEnabled(ucsd: *const UCharsetDetector) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ucsdet_isInputFilterEnabled(ucsd : *const UCharsetDetector) -> UBool);
    unsafe { ucsdet_isInputFilterEnabled(ucsd) }
}
#[inline]
pub unsafe fn ucsdet_open(status: *mut UErrorCode) -> *mut UCharsetDetector {
    windows_core::link!("icuin.dll" "C" fn ucsdet_open(status : *mut UErrorCode) -> *mut UCharsetDetector);
    unsafe { ucsdet_open(status as _) }
}
#[inline]
pub unsafe fn ucsdet_setDeclaredEncoding(ucsd: *mut UCharsetDetector, encoding: *const i8, length: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucsdet_setDeclaredEncoding(ucsd : *mut UCharsetDetector, encoding : *const i8, length : i32, status : *mut UErrorCode));
    unsafe { ucsdet_setDeclaredEncoding(ucsd as _, encoding, length, status as _) }
}
#[inline]
pub unsafe fn ucsdet_setText(ucsd: *mut UCharsetDetector, textin: *const i8, len: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucsdet_setText(ucsd : *mut UCharsetDetector, textin : *const i8, len : i32, status : *mut UErrorCode));
    unsafe { ucsdet_setText(ucsd as _, textin, len, status as _) }
}
#[inline]
pub unsafe fn ucurr_countCurrencies(locale: *const i8, date: f64, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_countCurrencies(locale : *const i8, date : f64, ec : *mut UErrorCode) -> i32);
    unsafe { ucurr_countCurrencies(locale, date, ec as _) }
}
#[inline]
pub unsafe fn ucurr_forLocale(locale: *const i8, buff: *mut UChar, buffcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_forLocale(locale : *const i8, buff : *mut UChar, buffcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { ucurr_forLocale(locale, buff as _, buffcapacity, ec as _) }
}
#[inline]
pub unsafe fn ucurr_forLocaleAndDate(locale: *const i8, date: f64, index: i32, buff: *mut UChar, buffcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_forLocaleAndDate(locale : *const i8, date : f64, index : i32, buff : *mut UChar, buffcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { ucurr_forLocaleAndDate(locale, date, index, buff as _, buffcapacity, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getDefaultFractionDigits(currency: *const UChar, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getDefaultFractionDigits(currency : *const UChar, ec : *mut UErrorCode) -> i32);
    unsafe { ucurr_getDefaultFractionDigits(currency, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getDefaultFractionDigitsForUsage(currency: *const UChar, usage: UCurrencyUsage, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getDefaultFractionDigitsForUsage(currency : *const UChar, usage : UCurrencyUsage, ec : *mut UErrorCode) -> i32);
    unsafe { ucurr_getDefaultFractionDigitsForUsage(currency, usage, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getKeywordValuesForLocale(key: *const i8, locale: *const i8, commonlyused: UBool, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getKeywordValuesForLocale(key : *const i8, locale : *const i8, commonlyused : UBool, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucurr_getKeywordValuesForLocale(key, locale, commonlyused, status as _) }
}
#[inline]
pub unsafe fn ucurr_getName(currency: *const UChar, locale: *const i8, namestyle: UCurrNameStyle, ischoiceformat: *mut UBool, len: *mut i32, ec: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getName(currency : *const UChar, locale : *const i8, namestyle : UCurrNameStyle, ischoiceformat : *mut UBool, len : *mut i32, ec : *mut UErrorCode) -> *const UChar);
    unsafe { ucurr_getName(currency, locale, namestyle, ischoiceformat as _, len as _, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getNumericCode(currency: *const UChar) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getNumericCode(currency : *const UChar) -> i32);
    unsafe { ucurr_getNumericCode(currency) }
}
#[inline]
pub unsafe fn ucurr_getPluralName(currency: *const UChar, locale: *const i8, ischoiceformat: *mut UBool, pluralcount: *const i8, len: *mut i32, ec: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getPluralName(currency : *const UChar, locale : *const i8, ischoiceformat : *mut UBool, pluralcount : *const i8, len : *mut i32, ec : *mut UErrorCode) -> *const UChar);
    unsafe { ucurr_getPluralName(currency, locale, ischoiceformat as _, pluralcount, len as _, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getRoundingIncrement(currency: *const UChar, ec: *mut UErrorCode) -> f64 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getRoundingIncrement(currency : *const UChar, ec : *mut UErrorCode) -> f64);
    unsafe { ucurr_getRoundingIncrement(currency, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getRoundingIncrementForUsage(currency: *const UChar, usage: UCurrencyUsage, ec: *mut UErrorCode) -> f64 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getRoundingIncrementForUsage(currency : *const UChar, usage : UCurrencyUsage, ec : *mut UErrorCode) -> f64);
    unsafe { ucurr_getRoundingIncrementForUsage(currency, usage, ec as _) }
}
#[inline]
pub unsafe fn ucurr_isAvailable(isocode: *const UChar, from: f64, to: f64, errorcode: *mut UErrorCode) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn ucurr_isAvailable(isocode : *const UChar, from : f64, to : f64, errorcode : *mut UErrorCode) -> UBool);
    unsafe { ucurr_isAvailable(isocode, from, to, errorcode as _) }
}
#[inline]
pub unsafe fn ucurr_openISOCurrencies(currtype: u32, perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn ucurr_openISOCurrencies(currtype : u32, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucurr_openISOCurrencies(currtype, perrorcode as _) }
}
#[inline]
pub unsafe fn ucurr_register(isocode: *const UChar, locale: *const i8, status: *mut UErrorCode) -> UCurrRegistryKey {
    windows_core::link!("icuuc.dll" "C" fn ucurr_register(isocode : *const UChar, locale : *const i8, status : *mut UErrorCode) -> UCurrRegistryKey);
    unsafe { ucurr_register(isocode, locale, status as _) }
}
#[inline]
pub unsafe fn ucurr_unregister(key: UCurrRegistryKey, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn ucurr_unregister(key : UCurrRegistryKey, status : *mut UErrorCode) -> UBool);
    unsafe { ucurr_unregister(key, status as _) }
}
#[inline]
pub unsafe fn udat_adoptNumberFormat(fmt: *mut UDateFormat, numberformattoadopt: *mut UNumberFormat) {
    windows_core::link!("icuin.dll" "C" fn udat_adoptNumberFormat(fmt : *mut UDateFormat, numberformattoadopt : *mut UNumberFormat));
    unsafe { udat_adoptNumberFormat(fmt as _, numberformattoadopt as _) }
}
#[inline]
pub unsafe fn udat_adoptNumberFormatForFields(fmt: *mut UDateFormat, fields: *const UChar, numberformattoset: *mut UNumberFormat, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_adoptNumberFormatForFields(fmt : *mut UDateFormat, fields : *const UChar, numberformattoset : *mut UNumberFormat, status : *mut UErrorCode));
    unsafe { udat_adoptNumberFormatForFields(fmt as _, fields, numberformattoset as _, status as _) }
}
#[inline]
pub unsafe fn udat_applyPattern(format: *mut UDateFormat, localized: UBool, pattern: *const UChar, patternlength: i32) {
    windows_core::link!("icuin.dll" "C" fn udat_applyPattern(format : *mut UDateFormat, localized : UBool, pattern : *const UChar, patternlength : i32));
    unsafe { udat_applyPattern(format as _, localized, pattern, patternlength) }
}
#[inline]
pub unsafe fn udat_clone(fmt: *const UDateFormat, status: *mut UErrorCode) -> *mut UDateFormat {
    windows_core::link!("icuin.dll" "C" fn udat_clone(fmt : *const UDateFormat, status : *mut UErrorCode) -> *mut UDateFormat);
    unsafe { udat_clone(fmt, status as _) }
}
#[inline]
pub unsafe fn udat_close() -> UDateFormat {
    windows_core::link!("icuin.dll" "C" fn udat_close(format : *mut UDateFormat));
    unsafe {
        let mut result__ = core::mem::zeroed();
        udat_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn udat_countAvailable() -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_countAvailable() -> i32);
    unsafe { udat_countAvailable() }
}
#[inline]
pub unsafe fn udat_countSymbols(fmt: *const UDateFormat, r#type: UDateFormatSymbolType) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_countSymbols(fmt : *const UDateFormat, r#type : UDateFormatSymbolType) -> i32);
    unsafe { udat_countSymbols(fmt, r#type) }
}
#[inline]
pub unsafe fn udat_format(format: *const UDateFormat, datetoformat: f64, result: *mut UChar, resultlength: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_format(format : *const UDateFormat, datetoformat : f64, result : *mut UChar, resultlength : i32, position : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { udat_format(format, datetoformat, result as _, resultlength, position as _, status as _) }
}
#[inline]
pub unsafe fn udat_formatCalendar(format: *const UDateFormat, calendar: *mut UCalendar, result: *mut UChar, capacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_formatCalendar(format : *const UDateFormat, calendar : *mut UCalendar, result : *mut UChar, capacity : i32, position : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { udat_formatCalendar(format, calendar as _, result as _, capacity, position as _, status as _) }
}
#[inline]
pub unsafe fn udat_formatCalendarForFields(format: *const UDateFormat, calendar: *mut UCalendar, result: *mut UChar, capacity: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_formatCalendarForFields(format : *const UDateFormat, calendar : *mut UCalendar, result : *mut UChar, capacity : i32, fpositer : *mut UFieldPositionIterator, status : *mut UErrorCode) -> i32);
    unsafe { udat_formatCalendarForFields(format, calendar as _, result as _, capacity, fpositer as _, status as _) }
}
#[inline]
pub unsafe fn udat_formatForFields(format: *const UDateFormat, datetoformat: f64, result: *mut UChar, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_formatForFields(format : *const UDateFormat, datetoformat : f64, result : *mut UChar, resultlength : i32, fpositer : *mut UFieldPositionIterator, status : *mut UErrorCode) -> i32);
    unsafe { udat_formatForFields(format, datetoformat, result as _, resultlength, fpositer as _, status as _) }
}
#[inline]
pub unsafe fn udat_get2DigitYearStart(fmt: *const UDateFormat, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn udat_get2DigitYearStart(fmt : *const UDateFormat, status : *mut UErrorCode) -> f64);
    unsafe { udat_get2DigitYearStart(fmt, status as _) }
}
#[inline]
pub unsafe fn udat_getAvailable(localeindex: i32) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn udat_getAvailable(localeindex : i32) -> *const i8);
    unsafe { udat_getAvailable(localeindex) }
}
#[inline]
pub unsafe fn udat_getBooleanAttribute(fmt: *const UDateFormat, attr: UDateFormatBooleanAttribute, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn udat_getBooleanAttribute(fmt : *const UDateFormat, attr : UDateFormatBooleanAttribute, status : *mut UErrorCode) -> UBool);
    unsafe { udat_getBooleanAttribute(fmt, attr, status as _) }
}
#[inline]
pub unsafe fn udat_getCalendar(fmt: *const UDateFormat) -> *const UCalendar {
    windows_core::link!("icuin.dll" "C" fn udat_getCalendar(fmt : *const UDateFormat) -> *const UCalendar);
    unsafe { udat_getCalendar(fmt) }
}
#[inline]
pub unsafe fn udat_getContext(fmt: *const UDateFormat, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext {
    windows_core::link!("icuin.dll" "C" fn udat_getContext(fmt : *const UDateFormat, r#type : UDisplayContextType, status : *mut UErrorCode) -> UDisplayContext);
    unsafe { udat_getContext(fmt, r#type, status as _) }
}
#[inline]
pub unsafe fn udat_getLocaleByType(fmt: *const UDateFormat, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn udat_getLocaleByType(fmt : *const UDateFormat, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> *const i8);
    unsafe { udat_getLocaleByType(fmt, r#type, status as _) }
}
#[inline]
pub unsafe fn udat_getNumberFormat(fmt: *const UDateFormat) -> *const UNumberFormat {
    windows_core::link!("icuin.dll" "C" fn udat_getNumberFormat(fmt : *const UDateFormat) -> *const UNumberFormat);
    unsafe { udat_getNumberFormat(fmt) }
}
#[inline]
pub unsafe fn udat_getNumberFormatForField(fmt: *const UDateFormat, field: UChar) -> *const UNumberFormat {
    windows_core::link!("icuin.dll" "C" fn udat_getNumberFormatForField(fmt : *const UDateFormat, field : UChar) -> *const UNumberFormat);
    unsafe { udat_getNumberFormatForField(fmt, field) }
}
#[inline]
pub unsafe fn udat_getSymbols(fmt: *const UDateFormat, r#type: UDateFormatSymbolType, symbolindex: i32, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_getSymbols(fmt : *const UDateFormat, r#type : UDateFormatSymbolType, symbolindex : i32, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { udat_getSymbols(fmt, r#type, symbolindex, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn udat_isLenient(fmt: *const UDateFormat) -> UBool {
    windows_core::link!("icuin.dll" "C" fn udat_isLenient(fmt : *const UDateFormat) -> UBool);
    unsafe { udat_isLenient(fmt) }
}
#[inline]
pub unsafe fn udat_open(timestyle: UDateFormatStyle, datestyle: UDateFormatStyle, locale: *const i8, tzid: *const UChar, tzidlength: i32, pattern: *const UChar, patternlength: i32, status: *mut UErrorCode) -> *mut UDateFormat {
    windows_core::link!("icuin.dll" "C" fn udat_open(timestyle : UDateFormatStyle, datestyle : UDateFormatStyle, locale : *const i8, tzid : *const UChar, tzidlength : i32, pattern : *const UChar, patternlength : i32, status : *mut UErrorCode) -> *mut UDateFormat);
    unsafe { udat_open(timestyle, datestyle, locale, tzid, tzidlength, pattern, patternlength, status as _) }
}
#[inline]
pub unsafe fn udat_parse(format: *const UDateFormat, text: *const UChar, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn udat_parse(format : *const UDateFormat, text : *const UChar, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> f64);
    unsafe { udat_parse(format, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn udat_parseCalendar(format: *const UDateFormat, calendar: *mut UCalendar, text: *const UChar, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_parseCalendar(format : *const UDateFormat, calendar : *mut UCalendar, text : *const UChar, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode));
    unsafe { udat_parseCalendar(format, calendar as _, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn udat_set2DigitYearStart(fmt: *mut UDateFormat, d: f64, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_set2DigitYearStart(fmt : *mut UDateFormat, d : f64, status : *mut UErrorCode));
    unsafe { udat_set2DigitYearStart(fmt as _, d, status as _) }
}
#[inline]
pub unsafe fn udat_setBooleanAttribute(fmt: *mut UDateFormat, attr: UDateFormatBooleanAttribute, newvalue: UBool, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_setBooleanAttribute(fmt : *mut UDateFormat, attr : UDateFormatBooleanAttribute, newvalue : UBool, status : *mut UErrorCode));
    unsafe { udat_setBooleanAttribute(fmt as _, attr, newvalue, status as _) }
}
#[inline]
pub unsafe fn udat_setCalendar(fmt: *mut UDateFormat, calendartoset: *const UCalendar) {
    windows_core::link!("icuin.dll" "C" fn udat_setCalendar(fmt : *mut UDateFormat, calendartoset : *const UCalendar));
    unsafe { udat_setCalendar(fmt as _, calendartoset) }
}
#[inline]
pub unsafe fn udat_setContext(fmt: *mut UDateFormat, value: UDisplayContext, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_setContext(fmt : *mut UDateFormat, value : UDisplayContext, status : *mut UErrorCode));
    unsafe { udat_setContext(fmt as _, value, status as _) }
}
#[inline]
pub unsafe fn udat_setLenient(fmt: *mut UDateFormat, islenient: UBool) {
    windows_core::link!("icuin.dll" "C" fn udat_setLenient(fmt : *mut UDateFormat, islenient : UBool));
    unsafe { udat_setLenient(fmt as _, islenient) }
}
#[inline]
pub unsafe fn udat_setNumberFormat(fmt: *mut UDateFormat, numberformattoset: *const UNumberFormat) {
    windows_core::link!("icuin.dll" "C" fn udat_setNumberFormat(fmt : *mut UDateFormat, numberformattoset : *const UNumberFormat));
    unsafe { udat_setNumberFormat(fmt as _, numberformattoset) }
}
#[inline]
pub unsafe fn udat_setSymbols(format: *mut UDateFormat, r#type: UDateFormatSymbolType, symbolindex: i32, value: *mut UChar, valuelength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_setSymbols(format : *mut UDateFormat, r#type : UDateFormatSymbolType, symbolindex : i32, value : *mut UChar, valuelength : i32, status : *mut UErrorCode));
    unsafe { udat_setSymbols(format as _, r#type, symbolindex, value as _, valuelength, status as _) }
}
#[inline]
pub unsafe fn udat_toCalendarDateField(field: UDateFormatField) -> UCalendarDateFields {
    windows_core::link!("icuin.dll" "C" fn udat_toCalendarDateField(field : UDateFormatField) -> UCalendarDateFields);
    unsafe { udat_toCalendarDateField(field) }
}
#[inline]
pub unsafe fn udat_toPattern(fmt: *const UDateFormat, localized: UBool, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_toPattern(fmt : *const UDateFormat, localized : UBool, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { udat_toPattern(fmt, localized, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn udatpg_addPattern(dtpg: *mut UDateTimePatternGenerator, pattern: *const UChar, patternlength: i32, r#override: UBool, conflictingpattern: *mut UChar, capacity: i32, plength: *mut i32, perrorcode: *mut UErrorCode) -> UDateTimePatternConflict {
    windows_core::link!("icuin.dll" "C" fn udatpg_addPattern(dtpg : *mut UDateTimePatternGenerator, pattern : *const UChar, patternlength : i32, r#override : UBool, conflictingpattern : *mut UChar, capacity : i32, plength : *mut i32, perrorcode : *mut UErrorCode) -> UDateTimePatternConflict);
    unsafe { udatpg_addPattern(dtpg as _, pattern, patternlength, r#override, conflictingpattern as _, capacity, plength as _, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_clone(dtpg: *const UDateTimePatternGenerator, perrorcode: *mut UErrorCode) -> *mut UDateTimePatternGenerator {
    windows_core::link!("icuin.dll" "C" fn udatpg_clone(dtpg : *const UDateTimePatternGenerator, perrorcode : *mut UErrorCode) -> *mut UDateTimePatternGenerator);
    unsafe { udatpg_clone(dtpg, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_close() -> UDateTimePatternGenerator {
    windows_core::link!("icuin.dll" "C" fn udatpg_close(dtpg : *mut UDateTimePatternGenerator));
    unsafe {
        let mut result__ = core::mem::zeroed();
        udatpg_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn udatpg_getAppendItemFormat(dtpg: *const UDateTimePatternGenerator, field: UDateTimePatternField, plength: *mut i32) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn udatpg_getAppendItemFormat(dtpg : *const UDateTimePatternGenerator, field : UDateTimePatternField, plength : *mut i32) -> *const UChar);
    unsafe { udatpg_getAppendItemFormat(dtpg, field, plength as _) }
}
#[inline]
pub unsafe fn udatpg_getAppendItemName(dtpg: *const UDateTimePatternGenerator, field: UDateTimePatternField, plength: *mut i32) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn udatpg_getAppendItemName(dtpg : *const UDateTimePatternGenerator, field : UDateTimePatternField, plength : *mut i32) -> *const UChar);
    unsafe { udatpg_getAppendItemName(dtpg, field, plength as _) }
}
#[inline]
pub unsafe fn udatpg_getBaseSkeleton(unuseddtpg: *mut UDateTimePatternGenerator, pattern: *const UChar, length: i32, baseskeleton: *mut UChar, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getBaseSkeleton(unuseddtpg : *mut UDateTimePatternGenerator, pattern : *const UChar, length : i32, baseskeleton : *mut UChar, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_getBaseSkeleton(unuseddtpg as _, pattern, length, baseskeleton as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_getBestPattern(dtpg: *mut UDateTimePatternGenerator, skeleton: *const UChar, length: i32, bestpattern: *mut UChar, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getBestPattern(dtpg : *mut UDateTimePatternGenerator, skeleton : *const UChar, length : i32, bestpattern : *mut UChar, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_getBestPattern(dtpg as _, skeleton, length, bestpattern as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_getBestPatternWithOptions(dtpg: *mut UDateTimePatternGenerator, skeleton: *const UChar, length: i32, options: UDateTimePatternMatchOptions, bestpattern: *mut UChar, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getBestPatternWithOptions(dtpg : *mut UDateTimePatternGenerator, skeleton : *const UChar, length : i32, options : UDateTimePatternMatchOptions, bestpattern : *mut UChar, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_getBestPatternWithOptions(dtpg as _, skeleton, length, options, bestpattern as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_getDateTimeFormat(dtpg: *const UDateTimePatternGenerator, plength: *mut i32) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn udatpg_getDateTimeFormat(dtpg : *const UDateTimePatternGenerator, plength : *mut i32) -> *const UChar);
    unsafe { udatpg_getDateTimeFormat(dtpg, plength as _) }
}
#[inline]
pub unsafe fn udatpg_getDecimal(dtpg: *const UDateTimePatternGenerator, plength: *mut i32) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn udatpg_getDecimal(dtpg : *const UDateTimePatternGenerator, plength : *mut i32) -> *const UChar);
    unsafe { udatpg_getDecimal(dtpg, plength as _) }
}
#[inline]
pub unsafe fn udatpg_getDefaultHourCycle(dtpg: *const UDateTimePatternGenerator, perrorcode: *mut UErrorCode) -> UDateFormatHourCycle {
    windows_core::link!("icu.dll" "C" fn udatpg_getDefaultHourCycle(dtpg : *const UDateTimePatternGenerator, perrorcode : *mut UErrorCode) -> UDateFormatHourCycle);
    unsafe { udatpg_getDefaultHourCycle(dtpg, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_getFieldDisplayName(dtpg: *const UDateTimePatternGenerator, field: UDateTimePatternField, width: UDateTimePGDisplayWidth, fieldname: *mut UChar, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn udatpg_getFieldDisplayName(dtpg : *const UDateTimePatternGenerator, field : UDateTimePatternField, width : UDateTimePGDisplayWidth, fieldname : *mut UChar, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_getFieldDisplayName(dtpg, field, width, fieldname as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_getPatternForSkeleton(dtpg: *const UDateTimePatternGenerator, skeleton: *const UChar, skeletonlength: i32, plength: *mut i32) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn udatpg_getPatternForSkeleton(dtpg : *const UDateTimePatternGenerator, skeleton : *const UChar, skeletonlength : i32, plength : *mut i32) -> *const UChar);
    unsafe { udatpg_getPatternForSkeleton(dtpg, skeleton, skeletonlength, plength as _) }
}
#[inline]
pub unsafe fn udatpg_getSkeleton(unuseddtpg: *mut UDateTimePatternGenerator, pattern: *const UChar, length: i32, skeleton: *mut UChar, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getSkeleton(unuseddtpg : *mut UDateTimePatternGenerator, pattern : *const UChar, length : i32, skeleton : *mut UChar, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_getSkeleton(unuseddtpg as _, pattern, length, skeleton as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_open(locale: *const i8, perrorcode: *mut UErrorCode) -> *mut UDateTimePatternGenerator {
    windows_core::link!("icuin.dll" "C" fn udatpg_open(locale : *const i8, perrorcode : *mut UErrorCode) -> *mut UDateTimePatternGenerator);
    unsafe { udatpg_open(locale, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_openBaseSkeletons(dtpg: *const UDateTimePatternGenerator, perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn udatpg_openBaseSkeletons(dtpg : *const UDateTimePatternGenerator, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { udatpg_openBaseSkeletons(dtpg, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_openEmpty(perrorcode: *mut UErrorCode) -> *mut UDateTimePatternGenerator {
    windows_core::link!("icuin.dll" "C" fn udatpg_openEmpty(perrorcode : *mut UErrorCode) -> *mut UDateTimePatternGenerator);
    unsafe { udatpg_openEmpty(perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_openSkeletons(dtpg: *const UDateTimePatternGenerator, perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn udatpg_openSkeletons(dtpg : *const UDateTimePatternGenerator, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { udatpg_openSkeletons(dtpg, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_replaceFieldTypes(dtpg: *mut UDateTimePatternGenerator, pattern: *const UChar, patternlength: i32, skeleton: *const UChar, skeletonlength: i32, dest: *mut UChar, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_replaceFieldTypes(dtpg : *mut UDateTimePatternGenerator, pattern : *const UChar, patternlength : i32, skeleton : *const UChar, skeletonlength : i32, dest : *mut UChar, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_replaceFieldTypes(dtpg as _, pattern, patternlength, skeleton, skeletonlength, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_replaceFieldTypesWithOptions(dtpg: *mut UDateTimePatternGenerator, pattern: *const UChar, patternlength: i32, skeleton: *const UChar, skeletonlength: i32, options: UDateTimePatternMatchOptions, dest: *mut UChar, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_replaceFieldTypesWithOptions(dtpg : *mut UDateTimePatternGenerator, pattern : *const UChar, patternlength : i32, skeleton : *const UChar, skeletonlength : i32, options : UDateTimePatternMatchOptions, dest : *mut UChar, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_replaceFieldTypesWithOptions(dtpg as _, pattern, patternlength, skeleton, skeletonlength, options, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_setAppendItemFormat(dtpg: *mut UDateTimePatternGenerator, field: UDateTimePatternField, value: *const UChar, length: i32) {
    windows_core::link!("icuin.dll" "C" fn udatpg_setAppendItemFormat(dtpg : *mut UDateTimePatternGenerator, field : UDateTimePatternField, value : *const UChar, length : i32));
    unsafe { udatpg_setAppendItemFormat(dtpg as _, field, value, length) }
}
#[inline]
pub unsafe fn udatpg_setAppendItemName(dtpg: *mut UDateTimePatternGenerator, field: UDateTimePatternField, value: *const UChar, length: i32) {
    windows_core::link!("icuin.dll" "C" fn udatpg_setAppendItemName(dtpg : *mut UDateTimePatternGenerator, field : UDateTimePatternField, value : *const UChar, length : i32));
    unsafe { udatpg_setAppendItemName(dtpg as _, field, value, length) }
}
#[inline]
pub unsafe fn udatpg_setDateTimeFormat(dtpg: *const UDateTimePatternGenerator, dtformat: *const UChar, length: i32) {
    windows_core::link!("icuin.dll" "C" fn udatpg_setDateTimeFormat(dtpg : *const UDateTimePatternGenerator, dtformat : *const UChar, length : i32));
    unsafe { udatpg_setDateTimeFormat(dtpg, dtformat, length) }
}
#[inline]
pub unsafe fn udatpg_setDecimal(dtpg: *mut UDateTimePatternGenerator, decimal: *const UChar, length: i32) {
    windows_core::link!("icuin.dll" "C" fn udatpg_setDecimal(dtpg : *mut UDateTimePatternGenerator, decimal : *const UChar, length : i32));
    unsafe { udatpg_setDecimal(dtpg as _, decimal, length) }
}
#[inline]
pub unsafe fn udtitvfmt_close() -> UDateIntervalFormat {
    windows_core::link!("icuin.dll" "C" fn udtitvfmt_close(formatter : *mut UDateIntervalFormat));
    unsafe {
        let mut result__ = core::mem::zeroed();
        udtitvfmt_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn udtitvfmt_closeResult() -> UFormattedDateInterval {
    windows_core::link!("icu.dll" "C" fn udtitvfmt_closeResult(uresult : *mut UFormattedDateInterval));
    unsafe {
        let mut result__ = core::mem::zeroed();
        udtitvfmt_closeResult(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn udtitvfmt_format(formatter: *const UDateIntervalFormat, fromdate: f64, todate: f64, result: *mut UChar, resultcapacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udtitvfmt_format(formatter : *const UDateIntervalFormat, fromdate : f64, todate : f64, result : *mut UChar, resultcapacity : i32, position : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { udtitvfmt_format(formatter, fromdate, todate, result as _, resultcapacity, position as _, status as _) }
}
#[inline]
pub unsafe fn udtitvfmt_formatToResult(formatter: *const UDateIntervalFormat, fromdate: f64, todate: f64, result: *mut UFormattedDateInterval, status: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn udtitvfmt_formatToResult(formatter : *const UDateIntervalFormat, fromdate : f64, todate : f64, result : *mut UFormattedDateInterval, status : *mut UErrorCode));
    unsafe { udtitvfmt_formatToResult(formatter, fromdate, todate, result as _, status as _) }
}
#[inline]
pub unsafe fn udtitvfmt_getContext(formatter: *const UDateIntervalFormat, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext {
    windows_core::link!("icu.dll" "C" fn udtitvfmt_getContext(formatter : *const UDateIntervalFormat, r#type : UDisplayContextType, status : *mut UErrorCode) -> UDisplayContext);
    unsafe { udtitvfmt_getContext(formatter, r#type, status as _) }
}
#[inline]
pub unsafe fn udtitvfmt_open(locale: *const i8, skeleton: *const UChar, skeletonlength: i32, tzid: *const UChar, tzidlength: i32, status: *mut UErrorCode) -> *mut UDateIntervalFormat {
    windows_core::link!("icuin.dll" "C" fn udtitvfmt_open(locale : *const i8, skeleton : *const UChar, skeletonlength : i32, tzid : *const UChar, tzidlength : i32, status : *mut UErrorCode) -> *mut UDateIntervalFormat);
    unsafe { udtitvfmt_open(locale, skeleton, skeletonlength, tzid, tzidlength, status as _) }
}
#[inline]
pub unsafe fn udtitvfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedDateInterval {
    windows_core::link!("icu.dll" "C" fn udtitvfmt_openResult(ec : *mut UErrorCode) -> *mut UFormattedDateInterval);
    unsafe { udtitvfmt_openResult(ec as _) }
}
#[inline]
pub unsafe fn udtitvfmt_resultAsValue(uresult: *const UFormattedDateInterval, ec: *mut UErrorCode) -> *const UFormattedValue {
    windows_core::link!("icu.dll" "C" fn udtitvfmt_resultAsValue(uresult : *const UFormattedDateInterval, ec : *mut UErrorCode) -> *const UFormattedValue);
    unsafe { udtitvfmt_resultAsValue(uresult, ec as _) }
}
#[inline]
pub unsafe fn udtitvfmt_setContext(formatter: *mut UDateIntervalFormat, value: UDisplayContext, status: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn udtitvfmt_setContext(formatter : *mut UDateIntervalFormat, value : UDisplayContext, status : *mut UErrorCode));
    unsafe { udtitvfmt_setContext(formatter as _, value, status as _) }
}
#[inline]
pub unsafe fn uenum_close() -> UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn uenum_close(en : *mut UEnumeration));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uenum_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uenum_count(en: *mut UEnumeration, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uenum_count(en : *mut UEnumeration, status : *mut UErrorCode) -> i32);
    unsafe { uenum_count(en as _, status as _) }
}
#[inline]
pub unsafe fn uenum_next(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uenum_next(en : *mut UEnumeration, resultlength : *mut i32, status : *mut UErrorCode) -> *const i8);
    unsafe { uenum_next(en as _, resultlength as _, status as _) }
}
#[inline]
pub unsafe fn uenum_openCharStringsEnumeration(strings: *mut *mut i8, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn uenum_openCharStringsEnumeration(strings : *mut *mut i8, count : i32, ec : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uenum_openCharStringsEnumeration(strings as _, count, ec as _) }
}
#[inline]
pub unsafe fn uenum_openUCharStringsEnumeration(strings: *mut *mut UChar, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn uenum_openUCharStringsEnumeration(strings : *mut *mut UChar, count : i32, ec : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uenum_openUCharStringsEnumeration(strings as _, count, ec as _) }
}
#[inline]
pub unsafe fn uenum_reset(en: *mut UEnumeration, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn uenum_reset(en : *mut UEnumeration, status : *mut UErrorCode));
    unsafe { uenum_reset(en as _, status as _) }
}
#[inline]
pub unsafe fn uenum_unext(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuuc.dll" "C" fn uenum_unext(en : *mut UEnumeration, resultlength : *mut i32, status : *mut UErrorCode) -> *const UChar);
    unsafe { uenum_unext(en as _, resultlength as _, status as _) }
}
#[inline]
pub unsafe fn ufieldpositer_close() -> UFieldPositionIterator {
    windows_core::link!("icuin.dll" "C" fn ufieldpositer_close(fpositer : *mut UFieldPositionIterator));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ufieldpositer_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ufieldpositer_next(fpositer: *mut UFieldPositionIterator, beginindex: *mut i32, endindex: *mut i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ufieldpositer_next(fpositer : *mut UFieldPositionIterator, beginindex : *mut i32, endindex : *mut i32) -> i32);
    unsafe { ufieldpositer_next(fpositer as _, beginindex as _, endindex as _) }
}
#[inline]
pub unsafe fn ufieldpositer_open(status: *mut UErrorCode) -> *mut UFieldPositionIterator {
    windows_core::link!("icuin.dll" "C" fn ufieldpositer_open(status : *mut UErrorCode) -> *mut UFieldPositionIterator);
    unsafe { ufieldpositer_open(status as _) }
}
#[inline]
pub unsafe fn ufmt_close() -> UFormattable {
    windows_core::link!("icuin.dll" "C" fn ufmt_close(fmt : *mut UFormattable));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ufmt_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ufmt_getArrayItemByIndex(fmt: *mut UFormattable, n: i32, status: *mut UErrorCode) -> *mut UFormattable {
    windows_core::link!("icuin.dll" "C" fn ufmt_getArrayItemByIndex(fmt : *mut UFormattable, n : i32, status : *mut UErrorCode) -> *mut UFormattable);
    unsafe { ufmt_getArrayItemByIndex(fmt as _, n, status as _) }
}
#[inline]
pub unsafe fn ufmt_getArrayLength(fmt: *const UFormattable, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getArrayLength(fmt : *const UFormattable, status : *mut UErrorCode) -> i32);
    unsafe { ufmt_getArrayLength(fmt, status as _) }
}
#[inline]
pub unsafe fn ufmt_getDate(fmt: *const UFormattable, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getDate(fmt : *const UFormattable, status : *mut UErrorCode) -> f64);
    unsafe { ufmt_getDate(fmt, status as _) }
}
#[inline]
pub unsafe fn ufmt_getDecNumChars(fmt: *mut UFormattable, len: *mut i32, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getDecNumChars(fmt : *mut UFormattable, len : *mut i32, status : *mut UErrorCode) -> *const i8);
    unsafe { ufmt_getDecNumChars(fmt as _, len as _, status as _) }
}
#[inline]
pub unsafe fn ufmt_getDouble(fmt: *mut UFormattable, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getDouble(fmt : *mut UFormattable, status : *mut UErrorCode) -> f64);
    unsafe { ufmt_getDouble(fmt as _, status as _) }
}
#[inline]
pub unsafe fn ufmt_getInt64(fmt: *mut UFormattable, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getInt64(fmt : *mut UFormattable, status : *mut UErrorCode) -> i64);
    unsafe { ufmt_getInt64(fmt as _, status as _) }
}
#[inline]
pub unsafe fn ufmt_getLong(fmt: *mut UFormattable, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getLong(fmt : *mut UFormattable, status : *mut UErrorCode) -> i32);
    unsafe { ufmt_getLong(fmt as _, status as _) }
}
#[inline]
pub unsafe fn ufmt_getObject(fmt: *const UFormattable, status: *mut UErrorCode) -> *const core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn ufmt_getObject(fmt : *const UFormattable, status : *mut UErrorCode) -> *const core::ffi::c_void);
    unsafe { ufmt_getObject(fmt, status as _) }
}
#[inline]
pub unsafe fn ufmt_getType(fmt: *const UFormattable, status: *mut UErrorCode) -> UFormattableType {
    windows_core::link!("icuin.dll" "C" fn ufmt_getType(fmt : *const UFormattable, status : *mut UErrorCode) -> UFormattableType);
    unsafe { ufmt_getType(fmt, status as _) }
}
#[inline]
pub unsafe fn ufmt_getUChars(fmt: *mut UFormattable, len: *mut i32, status: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn ufmt_getUChars(fmt : *mut UFormattable, len : *mut i32, status : *mut UErrorCode) -> *const UChar);
    unsafe { ufmt_getUChars(fmt as _, len as _, status as _) }
}
#[inline]
pub unsafe fn ufmt_isNumeric(fmt: *const UFormattable) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ufmt_isNumeric(fmt : *const UFormattable) -> UBool);
    unsafe { ufmt_isNumeric(fmt) }
}
#[inline]
pub unsafe fn ufmt_open(status: *mut UErrorCode) -> *mut UFormattable {
    windows_core::link!("icuin.dll" "C" fn ufmt_open(status : *mut UErrorCode) -> *mut UFormattable);
    unsafe { ufmt_open(status as _) }
}
#[inline]
pub unsafe fn ufmtval_getString(ufmtval: *const UFormattedValue, plength: *mut i32, ec: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icu.dll" "C" fn ufmtval_getString(ufmtval : *const UFormattedValue, plength : *mut i32, ec : *mut UErrorCode) -> *const UChar);
    unsafe { ufmtval_getString(ufmtval, plength as _, ec as _) }
}
#[inline]
pub unsafe fn ufmtval_nextPosition(ufmtval: *const UFormattedValue, ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode) -> UBool {
    windows_core::link!("icu.dll" "C" fn ufmtval_nextPosition(ufmtval : *const UFormattedValue, ucfpos : *mut UConstrainedFieldPosition, ec : *mut UErrorCode) -> UBool);
    unsafe { ufmtval_nextPosition(ufmtval, ucfpos as _, ec as _) }
}
#[inline]
pub unsafe fn ugender_getInstance(locale: *const i8, status: *mut UErrorCode) -> *const UGenderInfo {
    windows_core::link!("icuin.dll" "C" fn ugender_getInstance(locale : *const i8, status : *mut UErrorCode) -> *const UGenderInfo);
    unsafe { ugender_getInstance(locale, status as _) }
}
#[inline]
pub unsafe fn ugender_getListGender(genderinfo: *const UGenderInfo, genders: *const UGender, size: i32, status: *mut UErrorCode) -> UGender {
    windows_core::link!("icuin.dll" "C" fn ugender_getListGender(genderinfo : *const UGenderInfo, genders : *const UGender, size : i32, status : *mut UErrorCode) -> UGender);
    unsafe { ugender_getListGender(genderinfo, genders, size, status as _) }
}
#[inline]
pub unsafe fn uidna_close() -> UIDNA {
    windows_core::link!("icuuc.dll" "C" fn uidna_close(idna : *mut UIDNA));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uidna_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uidna_labelToASCII(idna: *const UIDNA, label: *const UChar, length: i32, dest: *mut UChar, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_labelToASCII(idna : *const UIDNA, label : *const UChar, length : i32, dest : *mut UChar, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_labelToASCII(idna, label, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_labelToASCII_UTF8(idna: *const UIDNA, label: *const i8, length: i32, dest: *mut i8, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_labelToASCII_UTF8(idna : *const UIDNA, label : *const i8, length : i32, dest : *mut i8, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_labelToASCII_UTF8(idna, label, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_labelToUnicode(idna: *const UIDNA, label: *const UChar, length: i32, dest: *mut UChar, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_labelToUnicode(idna : *const UIDNA, label : *const UChar, length : i32, dest : *mut UChar, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_labelToUnicode(idna, label, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_labelToUnicodeUTF8(idna: *const UIDNA, label: *const i8, length: i32, dest: *mut i8, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_labelToUnicodeUTF8(idna : *const UIDNA, label : *const i8, length : i32, dest : *mut i8, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_labelToUnicodeUTF8(idna, label, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_nameToASCII(idna: *const UIDNA, name: *const UChar, length: i32, dest: *mut UChar, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_nameToASCII(idna : *const UIDNA, name : *const UChar, length : i32, dest : *mut UChar, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_nameToASCII(idna, name, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_nameToASCII_UTF8(idna: *const UIDNA, name: *const i8, length: i32, dest: *mut i8, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_nameToASCII_UTF8(idna : *const UIDNA, name : *const i8, length : i32, dest : *mut i8, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_nameToASCII_UTF8(idna, name, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_nameToUnicode(idna: *const UIDNA, name: *const UChar, length: i32, dest: *mut UChar, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_nameToUnicode(idna : *const UIDNA, name : *const UChar, length : i32, dest : *mut UChar, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_nameToUnicode(idna, name, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_nameToUnicodeUTF8(idna: *const UIDNA, name: *const i8, length: i32, dest: *mut i8, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_nameToUnicodeUTF8(idna : *const UIDNA, name : *const i8, length : i32, dest : *mut i8, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_nameToUnicodeUTF8(idna, name, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_openUTS46(options: u32, perrorcode: *mut UErrorCode) -> *mut UIDNA {
    windows_core::link!("icuuc.dll" "C" fn uidna_openUTS46(options : u32, perrorcode : *mut UErrorCode) -> *mut UIDNA);
    unsafe { uidna_openUTS46(options, perrorcode as _) }
}
#[inline]
pub unsafe fn uiter_current32(iter: *mut UCharIterator) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn uiter_current32(iter : *mut UCharIterator) -> UChar32);
    unsafe { uiter_current32(iter as _) }
}
#[inline]
pub unsafe fn uiter_getState(iter: *const UCharIterator) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn uiter_getState(iter : *const UCharIterator) -> u32);
    unsafe { uiter_getState(iter) }
}
#[inline]
pub unsafe fn uiter_next32(iter: *mut UCharIterator) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn uiter_next32(iter : *mut UCharIterator) -> UChar32);
    unsafe { uiter_next32(iter as _) }
}
#[inline]
pub unsafe fn uiter_previous32(iter: *mut UCharIterator) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn uiter_previous32(iter : *mut UCharIterator) -> UChar32);
    unsafe { uiter_previous32(iter as _) }
}
#[inline]
pub unsafe fn uiter_setState(iter: *mut UCharIterator, state: u32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn uiter_setState(iter : *mut UCharIterator, state : u32, perrorcode : *mut UErrorCode));
    unsafe { uiter_setState(iter as _, state, perrorcode as _) }
}
#[inline]
pub unsafe fn uiter_setString(iter: *mut UCharIterator, s: *const UChar, length: i32) {
    windows_core::link!("icuuc.dll" "C" fn uiter_setString(iter : *mut UCharIterator, s : *const UChar, length : i32));
    unsafe { uiter_setString(iter as _, s, length) }
}
#[inline]
pub unsafe fn uiter_setUTF16BE(iter: *mut UCharIterator, s: *const i8, length: i32) {
    windows_core::link!("icuuc.dll" "C" fn uiter_setUTF16BE(iter : *mut UCharIterator, s : *const i8, length : i32));
    unsafe { uiter_setUTF16BE(iter as _, s, length) }
}
#[inline]
pub unsafe fn uiter_setUTF8(iter: *mut UCharIterator, s: *const i8, length: i32) {
    windows_core::link!("icuuc.dll" "C" fn uiter_setUTF8(iter : *mut UCharIterator, s : *const i8, length : i32));
    unsafe { uiter_setUTF8(iter as _, s, length) }
}
#[inline]
pub unsafe fn uldn_close() -> ULocaleDisplayNames {
    windows_core::link!("icuuc.dll" "C" fn uldn_close(ldn : *mut ULocaleDisplayNames));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uldn_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uldn_getContext(ldn: *const ULocaleDisplayNames, r#type: UDisplayContextType, perrorcode: *mut UErrorCode) -> UDisplayContext {
    windows_core::link!("icuuc.dll" "C" fn uldn_getContext(ldn : *const ULocaleDisplayNames, r#type : UDisplayContextType, perrorcode : *mut UErrorCode) -> UDisplayContext);
    unsafe { uldn_getContext(ldn, r#type, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_getDialectHandling(ldn: *const ULocaleDisplayNames) -> UDialectHandling {
    windows_core::link!("icuuc.dll" "C" fn uldn_getDialectHandling(ldn : *const ULocaleDisplayNames) -> UDialectHandling);
    unsafe { uldn_getDialectHandling(ldn) }
}
#[inline]
pub unsafe fn uldn_getLocale(ldn: *const ULocaleDisplayNames) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uldn_getLocale(ldn : *const ULocaleDisplayNames) -> *const i8);
    unsafe { uldn_getLocale(ldn) }
}
#[inline]
pub unsafe fn uldn_keyDisplayName(ldn: *const ULocaleDisplayNames, key: *const i8, result: *mut UChar, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uldn_keyDisplayName(ldn : *const ULocaleDisplayNames, key : *const i8, result : *mut UChar, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_keyDisplayName(ldn, key, result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_keyValueDisplayName(ldn: *const ULocaleDisplayNames, key: *const i8, value: *const i8, result: *mut UChar, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uldn_keyValueDisplayName(ldn : *const ULocaleDisplayNames, key : *const i8, value : *const i8, result : *mut UChar, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_keyValueDisplayName(ldn, key, value, result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_languageDisplayName(ldn: *const ULocaleDisplayNames, lang: *const i8, result: *mut UChar, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uldn_languageDisplayName(ldn : *const ULocaleDisplayNames, lang : *const i8, result : *mut UChar, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_languageDisplayName(ldn, lang, result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_localeDisplayName(ldn: *const ULocaleDisplayNames, locale: *const i8, result: *mut UChar, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uldn_localeDisplayName(ldn : *const ULocaleDisplayNames, locale : *const i8, result : *mut UChar, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_localeDisplayName(ldn, locale, result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_open(locale: *const i8, dialecthandling: UDialectHandling, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames {
    windows_core::link!("icuuc.dll" "C" fn uldn_open(locale : *const i8, dialecthandling : UDialectHandling, perrorcode : *mut UErrorCode) -> *mut ULocaleDisplayNames);
    unsafe { uldn_open(locale, dialecthandling, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_openForContext(locale: *const i8, contexts: *mut UDisplayContext, length: i32, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames {
    windows_core::link!("icuuc.dll" "C" fn uldn_openForContext(locale : *const i8, contexts : *mut UDisplayContext, length : i32, perrorcode : *mut UErrorCode) -> *mut ULocaleDisplayNames);
    unsafe { uldn_openForContext(locale, contexts as _, length, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_regionDisplayName(ldn: *const ULocaleDisplayNames, region: *const i8, result: *mut UChar, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uldn_regionDisplayName(ldn : *const ULocaleDisplayNames, region : *const i8, result : *mut UChar, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_regionDisplayName(ldn, region, result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_scriptCodeDisplayName(ldn: *const ULocaleDisplayNames, scriptcode: UScriptCode, result: *mut UChar, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uldn_scriptCodeDisplayName(ldn : *const ULocaleDisplayNames, scriptcode : UScriptCode, result : *mut UChar, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_scriptCodeDisplayName(ldn, scriptcode, result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_scriptDisplayName(ldn: *const ULocaleDisplayNames, script: *const i8, result: *mut UChar, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uldn_scriptDisplayName(ldn : *const ULocaleDisplayNames, script : *const i8, result : *mut UChar, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_scriptDisplayName(ldn, script, result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_variantDisplayName(ldn: *const ULocaleDisplayNames, variant: *const i8, result: *mut UChar, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uldn_variantDisplayName(ldn : *const ULocaleDisplayNames, variant : *const i8, result : *mut UChar, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_variantDisplayName(ldn, variant, result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn ulistfmt_close() -> UListFormatter {
    windows_core::link!("icuuc.dll" "C" fn ulistfmt_close(listfmt : *mut UListFormatter));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ulistfmt_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ulistfmt_closeResult() -> UFormattedList {
    windows_core::link!("icu.dll" "C" fn ulistfmt_closeResult(uresult : *mut UFormattedList));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ulistfmt_closeResult(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ulistfmt_format(listfmt: *const UListFormatter, strings: *mut *mut UChar, stringlengths: *const i32, stringcount: i32, result: *mut UChar, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ulistfmt_format(listfmt : *const UListFormatter, strings : *mut *mut UChar, stringlengths : *const i32, stringcount : i32, result : *mut UChar, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ulistfmt_format(listfmt, strings as _, stringlengths, stringcount, result as _, resultcapacity, status as _) }
}
#[inline]
pub unsafe fn ulistfmt_formatStringsToResult(listfmt: *const UListFormatter, strings: *mut *mut UChar, stringlengths: *const i32, stringcount: i32, uresult: *mut UFormattedList, status: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ulistfmt_formatStringsToResult(listfmt : *const UListFormatter, strings : *mut *mut UChar, stringlengths : *const i32, stringcount : i32, uresult : *mut UFormattedList, status : *mut UErrorCode));
    unsafe { ulistfmt_formatStringsToResult(listfmt, strings as _, stringlengths, stringcount, uresult as _, status as _) }
}
#[inline]
pub unsafe fn ulistfmt_open(locale: *const i8, status: *mut UErrorCode) -> *mut UListFormatter {
    windows_core::link!("icuuc.dll" "C" fn ulistfmt_open(locale : *const i8, status : *mut UErrorCode) -> *mut UListFormatter);
    unsafe { ulistfmt_open(locale, status as _) }
}
#[inline]
pub unsafe fn ulistfmt_openForType(locale: *const i8, r#type: UListFormatterType, width: UListFormatterWidth, status: *mut UErrorCode) -> *mut UListFormatter {
    windows_core::link!("icu.dll" "C" fn ulistfmt_openForType(locale : *const i8, r#type : UListFormatterType, width : UListFormatterWidth, status : *mut UErrorCode) -> *mut UListFormatter);
    unsafe { ulistfmt_openForType(locale, r#type, width, status as _) }
}
#[inline]
pub unsafe fn ulistfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedList {
    windows_core::link!("icu.dll" "C" fn ulistfmt_openResult(ec : *mut UErrorCode) -> *mut UFormattedList);
    unsafe { ulistfmt_openResult(ec as _) }
}
#[inline]
pub unsafe fn ulistfmt_resultAsValue(uresult: *const UFormattedList, ec: *mut UErrorCode) -> *const UFormattedValue {
    windows_core::link!("icu.dll" "C" fn ulistfmt_resultAsValue(uresult : *const UFormattedList, ec : *mut UErrorCode) -> *const UFormattedValue);
    unsafe { ulistfmt_resultAsValue(uresult, ec as _) }
}
#[inline]
pub unsafe fn uloc_acceptLanguage(result: *mut i8, resultavailable: i32, outresult: *mut UAcceptResult, acceptlist: *mut *mut i8, acceptlistcount: i32, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_acceptLanguage(result : *mut i8, resultavailable : i32, outresult : *mut UAcceptResult, acceptlist : *mut *mut i8, acceptlistcount : i32, availablelocales : *mut UEnumeration, status : *mut UErrorCode) -> i32);
    unsafe { uloc_acceptLanguage(result as _, resultavailable, outresult as _, acceptlist as _, acceptlistcount, availablelocales as _, status as _) }
}
#[inline]
pub unsafe fn uloc_acceptLanguageFromHTTP(result: *mut i8, resultavailable: i32, outresult: *mut UAcceptResult, httpacceptlanguage: *const i8, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_acceptLanguageFromHTTP(result : *mut i8, resultavailable : i32, outresult : *mut UAcceptResult, httpacceptlanguage : *const i8, availablelocales : *mut UEnumeration, status : *mut UErrorCode) -> i32);
    unsafe { uloc_acceptLanguageFromHTTP(result as _, resultavailable, outresult as _, httpacceptlanguage, availablelocales as _, status as _) }
}
#[inline]
pub unsafe fn uloc_addLikelySubtags(localeid: *const i8, maximizedlocaleid: *mut i8, maximizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_addLikelySubtags(localeid : *const i8, maximizedlocaleid : *mut i8, maximizedlocaleidcapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_addLikelySubtags(localeid, maximizedlocaleid as _, maximizedlocaleidcapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_canonicalize(localeid: *const i8, name: *mut i8, namecapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_canonicalize(localeid : *const i8, name : *mut i8, namecapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_canonicalize(localeid, name as _, namecapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_countAvailable() -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_countAvailable() -> i32);
    unsafe { uloc_countAvailable() }
}
#[inline]
pub unsafe fn uloc_forLanguageTag(langtag: *const i8, localeid: *mut i8, localeidcapacity: i32, parsedlength: *mut i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_forLanguageTag(langtag : *const i8, localeid : *mut i8, localeidcapacity : i32, parsedlength : *mut i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_forLanguageTag(langtag, localeid as _, localeidcapacity, parsedlength as _, err as _) }
}
#[inline]
pub unsafe fn uloc_getAvailable(n: i32) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getAvailable(n : i32) -> *const i8);
    unsafe { uloc_getAvailable(n) }
}
#[inline]
pub unsafe fn uloc_getBaseName(localeid: *const i8, name: *mut i8, namecapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getBaseName(localeid : *const i8, name : *mut i8, namecapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getBaseName(localeid, name as _, namecapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getCharacterOrientation(localeid: *const i8, status: *mut UErrorCode) -> ULayoutType {
    windows_core::link!("icuuc.dll" "C" fn uloc_getCharacterOrientation(localeid : *const i8, status : *mut UErrorCode) -> ULayoutType);
    unsafe { uloc_getCharacterOrientation(localeid, status as _) }
}
#[inline]
pub unsafe fn uloc_getCountry(localeid: *const i8, country: *mut i8, countrycapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getCountry(localeid : *const i8, country : *mut i8, countrycapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getCountry(localeid, country as _, countrycapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getDefault() -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getDefault() -> *const i8);
    unsafe { uloc_getDefault() }
}
#[inline]
pub unsafe fn uloc_getDisplayCountry(locale: *const i8, displaylocale: *const i8, country: *mut UChar, countrycapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayCountry(locale : *const i8, displaylocale : *const i8, country : *mut UChar, countrycapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayCountry(locale, displaylocale, country as _, countrycapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayKeyword(keyword: *const i8, displaylocale: *const i8, dest: *mut UChar, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayKeyword(keyword : *const i8, displaylocale : *const i8, dest : *mut UChar, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayKeyword(keyword, displaylocale, dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayKeywordValue(locale: *const i8, keyword: *const i8, displaylocale: *const i8, dest: *mut UChar, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayKeywordValue(locale : *const i8, keyword : *const i8, displaylocale : *const i8, dest : *mut UChar, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayKeywordValue(locale, keyword, displaylocale, dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayLanguage(locale: *const i8, displaylocale: *const i8, language: *mut UChar, languagecapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayLanguage(locale : *const i8, displaylocale : *const i8, language : *mut UChar, languagecapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayLanguage(locale, displaylocale, language as _, languagecapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayName(localeid: *const i8, inlocaleid: *const i8, result: *mut UChar, maxresultsize: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayName(localeid : *const i8, inlocaleid : *const i8, result : *mut UChar, maxresultsize : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayName(localeid, inlocaleid, result as _, maxresultsize, err as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayScript(locale: *const i8, displaylocale: *const i8, script: *mut UChar, scriptcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayScript(locale : *const i8, displaylocale : *const i8, script : *mut UChar, scriptcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayScript(locale, displaylocale, script as _, scriptcapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayVariant(locale: *const i8, displaylocale: *const i8, variant: *mut UChar, variantcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayVariant(locale : *const i8, displaylocale : *const i8, variant : *mut UChar, variantcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayVariant(locale, displaylocale, variant as _, variantcapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getISO3Country(localeid: *const i8) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getISO3Country(localeid : *const i8) -> *const i8);
    unsafe { uloc_getISO3Country(localeid) }
}
#[inline]
pub unsafe fn uloc_getISO3Language(localeid: *const i8) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getISO3Language(localeid : *const i8) -> *const i8);
    unsafe { uloc_getISO3Language(localeid) }
}
#[inline]
pub unsafe fn uloc_getISOCountries() -> *const *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getISOCountries() -> *const *const i8);
    unsafe { uloc_getISOCountries() }
}
#[inline]
pub unsafe fn uloc_getISOLanguages() -> *const *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getISOLanguages() -> *const *const i8);
    unsafe { uloc_getISOLanguages() }
}
#[inline]
pub unsafe fn uloc_getKeywordValue(localeid: *const i8, keywordname: *const i8, buffer: *mut i8, buffercapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getKeywordValue(localeid : *const i8, keywordname : *const i8, buffer : *mut i8, buffercapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getKeywordValue(localeid, keywordname, buffer as _, buffercapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getLCID(localeid: *const i8) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getLCID(localeid : *const i8) -> u32);
    unsafe { uloc_getLCID(localeid) }
}
#[inline]
pub unsafe fn uloc_getLanguage(localeid: *const i8, language: *mut i8, languagecapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getLanguage(localeid : *const i8, language : *mut i8, languagecapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getLanguage(localeid, language as _, languagecapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getLineOrientation(localeid: *const i8, status: *mut UErrorCode) -> ULayoutType {
    windows_core::link!("icuuc.dll" "C" fn uloc_getLineOrientation(localeid : *const i8, status : *mut UErrorCode) -> ULayoutType);
    unsafe { uloc_getLineOrientation(localeid, status as _) }
}
#[inline]
pub unsafe fn uloc_getLocaleForLCID(hostid: u32, locale: *mut i8, localecapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getLocaleForLCID(hostid : u32, locale : *mut i8, localecapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getLocaleForLCID(hostid, locale as _, localecapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getName(localeid: *const i8, name: *mut i8, namecapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getName(localeid : *const i8, name : *mut i8, namecapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getName(localeid, name as _, namecapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getParent(localeid: *const i8, parent: *mut i8, parentcapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getParent(localeid : *const i8, parent : *mut i8, parentcapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getParent(localeid, parent as _, parentcapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getScript(localeid: *const i8, script: *mut i8, scriptcapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getScript(localeid : *const i8, script : *mut i8, scriptcapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getScript(localeid, script as _, scriptcapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getVariant(localeid: *const i8, variant: *mut i8, variantcapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getVariant(localeid : *const i8, variant : *mut i8, variantcapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getVariant(localeid, variant as _, variantcapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_isRightToLeft(locale: *const i8) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uloc_isRightToLeft(locale : *const i8) -> UBool);
    unsafe { uloc_isRightToLeft(locale) }
}
#[inline]
pub unsafe fn uloc_minimizeSubtags(localeid: *const i8, minimizedlocaleid: *mut i8, minimizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_minimizeSubtags(localeid : *const i8, minimizedlocaleid : *mut i8, minimizedlocaleidcapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_minimizeSubtags(localeid, minimizedlocaleid as _, minimizedlocaleidcapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_openAvailableByType(r#type: ULocAvailableType, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icu.dll" "C" fn uloc_openAvailableByType(r#type : ULocAvailableType, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uloc_openAvailableByType(r#type, status as _) }
}
#[inline]
pub unsafe fn uloc_openKeywords(localeid: *const i8, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn uloc_openKeywords(localeid : *const i8, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uloc_openKeywords(localeid, status as _) }
}
#[inline]
pub unsafe fn uloc_setDefault(localeid: *const i8) -> UErrorCode {
    windows_core::link!("icuuc.dll" "C" fn uloc_setDefault(localeid : *const i8, status : *mut UErrorCode));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uloc_setDefault(localeid, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uloc_setKeywordValue(keywordname: *const i8, keywordvalue: *const i8, buffer: *mut i8, buffercapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_setKeywordValue(keywordname : *const i8, keywordvalue : *const i8, buffer : *mut i8, buffercapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_setKeywordValue(keywordname, keywordvalue, buffer as _, buffercapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_toLanguageTag(localeid: *const i8, langtag: *mut i8, langtagcapacity: i32, strict: UBool, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_toLanguageTag(localeid : *const i8, langtag : *mut i8, langtagcapacity : i32, strict : UBool, err : *mut UErrorCode) -> i32);
    unsafe { uloc_toLanguageTag(localeid, langtag as _, langtagcapacity, strict, err as _) }
}
#[inline]
pub unsafe fn uloc_toLegacyKey(keyword: *const i8) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_toLegacyKey(keyword : *const i8) -> *const i8);
    unsafe { uloc_toLegacyKey(keyword) }
}
#[inline]
pub unsafe fn uloc_toLegacyType(keyword: *const i8, value: *const i8) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_toLegacyType(keyword : *const i8, value : *const i8) -> *const i8);
    unsafe { uloc_toLegacyType(keyword, value) }
}
#[inline]
pub unsafe fn uloc_toUnicodeLocaleKey(keyword: *const i8) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_toUnicodeLocaleKey(keyword : *const i8) -> *const i8);
    unsafe { uloc_toUnicodeLocaleKey(keyword) }
}
#[inline]
pub unsafe fn uloc_toUnicodeLocaleType(keyword: *const i8, value: *const i8) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_toUnicodeLocaleType(keyword : *const i8, value : *const i8) -> *const i8);
    unsafe { uloc_toUnicodeLocaleType(keyword, value) }
}
#[inline]
pub unsafe fn ulocdata_close() -> ULocaleData {
    windows_core::link!("icuin.dll" "C" fn ulocdata_close(uld : *mut ULocaleData));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ulocdata_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ulocdata_getCLDRVersion(versionarray: *mut u8, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getCLDRVersion(versionarray : *mut u8, status : *mut UErrorCode));
    unsafe { ulocdata_getCLDRVersion(versionarray as _, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getDelimiter(uld: *mut ULocaleData, r#type: ULocaleDataDelimiterType, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getDelimiter(uld : *mut ULocaleData, r#type : ULocaleDataDelimiterType, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { ulocdata_getDelimiter(uld as _, r#type, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getExemplarSet(uld: *mut ULocaleData, fillin: *mut USet, options: u32, extype: ULocaleDataExemplarSetType, status: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getExemplarSet(uld : *mut ULocaleData, fillin : *mut USet, options : u32, extype : ULocaleDataExemplarSetType, status : *mut UErrorCode) -> *mut USet);
    unsafe { ulocdata_getExemplarSet(uld as _, fillin as _, options, extype, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getLocaleDisplayPattern(uld: *mut ULocaleData, pattern: *mut UChar, patterncapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getLocaleDisplayPattern(uld : *mut ULocaleData, pattern : *mut UChar, patterncapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ulocdata_getLocaleDisplayPattern(uld as _, pattern as _, patterncapacity, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getLocaleSeparator(uld: *mut ULocaleData, separator: *mut UChar, separatorcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getLocaleSeparator(uld : *mut ULocaleData, separator : *mut UChar, separatorcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ulocdata_getLocaleSeparator(uld as _, separator as _, separatorcapacity, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getMeasurementSystem(localeid: *const i8, status: *mut UErrorCode) -> UMeasurementSystem {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getMeasurementSystem(localeid : *const i8, status : *mut UErrorCode) -> UMeasurementSystem);
    unsafe { ulocdata_getMeasurementSystem(localeid, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getNoSubstitute(uld: *mut ULocaleData) -> UBool {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getNoSubstitute(uld : *mut ULocaleData) -> UBool);
    unsafe { ulocdata_getNoSubstitute(uld as _) }
}
#[inline]
pub unsafe fn ulocdata_getPaperSize(localeid: *const i8, height: *mut i32, width: *mut i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getPaperSize(localeid : *const i8, height : *mut i32, width : *mut i32, status : *mut UErrorCode));
    unsafe { ulocdata_getPaperSize(localeid, height as _, width as _, status as _) }
}
#[inline]
pub unsafe fn ulocdata_open(localeid: *const i8, status: *mut UErrorCode) -> *mut ULocaleData {
    windows_core::link!("icuin.dll" "C" fn ulocdata_open(localeid : *const i8, status : *mut UErrorCode) -> *mut ULocaleData);
    unsafe { ulocdata_open(localeid, status as _) }
}
#[inline]
pub unsafe fn ulocdata_setNoSubstitute(uld: *mut ULocaleData, setting: UBool) {
    windows_core::link!("icuin.dll" "C" fn ulocdata_setNoSubstitute(uld : *mut ULocaleData, setting : UBool));
    unsafe { ulocdata_setNoSubstitute(uld as _, setting) }
}
#[inline]
pub unsafe fn umsg_applyPattern(fmt: *mut UMessageFormat, pattern: *const UChar, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn umsg_applyPattern(fmt : *mut UMessageFormat, pattern : *const UChar, patternlength : i32, parseerror : *mut UParseError, status : *mut UErrorCode));
    unsafe { umsg_applyPattern(fmt as _, pattern, patternlength, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn umsg_autoQuoteApostrophe(pattern: *const UChar, patternlength: i32, dest: *mut UChar, destcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn umsg_autoQuoteApostrophe(pattern : *const UChar, patternlength : i32, dest : *mut UChar, destcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { umsg_autoQuoteApostrophe(pattern, patternlength, dest as _, destcapacity, ec as _) }
}
#[inline]
pub unsafe fn umsg_clone(fmt: *const UMessageFormat, status: *mut UErrorCode) -> UMessageFormat {
    windows_core::link!("icuin.dll" "C" fn umsg_clone(fmt : *const UMessageFormat, status : *mut UErrorCode) -> UMessageFormat);
    unsafe { umsg_clone(fmt, status as _) }
}
#[inline]
pub unsafe fn umsg_close() -> UMessageFormat {
    windows_core::link!("icuin.dll" "C" fn umsg_close(format : *mut UMessageFormat));
    unsafe {
        let mut result__ = core::mem::zeroed();
        umsg_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn umsg_format(fmt: *const UMessageFormat, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn umsg_format(fmt : *const UMessageFormat, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { umsg_format(fmt, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn umsg_getLocale(fmt: *const UMessageFormat) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn umsg_getLocale(fmt : *const UMessageFormat) -> *const i8);
    unsafe { umsg_getLocale(fmt) }
}
#[inline]
pub unsafe fn umsg_open(pattern: *const UChar, patternlength: i32, locale: *const i8, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut UMessageFormat {
    windows_core::link!("icuin.dll" "C" fn umsg_open(pattern : *const UChar, patternlength : i32, locale : *const i8, parseerror : *mut UParseError, status : *mut UErrorCode) -> *mut UMessageFormat);
    unsafe { umsg_open(pattern, patternlength, locale, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn umsg_parse(fmt: *const UMessageFormat, source: *const UChar, sourcelength: i32, count: *mut i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn umsg_parse(fmt : *const UMessageFormat, source : *const UChar, sourcelength : i32, count : *mut i32, status : *mut UErrorCode));
    unsafe { umsg_parse(fmt, source, sourcelength, count as _, status as _) }
}
#[inline]
pub unsafe fn umsg_setLocale(fmt: *mut UMessageFormat, locale: *const i8) {
    windows_core::link!("icuin.dll" "C" fn umsg_setLocale(fmt : *mut UMessageFormat, locale : *const i8));
    unsafe { umsg_setLocale(fmt as _, locale) }
}
#[inline]
pub unsafe fn umsg_toPattern(fmt: *const UMessageFormat, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn umsg_toPattern(fmt : *const UMessageFormat, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { umsg_toPattern(fmt, result as _, resultlength, status as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn umsg_vformat(fmt: *const UMessageFormat, result: *mut UChar, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn umsg_vformat(fmt : *const UMessageFormat, result : *mut UChar, resultlength : i32, ap : *mut i8, status : *mut UErrorCode) -> i32);
    unsafe { umsg_vformat(fmt, result as _, resultlength, ap as _, status as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn umsg_vformat(fmt: *const UMessageFormat, result: *mut UChar, resultlength: i32, ap: super::vadefs::va_list, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn umsg_vformat(fmt : *const UMessageFormat, result : *mut UChar, resultlength : i32, ap : super::vadefs::va_list, status : *mut UErrorCode) -> i32);
    unsafe { umsg_vformat(fmt, result as _, resultlength, ap, status as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn umsg_vparse(fmt: *const UMessageFormat, source: *const UChar, sourcelength: i32, count: *mut i32, ap: *mut i8, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn umsg_vparse(fmt : *const UMessageFormat, source : *const UChar, sourcelength : i32, count : *mut i32, ap : *mut i8, status : *mut UErrorCode));
    unsafe { umsg_vparse(fmt, source, sourcelength, count as _, ap as _, status as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn umsg_vparse(fmt: *const UMessageFormat, source: *const UChar, sourcelength: i32, count: *mut i32, ap: super::vadefs::va_list, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn umsg_vparse(fmt : *const UMessageFormat, source : *const UChar, sourcelength : i32, count : *mut i32, ap : super::vadefs::va_list, status : *mut UErrorCode));
    unsafe { umsg_vparse(fmt, source, sourcelength, count as _, ap, status as _) }
}
#[inline]
pub unsafe fn umutablecptrie_buildImmutable(trie: *mut UMutableCPTrie, r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, perrorcode: *mut UErrorCode) -> *mut UCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_buildImmutable(trie : *mut UMutableCPTrie, r#type : UCPTrieType, valuewidth : UCPTrieValueWidth, perrorcode : *mut UErrorCode) -> *mut UCPTrie);
    unsafe { umutablecptrie_buildImmutable(trie as _, r#type, valuewidth, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_clone(other: *const UMutableCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_clone(other : *const UMutableCPTrie, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    unsafe { umutablecptrie_clone(other, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_close() -> UMutableCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_close(trie : *mut UMutableCPTrie));
    unsafe {
        let mut result__ = core::mem::zeroed();
        umutablecptrie_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn umutablecptrie_fromUCPMap(map: *const UCPMap, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_fromUCPMap(map : *const UCPMap, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    unsafe { umutablecptrie_fromUCPMap(map, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_fromUCPTrie(trie: *const UCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_fromUCPTrie(trie : *const UCPTrie, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    unsafe { umutablecptrie_fromUCPTrie(trie, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_get(trie: *const UMutableCPTrie, c: UChar32) -> u32 {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_get(trie : *const UMutableCPTrie, c : UChar32) -> u32);
    unsafe { umutablecptrie_get(trie, c) }
}
#[inline]
pub unsafe fn umutablecptrie_getRange(trie: *const UMutableCPTrie, start: UChar32, option: UCPMapRangeOption, surrogatevalue: u32, filter: UCPMapValueFilter, context: *const core::ffi::c_void, pvalue: *mut u32) -> UChar32 {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_getRange(trie : *const UMutableCPTrie, start : UChar32, option : UCPMapRangeOption, surrogatevalue : u32, filter : UCPMapValueFilter, context : *const core::ffi::c_void, pvalue : *mut u32) -> UChar32);
    unsafe { umutablecptrie_getRange(trie, start, option, surrogatevalue, filter, context, pvalue as _) }
}
#[inline]
pub unsafe fn umutablecptrie_open(initialvalue: u32, errorvalue: u32, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_open(initialvalue : u32, errorvalue : u32, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    unsafe { umutablecptrie_open(initialvalue, errorvalue, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_set(trie: *mut UMutableCPTrie, c: UChar32, value: u32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_set(trie : *mut UMutableCPTrie, c : UChar32, value : u32, perrorcode : *mut UErrorCode));
    unsafe { umutablecptrie_set(trie as _, c, value, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_setRange(trie: *mut UMutableCPTrie, start: UChar32, end: UChar32, value: u32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_setRange(trie : *mut UMutableCPTrie, start : UChar32, end : UChar32, value : u32, perrorcode : *mut UErrorCode));
    unsafe { umutablecptrie_setRange(trie as _, start, end, value, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_append(norm2: *const UNormalizer2, first: *mut UChar, firstlength: i32, firstcapacity: i32, second: *const UChar, secondlength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_append(norm2 : *const UNormalizer2, first : *mut UChar, firstlength : i32, firstcapacity : i32, second : *const UChar, secondlength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_append(norm2, first as _, firstlength, firstcapacity, second, secondlength, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_close() -> UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_close(norm2 : *mut UNormalizer2));
    unsafe {
        let mut result__ = core::mem::zeroed();
        unorm2_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn unorm2_composePair(norm2: *const UNormalizer2, a: UChar32, b: UChar32) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_composePair(norm2 : *const UNormalizer2, a : UChar32, b : UChar32) -> UChar32);
    unsafe { unorm2_composePair(norm2, a, b) }
}
#[inline]
pub unsafe fn unorm2_getCombiningClass(norm2: *const UNormalizer2, c: UChar32) -> u8 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getCombiningClass(norm2 : *const UNormalizer2, c : UChar32) -> u8);
    unsafe { unorm2_getCombiningClass(norm2, c) }
}
#[inline]
pub unsafe fn unorm2_getDecomposition(norm2: *const UNormalizer2, c: UChar32, decomposition: *mut UChar, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getDecomposition(norm2 : *const UNormalizer2, c : UChar32, decomposition : *mut UChar, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_getDecomposition(norm2, c, decomposition as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getInstance(packagename: *const i8, name: *const i8, mode: UNormalization2Mode, perrorcode: *mut UErrorCode) -> *const UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getInstance(packagename : *const i8, name : *const i8, mode : UNormalization2Mode, perrorcode : *mut UErrorCode) -> *const UNormalizer2);
    unsafe { unorm2_getInstance(packagename, name, mode, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getNFCInstance(perrorcode: *mut UErrorCode) -> *const UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getNFCInstance(perrorcode : *mut UErrorCode) -> *const UNormalizer2);
    unsafe { unorm2_getNFCInstance(perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getNFDInstance(perrorcode: *mut UErrorCode) -> *const UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getNFDInstance(perrorcode : *mut UErrorCode) -> *const UNormalizer2);
    unsafe { unorm2_getNFDInstance(perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getNFKCCasefoldInstance(perrorcode: *mut UErrorCode) -> *const UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getNFKCCasefoldInstance(perrorcode : *mut UErrorCode) -> *const UNormalizer2);
    unsafe { unorm2_getNFKCCasefoldInstance(perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getNFKCInstance(perrorcode: *mut UErrorCode) -> *const UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getNFKCInstance(perrorcode : *mut UErrorCode) -> *const UNormalizer2);
    unsafe { unorm2_getNFKCInstance(perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getNFKDInstance(perrorcode: *mut UErrorCode) -> *const UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getNFKDInstance(perrorcode : *mut UErrorCode) -> *const UNormalizer2);
    unsafe { unorm2_getNFKDInstance(perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getRawDecomposition(norm2: *const UNormalizer2, c: UChar32, decomposition: *mut UChar, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getRawDecomposition(norm2 : *const UNormalizer2, c : UChar32, decomposition : *mut UChar, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_getRawDecomposition(norm2, c, decomposition as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_hasBoundaryAfter(norm2: *const UNormalizer2, c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn unorm2_hasBoundaryAfter(norm2 : *const UNormalizer2, c : UChar32) -> UBool);
    unsafe { unorm2_hasBoundaryAfter(norm2, c) }
}
#[inline]
pub unsafe fn unorm2_hasBoundaryBefore(norm2: *const UNormalizer2, c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn unorm2_hasBoundaryBefore(norm2 : *const UNormalizer2, c : UChar32) -> UBool);
    unsafe { unorm2_hasBoundaryBefore(norm2, c) }
}
#[inline]
pub unsafe fn unorm2_isInert(norm2: *const UNormalizer2, c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn unorm2_isInert(norm2 : *const UNormalizer2, c : UChar32) -> UBool);
    unsafe { unorm2_isInert(norm2, c) }
}
#[inline]
pub unsafe fn unorm2_isNormalized(norm2: *const UNormalizer2, s: *const UChar, length: i32, perrorcode: *mut UErrorCode) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn unorm2_isNormalized(norm2 : *const UNormalizer2, s : *const UChar, length : i32, perrorcode : *mut UErrorCode) -> UBool);
    unsafe { unorm2_isNormalized(norm2, s, length, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_normalize(norm2: *const UNormalizer2, src: *const UChar, length: i32, dest: *mut UChar, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_normalize(norm2 : *const UNormalizer2, src : *const UChar, length : i32, dest : *mut UChar, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_normalize(norm2, src, length, dest as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_normalizeSecondAndAppend(norm2: *const UNormalizer2, first: *mut UChar, firstlength: i32, firstcapacity: i32, second: *const UChar, secondlength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_normalizeSecondAndAppend(norm2 : *const UNormalizer2, first : *mut UChar, firstlength : i32, firstcapacity : i32, second : *const UChar, secondlength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_normalizeSecondAndAppend(norm2, first as _, firstlength, firstcapacity, second, secondlength, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_openFiltered(norm2: *const UNormalizer2, filterset: *const USet, perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_openFiltered(norm2 : *const UNormalizer2, filterset : *const USet, perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unsafe { unorm2_openFiltered(norm2, filterset, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_quickCheck(norm2: *const UNormalizer2, s: *const UChar, length: i32, perrorcode: *mut UErrorCode) -> UNormalizationCheckResult {
    windows_core::link!("icuuc.dll" "C" fn unorm2_quickCheck(norm2 : *const UNormalizer2, s : *const UChar, length : i32, perrorcode : *mut UErrorCode) -> UNormalizationCheckResult);
    unsafe { unorm2_quickCheck(norm2, s, length, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_spanQuickCheckYes(norm2: *const UNormalizer2, s: *const UChar, length: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_spanQuickCheckYes(norm2 : *const UNormalizer2, s : *const UChar, length : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_spanQuickCheckYes(norm2, s, length, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm_compare(s1: *const UChar, length1: i32, s2: *const UChar, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm_compare(s1 : *const UChar, length1 : i32, s2 : *const UChar, length2 : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm_compare(s1, length1, s2, length2, options, perrorcode as _) }
}
#[inline]
pub unsafe fn unum_applyPattern(format: *mut UNumberFormat, localized: UBool, pattern: *const UChar, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn unum_applyPattern(format : *mut UNumberFormat, localized : UBool, pattern : *const UChar, patternlength : i32, parseerror : *mut UParseError, status : *mut UErrorCode));
    unsafe { unum_applyPattern(format as _, localized, pattern, patternlength, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn unum_clone(fmt: *const UNumberFormat, status: *mut UErrorCode) -> *mut UNumberFormat {
    windows_core::link!("icuin.dll" "C" fn unum_clone(fmt : *const UNumberFormat, status : *mut UErrorCode) -> *mut UNumberFormat);
    unsafe { unum_clone(fmt, status as _) }
}
#[inline]
pub unsafe fn unum_close() -> UNumberFormat {
    windows_core::link!("icuin.dll" "C" fn unum_close(fmt : *mut UNumberFormat));
    unsafe {
        let mut result__ = core::mem::zeroed();
        unum_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn unum_countAvailable() -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_countAvailable() -> i32);
    unsafe { unum_countAvailable() }
}
#[inline]
pub unsafe fn unum_format(fmt: *const UNumberFormat, number: i32, result: *mut UChar, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_format(fmt : *const UNumberFormat, number : i32, result : *mut UChar, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_format(fmt, number, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatDecimal(fmt: *const UNumberFormat, number: *const i8, length: i32, result: *mut UChar, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatDecimal(fmt : *const UNumberFormat, number : *const i8, length : i32, result : *mut UChar, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatDecimal(fmt, number, length, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatDouble(fmt: *const UNumberFormat, number: f64, result: *mut UChar, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatDouble(fmt : *const UNumberFormat, number : f64, result : *mut UChar, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatDouble(fmt, number, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatDoubleCurrency(fmt: *const UNumberFormat, number: f64, currency: *mut UChar, result: *mut UChar, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatDoubleCurrency(fmt : *const UNumberFormat, number : f64, currency : *mut UChar, result : *mut UChar, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatDoubleCurrency(fmt, number, currency as _, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatDoubleForFields(format: *const UNumberFormat, number: f64, result: *mut UChar, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatDoubleForFields(format : *const UNumberFormat, number : f64, result : *mut UChar, resultlength : i32, fpositer : *mut UFieldPositionIterator, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatDoubleForFields(format, number, result as _, resultlength, fpositer as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatInt64(fmt: *const UNumberFormat, number: i64, result: *mut UChar, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatInt64(fmt : *const UNumberFormat, number : i64, result : *mut UChar, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatInt64(fmt, number, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatUFormattable(fmt: *const UNumberFormat, number: *const UFormattable, result: *mut UChar, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatUFormattable(fmt : *const UNumberFormat, number : *const UFormattable, result : *mut UChar, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatUFormattable(fmt, number, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_getAttribute(fmt: *const UNumberFormat, attr: UNumberFormatAttribute) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_getAttribute(fmt : *const UNumberFormat, attr : UNumberFormatAttribute) -> i32);
    unsafe { unum_getAttribute(fmt, attr) }
}
#[inline]
pub unsafe fn unum_getAvailable(localeindex: i32) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn unum_getAvailable(localeindex : i32) -> *const i8);
    unsafe { unum_getAvailable(localeindex) }
}
#[inline]
pub unsafe fn unum_getContext(fmt: *const UNumberFormat, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext {
    windows_core::link!("icuin.dll" "C" fn unum_getContext(fmt : *const UNumberFormat, r#type : UDisplayContextType, status : *mut UErrorCode) -> UDisplayContext);
    unsafe { unum_getContext(fmt, r#type, status as _) }
}
#[inline]
pub unsafe fn unum_getDoubleAttribute(fmt: *const UNumberFormat, attr: UNumberFormatAttribute) -> f64 {
    windows_core::link!("icuin.dll" "C" fn unum_getDoubleAttribute(fmt : *const UNumberFormat, attr : UNumberFormatAttribute) -> f64);
    unsafe { unum_getDoubleAttribute(fmt, attr) }
}
#[inline]
pub unsafe fn unum_getLocaleByType(fmt: *const UNumberFormat, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn unum_getLocaleByType(fmt : *const UNumberFormat, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> *const i8);
    unsafe { unum_getLocaleByType(fmt, r#type, status as _) }
}
#[inline]
pub unsafe fn unum_getSymbol(fmt: *const UNumberFormat, symbol: UNumberFormatSymbol, buffer: *mut UChar, size: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_getSymbol(fmt : *const UNumberFormat, symbol : UNumberFormatSymbol, buffer : *mut UChar, size : i32, status : *mut UErrorCode) -> i32);
    unsafe { unum_getSymbol(fmt, symbol, buffer as _, size, status as _) }
}
#[inline]
pub unsafe fn unum_getTextAttribute(fmt: *const UNumberFormat, tag: UNumberFormatTextAttribute, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_getTextAttribute(fmt : *const UNumberFormat, tag : UNumberFormatTextAttribute, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { unum_getTextAttribute(fmt, tag, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn unum_open(style: UNumberFormatStyle, pattern: *const UChar, patternlength: i32, locale: *const i8, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut UNumberFormat {
    windows_core::link!("icuin.dll" "C" fn unum_open(style : UNumberFormatStyle, pattern : *const UChar, patternlength : i32, locale : *const i8, parseerr : *mut UParseError, status : *mut UErrorCode) -> *mut UNumberFormat);
    unsafe { unum_open(style, pattern, patternlength, locale, parseerr as _, status as _) }
}
#[inline]
pub unsafe fn unum_parse(fmt: *const UNumberFormat, text: *const UChar, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_parse(fmt : *const UNumberFormat, text : *const UChar, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> i32);
    unsafe { unum_parse(fmt, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn unum_parseDecimal(fmt: *const UNumberFormat, text: *const UChar, textlength: i32, parsepos: *mut i32, outbuf: *mut i8, outbuflength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_parseDecimal(fmt : *const UNumberFormat, text : *const UChar, textlength : i32, parsepos : *mut i32, outbuf : *mut i8, outbuflength : i32, status : *mut UErrorCode) -> i32);
    unsafe { unum_parseDecimal(fmt, text, textlength, parsepos as _, outbuf as _, outbuflength, status as _) }
}
#[inline]
pub unsafe fn unum_parseDouble(fmt: *const UNumberFormat, text: *const UChar, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn unum_parseDouble(fmt : *const UNumberFormat, text : *const UChar, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> f64);
    unsafe { unum_parseDouble(fmt, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn unum_parseDoubleCurrency(fmt: *const UNumberFormat, text: *const UChar, textlength: i32, parsepos: *mut i32, currency: *mut UChar, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn unum_parseDoubleCurrency(fmt : *const UNumberFormat, text : *const UChar, textlength : i32, parsepos : *mut i32, currency : *mut UChar, status : *mut UErrorCode) -> f64);
    unsafe { unum_parseDoubleCurrency(fmt, text, textlength, parsepos as _, currency as _, status as _) }
}
#[inline]
pub unsafe fn unum_parseInt64(fmt: *const UNumberFormat, text: *const UChar, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn unum_parseInt64(fmt : *const UNumberFormat, text : *const UChar, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> i64);
    unsafe { unum_parseInt64(fmt, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn unum_parseToUFormattable(fmt: *const UNumberFormat, result: *mut UFormattable, text: *const UChar, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> *mut UFormattable {
    windows_core::link!("icuin.dll" "C" fn unum_parseToUFormattable(fmt : *const UNumberFormat, result : *mut UFormattable, text : *const UChar, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> *mut UFormattable);
    unsafe { unum_parseToUFormattable(fmt, result as _, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn unum_setAttribute(fmt: *mut UNumberFormat, attr: UNumberFormatAttribute, newvalue: i32) {
    windows_core::link!("icuin.dll" "C" fn unum_setAttribute(fmt : *mut UNumberFormat, attr : UNumberFormatAttribute, newvalue : i32));
    unsafe { unum_setAttribute(fmt as _, attr, newvalue) }
}
#[inline]
pub unsafe fn unum_setContext(fmt: *mut UNumberFormat, value: UDisplayContext, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn unum_setContext(fmt : *mut UNumberFormat, value : UDisplayContext, status : *mut UErrorCode));
    unsafe { unum_setContext(fmt as _, value, status as _) }
}
#[inline]
pub unsafe fn unum_setDoubleAttribute(fmt: *mut UNumberFormat, attr: UNumberFormatAttribute, newvalue: f64) {
    windows_core::link!("icuin.dll" "C" fn unum_setDoubleAttribute(fmt : *mut UNumberFormat, attr : UNumberFormatAttribute, newvalue : f64));
    unsafe { unum_setDoubleAttribute(fmt as _, attr, newvalue) }
}
#[inline]
pub unsafe fn unum_setSymbol(fmt: *mut UNumberFormat, symbol: UNumberFormatSymbol, value: *const UChar, length: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn unum_setSymbol(fmt : *mut UNumberFormat, symbol : UNumberFormatSymbol, value : *const UChar, length : i32, status : *mut UErrorCode));
    unsafe { unum_setSymbol(fmt as _, symbol, value, length, status as _) }
}
#[inline]
pub unsafe fn unum_setTextAttribute(fmt: *mut UNumberFormat, tag: UNumberFormatTextAttribute, newvalue: *const UChar, newvaluelength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn unum_setTextAttribute(fmt : *mut UNumberFormat, tag : UNumberFormatTextAttribute, newvalue : *const UChar, newvaluelength : i32, status : *mut UErrorCode));
    unsafe { unum_setTextAttribute(fmt as _, tag, newvalue, newvaluelength, status as _) }
}
#[inline]
pub unsafe fn unum_toPattern(fmt: *const UNumberFormat, ispatternlocalized: UBool, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_toPattern(fmt : *const UNumberFormat, ispatternlocalized : UBool, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { unum_toPattern(fmt, ispatternlocalized, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn unumf_close() -> UNumberFormatter {
    windows_core::link!("icu.dll" "C" fn unumf_close(uformatter : *mut UNumberFormatter));
    unsafe {
        let mut result__ = core::mem::zeroed();
        unumf_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn unumf_closeResult() -> UFormattedNumber {
    windows_core::link!("icu.dll" "C" fn unumf_closeResult(uresult : *mut UFormattedNumber));
    unsafe {
        let mut result__ = core::mem::zeroed();
        unumf_closeResult(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn unumf_formatDecimal(uformatter: *const UNumberFormatter, value: *const i8, valuelen: i32, uresult: *mut UFormattedNumber, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn unumf_formatDecimal(uformatter : *const UNumberFormatter, value : *const i8, valuelen : i32, uresult : *mut UFormattedNumber, ec : *mut UErrorCode));
    unsafe { unumf_formatDecimal(uformatter, value, valuelen, uresult as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_formatDouble(uformatter: *const UNumberFormatter, value: f64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn unumf_formatDouble(uformatter : *const UNumberFormatter, value : f64, uresult : *mut UFormattedNumber, ec : *mut UErrorCode));
    unsafe { unumf_formatDouble(uformatter, value, uresult as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_formatInt(uformatter: *const UNumberFormatter, value: i64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn unumf_formatInt(uformatter : *const UNumberFormatter, value : i64, uresult : *mut UFormattedNumber, ec : *mut UErrorCode));
    unsafe { unumf_formatInt(uformatter, value, uresult as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_openForSkeletonAndLocale(skeleton: *const UChar, skeletonlen: i32, locale: *const i8, ec: *mut UErrorCode) -> *mut UNumberFormatter {
    windows_core::link!("icu.dll" "C" fn unumf_openForSkeletonAndLocale(skeleton : *const UChar, skeletonlen : i32, locale : *const i8, ec : *mut UErrorCode) -> *mut UNumberFormatter);
    unsafe { unumf_openForSkeletonAndLocale(skeleton, skeletonlen, locale, ec as _) }
}
#[inline]
pub unsafe fn unumf_openForSkeletonAndLocaleWithError(skeleton: *const UChar, skeletonlen: i32, locale: *const i8, perror: *mut UParseError, ec: *mut UErrorCode) -> *mut UNumberFormatter {
    windows_core::link!("icu.dll" "C" fn unumf_openForSkeletonAndLocaleWithError(skeleton : *const UChar, skeletonlen : i32, locale : *const i8, perror : *mut UParseError, ec : *mut UErrorCode) -> *mut UNumberFormatter);
    unsafe { unumf_openForSkeletonAndLocaleWithError(skeleton, skeletonlen, locale, perror as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_openResult(ec: *mut UErrorCode) -> *mut UFormattedNumber {
    windows_core::link!("icu.dll" "C" fn unumf_openResult(ec : *mut UErrorCode) -> *mut UFormattedNumber);
    unsafe { unumf_openResult(ec as _) }
}
#[inline]
pub unsafe fn unumf_resultAsValue(uresult: *const UFormattedNumber, ec: *mut UErrorCode) -> *const UFormattedValue {
    windows_core::link!("icu.dll" "C" fn unumf_resultAsValue(uresult : *const UFormattedNumber, ec : *mut UErrorCode) -> *const UFormattedValue);
    unsafe { unumf_resultAsValue(uresult, ec as _) }
}
#[inline]
pub unsafe fn unumf_resultGetAllFieldPositions(uresult: *const UFormattedNumber, ufpositer: *mut UFieldPositionIterator, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn unumf_resultGetAllFieldPositions(uresult : *const UFormattedNumber, ufpositer : *mut UFieldPositionIterator, ec : *mut UErrorCode));
    unsafe { unumf_resultGetAllFieldPositions(uresult, ufpositer as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_resultNextFieldPosition(uresult: *const UFormattedNumber, ufpos: *mut UFieldPosition, ec: *mut UErrorCode) -> UBool {
    windows_core::link!("icu.dll" "C" fn unumf_resultNextFieldPosition(uresult : *const UFormattedNumber, ufpos : *mut UFieldPosition, ec : *mut UErrorCode) -> UBool);
    unsafe { unumf_resultNextFieldPosition(uresult, ufpos as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_resultToDecimalNumber(uresult: *const UFormattedNumber, dest: *mut i8, destcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn unumf_resultToDecimalNumber(uresult : *const UFormattedNumber, dest : *mut i8, destcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { unumf_resultToDecimalNumber(uresult, dest as _, destcapacity, ec as _) }
}
#[inline]
pub unsafe fn unumf_resultToString(uresult: *const UFormattedNumber, buffer: *mut UChar, buffercapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn unumf_resultToString(uresult : *const UFormattedNumber, buffer : *mut UChar, buffercapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { unumf_resultToString(uresult, buffer as _, buffercapacity, ec as _) }
}
#[inline]
pub unsafe fn unumrf_close() -> UNumberRangeFormatter {
    windows_core::link!("icu.dll" "C" fn unumrf_close(uformatter : *mut UNumberRangeFormatter));
    unsafe {
        let mut result__ = core::mem::zeroed();
        unumrf_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn unumrf_closeResult() -> UFormattedNumberRange {
    windows_core::link!("icu.dll" "C" fn unumrf_closeResult(uresult : *mut UFormattedNumberRange));
    unsafe {
        let mut result__ = core::mem::zeroed();
        unumrf_closeResult(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn unumrf_formatDecimalRange(uformatter: *const UNumberRangeFormatter, first: *const i8, firstlen: i32, second: *const i8, secondlen: i32, uresult: *mut UFormattedNumberRange, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn unumrf_formatDecimalRange(uformatter : *const UNumberRangeFormatter, first : *const i8, firstlen : i32, second : *const i8, secondlen : i32, uresult : *mut UFormattedNumberRange, ec : *mut UErrorCode));
    unsafe { unumrf_formatDecimalRange(uformatter, first, firstlen, second, secondlen, uresult as _, ec as _) }
}
#[inline]
pub unsafe fn unumrf_formatDoubleRange(uformatter: *const UNumberRangeFormatter, first: f64, second: f64, uresult: *mut UFormattedNumberRange, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn unumrf_formatDoubleRange(uformatter : *const UNumberRangeFormatter, first : f64, second : f64, uresult : *mut UFormattedNumberRange, ec : *mut UErrorCode));
    unsafe { unumrf_formatDoubleRange(uformatter, first, second, uresult as _, ec as _) }
}
#[inline]
pub unsafe fn unumrf_openForSkeletonWithCollapseAndIdentityFallback(skeleton: *const UChar, skeletonlen: i32, collapse: UNumberRangeCollapse, identityfallback: UNumberRangeIdentityFallback, locale: *const i8, perror: *mut UParseError, ec: *mut UErrorCode) -> *mut UNumberRangeFormatter {
    windows_core::link!("icu.dll" "C" fn unumrf_openForSkeletonWithCollapseAndIdentityFallback(skeleton : *const UChar, skeletonlen : i32, collapse : UNumberRangeCollapse, identityfallback : UNumberRangeIdentityFallback, locale : *const i8, perror : *mut UParseError, ec : *mut UErrorCode) -> *mut UNumberRangeFormatter);
    unsafe { unumrf_openForSkeletonWithCollapseAndIdentityFallback(skeleton, skeletonlen, collapse, identityfallback, locale, perror as _, ec as _) }
}
#[inline]
pub unsafe fn unumrf_openResult(ec: *mut UErrorCode) -> *mut UFormattedNumberRange {
    windows_core::link!("icu.dll" "C" fn unumrf_openResult(ec : *mut UErrorCode) -> *mut UFormattedNumberRange);
    unsafe { unumrf_openResult(ec as _) }
}
#[inline]
pub unsafe fn unumrf_resultAsValue(uresult: *const UFormattedNumberRange, ec: *mut UErrorCode) -> *const UFormattedValue {
    windows_core::link!("icu.dll" "C" fn unumrf_resultAsValue(uresult : *const UFormattedNumberRange, ec : *mut UErrorCode) -> *const UFormattedValue);
    unsafe { unumrf_resultAsValue(uresult, ec as _) }
}
#[inline]
pub unsafe fn unumrf_resultGetFirstDecimalNumber(uresult: *const UFormattedNumberRange, dest: *mut i8, destcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn unumrf_resultGetFirstDecimalNumber(uresult : *const UFormattedNumberRange, dest : *mut i8, destcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { unumrf_resultGetFirstDecimalNumber(uresult, dest as _, destcapacity, ec as _) }
}
#[inline]
pub unsafe fn unumrf_resultGetIdentityResult(uresult: *const UFormattedNumberRange, ec: *mut UErrorCode) -> UNumberRangeIdentityResult {
    windows_core::link!("icu.dll" "C" fn unumrf_resultGetIdentityResult(uresult : *const UFormattedNumberRange, ec : *mut UErrorCode) -> UNumberRangeIdentityResult);
    unsafe { unumrf_resultGetIdentityResult(uresult, ec as _) }
}
#[inline]
pub unsafe fn unumrf_resultGetSecondDecimalNumber(uresult: *const UFormattedNumberRange, dest: *mut i8, destcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn unumrf_resultGetSecondDecimalNumber(uresult : *const UFormattedNumberRange, dest : *mut i8, destcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { unumrf_resultGetSecondDecimalNumber(uresult, dest as _, destcapacity, ec as _) }
}
#[inline]
pub unsafe fn unumsys_close() -> UNumberingSystem {
    windows_core::link!("icuin.dll" "C" fn unumsys_close(unumsys : *mut UNumberingSystem));
    unsafe {
        let mut result__ = core::mem::zeroed();
        unumsys_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn unumsys_getDescription(unumsys: *const UNumberingSystem, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unumsys_getDescription(unumsys : *const UNumberingSystem, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { unumsys_getDescription(unumsys, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn unumsys_getName(unumsys: *const UNumberingSystem) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn unumsys_getName(unumsys : *const UNumberingSystem) -> *const i8);
    unsafe { unumsys_getName(unumsys) }
}
#[inline]
pub unsafe fn unumsys_getRadix(unumsys: *const UNumberingSystem) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unumsys_getRadix(unumsys : *const UNumberingSystem) -> i32);
    unsafe { unumsys_getRadix(unumsys) }
}
#[inline]
pub unsafe fn unumsys_isAlgorithmic(unumsys: *const UNumberingSystem) -> UBool {
    windows_core::link!("icuin.dll" "C" fn unumsys_isAlgorithmic(unumsys : *const UNumberingSystem) -> UBool);
    unsafe { unumsys_isAlgorithmic(unumsys) }
}
#[inline]
pub unsafe fn unumsys_open(locale: *const i8, status: *mut UErrorCode) -> *mut UNumberingSystem {
    windows_core::link!("icuin.dll" "C" fn unumsys_open(locale : *const i8, status : *mut UErrorCode) -> *mut UNumberingSystem);
    unsafe { unumsys_open(locale, status as _) }
}
#[inline]
pub unsafe fn unumsys_openAvailableNames(status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn unumsys_openAvailableNames(status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { unumsys_openAvailableNames(status as _) }
}
#[inline]
pub unsafe fn unumsys_openByName(name: *const i8, status: *mut UErrorCode) -> *mut UNumberingSystem {
    windows_core::link!("icuin.dll" "C" fn unumsys_openByName(name : *const i8, status : *mut UErrorCode) -> *mut UNumberingSystem);
    unsafe { unumsys_openByName(name, status as _) }
}
#[inline]
pub unsafe fn uplrules_close() -> UPluralRules {
    windows_core::link!("icuin.dll" "C" fn uplrules_close(uplrules : *mut UPluralRules));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uplrules_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uplrules_getKeywords(uplrules: *const UPluralRules, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn uplrules_getKeywords(uplrules : *const UPluralRules, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uplrules_getKeywords(uplrules, status as _) }
}
#[inline]
pub unsafe fn uplrules_open(locale: *const i8, status: *mut UErrorCode) -> *mut UPluralRules {
    windows_core::link!("icuin.dll" "C" fn uplrules_open(locale : *const i8, status : *mut UErrorCode) -> *mut UPluralRules);
    unsafe { uplrules_open(locale, status as _) }
}
#[inline]
pub unsafe fn uplrules_openForType(locale: *const i8, r#type: UPluralType, status: *mut UErrorCode) -> *mut UPluralRules {
    windows_core::link!("icuin.dll" "C" fn uplrules_openForType(locale : *const i8, r#type : UPluralType, status : *mut UErrorCode) -> *mut UPluralRules);
    unsafe { uplrules_openForType(locale, r#type, status as _) }
}
#[inline]
pub unsafe fn uplrules_select(uplrules: *const UPluralRules, number: f64, keyword: *mut UChar, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uplrules_select(uplrules : *const UPluralRules, number : f64, keyword : *mut UChar, capacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uplrules_select(uplrules, number, keyword as _, capacity, status as _) }
}
#[inline]
pub unsafe fn uplrules_selectFormatted(uplrules: *const UPluralRules, number: *const UFormattedNumber, keyword: *mut UChar, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn uplrules_selectFormatted(uplrules : *const UPluralRules, number : *const UFormattedNumber, keyword : *mut UChar, capacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uplrules_selectFormatted(uplrules, number, keyword as _, capacity, status as _) }
}
#[inline]
pub unsafe fn uregex_appendReplacement(regexp: *mut URegularExpression, replacementtext: *const UChar, replacementlength: i32, destbuf: *mut *mut UChar, destcapacity: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_appendReplacement(regexp : *mut URegularExpression, replacementtext : *const UChar, replacementlength : i32, destbuf : *mut *mut UChar, destcapacity : *mut i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_appendReplacement(regexp as _, replacementtext, replacementlength, destbuf as _, destcapacity as _, status as _) }
}
#[inline]
pub unsafe fn uregex_appendReplacementUText(regexp: *mut URegularExpression, replacementtext: *mut UText, dest: *mut UText, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_appendReplacementUText(regexp : *mut URegularExpression, replacementtext : *mut UText, dest : *mut UText, status : *mut UErrorCode));
    unsafe { uregex_appendReplacementUText(regexp as _, replacementtext as _, dest as _, status as _) }
}
#[inline]
pub unsafe fn uregex_appendTail(regexp: *mut URegularExpression, destbuf: *mut *mut UChar, destcapacity: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_appendTail(regexp : *mut URegularExpression, destbuf : *mut *mut UChar, destcapacity : *mut i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_appendTail(regexp as _, destbuf as _, destcapacity as _, status as _) }
}
#[inline]
pub unsafe fn uregex_appendTailUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_appendTailUText(regexp : *mut URegularExpression, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_appendTailUText(regexp as _, dest as _, status as _) }
}
#[inline]
pub unsafe fn uregex_clone(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut URegularExpression {
    windows_core::link!("icuin.dll" "C" fn uregex_clone(regexp : *const URegularExpression, status : *mut UErrorCode) -> *mut URegularExpression);
    unsafe { uregex_clone(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_close() -> URegularExpression {
    windows_core::link!("icuin.dll" "C" fn uregex_close(regexp : *mut URegularExpression));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uregex_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uregex_end(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_end(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_end(regexp as _, groupnum, status as _) }
}
#[inline]
pub unsafe fn uregex_end64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn uregex_end64(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i64);
    unsafe { uregex_end64(regexp as _, groupnum, status as _) }
}
#[inline]
pub unsafe fn uregex_find(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_find(regexp : *mut URegularExpression, startindex : i32, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_find(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_find64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_find64(regexp : *mut URegularExpression, startindex : i64, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_find64(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_findNext(regexp: *mut URegularExpression, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_findNext(regexp : *mut URegularExpression, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_findNext(regexp as _, status as _) }
}
#[inline]
pub unsafe fn uregex_flags(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_flags(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_flags(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_getFindProgressCallback(regexp: *const URegularExpression, callback: *mut URegexFindProgressCallback, context: *mut *mut core::ffi::c_void, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_getFindProgressCallback(regexp : *const URegularExpression, callback : *mut URegexFindProgressCallback, context : *mut *mut core::ffi::c_void, status : *mut UErrorCode));
    unsafe { uregex_getFindProgressCallback(regexp, callback as _, context as _, status as _) }
}
#[inline]
pub unsafe fn uregex_getMatchCallback(regexp: *const URegularExpression, callback: *mut URegexMatchCallback, context: *mut *mut core::ffi::c_void, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_getMatchCallback(regexp : *const URegularExpression, callback : *mut URegexMatchCallback, context : *mut *mut core::ffi::c_void, status : *mut UErrorCode));
    unsafe { uregex_getMatchCallback(regexp, callback as _, context as _, status as _) }
}
#[inline]
pub unsafe fn uregex_getStackLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_getStackLimit(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_getStackLimit(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_getText(regexp: *mut URegularExpression, textlength: *mut i32, status: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn uregex_getText(regexp : *mut URegularExpression, textlength : *mut i32, status : *mut UErrorCode) -> *const UChar);
    unsafe { uregex_getText(regexp as _, textlength as _, status as _) }
}
#[inline]
pub unsafe fn uregex_getTimeLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_getTimeLimit(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_getTimeLimit(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_getUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_getUText(regexp : *mut URegularExpression, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_getUText(regexp as _, dest as _, status as _) }
}
#[inline]
pub unsafe fn uregex_group(regexp: *mut URegularExpression, groupnum: i32, dest: *mut UChar, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_group(regexp : *mut URegularExpression, groupnum : i32, dest : *mut UChar, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_group(regexp as _, groupnum, dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uregex_groupCount(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_groupCount(regexp : *mut URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_groupCount(regexp as _, status as _) }
}
#[inline]
pub unsafe fn uregex_groupNumberFromCName(regexp: *mut URegularExpression, groupname: *const i8, namelength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_groupNumberFromCName(regexp : *mut URegularExpression, groupname : *const i8, namelength : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_groupNumberFromCName(regexp as _, groupname, namelength, status as _) }
}
#[inline]
pub unsafe fn uregex_groupNumberFromName(regexp: *mut URegularExpression, groupname: *const UChar, namelength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_groupNumberFromName(regexp : *mut URegularExpression, groupname : *const UChar, namelength : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_groupNumberFromName(regexp as _, groupname, namelength, status as _) }
}
#[inline]
pub unsafe fn uregex_groupUText(regexp: *mut URegularExpression, groupnum: i32, dest: *mut UText, grouplength: *mut i64, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_groupUText(regexp : *mut URegularExpression, groupnum : i32, dest : *mut UText, grouplength : *mut i64, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_groupUText(regexp as _, groupnum, dest as _, grouplength as _, status as _) }
}
#[inline]
pub unsafe fn uregex_hasAnchoringBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_hasAnchoringBounds(regexp : *const URegularExpression, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_hasAnchoringBounds(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_hasTransparentBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_hasTransparentBounds(regexp : *const URegularExpression, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_hasTransparentBounds(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_hitEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_hitEnd(regexp : *const URegularExpression, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_hitEnd(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_lookingAt(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_lookingAt(regexp : *mut URegularExpression, startindex : i32, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_lookingAt(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_lookingAt64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_lookingAt64(regexp : *mut URegularExpression, startindex : i64, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_lookingAt64(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_matches(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_matches(regexp : *mut URegularExpression, startindex : i32, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_matches(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_matches64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_matches64(regexp : *mut URegularExpression, startindex : i64, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_matches64(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_open(pattern: *const UChar, patternlength: i32, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression {
    windows_core::link!("icuin.dll" "C" fn uregex_open(pattern : *const UChar, patternlength : i32, flags : u32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut URegularExpression);
    unsafe { uregex_open(pattern, patternlength, flags, pe as _, status as _) }
}
#[inline]
pub unsafe fn uregex_openC(pattern: *const i8, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression {
    windows_core::link!("icuin.dll" "C" fn uregex_openC(pattern : *const i8, flags : u32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut URegularExpression);
    unsafe { uregex_openC(pattern, flags, pe as _, status as _) }
}
#[inline]
pub unsafe fn uregex_openUText(pattern: *mut UText, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression {
    windows_core::link!("icuin.dll" "C" fn uregex_openUText(pattern : *mut UText, flags : u32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut URegularExpression);
    unsafe { uregex_openUText(pattern as _, flags, pe as _, status as _) }
}
#[inline]
pub unsafe fn uregex_pattern(regexp: *const URegularExpression, patlength: *mut i32, status: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn uregex_pattern(regexp : *const URegularExpression, patlength : *mut i32, status : *mut UErrorCode) -> *const UChar);
    unsafe { uregex_pattern(regexp, patlength as _, status as _) }
}
#[inline]
pub unsafe fn uregex_patternUText(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_patternUText(regexp : *const URegularExpression, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_patternUText(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_refreshUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_refreshUText(regexp : *mut URegularExpression, text : *mut UText, status : *mut UErrorCode));
    unsafe { uregex_refreshUText(regexp as _, text as _, status as _) }
}
#[inline]
pub unsafe fn uregex_regionEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_regionEnd(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_regionEnd(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_regionEnd64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn uregex_regionEnd64(regexp : *const URegularExpression, status : *mut UErrorCode) -> i64);
    unsafe { uregex_regionEnd64(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_regionStart(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_regionStart(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_regionStart(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_regionStart64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn uregex_regionStart64(regexp : *const URegularExpression, status : *mut UErrorCode) -> i64);
    unsafe { uregex_regionStart64(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_replaceAll(regexp: *mut URegularExpression, replacementtext: *const UChar, replacementlength: i32, destbuf: *mut UChar, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_replaceAll(regexp : *mut URegularExpression, replacementtext : *const UChar, replacementlength : i32, destbuf : *mut UChar, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_replaceAll(regexp as _, replacementtext, replacementlength, destbuf as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uregex_replaceAllUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_replaceAllUText(regexp : *mut URegularExpression, replacement : *mut UText, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_replaceAllUText(regexp as _, replacement as _, dest as _, status as _) }
}
#[inline]
pub unsafe fn uregex_replaceFirst(regexp: *mut URegularExpression, replacementtext: *const UChar, replacementlength: i32, destbuf: *mut UChar, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_replaceFirst(regexp : *mut URegularExpression, replacementtext : *const UChar, replacementlength : i32, destbuf : *mut UChar, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_replaceFirst(regexp as _, replacementtext, replacementlength, destbuf as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uregex_replaceFirstUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_replaceFirstUText(regexp : *mut URegularExpression, replacement : *mut UText, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_replaceFirstUText(regexp as _, replacement as _, dest as _, status as _) }
}
#[inline]
pub unsafe fn uregex_requireEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregex_requireEnd(regexp : *const URegularExpression, status : *mut UErrorCode) -> UBool);
    unsafe { uregex_requireEnd(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_reset(regexp: *mut URegularExpression, index: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_reset(regexp : *mut URegularExpression, index : i32, status : *mut UErrorCode));
    unsafe { uregex_reset(regexp as _, index, status as _) }
}
#[inline]
pub unsafe fn uregex_reset64(regexp: *mut URegularExpression, index: i64, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_reset64(regexp : *mut URegularExpression, index : i64, status : *mut UErrorCode));
    unsafe { uregex_reset64(regexp as _, index, status as _) }
}
#[inline]
pub unsafe fn uregex_setFindProgressCallback(regexp: *mut URegularExpression, callback: URegexFindProgressCallback, context: *const core::ffi::c_void, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setFindProgressCallback(regexp : *mut URegularExpression, callback : URegexFindProgressCallback, context : *const core::ffi::c_void, status : *mut UErrorCode));
    unsafe { uregex_setFindProgressCallback(regexp as _, callback, context, status as _) }
}
#[inline]
pub unsafe fn uregex_setMatchCallback(regexp: *mut URegularExpression, callback: URegexMatchCallback, context: *const core::ffi::c_void, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setMatchCallback(regexp : *mut URegularExpression, callback : URegexMatchCallback, context : *const core::ffi::c_void, status : *mut UErrorCode));
    unsafe { uregex_setMatchCallback(regexp as _, callback, context, status as _) }
}
#[inline]
pub unsafe fn uregex_setRegion(regexp: *mut URegularExpression, regionstart: i32, regionlimit: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setRegion(regexp : *mut URegularExpression, regionstart : i32, regionlimit : i32, status : *mut UErrorCode));
    unsafe { uregex_setRegion(regexp as _, regionstart, regionlimit, status as _) }
}
#[inline]
pub unsafe fn uregex_setRegion64(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setRegion64(regexp : *mut URegularExpression, regionstart : i64, regionlimit : i64, status : *mut UErrorCode));
    unsafe { uregex_setRegion64(regexp as _, regionstart, regionlimit, status as _) }
}
#[inline]
pub unsafe fn uregex_setRegionAndStart(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, startindex: i64, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setRegionAndStart(regexp : *mut URegularExpression, regionstart : i64, regionlimit : i64, startindex : i64, status : *mut UErrorCode));
    unsafe { uregex_setRegionAndStart(regexp as _, regionstart, regionlimit, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_setStackLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setStackLimit(regexp : *mut URegularExpression, limit : i32, status : *mut UErrorCode));
    unsafe { uregex_setStackLimit(regexp as _, limit, status as _) }
}
#[inline]
pub unsafe fn uregex_setText(regexp: *mut URegularExpression, text: *const UChar, textlength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setText(regexp : *mut URegularExpression, text : *const UChar, textlength : i32, status : *mut UErrorCode));
    unsafe { uregex_setText(regexp as _, text, textlength, status as _) }
}
#[inline]
pub unsafe fn uregex_setTimeLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setTimeLimit(regexp : *mut URegularExpression, limit : i32, status : *mut UErrorCode));
    unsafe { uregex_setTimeLimit(regexp as _, limit, status as _) }
}
#[inline]
pub unsafe fn uregex_setUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setUText(regexp : *mut URegularExpression, text : *mut UText, status : *mut UErrorCode));
    unsafe { uregex_setUText(regexp as _, text as _, status as _) }
}
#[inline]
pub unsafe fn uregex_split(regexp: *mut URegularExpression, destbuf: *mut UChar, destcapacity: i32, requiredcapacity: *mut i32, destfields: *mut *mut UChar, destfieldscapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_split(regexp : *mut URegularExpression, destbuf : *mut UChar, destcapacity : i32, requiredcapacity : *mut i32, destfields : *mut *mut UChar, destfieldscapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_split(regexp as _, destbuf as _, destcapacity, requiredcapacity as _, destfields as _, destfieldscapacity, status as _) }
}
#[inline]
pub unsafe fn uregex_splitUText(regexp: *mut URegularExpression, destfields: *mut *mut UText, destfieldscapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_splitUText(regexp : *mut URegularExpression, destfields : *mut *mut UText, destfieldscapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_splitUText(regexp as _, destfields as _, destfieldscapacity, status as _) }
}
#[inline]
pub unsafe fn uregex_start(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_start(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_start(regexp as _, groupnum, status as _) }
}
#[inline]
pub unsafe fn uregex_start64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn uregex_start64(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i64);
    unsafe { uregex_start64(regexp as _, groupnum, status as _) }
}
#[inline]
pub unsafe fn uregex_useAnchoringBounds(regexp: *mut URegularExpression, b: UBool, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_useAnchoringBounds(regexp : *mut URegularExpression, b : UBool, status : *mut UErrorCode));
    unsafe { uregex_useAnchoringBounds(regexp as _, b, status as _) }
}
#[inline]
pub unsafe fn uregex_useTransparentBounds(regexp: *mut URegularExpression, b: UBool, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_useTransparentBounds(regexp : *mut URegularExpression, b : UBool, status : *mut UErrorCode));
    unsafe { uregex_useTransparentBounds(regexp as _, b, status as _) }
}
#[inline]
pub unsafe fn uregion_areEqual(uregion: *const URegion, otherregion: *const URegion) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregion_areEqual(uregion : *const URegion, otherregion : *const URegion) -> UBool);
    unsafe { uregion_areEqual(uregion, otherregion) }
}
#[inline]
pub unsafe fn uregion_contains(uregion: *const URegion, otherregion: *const URegion) -> UBool {
    windows_core::link!("icuin.dll" "C" fn uregion_contains(uregion : *const URegion, otherregion : *const URegion) -> UBool);
    unsafe { uregion_contains(uregion, otherregion) }
}
#[inline]
pub unsafe fn uregion_getAvailable(r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn uregion_getAvailable(r#type : URegionType, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uregion_getAvailable(r#type, status as _) }
}
#[inline]
pub unsafe fn uregion_getContainedRegions(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn uregion_getContainedRegions(uregion : *const URegion, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uregion_getContainedRegions(uregion, status as _) }
}
#[inline]
pub unsafe fn uregion_getContainedRegionsOfType(uregion: *const URegion, r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn uregion_getContainedRegionsOfType(uregion : *const URegion, r#type : URegionType, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uregion_getContainedRegionsOfType(uregion, r#type, status as _) }
}
#[inline]
pub unsafe fn uregion_getContainingRegion(uregion: *const URegion) -> *const URegion {
    windows_core::link!("icuin.dll" "C" fn uregion_getContainingRegion(uregion : *const URegion) -> *const URegion);
    unsafe { uregion_getContainingRegion(uregion) }
}
#[inline]
pub unsafe fn uregion_getContainingRegionOfType(uregion: *const URegion, r#type: URegionType) -> *const URegion {
    windows_core::link!("icuin.dll" "C" fn uregion_getContainingRegionOfType(uregion : *const URegion, r#type : URegionType) -> *const URegion);
    unsafe { uregion_getContainingRegionOfType(uregion, r#type) }
}
#[inline]
pub unsafe fn uregion_getNumericCode(uregion: *const URegion) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregion_getNumericCode(uregion : *const URegion) -> i32);
    unsafe { uregion_getNumericCode(uregion) }
}
#[inline]
pub unsafe fn uregion_getPreferredValues(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn uregion_getPreferredValues(uregion : *const URegion, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uregion_getPreferredValues(uregion, status as _) }
}
#[inline]
pub unsafe fn uregion_getRegionCode(uregion: *const URegion) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn uregion_getRegionCode(uregion : *const URegion) -> *const i8);
    unsafe { uregion_getRegionCode(uregion) }
}
#[inline]
pub unsafe fn uregion_getRegionFromCode(regioncode: *const i8, status: *mut UErrorCode) -> *const URegion {
    windows_core::link!("icuin.dll" "C" fn uregion_getRegionFromCode(regioncode : *const i8, status : *mut UErrorCode) -> *const URegion);
    unsafe { uregion_getRegionFromCode(regioncode, status as _) }
}
#[inline]
pub unsafe fn uregion_getRegionFromNumericCode(code: i32, status: *mut UErrorCode) -> *const URegion {
    windows_core::link!("icuin.dll" "C" fn uregion_getRegionFromNumericCode(code : i32, status : *mut UErrorCode) -> *const URegion);
    unsafe { uregion_getRegionFromNumericCode(code, status as _) }
}
#[inline]
pub unsafe fn uregion_getType(uregion: *const URegion) -> URegionType {
    windows_core::link!("icuin.dll" "C" fn uregion_getType(uregion : *const URegion) -> URegionType);
    unsafe { uregion_getType(uregion) }
}
#[inline]
pub unsafe fn ureldatefmt_close() -> URelativeDateTimeFormatter {
    windows_core::link!("icuin.dll" "C" fn ureldatefmt_close(reldatefmt : *mut URelativeDateTimeFormatter));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ureldatefmt_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ureldatefmt_closeResult() -> UFormattedRelativeDateTime {
    windows_core::link!("icu.dll" "C" fn ureldatefmt_closeResult(ufrdt : *mut UFormattedRelativeDateTime));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ureldatefmt_closeResult(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ureldatefmt_combineDateAndTime(reldatefmt: *const URelativeDateTimeFormatter, relativedatestring: *const UChar, relativedatestringlen: i32, timestring: *const UChar, timestringlen: i32, result: *mut UChar, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ureldatefmt_combineDateAndTime(reldatefmt : *const URelativeDateTimeFormatter, relativedatestring : *const UChar, relativedatestringlen : i32, timestring : *const UChar, timestringlen : i32, result : *mut UChar, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ureldatefmt_combineDateAndTime(reldatefmt, relativedatestring, relativedatestringlen, timestring, timestringlen, result as _, resultcapacity, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_format(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UChar, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ureldatefmt_format(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut UChar, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ureldatefmt_format(reldatefmt, offset, unit, result as _, resultcapacity, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_formatNumeric(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UChar, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ureldatefmt_formatNumeric(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut UChar, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ureldatefmt_formatNumeric(reldatefmt, offset, unit, result as _, resultcapacity, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_formatNumericToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ureldatefmt_formatNumericToResult(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut UFormattedRelativeDateTime, status : *mut UErrorCode));
    unsafe { ureldatefmt_formatNumericToResult(reldatefmt, offset, unit, result as _, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_formatToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ureldatefmt_formatToResult(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut UFormattedRelativeDateTime, status : *mut UErrorCode));
    unsafe { ureldatefmt_formatToResult(reldatefmt, offset, unit, result as _, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_open(locale: *const i8, nftoadopt: *mut UNumberFormat, width: UDateRelativeDateTimeFormatterStyle, capitalizationcontext: UDisplayContext, status: *mut UErrorCode) -> *mut URelativeDateTimeFormatter {
    windows_core::link!("icuin.dll" "C" fn ureldatefmt_open(locale : *const i8, nftoadopt : *mut UNumberFormat, width : UDateRelativeDateTimeFormatterStyle, capitalizationcontext : UDisplayContext, status : *mut UErrorCode) -> *mut URelativeDateTimeFormatter);
    unsafe { ureldatefmt_open(locale, nftoadopt as _, width, capitalizationcontext, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedRelativeDateTime {
    windows_core::link!("icu.dll" "C" fn ureldatefmt_openResult(ec : *mut UErrorCode) -> *mut UFormattedRelativeDateTime);
    unsafe { ureldatefmt_openResult(ec as _) }
}
#[inline]
pub unsafe fn ureldatefmt_resultAsValue(ufrdt: *const UFormattedRelativeDateTime, ec: *mut UErrorCode) -> *const UFormattedValue {
    windows_core::link!("icu.dll" "C" fn ureldatefmt_resultAsValue(ufrdt : *const UFormattedRelativeDateTime, ec : *mut UErrorCode) -> *const UFormattedValue);
    unsafe { ureldatefmt_resultAsValue(ufrdt, ec as _) }
}
#[inline]
pub unsafe fn ures_close() -> UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn ures_close(resourcebundle : *mut UResourceBundle));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ures_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ures_getBinary(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *const u8 {
    windows_core::link!("icuuc.dll" "C" fn ures_getBinary(resourcebundle : *const UResourceBundle, len : *mut i32, status : *mut UErrorCode) -> *const u8);
    unsafe { ures_getBinary(resourcebundle, len as _, status as _) }
}
#[inline]
pub unsafe fn ures_getByIndex(resourcebundle: *const UResourceBundle, indexr: i32, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn ures_getByIndex(resourcebundle : *const UResourceBundle, indexr : i32, fillin : *mut UResourceBundle, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_getByIndex(resourcebundle, indexr, fillin as _, status as _) }
}
#[inline]
pub unsafe fn ures_getByKey(resourcebundle: *const UResourceBundle, key: *const i8, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn ures_getByKey(resourcebundle : *const UResourceBundle, key : *const i8, fillin : *mut UResourceBundle, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_getByKey(resourcebundle, key, fillin as _, status as _) }
}
#[inline]
pub unsafe fn ures_getInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ures_getInt(resourcebundle : *const UResourceBundle, status : *mut UErrorCode) -> i32);
    unsafe { ures_getInt(resourcebundle, status as _) }
}
#[inline]
pub unsafe fn ures_getIntVector(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *const i32 {
    windows_core::link!("icuuc.dll" "C" fn ures_getIntVector(resourcebundle : *const UResourceBundle, len : *mut i32, status : *mut UErrorCode) -> *const i32);
    unsafe { ures_getIntVector(resourcebundle, len as _, status as _) }
}
#[inline]
pub unsafe fn ures_getKey(resourcebundle: *const UResourceBundle) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ures_getKey(resourcebundle : *const UResourceBundle) -> *const i8);
    unsafe { ures_getKey(resourcebundle) }
}
#[inline]
pub unsafe fn ures_getLocaleByType(resourcebundle: *const UResourceBundle, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ures_getLocaleByType(resourcebundle : *const UResourceBundle, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> *const i8);
    unsafe { ures_getLocaleByType(resourcebundle, r#type, status as _) }
}
#[inline]
pub unsafe fn ures_getNextResource(resourcebundle: *mut UResourceBundle, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn ures_getNextResource(resourcebundle : *mut UResourceBundle, fillin : *mut UResourceBundle, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_getNextResource(resourcebundle as _, fillin as _, status as _) }
}
#[inline]
pub unsafe fn ures_getNextString(resourcebundle: *mut UResourceBundle, len: *mut i32, key: *mut *mut i8, status: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuuc.dll" "C" fn ures_getNextString(resourcebundle : *mut UResourceBundle, len : *mut i32, key : *mut *mut i8, status : *mut UErrorCode) -> *const UChar);
    unsafe { ures_getNextString(resourcebundle as _, len as _, key as _, status as _) }
}
#[inline]
pub unsafe fn ures_getSize(resourcebundle: *const UResourceBundle) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ures_getSize(resourcebundle : *const UResourceBundle) -> i32);
    unsafe { ures_getSize(resourcebundle) }
}
#[inline]
pub unsafe fn ures_getString(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuuc.dll" "C" fn ures_getString(resourcebundle : *const UResourceBundle, len : *mut i32, status : *mut UErrorCode) -> *const UChar);
    unsafe { ures_getString(resourcebundle, len as _, status as _) }
}
#[inline]
pub unsafe fn ures_getStringByIndex(resourcebundle: *const UResourceBundle, indexs: i32, len: *mut i32, status: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuuc.dll" "C" fn ures_getStringByIndex(resourcebundle : *const UResourceBundle, indexs : i32, len : *mut i32, status : *mut UErrorCode) -> *const UChar);
    unsafe { ures_getStringByIndex(resourcebundle, indexs, len as _, status as _) }
}
#[inline]
pub unsafe fn ures_getStringByKey(resb: *const UResourceBundle, key: *const i8, len: *mut i32, status: *mut UErrorCode) -> *const UChar {
    windows_core::link!("icuuc.dll" "C" fn ures_getStringByKey(resb : *const UResourceBundle, key : *const i8, len : *mut i32, status : *mut UErrorCode) -> *const UChar);
    unsafe { ures_getStringByKey(resb, key, len as _, status as _) }
}
#[inline]
pub unsafe fn ures_getType(resourcebundle: *const UResourceBundle) -> UResType {
    windows_core::link!("icuuc.dll" "C" fn ures_getType(resourcebundle : *const UResourceBundle) -> UResType);
    unsafe { ures_getType(resourcebundle) }
}
#[inline]
pub unsafe fn ures_getUInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn ures_getUInt(resourcebundle : *const UResourceBundle, status : *mut UErrorCode) -> u32);
    unsafe { ures_getUInt(resourcebundle, status as _) }
}
#[inline]
pub unsafe fn ures_getUTF8String(resb: *const UResourceBundle, dest: *mut i8, length: *mut i32, forcecopy: UBool, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ures_getUTF8String(resb : *const UResourceBundle, dest : *mut i8, length : *mut i32, forcecopy : UBool, status : *mut UErrorCode) -> *const i8);
    unsafe { ures_getUTF8String(resb, dest as _, length as _, forcecopy, status as _) }
}
#[inline]
pub unsafe fn ures_getUTF8StringByIndex(resb: *const UResourceBundle, stringindex: i32, dest: *mut i8, plength: *mut i32, forcecopy: UBool, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ures_getUTF8StringByIndex(resb : *const UResourceBundle, stringindex : i32, dest : *mut i8, plength : *mut i32, forcecopy : UBool, status : *mut UErrorCode) -> *const i8);
    unsafe { ures_getUTF8StringByIndex(resb, stringindex, dest as _, plength as _, forcecopy, status as _) }
}
#[inline]
pub unsafe fn ures_getUTF8StringByKey(resb: *const UResourceBundle, key: *const i8, dest: *mut i8, plength: *mut i32, forcecopy: UBool, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn ures_getUTF8StringByKey(resb : *const UResourceBundle, key : *const i8, dest : *mut i8, plength : *mut i32, forcecopy : UBool, status : *mut UErrorCode) -> *const i8);
    unsafe { ures_getUTF8StringByKey(resb, key, dest as _, plength as _, forcecopy, status as _) }
}
#[inline]
pub unsafe fn ures_getVersion(resb: *const UResourceBundle) -> u8 {
    windows_core::link!("icuuc.dll" "C" fn ures_getVersion(resb : *const UResourceBundle, versioninfo : *mut u8));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ures_getVersion(resb, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn ures_hasNext(resourcebundle: *const UResourceBundle) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn ures_hasNext(resourcebundle : *const UResourceBundle) -> UBool);
    unsafe { ures_hasNext(resourcebundle) }
}
#[inline]
pub unsafe fn ures_open(packagename: *const i8, locale: *const i8, status: *mut UErrorCode) -> *mut UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn ures_open(packagename : *const i8, locale : *const i8, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_open(packagename, locale, status as _) }
}
#[inline]
pub unsafe fn ures_openAvailableLocales(packagename: *const i8, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn ures_openAvailableLocales(packagename : *const i8, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ures_openAvailableLocales(packagename, status as _) }
}
#[inline]
pub unsafe fn ures_openDirect(packagename: *const i8, locale: *const i8, status: *mut UErrorCode) -> *mut UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn ures_openDirect(packagename : *const i8, locale : *const i8, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_openDirect(packagename, locale, status as _) }
}
#[inline]
pub unsafe fn ures_openU(packagename: *const UChar, locale: *const i8, status: *mut UErrorCode) -> *mut UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn ures_openU(packagename : *const UChar, locale : *const i8, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_openU(packagename, locale, status as _) }
}
#[inline]
pub unsafe fn ures_resetIterator() -> UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn ures_resetIterator(resourcebundle : *mut UResourceBundle));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ures_resetIterator(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uscript_breaksBetweenLetters(script: UScriptCode) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uscript_breaksBetweenLetters(script : UScriptCode) -> UBool);
    unsafe { uscript_breaksBetweenLetters(script) }
}
#[inline]
pub unsafe fn uscript_getCode(nameorabbrorlocale: *const i8, fillin: *mut UScriptCode, capacity: i32, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uscript_getCode(nameorabbrorlocale : *const i8, fillin : *mut UScriptCode, capacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uscript_getCode(nameorabbrorlocale, fillin as _, capacity, err as _) }
}
#[inline]
pub unsafe fn uscript_getName(scriptcode: UScriptCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uscript_getName(scriptcode : UScriptCode) -> *const i8);
    unsafe { uscript_getName(scriptcode) }
}
#[inline]
pub unsafe fn uscript_getSampleString(script: UScriptCode, dest: *mut UChar, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uscript_getSampleString(script : UScriptCode, dest : *mut UChar, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uscript_getSampleString(script, dest as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn uscript_getScript(codepoint: UChar32, err: *mut UErrorCode) -> UScriptCode {
    windows_core::link!("icuuc.dll" "C" fn uscript_getScript(codepoint : UChar32, err : *mut UErrorCode) -> UScriptCode);
    unsafe { uscript_getScript(codepoint, err as _) }
}
#[inline]
pub unsafe fn uscript_getScriptExtensions(c: UChar32, scripts: *mut UScriptCode, capacity: i32, errorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uscript_getScriptExtensions(c : UChar32, scripts : *mut UScriptCode, capacity : i32, errorcode : *mut UErrorCode) -> i32);
    unsafe { uscript_getScriptExtensions(c, scripts as _, capacity, errorcode as _) }
}
#[inline]
pub unsafe fn uscript_getShortName(scriptcode: UScriptCode) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn uscript_getShortName(scriptcode : UScriptCode) -> *const i8);
    unsafe { uscript_getShortName(scriptcode) }
}
#[inline]
pub unsafe fn uscript_getUsage(script: UScriptCode) -> UScriptUsage {
    windows_core::link!("icuuc.dll" "C" fn uscript_getUsage(script : UScriptCode) -> UScriptUsage);
    unsafe { uscript_getUsage(script) }
}
#[inline]
pub unsafe fn uscript_hasScript(c: UChar32, sc: UScriptCode) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uscript_hasScript(c : UChar32, sc : UScriptCode) -> UBool);
    unsafe { uscript_hasScript(c, sc) }
}
#[inline]
pub unsafe fn uscript_isCased(script: UScriptCode) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uscript_isCased(script : UScriptCode) -> UBool);
    unsafe { uscript_isCased(script) }
}
#[inline]
pub unsafe fn uscript_isRightToLeft(script: UScriptCode) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uscript_isRightToLeft(script : UScriptCode) -> UBool);
    unsafe { uscript_isRightToLeft(script) }
}
#[inline]
pub unsafe fn usearch_close() -> UStringSearch {
    windows_core::link!("icuin.dll" "C" fn usearch_close(searchiter : *mut UStringSearch));
    unsafe {
        let mut result__ = core::mem::zeroed();
        usearch_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn usearch_first(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_first(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    unsafe { usearch_first(strsrch as _, status as _) }
}
#[inline]
pub unsafe fn usearch_following(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_following(strsrch : *mut UStringSearch, position : i32, status : *mut UErrorCode) -> i32);
    unsafe { usearch_following(strsrch as _, position, status as _) }
}
#[inline]
pub unsafe fn usearch_getAttribute(strsrch: *const UStringSearch, attribute: USearchAttribute) -> USearchAttributeValue {
    windows_core::link!("icuin.dll" "C" fn usearch_getAttribute(strsrch : *const UStringSearch, attribute : USearchAttribute) -> USearchAttributeValue);
    unsafe { usearch_getAttribute(strsrch, attribute) }
}
#[inline]
pub unsafe fn usearch_getBreakIterator(strsrch: *const UStringSearch) -> *const UBreakIterator {
    windows_core::link!("icuin.dll" "C" fn usearch_getBreakIterator(strsrch : *const UStringSearch) -> *const UBreakIterator);
    unsafe { usearch_getBreakIterator(strsrch) }
}
#[inline]
pub unsafe fn usearch_getCollator(strsrch: *const UStringSearch) -> *mut UCollator {
    windows_core::link!("icuin.dll" "C" fn usearch_getCollator(strsrch : *const UStringSearch) -> *mut UCollator);
    unsafe { usearch_getCollator(strsrch) }
}
#[inline]
pub unsafe fn usearch_getMatchedLength(strsrch: *const UStringSearch) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_getMatchedLength(strsrch : *const UStringSearch) -> i32);
    unsafe { usearch_getMatchedLength(strsrch) }
}
#[inline]
pub unsafe fn usearch_getMatchedStart(strsrch: *const UStringSearch) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_getMatchedStart(strsrch : *const UStringSearch) -> i32);
    unsafe { usearch_getMatchedStart(strsrch) }
}
#[inline]
pub unsafe fn usearch_getMatchedText(strsrch: *const UStringSearch, result: *mut UChar, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_getMatchedText(strsrch : *const UStringSearch, result : *mut UChar, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { usearch_getMatchedText(strsrch, result as _, resultcapacity, status as _) }
}
#[inline]
pub unsafe fn usearch_getOffset(strsrch: *const UStringSearch) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_getOffset(strsrch : *const UStringSearch) -> i32);
    unsafe { usearch_getOffset(strsrch) }
}
#[inline]
pub unsafe fn usearch_getPattern(strsrch: *const UStringSearch, length: *mut i32) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn usearch_getPattern(strsrch : *const UStringSearch, length : *mut i32) -> *const UChar);
    unsafe { usearch_getPattern(strsrch, length as _) }
}
#[inline]
pub unsafe fn usearch_getText(strsrch: *const UStringSearch, length: *mut i32) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn usearch_getText(strsrch : *const UStringSearch, length : *mut i32) -> *const UChar);
    unsafe { usearch_getText(strsrch, length as _) }
}
#[inline]
pub unsafe fn usearch_last(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_last(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    unsafe { usearch_last(strsrch as _, status as _) }
}
#[inline]
pub unsafe fn usearch_next(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_next(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    unsafe { usearch_next(strsrch as _, status as _) }
}
#[inline]
pub unsafe fn usearch_open(pattern: *const UChar, patternlength: i32, text: *const UChar, textlength: i32, locale: *const i8, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch {
    windows_core::link!("icuin.dll" "C" fn usearch_open(pattern : *const UChar, patternlength : i32, text : *const UChar, textlength : i32, locale : *const i8, breakiter : *mut UBreakIterator, status : *mut UErrorCode) -> *mut UStringSearch);
    unsafe { usearch_open(pattern, patternlength, text, textlength, locale, breakiter as _, status as _) }
}
#[inline]
pub unsafe fn usearch_openFromCollator(pattern: *const UChar, patternlength: i32, text: *const UChar, textlength: i32, collator: *const UCollator, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch {
    windows_core::link!("icuin.dll" "C" fn usearch_openFromCollator(pattern : *const UChar, patternlength : i32, text : *const UChar, textlength : i32, collator : *const UCollator, breakiter : *mut UBreakIterator, status : *mut UErrorCode) -> *mut UStringSearch);
    unsafe { usearch_openFromCollator(pattern, patternlength, text, textlength, collator, breakiter as _, status as _) }
}
#[inline]
pub unsafe fn usearch_preceding(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_preceding(strsrch : *mut UStringSearch, position : i32, status : *mut UErrorCode) -> i32);
    unsafe { usearch_preceding(strsrch as _, position, status as _) }
}
#[inline]
pub unsafe fn usearch_previous(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_previous(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    unsafe { usearch_previous(strsrch as _, status as _) }
}
#[inline]
pub unsafe fn usearch_reset() -> UStringSearch {
    windows_core::link!("icuin.dll" "C" fn usearch_reset(strsrch : *mut UStringSearch));
    unsafe {
        let mut result__ = core::mem::zeroed();
        usearch_reset(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn usearch_setAttribute(strsrch: *mut UStringSearch, attribute: USearchAttribute, value: USearchAttributeValue, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setAttribute(strsrch : *mut UStringSearch, attribute : USearchAttribute, value : USearchAttributeValue, status : *mut UErrorCode));
    unsafe { usearch_setAttribute(strsrch as _, attribute, value, status as _) }
}
#[inline]
pub unsafe fn usearch_setBreakIterator(strsrch: *mut UStringSearch, breakiter: *mut UBreakIterator, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setBreakIterator(strsrch : *mut UStringSearch, breakiter : *mut UBreakIterator, status : *mut UErrorCode));
    unsafe { usearch_setBreakIterator(strsrch as _, breakiter as _, status as _) }
}
#[inline]
pub unsafe fn usearch_setCollator(strsrch: *mut UStringSearch, collator: *const UCollator, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setCollator(strsrch : *mut UStringSearch, collator : *const UCollator, status : *mut UErrorCode));
    unsafe { usearch_setCollator(strsrch as _, collator, status as _) }
}
#[inline]
pub unsafe fn usearch_setOffset(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setOffset(strsrch : *mut UStringSearch, position : i32, status : *mut UErrorCode));
    unsafe { usearch_setOffset(strsrch as _, position, status as _) }
}
#[inline]
pub unsafe fn usearch_setPattern(strsrch: *mut UStringSearch, pattern: *const UChar, patternlength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setPattern(strsrch : *mut UStringSearch, pattern : *const UChar, patternlength : i32, status : *mut UErrorCode));
    unsafe { usearch_setPattern(strsrch as _, pattern, patternlength, status as _) }
}
#[inline]
pub unsafe fn usearch_setText(strsrch: *mut UStringSearch, text: *const UChar, textlength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setText(strsrch : *mut UStringSearch, text : *const UChar, textlength : i32, status : *mut UErrorCode));
    unsafe { usearch_setText(strsrch as _, text, textlength, status as _) }
}
#[inline]
pub unsafe fn uset_add(set: *mut USet, c: UChar32) {
    windows_core::link!("icuuc.dll" "C" fn uset_add(set : *mut USet, c : UChar32));
    unsafe { uset_add(set as _, c) }
}
#[inline]
pub unsafe fn uset_addAll(set: *mut USet, additionalset: *const USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_addAll(set : *mut USet, additionalset : *const USet));
    unsafe { uset_addAll(set as _, additionalset) }
}
#[inline]
pub unsafe fn uset_addAllCodePoints(set: *mut USet, str: *const UChar, strlen: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_addAllCodePoints(set : *mut USet, str : *const UChar, strlen : i32));
    unsafe { uset_addAllCodePoints(set as _, str, strlen) }
}
#[inline]
pub unsafe fn uset_addRange(set: *mut USet, start: UChar32, end: UChar32) {
    windows_core::link!("icuuc.dll" "C" fn uset_addRange(set : *mut USet, start : UChar32, end : UChar32));
    unsafe { uset_addRange(set as _, start, end) }
}
#[inline]
pub unsafe fn uset_addString(set: *mut USet, str: *const UChar, strlen: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_addString(set : *mut USet, str : *const UChar, strlen : i32));
    unsafe { uset_addString(set as _, str, strlen) }
}
#[inline]
pub unsafe fn uset_applyIntPropertyValue(set: *mut USet, prop: UProperty, value: i32, ec: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn uset_applyIntPropertyValue(set : *mut USet, prop : UProperty, value : i32, ec : *mut UErrorCode));
    unsafe { uset_applyIntPropertyValue(set as _, prop, value, ec as _) }
}
#[inline]
pub unsafe fn uset_applyPattern(set: *mut USet, pattern: *const UChar, patternlength: i32, options: u32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_applyPattern(set : *mut USet, pattern : *const UChar, patternlength : i32, options : u32, status : *mut UErrorCode) -> i32);
    unsafe { uset_applyPattern(set as _, pattern, patternlength, options, status as _) }
}
#[inline]
pub unsafe fn uset_applyPropertyAlias(set: *mut USet, prop: *const UChar, proplength: i32, value: *const UChar, valuelength: i32, ec: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn uset_applyPropertyAlias(set : *mut USet, prop : *const UChar, proplength : i32, value : *const UChar, valuelength : i32, ec : *mut UErrorCode));
    unsafe { uset_applyPropertyAlias(set as _, prop, proplength, value, valuelength, ec as _) }
}
#[inline]
pub unsafe fn uset_charAt(set: *const USet, charindex: i32) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn uset_charAt(set : *const USet, charindex : i32) -> UChar32);
    unsafe { uset_charAt(set, charindex) }
}
#[inline]
pub unsafe fn uset_clear() -> USet {
    windows_core::link!("icuuc.dll" "C" fn uset_clear(set : *mut USet));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uset_clear(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uset_clone(set: *const USet) -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_clone(set : *const USet) -> *mut USet);
    unsafe { uset_clone(set) }
}
#[inline]
pub unsafe fn uset_cloneAsThawed(set: *const USet) -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_cloneAsThawed(set : *const USet) -> *mut USet);
    unsafe { uset_cloneAsThawed(set) }
}
#[inline]
pub unsafe fn uset_close() -> USet {
    windows_core::link!("icuuc.dll" "C" fn uset_close(set : *mut USet));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uset_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uset_closeOver(set: *mut USet, attributes: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_closeOver(set : *mut USet, attributes : i32));
    unsafe { uset_closeOver(set as _, attributes) }
}
#[inline]
pub unsafe fn uset_compact() -> USet {
    windows_core::link!("icuuc.dll" "C" fn uset_compact(set : *mut USet));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uset_compact(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uset_complement() -> USet {
    windows_core::link!("icuuc.dll" "C" fn uset_complement(set : *mut USet));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uset_complement(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uset_complementAll(set: *mut USet, complement: *const USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_complementAll(set : *mut USet, complement : *const USet));
    unsafe { uset_complementAll(set as _, complement) }
}
#[inline]
pub unsafe fn uset_complementAllCodePoints(set: *mut USet, str: *const UChar, length: i32) {
    windows_core::link!("icu.dll" "C" fn uset_complementAllCodePoints(set : *mut USet, str : *const UChar, length : i32));
    unsafe { uset_complementAllCodePoints(set as _, str, length) }
}
#[inline]
pub unsafe fn uset_complementRange(set: *mut USet, start: UChar32, end: UChar32) {
    windows_core::link!("icu.dll" "C" fn uset_complementRange(set : *mut USet, start : UChar32, end : UChar32));
    unsafe { uset_complementRange(set as _, start, end) }
}
#[inline]
pub unsafe fn uset_complementString(set: *mut USet, str: *const UChar, length: i32) {
    windows_core::link!("icu.dll" "C" fn uset_complementString(set : *mut USet, str : *const UChar, length : i32));
    unsafe { uset_complementString(set as _, str, length) }
}
#[inline]
pub unsafe fn uset_contains(set: *const USet, c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_contains(set : *const USet, c : UChar32) -> UBool);
    unsafe { uset_contains(set, c) }
}
#[inline]
pub unsafe fn uset_containsAll(set1: *const USet, set2: *const USet) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_containsAll(set1 : *const USet, set2 : *const USet) -> UBool);
    unsafe { uset_containsAll(set1, set2) }
}
#[inline]
pub unsafe fn uset_containsAllCodePoints(set: *const USet, str: *const UChar, strlen: i32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_containsAllCodePoints(set : *const USet, str : *const UChar, strlen : i32) -> UBool);
    unsafe { uset_containsAllCodePoints(set, str, strlen) }
}
#[inline]
pub unsafe fn uset_containsNone(set1: *const USet, set2: *const USet) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_containsNone(set1 : *const USet, set2 : *const USet) -> UBool);
    unsafe { uset_containsNone(set1, set2) }
}
#[inline]
pub unsafe fn uset_containsRange(set: *const USet, start: UChar32, end: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_containsRange(set : *const USet, start : UChar32, end : UChar32) -> UBool);
    unsafe { uset_containsRange(set, start, end) }
}
#[inline]
pub unsafe fn uset_containsSome(set1: *const USet, set2: *const USet) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_containsSome(set1 : *const USet, set2 : *const USet) -> UBool);
    unsafe { uset_containsSome(set1, set2) }
}
#[inline]
pub unsafe fn uset_containsString(set: *const USet, str: *const UChar, strlen: i32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_containsString(set : *const USet, str : *const UChar, strlen : i32) -> UBool);
    unsafe { uset_containsString(set, str, strlen) }
}
#[inline]
pub unsafe fn uset_equals(set1: *const USet, set2: *const USet) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_equals(set1 : *const USet, set2 : *const USet) -> UBool);
    unsafe { uset_equals(set1, set2) }
}
#[inline]
pub unsafe fn uset_freeze() -> USet {
    windows_core::link!("icuuc.dll" "C" fn uset_freeze(set : *mut USet));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uset_freeze(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uset_getItem(set: *const USet, itemindex: i32, start: *mut UChar32, end: *mut UChar32, str: *mut UChar, strcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_getItem(set : *const USet, itemindex : i32, start : *mut UChar32, end : *mut UChar32, str : *mut UChar, strcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { uset_getItem(set, itemindex, start as _, end as _, str as _, strcapacity, ec as _) }
}
#[inline]
pub unsafe fn uset_getItemCount(set: *const USet) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_getItemCount(set : *const USet) -> i32);
    unsafe { uset_getItemCount(set) }
}
#[inline]
pub unsafe fn uset_getRangeCount(set: *const USet) -> i32 {
    windows_core::link!("icu.dll" "C" fn uset_getRangeCount(set : *const USet) -> i32);
    unsafe { uset_getRangeCount(set) }
}
#[inline]
pub unsafe fn uset_getSerializedRange(set: *const USerializedSet, rangeindex: i32, pstart: *mut UChar32, pend: *mut UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_getSerializedRange(set : *const USerializedSet, rangeindex : i32, pstart : *mut UChar32, pend : *mut UChar32) -> UBool);
    unsafe { uset_getSerializedRange(set, rangeindex, pstart as _, pend as _) }
}
#[inline]
pub unsafe fn uset_getSerializedRangeCount(set: *const USerializedSet) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_getSerializedRangeCount(set : *const USerializedSet) -> i32);
    unsafe { uset_getSerializedRangeCount(set) }
}
#[inline]
pub unsafe fn uset_getSerializedSet(fillset: *mut USerializedSet, src: *const u16, srclength: i32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_getSerializedSet(fillset : *mut USerializedSet, src : *const u16, srclength : i32) -> UBool);
    unsafe { uset_getSerializedSet(fillset as _, src, srclength) }
}
#[inline]
pub unsafe fn uset_hasStrings(set: *const USet) -> UBool {
    windows_core::link!("icu.dll" "C" fn uset_hasStrings(set : *const USet) -> UBool);
    unsafe { uset_hasStrings(set) }
}
#[inline]
pub unsafe fn uset_indexOf(set: *const USet, c: UChar32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_indexOf(set : *const USet, c : UChar32) -> i32);
    unsafe { uset_indexOf(set, c) }
}
#[inline]
pub unsafe fn uset_isEmpty(set: *const USet) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_isEmpty(set : *const USet) -> UBool);
    unsafe { uset_isEmpty(set) }
}
#[inline]
pub unsafe fn uset_isFrozen(set: *const USet) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_isFrozen(set : *const USet) -> UBool);
    unsafe { uset_isFrozen(set) }
}
#[inline]
pub unsafe fn uset_open(start: UChar32, end: UChar32) -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_open(start : UChar32, end : UChar32) -> *mut USet);
    unsafe { uset_open(start, end) }
}
#[inline]
pub unsafe fn uset_openEmpty() -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_openEmpty() -> *mut USet);
    unsafe { uset_openEmpty() }
}
#[inline]
pub unsafe fn uset_openPattern(pattern: *const UChar, patternlength: i32, ec: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_openPattern(pattern : *const UChar, patternlength : i32, ec : *mut UErrorCode) -> *mut USet);
    unsafe { uset_openPattern(pattern, patternlength, ec as _) }
}
#[inline]
pub unsafe fn uset_openPatternOptions(pattern: *const UChar, patternlength: i32, options: u32, ec: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_openPatternOptions(pattern : *const UChar, patternlength : i32, options : u32, ec : *mut UErrorCode) -> *mut USet);
    unsafe { uset_openPatternOptions(pattern, patternlength, options, ec as _) }
}
#[inline]
pub unsafe fn uset_remove(set: *mut USet, c: UChar32) {
    windows_core::link!("icuuc.dll" "C" fn uset_remove(set : *mut USet, c : UChar32));
    unsafe { uset_remove(set as _, c) }
}
#[inline]
pub unsafe fn uset_removeAll(set: *mut USet, removeset: *const USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_removeAll(set : *mut USet, removeset : *const USet));
    unsafe { uset_removeAll(set as _, removeset) }
}
#[inline]
pub unsafe fn uset_removeAllCodePoints(set: *mut USet, str: *const UChar, length: i32) {
    windows_core::link!("icu.dll" "C" fn uset_removeAllCodePoints(set : *mut USet, str : *const UChar, length : i32));
    unsafe { uset_removeAllCodePoints(set as _, str, length) }
}
#[inline]
pub unsafe fn uset_removeAllStrings() -> USet {
    windows_core::link!("icuuc.dll" "C" fn uset_removeAllStrings(set : *mut USet));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uset_removeAllStrings(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uset_removeRange(set: *mut USet, start: UChar32, end: UChar32) {
    windows_core::link!("icuuc.dll" "C" fn uset_removeRange(set : *mut USet, start : UChar32, end : UChar32));
    unsafe { uset_removeRange(set as _, start, end) }
}
#[inline]
pub unsafe fn uset_removeString(set: *mut USet, str: *const UChar, strlen: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_removeString(set : *mut USet, str : *const UChar, strlen : i32));
    unsafe { uset_removeString(set as _, str, strlen) }
}
#[inline]
pub unsafe fn uset_resemblesPattern(pattern: *const UChar, patternlength: i32, pos: i32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_resemblesPattern(pattern : *const UChar, patternlength : i32, pos : i32) -> UBool);
    unsafe { uset_resemblesPattern(pattern, patternlength, pos) }
}
#[inline]
pub unsafe fn uset_retain(set: *mut USet, start: UChar32, end: UChar32) {
    windows_core::link!("icuuc.dll" "C" fn uset_retain(set : *mut USet, start : UChar32, end : UChar32));
    unsafe { uset_retain(set as _, start, end) }
}
#[inline]
pub unsafe fn uset_retainAll(set: *mut USet, retain: *const USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_retainAll(set : *mut USet, retain : *const USet));
    unsafe { uset_retainAll(set as _, retain) }
}
#[inline]
pub unsafe fn uset_retainAllCodePoints(set: *mut USet, str: *const UChar, length: i32) {
    windows_core::link!("icu.dll" "C" fn uset_retainAllCodePoints(set : *mut USet, str : *const UChar, length : i32));
    unsafe { uset_retainAllCodePoints(set as _, str, length) }
}
#[inline]
pub unsafe fn uset_retainString(set: *mut USet, str: *const UChar, length: i32) {
    windows_core::link!("icu.dll" "C" fn uset_retainString(set : *mut USet, str : *const UChar, length : i32));
    unsafe { uset_retainString(set as _, str, length) }
}
#[inline]
pub unsafe fn uset_serialize(set: *const USet, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_serialize(set : *const USet, dest : *mut u16, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uset_serialize(set, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn uset_serializedContains(set: *const USerializedSet, c: UChar32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn uset_serializedContains(set : *const USerializedSet, c : UChar32) -> UBool);
    unsafe { uset_serializedContains(set, c) }
}
#[inline]
pub unsafe fn uset_set(set: *mut USet, start: UChar32, end: UChar32) {
    windows_core::link!("icuuc.dll" "C" fn uset_set(set : *mut USet, start : UChar32, end : UChar32));
    unsafe { uset_set(set as _, start, end) }
}
#[inline]
pub unsafe fn uset_setSerializedToOne(fillset: *mut USerializedSet, c: UChar32) {
    windows_core::link!("icuuc.dll" "C" fn uset_setSerializedToOne(fillset : *mut USerializedSet, c : UChar32));
    unsafe { uset_setSerializedToOne(fillset as _, c) }
}
#[inline]
pub unsafe fn uset_size(set: *const USet) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_size(set : *const USet) -> i32);
    unsafe { uset_size(set) }
}
#[inline]
pub unsafe fn uset_span(set: *const USet, s: *const UChar, length: i32, spancondition: USetSpanCondition) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_span(set : *const USet, s : *const UChar, length : i32, spancondition : USetSpanCondition) -> i32);
    unsafe { uset_span(set, s, length, spancondition) }
}
#[inline]
pub unsafe fn uset_spanBack(set: *const USet, s: *const UChar, length: i32, spancondition: USetSpanCondition) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_spanBack(set : *const USet, s : *const UChar, length : i32, spancondition : USetSpanCondition) -> i32);
    unsafe { uset_spanBack(set, s, length, spancondition) }
}
#[inline]
pub unsafe fn uset_spanBackUTF8(set: *const USet, s: *const i8, length: i32, spancondition: USetSpanCondition) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_spanBackUTF8(set : *const USet, s : *const i8, length : i32, spancondition : USetSpanCondition) -> i32);
    unsafe { uset_spanBackUTF8(set, s, length, spancondition) }
}
#[inline]
pub unsafe fn uset_spanUTF8(set: *const USet, s: *const i8, length: i32, spancondition: USetSpanCondition) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_spanUTF8(set : *const USet, s : *const i8, length : i32, spancondition : USetSpanCondition) -> i32);
    unsafe { uset_spanUTF8(set, s, length, spancondition) }
}
#[inline]
pub unsafe fn uset_toPattern(set: *const USet, result: *mut UChar, resultcapacity: i32, escapeunprintable: UBool, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_toPattern(set : *const USet, result : *mut UChar, resultcapacity : i32, escapeunprintable : UBool, ec : *mut UErrorCode) -> i32);
    unsafe { uset_toPattern(set, result as _, resultcapacity, escapeunprintable, ec as _) }
}
#[inline]
pub unsafe fn uspoof_areConfusable(sc: *const USpoofChecker, id1: *const UChar, length1: i32, id2: *const UChar, length2: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_areConfusable(sc : *const USpoofChecker, id1 : *const UChar, length1 : i32, id2 : *const UChar, length2 : i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_areConfusable(sc, id1, length1, id2, length2, status as _) }
}
#[inline]
pub unsafe fn uspoof_areConfusableUTF8(sc: *const USpoofChecker, id1: *const i8, length1: i32, id2: *const i8, length2: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_areConfusableUTF8(sc : *const USpoofChecker, id1 : *const i8, length1 : i32, id2 : *const i8, length2 : i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_areConfusableUTF8(sc, id1, length1, id2, length2, status as _) }
}
#[inline]
pub unsafe fn uspoof_check(sc: *const USpoofChecker, id: *const UChar, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_check(sc : *const USpoofChecker, id : *const UChar, length : i32, position : *mut i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_check(sc, id, length, position as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_check2(sc: *const USpoofChecker, id: *const UChar, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_check2(sc : *const USpoofChecker, id : *const UChar, length : i32, checkresult : *mut USpoofCheckResult, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_check2(sc, id, length, checkresult as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_check2UTF8(sc: *const USpoofChecker, id: *const i8, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_check2UTF8(sc : *const USpoofChecker, id : *const i8, length : i32, checkresult : *mut USpoofCheckResult, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_check2UTF8(sc, id, length, checkresult as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_checkUTF8(sc: *const USpoofChecker, id: *const i8, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_checkUTF8(sc : *const USpoofChecker, id : *const i8, length : i32, position : *mut i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_checkUTF8(sc, id, length, position as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_clone(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USpoofChecker {
    windows_core::link!("icuin.dll" "C" fn uspoof_clone(sc : *const USpoofChecker, status : *mut UErrorCode) -> *mut USpoofChecker);
    unsafe { uspoof_clone(sc, status as _) }
}
#[inline]
pub unsafe fn uspoof_close() -> USpoofChecker {
    windows_core::link!("icuin.dll" "C" fn uspoof_close(sc : *mut USpoofChecker));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uspoof_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uspoof_closeCheckResult() -> USpoofCheckResult {
    windows_core::link!("icuin.dll" "C" fn uspoof_closeCheckResult(checkresult : *mut USpoofCheckResult));
    unsafe {
        let mut result__ = core::mem::zeroed();
        uspoof_closeCheckResult(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn uspoof_getAllowedChars(sc: *const USpoofChecker, status: *mut UErrorCode) -> *const USet {
    windows_core::link!("icuin.dll" "C" fn uspoof_getAllowedChars(sc : *const USpoofChecker, status : *mut UErrorCode) -> *const USet);
    unsafe { uspoof_getAllowedChars(sc, status as _) }
}
#[inline]
pub unsafe fn uspoof_getAllowedLocales(sc: *mut USpoofChecker, status: *mut UErrorCode) -> *const i8 {
    windows_core::link!("icuin.dll" "C" fn uspoof_getAllowedLocales(sc : *mut USpoofChecker, status : *mut UErrorCode) -> *const i8);
    unsafe { uspoof_getAllowedLocales(sc as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_getCheckResultChecks(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_getCheckResultChecks(checkresult : *const USpoofCheckResult, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_getCheckResultChecks(checkresult, status as _) }
}
#[inline]
pub unsafe fn uspoof_getCheckResultNumerics(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> *const USet {
    windows_core::link!("icuin.dll" "C" fn uspoof_getCheckResultNumerics(checkresult : *const USpoofCheckResult, status : *mut UErrorCode) -> *const USet);
    unsafe { uspoof_getCheckResultNumerics(checkresult, status as _) }
}
#[inline]
pub unsafe fn uspoof_getCheckResultRestrictionLevel(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> URestrictionLevel {
    windows_core::link!("icuin.dll" "C" fn uspoof_getCheckResultRestrictionLevel(checkresult : *const USpoofCheckResult, status : *mut UErrorCode) -> URestrictionLevel);
    unsafe { uspoof_getCheckResultRestrictionLevel(checkresult, status as _) }
}
#[inline]
pub unsafe fn uspoof_getChecks(sc: *const USpoofChecker, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_getChecks(sc : *const USpoofChecker, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_getChecks(sc, status as _) }
}
#[inline]
pub unsafe fn uspoof_getInclusionSet(status: *mut UErrorCode) -> *const USet {
    windows_core::link!("icuin.dll" "C" fn uspoof_getInclusionSet(status : *mut UErrorCode) -> *const USet);
    unsafe { uspoof_getInclusionSet(status as _) }
}
#[inline]
pub unsafe fn uspoof_getRecommendedSet(status: *mut UErrorCode) -> *const USet {
    windows_core::link!("icuin.dll" "C" fn uspoof_getRecommendedSet(status : *mut UErrorCode) -> *const USet);
    unsafe { uspoof_getRecommendedSet(status as _) }
}
#[inline]
pub unsafe fn uspoof_getRestrictionLevel(sc: *const USpoofChecker) -> URestrictionLevel {
    windows_core::link!("icuin.dll" "C" fn uspoof_getRestrictionLevel(sc : *const USpoofChecker) -> URestrictionLevel);
    unsafe { uspoof_getRestrictionLevel(sc) }
}
#[inline]
pub unsafe fn uspoof_getSkeleton(sc: *const USpoofChecker, r#type: u32, id: *const UChar, length: i32, dest: *mut UChar, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_getSkeleton(sc : *const USpoofChecker, r#type : u32, id : *const UChar, length : i32, dest : *mut UChar, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_getSkeleton(sc, r#type, id, length, dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uspoof_getSkeletonUTF8(sc: *const USpoofChecker, r#type: u32, id: *const i8, length: i32, dest: *mut i8, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_getSkeletonUTF8(sc : *const USpoofChecker, r#type : u32, id : *const i8, length : i32, dest : *mut i8, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_getSkeletonUTF8(sc, r#type, id, length, dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uspoof_open(status: *mut UErrorCode) -> *mut USpoofChecker {
    windows_core::link!("icuin.dll" "C" fn uspoof_open(status : *mut UErrorCode) -> *mut USpoofChecker);
    unsafe { uspoof_open(status as _) }
}
#[inline]
pub unsafe fn uspoof_openCheckResult(status: *mut UErrorCode) -> *mut USpoofCheckResult {
    windows_core::link!("icuin.dll" "C" fn uspoof_openCheckResult(status : *mut UErrorCode) -> *mut USpoofCheckResult);
    unsafe { uspoof_openCheckResult(status as _) }
}
#[inline]
pub unsafe fn uspoof_openFromSerialized(data: *const core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut USpoofChecker {
    windows_core::link!("icuin.dll" "C" fn uspoof_openFromSerialized(data : *const core::ffi::c_void, length : i32, pactuallength : *mut i32, perrorcode : *mut UErrorCode) -> *mut USpoofChecker);
    unsafe { uspoof_openFromSerialized(data, length, pactuallength as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uspoof_openFromSource(confusables: *const i8, confusableslen: i32, confusableswholescript: *const i8, confusableswholescriptlen: i32, errtype: *mut i32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut USpoofChecker {
    windows_core::link!("icuin.dll" "C" fn uspoof_openFromSource(confusables : *const i8, confusableslen : i32, confusableswholescript : *const i8, confusableswholescriptlen : i32, errtype : *mut i32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut USpoofChecker);
    unsafe { uspoof_openFromSource(confusables, confusableslen, confusableswholescript, confusableswholescriptlen, errtype as _, pe as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_serialize(sc: *mut USpoofChecker, data: *mut core::ffi::c_void, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_serialize(sc : *mut USpoofChecker, data : *mut core::ffi::c_void, capacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_serialize(sc as _, data as _, capacity, status as _) }
}
#[inline]
pub unsafe fn uspoof_setAllowedChars(sc: *mut USpoofChecker, chars: *const USet, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uspoof_setAllowedChars(sc : *mut USpoofChecker, chars : *const USet, status : *mut UErrorCode));
    unsafe { uspoof_setAllowedChars(sc as _, chars, status as _) }
}
#[inline]
pub unsafe fn uspoof_setAllowedLocales(sc: *mut USpoofChecker, localeslist: *const i8, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uspoof_setAllowedLocales(sc : *mut USpoofChecker, localeslist : *const i8, status : *mut UErrorCode));
    unsafe { uspoof_setAllowedLocales(sc as _, localeslist, status as _) }
}
#[inline]
pub unsafe fn uspoof_setChecks(sc: *mut USpoofChecker, checks: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uspoof_setChecks(sc : *mut USpoofChecker, checks : i32, status : *mut UErrorCode));
    unsafe { uspoof_setChecks(sc as _, checks, status as _) }
}
#[inline]
pub unsafe fn uspoof_setRestrictionLevel(sc: *mut USpoofChecker, restrictionlevel: URestrictionLevel) {
    windows_core::link!("icuin.dll" "C" fn uspoof_setRestrictionLevel(sc : *mut USpoofChecker, restrictionlevel : URestrictionLevel));
    unsafe { uspoof_setRestrictionLevel(sc as _, restrictionlevel) }
}
#[inline]
pub unsafe fn usprep_close() -> UStringPrepProfile {
    windows_core::link!("icuuc.dll" "C" fn usprep_close(profile : *mut UStringPrepProfile));
    unsafe {
        let mut result__ = core::mem::zeroed();
        usprep_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn usprep_open(path: *const i8, filename: *const i8, status: *mut UErrorCode) -> *mut UStringPrepProfile {
    windows_core::link!("icuuc.dll" "C" fn usprep_open(path : *const i8, filename : *const i8, status : *mut UErrorCode) -> *mut UStringPrepProfile);
    unsafe { usprep_open(path, filename, status as _) }
}
#[inline]
pub unsafe fn usprep_openByType(r#type: UStringPrepProfileType, status: *mut UErrorCode) -> *mut UStringPrepProfile {
    windows_core::link!("icuuc.dll" "C" fn usprep_openByType(r#type : UStringPrepProfileType, status : *mut UErrorCode) -> *mut UStringPrepProfile);
    unsafe { usprep_openByType(r#type, status as _) }
}
#[inline]
pub unsafe fn usprep_prepare(prep: *const UStringPrepProfile, src: *const UChar, srclength: i32, dest: *mut UChar, destcapacity: i32, options: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn usprep_prepare(prep : *const UStringPrepProfile, src : *const UChar, srclength : i32, dest : *mut UChar, destcapacity : i32, options : i32, parseerror : *mut UParseError, status : *mut UErrorCode) -> i32);
    unsafe { usprep_prepare(prep, src, srclength, dest as _, destcapacity, options, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn utext_char32At(ut: *mut UText, nativeindex: i64) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn utext_char32At(ut : *mut UText, nativeindex : i64) -> UChar32);
    unsafe { utext_char32At(ut as _, nativeindex) }
}
#[inline]
pub unsafe fn utext_clone(dest: *mut UText, src: *const UText, deep: UBool, readonly: UBool, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuuc.dll" "C" fn utext_clone(dest : *mut UText, src : *const UText, deep : UBool, readonly : UBool, status : *mut UErrorCode) -> *mut UText);
    unsafe { utext_clone(dest as _, src, deep, readonly, status as _) }
}
#[inline]
pub unsafe fn utext_close(ut: *mut UText) -> *mut UText {
    windows_core::link!("icuuc.dll" "C" fn utext_close(ut : *mut UText) -> *mut UText);
    unsafe { utext_close(ut as _) }
}
#[inline]
pub unsafe fn utext_copy(ut: *mut UText, nativestart: i64, nativelimit: i64, destindex: i64, r#move: UBool, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn utext_copy(ut : *mut UText, nativestart : i64, nativelimit : i64, destindex : i64, r#move : UBool, status : *mut UErrorCode));
    unsafe { utext_copy(ut as _, nativestart, nativelimit, destindex, r#move, status as _) }
}
#[inline]
pub unsafe fn utext_current32(ut: *mut UText) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn utext_current32(ut : *mut UText) -> UChar32);
    unsafe { utext_current32(ut as _) }
}
#[inline]
pub unsafe fn utext_equals(a: *const UText, b: *const UText) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn utext_equals(a : *const UText, b : *const UText) -> UBool);
    unsafe { utext_equals(a, b) }
}
#[inline]
pub unsafe fn utext_extract(ut: *mut UText, nativestart: i64, nativelimit: i64, dest: *mut UChar, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utext_extract(ut : *mut UText, nativestart : i64, nativelimit : i64, dest : *mut UChar, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { utext_extract(ut as _, nativestart, nativelimit, dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn utext_freeze(ut: *mut UText) {
    windows_core::link!("icuuc.dll" "C" fn utext_freeze(ut : *mut UText));
    unsafe { utext_freeze(ut as _) }
}
#[inline]
pub unsafe fn utext_getNativeIndex(ut: *const UText) -> i64 {
    windows_core::link!("icuuc.dll" "C" fn utext_getNativeIndex(ut : *const UText) -> i64);
    unsafe { utext_getNativeIndex(ut) }
}
#[inline]
pub unsafe fn utext_getPreviousNativeIndex(ut: *mut UText) -> i64 {
    windows_core::link!("icuuc.dll" "C" fn utext_getPreviousNativeIndex(ut : *mut UText) -> i64);
    unsafe { utext_getPreviousNativeIndex(ut as _) }
}
#[inline]
pub unsafe fn utext_hasMetaData(ut: *const UText) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn utext_hasMetaData(ut : *const UText) -> UBool);
    unsafe { utext_hasMetaData(ut) }
}
#[inline]
pub unsafe fn utext_isLengthExpensive(ut: *const UText) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn utext_isLengthExpensive(ut : *const UText) -> UBool);
    unsafe { utext_isLengthExpensive(ut) }
}
#[inline]
pub unsafe fn utext_isWritable(ut: *const UText) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn utext_isWritable(ut : *const UText) -> UBool);
    unsafe { utext_isWritable(ut) }
}
#[inline]
pub unsafe fn utext_moveIndex32(ut: *mut UText, delta: i32) -> UBool {
    windows_core::link!("icuuc.dll" "C" fn utext_moveIndex32(ut : *mut UText, delta : i32) -> UBool);
    unsafe { utext_moveIndex32(ut as _, delta) }
}
#[inline]
pub unsafe fn utext_nativeLength(ut: *mut UText) -> i64 {
    windows_core::link!("icuuc.dll" "C" fn utext_nativeLength(ut : *mut UText) -> i64);
    unsafe { utext_nativeLength(ut as _) }
}
#[inline]
pub unsafe fn utext_next32(ut: *mut UText) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn utext_next32(ut : *mut UText) -> UChar32);
    unsafe { utext_next32(ut as _) }
}
#[inline]
pub unsafe fn utext_next32From(ut: *mut UText, nativeindex: i64) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn utext_next32From(ut : *mut UText, nativeindex : i64) -> UChar32);
    unsafe { utext_next32From(ut as _, nativeindex) }
}
#[inline]
pub unsafe fn utext_openUChars(ut: *mut UText, s: *const UChar, length: i64, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuuc.dll" "C" fn utext_openUChars(ut : *mut UText, s : *const UChar, length : i64, status : *mut UErrorCode) -> *mut UText);
    unsafe { utext_openUChars(ut as _, s, length, status as _) }
}
#[inline]
pub unsafe fn utext_openUTF8(ut: *mut UText, s: *const i8, length: i64, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuuc.dll" "C" fn utext_openUTF8(ut : *mut UText, s : *const i8, length : i64, status : *mut UErrorCode) -> *mut UText);
    unsafe { utext_openUTF8(ut as _, s, length, status as _) }
}
#[inline]
pub unsafe fn utext_previous32(ut: *mut UText) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn utext_previous32(ut : *mut UText) -> UChar32);
    unsafe { utext_previous32(ut as _) }
}
#[inline]
pub unsafe fn utext_previous32From(ut: *mut UText, nativeindex: i64) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn utext_previous32From(ut : *mut UText, nativeindex : i64) -> UChar32);
    unsafe { utext_previous32From(ut as _, nativeindex) }
}
#[inline]
pub unsafe fn utext_replace(ut: *mut UText, nativestart: i64, nativelimit: i64, replacementtext: *const UChar, replacementlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utext_replace(ut : *mut UText, nativestart : i64, nativelimit : i64, replacementtext : *const UChar, replacementlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { utext_replace(ut as _, nativestart, nativelimit, replacementtext, replacementlength, status as _) }
}
#[inline]
pub unsafe fn utext_setNativeIndex(ut: *mut UText, nativeindex: i64) {
    windows_core::link!("icuuc.dll" "C" fn utext_setNativeIndex(ut : *mut UText, nativeindex : i64));
    unsafe { utext_setNativeIndex(ut as _, nativeindex) }
}
#[inline]
pub unsafe fn utext_setup(ut: *mut UText, extraspace: i32, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuuc.dll" "C" fn utext_setup(ut : *mut UText, extraspace : i32, status : *mut UErrorCode) -> *mut UText);
    unsafe { utext_setup(ut as _, extraspace, status as _) }
}
#[inline]
pub unsafe fn utf8_appendCharSafeBody(s: *mut u8, i: i32, length: i32, c: UChar32, piserror: *mut UBool) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utf8_appendCharSafeBody(s : *mut u8, i : i32, length : i32, c : UChar32, piserror : *mut UBool) -> i32);
    unsafe { utf8_appendCharSafeBody(s as _, i, length, c, piserror as _) }
}
#[inline]
pub unsafe fn utf8_back1SafeBody(s: *const u8, start: i32, i: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utf8_back1SafeBody(s : *const u8, start : i32, i : i32) -> i32);
    unsafe { utf8_back1SafeBody(s, start, i) }
}
#[inline]
pub unsafe fn utf8_nextCharSafeBody(s: *const u8, pi: *mut i32, length: i32, c: UChar32, strict: UBool) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn utf8_nextCharSafeBody(s : *const u8, pi : *mut i32, length : i32, c : UChar32, strict : UBool) -> UChar32);
    unsafe { utf8_nextCharSafeBody(s, pi as _, length, c, strict) }
}
#[inline]
pub unsafe fn utf8_prevCharSafeBody(s: *const u8, start: i32, pi: *mut i32, c: UChar32, strict: UBool) -> UChar32 {
    windows_core::link!("icuuc.dll" "C" fn utf8_prevCharSafeBody(s : *const u8, start : i32, pi : *mut i32, c : UChar32, strict : UBool) -> UChar32);
    unsafe { utf8_prevCharSafeBody(s, start, pi as _, c, strict) }
}
#[inline]
pub unsafe fn utmscale_fromInt64(othertime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn utmscale_fromInt64(othertime : i64, timescale : UDateTimeScale, status : *mut UErrorCode) -> i64);
    unsafe { utmscale_fromInt64(othertime, timescale, status as _) }
}
#[inline]
pub unsafe fn utmscale_getTimeScaleValue(timescale: UDateTimeScale, value: UTimeScaleValue, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn utmscale_getTimeScaleValue(timescale : UDateTimeScale, value : UTimeScaleValue, status : *mut UErrorCode) -> i64);
    unsafe { utmscale_getTimeScaleValue(timescale, value, status as _) }
}
#[inline]
pub unsafe fn utmscale_toInt64(universaltime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn utmscale_toInt64(universaltime : i64, timescale : UDateTimeScale, status : *mut UErrorCode) -> i64);
    unsafe { utmscale_toInt64(universaltime, timescale, status as _) }
}
#[inline]
pub unsafe fn utrace_format(outbuf: *mut i8, capacity: i32, indent: i32, fmt: *const i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utrace_format(outbuf : *mut i8, capacity : i32, indent : i32, fmt : *const i8) -> i32);
    unsafe { utrace_format(outbuf as _, capacity, indent, fmt) }
}
#[inline]
pub unsafe fn utrace_functionName(fnnumber: i32) -> *const i8 {
    windows_core::link!("icuuc.dll" "C" fn utrace_functionName(fnnumber : i32) -> *const i8);
    unsafe { utrace_functionName(fnnumber) }
}
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn utrace_getFunctions(context: *mut *mut core::ffi::c_void, e: *mut UTraceEntry, x: *mut UTraceExit, d: *mut UTraceData) {
    windows_core::link!("icuuc.dll" "C" fn utrace_getFunctions(context : *mut *mut core::ffi::c_void, e : *mut UTraceEntry, x : *mut UTraceExit, d : *mut UTraceData));
    unsafe { utrace_getFunctions(context as _, e as _, x as _, d as _) }
}
#[inline]
pub unsafe fn utrace_getLevel() -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utrace_getLevel() -> i32);
    unsafe { utrace_getLevel() }
}
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn utrace_setFunctions(context: *const core::ffi::c_void, e: UTraceEntry, x: UTraceExit, d: UTraceData) {
    windows_core::link!("icuuc.dll" "C" fn utrace_setFunctions(context : *const core::ffi::c_void, e : UTraceEntry, x : UTraceExit, d : UTraceData));
    unsafe { utrace_setFunctions(context, e, x, d) }
}
#[inline]
pub unsafe fn utrace_setLevel(tracelevel: i32) {
    windows_core::link!("icuuc.dll" "C" fn utrace_setLevel(tracelevel : i32));
    unsafe { utrace_setLevel(tracelevel) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn utrace_vformat(outbuf: *mut i8, capacity: i32, indent: i32, fmt: *const i8, args: *mut i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utrace_vformat(outbuf : *mut i8, capacity : i32, indent : i32, fmt : *const i8, args : *mut i8) -> i32);
    unsafe { utrace_vformat(outbuf as _, capacity, indent, fmt, args as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn utrace_vformat(outbuf: *mut i8, capacity: i32, indent: i32, fmt: *const i8, args: super::vadefs::va_list) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utrace_vformat(outbuf : *mut i8, capacity : i32, indent : i32, fmt : *const i8, args : super::vadefs::va_list) -> i32);
    unsafe { utrace_vformat(outbuf as _, capacity, indent, fmt, args) }
}
#[inline]
pub unsafe fn utrans_clone(trans: *const UTransliterator, status: *mut UErrorCode) -> *mut UTransliterator {
    windows_core::link!("icuin.dll" "C" fn utrans_clone(trans : *const UTransliterator, status : *mut UErrorCode) -> *mut UTransliterator);
    unsafe { utrans_clone(trans, status as _) }
}
#[inline]
pub unsafe fn utrans_close() -> UTransliterator {
    windows_core::link!("icuin.dll" "C" fn utrans_close(trans : *mut UTransliterator));
    unsafe {
        let mut result__ = core::mem::zeroed();
        utrans_close(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn utrans_countAvailableIDs() -> i32 {
    windows_core::link!("icuin.dll" "C" fn utrans_countAvailableIDs() -> i32);
    unsafe { utrans_countAvailableIDs() }
}
#[inline]
pub unsafe fn utrans_getSourceSet(trans: *const UTransliterator, ignorefilter: UBool, fillin: *mut USet, status: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuin.dll" "C" fn utrans_getSourceSet(trans : *const UTransliterator, ignorefilter : UBool, fillin : *mut USet, status : *mut UErrorCode) -> *mut USet);
    unsafe { utrans_getSourceSet(trans, ignorefilter, fillin as _, status as _) }
}
#[inline]
pub unsafe fn utrans_getUnicodeID(trans: *const UTransliterator, resultlength: *mut i32) -> *const UChar {
    windows_core::link!("icuin.dll" "C" fn utrans_getUnicodeID(trans : *const UTransliterator, resultlength : *mut i32) -> *const UChar);
    unsafe { utrans_getUnicodeID(trans, resultlength as _) }
}
#[inline]
pub unsafe fn utrans_openIDs(perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn utrans_openIDs(perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { utrans_openIDs(perrorcode as _) }
}
#[inline]
pub unsafe fn utrans_openInverse(trans: *const UTransliterator, status: *mut UErrorCode) -> *mut UTransliterator {
    windows_core::link!("icuin.dll" "C" fn utrans_openInverse(trans : *const UTransliterator, status : *mut UErrorCode) -> *mut UTransliterator);
    unsafe { utrans_openInverse(trans, status as _) }
}
#[inline]
pub unsafe fn utrans_openU(id: *const UChar, idlength: i32, dir: UTransDirection, rules: *const UChar, ruleslength: i32, parseerror: *mut UParseError, perrorcode: *mut UErrorCode) -> *mut UTransliterator {
    windows_core::link!("icuin.dll" "C" fn utrans_openU(id : *const UChar, idlength : i32, dir : UTransDirection, rules : *const UChar, ruleslength : i32, parseerror : *mut UParseError, perrorcode : *mut UErrorCode) -> *mut UTransliterator);
    unsafe { utrans_openU(id, idlength, dir, rules, ruleslength, parseerror as _, perrorcode as _) }
}
#[inline]
pub unsafe fn utrans_register(adoptedtrans: *mut UTransliterator, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_register(adoptedtrans : *mut UTransliterator, status : *mut UErrorCode));
    unsafe { utrans_register(adoptedtrans as _, status as _) }
}
#[inline]
pub unsafe fn utrans_setFilter(trans: *mut UTransliterator, filterpattern: *const UChar, filterpatternlen: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_setFilter(trans : *mut UTransliterator, filterpattern : *const UChar, filterpatternlen : i32, status : *mut UErrorCode));
    unsafe { utrans_setFilter(trans as _, filterpattern, filterpatternlen, status as _) }
}
#[inline]
pub unsafe fn utrans_toRules(trans: *const UTransliterator, escapeunprintable: UBool, result: *mut UChar, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn utrans_toRules(trans : *const UTransliterator, escapeunprintable : UBool, result : *mut UChar, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { utrans_toRules(trans, escapeunprintable, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn utrans_trans(trans: *const UTransliterator, rep: *mut UReplaceable, repfunc: *const UReplaceableCallbacks, start: i32, limit: *mut i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_trans(trans : *const UTransliterator, rep : *mut UReplaceable, repfunc : *const UReplaceableCallbacks, start : i32, limit : *mut i32, status : *mut UErrorCode));
    unsafe { utrans_trans(trans, rep as _, repfunc, start, limit as _, status as _) }
}
#[inline]
pub unsafe fn utrans_transIncremental(trans: *const UTransliterator, rep: *mut UReplaceable, repfunc: *const UReplaceableCallbacks, pos: *mut UTransPosition, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_transIncremental(trans : *const UTransliterator, rep : *mut UReplaceable, repfunc : *const UReplaceableCallbacks, pos : *mut UTransPosition, status : *mut UErrorCode));
    unsafe { utrans_transIncremental(trans, rep as _, repfunc, pos as _, status as _) }
}
#[inline]
pub unsafe fn utrans_transIncrementalUChars(trans: *const UTransliterator, text: *mut UChar, textlength: *mut i32, textcapacity: i32, pos: *mut UTransPosition, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_transIncrementalUChars(trans : *const UTransliterator, text : *mut UChar, textlength : *mut i32, textcapacity : i32, pos : *mut UTransPosition, status : *mut UErrorCode));
    unsafe { utrans_transIncrementalUChars(trans, text as _, textlength as _, textcapacity, pos as _, status as _) }
}
#[inline]
pub unsafe fn utrans_transUChars(trans: *const UTransliterator, text: *mut UChar, textlength: *mut i32, textcapacity: i32, start: i32, limit: *mut i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_transUChars(trans : *const UTransliterator, text : *mut UChar, textlength : *mut i32, textcapacity : i32, start : i32, limit : *mut i32, status : *mut UErrorCode));
    unsafe { utrans_transUChars(trans, text as _, textlength as _, textcapacity, start, limit as _, status as _) }
}
#[inline]
pub unsafe fn utrans_unregisterID(id: *const UChar, idlength: i32) {
    windows_core::link!("icuin.dll" "C" fn utrans_unregisterID(id : *const UChar, idlength : i32));
    unsafe { utrans_unregisterID(id, idlength) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OldUChar(pub u16);
pub const U16_MAX_LENGTH: u32 = 2;
pub const U16_SURROGATE_OFFSET: u32 = 56613888;
pub const U8_LEAD3_T1_BITS: windows_core::PCSTR = windows_core::s!(" 000000000000\u{10}00");
pub const U8_LEAD4_T1_BITS: windows_core::PCSTR = windows_core::s!("\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{1e}\u{f}\u{f}\u{f}\u{0}\u{0}\u{0}\u{0}");
pub const U8_MAX_LENGTH: u32 = 4;
pub type UAcceptResult = i32;
pub const UBIDI_DEFAULT_LTR: u32 = 254;
pub const UBIDI_DEFAULT_RTL: u32 = 255;
pub const UBIDI_DO_MIRRORING: u32 = 2;
pub const UBIDI_INSERT_LRM_FOR_NUMERIC: u32 = 4;
pub const UBIDI_KEEP_BASE_COMBINING: u32 = 1;
pub const UBIDI_LEVEL_OVERRIDE: u32 = 128;
pub const UBIDI_LOGICAL: UBiDiOrder = 0;
pub const UBIDI_LTR: UBiDiDirection = 0;
pub const UBIDI_MAP_NOWHERE: i32 = -1;
pub const UBIDI_MAX_EXPLICIT_LEVEL: u32 = 125;
pub const UBIDI_MIRRORING_OFF: UBiDiMirroring = 0;
pub const UBIDI_MIRRORING_ON: UBiDiMirroring = 1;
pub const UBIDI_MIXED: UBiDiDirection = 2;
pub const UBIDI_NEUTRAL: UBiDiDirection = 3;
pub const UBIDI_OPTION_DEFAULT: UBiDiReorderingOption = 0;
pub const UBIDI_OPTION_INSERT_MARKS: UBiDiReorderingOption = 1;
pub const UBIDI_OPTION_REMOVE_CONTROLS: UBiDiReorderingOption = 2;
pub const UBIDI_OPTION_STREAMING: UBiDiReorderingOption = 4;
pub const UBIDI_OUTPUT_REVERSE: u32 = 16;
pub const UBIDI_REMOVE_BIDI_CONTROLS: u32 = 8;
pub const UBIDI_REORDER_DEFAULT: UBiDiReorderingMode = 0;
pub const UBIDI_REORDER_GROUP_NUMBERS_WITH_R: UBiDiReorderingMode = 2;
pub const UBIDI_REORDER_INVERSE_FOR_NUMBERS_SPECIAL: UBiDiReorderingMode = 6;
pub const UBIDI_REORDER_INVERSE_LIKE_DIRECT: UBiDiReorderingMode = 5;
pub const UBIDI_REORDER_INVERSE_NUMBERS_AS_L: UBiDiReorderingMode = 4;
pub const UBIDI_REORDER_NUMBERS_SPECIAL: UBiDiReorderingMode = 1;
pub const UBIDI_REORDER_RUNS_ONLY: UBiDiReorderingMode = 3;
pub const UBIDI_RTL: UBiDiDirection = 1;
pub const UBIDI_VISUAL: UBiDiOrder = 1;
pub const UBLOCK_ADLAM: UBlockCode = 263;
pub const UBLOCK_AEGEAN_NUMBERS: UBlockCode = 119;
pub const UBLOCK_AHOM: UBlockCode = 253;
pub const UBLOCK_ALCHEMICAL_SYMBOLS: UBlockCode = 208;
pub const UBLOCK_ALPHABETIC_PRESENTATION_FORMS: UBlockCode = 80;
pub const UBLOCK_ANATOLIAN_HIEROGLYPHS: UBlockCode = 254;
pub const UBLOCK_ANCIENT_GREEK_MUSICAL_NOTATION: UBlockCode = 126;
pub const UBLOCK_ANCIENT_GREEK_NUMBERS: UBlockCode = 127;
pub const UBLOCK_ANCIENT_SYMBOLS: UBlockCode = 165;
pub const UBLOCK_ARABIC: UBlockCode = 12;
pub const UBLOCK_ARABIC_EXTENDED_A: UBlockCode = 210;
pub const UBLOCK_ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS: UBlockCode = 211;
pub const UBLOCK_ARABIC_PRESENTATION_FORMS_A: UBlockCode = 81;
pub const UBLOCK_ARABIC_PRESENTATION_FORMS_B: UBlockCode = 85;
pub const UBLOCK_ARABIC_SUPPLEMENT: UBlockCode = 128;
pub const UBLOCK_ARMENIAN: UBlockCode = 10;
pub const UBLOCK_ARROWS: UBlockCode = 46;
pub const UBLOCK_AVESTAN: UBlockCode = 188;
pub const UBLOCK_BALINESE: UBlockCode = 147;
pub const UBLOCK_BAMUM: UBlockCode = 177;
pub const UBLOCK_BAMUM_SUPPLEMENT: UBlockCode = 202;
pub const UBLOCK_BASIC_LATIN: UBlockCode = 1;
pub const UBLOCK_BASSA_VAH: UBlockCode = 221;
pub const UBLOCK_BATAK: UBlockCode = 199;
pub const UBLOCK_BENGALI: UBlockCode = 16;
pub const UBLOCK_BHAIKSUKI: UBlockCode = 264;
pub const UBLOCK_BLOCK_ELEMENTS: UBlockCode = 53;
pub const UBLOCK_BOPOMOFO: UBlockCode = 64;
pub const UBLOCK_BOPOMOFO_EXTENDED: UBlockCode = 67;
pub const UBLOCK_BOX_DRAWING: UBlockCode = 52;
pub const UBLOCK_BRAHMI: UBlockCode = 201;
pub const UBLOCK_BRAILLE_PATTERNS: UBlockCode = 57;
pub const UBLOCK_BUGINESE: UBlockCode = 129;
pub const UBLOCK_BUHID: UBlockCode = 100;
pub const UBLOCK_BYZANTINE_MUSICAL_SYMBOLS: UBlockCode = 91;
pub const UBLOCK_CARIAN: UBlockCode = 168;
pub const UBLOCK_CAUCASIAN_ALBANIAN: UBlockCode = 222;
pub const UBLOCK_CHAKMA: UBlockCode = 212;
pub const UBLOCK_CHAM: UBlockCode = 164;
pub const UBLOCK_CHEROKEE: UBlockCode = 32;
pub const UBLOCK_CHEROKEE_SUPPLEMENT: UBlockCode = 255;
pub const UBLOCK_CHESS_SYMBOLS: UBlockCode = 281;
pub const UBLOCK_CHORASMIAN: UBlockCode = 301;
pub const UBLOCK_CJK_COMPATIBILITY: UBlockCode = 69;
pub const UBLOCK_CJK_COMPATIBILITY_FORMS: UBlockCode = 83;
pub const UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS: UBlockCode = 79;
pub const UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT: UBlockCode = 95;
pub const UBLOCK_CJK_RADICALS_SUPPLEMENT: UBlockCode = 58;
pub const UBLOCK_CJK_STROKES: UBlockCode = 130;
pub const UBLOCK_CJK_SYMBOLS_AND_PUNCTUATION: UBlockCode = 61;
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS: UBlockCode = 71;
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A: UBlockCode = 70;
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B: UBlockCode = 94;
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C: UBlockCode = 197;
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D: UBlockCode = 209;
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E: UBlockCode = 256;
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F: UBlockCode = 274;
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G: UBlockCode = 302;
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS: UBlockCode = 7;
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS_EXTENDED: UBlockCode = 224;
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS_SUPPLEMENT: UBlockCode = 131;
pub const UBLOCK_COMBINING_HALF_MARKS: UBlockCode = 82;
pub const UBLOCK_COMBINING_MARKS_FOR_SYMBOLS: UBlockCode = 43;
pub const UBLOCK_COMMON_INDIC_NUMBER_FORMS: UBlockCode = 178;
pub const UBLOCK_CONTROL_PICTURES: UBlockCode = 49;
pub const UBLOCK_COPTIC: UBlockCode = 132;
pub const UBLOCK_COPTIC_EPACT_NUMBERS: UBlockCode = 223;
pub const UBLOCK_COUNTING_ROD_NUMERALS: UBlockCode = 154;
pub const UBLOCK_CUNEIFORM: UBlockCode = 152;
pub const UBLOCK_CUNEIFORM_NUMBERS_AND_PUNCTUATION: UBlockCode = 153;
pub const UBLOCK_CURRENCY_SYMBOLS: UBlockCode = 42;
pub const UBLOCK_CYPRIOT_SYLLABARY: UBlockCode = 123;
pub const UBLOCK_CYRILLIC: UBlockCode = 9;
pub const UBLOCK_CYRILLIC_EXTENDED_A: UBlockCode = 158;
pub const UBLOCK_CYRILLIC_EXTENDED_B: UBlockCode = 160;
pub const UBLOCK_CYRILLIC_EXTENDED_C: UBlockCode = 265;
pub const UBLOCK_CYRILLIC_SUPPLEMENT: UBlockCode = 97;
pub const UBLOCK_CYRILLIC_SUPPLEMENTARY: UBlockCode = 97;
pub const UBLOCK_DESERET: UBlockCode = 90;
pub const UBLOCK_DEVANAGARI: UBlockCode = 15;
pub const UBLOCK_DEVANAGARI_EXTENDED: UBlockCode = 179;
pub const UBLOCK_DINGBATS: UBlockCode = 56;
pub const UBLOCK_DIVES_AKURU: UBlockCode = 303;
pub const UBLOCK_DOGRA: UBlockCode = 282;
pub const UBLOCK_DOMINO_TILES: UBlockCode = 171;
pub const UBLOCK_DUPLOYAN: UBlockCode = 225;
pub const UBLOCK_EARLY_DYNASTIC_CUNEIFORM: UBlockCode = 257;
pub const UBLOCK_EGYPTIAN_HIEROGLYPHS: UBlockCode = 194;
pub const UBLOCK_EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS: UBlockCode = 292;
pub const UBLOCK_ELBASAN: UBlockCode = 226;
pub const UBLOCK_ELYMAIC: UBlockCode = 293;
pub const UBLOCK_EMOTICONS: UBlockCode = 206;
pub const UBLOCK_ENCLOSED_ALPHANUMERICS: UBlockCode = 51;
pub const UBLOCK_ENCLOSED_ALPHANUMERIC_SUPPLEMENT: UBlockCode = 195;
pub const UBLOCK_ENCLOSED_CJK_LETTERS_AND_MONTHS: UBlockCode = 68;
pub const UBLOCK_ENCLOSED_IDEOGRAPHIC_SUPPLEMENT: UBlockCode = 196;
pub const UBLOCK_ETHIOPIC: UBlockCode = 31;
pub const UBLOCK_ETHIOPIC_EXTENDED: UBlockCode = 133;
pub const UBLOCK_ETHIOPIC_EXTENDED_A: UBlockCode = 200;
pub const UBLOCK_ETHIOPIC_SUPPLEMENT: UBlockCode = 134;
pub const UBLOCK_GENERAL_PUNCTUATION: UBlockCode = 40;
pub const UBLOCK_GEOMETRIC_SHAPES: UBlockCode = 54;
pub const UBLOCK_GEOMETRIC_SHAPES_EXTENDED: UBlockCode = 227;
pub const UBLOCK_GEORGIAN: UBlockCode = 29;
pub const UBLOCK_GEORGIAN_EXTENDED: UBlockCode = 283;
pub const UBLOCK_GEORGIAN_SUPPLEMENT: UBlockCode = 135;
pub const UBLOCK_GLAGOLITIC: UBlockCode = 136;
pub const UBLOCK_GLAGOLITIC_SUPPLEMENT: UBlockCode = 266;
pub const UBLOCK_GOTHIC: UBlockCode = 89;
pub const UBLOCK_GRANTHA: UBlockCode = 228;
pub const UBLOCK_GREEK: UBlockCode = 8;
pub const UBLOCK_GREEK_EXTENDED: UBlockCode = 39;
pub const UBLOCK_GUJARATI: UBlockCode = 18;
pub const UBLOCK_GUNJALA_GONDI: UBlockCode = 284;
pub const UBLOCK_GURMUKHI: UBlockCode = 17;
pub const UBLOCK_HALFWIDTH_AND_FULLWIDTH_FORMS: UBlockCode = 87;
pub const UBLOCK_HANGUL_COMPATIBILITY_JAMO: UBlockCode = 65;
pub const UBLOCK_HANGUL_JAMO: UBlockCode = 30;
pub const UBLOCK_HANGUL_JAMO_EXTENDED_A: UBlockCode = 180;
pub const UBLOCK_HANGUL_JAMO_EXTENDED_B: UBlockCode = 185;
pub const UBLOCK_HANGUL_SYLLABLES: UBlockCode = 74;
pub const UBLOCK_HANIFI_ROHINGYA: UBlockCode = 285;
pub const UBLOCK_HANUNOO: UBlockCode = 99;
pub const UBLOCK_HATRAN: UBlockCode = 258;
pub const UBLOCK_HEBREW: UBlockCode = 11;
pub const UBLOCK_HIGH_PRIVATE_USE_SURROGATES: UBlockCode = 76;
pub const UBLOCK_HIGH_SURROGATES: UBlockCode = 75;
pub const UBLOCK_HIRAGANA: UBlockCode = 62;
pub const UBLOCK_IDEOGRAPHIC_DESCRIPTION_CHARACTERS: UBlockCode = 60;
pub const UBLOCK_IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION: UBlockCode = 267;
pub const UBLOCK_IMPERIAL_ARAMAIC: UBlockCode = 186;
pub const UBLOCK_INDIC_SIYAQ_NUMBERS: UBlockCode = 286;
pub const UBLOCK_INSCRIPTIONAL_PAHLAVI: UBlockCode = 190;
pub const UBLOCK_INSCRIPTIONAL_PARTHIAN: UBlockCode = 189;
pub const UBLOCK_INVALID_CODE: UBlockCode = -1;
pub const UBLOCK_IPA_EXTENSIONS: UBlockCode = 5;
pub const UBLOCK_JAVANESE: UBlockCode = 181;
pub const UBLOCK_KAITHI: UBlockCode = 193;
pub const UBLOCK_KANA_EXTENDED_A: UBlockCode = 275;
pub const UBLOCK_KANA_SUPPLEMENT: UBlockCode = 203;
pub const UBLOCK_KANBUN: UBlockCode = 66;
pub const UBLOCK_KANGXI_RADICALS: UBlockCode = 59;
pub const UBLOCK_KANNADA: UBlockCode = 22;
pub const UBLOCK_KATAKANA: UBlockCode = 63;
pub const UBLOCK_KATAKANA_PHONETIC_EXTENSIONS: UBlockCode = 107;
pub const UBLOCK_KAYAH_LI: UBlockCode = 162;
pub const UBLOCK_KHAROSHTHI: UBlockCode = 137;
pub const UBLOCK_KHITAN_SMALL_SCRIPT: UBlockCode = 304;
pub const UBLOCK_KHMER: UBlockCode = 36;
pub const UBLOCK_KHMER_SYMBOLS: UBlockCode = 113;
pub const UBLOCK_KHOJKI: UBlockCode = 229;
pub const UBLOCK_KHUDAWADI: UBlockCode = 230;
pub const UBLOCK_LAO: UBlockCode = 26;
pub const UBLOCK_LATIN_1_SUPPLEMENT: UBlockCode = 2;
pub const UBLOCK_LATIN_EXTENDED_A: UBlockCode = 3;
pub const UBLOCK_LATIN_EXTENDED_ADDITIONAL: UBlockCode = 38;
pub const UBLOCK_LATIN_EXTENDED_B: UBlockCode = 4;
pub const UBLOCK_LATIN_EXTENDED_C: UBlockCode = 148;
pub const UBLOCK_LATIN_EXTENDED_D: UBlockCode = 149;
pub const UBLOCK_LATIN_EXTENDED_E: UBlockCode = 231;
pub const UBLOCK_LEPCHA: UBlockCode = 156;
pub const UBLOCK_LETTERLIKE_SYMBOLS: UBlockCode = 44;
pub const UBLOCK_LIMBU: UBlockCode = 111;
pub const UBLOCK_LINEAR_A: UBlockCode = 232;
pub const UBLOCK_LINEAR_B_IDEOGRAMS: UBlockCode = 118;
pub const UBLOCK_LINEAR_B_SYLLABARY: UBlockCode = 117;
pub const UBLOCK_LISU: UBlockCode = 176;
pub const UBLOCK_LISU_SUPPLEMENT: UBlockCode = 305;
pub const UBLOCK_LOW_SURROGATES: UBlockCode = 77;
pub const UBLOCK_LYCIAN: UBlockCode = 167;
pub const UBLOCK_LYDIAN: UBlockCode = 169;
pub const UBLOCK_MAHAJANI: UBlockCode = 233;
pub const UBLOCK_MAHJONG_TILES: UBlockCode = 170;
pub const UBLOCK_MAKASAR: UBlockCode = 287;
pub const UBLOCK_MALAYALAM: UBlockCode = 23;
pub const UBLOCK_MANDAIC: UBlockCode = 198;
pub const UBLOCK_MANICHAEAN: UBlockCode = 234;
pub const UBLOCK_MARCHEN: UBlockCode = 268;
pub const UBLOCK_MASARAM_GONDI: UBlockCode = 276;
pub const UBLOCK_MATHEMATICAL_ALPHANUMERIC_SYMBOLS: UBlockCode = 93;
pub const UBLOCK_MATHEMATICAL_OPERATORS: UBlockCode = 47;
pub const UBLOCK_MAYAN_NUMERALS: UBlockCode = 288;
pub const UBLOCK_MEDEFAIDRIN: UBlockCode = 289;
pub const UBLOCK_MEETEI_MAYEK: UBlockCode = 184;
pub const UBLOCK_MEETEI_MAYEK_EXTENSIONS: UBlockCode = 213;
pub const UBLOCK_MENDE_KIKAKUI: UBlockCode = 235;
pub const UBLOCK_MEROITIC_CURSIVE: UBlockCode = 214;
pub const UBLOCK_MEROITIC_HIEROGLYPHS: UBlockCode = 215;
pub const UBLOCK_MIAO: UBlockCode = 216;
pub const UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A: UBlockCode = 102;
pub const UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B: UBlockCode = 105;
pub const UBLOCK_MISCELLANEOUS_SYMBOLS: UBlockCode = 55;
pub const UBLOCK_MISCELLANEOUS_SYMBOLS_AND_ARROWS: UBlockCode = 115;
pub const UBLOCK_MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS: UBlockCode = 205;
pub const UBLOCK_MISCELLANEOUS_TECHNICAL: UBlockCode = 48;
pub const UBLOCK_MODI: UBlockCode = 236;
pub const UBLOCK_MODIFIER_TONE_LETTERS: UBlockCode = 138;
pub const UBLOCK_MONGOLIAN: UBlockCode = 37;
pub const UBLOCK_MONGOLIAN_SUPPLEMENT: UBlockCode = 269;
pub const UBLOCK_MRO: UBlockCode = 237;
pub const UBLOCK_MULTANI: UBlockCode = 259;
pub const UBLOCK_MUSICAL_SYMBOLS: UBlockCode = 92;
pub const UBLOCK_MYANMAR: UBlockCode = 28;
pub const UBLOCK_MYANMAR_EXTENDED_A: UBlockCode = 182;
pub const UBLOCK_MYANMAR_EXTENDED_B: UBlockCode = 238;
pub const UBLOCK_NABATAEAN: UBlockCode = 239;
pub const UBLOCK_NANDINAGARI: UBlockCode = 294;
pub const UBLOCK_NEWA: UBlockCode = 270;
pub const UBLOCK_NEW_TAI_LUE: UBlockCode = 139;
pub const UBLOCK_NKO: UBlockCode = 146;
pub const UBLOCK_NO_BLOCK: UBlockCode = 0;
pub const UBLOCK_NUMBER_FORMS: UBlockCode = 45;
pub const UBLOCK_NUSHU: UBlockCode = 277;
pub const UBLOCK_NYIAKENG_PUACHUE_HMONG: UBlockCode = 295;
pub const UBLOCK_OGHAM: UBlockCode = 34;
pub const UBLOCK_OLD_HUNGARIAN: UBlockCode = 260;
pub const UBLOCK_OLD_ITALIC: UBlockCode = 88;
pub const UBLOCK_OLD_NORTH_ARABIAN: UBlockCode = 240;
pub const UBLOCK_OLD_PERMIC: UBlockCode = 241;
pub const UBLOCK_OLD_PERSIAN: UBlockCode = 140;
pub const UBLOCK_OLD_SOGDIAN: UBlockCode = 290;
pub const UBLOCK_OLD_SOUTH_ARABIAN: UBlockCode = 187;
pub const UBLOCK_OLD_TURKIC: UBlockCode = 191;
pub const UBLOCK_OL_CHIKI: UBlockCode = 157;
pub const UBLOCK_OPTICAL_CHARACTER_RECOGNITION: UBlockCode = 50;
pub const UBLOCK_ORIYA: UBlockCode = 19;
pub const UBLOCK_ORNAMENTAL_DINGBATS: UBlockCode = 242;
pub const UBLOCK_OSAGE: UBlockCode = 271;
pub const UBLOCK_OSMANYA: UBlockCode = 122;
pub const UBLOCK_OTTOMAN_SIYAQ_NUMBERS: UBlockCode = 296;
pub const UBLOCK_PAHAWH_HMONG: UBlockCode = 243;
pub const UBLOCK_PALMYRENE: UBlockCode = 244;
pub const UBLOCK_PAU_CIN_HAU: UBlockCode = 245;
pub const UBLOCK_PHAGS_PA: UBlockCode = 150;
pub const UBLOCK_PHAISTOS_DISC: UBlockCode = 166;
pub const UBLOCK_PHOENICIAN: UBlockCode = 151;
pub const UBLOCK_PHONETIC_EXTENSIONS: UBlockCode = 114;
pub const UBLOCK_PHONETIC_EXTENSIONS_SUPPLEMENT: UBlockCode = 141;
pub const UBLOCK_PLAYING_CARDS: UBlockCode = 204;
pub const UBLOCK_PRIVATE_USE: UBlockCode = 78;
pub const UBLOCK_PRIVATE_USE_AREA: UBlockCode = 78;
pub const UBLOCK_PSALTER_PAHLAVI: UBlockCode = 246;
pub const UBLOCK_REJANG: UBlockCode = 163;
pub const UBLOCK_RUMI_NUMERAL_SYMBOLS: UBlockCode = 192;
pub const UBLOCK_RUNIC: UBlockCode = 35;
pub const UBLOCK_SAMARITAN: UBlockCode = 172;
pub const UBLOCK_SAURASHTRA: UBlockCode = 161;
pub const UBLOCK_SHARADA: UBlockCode = 217;
pub const UBLOCK_SHAVIAN: UBlockCode = 121;
pub const UBLOCK_SHORTHAND_FORMAT_CONTROLS: UBlockCode = 247;
pub const UBLOCK_SIDDHAM: UBlockCode = 248;
pub const UBLOCK_SINHALA: UBlockCode = 24;
pub const UBLOCK_SINHALA_ARCHAIC_NUMBERS: UBlockCode = 249;
pub const UBLOCK_SMALL_FORM_VARIANTS: UBlockCode = 84;
pub const UBLOCK_SMALL_KANA_EXTENSION: UBlockCode = 297;
pub const UBLOCK_SOGDIAN: UBlockCode = 291;
pub const UBLOCK_SORA_SOMPENG: UBlockCode = 218;
pub const UBLOCK_SOYOMBO: UBlockCode = 278;
pub const UBLOCK_SPACING_MODIFIER_LETTERS: UBlockCode = 6;
pub const UBLOCK_SPECIALS: UBlockCode = 86;
pub const UBLOCK_SUNDANESE: UBlockCode = 155;
pub const UBLOCK_SUNDANESE_SUPPLEMENT: UBlockCode = 219;
pub const UBLOCK_SUPERSCRIPTS_AND_SUBSCRIPTS: UBlockCode = 41;
pub const UBLOCK_SUPPLEMENTAL_ARROWS_A: UBlockCode = 103;
pub const UBLOCK_SUPPLEMENTAL_ARROWS_B: UBlockCode = 104;
pub const UBLOCK_SUPPLEMENTAL_ARROWS_C: UBlockCode = 250;
pub const UBLOCK_SUPPLEMENTAL_MATHEMATICAL_OPERATORS: UBlockCode = 106;
pub const UBLOCK_SUPPLEMENTAL_PUNCTUATION: UBlockCode = 142;
pub const UBLOCK_SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS: UBlockCode = 261;
pub const UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_A: UBlockCode = 109;
pub const UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_B: UBlockCode = 110;
pub const UBLOCK_SUTTON_SIGNWRITING: UBlockCode = 262;
pub const UBLOCK_SYLOTI_NAGRI: UBlockCode = 143;
pub const UBLOCK_SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A: UBlockCode = 298;
pub const UBLOCK_SYMBOLS_FOR_LEGACY_COMPUTING: UBlockCode = 306;
pub const UBLOCK_SYRIAC: UBlockCode = 13;
pub const UBLOCK_SYRIAC_SUPPLEMENT: UBlockCode = 279;
pub const UBLOCK_TAGALOG: UBlockCode = 98;
pub const UBLOCK_TAGBANWA: UBlockCode = 101;
pub const UBLOCK_TAGS: UBlockCode = 96;
pub const UBLOCK_TAI_LE: UBlockCode = 112;
pub const UBLOCK_TAI_THAM: UBlockCode = 174;
pub const UBLOCK_TAI_VIET: UBlockCode = 183;
pub const UBLOCK_TAI_XUAN_JING_SYMBOLS: UBlockCode = 124;
pub const UBLOCK_TAKRI: UBlockCode = 220;
pub const UBLOCK_TAMIL: UBlockCode = 20;
pub const UBLOCK_TAMIL_SUPPLEMENT: UBlockCode = 299;
pub const UBLOCK_TANGUT: UBlockCode = 272;
pub const UBLOCK_TANGUT_COMPONENTS: UBlockCode = 273;
pub const UBLOCK_TANGUT_SUPPLEMENT: UBlockCode = 307;
pub const UBLOCK_TELUGU: UBlockCode = 21;
pub const UBLOCK_THAANA: UBlockCode = 14;
pub const UBLOCK_THAI: UBlockCode = 25;
pub const UBLOCK_TIBETAN: UBlockCode = 27;
pub const UBLOCK_TIFINAGH: UBlockCode = 144;
pub const UBLOCK_TIRHUTA: UBlockCode = 251;
pub const UBLOCK_TRANSPORT_AND_MAP_SYMBOLS: UBlockCode = 207;
pub const UBLOCK_UGARITIC: UBlockCode = 120;
pub const UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS: UBlockCode = 33;
pub const UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED: UBlockCode = 173;
pub const UBLOCK_VAI: UBlockCode = 159;
pub const UBLOCK_VARIATION_SELECTORS: UBlockCode = 108;
pub const UBLOCK_VARIATION_SELECTORS_SUPPLEMENT: UBlockCode = 125;
pub const UBLOCK_VEDIC_EXTENSIONS: UBlockCode = 175;
pub const UBLOCK_VERTICAL_FORMS: UBlockCode = 145;
pub const UBLOCK_WANCHO: UBlockCode = 300;
pub const UBLOCK_WARANG_CITI: UBlockCode = 252;
pub const UBLOCK_YEZIDI: UBlockCode = 308;
pub const UBLOCK_YIJING_HEXAGRAM_SYMBOLS: UBlockCode = 116;
pub const UBLOCK_YI_RADICALS: UBlockCode = 73;
pub const UBLOCK_YI_SYLLABLES: UBlockCode = 72;
pub const UBLOCK_ZANABAZAR_SQUARE: UBlockCode = 280;
pub const UBRK_CHARACTER: UBreakIteratorType = 0;
pub const UBRK_DONE: i32 = -1;
pub const UBRK_LINE: UBreakIteratorType = 2;
pub const UBRK_LINE_HARD: ULineBreakTag = 100;
pub const UBRK_LINE_HARD_LIMIT: ULineBreakTag = 200;
pub const UBRK_LINE_SOFT: ULineBreakTag = 0;
pub const UBRK_LINE_SOFT_LIMIT: ULineBreakTag = 100;
pub const UBRK_SENTENCE: UBreakIteratorType = 3;
pub const UBRK_SENTENCE_SEP: USentenceBreakTag = 100;
pub const UBRK_SENTENCE_SEP_LIMIT: USentenceBreakTag = 200;
pub const UBRK_SENTENCE_TERM: USentenceBreakTag = 0;
pub const UBRK_SENTENCE_TERM_LIMIT: USentenceBreakTag = 100;
pub const UBRK_WORD: UBreakIteratorType = 1;
pub const UBRK_WORD_IDEO: UWordBreak = 400;
pub const UBRK_WORD_IDEO_LIMIT: UWordBreak = 500;
pub const UBRK_WORD_KANA: UWordBreak = 300;
pub const UBRK_WORD_KANA_LIMIT: UWordBreak = 400;
pub const UBRK_WORD_LETTER: UWordBreak = 200;
pub const UBRK_WORD_LETTER_LIMIT: UWordBreak = 300;
pub const UBRK_WORD_NONE: UWordBreak = 0;
pub const UBRK_WORD_NONE_LIMIT: UWordBreak = 100;
pub const UBRK_WORD_NUMBER: UWordBreak = 100;
pub const UBRK_WORD_NUMBER_LIMIT: UWordBreak = 200;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UBiDi(pub u8);
pub type UBiDiClassCallback = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, c: UChar32) -> UCharDirection>;
pub type UBiDiDirection = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UBiDiLevel(pub u8);
pub type UBiDiMirroring = i32;
pub type UBiDiOrder = i32;
pub type UBiDiReorderingMode = i32;
pub type UBiDiReorderingOption = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UBiDiTransform(pub u8);
pub type UBidiPairedBracketType = i32;
pub type UBlockCode = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UBool(pub i8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UBreakIterator(pub u8);
pub type UBreakIteratorType = i32;
pub const UCAL_ACTUAL_MAXIMUM: UCalendarLimitType = 5;
pub const UCAL_ACTUAL_MINIMUM: UCalendarLimitType = 4;
pub const UCAL_AM: UCalendarAMPMs = 0;
pub const UCAL_AM_PM: UCalendarDateFields = 9;
pub const UCAL_APRIL: UCalendarMonths = 3;
pub const UCAL_AUGUST: UCalendarMonths = 7;
pub const UCAL_DATE: UCalendarDateFields = 5;
pub const UCAL_DAY_OF_MONTH: UCalendarDateFields = 5;
pub const UCAL_DAY_OF_WEEK: UCalendarDateFields = 7;
pub const UCAL_DAY_OF_WEEK_IN_MONTH: UCalendarDateFields = 8;
pub const UCAL_DAY_OF_YEAR: UCalendarDateFields = 6;
pub const UCAL_DECEMBER: UCalendarMonths = 11;
pub const UCAL_DEFAULT: UCalendarType = 0;
pub const UCAL_DOW_LOCAL: UCalendarDateFields = 18;
pub const UCAL_DST: UCalendarDisplayNameType = 2;
pub const UCAL_DST_OFFSET: UCalendarDateFields = 16;
pub const UCAL_ERA: UCalendarDateFields = 0;
pub const UCAL_EXTENDED_YEAR: UCalendarDateFields = 19;
pub const UCAL_FEBRUARY: UCalendarMonths = 1;
pub const UCAL_FIELD_COUNT: UCalendarDateFields = 23;
pub const UCAL_FIRST_DAY_OF_WEEK: UCalendarAttribute = 1;
pub const UCAL_FRIDAY: UCalendarDaysOfWeek = 6;
pub const UCAL_GREATEST_MINIMUM: UCalendarLimitType = 2;
pub const UCAL_GREGORIAN: UCalendarType = 1;
pub const UCAL_HOUR: UCalendarDateFields = 10;
pub const UCAL_HOUR_OF_DAY: UCalendarDateFields = 11;
pub const UCAL_IS_LEAP_MONTH: UCalendarDateFields = 22;
pub const UCAL_JANUARY: UCalendarMonths = 0;
pub const UCAL_JULIAN_DAY: UCalendarDateFields = 20;
pub const UCAL_JULY: UCalendarMonths = 6;
pub const UCAL_JUNE: UCalendarMonths = 5;
pub const UCAL_LEAST_MAXIMUM: UCalendarLimitType = 3;
pub const UCAL_LENIENT: UCalendarAttribute = 0;
pub const UCAL_MARCH: UCalendarMonths = 2;
pub const UCAL_MAXIMUM: UCalendarLimitType = 1;
pub const UCAL_MAY: UCalendarMonths = 4;
pub const UCAL_MILLISECOND: UCalendarDateFields = 14;
pub const UCAL_MILLISECONDS_IN_DAY: UCalendarDateFields = 21;
pub const UCAL_MINIMAL_DAYS_IN_FIRST_WEEK: UCalendarAttribute = 2;
pub const UCAL_MINIMUM: UCalendarLimitType = 0;
pub const UCAL_MINUTE: UCalendarDateFields = 12;
pub const UCAL_MONDAY: UCalendarDaysOfWeek = 2;
pub const UCAL_MONTH: UCalendarDateFields = 2;
pub const UCAL_NOVEMBER: UCalendarMonths = 10;
pub const UCAL_OCTOBER: UCalendarMonths = 9;
pub const UCAL_PM: UCalendarAMPMs = 1;
pub const UCAL_REPEATED_WALL_TIME: UCalendarAttribute = 3;
pub const UCAL_SATURDAY: UCalendarDaysOfWeek = 7;
pub const UCAL_SECOND: UCalendarDateFields = 13;
pub const UCAL_SEPTEMBER: UCalendarMonths = 8;
pub const UCAL_SHORT_DST: UCalendarDisplayNameType = 3;
pub const UCAL_SHORT_STANDARD: UCalendarDisplayNameType = 1;
pub const UCAL_SKIPPED_WALL_TIME: UCalendarAttribute = 4;
pub const UCAL_STANDARD: UCalendarDisplayNameType = 0;
pub const UCAL_SUNDAY: UCalendarDaysOfWeek = 1;
pub const UCAL_THURSDAY: UCalendarDaysOfWeek = 5;
pub const UCAL_TRADITIONAL: UCalendarType = 0;
pub const UCAL_TUESDAY: UCalendarDaysOfWeek = 3;
pub const UCAL_TZ_LOCAL_DAYLIGHT_FORMER: UTimeZoneLocalOption = 7;
pub const UCAL_TZ_LOCAL_DAYLIGHT_LATTER: UTimeZoneLocalOption = 15;
pub const UCAL_TZ_LOCAL_FORMER: UTimeZoneLocalOption = 4;
pub const UCAL_TZ_LOCAL_LATTER: UTimeZoneLocalOption = 12;
pub const UCAL_TZ_LOCAL_STANDARD_FORMER: UTimeZoneLocalOption = 5;
pub const UCAL_TZ_LOCAL_STANDARD_LATTER: UTimeZoneLocalOption = 13;
pub const UCAL_TZ_TRANSITION_NEXT: UTimeZoneTransitionType = 0;
pub const UCAL_TZ_TRANSITION_NEXT_INCLUSIVE: UTimeZoneTransitionType = 1;
pub const UCAL_TZ_TRANSITION_PREVIOUS: UTimeZoneTransitionType = 2;
pub const UCAL_TZ_TRANSITION_PREVIOUS_INCLUSIVE: UTimeZoneTransitionType = 3;
pub const UCAL_UNDECIMBER: UCalendarMonths = 12;
pub const UCAL_UNKNOWN_ZONE_ID: windows_core::PCSTR = windows_core::s!("Etc/Unknown");
pub const UCAL_WALLTIME_FIRST: UCalendarWallTimeOption = 1;
pub const UCAL_WALLTIME_LAST: UCalendarWallTimeOption = 0;
pub const UCAL_WALLTIME_NEXT_VALID: UCalendarWallTimeOption = 2;
pub const UCAL_WEDNESDAY: UCalendarDaysOfWeek = 4;
pub const UCAL_WEEKDAY: UCalendarWeekdayType = 0;
pub const UCAL_WEEKEND: UCalendarWeekdayType = 1;
pub const UCAL_WEEKEND_CEASE: UCalendarWeekdayType = 3;
pub const UCAL_WEEKEND_ONSET: UCalendarWeekdayType = 2;
pub const UCAL_WEEK_OF_MONTH: UCalendarDateFields = 4;
pub const UCAL_WEEK_OF_YEAR: UCalendarDateFields = 3;
pub const UCAL_YEAR: UCalendarDateFields = 1;
pub const UCAL_YEAR_WOY: UCalendarDateFields = 17;
pub const UCAL_ZONE_OFFSET: UCalendarDateFields = 15;
pub const UCAL_ZONE_TYPE_ANY: USystemTimeZoneType = 0;
pub const UCAL_ZONE_TYPE_CANONICAL: USystemTimeZoneType = 1;
pub const UCAL_ZONE_TYPE_CANONICAL_LOCATION: USystemTimeZoneType = 2;
pub const UCHAR_AGE: UProperty = 16384;
pub const UCHAR_ALPHABETIC: UProperty = 0;
pub const UCHAR_ASCII_HEX_DIGIT: UProperty = 1;
pub const UCHAR_BIDI_CLASS: UProperty = 4096;
pub const UCHAR_BIDI_CONTROL: UProperty = 2;
pub const UCHAR_BIDI_MIRRORED: UProperty = 3;
pub const UCHAR_BIDI_MIRRORING_GLYPH: UProperty = 16385;
pub const UCHAR_BIDI_PAIRED_BRACKET: UProperty = 16397;
pub const UCHAR_BIDI_PAIRED_BRACKET_TYPE: UProperty = 4117;
pub const UCHAR_BINARY_START: UProperty = 0;
pub const UCHAR_BLOCK: UProperty = 4097;
pub const UCHAR_CANONICAL_COMBINING_CLASS: UProperty = 4098;
pub const UCHAR_CASED: UProperty = 49;
pub const UCHAR_CASE_FOLDING: UProperty = 16386;
pub const UCHAR_CASE_IGNORABLE: UProperty = 50;
pub const UCHAR_CASE_SENSITIVE: UProperty = 34;
pub const UCHAR_CHANGES_WHEN_CASEFOLDED: UProperty = 54;
pub const UCHAR_CHANGES_WHEN_CASEMAPPED: UProperty = 55;
pub const UCHAR_CHANGES_WHEN_LOWERCASED: UProperty = 51;
pub const UCHAR_CHANGES_WHEN_NFKC_CASEFOLDED: UProperty = 56;
pub const UCHAR_CHANGES_WHEN_TITLECASED: UProperty = 53;
pub const UCHAR_CHANGES_WHEN_UPPERCASED: UProperty = 52;
pub const UCHAR_DASH: UProperty = 4;
pub const UCHAR_DECOMPOSITION_TYPE: UProperty = 4099;
pub const UCHAR_DEFAULT_IGNORABLE_CODE_POINT: UProperty = 5;
pub const UCHAR_DEPRECATED: UProperty = 6;
pub const UCHAR_DIACRITIC: UProperty = 7;
pub const UCHAR_DOUBLE_START: UProperty = 12288;
pub const UCHAR_EAST_ASIAN_WIDTH: UProperty = 4100;
pub const UCHAR_EMOJI: UProperty = 57;
pub const UCHAR_EMOJI_COMPONENT: UProperty = 61;
pub const UCHAR_EMOJI_MODIFIER: UProperty = 59;
pub const UCHAR_EMOJI_MODIFIER_BASE: UProperty = 60;
pub const UCHAR_EMOJI_PRESENTATION: UProperty = 58;
pub const UCHAR_EXTENDED_PICTOGRAPHIC: UProperty = 64;
pub const UCHAR_EXTENDER: UProperty = 8;
pub const UCHAR_FULL_COMPOSITION_EXCLUSION: UProperty = 9;
pub const UCHAR_GENERAL_CATEGORY: UProperty = 4101;
pub const UCHAR_GENERAL_CATEGORY_MASK: UProperty = 8192;
pub const UCHAR_GRAPHEME_BASE: UProperty = 10;
pub const UCHAR_GRAPHEME_CLUSTER_BREAK: UProperty = 4114;
pub const UCHAR_GRAPHEME_EXTEND: UProperty = 11;
pub const UCHAR_GRAPHEME_LINK: UProperty = 12;
pub const UCHAR_HANGUL_SYLLABLE_TYPE: UProperty = 4107;
pub const UCHAR_HEX_DIGIT: UProperty = 13;
pub const UCHAR_HYPHEN: UProperty = 14;
pub const UCHAR_IDEOGRAPHIC: UProperty = 17;
pub const UCHAR_IDS_BINARY_OPERATOR: UProperty = 18;
pub const UCHAR_IDS_TRINARY_OPERATOR: UProperty = 19;
pub const UCHAR_ID_CONTINUE: UProperty = 15;
pub const UCHAR_ID_START: UProperty = 16;
pub const UCHAR_INDIC_POSITIONAL_CATEGORY: UProperty = 4118;
pub const UCHAR_INDIC_SYLLABIC_CATEGORY: UProperty = 4119;
pub const UCHAR_INT_START: UProperty = 4096;
pub const UCHAR_INVALID_CODE: UProperty = -1;
pub const UCHAR_JOINING_GROUP: UProperty = 4102;
pub const UCHAR_JOINING_TYPE: UProperty = 4103;
pub const UCHAR_JOIN_CONTROL: UProperty = 20;
pub const UCHAR_LEAD_CANONICAL_COMBINING_CLASS: UProperty = 4112;
pub const UCHAR_LINE_BREAK: UProperty = 4104;
pub const UCHAR_LOGICAL_ORDER_EXCEPTION: UProperty = 21;
pub const UCHAR_LOWERCASE: UProperty = 22;
pub const UCHAR_LOWERCASE_MAPPING: UProperty = 16388;
pub const UCHAR_MASK_START: UProperty = 8192;
pub const UCHAR_MATH: UProperty = 23;
pub const UCHAR_MAX_VALUE: u32 = 1114111;
pub const UCHAR_MIN_VALUE: u32 = 0;
pub const UCHAR_NAME: UProperty = 16389;
pub const UCHAR_NFC_INERT: UProperty = 39;
pub const UCHAR_NFC_QUICK_CHECK: UProperty = 4110;
pub const UCHAR_NFD_INERT: UProperty = 37;
pub const UCHAR_NFD_QUICK_CHECK: UProperty = 4108;
pub const UCHAR_NFKC_INERT: UProperty = 40;
pub const UCHAR_NFKC_QUICK_CHECK: UProperty = 4111;
pub const UCHAR_NFKD_INERT: UProperty = 38;
pub const UCHAR_NFKD_QUICK_CHECK: UProperty = 4109;
pub const UCHAR_NONCHARACTER_CODE_POINT: UProperty = 24;
pub const UCHAR_NUMERIC_TYPE: UProperty = 4105;
pub const UCHAR_NUMERIC_VALUE: UProperty = 12288;
pub const UCHAR_OTHER_PROPERTY_START: UProperty = 28672;
pub const UCHAR_PATTERN_SYNTAX: UProperty = 42;
pub const UCHAR_PATTERN_WHITE_SPACE: UProperty = 43;
pub const UCHAR_POSIX_ALNUM: UProperty = 44;
pub const UCHAR_POSIX_BLANK: UProperty = 45;
pub const UCHAR_POSIX_GRAPH: UProperty = 46;
pub const UCHAR_POSIX_PRINT: UProperty = 47;
pub const UCHAR_POSIX_XDIGIT: UProperty = 48;
pub const UCHAR_PREPENDED_CONCATENATION_MARK: UProperty = 63;
pub const UCHAR_QUOTATION_MARK: UProperty = 25;
pub const UCHAR_RADICAL: UProperty = 26;
pub const UCHAR_REGIONAL_INDICATOR: UProperty = 62;
pub const UCHAR_SCRIPT: UProperty = 4106;
pub const UCHAR_SCRIPT_EXTENSIONS: UProperty = 28672;
pub const UCHAR_SEGMENT_STARTER: UProperty = 41;
pub const UCHAR_SENTENCE_BREAK: UProperty = 4115;
pub const UCHAR_SIMPLE_CASE_FOLDING: UProperty = 16390;
pub const UCHAR_SIMPLE_LOWERCASE_MAPPING: UProperty = 16391;
pub const UCHAR_SIMPLE_TITLECASE_MAPPING: UProperty = 16392;
pub const UCHAR_SIMPLE_UPPERCASE_MAPPING: UProperty = 16393;
pub const UCHAR_SOFT_DOTTED: UProperty = 27;
pub const UCHAR_STRING_START: UProperty = 16384;
pub const UCHAR_S_TERM: UProperty = 35;
pub const UCHAR_TERMINAL_PUNCTUATION: UProperty = 28;
pub const UCHAR_TITLECASE_MAPPING: UProperty = 16394;
pub const UCHAR_TRAIL_CANONICAL_COMBINING_CLASS: UProperty = 4113;
pub const UCHAR_UNIFIED_IDEOGRAPH: UProperty = 29;
pub const UCHAR_UPPERCASE: UProperty = 30;
pub const UCHAR_UPPERCASE_MAPPING: UProperty = 16396;
pub const UCHAR_VARIATION_SELECTOR: UProperty = 36;
pub const UCHAR_VERTICAL_ORIENTATION: UProperty = 4120;
pub const UCHAR_WHITE_SPACE: UProperty = 31;
pub const UCHAR_WORD_BREAK: UProperty = 4116;
pub const UCHAR_XID_CONTINUE: UProperty = 32;
pub const UCHAR_XID_START: UProperty = 33;
pub const UCLN_NO_AUTO_CLEANUP: u32 = 1;
pub const UCNV_BOCU1: UConverterType = 28;
pub const UCNV_CESU8: UConverterType = 31;
pub const UCNV_CLONE: UConverterCallbackReason = 5;
pub const UCNV_CLOSE: UConverterCallbackReason = 4;
pub const UCNV_COMPOUND_TEXT: UConverterType = 33;
pub const UCNV_DBCS: UConverterType = 1;
pub const UCNV_EBCDIC_STATEFUL: UConverterType = 9;
pub const UCNV_ESCAPE_C: windows_core::PCSTR = windows_core::s!("C");
pub const UCNV_ESCAPE_CSS2: windows_core::PCSTR = windows_core::s!("S");
pub const UCNV_ESCAPE_ICU: u32 = 0;
pub const UCNV_ESCAPE_JAVA: windows_core::PCSTR = windows_core::s!("J");
pub const UCNV_ESCAPE_UNICODE: windows_core::PCSTR = windows_core::s!("U");
pub const UCNV_ESCAPE_XML_DEC: windows_core::PCSTR = windows_core::s!("D");
pub const UCNV_ESCAPE_XML_HEX: windows_core::PCSTR = windows_core::s!("X");
pub const UCNV_HZ: UConverterType = 23;
pub const UCNV_IBM: UConverterPlatform = 0;
pub const UCNV_ILLEGAL: UConverterCallbackReason = 1;
pub const UCNV_IMAP_MAILBOX: UConverterType = 32;
pub const UCNV_IRREGULAR: UConverterCallbackReason = 2;
pub const UCNV_ISCII: UConverterType = 25;
pub const UCNV_ISO_2022: UConverterType = 10;
pub const UCNV_LATIN_1: UConverterType = 3;
pub const UCNV_LMBCS_1: UConverterType = 11;
pub const UCNV_LMBCS_11: UConverterType = 18;
pub const UCNV_LMBCS_16: UConverterType = 19;
pub const UCNV_LMBCS_17: UConverterType = 20;
pub const UCNV_LMBCS_18: UConverterType = 21;
pub const UCNV_LMBCS_19: UConverterType = 22;
pub const UCNV_LMBCS_2: UConverterType = 12;
pub const UCNV_LMBCS_3: UConverterType = 13;
pub const UCNV_LMBCS_4: UConverterType = 14;
pub const UCNV_LMBCS_5: UConverterType = 15;
pub const UCNV_LMBCS_6: UConverterType = 16;
pub const UCNV_LMBCS_8: UConverterType = 17;
pub const UCNV_LMBCS_LAST: UConverterType = 22;
pub const UCNV_LOCALE_OPTION_STRING: windows_core::PCSTR = windows_core::s!(",locale=");
pub const UCNV_MAX_CONVERTER_NAME_LENGTH: u32 = 60;
pub const UCNV_MAX_FULL_FILE_NAME_LENGTH: u32 = 660;
pub const UCNV_MBCS: UConverterType = 2;
pub const UCNV_NUMBER_OF_SUPPORTED_CONVERTER_TYPES: UConverterType = 34;
pub const UCNV_OPTION_SEP_CHAR: u32 = 44;
pub const UCNV_OPTION_SEP_STRING: windows_core::PCSTR = windows_core::s!(",");
pub const UCNV_RESET: UConverterCallbackReason = 3;
pub const UCNV_ROUNDTRIP_AND_FALLBACK_SET: UConverterUnicodeSet = 1;
pub const UCNV_ROUNDTRIP_SET: UConverterUnicodeSet = 0;
pub const UCNV_SBCS: UConverterType = 0;
pub const UCNV_SCSU: UConverterType = 24;
pub const UCNV_SI: u32 = 15;
pub const UCNV_SKIP_STOP_ON_ILLEGAL: windows_core::PCSTR = windows_core::s!("i");
pub const UCNV_SO: u32 = 14;
pub const UCNV_SUB_STOP_ON_ILLEGAL: windows_core::PCSTR = windows_core::s!("i");
pub const UCNV_SWAP_LFNL_OPTION_STRING: windows_core::PCSTR = windows_core::s!(",swaplfnl");
pub const UCNV_UNASSIGNED: UConverterCallbackReason = 0;
pub const UCNV_UNKNOWN: UConverterPlatform = -1;
pub const UCNV_UNSUPPORTED_CONVERTER: UConverterType = -1;
pub const UCNV_US_ASCII: UConverterType = 26;
pub const UCNV_UTF16: UConverterType = 29;
pub const UCNV_UTF16_BigEndian: UConverterType = 5;
pub const UCNV_UTF16_LittleEndian: UConverterType = 6;
pub const UCNV_UTF32: UConverterType = 30;
pub const UCNV_UTF32_BigEndian: UConverterType = 7;
pub const UCNV_UTF32_LittleEndian: UConverterType = 8;
pub const UCNV_UTF7: UConverterType = 27;
pub const UCNV_UTF8: UConverterType = 4;
pub const UCNV_VALUE_SEP_CHAR: u32 = 61;
pub const UCNV_VALUE_SEP_STRING: windows_core::PCSTR = windows_core::s!("=");
pub const UCNV_VERSION_OPTION_STRING: windows_core::PCSTR = windows_core::s!(",version=");
pub const UCOL_ALTERNATE_HANDLING: UColAttribute = 1;
pub const UCOL_ATTRIBUTE_COUNT: UColAttribute = 8;
pub const UCOL_BOUND_LOWER: UColBoundMode = 0;
pub const UCOL_BOUND_UPPER: UColBoundMode = 1;
pub const UCOL_BOUND_UPPER_LONG: UColBoundMode = 2;
pub const UCOL_CASE_FIRST: UColAttribute = 2;
pub const UCOL_CASE_LEVEL: UColAttribute = 3;
pub const UCOL_CE_STRENGTH_LIMIT: UColAttributeValue = 3;
pub const UCOL_DECOMPOSITION_MODE: UColAttribute = 4;
pub const UCOL_DEFAULT: UColAttributeValue = -1;
pub const UCOL_DEFAULT_STRENGTH: UColAttributeValue = 2;
pub const UCOL_EQUAL: UCollationResult = 0;
pub const UCOL_FRENCH_COLLATION: UColAttribute = 0;
pub const UCOL_FULL_RULES: UColRuleOption = 1;
pub const UCOL_GREATER: UCollationResult = 1;
pub const UCOL_IDENTICAL: UColAttributeValue = 15;
pub const UCOL_LESS: UCollationResult = -1;
pub const UCOL_LOWER_FIRST: UColAttributeValue = 24;
pub const UCOL_NON_IGNORABLE: UColAttributeValue = 21;
pub const UCOL_NORMALIZATION_MODE: UColAttribute = 4;
pub const UCOL_NULLORDER: i32 = -1;
pub const UCOL_NUMERIC_COLLATION: UColAttribute = 7;
pub const UCOL_OFF: UColAttributeValue = 16;
pub const UCOL_ON: UColAttributeValue = 17;
pub const UCOL_PRIMARY: UColAttributeValue = 0;
pub const UCOL_QUATERNARY: UColAttributeValue = 3;
pub const UCOL_REORDER_CODE_CURRENCY: UColReorderCode = 4099;
pub const UCOL_REORDER_CODE_DEFAULT: UColReorderCode = -1;
pub const UCOL_REORDER_CODE_DIGIT: UColReorderCode = 4100;
pub const UCOL_REORDER_CODE_FIRST: UColReorderCode = 4096;
pub const UCOL_REORDER_CODE_NONE: UColReorderCode = 103;
pub const UCOL_REORDER_CODE_OTHERS: UColReorderCode = 103;
pub const UCOL_REORDER_CODE_PUNCTUATION: UColReorderCode = 4097;
pub const UCOL_REORDER_CODE_SPACE: UColReorderCode = 4096;
pub const UCOL_REORDER_CODE_SYMBOL: UColReorderCode = 4098;
pub const UCOL_SECONDARY: UColAttributeValue = 1;
pub const UCOL_SHIFTED: UColAttributeValue = 20;
pub const UCOL_STRENGTH: UColAttribute = 5;
pub const UCOL_STRENGTH_LIMIT: UColAttributeValue = 16;
pub const UCOL_TAILORING_ONLY: UColRuleOption = 0;
pub const UCOL_TERTIARY: UColAttributeValue = 2;
pub const UCOL_UPPER_FIRST: UColAttributeValue = 25;
pub const UCONFIG_ENABLE_PLUGINS: u32 = 0;
pub const UCONFIG_HAVE_PARSEALLINPUT: u32 = 1;
pub const UCONFIG_NO_BREAK_ITERATION: u32 = 0;
pub const UCONFIG_NO_COLLATION: u32 = 0;
pub const UCONFIG_NO_CONVERSION: u32 = 0;
pub const UCONFIG_NO_FILE_IO: u32 = 0;
pub const UCONFIG_NO_FILTERED_BREAK_ITERATION: u32 = 0;
pub const UCONFIG_NO_FORMATTING: u32 = 0;
pub const UCONFIG_NO_IDNA: u32 = 0;
pub const UCONFIG_NO_LEGACY_CONVERSION: u32 = 0;
pub const UCONFIG_NO_NORMALIZATION: u32 = 0;
pub const UCONFIG_NO_REGULAR_EXPRESSIONS: u32 = 0;
pub const UCONFIG_NO_SERVICE: u32 = 0;
pub const UCONFIG_NO_TRANSLITERATION: u32 = 0;
pub const UCONFIG_ONLY_COLLATION: u32 = 0;
pub const UCONFIG_ONLY_HTML_CONVERSION: u32 = 0;
pub const UCPMAP_RANGE_FIXED_ALL_SURROGATES: UCPMapRangeOption = 2;
pub const UCPMAP_RANGE_FIXED_LEAD_SURROGATES: UCPMapRangeOption = 1;
pub const UCPMAP_RANGE_NORMAL: UCPMapRangeOption = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UCPMap(pub u8);
pub type UCPMapRangeOption = i32;
pub type UCPMapValueFilter = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, value: u32) -> u32>;
pub const UCPTRIE_ERROR_VALUE_NEG_DATA_OFFSET: i32 = 1;
pub const UCPTRIE_FAST_DATA_BLOCK_LENGTH: i32 = 64;
pub const UCPTRIE_FAST_DATA_MASK: i32 = 63;
pub const UCPTRIE_FAST_SHIFT: i32 = 6;
pub const UCPTRIE_HIGH_VALUE_NEG_DATA_OFFSET: i32 = 2;
pub const UCPTRIE_SMALL_MAX: i32 = 4095;
pub const UCPTRIE_TYPE_ANY: UCPTrieType = -1;
pub const UCPTRIE_TYPE_FAST: UCPTrieType = 0;
pub const UCPTRIE_TYPE_SMALL: UCPTrieType = 1;
pub const UCPTRIE_VALUE_BITS_16: UCPTrieValueWidth = 0;
pub const UCPTRIE_VALUE_BITS_32: UCPTrieValueWidth = 1;
pub const UCPTRIE_VALUE_BITS_8: UCPTrieValueWidth = 2;
pub const UCPTRIE_VALUE_BITS_ANY: UCPTrieValueWidth = -1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UCPTrie {
    pub index: *const u16,
    pub data: UCPTrieData,
    pub indexLength: i32,
    pub dataLength: i32,
    pub highStart: UChar32,
    pub shifted12HighStart: u16,
    pub r#type: i8,
    pub valueWidth: i8,
    pub reserved32: u32,
    pub reserved16: u16,
    pub index3NullOffset: u16,
    pub dataNullOffset: i32,
    pub nullValue: u32,
}
impl Default for UCPTrie {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union UCPTrieData {
    pub ptr0: *const core::ffi::c_void,
    pub ptr16: *const u16,
    pub ptr32: *const u32,
    pub ptr8: *const u8,
}
impl Default for UCPTrieData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UCPTrieType = i32;
pub type UCPTrieValueWidth = i32;
pub const UCURR_ALL: UCurrCurrencyType = 2147483647;
pub const UCURR_COMMON: UCurrCurrencyType = 1;
pub const UCURR_DEPRECATED: UCurrCurrencyType = 4;
pub const UCURR_LONG_NAME: UCurrNameStyle = 1;
pub const UCURR_NARROW_SYMBOL_NAME: UCurrNameStyle = 2;
pub const UCURR_NON_DEPRECATED: UCurrCurrencyType = 8;
pub const UCURR_SYMBOL_NAME: UCurrNameStyle = 0;
pub const UCURR_UNCOMMON: UCurrCurrencyType = 2;
pub const UCURR_USAGE_CASH: UCurrencyUsage = 1;
pub const UCURR_USAGE_STANDARD: UCurrencyUsage = 0;
pub const UCURR_VARIANT_SYMBOL_NAME: UCurrNameStyle = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UCalendar(pub *mut core::ffi::c_void);
impl Default for UCalendar {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UCalendarAMPMs = i32;
pub type UCalendarAttribute = i32;
pub type UCalendarDateFields = i32;
pub type UCalendarDaysOfWeek = i32;
pub type UCalendarDisplayNameType = i32;
pub type UCalendarLimitType = i32;
pub type UCalendarMonths = i32;
pub type UCalendarType = i32;
pub type UCalendarWallTimeOption = i32;
pub type UCalendarWeekdayType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UCaseMap(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UChar(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UChar32(pub i32);
pub type UCharCategory = i32;
pub type UCharDirection = i32;
pub type UCharEnumTypeRange = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, start: UChar32, limit: UChar32, r#type: UCharCategory) -> UBool>;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UCharIterator {
    pub context: *const core::ffi::c_void,
    pub length: i32,
    pub start: i32,
    pub index: i32,
    pub limit: i32,
    pub reservedField: i32,
    pub getIndex: UCharIteratorGetIndex,
    pub r#move: UCharIteratorMove,
    pub hasNext: UCharIteratorHasNext,
    pub hasPrevious: UCharIteratorHasPrevious,
    pub current: UCharIteratorCurrent,
    pub next: UCharIteratorNext,
    pub previous: UCharIteratorPrevious,
    pub reservedFn: UCharIteratorReserved,
    pub getState: UCharIteratorGetState,
    pub setState: UCharIteratorSetState,
}
impl Default for UCharIterator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UCharIteratorCurrent = Option<unsafe extern "C" fn(iter: *mut UCharIterator) -> UChar32>;
pub type UCharIteratorGetIndex = Option<unsafe extern "C" fn(iter: *mut UCharIterator, origin: UCharIteratorOrigin) -> i32>;
pub type UCharIteratorGetState = Option<unsafe extern "C" fn(iter: *const UCharIterator) -> u32>;
pub type UCharIteratorHasNext = Option<unsafe extern "C" fn(iter: *mut UCharIterator) -> UBool>;
pub type UCharIteratorHasPrevious = Option<unsafe extern "C" fn(iter: *mut UCharIterator) -> UBool>;
pub type UCharIteratorMove = Option<unsafe extern "C" fn(iter: *mut UCharIterator, delta: i32, origin: UCharIteratorOrigin) -> i32>;
pub type UCharIteratorNext = Option<unsafe extern "C" fn(iter: *mut UCharIterator) -> UChar32>;
pub type UCharIteratorOrigin = i32;
pub type UCharIteratorPrevious = Option<unsafe extern "C" fn(iter: *mut UCharIterator) -> UChar32>;
pub type UCharIteratorReserved = Option<unsafe extern "C" fn(iter: *mut UCharIterator, something: i32) -> i32>;
pub type UCharIteratorSetState = Option<unsafe extern "C" fn(iter: *mut UCharIterator, state: u32, perrorcode: *mut UErrorCode)>;
pub type UCharNameChoice = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UCharsetDetector(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UCharsetMatch(pub u8);
pub type UColAttribute = i32;
pub type UColAttributeValue = i32;
pub type UColBoundMode = i32;
pub type UColReorderCode = i32;
pub type UColRuleOption = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UCollationElements(pub u8);
pub type UCollationResult = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UCollationStrength(pub UColAttributeValue);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UCollator(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UConstrainedFieldPosition(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UConverter(pub u8);
pub type UConverterCallbackReason = i32;
pub type UConverterFromUCallback = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, args: *mut UConverterFromUnicodeArgs, codeunits: *const UChar, length: i32, codepoint: UChar32, reason: UConverterCallbackReason, perrorcode: *mut UErrorCode)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UConverterFromUnicodeArgs {
    pub size: u16,
    pub flush: UBool,
    pub converter: *mut UConverter,
    pub source: *const UChar,
    pub sourceLimit: *const UChar,
    pub target: *mut i8,
    pub targetLimit: *const i8,
    pub offsets: *mut i32,
}
impl Default for UConverterFromUnicodeArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UConverterPlatform = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UConverterSelector(pub u8);
pub type UConverterToUCallback = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, args: *mut UConverterToUnicodeArgs, codeunits: *const i8, length: i32, reason: UConverterCallbackReason, perrorcode: *mut UErrorCode)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UConverterToUnicodeArgs {
    pub size: u16,
    pub flush: UBool,
    pub converter: *mut UConverter,
    pub source: *const i8,
    pub sourceLimit: *const i8,
    pub target: *mut UChar,
    pub targetLimit: *const UChar,
    pub offsets: *mut i32,
}
impl Default for UConverterToUnicodeArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UConverterType = i32;
pub type UConverterUnicodeSet = i32;
pub type UCurrCurrencyType = i32;
pub type UCurrNameStyle = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UCurrRegistryKey(pub *const core::ffi::c_void);
impl Default for UCurrRegistryKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UCurrencySpacing = i32;
pub type UCurrencyUsage = i32;
pub const UDATPG_ABBREVIATED: UDateTimePGDisplayWidth = 1;
pub const UDATPG_BASE_CONFLICT: UDateTimePatternConflict = 1;
pub const UDATPG_CONFLICT: UDateTimePatternConflict = 2;
pub const UDATPG_DAYPERIOD_FIELD: UDateTimePatternField = 10;
pub const UDATPG_DAY_FIELD: UDateTimePatternField = 9;
pub const UDATPG_DAY_OF_WEEK_IN_MONTH_FIELD: UDateTimePatternField = 8;
pub const UDATPG_DAY_OF_YEAR_FIELD: UDateTimePatternField = 7;
pub const UDATPG_ERA_FIELD: UDateTimePatternField = 0;
pub const UDATPG_FIELD_COUNT: UDateTimePatternField = 16;
pub const UDATPG_FRACTIONAL_SECOND_FIELD: UDateTimePatternField = 14;
pub const UDATPG_HOUR_FIELD: UDateTimePatternField = 11;
pub const UDATPG_MATCH_ALL_FIELDS_LENGTH: UDateTimePatternMatchOptions = 65535;
pub const UDATPG_MATCH_HOUR_FIELD_LENGTH: UDateTimePatternMatchOptions = 2048;
pub const UDATPG_MATCH_NO_OPTIONS: UDateTimePatternMatchOptions = 0;
pub const UDATPG_MINUTE_FIELD: UDateTimePatternField = 12;
pub const UDATPG_MONTH_FIELD: UDateTimePatternField = 3;
pub const UDATPG_NARROW: UDateTimePGDisplayWidth = 2;
pub const UDATPG_NO_CONFLICT: UDateTimePatternConflict = 0;
pub const UDATPG_QUARTER_FIELD: UDateTimePatternField = 2;
pub const UDATPG_SECOND_FIELD: UDateTimePatternField = 13;
pub const UDATPG_WEEKDAY_FIELD: UDateTimePatternField = 6;
pub const UDATPG_WEEK_OF_MONTH_FIELD: UDateTimePatternField = 5;
pub const UDATPG_WEEK_OF_YEAR_FIELD: UDateTimePatternField = 4;
pub const UDATPG_WIDE: UDateTimePGDisplayWidth = 0;
pub const UDATPG_YEAR_FIELD: UDateTimePatternField = 1;
pub const UDATPG_ZONE_FIELD: UDateTimePatternField = 15;
pub const UDAT_ABBR_GENERIC_TZ: windows_core::PCSTR = windows_core::s!("v");
pub const UDAT_ABBR_MONTH: windows_core::PCSTR = windows_core::s!("MMM");
pub const UDAT_ABBR_MONTH_DAY: windows_core::PCSTR = windows_core::s!("MMMd");
pub const UDAT_ABBR_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("MMMEd");
pub const UDAT_ABBR_QUARTER: windows_core::PCSTR = windows_core::s!("QQQ");
pub const UDAT_ABBR_SPECIFIC_TZ: windows_core::PCSTR = windows_core::s!("z");
pub const UDAT_ABBR_UTC_TZ: windows_core::PCSTR = windows_core::s!("ZZZZ");
pub const UDAT_ABBR_WEEKDAY: windows_core::PCSTR = windows_core::s!("E");
pub const UDAT_AM_PMS: UDateFormatSymbolType = 5;
pub const UDAT_AM_PM_FIELD: UDateFormatField = 14;
pub const UDAT_AM_PM_MIDNIGHT_NOON_FIELD: UDateFormatField = 35;
pub const UDAT_BOOLEAN_ATTRIBUTE_COUNT: UDateFormatBooleanAttribute = 4;
pub const UDAT_CYCLIC_YEARS_ABBREVIATED: UDateFormatSymbolType = 23;
pub const UDAT_CYCLIC_YEARS_NARROW: UDateFormatSymbolType = 24;
pub const UDAT_CYCLIC_YEARS_WIDE: UDateFormatSymbolType = 22;
pub const UDAT_DATE_FIELD: UDateFormatField = 3;
pub const UDAT_DAY: windows_core::PCSTR = windows_core::s!("d");
pub const UDAT_DAY_OF_WEEK_FIELD: UDateFormatField = 9;
pub const UDAT_DAY_OF_WEEK_IN_MONTH_FIELD: UDateFormatField = 11;
pub const UDAT_DAY_OF_YEAR_FIELD: UDateFormatField = 10;
pub const UDAT_DEFAULT: UDateFormatStyle = 2;
pub const UDAT_DOW_LOCAL_FIELD: UDateFormatField = 19;
pub const UDAT_ERAS: UDateFormatSymbolType = 0;
pub const UDAT_ERA_FIELD: UDateFormatField = 0;
pub const UDAT_ERA_NAMES: UDateFormatSymbolType = 7;
pub const UDAT_EXTENDED_YEAR_FIELD: UDateFormatField = 20;
pub const UDAT_FLEXIBLE_DAY_PERIOD_FIELD: UDateFormatField = 36;
pub const UDAT_FRACTIONAL_SECOND_FIELD: UDateFormatField = 8;
pub const UDAT_FULL: UDateFormatStyle = 0;
pub const UDAT_FULL_RELATIVE: UDateFormatStyle = 128;
pub const UDAT_GENERIC_TZ: windows_core::PCSTR = windows_core::s!("vvvv");
pub const UDAT_HOUR: windows_core::PCSTR = windows_core::s!("j");
pub const UDAT_HOUR0_FIELD: UDateFormatField = 16;
pub const UDAT_HOUR1_FIELD: UDateFormatField = 15;
pub const UDAT_HOUR24: windows_core::PCSTR = windows_core::s!("H");
pub const UDAT_HOUR24_MINUTE: windows_core::PCSTR = windows_core::s!("Hm");
pub const UDAT_HOUR24_MINUTE_SECOND: windows_core::PCSTR = windows_core::s!("Hms");
pub const UDAT_HOUR_CYCLE_11: UDateFormatHourCycle = 0;
pub const UDAT_HOUR_CYCLE_12: UDateFormatHourCycle = 1;
pub const UDAT_HOUR_CYCLE_23: UDateFormatHourCycle = 2;
pub const UDAT_HOUR_CYCLE_24: UDateFormatHourCycle = 3;
pub const UDAT_HOUR_MINUTE: windows_core::PCSTR = windows_core::s!("jm");
pub const UDAT_HOUR_MINUTE_SECOND: windows_core::PCSTR = windows_core::s!("jms");
pub const UDAT_HOUR_OF_DAY0_FIELD: UDateFormatField = 5;
pub const UDAT_HOUR_OF_DAY1_FIELD: UDateFormatField = 4;
pub const UDAT_JULIAN_DAY_FIELD: UDateFormatField = 21;
pub const UDAT_LOCALIZED_CHARS: UDateFormatSymbolType = 6;
pub const UDAT_LOCATION_TZ: windows_core::PCSTR = windows_core::s!("VVVV");
pub const UDAT_LONG: UDateFormatStyle = 1;
pub const UDAT_LONG_RELATIVE: UDateFormatStyle = 129;
pub const UDAT_MEDIUM: UDateFormatStyle = 2;
pub const UDAT_MEDIUM_RELATIVE: UDateFormatStyle = 130;
pub const UDAT_MILLISECONDS_IN_DAY_FIELD: UDateFormatField = 22;
pub const UDAT_MINUTE: windows_core::PCSTR = windows_core::s!("m");
pub const UDAT_MINUTE_FIELD: UDateFormatField = 6;
pub const UDAT_MINUTE_SECOND: windows_core::PCSTR = windows_core::s!("ms");
pub const UDAT_MONTH: windows_core::PCSTR = windows_core::s!("MMMM");
pub const UDAT_MONTHS: UDateFormatSymbolType = 1;
pub const UDAT_MONTH_DAY: windows_core::PCSTR = windows_core::s!("MMMMd");
pub const UDAT_MONTH_FIELD: UDateFormatField = 2;
pub const UDAT_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("MMMMEEEEd");
pub const UDAT_NARROW_MONTHS: UDateFormatSymbolType = 8;
pub const UDAT_NARROW_QUARTERS: UDateFormatSymbolType = 28;
pub const UDAT_NARROW_WEEKDAYS: UDateFormatSymbolType = 9;
pub const UDAT_NONE: UDateFormatStyle = -1;
pub const UDAT_NUM_MONTH: windows_core::PCSTR = windows_core::s!("M");
pub const UDAT_NUM_MONTH_DAY: windows_core::PCSTR = windows_core::s!("Md");
pub const UDAT_NUM_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("MEd");
pub const UDAT_PARSE_ALLOW_NUMERIC: UDateFormatBooleanAttribute = 1;
pub const UDAT_PARSE_ALLOW_WHITESPACE: UDateFormatBooleanAttribute = 0;
pub const UDAT_PARSE_MULTIPLE_PATTERNS_FOR_MATCH: UDateFormatBooleanAttribute = 3;
pub const UDAT_PARSE_PARTIAL_LITERAL_MATCH: UDateFormatBooleanAttribute = 2;
pub const UDAT_PATTERN: UDateFormatStyle = -2;
pub const UDAT_QUARTER: windows_core::PCSTR = windows_core::s!("QQQQ");
pub const UDAT_QUARTERS: UDateFormatSymbolType = 16;
pub const UDAT_QUARTER_FIELD: UDateFormatField = 27;
pub const UDAT_RELATIVE: UDateFormatStyle = 128;
pub const UDAT_REL_LITERAL_FIELD: URelativeDateTimeFormatterField = 0;
pub const UDAT_REL_NUMERIC_FIELD: URelativeDateTimeFormatterField = 1;
pub const UDAT_REL_UNIT_DAY: URelativeDateTimeUnit = 4;
pub const UDAT_REL_UNIT_FRIDAY: URelativeDateTimeUnit = 13;
pub const UDAT_REL_UNIT_HOUR: URelativeDateTimeUnit = 5;
pub const UDAT_REL_UNIT_MINUTE: URelativeDateTimeUnit = 6;
pub const UDAT_REL_UNIT_MONDAY: URelativeDateTimeUnit = 9;
pub const UDAT_REL_UNIT_MONTH: URelativeDateTimeUnit = 2;
pub const UDAT_REL_UNIT_QUARTER: URelativeDateTimeUnit = 1;
pub const UDAT_REL_UNIT_SATURDAY: URelativeDateTimeUnit = 14;
pub const UDAT_REL_UNIT_SECOND: URelativeDateTimeUnit = 7;
pub const UDAT_REL_UNIT_SUNDAY: URelativeDateTimeUnit = 8;
pub const UDAT_REL_UNIT_THURSDAY: URelativeDateTimeUnit = 12;
pub const UDAT_REL_UNIT_TUESDAY: URelativeDateTimeUnit = 10;
pub const UDAT_REL_UNIT_WEDNESDAY: URelativeDateTimeUnit = 11;
pub const UDAT_REL_UNIT_WEEK: URelativeDateTimeUnit = 3;
pub const UDAT_REL_UNIT_YEAR: URelativeDateTimeUnit = 0;
pub const UDAT_SECOND: windows_core::PCSTR = windows_core::s!("s");
pub const UDAT_SECOND_FIELD: UDateFormatField = 7;
pub const UDAT_SHORT: UDateFormatStyle = 3;
pub const UDAT_SHORTER_WEEKDAYS: UDateFormatSymbolType = 20;
pub const UDAT_SHORT_MONTHS: UDateFormatSymbolType = 2;
pub const UDAT_SHORT_QUARTERS: UDateFormatSymbolType = 17;
pub const UDAT_SHORT_RELATIVE: UDateFormatStyle = 131;
pub const UDAT_SHORT_WEEKDAYS: UDateFormatSymbolType = 4;
pub const UDAT_SPECIFIC_TZ: windows_core::PCSTR = windows_core::s!("zzzz");
pub const UDAT_STANDALONE_DAY_FIELD: UDateFormatField = 25;
pub const UDAT_STANDALONE_MONTHS: UDateFormatSymbolType = 10;
pub const UDAT_STANDALONE_MONTH_FIELD: UDateFormatField = 26;
pub const UDAT_STANDALONE_NARROW_MONTHS: UDateFormatSymbolType = 12;
pub const UDAT_STANDALONE_NARROW_QUARTERS: UDateFormatSymbolType = 29;
pub const UDAT_STANDALONE_NARROW_WEEKDAYS: UDateFormatSymbolType = 15;
pub const UDAT_STANDALONE_QUARTERS: UDateFormatSymbolType = 18;
pub const UDAT_STANDALONE_QUARTER_FIELD: UDateFormatField = 28;
pub const UDAT_STANDALONE_SHORTER_WEEKDAYS: UDateFormatSymbolType = 21;
pub const UDAT_STANDALONE_SHORT_MONTHS: UDateFormatSymbolType = 11;
pub const UDAT_STANDALONE_SHORT_QUARTERS: UDateFormatSymbolType = 19;
pub const UDAT_STANDALONE_SHORT_WEEKDAYS: UDateFormatSymbolType = 14;
pub const UDAT_STANDALONE_WEEKDAYS: UDateFormatSymbolType = 13;
pub const UDAT_STYLE_LONG: UDateRelativeDateTimeFormatterStyle = 0;
pub const UDAT_STYLE_NARROW: UDateRelativeDateTimeFormatterStyle = 2;
pub const UDAT_STYLE_SHORT: UDateRelativeDateTimeFormatterStyle = 1;
pub const UDAT_TIMEZONE_FIELD: UDateFormatField = 17;
pub const UDAT_TIMEZONE_GENERIC_FIELD: UDateFormatField = 24;
pub const UDAT_TIMEZONE_ISO_FIELD: UDateFormatField = 32;
pub const UDAT_TIMEZONE_ISO_LOCAL_FIELD: UDateFormatField = 33;
pub const UDAT_TIMEZONE_LOCALIZED_GMT_OFFSET_FIELD: UDateFormatField = 31;
pub const UDAT_TIMEZONE_RFC_FIELD: UDateFormatField = 23;
pub const UDAT_TIMEZONE_SPECIAL_FIELD: UDateFormatField = 29;
pub const UDAT_WEEKDAY: windows_core::PCSTR = windows_core::s!("EEEE");
pub const UDAT_WEEKDAYS: UDateFormatSymbolType = 3;
pub const UDAT_WEEK_OF_MONTH_FIELD: UDateFormatField = 13;
pub const UDAT_WEEK_OF_YEAR_FIELD: UDateFormatField = 12;
pub const UDAT_YEAR: windows_core::PCSTR = windows_core::s!("y");
pub const UDAT_YEAR_ABBR_MONTH: windows_core::PCSTR = windows_core::s!("yMMM");
pub const UDAT_YEAR_ABBR_MONTH_DAY: windows_core::PCSTR = windows_core::s!("yMMMd");
pub const UDAT_YEAR_ABBR_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("yMMMEd");
pub const UDAT_YEAR_ABBR_QUARTER: windows_core::PCSTR = windows_core::s!("yQQQ");
pub const UDAT_YEAR_FIELD: UDateFormatField = 1;
pub const UDAT_YEAR_MONTH: windows_core::PCSTR = windows_core::s!("yMMMM");
pub const UDAT_YEAR_MONTH_DAY: windows_core::PCSTR = windows_core::s!("yMMMMd");
pub const UDAT_YEAR_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("yMMMMEEEEd");
pub const UDAT_YEAR_NAME_FIELD: UDateFormatField = 30;
pub const UDAT_YEAR_NUM_MONTH: windows_core::PCSTR = windows_core::s!("yM");
pub const UDAT_YEAR_NUM_MONTH_DAY: windows_core::PCSTR = windows_core::s!("yMd");
pub const UDAT_YEAR_NUM_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("yMEd");
pub const UDAT_YEAR_QUARTER: windows_core::PCSTR = windows_core::s!("yQQQQ");
pub const UDAT_YEAR_WOY_FIELD: UDateFormatField = 18;
pub const UDAT_ZODIAC_NAMES_ABBREVIATED: UDateFormatSymbolType = 26;
pub const UDAT_ZODIAC_NAMES_NARROW: UDateFormatSymbolType = 27;
pub const UDAT_ZODIAC_NAMES_WIDE: UDateFormatSymbolType = 25;
pub const UDISPCTX_CAPITALIZATION_FOR_BEGINNING_OF_SENTENCE: UDisplayContext = 258;
pub const UDISPCTX_CAPITALIZATION_FOR_MIDDLE_OF_SENTENCE: UDisplayContext = 257;
pub const UDISPCTX_CAPITALIZATION_FOR_STANDALONE: UDisplayContext = 260;
pub const UDISPCTX_CAPITALIZATION_FOR_UI_LIST_OR_MENU: UDisplayContext = 259;
pub const UDISPCTX_CAPITALIZATION_NONE: UDisplayContext = 256;
pub const UDISPCTX_DIALECT_NAMES: UDisplayContext = 1;
pub const UDISPCTX_LENGTH_FULL: UDisplayContext = 512;
pub const UDISPCTX_LENGTH_SHORT: UDisplayContext = 513;
pub const UDISPCTX_NO_SUBSTITUTE: UDisplayContext = 769;
pub const UDISPCTX_STANDARD_NAMES: UDisplayContext = 0;
pub const UDISPCTX_SUBSTITUTE: UDisplayContext = 768;
pub const UDISPCTX_TYPE_CAPITALIZATION: UDisplayContextType = 1;
pub const UDISPCTX_TYPE_DIALECT_HANDLING: UDisplayContextType = 0;
pub const UDISPCTX_TYPE_DISPLAY_LENGTH: UDisplayContextType = 2;
pub const UDISPCTX_TYPE_SUBSTITUTE_HANDLING: UDisplayContextType = 3;
pub const UDTS_DB2_TIME: UDateTimeScale = 8;
pub const UDTS_DOTNET_DATE_TIME: UDateTimeScale = 4;
pub const UDTS_EXCEL_TIME: UDateTimeScale = 7;
pub const UDTS_ICU4C_TIME: UDateTimeScale = 2;
pub const UDTS_JAVA_TIME: UDateTimeScale = 0;
pub const UDTS_MAC_OLD_TIME: UDateTimeScale = 5;
pub const UDTS_MAC_TIME: UDateTimeScale = 6;
pub const UDTS_UNIX_MICROSECONDS_TIME: UDateTimeScale = 9;
pub const UDTS_UNIX_TIME: UDateTimeScale = 1;
pub const UDTS_WINDOWS_FILE_TIME: UDateTimeScale = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UDateFormat(pub *mut core::ffi::c_void);
impl Default for UDateFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UDateFormatBooleanAttribute = i32;
pub type UDateFormatField = i32;
pub type UDateFormatHourCycle = i32;
pub type UDateFormatStyle = i32;
pub type UDateFormatSymbolType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UDateFormatSymbols(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UDateIntervalFormat(pub u8);
pub type UDateRelativeDateTimeFormatterStyle = i32;
pub type UDateTimePGDisplayWidth = i32;
pub type UDateTimePatternConflict = i32;
pub type UDateTimePatternField = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UDateTimePatternGenerator(pub *mut core::ffi::c_void);
impl Default for UDateTimePatternGenerator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UDateTimePatternMatchOptions = i32;
pub type UDateTimeScale = i32;
pub type UDecompositionType = i32;
pub type UDialectHandling = i32;
pub type UDisplayContext = i32;
pub type UDisplayContextType = i32;
pub type UEastAsianWidth = i32;
pub type UEnumCharNamesFn = Option<unsafe extern "C" fn(context: *mut core::ffi::c_void, code: UChar32, namechoice: UCharNameChoice, name: *const i8, length: i32) -> UBool>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UEnumeration(pub u8);
pub type UErrorCode = i32;
pub const UFIELD_CATEGORY_DATE: UFieldCategory = 1;
pub const UFIELD_CATEGORY_DATE_INTERVAL: UFieldCategory = 5;
pub const UFIELD_CATEGORY_DATE_INTERVAL_SPAN: UFieldCategory = 4101;
pub const UFIELD_CATEGORY_LIST: UFieldCategory = 3;
pub const UFIELD_CATEGORY_LIST_SPAN: UFieldCategory = 4099;
pub const UFIELD_CATEGORY_NUMBER: UFieldCategory = 2;
pub const UFIELD_CATEGORY_NUMBER_RANGE_SPAN: UFieldCategory = 4098;
pub const UFIELD_CATEGORY_RELATIVE_DATETIME: UFieldCategory = 4;
pub const UFIELD_CATEGORY_UNDEFINED: UFieldCategory = 0;
pub const UFMT_ARRAY: UFormattableType = 4;
pub const UFMT_DATE: UFormattableType = 0;
pub const UFMT_DOUBLE: UFormattableType = 1;
pub const UFMT_INT64: UFormattableType = 5;
pub const UFMT_LONG: UFormattableType = 2;
pub const UFMT_OBJECT: UFormattableType = 6;
pub const UFMT_STRING: UFormattableType = 3;
pub type UFieldCategory = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFieldPosition {
    pub field: i32,
    pub beginIndex: i32,
    pub endIndex: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFieldPositionIterator(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UFormattable(pub *mut core::ffi::c_void);
impl Default for UFormattable {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UFormattableType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFormattedDateInterval(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFormattedList(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFormattedNumber(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFormattedNumberRange(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFormattedRelativeDateTime(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFormattedValue(pub u8);
pub const UGENDER_FEMALE: UGender = 1;
pub const UGENDER_MALE: UGender = 0;
pub const UGENDER_OTHER: UGender = 2;
pub type UGender = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UGenderInfo(pub u8);
pub type UGraphemeClusterBreak = i32;
pub type UHangulSyllableType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UIDNA(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UIDNAInfo {
    pub size: i16,
    pub isTransitionalDifferent: UBool,
    pub reservedB3: UBool,
    pub errors: u32,
    pub reservedI2: i32,
    pub reservedI3: i32,
}
pub const UIDNA_CHECK_BIDI: i32 = 4;
pub const UIDNA_CHECK_CONTEXTJ: i32 = 8;
pub const UIDNA_CHECK_CONTEXTO: i32 = 64;
pub const UIDNA_DEFAULT: i32 = 0;
pub const UIDNA_ERROR_BIDI: i32 = 2048;
pub const UIDNA_ERROR_CONTEXTJ: i32 = 4096;
pub const UIDNA_ERROR_CONTEXTO_DIGITS: i32 = 16384;
pub const UIDNA_ERROR_CONTEXTO_PUNCTUATION: i32 = 8192;
pub const UIDNA_ERROR_DISALLOWED: i32 = 128;
pub const UIDNA_ERROR_DOMAIN_NAME_TOO_LONG: i32 = 4;
pub const UIDNA_ERROR_EMPTY_LABEL: i32 = 1;
pub const UIDNA_ERROR_HYPHEN_3_4: i32 = 32;
pub const UIDNA_ERROR_INVALID_ACE_LABEL: i32 = 1024;
pub const UIDNA_ERROR_LABEL_HAS_DOT: i32 = 512;
pub const UIDNA_ERROR_LABEL_TOO_LONG: i32 = 2;
pub const UIDNA_ERROR_LEADING_COMBINING_MARK: i32 = 64;
pub const UIDNA_ERROR_LEADING_HYPHEN: i32 = 8;
pub const UIDNA_ERROR_PUNYCODE: i32 = 256;
pub const UIDNA_ERROR_TRAILING_HYPHEN: i32 = 16;
pub const UIDNA_NONTRANSITIONAL_TO_ASCII: i32 = 16;
pub const UIDNA_NONTRANSITIONAL_TO_UNICODE: i32 = 32;
pub const UIDNA_USE_STD3_RULES: i32 = 2;
pub const UITER_CURRENT: UCharIteratorOrigin = 1;
pub const UITER_LENGTH: UCharIteratorOrigin = 4;
pub const UITER_LIMIT: UCharIteratorOrigin = 2;
pub const UITER_NO_STATE: u32 = 4294967295;
pub const UITER_START: UCharIteratorOrigin = 0;
pub const UITER_UNKNOWN_INDEX: i32 = -2;
pub const UITER_ZERO: UCharIteratorOrigin = 3;
pub type UIndicPositionalCategory = i32;
pub type UIndicSyllabicCategory = i32;
pub type UJoiningGroup = i32;
pub type UJoiningType = i32;
pub const ULDN_DIALECT_NAMES: UDialectHandling = 1;
pub const ULDN_STANDARD_NAMES: UDialectHandling = 0;
pub const ULISTFMT_ELEMENT_FIELD: UListFormatterField = 1;
pub const ULISTFMT_LITERAL_FIELD: UListFormatterField = 0;
pub const ULISTFMT_TYPE_AND: UListFormatterType = 0;
pub const ULISTFMT_TYPE_OR: UListFormatterType = 1;
pub const ULISTFMT_TYPE_UNITS: UListFormatterType = 2;
pub const ULISTFMT_WIDTH_NARROW: UListFormatterWidth = 2;
pub const ULISTFMT_WIDTH_SHORT: UListFormatterWidth = 1;
pub const ULISTFMT_WIDTH_WIDE: UListFormatterWidth = 0;
pub const ULOCDATA_ALT_QUOTATION_END: ULocaleDataDelimiterType = 3;
pub const ULOCDATA_ALT_QUOTATION_START: ULocaleDataDelimiterType = 2;
pub const ULOCDATA_ES_AUXILIARY: ULocaleDataExemplarSetType = 1;
pub const ULOCDATA_ES_INDEX: ULocaleDataExemplarSetType = 2;
pub const ULOCDATA_ES_PUNCTUATION: ULocaleDataExemplarSetType = 3;
pub const ULOCDATA_ES_STANDARD: ULocaleDataExemplarSetType = 0;
pub const ULOCDATA_QUOTATION_END: ULocaleDataDelimiterType = 1;
pub const ULOCDATA_QUOTATION_START: ULocaleDataDelimiterType = 0;
pub const ULOC_ACCEPT_FAILED: UAcceptResult = 0;
pub const ULOC_ACCEPT_FALLBACK: UAcceptResult = 2;
pub const ULOC_ACCEPT_VALID: UAcceptResult = 1;
pub const ULOC_ACTUAL_LOCALE: ULocDataLocaleType = 0;
pub const ULOC_AVAILABLE_DEFAULT: ULocAvailableType = 0;
pub const ULOC_AVAILABLE_ONLY_LEGACY_ALIASES: ULocAvailableType = 1;
pub const ULOC_AVAILABLE_WITH_LEGACY_ALIASES: ULocAvailableType = 2;
pub const ULOC_CANADA: windows_core::PCSTR = windows_core::s!("en_CA");
pub const ULOC_CANADA_FRENCH: windows_core::PCSTR = windows_core::s!("fr_CA");
pub const ULOC_CHINA: windows_core::PCSTR = windows_core::s!("zh_CN");
pub const ULOC_CHINESE: windows_core::PCSTR = windows_core::s!("zh");
pub const ULOC_COUNTRY_CAPACITY: u32 = 4;
pub const ULOC_ENGLISH: windows_core::PCSTR = windows_core::s!("en");
pub const ULOC_FRANCE: windows_core::PCSTR = windows_core::s!("fr_FR");
pub const ULOC_FRENCH: windows_core::PCSTR = windows_core::s!("fr");
pub const ULOC_FULLNAME_CAPACITY: u32 = 157;
pub const ULOC_GERMAN: windows_core::PCSTR = windows_core::s!("de");
pub const ULOC_GERMANY: windows_core::PCSTR = windows_core::s!("de_DE");
pub const ULOC_ITALIAN: windows_core::PCSTR = windows_core::s!("it");
pub const ULOC_ITALY: windows_core::PCSTR = windows_core::s!("it_IT");
pub const ULOC_JAPAN: windows_core::PCSTR = windows_core::s!("ja_JP");
pub const ULOC_JAPANESE: windows_core::PCSTR = windows_core::s!("ja");
pub const ULOC_KEYWORDS_CAPACITY: u32 = 96;
pub const ULOC_KEYWORD_AND_VALUES_CAPACITY: u32 = 100;
pub const ULOC_KEYWORD_ASSIGN: u32 = 61;
pub const ULOC_KEYWORD_ASSIGN_UNICODE: u32 = 61;
pub const ULOC_KEYWORD_ITEM_SEPARATOR: u32 = 59;
pub const ULOC_KEYWORD_ITEM_SEPARATOR_UNICODE: u32 = 59;
pub const ULOC_KEYWORD_SEPARATOR: u32 = 64;
pub const ULOC_KEYWORD_SEPARATOR_UNICODE: u32 = 64;
pub const ULOC_KOREA: windows_core::PCSTR = windows_core::s!("ko_KR");
pub const ULOC_KOREAN: windows_core::PCSTR = windows_core::s!("ko");
pub const ULOC_LANG_CAPACITY: u32 = 12;
pub const ULOC_LAYOUT_BTT: ULayoutType = 3;
pub const ULOC_LAYOUT_LTR: ULayoutType = 0;
pub const ULOC_LAYOUT_RTL: ULayoutType = 1;
pub const ULOC_LAYOUT_TTB: ULayoutType = 2;
pub const ULOC_LAYOUT_UNKNOWN: ULayoutType = 4;
pub const ULOC_PRC: windows_core::PCSTR = windows_core::s!("zh_CN");
pub const ULOC_SCRIPT_CAPACITY: u32 = 6;
pub const ULOC_SIMPLIFIED_CHINESE: windows_core::PCSTR = windows_core::s!("zh_CN");
pub const ULOC_TAIWAN: windows_core::PCSTR = windows_core::s!("zh_TW");
pub const ULOC_TRADITIONAL_CHINESE: windows_core::PCSTR = windows_core::s!("zh_TW");
pub const ULOC_UK: windows_core::PCSTR = windows_core::s!("en_GB");
pub const ULOC_US: windows_core::PCSTR = windows_core::s!("en_US");
pub const ULOC_VALID_LOCALE: ULocDataLocaleType = 1;
pub type ULayoutType = i32;
pub type ULineBreak = i32;
pub type ULineBreakTag = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UListFormatter(pub u8);
pub type UListFormatterField = i32;
pub type UListFormatterType = i32;
pub type UListFormatterWidth = i32;
pub type ULocAvailableType = i32;
pub type ULocDataLocaleType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ULocaleData(pub u8);
pub type ULocaleDataDelimiterType = i32;
pub type ULocaleDataExemplarSetType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ULocaleDisplayNames(pub u8);
pub const UMS_SI: UMeasurementSystem = 0;
pub const UMS_UK: UMeasurementSystem = 2;
pub const UMS_US: UMeasurementSystem = 1;
pub type UMeasurementSystem = i32;
pub type UMemAllocFn = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
pub type UMemFreeFn = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, mem: *mut core::ffi::c_void)>;
pub type UMemReallocFn = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, mem: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UMessageFormat(pub *mut core::ffi::c_void);
impl Default for UMessageFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UMutableCPTrie(pub u8);
pub type UNESCAPE_CHAR_AT = Option<unsafe extern "C" fn(offset: i32, context: *mut core::ffi::c_void) -> UChar>;
pub const UNORM2_COMPOSE: UNormalization2Mode = 0;
pub const UNORM2_COMPOSE_CONTIGUOUS: UNormalization2Mode = 3;
pub const UNORM2_DECOMPOSE: UNormalization2Mode = 1;
pub const UNORM2_FCD: UNormalization2Mode = 2;
pub const UNORM_DEFAULT: UNormalizationMode = 4;
pub const UNORM_FCD: UNormalizationMode = 6;
pub const UNORM_INPUT_IS_FCD: u32 = 131072;
pub const UNORM_MAYBE: UNormalizationCheckResult = 2;
pub const UNORM_MODE_COUNT: UNormalizationMode = 7;
pub const UNORM_NFC: UNormalizationMode = 4;
pub const UNORM_NFD: UNormalizationMode = 2;
pub const UNORM_NFKC: UNormalizationMode = 5;
pub const UNORM_NFKD: UNormalizationMode = 3;
pub const UNORM_NO: UNormalizationCheckResult = 0;
pub const UNORM_NONE: UNormalizationMode = 1;
pub const UNORM_YES: UNormalizationCheckResult = 1;
pub const UNUM_CASH_CURRENCY: UNumberFormatStyle = 13;
pub const UNUM_COMPACT_FIELD: UNumberFormatFields = 12;
pub const UNUM_CURRENCY: UNumberFormatStyle = 2;
pub const UNUM_CURRENCY_ACCOUNTING: UNumberFormatStyle = 12;
pub const UNUM_CURRENCY_CODE: UNumberFormatTextAttribute = 5;
pub const UNUM_CURRENCY_FIELD: UNumberFormatFields = 7;
pub const UNUM_CURRENCY_INSERT: UCurrencySpacing = 2;
pub const UNUM_CURRENCY_ISO: UNumberFormatStyle = 10;
pub const UNUM_CURRENCY_MATCH: UCurrencySpacing = 0;
pub const UNUM_CURRENCY_PLURAL: UNumberFormatStyle = 11;
pub const UNUM_CURRENCY_SPACING_COUNT: UCurrencySpacing = 3;
pub const UNUM_CURRENCY_STANDARD: UNumberFormatStyle = 16;
pub const UNUM_CURRENCY_SURROUNDING_MATCH: UCurrencySpacing = 1;
pub const UNUM_CURRENCY_SYMBOL: UNumberFormatSymbol = 8;
pub const UNUM_CURRENCY_USAGE: UNumberFormatAttribute = 23;
pub const UNUM_DECIMAL: UNumberFormatStyle = 1;
pub const UNUM_DECIMAL_ALWAYS_SHOWN: UNumberFormatAttribute = 2;
pub const UNUM_DECIMAL_COMPACT_LONG: UNumberFormatStyle = 15;
pub const UNUM_DECIMAL_COMPACT_SHORT: UNumberFormatStyle = 14;
pub const UNUM_DECIMAL_SEPARATOR_ALWAYS: UNumberDecimalSeparatorDisplay = 1;
pub const UNUM_DECIMAL_SEPARATOR_AUTO: UNumberDecimalSeparatorDisplay = 0;
pub const UNUM_DECIMAL_SEPARATOR_COUNT: UNumberDecimalSeparatorDisplay = 2;
pub const UNUM_DECIMAL_SEPARATOR_FIELD: UNumberFormatFields = 2;
pub const UNUM_DECIMAL_SEPARATOR_SYMBOL: UNumberFormatSymbol = 0;
pub const UNUM_DEFAULT: UNumberFormatStyle = 1;
pub const UNUM_DEFAULT_RULESET: UNumberFormatTextAttribute = 6;
pub const UNUM_DIGIT_SYMBOL: UNumberFormatSymbol = 5;
pub const UNUM_DURATION: UNumberFormatStyle = 7;
pub const UNUM_EIGHT_DIGIT_SYMBOL: UNumberFormatSymbol = 25;
pub const UNUM_EXPONENTIAL_SYMBOL: UNumberFormatSymbol = 11;
pub const UNUM_EXPONENT_FIELD: UNumberFormatFields = 5;
pub const UNUM_EXPONENT_MULTIPLICATION_SYMBOL: UNumberFormatSymbol = 27;
pub const UNUM_EXPONENT_SIGN_FIELD: UNumberFormatFields = 4;
pub const UNUM_EXPONENT_SYMBOL_FIELD: UNumberFormatFields = 3;
pub const UNUM_FIVE_DIGIT_SYMBOL: UNumberFormatSymbol = 22;
pub const UNUM_FORMAT_ATTRIBUTE_VALUE_HIDDEN: UNumberFormatAttributeValue = 0;
pub const UNUM_FORMAT_FAIL_IF_MORE_THAN_MAX_DIGITS: UNumberFormatAttribute = 4096;
pub const UNUM_FORMAT_WIDTH: UNumberFormatAttribute = 13;
pub const UNUM_FOUR_DIGIT_SYMBOL: UNumberFormatSymbol = 21;
pub const UNUM_FRACTION_DIGITS: UNumberFormatAttribute = 8;
pub const UNUM_FRACTION_FIELD: UNumberFormatFields = 1;
pub const UNUM_GROUPING_AUTO: UNumberGroupingStrategy = 2;
pub const UNUM_GROUPING_MIN2: UNumberGroupingStrategy = 1;
pub const UNUM_GROUPING_OFF: UNumberGroupingStrategy = 0;
pub const UNUM_GROUPING_ON_ALIGNED: UNumberGroupingStrategy = 3;
pub const UNUM_GROUPING_SEPARATOR_FIELD: UNumberFormatFields = 6;
pub const UNUM_GROUPING_SEPARATOR_SYMBOL: UNumberFormatSymbol = 1;
pub const UNUM_GROUPING_SIZE: UNumberFormatAttribute = 10;
pub const UNUM_GROUPING_THOUSANDS: UNumberGroupingStrategy = 4;
pub const UNUM_GROUPING_USED: UNumberFormatAttribute = 1;
pub const UNUM_IDENTITY_FALLBACK_APPROXIMATELY: UNumberRangeIdentityFallback = 2;
pub const UNUM_IDENTITY_FALLBACK_APPROXIMATELY_OR_SINGLE_VALUE: UNumberRangeIdentityFallback = 1;
pub const UNUM_IDENTITY_FALLBACK_RANGE: UNumberRangeIdentityFallback = 3;
pub const UNUM_IDENTITY_FALLBACK_SINGLE_VALUE: UNumberRangeIdentityFallback = 0;
pub const UNUM_IDENTITY_RESULT_EQUAL_AFTER_ROUNDING: UNumberRangeIdentityResult = 1;
pub const UNUM_IDENTITY_RESULT_EQUAL_BEFORE_ROUNDING: UNumberRangeIdentityResult = 0;
pub const UNUM_IDENTITY_RESULT_NOT_EQUAL: UNumberRangeIdentityResult = 2;
pub const UNUM_IGNORE: UNumberFormatStyle = 0;
pub const UNUM_INFINITY_SYMBOL: UNumberFormatSymbol = 14;
pub const UNUM_INTEGER_DIGITS: UNumberFormatAttribute = 5;
pub const UNUM_INTEGER_FIELD: UNumberFormatFields = 0;
pub const UNUM_INTL_CURRENCY_SYMBOL: UNumberFormatSymbol = 9;
pub const UNUM_LENIENT_PARSE: UNumberFormatAttribute = 19;
pub const UNUM_LONG: UNumberCompactStyle = 1;
pub const UNUM_MAX_FRACTION_DIGITS: UNumberFormatAttribute = 6;
pub const UNUM_MAX_INTEGER_DIGITS: UNumberFormatAttribute = 3;
pub const UNUM_MAX_SIGNIFICANT_DIGITS: UNumberFormatAttribute = 18;
pub const UNUM_MEASURE_UNIT_FIELD: UNumberFormatFields = 11;
pub const UNUM_MINIMUM_GROUPING_DIGITS: UNumberFormatAttribute = 22;
pub const UNUM_MINIMUM_GROUPING_DIGITS_AUTO: UNumberFormatMinimumGroupingDigits = -2;
pub const UNUM_MINIMUM_GROUPING_DIGITS_MIN2: UNumberFormatMinimumGroupingDigits = -3;
pub const UNUM_MINUS_SIGN_SYMBOL: UNumberFormatSymbol = 6;
pub const UNUM_MIN_FRACTION_DIGITS: UNumberFormatAttribute = 7;
pub const UNUM_MIN_INTEGER_DIGITS: UNumberFormatAttribute = 4;
pub const UNUM_MIN_SIGNIFICANT_DIGITS: UNumberFormatAttribute = 17;
pub const UNUM_MONETARY_GROUPING_SEPARATOR_SYMBOL: UNumberFormatSymbol = 17;
pub const UNUM_MONETARY_SEPARATOR_SYMBOL: UNumberFormatSymbol = 10;
pub const UNUM_MULTIPLIER: UNumberFormatAttribute = 9;
pub const UNUM_NAN_SYMBOL: UNumberFormatSymbol = 15;
pub const UNUM_NEGATIVE_PREFIX: UNumberFormatTextAttribute = 2;
pub const UNUM_NEGATIVE_SUFFIX: UNumberFormatTextAttribute = 3;
pub const UNUM_NINE_DIGIT_SYMBOL: UNumberFormatSymbol = 26;
pub const UNUM_NUMBERING_SYSTEM: UNumberFormatStyle = 8;
pub const UNUM_ONE_DIGIT_SYMBOL: UNumberFormatSymbol = 18;
pub const UNUM_ORDINAL: UNumberFormatStyle = 6;
pub const UNUM_PADDING_CHARACTER: UNumberFormatTextAttribute = 4;
pub const UNUM_PADDING_POSITION: UNumberFormatAttribute = 14;
pub const UNUM_PAD_AFTER_PREFIX: UNumberFormatPadPosition = 1;
pub const UNUM_PAD_AFTER_SUFFIX: UNumberFormatPadPosition = 3;
pub const UNUM_PAD_BEFORE_PREFIX: UNumberFormatPadPosition = 0;
pub const UNUM_PAD_BEFORE_SUFFIX: UNumberFormatPadPosition = 2;
pub const UNUM_PAD_ESCAPE_SYMBOL: UNumberFormatSymbol = 13;
pub const UNUM_PARSE_ALL_INPUT: UNumberFormatAttribute = 20;
pub const UNUM_PARSE_CASE_SENSITIVE: UNumberFormatAttribute = 4099;
pub const UNUM_PARSE_DECIMAL_MARK_REQUIRED: UNumberFormatAttribute = 4098;
pub const UNUM_PARSE_INT_ONLY: UNumberFormatAttribute = 0;
pub const UNUM_PARSE_NO_EXPONENT: UNumberFormatAttribute = 4097;
pub const UNUM_PATTERN_DECIMAL: UNumberFormatStyle = 0;
pub const UNUM_PATTERN_RULEBASED: UNumberFormatStyle = 9;
pub const UNUM_PATTERN_SEPARATOR_SYMBOL: UNumberFormatSymbol = 2;
pub const UNUM_PERCENT: UNumberFormatStyle = 3;
pub const UNUM_PERCENT_FIELD: UNumberFormatFields = 8;
pub const UNUM_PERCENT_SYMBOL: UNumberFormatSymbol = 3;
pub const UNUM_PERMILL_FIELD: UNumberFormatFields = 9;
pub const UNUM_PERMILL_SYMBOL: UNumberFormatSymbol = 12;
pub const UNUM_PLUS_SIGN_SYMBOL: UNumberFormatSymbol = 7;
pub const UNUM_POSITIVE_PREFIX: UNumberFormatTextAttribute = 0;
pub const UNUM_POSITIVE_SUFFIX: UNumberFormatTextAttribute = 1;
pub const UNUM_PUBLIC_RULESETS: UNumberFormatTextAttribute = 7;
pub const UNUM_RANGE_COLLAPSE_ALL: UNumberRangeCollapse = 3;
pub const UNUM_RANGE_COLLAPSE_AUTO: UNumberRangeCollapse = 0;
pub const UNUM_RANGE_COLLAPSE_NONE: UNumberRangeCollapse = 1;
pub const UNUM_RANGE_COLLAPSE_UNIT: UNumberRangeCollapse = 2;
pub const UNUM_ROUNDING_INCREMENT: UNumberFormatAttribute = 12;
pub const UNUM_ROUNDING_MODE: UNumberFormatAttribute = 11;
pub const UNUM_ROUND_CEILING: UNumberFormatRoundingMode = 0;
pub const UNUM_ROUND_DOWN: UNumberFormatRoundingMode = 2;
pub const UNUM_ROUND_FLOOR: UNumberFormatRoundingMode = 1;
pub const UNUM_ROUND_HALFDOWN: UNumberFormatRoundingMode = 5;
pub const UNUM_ROUND_HALFEVEN: UNumberFormatRoundingMode = 4;
pub const UNUM_ROUND_HALFUP: UNumberFormatRoundingMode = 6;
pub const UNUM_ROUND_HALF_CEILING: UNumberFormatRoundingMode = 9;
pub const UNUM_ROUND_HALF_FLOOR: UNumberFormatRoundingMode = 10;
pub const UNUM_ROUND_HALF_ODD: UNumberFormatRoundingMode = 8;
pub const UNUM_ROUND_UNNECESSARY: UNumberFormatRoundingMode = 7;
pub const UNUM_ROUND_UP: UNumberFormatRoundingMode = 3;
pub const UNUM_SCALE: UNumberFormatAttribute = 21;
pub const UNUM_SCIENTIFIC: UNumberFormatStyle = 4;
pub const UNUM_SECONDARY_GROUPING_SIZE: UNumberFormatAttribute = 15;
pub const UNUM_SEVEN_DIGIT_SYMBOL: UNumberFormatSymbol = 24;
pub const UNUM_SHORT: UNumberCompactStyle = 0;
pub const UNUM_SIGNIFICANT_DIGITS_USED: UNumberFormatAttribute = 16;
pub const UNUM_SIGNIFICANT_DIGIT_SYMBOL: UNumberFormatSymbol = 16;
pub const UNUM_SIGN_ACCOUNTING: UNumberSignDisplay = 3;
pub const UNUM_SIGN_ACCOUNTING_ALWAYS: UNumberSignDisplay = 4;
pub const UNUM_SIGN_ACCOUNTING_EXCEPT_ZERO: UNumberSignDisplay = 6;
pub const UNUM_SIGN_ACCOUNTING_NEGATIVE: UNumberSignDisplay = 8;
pub const UNUM_SIGN_ALWAYS: UNumberSignDisplay = 1;
pub const UNUM_SIGN_ALWAYS_SHOWN: UNumberFormatAttribute = 4100;
pub const UNUM_SIGN_AUTO: UNumberSignDisplay = 0;
pub const UNUM_SIGN_COUNT: UNumberSignDisplay = 9;
pub const UNUM_SIGN_EXCEPT_ZERO: UNumberSignDisplay = 5;
pub const UNUM_SIGN_FIELD: UNumberFormatFields = 10;
pub const UNUM_SIGN_NEGATIVE: UNumberSignDisplay = 7;
pub const UNUM_SIGN_NEVER: UNumberSignDisplay = 2;
pub const UNUM_SIX_DIGIT_SYMBOL: UNumberFormatSymbol = 23;
pub const UNUM_SPELLOUT: UNumberFormatStyle = 5;
pub const UNUM_THREE_DIGIT_SYMBOL: UNumberFormatSymbol = 20;
pub const UNUM_TRAILING_ZERO_AUTO: UNumberTrailingZeroDisplay = 0;
pub const UNUM_TRAILING_ZERO_HIDE_IF_WHOLE: UNumberTrailingZeroDisplay = 1;
pub const UNUM_TWO_DIGIT_SYMBOL: UNumberFormatSymbol = 19;
pub const UNUM_UNIT_WIDTH_COUNT: UNumberUnitWidth = 7;
pub const UNUM_UNIT_WIDTH_FORMAL: UNumberUnitWidth = 4;
pub const UNUM_UNIT_WIDTH_FULL_NAME: UNumberUnitWidth = 2;
pub const UNUM_UNIT_WIDTH_HIDDEN: UNumberUnitWidth = 6;
pub const UNUM_UNIT_WIDTH_ISO_CODE: UNumberUnitWidth = 3;
pub const UNUM_UNIT_WIDTH_NARROW: UNumberUnitWidth = 0;
pub const UNUM_UNIT_WIDTH_SHORT: UNumberUnitWidth = 1;
pub const UNUM_UNIT_WIDTH_VARIANT: UNumberUnitWidth = 5;
pub const UNUM_ZERO_DIGIT_SYMBOL: UNumberFormatSymbol = 4;
pub type UNormalization2Mode = i32;
pub type UNormalizationCheckResult = i32;
pub type UNormalizationMode = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNormalizer2(pub u8);
pub type UNumberCompactStyle = i32;
pub type UNumberDecimalSeparatorDisplay = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UNumberFormat(pub *mut core::ffi::c_void);
impl Default for UNumberFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UNumberFormatAttribute = i32;
pub type UNumberFormatAttributeValue = i32;
pub type UNumberFormatFields = i32;
pub type UNumberFormatMinimumGroupingDigits = i32;
pub type UNumberFormatPadPosition = i32;
pub type UNumberFormatRoundingMode = i32;
pub type UNumberFormatStyle = i32;
pub type UNumberFormatSymbol = i32;
pub type UNumberFormatTextAttribute = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNumberFormatter(pub u8);
pub type UNumberGroupingStrategy = i32;
pub type UNumberRangeCollapse = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNumberRangeFormatter(pub u8);
pub type UNumberRangeIdentityFallback = i32;
pub type UNumberRangeIdentityResult = i32;
pub type UNumberSignDisplay = i32;
pub type UNumberTrailingZeroDisplay = i32;
pub type UNumberUnitWidth = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNumberingSystem(pub u8);
pub type UNumericType = i32;
pub const UPLURAL_TYPE_CARDINAL: UPluralType = 0;
pub const UPLURAL_TYPE_ORDINAL: UPluralType = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UParseError {
    pub line: i32,
    pub offset: i32,
    pub preContext: [UChar; 16],
    pub postContext: [UChar; 16],
}
impl Default for UParseError {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UPluralRules(pub u8);
pub type UPluralType = i32;
pub type UProperty = i32;
pub type UPropertyNameChoice = i32;
pub const UREGEX_CASE_INSENSITIVE: URegexpFlag = 2;
pub const UREGEX_COMMENTS: URegexpFlag = 4;
pub const UREGEX_DOTALL: URegexpFlag = 32;
pub const UREGEX_ERROR_ON_UNKNOWN_ESCAPES: URegexpFlag = 512;
pub const UREGEX_LITERAL: URegexpFlag = 16;
pub const UREGEX_MULTILINE: URegexpFlag = 8;
pub const UREGEX_UNIX_LINES: URegexpFlag = 1;
pub const UREGEX_UWORD: URegexpFlag = 256;
pub const URES_ALIAS: UResType = 3;
pub const URES_ARRAY: UResType = 8;
pub const URES_BINARY: UResType = 1;
pub const URES_INT: UResType = 7;
pub const URES_INT_VECTOR: UResType = 14;
pub const URES_NONE: UResType = -1;
pub const URES_STRING: UResType = 0;
pub const URES_TABLE: UResType = 2;
pub const URGN_CONTINENT: URegionType = 3;
pub const URGN_DEPRECATED: URegionType = 6;
pub const URGN_GROUPING: URegionType = 5;
pub const URGN_SUBCONTINENT: URegionType = 4;
pub const URGN_TERRITORY: URegionType = 1;
pub const URGN_UNKNOWN: URegionType = 0;
pub const URGN_WORLD: URegionType = 2;
pub type URegexFindProgressCallback = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, matchindex: i64) -> UBool>;
pub type URegexMatchCallback = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, steps: i32) -> UBool>;
pub type URegexpFlag = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct URegion(pub u8);
pub type URegionType = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct URegistryKey(pub *const core::ffi::c_void);
impl Default for URegistryKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct URegularExpression(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct URelativeDateTimeFormatter(pub u8);
pub type URelativeDateTimeFormatterField = i32;
pub type URelativeDateTimeUnit = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UReplaceable(pub *mut core::ffi::c_void);
impl Default for UReplaceable {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UReplaceableCallbacks {
    pub length: *mut u8,
    pub charAt: *mut u8,
    pub char32At: *mut u8,
    pub replace: *mut u8,
    pub extract: *mut u8,
    pub copy: *mut u8,
}
impl Default for UReplaceableCallbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UResType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UResourceBundle(pub u8);
pub type URestrictionLevel = i32;
pub const USCRIPT_ADLAM: UScriptCode = 167;
pub const USCRIPT_AFAKA: UScriptCode = 147;
pub const USCRIPT_AHOM: UScriptCode = 161;
pub const USCRIPT_ANATOLIAN_HIEROGLYPHS: UScriptCode = 156;
pub const USCRIPT_ARABIC: UScriptCode = 2;
pub const USCRIPT_ARMENIAN: UScriptCode = 3;
pub const USCRIPT_AVESTAN: UScriptCode = 117;
pub const USCRIPT_BALINESE: UScriptCode = 62;
pub const USCRIPT_BAMUM: UScriptCode = 130;
pub const USCRIPT_BASSA_VAH: UScriptCode = 134;
pub const USCRIPT_BATAK: UScriptCode = 63;
pub const USCRIPT_BENGALI: UScriptCode = 4;
pub const USCRIPT_BHAIKSUKI: UScriptCode = 168;
pub const USCRIPT_BLISSYMBOLS: UScriptCode = 64;
pub const USCRIPT_BOOK_PAHLAVI: UScriptCode = 124;
pub const USCRIPT_BOPOMOFO: UScriptCode = 5;
pub const USCRIPT_BRAHMI: UScriptCode = 65;
pub const USCRIPT_BRAILLE: UScriptCode = 46;
pub const USCRIPT_BUGINESE: UScriptCode = 55;
pub const USCRIPT_BUHID: UScriptCode = 44;
pub const USCRIPT_CANADIAN_ABORIGINAL: UScriptCode = 40;
pub const USCRIPT_CARIAN: UScriptCode = 104;
pub const USCRIPT_CAUCASIAN_ALBANIAN: UScriptCode = 159;
pub const USCRIPT_CHAKMA: UScriptCode = 118;
pub const USCRIPT_CHAM: UScriptCode = 66;
pub const USCRIPT_CHEROKEE: UScriptCode = 6;
pub const USCRIPT_CHORASMIAN: UScriptCode = 189;
pub const USCRIPT_CIRTH: UScriptCode = 67;
pub const USCRIPT_COMMON: UScriptCode = 0;
pub const USCRIPT_COPTIC: UScriptCode = 7;
pub const USCRIPT_CUNEIFORM: UScriptCode = 101;
pub const USCRIPT_CYPRIOT: UScriptCode = 47;
pub const USCRIPT_CYRILLIC: UScriptCode = 8;
pub const USCRIPT_DEMOTIC_EGYPTIAN: UScriptCode = 69;
pub const USCRIPT_DESERET: UScriptCode = 9;
pub const USCRIPT_DEVANAGARI: UScriptCode = 10;
pub const USCRIPT_DIVES_AKURU: UScriptCode = 190;
pub const USCRIPT_DOGRA: UScriptCode = 178;
pub const USCRIPT_DUPLOYAN: UScriptCode = 135;
pub const USCRIPT_EASTERN_SYRIAC: UScriptCode = 97;
pub const USCRIPT_EGYPTIAN_HIEROGLYPHS: UScriptCode = 71;
pub const USCRIPT_ELBASAN: UScriptCode = 136;
pub const USCRIPT_ELYMAIC: UScriptCode = 185;
pub const USCRIPT_ESTRANGELO_SYRIAC: UScriptCode = 95;
pub const USCRIPT_ETHIOPIC: UScriptCode = 11;
pub const USCRIPT_GEORGIAN: UScriptCode = 12;
pub const USCRIPT_GLAGOLITIC: UScriptCode = 56;
pub const USCRIPT_GOTHIC: UScriptCode = 13;
pub const USCRIPT_GRANTHA: UScriptCode = 137;
pub const USCRIPT_GREEK: UScriptCode = 14;
pub const USCRIPT_GUJARATI: UScriptCode = 15;
pub const USCRIPT_GUNJALA_GONDI: UScriptCode = 179;
pub const USCRIPT_GURMUKHI: UScriptCode = 16;
pub const USCRIPT_HAN: UScriptCode = 17;
pub const USCRIPT_HANGUL: UScriptCode = 18;
pub const USCRIPT_HANIFI_ROHINGYA: UScriptCode = 182;
pub const USCRIPT_HANUNOO: UScriptCode = 43;
pub const USCRIPT_HAN_WITH_BOPOMOFO: UScriptCode = 172;
pub const USCRIPT_HARAPPAN_INDUS: UScriptCode = 77;
pub const USCRIPT_HATRAN: UScriptCode = 162;
pub const USCRIPT_HEBREW: UScriptCode = 19;
pub const USCRIPT_HIERATIC_EGYPTIAN: UScriptCode = 70;
pub const USCRIPT_HIRAGANA: UScriptCode = 20;
pub const USCRIPT_IMPERIAL_ARAMAIC: UScriptCode = 116;
pub const USCRIPT_INHERITED: UScriptCode = 1;
pub const USCRIPT_INSCRIPTIONAL_PAHLAVI: UScriptCode = 122;
pub const USCRIPT_INSCRIPTIONAL_PARTHIAN: UScriptCode = 125;
pub const USCRIPT_INVALID_CODE: UScriptCode = -1;
pub const USCRIPT_JAMO: UScriptCode = 173;
pub const USCRIPT_JAPANESE: UScriptCode = 105;
pub const USCRIPT_JAVANESE: UScriptCode = 78;
pub const USCRIPT_JURCHEN: UScriptCode = 148;
pub const USCRIPT_KAITHI: UScriptCode = 120;
pub const USCRIPT_KANNADA: UScriptCode = 21;
pub const USCRIPT_KATAKANA: UScriptCode = 22;
pub const USCRIPT_KATAKANA_OR_HIRAGANA: UScriptCode = 54;
pub const USCRIPT_KAYAH_LI: UScriptCode = 79;
pub const USCRIPT_KHAROSHTHI: UScriptCode = 57;
pub const USCRIPT_KHITAN_SMALL_SCRIPT: UScriptCode = 191;
pub const USCRIPT_KHMER: UScriptCode = 23;
pub const USCRIPT_KHOJKI: UScriptCode = 157;
pub const USCRIPT_KHUDAWADI: UScriptCode = 145;
pub const USCRIPT_KHUTSURI: UScriptCode = 72;
pub const USCRIPT_KOREAN: UScriptCode = 119;
pub const USCRIPT_KPELLE: UScriptCode = 138;
pub const USCRIPT_LANNA: UScriptCode = 106;
pub const USCRIPT_LAO: UScriptCode = 24;
pub const USCRIPT_LATIN: UScriptCode = 25;
pub const USCRIPT_LATIN_FRAKTUR: UScriptCode = 80;
pub const USCRIPT_LATIN_GAELIC: UScriptCode = 81;
pub const USCRIPT_LEPCHA: UScriptCode = 82;
pub const USCRIPT_LIMBU: UScriptCode = 48;
pub const USCRIPT_LINEAR_A: UScriptCode = 83;
pub const USCRIPT_LINEAR_B: UScriptCode = 49;
pub const USCRIPT_LISU: UScriptCode = 131;
pub const USCRIPT_LOMA: UScriptCode = 139;
pub const USCRIPT_LYCIAN: UScriptCode = 107;
pub const USCRIPT_LYDIAN: UScriptCode = 108;
pub const USCRIPT_MAHAJANI: UScriptCode = 160;
pub const USCRIPT_MAKASAR: UScriptCode = 180;
pub const USCRIPT_MALAYALAM: UScriptCode = 26;
pub const USCRIPT_MANDAEAN: UScriptCode = 84;
pub const USCRIPT_MANDAIC: UScriptCode = 84;
pub const USCRIPT_MANICHAEAN: UScriptCode = 121;
pub const USCRIPT_MARCHEN: UScriptCode = 169;
pub const USCRIPT_MASARAM_GONDI: UScriptCode = 175;
pub const USCRIPT_MATHEMATICAL_NOTATION: UScriptCode = 128;
pub const USCRIPT_MAYAN_HIEROGLYPHS: UScriptCode = 85;
pub const USCRIPT_MEDEFAIDRIN: UScriptCode = 181;
pub const USCRIPT_MEITEI_MAYEK: UScriptCode = 115;
pub const USCRIPT_MENDE: UScriptCode = 140;
pub const USCRIPT_MEROITIC: UScriptCode = 86;
pub const USCRIPT_MEROITIC_CURSIVE: UScriptCode = 141;
pub const USCRIPT_MEROITIC_HIEROGLYPHS: UScriptCode = 86;
pub const USCRIPT_MIAO: UScriptCode = 92;
pub const USCRIPT_MODI: UScriptCode = 163;
pub const USCRIPT_MONGOLIAN: UScriptCode = 27;
pub const USCRIPT_MOON: UScriptCode = 114;
pub const USCRIPT_MRO: UScriptCode = 149;
pub const USCRIPT_MULTANI: UScriptCode = 164;
pub const USCRIPT_MYANMAR: UScriptCode = 28;
pub const USCRIPT_NABATAEAN: UScriptCode = 143;
pub const USCRIPT_NAKHI_GEBA: UScriptCode = 132;
pub const USCRIPT_NANDINAGARI: UScriptCode = 187;
pub const USCRIPT_NEWA: UScriptCode = 170;
pub const USCRIPT_NEW_TAI_LUE: UScriptCode = 59;
pub const USCRIPT_NKO: UScriptCode = 87;
pub const USCRIPT_NUSHU: UScriptCode = 150;
pub const USCRIPT_NYIAKENG_PUACHUE_HMONG: UScriptCode = 186;
pub const USCRIPT_OGHAM: UScriptCode = 29;
pub const USCRIPT_OLD_CHURCH_SLAVONIC_CYRILLIC: UScriptCode = 68;
pub const USCRIPT_OLD_HUNGARIAN: UScriptCode = 76;
pub const USCRIPT_OLD_ITALIC: UScriptCode = 30;
pub const USCRIPT_OLD_NORTH_ARABIAN: UScriptCode = 142;
pub const USCRIPT_OLD_PERMIC: UScriptCode = 89;
pub const USCRIPT_OLD_PERSIAN: UScriptCode = 61;
pub const USCRIPT_OLD_SOGDIAN: UScriptCode = 184;
pub const USCRIPT_OLD_SOUTH_ARABIAN: UScriptCode = 133;
pub const USCRIPT_OL_CHIKI: UScriptCode = 109;
pub const USCRIPT_ORIYA: UScriptCode = 31;
pub const USCRIPT_ORKHON: UScriptCode = 88;
pub const USCRIPT_OSAGE: UScriptCode = 171;
pub const USCRIPT_OSMANYA: UScriptCode = 50;
pub const USCRIPT_PAHAWH_HMONG: UScriptCode = 75;
pub const USCRIPT_PALMYRENE: UScriptCode = 144;
pub const USCRIPT_PAU_CIN_HAU: UScriptCode = 165;
pub const USCRIPT_PHAGS_PA: UScriptCode = 90;
pub const USCRIPT_PHOENICIAN: UScriptCode = 91;
pub const USCRIPT_PHONETIC_POLLARD: UScriptCode = 92;
pub const USCRIPT_PSALTER_PAHLAVI: UScriptCode = 123;
pub const USCRIPT_REJANG: UScriptCode = 110;
pub const USCRIPT_RONGORONGO: UScriptCode = 93;
pub const USCRIPT_RUNIC: UScriptCode = 32;
pub const USCRIPT_SAMARITAN: UScriptCode = 126;
pub const USCRIPT_SARATI: UScriptCode = 94;
pub const USCRIPT_SAURASHTRA: UScriptCode = 111;
pub const USCRIPT_SHARADA: UScriptCode = 151;
pub const USCRIPT_SHAVIAN: UScriptCode = 51;
pub const USCRIPT_SIDDHAM: UScriptCode = 166;
pub const USCRIPT_SIGN_WRITING: UScriptCode = 112;
pub const USCRIPT_SIMPLIFIED_HAN: UScriptCode = 73;
pub const USCRIPT_SINDHI: UScriptCode = 145;
pub const USCRIPT_SINHALA: UScriptCode = 33;
pub const USCRIPT_SOGDIAN: UScriptCode = 183;
pub const USCRIPT_SORA_SOMPENG: UScriptCode = 152;
pub const USCRIPT_SOYOMBO: UScriptCode = 176;
pub const USCRIPT_SUNDANESE: UScriptCode = 113;
pub const USCRIPT_SYLOTI_NAGRI: UScriptCode = 58;
pub const USCRIPT_SYMBOLS: UScriptCode = 129;
pub const USCRIPT_SYMBOLS_EMOJI: UScriptCode = 174;
pub const USCRIPT_SYRIAC: UScriptCode = 34;
pub const USCRIPT_TAGALOG: UScriptCode = 42;
pub const USCRIPT_TAGBANWA: UScriptCode = 45;
pub const USCRIPT_TAI_LE: UScriptCode = 52;
pub const USCRIPT_TAI_VIET: UScriptCode = 127;
pub const USCRIPT_TAKRI: UScriptCode = 153;
pub const USCRIPT_TAMIL: UScriptCode = 35;
pub const USCRIPT_TANGUT: UScriptCode = 154;
pub const USCRIPT_TELUGU: UScriptCode = 36;
pub const USCRIPT_TENGWAR: UScriptCode = 98;
pub const USCRIPT_THAANA: UScriptCode = 37;
pub const USCRIPT_THAI: UScriptCode = 38;
pub const USCRIPT_TIBETAN: UScriptCode = 39;
pub const USCRIPT_TIFINAGH: UScriptCode = 60;
pub const USCRIPT_TIRHUTA: UScriptCode = 158;
pub const USCRIPT_TRADITIONAL_HAN: UScriptCode = 74;
pub const USCRIPT_UCAS: UScriptCode = 40;
pub const USCRIPT_UGARITIC: UScriptCode = 53;
pub const USCRIPT_UNKNOWN: UScriptCode = 103;
pub const USCRIPT_UNWRITTEN_LANGUAGES: UScriptCode = 102;
pub const USCRIPT_USAGE_ASPIRATIONAL: UScriptUsage = 4;
pub const USCRIPT_USAGE_EXCLUDED: UScriptUsage = 2;
pub const USCRIPT_USAGE_LIMITED_USE: UScriptUsage = 3;
pub const USCRIPT_USAGE_NOT_ENCODED: UScriptUsage = 0;
pub const USCRIPT_USAGE_RECOMMENDED: UScriptUsage = 5;
pub const USCRIPT_USAGE_UNKNOWN: UScriptUsage = 1;
pub const USCRIPT_VAI: UScriptCode = 99;
pub const USCRIPT_VISIBLE_SPEECH: UScriptCode = 100;
pub const USCRIPT_WANCHO: UScriptCode = 188;
pub const USCRIPT_WARANG_CITI: UScriptCode = 146;
pub const USCRIPT_WESTERN_SYRIAC: UScriptCode = 96;
pub const USCRIPT_WOLEAI: UScriptCode = 155;
pub const USCRIPT_YEZIDI: UScriptCode = 192;
pub const USCRIPT_YI: UScriptCode = 41;
pub const USCRIPT_ZANABAZAR_SQUARE: UScriptCode = 177;
pub const USEARCH_ANY_BASE_WEIGHT_IS_WILDCARD: USearchAttributeValue = 4;
pub const USEARCH_DEFAULT: USearchAttributeValue = -1;
pub const USEARCH_DONE: i32 = -1;
pub const USEARCH_ELEMENT_COMPARISON: USearchAttribute = 2;
pub const USEARCH_OFF: USearchAttributeValue = 0;
pub const USEARCH_ON: USearchAttributeValue = 1;
pub const USEARCH_OVERLAP: USearchAttribute = 0;
pub const USEARCH_PATTERN_BASE_WEIGHT_IS_WILDCARD: USearchAttributeValue = 3;
pub const USEARCH_STANDARD_ELEMENT_COMPARISON: USearchAttributeValue = 2;
pub const USET_ADD_CASE_MAPPINGS: i32 = 4;
pub const USET_CASE_INSENSITIVE: i32 = 2;
pub const USET_IGNORE_SPACE: i32 = 1;
pub const USET_SERIALIZED_STATIC_ARRAY_CAPACITY: i32 = 8;
pub const USET_SPAN_CONTAINED: USetSpanCondition = 1;
pub const USET_SPAN_NOT_CONTAINED: USetSpanCondition = 0;
pub const USET_SPAN_SIMPLE: USetSpanCondition = 2;
pub const USPOOF_ALL_CHECKS: USpoofChecks = 65535;
pub const USPOOF_ASCII: URestrictionLevel = 268435456;
pub const USPOOF_AUX_INFO: USpoofChecks = 1073741824;
pub const USPOOF_CHAR_LIMIT: USpoofChecks = 64;
pub const USPOOF_CONFUSABLE: USpoofChecks = 7;
pub const USPOOF_HIDDEN_OVERLAY: USpoofChecks = 256;
pub const USPOOF_HIGHLY_RESTRICTIVE: URestrictionLevel = 805306368;
pub const USPOOF_INVISIBLE: USpoofChecks = 32;
pub const USPOOF_MINIMALLY_RESTRICTIVE: URestrictionLevel = 1342177280;
pub const USPOOF_MIXED_NUMBERS: USpoofChecks = 128;
pub const USPOOF_MIXED_SCRIPT_CONFUSABLE: USpoofChecks = 2;
pub const USPOOF_MODERATELY_RESTRICTIVE: URestrictionLevel = 1073741824;
pub const USPOOF_RESTRICTION_LEVEL: USpoofChecks = 16;
pub const USPOOF_RESTRICTION_LEVEL_MASK: URestrictionLevel = 2130706432;
pub const USPOOF_SINGLE_SCRIPT_CONFUSABLE: USpoofChecks = 1;
pub const USPOOF_SINGLE_SCRIPT_RESTRICTIVE: URestrictionLevel = 536870912;
pub const USPOOF_UNRESTRICTIVE: URestrictionLevel = 1610612736;
pub const USPOOF_WHOLE_SCRIPT_CONFUSABLE: USpoofChecks = 4;
pub const USPREP_ALLOW_UNASSIGNED: u32 = 1;
pub const USPREP_DEFAULT: u32 = 0;
pub const USPREP_RFC3491_NAMEPREP: UStringPrepProfileType = 0;
pub const USPREP_RFC3530_NFS4_CIS_PREP: UStringPrepProfileType = 3;
pub const USPREP_RFC3530_NFS4_CS_PREP: UStringPrepProfileType = 1;
pub const USPREP_RFC3530_NFS4_CS_PREP_CI: UStringPrepProfileType = 2;
pub const USPREP_RFC3530_NFS4_MIXED_PREP_PREFIX: UStringPrepProfileType = 4;
pub const USPREP_RFC3530_NFS4_MIXED_PREP_SUFFIX: UStringPrepProfileType = 5;
pub const USPREP_RFC3722_ISCSI: UStringPrepProfileType = 6;
pub const USPREP_RFC3920_NODEPREP: UStringPrepProfileType = 7;
pub const USPREP_RFC3920_RESOURCEPREP: UStringPrepProfileType = 8;
pub const USPREP_RFC4011_MIB: UStringPrepProfileType = 9;
pub const USPREP_RFC4013_SASLPREP: UStringPrepProfileType = 10;
pub const USPREP_RFC4505_TRACE: UStringPrepProfileType = 11;
pub const USPREP_RFC4518_LDAP: UStringPrepProfileType = 12;
pub const USPREP_RFC4518_LDAP_CI: UStringPrepProfileType = 13;
pub const USTRINGTRIE_FINAL_VALUE: UStringTrieResult = 2;
pub const USTRINGTRIE_INTERMEDIATE_VALUE: UStringTrieResult = 3;
pub const USTRINGTRIE_NO_MATCH: UStringTrieResult = 0;
pub const USTRINGTRIE_NO_VALUE: UStringTrieResult = 1;
pub type UScriptCode = i32;
pub type UScriptUsage = i32;
pub type USearchAttribute = i32;
pub type USearchAttributeValue = i32;
pub type USentenceBreak = i32;
pub type USentenceBreakTag = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USerializedSet {
    pub array: *const u16,
    pub bmpLength: i32,
    pub length: i32,
    pub staticArray: [u16; 8],
}
impl Default for USerializedSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USet(pub u8);
pub type USetSpanCondition = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USpoofCheckResult(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USpoofChecker(pub u8);
pub type USpoofChecks = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UStringPrepProfile(pub u8);
pub type UStringPrepProfileType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UStringSearch(pub u8);
pub type UStringTrieResult = i32;
pub type USystemTimeZoneType = i32;
pub const UTEXT_MAGIC: i32 = 878368812;
pub const UTEXT_PROVIDER_HAS_META_DATA: i32 = 4;
pub const UTEXT_PROVIDER_LENGTH_IS_EXPENSIVE: i32 = 1;
pub const UTEXT_PROVIDER_OWNS_TEXT: i32 = 5;
pub const UTEXT_PROVIDER_STABLE_CHUNKS: i32 = 2;
pub const UTEXT_PROVIDER_WRITABLE: i32 = 3;
pub const UTRACE_COLLATION_START: UTraceFunctionNumber = 8192;
pub const UTRACE_CONVERSION_START: UTraceFunctionNumber = 4096;
pub const UTRACE_ERROR: UTraceLevel = 0;
pub const UTRACE_FUNCTION_START: UTraceFunctionNumber = 0;
pub const UTRACE_INFO: UTraceLevel = 7;
pub const UTRACE_OFF: UTraceLevel = -1;
pub const UTRACE_OPEN_CLOSE: UTraceLevel = 5;
pub const UTRACE_UCNV_CLONE: UTraceFunctionNumber = 4099;
pub const UTRACE_UCNV_CLOSE: UTraceFunctionNumber = 4100;
pub const UTRACE_UCNV_FLUSH_CACHE: UTraceFunctionNumber = 4101;
pub const UTRACE_UCNV_LOAD: UTraceFunctionNumber = 4102;
pub const UTRACE_UCNV_OPEN: UTraceFunctionNumber = 4096;
pub const UTRACE_UCNV_OPEN_ALGORITHMIC: UTraceFunctionNumber = 4098;
pub const UTRACE_UCNV_OPEN_PACKAGE: UTraceFunctionNumber = 4097;
pub const UTRACE_UCNV_UNLOAD: UTraceFunctionNumber = 4103;
pub const UTRACE_UCOL_CLOSE: UTraceFunctionNumber = 8193;
pub const UTRACE_UCOL_GETLOCALE: UTraceFunctionNumber = 8196;
pub const UTRACE_UCOL_GET_SORTKEY: UTraceFunctionNumber = 8195;
pub const UTRACE_UCOL_NEXTSORTKEYPART: UTraceFunctionNumber = 8197;
pub const UTRACE_UCOL_OPEN: UTraceFunctionNumber = 8192;
pub const UTRACE_UCOL_OPEN_FROM_SHORT_STRING: UTraceFunctionNumber = 8199;
pub const UTRACE_UCOL_STRCOLL: UTraceFunctionNumber = 8194;
pub const UTRACE_UCOL_STRCOLLITER: UTraceFunctionNumber = 8198;
pub const UTRACE_UCOL_STRCOLLUTF8: UTraceFunctionNumber = 8200;
pub const UTRACE_UDATA_BUNDLE: UTraceFunctionNumber = 12289;
pub const UTRACE_UDATA_DATA_FILE: UTraceFunctionNumber = 12290;
pub const UTRACE_UDATA_RESOURCE: UTraceFunctionNumber = 12288;
pub const UTRACE_UDATA_RES_FILE: UTraceFunctionNumber = 12291;
pub const UTRACE_UDATA_START: UTraceFunctionNumber = 12288;
pub const UTRACE_U_CLEANUP: UTraceFunctionNumber = 1;
pub const UTRACE_U_INIT: UTraceFunctionNumber = 0;
pub const UTRACE_VERBOSE: UTraceLevel = 9;
pub const UTRACE_WARNING: UTraceLevel = 3;
pub const UTRANS_FORWARD: UTransDirection = 0;
pub const UTRANS_REVERSE: UTransDirection = 1;
pub const UTSV_EPOCH_OFFSET_VALUE: UTimeScaleValue = 1;
pub const UTSV_FROM_MAX_VALUE: UTimeScaleValue = 3;
pub const UTSV_FROM_MIN_VALUE: UTimeScaleValue = 2;
pub const UTSV_TO_MAX_VALUE: UTimeScaleValue = 5;
pub const UTSV_TO_MIN_VALUE: UTimeScaleValue = 4;
pub const UTSV_UNITS_VALUE: UTimeScaleValue = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UText {
    pub magic: u32,
    pub flags: i32,
    pub providerProperties: i32,
    pub sizeOfStruct: i32,
    pub chunkNativeLimit: i64,
    pub extraSize: i32,
    pub nativeIndexingLimit: i32,
    pub chunkNativeStart: i64,
    pub chunkOffset: i32,
    pub chunkLength: i32,
    pub chunkContents: *const UChar,
    pub pFuncs: *const UTextFuncs,
    pub pExtra: *mut core::ffi::c_void,
    pub context: *const core::ffi::c_void,
    pub p: *const core::ffi::c_void,
    pub q: *const core::ffi::c_void,
    pub r: *const core::ffi::c_void,
    pub privP: *mut core::ffi::c_void,
    pub a: i64,
    pub b: i32,
    pub c: i32,
    pub privA: i64,
    pub privB: i32,
    pub privC: i32,
}
impl Default for UText {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UTextAccess = Option<unsafe extern "C" fn(ut: *mut UText, nativeindex: i64, forward: UBool) -> UBool>;
pub type UTextClone = Option<unsafe extern "C" fn(dest: *mut UText, src: *const UText, deep: UBool, status: *mut UErrorCode) -> *mut UText>;
pub type UTextClose = Option<unsafe extern "C" fn(ut: *mut UText)>;
pub type UTextCopy = Option<unsafe extern "C" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, nativedest: i64, r#move: UBool, status: *mut UErrorCode)>;
pub type UTextExtract = Option<unsafe extern "C" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, dest: *mut UChar, destcapacity: i32, status: *mut UErrorCode) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct UTextFuncs {
    pub tableSize: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: i32,
    pub clone: UTextClone,
    pub nativeLength: UTextNativeLength,
    pub access: UTextAccess,
    pub extract: UTextExtract,
    pub replace: UTextReplace,
    pub copy: UTextCopy,
    pub mapOffsetToNative: UTextMapOffsetToNative,
    pub mapNativeIndexToUTF16: UTextMapNativeIndexToUTF16,
    pub close: UTextClose,
    pub spare1: UTextClose,
    pub spare2: UTextClose,
    pub spare3: UTextClose,
}
pub type UTextMapNativeIndexToUTF16 = Option<unsafe extern "C" fn(ut: *const UText, nativeindex: i64) -> i32>;
pub type UTextMapOffsetToNative = Option<unsafe extern "C" fn(ut: *const UText) -> i64>;
pub type UTextNativeLength = Option<unsafe extern "C" fn(ut: *mut UText) -> i64>;
pub type UTextReplace = Option<unsafe extern "C" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, replacementtext: *const UChar, replacmentlength: i32, status: *mut UErrorCode) -> i32>;
pub type UTimeScaleValue = i32;
pub type UTimeZoneLocalOption = i32;
pub type UTimeZoneTransitionType = i32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type UTraceData = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, fnnumber: i32, level: i32, fmt: *const i8, args: *mut i8)>;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
pub type UTraceData = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, fnnumber: i32, level: i32, fmt: *const i8, args: super::vadefs::va_list)>;
pub type UTraceEntry = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, fnnumber: i32)>;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type UTraceExit = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, fnnumber: i32, fmt: *const i8, args: *mut i8)>;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
pub type UTraceExit = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, fnnumber: i32, fmt: *const i8, args: super::vadefs::va_list)>;
pub type UTraceFunctionNumber = i32;
pub type UTraceLevel = i32;
pub type UTransDirection = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UTransPosition {
    pub contextStart: i32,
    pub contextLimit: i32,
    pub start: i32,
    pub limit: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UTransliterator(pub *mut core::ffi::c_void);
impl Default for UTransliterator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UVersionInfo = [u8; 4];
pub type UVerticalOrientation = i32;
pub type UWordBreak = i32;
pub type UWordBreakValues = i32;
pub const U_AMBIGUOUS_ALIAS_WARNING: UErrorCode = -122;
pub const U_ARABIC_NUMBER: UCharDirection = 5;
pub const U_ARGUMENT_TYPE_MISMATCH: UErrorCode = 65804;
pub const U_ASCII_FAMILY: u32 = 0;
pub const U_BAD_VARIABLE_DEFINITION: UErrorCode = 65536;
pub const U_BLOCK_SEPARATOR: UCharDirection = 7;
pub const U_BOUNDARY_NEUTRAL: UCharDirection = 18;
pub const U_BPT_CLOSE: UBidiPairedBracketType = 2;
pub const U_BPT_NONE: UBidiPairedBracketType = 0;
pub const U_BPT_OPEN: UBidiPairedBracketType = 1;
pub const U_BRK_ASSIGN_ERROR: UErrorCode = 66053;
pub const U_BRK_ERROR_START: UErrorCode = 66048;
pub const U_BRK_HEX_DIGITS_EXPECTED: UErrorCode = 66049;
pub const U_BRK_INIT_ERROR: UErrorCode = 66058;
pub const U_BRK_INTERNAL_ERROR: UErrorCode = 66048;
pub const U_BRK_MALFORMED_RULE_TAG: UErrorCode = 66061;
pub const U_BRK_MISMATCHED_PAREN: UErrorCode = 66055;
pub const U_BRK_NEW_LINE_IN_QUOTED_STRING: UErrorCode = 66056;
pub const U_BRK_RULE_EMPTY_SET: UErrorCode = 66059;
pub const U_BRK_RULE_SYNTAX: UErrorCode = 66051;
pub const U_BRK_SEMICOLON_EXPECTED: UErrorCode = 66050;
pub const U_BRK_UNCLOSED_SET: UErrorCode = 66052;
pub const U_BRK_UNDEFINED_VARIABLE: UErrorCode = 66057;
pub const U_BRK_UNRECOGNIZED_OPTION: UErrorCode = 66060;
pub const U_BRK_VARIABLE_REDFINITION: UErrorCode = 66054;
pub const U_BUFFER_OVERFLOW_ERROR: UErrorCode = 15;
pub const U_CE_NOT_FOUND_ERROR: UErrorCode = 21;
pub const U_CHAR16_IS_TYPEDEF: u32 = 0;
pub const U_CHARSET_FAMILY: u32 = 0;
pub const U_CHARSET_IS_UTF8: u32 = 0;
pub const U_CHAR_CATEGORY_COUNT: UCharCategory = 30;
pub const U_CHAR_NAME_ALIAS: UCharNameChoice = 3;
pub const U_CHECK_DYLOAD: u32 = 1;
pub const U_COLLATOR_VERSION_MISMATCH: UErrorCode = 28;
pub const U_COMBINING_SPACING_MARK: UCharCategory = 8;
pub const U_COMMON_NUMBER_SEPARATOR: UCharDirection = 6;
pub const U_COMPARE_CODE_POINT_ORDER: u32 = 32768;
pub const U_COMPARE_IGNORE_CASE: u32 = 65536;
pub const U_CONNECTOR_PUNCTUATION: UCharCategory = 22;
pub const U_CONTROL_CHAR: UCharCategory = 15;
pub const U_COPYRIGHT_STRING_LENGTH: u32 = 128;
pub const U_CPLUSPLUS_VERSION: u32 = 14;
pub const U_CURRENCY_SYMBOL: UCharCategory = 25;
pub const U_DASH_PUNCTUATION: UCharCategory = 19;
pub const U_DEBUG: u32 = 0;
pub const U_DECIMAL_DIGIT_NUMBER: UCharCategory = 9;
pub const U_DECIMAL_NUMBER_SYNTAX_ERROR: UErrorCode = 65808;
pub const U_DEFAULT_KEYWORD_MISSING: UErrorCode = 65807;
pub const U_DEFAULT_SHOW_DRAFT: u32 = 0;
pub const U_DEFINE_FALSE_AND_TRUE: u32 = 0;
pub const U_DIFFERENT_UCA_VERSION: UErrorCode = -121;
pub const U_DIR_NON_SPACING_MARK: UCharDirection = 17;
pub const U_DISABLE_RENAMING: u32 = 1;
pub const U_DT_CANONICAL: UDecompositionType = 1;
pub const U_DT_CIRCLE: UDecompositionType = 3;
pub const U_DT_COMPAT: UDecompositionType = 2;
pub const U_DT_FINAL: UDecompositionType = 4;
pub const U_DT_FONT: UDecompositionType = 5;
pub const U_DT_FRACTION: UDecompositionType = 6;
pub const U_DT_INITIAL: UDecompositionType = 7;
pub const U_DT_ISOLATED: UDecompositionType = 8;
pub const U_DT_MEDIAL: UDecompositionType = 9;
pub const U_DT_NARROW: UDecompositionType = 10;
pub const U_DT_NOBREAK: UDecompositionType = 11;
pub const U_DT_NONE: UDecompositionType = 0;
pub const U_DT_SMALL: UDecompositionType = 12;
pub const U_DT_SQUARE: UDecompositionType = 13;
pub const U_DT_SUB: UDecompositionType = 14;
pub const U_DT_SUPER: UDecompositionType = 15;
pub const U_DT_VERTICAL: UDecompositionType = 16;
pub const U_DT_WIDE: UDecompositionType = 17;
pub const U_DUPLICATE_KEYWORD: UErrorCode = 65805;
pub const U_EA_AMBIGUOUS: UEastAsianWidth = 1;
pub const U_EA_FULLWIDTH: UEastAsianWidth = 3;
pub const U_EA_HALFWIDTH: UEastAsianWidth = 2;
pub const U_EA_NARROW: UEastAsianWidth = 4;
pub const U_EA_NEUTRAL: UEastAsianWidth = 0;
pub const U_EA_WIDE: UEastAsianWidth = 5;
pub const U_EBCDIC_FAMILY: u32 = 1;
pub const U_EDITS_NO_RESET: u32 = 8192;
pub const U_ENABLE_DYLOAD: u32 = 1;
pub const U_ENABLE_TRACING: u32 = 0;
pub const U_ENCLOSING_MARK: UCharCategory = 7;
pub const U_END_PUNCTUATION: UCharCategory = 21;
pub const U_ENUM_OUT_OF_SYNC_ERROR: UErrorCode = 25;
pub const U_ERROR_WARNING_START: UErrorCode = -128;
pub const U_EUROPEAN_NUMBER: UCharDirection = 2;
pub const U_EUROPEAN_NUMBER_SEPARATOR: UCharDirection = 3;
pub const U_EUROPEAN_NUMBER_TERMINATOR: UCharDirection = 4;
pub const U_EXTENDED_CHAR_NAME: UCharNameChoice = 2;
pub const U_FILE_ACCESS_ERROR: UErrorCode = 4;
pub const U_FINAL_PUNCTUATION: UCharCategory = 29;
pub const U_FIRST_STRONG_ISOLATE: UCharDirection = 19;
pub const U_FMT_PARSE_ERROR_START: UErrorCode = 65792;
pub const U_FOLD_CASE_DEFAULT: u32 = 0;
pub const U_FOLD_CASE_EXCLUDE_SPECIAL_I: u32 = 1;
pub const U_FORMAT_CHAR: UCharCategory = 16;
pub const U_FORMAT_INEXACT_ERROR: UErrorCode = 65809;
pub const U_GCB_CONTROL: UGraphemeClusterBreak = 1;
pub const U_GCB_CR: UGraphemeClusterBreak = 2;
pub const U_GCB_EXTEND: UGraphemeClusterBreak = 3;
pub const U_GCB_E_BASE: UGraphemeClusterBreak = 13;
pub const U_GCB_E_BASE_GAZ: UGraphemeClusterBreak = 14;
pub const U_GCB_E_MODIFIER: UGraphemeClusterBreak = 15;
pub const U_GCB_GLUE_AFTER_ZWJ: UGraphemeClusterBreak = 16;
pub const U_GCB_L: UGraphemeClusterBreak = 4;
pub const U_GCB_LF: UGraphemeClusterBreak = 5;
pub const U_GCB_LV: UGraphemeClusterBreak = 6;
pub const U_GCB_LVT: UGraphemeClusterBreak = 7;
pub const U_GCB_OTHER: UGraphemeClusterBreak = 0;
pub const U_GCB_PREPEND: UGraphemeClusterBreak = 11;
pub const U_GCB_REGIONAL_INDICATOR: UGraphemeClusterBreak = 12;
pub const U_GCB_SPACING_MARK: UGraphemeClusterBreak = 10;
pub const U_GCB_T: UGraphemeClusterBreak = 8;
pub const U_GCB_V: UGraphemeClusterBreak = 9;
pub const U_GCB_ZWJ: UGraphemeClusterBreak = 17;
pub const U_GCC_MAJOR_MINOR: u32 = 0;
pub const U_GC_CC_MASK: u32 = 32768;
pub const U_GC_CF_MASK: u32 = 65536;
pub const U_GC_CN_MASK: u32 = 1;
pub const U_GC_CO_MASK: u32 = 131072;
pub const U_GC_CS_MASK: u32 = 262144;
pub const U_GC_C_MASK: u32 = 491521;
pub const U_GC_LC_MASK: u32 = 14;
pub const U_GC_LL_MASK: u32 = 4;
pub const U_GC_LM_MASK: u32 = 16;
pub const U_GC_LO_MASK: u32 = 32;
pub const U_GC_LT_MASK: u32 = 8;
pub const U_GC_LU_MASK: u32 = 2;
pub const U_GC_L_MASK: u32 = 62;
pub const U_GC_MC_MASK: u32 = 256;
pub const U_GC_ME_MASK: u32 = 128;
pub const U_GC_MN_MASK: u32 = 64;
pub const U_GC_M_MASK: u32 = 448;
pub const U_GC_ND_MASK: u32 = 512;
pub const U_GC_NL_MASK: u32 = 1024;
pub const U_GC_NO_MASK: u32 = 2048;
pub const U_GC_N_MASK: u32 = 3584;
pub const U_GC_PC_MASK: u32 = 4194304;
pub const U_GC_PD_MASK: u32 = 524288;
pub const U_GC_PE_MASK: u32 = 2097152;
pub const U_GC_PF_MASK: u32 = 536870912;
pub const U_GC_PI_MASK: u32 = 268435456;
pub const U_GC_PO_MASK: u32 = 8388608;
pub const U_GC_PS_MASK: u32 = 1048576;
pub const U_GC_P_MASK: u32 = 821559296;
pub const U_GC_SC_MASK: u32 = 33554432;
pub const U_GC_SK_MASK: u32 = 67108864;
pub const U_GC_SM_MASK: u32 = 16777216;
pub const U_GC_SO_MASK: u32 = 134217728;
pub const U_GC_S_MASK: u32 = 251658240;
pub const U_GC_ZL_MASK: u32 = 8192;
pub const U_GC_ZP_MASK: u32 = 16384;
pub const U_GC_ZS_MASK: u32 = 4096;
pub const U_GC_Z_MASK: u32 = 28672;
pub const U_GENERAL_OTHER_TYPES: UCharCategory = 0;
pub const U_HAVE_CHAR16_T: u32 = 1;
pub const U_HAVE_DEBUG_LOCATION_NEW: u32 = 1;
pub const U_HAVE_INTTYPES_H: u32 = 1;
pub const U_HAVE_PLACEMENT_NEW: u32 = 1;
pub const U_HAVE_STDINT_H: u32 = 1;
pub const U_HAVE_WCHAR_H: u32 = 1;
pub const U_HAVE_WCSCPY: u32 = 1;
pub const U_HIDE_DEPRECATED_API: u32 = 1;
pub const U_HIDE_DRAFT_API: u32 = 1;
pub const U_HIDE_INTERNAL_API: u32 = 1;
pub const U_HIDE_OBSOLETE_API: u32 = 1;
pub const U_HIDE_OBSOLETE_UTF_OLD_H: u32 = 0;
pub const U_HST_LEADING_JAMO: UHangulSyllableType = 1;
pub const U_HST_LVT_SYLLABLE: UHangulSyllableType = 5;
pub const U_HST_LV_SYLLABLE: UHangulSyllableType = 4;
pub const U_HST_NOT_APPLICABLE: UHangulSyllableType = 0;
pub const U_HST_TRAILING_JAMO: UHangulSyllableType = 3;
pub const U_HST_VOWEL_JAMO: UHangulSyllableType = 2;
pub const U_ICU_DATA_KEY: windows_core::PCSTR = windows_core::s!("DataVersion");
pub const U_ICU_VERSION_BUNDLE: windows_core::PCSTR = windows_core::s!("icuver");
pub const U_IDNA_ACE_PREFIX_ERROR: UErrorCode = 66564;
pub const U_IDNA_CHECK_BIDI_ERROR: UErrorCode = 66562;
pub const U_IDNA_DOMAIN_NAME_TOO_LONG_ERROR: UErrorCode = 66568;
pub const U_IDNA_ERROR_START: UErrorCode = 66560;
pub const U_IDNA_LABEL_TOO_LONG_ERROR: UErrorCode = 66566;
pub const U_IDNA_PROHIBITED_ERROR: UErrorCode = 66560;
pub const U_IDNA_STD3_ASCII_RULES_ERROR: UErrorCode = 66563;
pub const U_IDNA_UNASSIGNED_ERROR: UErrorCode = 66561;
pub const U_IDNA_VERIFICATION_ERROR: UErrorCode = 66565;
pub const U_IDNA_ZERO_LENGTH_LABEL_ERROR: UErrorCode = 66567;
pub const U_ILLEGAL_ARGUMENT_ERROR: UErrorCode = 1;
pub const U_ILLEGAL_CHARACTER: UErrorCode = 65567;
pub const U_ILLEGAL_CHAR_FOUND: UErrorCode = 12;
pub const U_ILLEGAL_CHAR_IN_SEGMENT: UErrorCode = 65564;
pub const U_ILLEGAL_ESCAPE_SEQUENCE: UErrorCode = 18;
pub const U_ILLEGAL_PAD_POSITION: UErrorCode = 65800;
pub const U_INDEX_OUTOFBOUNDS_ERROR: UErrorCode = 8;
pub const U_INITIAL_PUNCTUATION: UCharCategory = 28;
pub const U_INPC_BOTTOM: UIndicPositionalCategory = 1;
pub const U_INPC_BOTTOM_AND_LEFT: UIndicPositionalCategory = 2;
pub const U_INPC_BOTTOM_AND_RIGHT: UIndicPositionalCategory = 3;
pub const U_INPC_LEFT: UIndicPositionalCategory = 4;
pub const U_INPC_LEFT_AND_RIGHT: UIndicPositionalCategory = 5;
pub const U_INPC_NA: UIndicPositionalCategory = 0;
pub const U_INPC_OVERSTRUCK: UIndicPositionalCategory = 6;
pub const U_INPC_RIGHT: UIndicPositionalCategory = 7;
pub const U_INPC_TOP: UIndicPositionalCategory = 8;
pub const U_INPC_TOP_AND_BOTTOM: UIndicPositionalCategory = 9;
pub const U_INPC_TOP_AND_BOTTOM_AND_LEFT: UIndicPositionalCategory = 15;
pub const U_INPC_TOP_AND_BOTTOM_AND_RIGHT: UIndicPositionalCategory = 10;
pub const U_INPC_TOP_AND_LEFT: UIndicPositionalCategory = 11;
pub const U_INPC_TOP_AND_LEFT_AND_RIGHT: UIndicPositionalCategory = 12;
pub const U_INPC_TOP_AND_RIGHT: UIndicPositionalCategory = 13;
pub const U_INPC_VISUAL_ORDER_LEFT: UIndicPositionalCategory = 14;
pub const U_INPUT_TOO_LONG_ERROR: UErrorCode = 31;
pub const U_INSC_AVAGRAHA: UIndicSyllabicCategory = 1;
pub const U_INSC_BINDU: UIndicSyllabicCategory = 2;
pub const U_INSC_BRAHMI_JOINING_NUMBER: UIndicSyllabicCategory = 3;
pub const U_INSC_CANTILLATION_MARK: UIndicSyllabicCategory = 4;
pub const U_INSC_CONSONANT: UIndicSyllabicCategory = 5;
pub const U_INSC_CONSONANT_DEAD: UIndicSyllabicCategory = 6;
pub const U_INSC_CONSONANT_FINAL: UIndicSyllabicCategory = 7;
pub const U_INSC_CONSONANT_HEAD_LETTER: UIndicSyllabicCategory = 8;
pub const U_INSC_CONSONANT_INITIAL_POSTFIXED: UIndicSyllabicCategory = 9;
pub const U_INSC_CONSONANT_KILLER: UIndicSyllabicCategory = 10;
pub const U_INSC_CONSONANT_MEDIAL: UIndicSyllabicCategory = 11;
pub const U_INSC_CONSONANT_PLACEHOLDER: UIndicSyllabicCategory = 12;
pub const U_INSC_CONSONANT_PRECEDING_REPHA: UIndicSyllabicCategory = 13;
pub const U_INSC_CONSONANT_PREFIXED: UIndicSyllabicCategory = 14;
pub const U_INSC_CONSONANT_SUBJOINED: UIndicSyllabicCategory = 15;
pub const U_INSC_CONSONANT_SUCCEEDING_REPHA: UIndicSyllabicCategory = 16;
pub const U_INSC_CONSONANT_WITH_STACKER: UIndicSyllabicCategory = 17;
pub const U_INSC_GEMINATION_MARK: UIndicSyllabicCategory = 18;
pub const U_INSC_INVISIBLE_STACKER: UIndicSyllabicCategory = 19;
pub const U_INSC_JOINER: UIndicSyllabicCategory = 20;
pub const U_INSC_MODIFYING_LETTER: UIndicSyllabicCategory = 21;
pub const U_INSC_NON_JOINER: UIndicSyllabicCategory = 22;
pub const U_INSC_NUKTA: UIndicSyllabicCategory = 23;
pub const U_INSC_NUMBER: UIndicSyllabicCategory = 24;
pub const U_INSC_NUMBER_JOINER: UIndicSyllabicCategory = 25;
pub const U_INSC_OTHER: UIndicSyllabicCategory = 0;
pub const U_INSC_PURE_KILLER: UIndicSyllabicCategory = 26;
pub const U_INSC_REGISTER_SHIFTER: UIndicSyllabicCategory = 27;
pub const U_INSC_SYLLABLE_MODIFIER: UIndicSyllabicCategory = 28;
pub const U_INSC_TONE_LETTER: UIndicSyllabicCategory = 29;
pub const U_INSC_TONE_MARK: UIndicSyllabicCategory = 30;
pub const U_INSC_VIRAMA: UIndicSyllabicCategory = 31;
pub const U_INSC_VISARGA: UIndicSyllabicCategory = 32;
pub const U_INSC_VOWEL: UIndicSyllabicCategory = 33;
pub const U_INSC_VOWEL_DEPENDENT: UIndicSyllabicCategory = 34;
pub const U_INSC_VOWEL_INDEPENDENT: UIndicSyllabicCategory = 35;
pub const U_INT64_MAX: i32 = -1;
pub const U_INT64_MIN: u32 = 0;
pub const U_INTERNAL_PROGRAM_ERROR: UErrorCode = 5;
pub const U_INTERNAL_TRANSLITERATOR_ERROR: UErrorCode = 65568;
pub const U_INVALID_CHAR_FOUND: UErrorCode = 10;
pub const U_INVALID_FORMAT_ERROR: UErrorCode = 3;
pub const U_INVALID_FUNCTION: UErrorCode = 65570;
pub const U_INVALID_ID: UErrorCode = 65569;
pub const U_INVALID_PROPERTY_PATTERN: UErrorCode = 65561;
pub const U_INVALID_RBT_SYNTAX: UErrorCode = 65560;
pub const U_INVALID_STATE_ERROR: UErrorCode = 27;
pub const U_INVALID_TABLE_FILE: UErrorCode = 14;
pub const U_INVALID_TABLE_FORMAT: UErrorCode = 13;
pub const U_INVARIANT_CONVERSION_ERROR: UErrorCode = 26;
pub const U_IS_BIG_ENDIAN: u32 = 0;
pub const U_JG_AFRICAN_FEH: UJoiningGroup = 86;
pub const U_JG_AFRICAN_NOON: UJoiningGroup = 87;
pub const U_JG_AFRICAN_QAF: UJoiningGroup = 88;
pub const U_JG_AIN: UJoiningGroup = 1;
pub const U_JG_ALAPH: UJoiningGroup = 2;
pub const U_JG_ALEF: UJoiningGroup = 3;
pub const U_JG_BEH: UJoiningGroup = 4;
pub const U_JG_BETH: UJoiningGroup = 5;
pub const U_JG_BURUSHASKI_YEH_BARREE: UJoiningGroup = 54;
pub const U_JG_DAL: UJoiningGroup = 6;
pub const U_JG_DALATH_RISH: UJoiningGroup = 7;
pub const U_JG_E: UJoiningGroup = 8;
pub const U_JG_FARSI_YEH: UJoiningGroup = 55;
pub const U_JG_FE: UJoiningGroup = 51;
pub const U_JG_FEH: UJoiningGroup = 9;
pub const U_JG_FINAL_SEMKATH: UJoiningGroup = 10;
pub const U_JG_GAF: UJoiningGroup = 11;
pub const U_JG_GAMAL: UJoiningGroup = 12;
pub const U_JG_HAH: UJoiningGroup = 13;
pub const U_JG_HAMZA_ON_HEH_GOAL: UJoiningGroup = 14;
pub const U_JG_HANIFI_ROHINGYA_KINNA_YA: UJoiningGroup = 100;
pub const U_JG_HANIFI_ROHINGYA_PA: UJoiningGroup = 101;
pub const U_JG_HE: UJoiningGroup = 15;
pub const U_JG_HEH: UJoiningGroup = 16;
pub const U_JG_HEH_GOAL: UJoiningGroup = 17;
pub const U_JG_HETH: UJoiningGroup = 18;
pub const U_JG_KAF: UJoiningGroup = 19;
pub const U_JG_KAPH: UJoiningGroup = 20;
pub const U_JG_KHAPH: UJoiningGroup = 52;
pub const U_JG_KNOTTED_HEH: UJoiningGroup = 21;
pub const U_JG_LAM: UJoiningGroup = 22;
pub const U_JG_LAMADH: UJoiningGroup = 23;
pub const U_JG_MALAYALAM_BHA: UJoiningGroup = 89;
pub const U_JG_MALAYALAM_JA: UJoiningGroup = 90;
pub const U_JG_MALAYALAM_LLA: UJoiningGroup = 91;
pub const U_JG_MALAYALAM_LLLA: UJoiningGroup = 92;
pub const U_JG_MALAYALAM_NGA: UJoiningGroup = 93;
pub const U_JG_MALAYALAM_NNA: UJoiningGroup = 94;
pub const U_JG_MALAYALAM_NNNA: UJoiningGroup = 95;
pub const U_JG_MALAYALAM_NYA: UJoiningGroup = 96;
pub const U_JG_MALAYALAM_RA: UJoiningGroup = 97;
pub const U_JG_MALAYALAM_SSA: UJoiningGroup = 98;
pub const U_JG_MALAYALAM_TTA: UJoiningGroup = 99;
pub const U_JG_MANICHAEAN_ALEPH: UJoiningGroup = 58;
pub const U_JG_MANICHAEAN_AYIN: UJoiningGroup = 59;
pub const U_JG_MANICHAEAN_BETH: UJoiningGroup = 60;
pub const U_JG_MANICHAEAN_DALETH: UJoiningGroup = 61;
pub const U_JG_MANICHAEAN_DHAMEDH: UJoiningGroup = 62;
pub const U_JG_MANICHAEAN_FIVE: UJoiningGroup = 63;
pub const U_JG_MANICHAEAN_GIMEL: UJoiningGroup = 64;
pub const U_JG_MANICHAEAN_HETH: UJoiningGroup = 65;
pub const U_JG_MANICHAEAN_HUNDRED: UJoiningGroup = 66;
pub const U_JG_MANICHAEAN_KAPH: UJoiningGroup = 67;
pub const U_JG_MANICHAEAN_LAMEDH: UJoiningGroup = 68;
pub const U_JG_MANICHAEAN_MEM: UJoiningGroup = 69;
pub const U_JG_MANICHAEAN_NUN: UJoiningGroup = 70;
pub const U_JG_MANICHAEAN_ONE: UJoiningGroup = 71;
pub const U_JG_MANICHAEAN_PE: UJoiningGroup = 72;
pub const U_JG_MANICHAEAN_QOPH: UJoiningGroup = 73;
pub const U_JG_MANICHAEAN_RESH: UJoiningGroup = 74;
pub const U_JG_MANICHAEAN_SADHE: UJoiningGroup = 75;
pub const U_JG_MANICHAEAN_SAMEKH: UJoiningGroup = 76;
pub const U_JG_MANICHAEAN_TAW: UJoiningGroup = 77;
pub const U_JG_MANICHAEAN_TEN: UJoiningGroup = 78;
pub const U_JG_MANICHAEAN_TETH: UJoiningGroup = 79;
pub const U_JG_MANICHAEAN_THAMEDH: UJoiningGroup = 80;
pub const U_JG_MANICHAEAN_TWENTY: UJoiningGroup = 81;
pub const U_JG_MANICHAEAN_WAW: UJoiningGroup = 82;
pub const U_JG_MANICHAEAN_YODH: UJoiningGroup = 83;
pub const U_JG_MANICHAEAN_ZAYIN: UJoiningGroup = 84;
pub const U_JG_MEEM: UJoiningGroup = 24;
pub const U_JG_MIM: UJoiningGroup = 25;
pub const U_JG_NOON: UJoiningGroup = 26;
pub const U_JG_NO_JOINING_GROUP: UJoiningGroup = 0;
pub const U_JG_NUN: UJoiningGroup = 27;
pub const U_JG_NYA: UJoiningGroup = 56;
pub const U_JG_PE: UJoiningGroup = 28;
pub const U_JG_QAF: UJoiningGroup = 29;
pub const U_JG_QAPH: UJoiningGroup = 30;
pub const U_JG_REH: UJoiningGroup = 31;
pub const U_JG_REVERSED_PE: UJoiningGroup = 32;
pub const U_JG_ROHINGYA_YEH: UJoiningGroup = 57;
pub const U_JG_SAD: UJoiningGroup = 33;
pub const U_JG_SADHE: UJoiningGroup = 34;
pub const U_JG_SEEN: UJoiningGroup = 35;
pub const U_JG_SEMKATH: UJoiningGroup = 36;
pub const U_JG_SHIN: UJoiningGroup = 37;
pub const U_JG_STRAIGHT_WAW: UJoiningGroup = 85;
pub const U_JG_SWASH_KAF: UJoiningGroup = 38;
pub const U_JG_SYRIAC_WAW: UJoiningGroup = 39;
pub const U_JG_TAH: UJoiningGroup = 40;
pub const U_JG_TAW: UJoiningGroup = 41;
pub const U_JG_TEH_MARBUTA: UJoiningGroup = 42;
pub const U_JG_TEH_MARBUTA_GOAL: UJoiningGroup = 14;
pub const U_JG_TETH: UJoiningGroup = 43;
pub const U_JG_THIN_YEH: UJoiningGroup = 102;
pub const U_JG_VERTICAL_TAIL: UJoiningGroup = 103;
pub const U_JG_WAW: UJoiningGroup = 44;
pub const U_JG_YEH: UJoiningGroup = 45;
pub const U_JG_YEH_BARREE: UJoiningGroup = 46;
pub const U_JG_YEH_WITH_TAIL: UJoiningGroup = 47;
pub const U_JG_YUDH: UJoiningGroup = 48;
pub const U_JG_YUDH_HE: UJoiningGroup = 49;
pub const U_JG_ZAIN: UJoiningGroup = 50;
pub const U_JG_ZHAIN: UJoiningGroup = 53;
pub const U_JT_DUAL_JOINING: UJoiningType = 2;
pub const U_JT_JOIN_CAUSING: UJoiningType = 1;
pub const U_JT_LEFT_JOINING: UJoiningType = 3;
pub const U_JT_NON_JOINING: UJoiningType = 0;
pub const U_JT_RIGHT_JOINING: UJoiningType = 4;
pub const U_JT_TRANSPARENT: UJoiningType = 5;
pub const U_LB_ALPHABETIC: ULineBreak = 2;
pub const U_LB_AMBIGUOUS: ULineBreak = 1;
pub const U_LB_BREAK_AFTER: ULineBreak = 4;
pub const U_LB_BREAK_BEFORE: ULineBreak = 5;
pub const U_LB_BREAK_BOTH: ULineBreak = 3;
pub const U_LB_BREAK_SYMBOLS: ULineBreak = 27;
pub const U_LB_CARRIAGE_RETURN: ULineBreak = 10;
pub const U_LB_CLOSE_PARENTHESIS: ULineBreak = 36;
pub const U_LB_CLOSE_PUNCTUATION: ULineBreak = 8;
pub const U_LB_COMBINING_MARK: ULineBreak = 9;
pub const U_LB_COMPLEX_CONTEXT: ULineBreak = 24;
pub const U_LB_CONDITIONAL_JAPANESE_STARTER: ULineBreak = 37;
pub const U_LB_CONTINGENT_BREAK: ULineBreak = 7;
pub const U_LB_EXCLAMATION: ULineBreak = 11;
pub const U_LB_E_BASE: ULineBreak = 40;
pub const U_LB_E_MODIFIER: ULineBreak = 41;
pub const U_LB_GLUE: ULineBreak = 12;
pub const U_LB_H2: ULineBreak = 31;
pub const U_LB_H3: ULineBreak = 32;
pub const U_LB_HEBREW_LETTER: ULineBreak = 38;
pub const U_LB_HYPHEN: ULineBreak = 13;
pub const U_LB_IDEOGRAPHIC: ULineBreak = 14;
pub const U_LB_INFIX_NUMERIC: ULineBreak = 16;
pub const U_LB_INSEPARABLE: ULineBreak = 15;
pub const U_LB_INSEPERABLE: ULineBreak = 15;
pub const U_LB_JL: ULineBreak = 33;
pub const U_LB_JT: ULineBreak = 34;
pub const U_LB_JV: ULineBreak = 35;
pub const U_LB_LINE_FEED: ULineBreak = 17;
pub const U_LB_MANDATORY_BREAK: ULineBreak = 6;
pub const U_LB_NEXT_LINE: ULineBreak = 29;
pub const U_LB_NONSTARTER: ULineBreak = 18;
pub const U_LB_NUMERIC: ULineBreak = 19;
pub const U_LB_OPEN_PUNCTUATION: ULineBreak = 20;
pub const U_LB_POSTFIX_NUMERIC: ULineBreak = 21;
pub const U_LB_PREFIX_NUMERIC: ULineBreak = 22;
pub const U_LB_QUOTATION: ULineBreak = 23;
pub const U_LB_REGIONAL_INDICATOR: ULineBreak = 39;
pub const U_LB_SPACE: ULineBreak = 26;
pub const U_LB_SURROGATE: ULineBreak = 25;
pub const U_LB_UNKNOWN: ULineBreak = 0;
pub const U_LB_WORD_JOINER: ULineBreak = 30;
pub const U_LB_ZWJ: ULineBreak = 42;
pub const U_LB_ZWSPACE: ULineBreak = 28;
pub const U_LEFT_TO_RIGHT: UCharDirection = 0;
pub const U_LEFT_TO_RIGHT_EMBEDDING: UCharDirection = 11;
pub const U_LEFT_TO_RIGHT_ISOLATE: UCharDirection = 20;
pub const U_LEFT_TO_RIGHT_OVERRIDE: UCharDirection = 12;
pub const U_LETTER_NUMBER: UCharCategory = 10;
pub const U_LIB_SUFFIX_C_NAME_STRING: windows_core::PCSTR = windows_core::s!("");
pub const U_LINE_SEPARATOR: UCharCategory = 13;
pub const U_LONG_PROPERTY_NAME: UPropertyNameChoice = 1;
pub const U_LOWERCASE_LETTER: UCharCategory = 2;
pub const U_MALFORMED_EXPONENTIAL_PATTERN: UErrorCode = 65795;
pub const U_MALFORMED_PRAGMA: UErrorCode = 65562;
pub const U_MALFORMED_RULE: UErrorCode = 65537;
pub const U_MALFORMED_SET: UErrorCode = 65538;
pub const U_MALFORMED_SYMBOL_REFERENCE: UErrorCode = 65539;
pub const U_MALFORMED_UNICODE_ESCAPE: UErrorCode = 65540;
pub const U_MALFORMED_VARIABLE_DEFINITION: UErrorCode = 65541;
pub const U_MALFORMED_VARIABLE_REFERENCE: UErrorCode = 65542;
pub const U_MATH_SYMBOL: UCharCategory = 24;
pub const U_MAX_VERSION_LENGTH: u32 = 4;
pub const U_MAX_VERSION_STRING_LENGTH: u32 = 20;
pub const U_MEMORY_ALLOCATION_ERROR: UErrorCode = 7;
pub const U_MESSAGE_PARSE_ERROR: UErrorCode = 6;
pub const U_MILLIS_PER_DAY: u32 = 86400000;
pub const U_MILLIS_PER_HOUR: u32 = 3600000;
pub const U_MILLIS_PER_MINUTE: u32 = 60000;
pub const U_MILLIS_PER_SECOND: u32 = 1000;
pub const U_MISMATCHED_SEGMENT_DELIMITERS: UErrorCode = 65543;
pub const U_MISPLACED_ANCHOR_START: UErrorCode = 65544;
pub const U_MISPLACED_COMPOUND_FILTER: UErrorCode = 65558;
pub const U_MISPLACED_CURSOR_OFFSET: UErrorCode = 65545;
pub const U_MISPLACED_QUANTIFIER: UErrorCode = 65546;
pub const U_MISSING_OPERATOR: UErrorCode = 65547;
pub const U_MISSING_RESOURCE_ERROR: UErrorCode = 2;
pub const U_MISSING_SEGMENT_CLOSE: UErrorCode = 65548;
pub const U_MODIFIER_LETTER: UCharCategory = 4;
pub const U_MODIFIER_SYMBOL: UCharCategory = 26;
pub const U_MULTIPLE_ANTE_CONTEXTS: UErrorCode = 65549;
pub const U_MULTIPLE_COMPOUND_FILTERS: UErrorCode = 65559;
pub const U_MULTIPLE_CURSORS: UErrorCode = 65550;
pub const U_MULTIPLE_DECIMAL_SEPARATORS: UErrorCode = 65793;
pub const U_MULTIPLE_DECIMAL_SEPERATORS: UErrorCode = 65793;
pub const U_MULTIPLE_EXPONENTIAL_SYMBOLS: UErrorCode = 65794;
pub const U_MULTIPLE_PAD_SPECIFIERS: UErrorCode = 65798;
pub const U_MULTIPLE_PERCENT_SYMBOLS: UErrorCode = 65796;
pub const U_MULTIPLE_PERMILL_SYMBOLS: UErrorCode = 65797;
pub const U_MULTIPLE_POST_CONTEXTS: UErrorCode = 65551;
pub const U_NON_SPACING_MARK: UCharCategory = 6;
pub const U_NO_DEFAULT_INCLUDE_UTF_HEADERS: u32 = 1;
pub const U_NO_SPACE_AVAILABLE: UErrorCode = 20;
pub const U_NO_WRITE_PERMISSION: UErrorCode = 30;
pub const U_NT_DECIMAL: UNumericType = 1;
pub const U_NT_DIGIT: UNumericType = 2;
pub const U_NT_NONE: UNumericType = 0;
pub const U_NT_NUMERIC: UNumericType = 3;
pub const U_NUMBER_ARG_OUTOFBOUNDS_ERROR: UErrorCode = 65810;
pub const U_NUMBER_SKELETON_SYNTAX_ERROR: UErrorCode = 65811;
pub const U_OMIT_UNCHANGED_TEXT: u32 = 16384;
pub const U_OTHER_LETTER: UCharCategory = 5;
pub const U_OTHER_NEUTRAL: UCharDirection = 10;
pub const U_OTHER_NUMBER: UCharCategory = 11;
pub const U_OTHER_PUNCTUATION: UCharCategory = 23;
pub const U_OTHER_SYMBOL: UCharCategory = 27;
pub const U_OVERRIDE_CXX_ALLOCATION: u32 = 1;
pub const U_PARAGRAPH_SEPARATOR: UCharCategory = 14;
pub const U_PARSE_CONTEXT_LEN: i32 = 16;
pub const U_PARSE_ERROR: UErrorCode = 9;
pub const U_PARSE_ERROR_START: UErrorCode = 65536;
pub const U_PATTERN_SYNTAX_ERROR: UErrorCode = 65799;
pub const U_PF_AIX: u32 = 3100;
pub const U_PF_ANDROID: u32 = 4050;
pub const U_PF_BROWSER_NATIVE_CLIENT: u32 = 4020;
pub const U_PF_BSD: u32 = 3000;
pub const U_PF_CYGWIN: u32 = 1900;
pub const U_PF_DARWIN: u32 = 3500;
pub const U_PF_EMSCRIPTEN: u32 = 5010;
pub const U_PF_FUCHSIA: u32 = 4100;
pub const U_PF_HPUX: u32 = 2100;
pub const U_PF_IPHONE: u32 = 3550;
pub const U_PF_IRIX: u32 = 3200;
pub const U_PF_LINUX: u32 = 4000;
pub const U_PF_MINGW: u32 = 1800;
pub const U_PF_OS390: u32 = 9000;
pub const U_PF_OS400: u32 = 9400;
pub const U_PF_QNX: u32 = 3700;
pub const U_PF_SOLARIS: u32 = 2600;
pub const U_PF_UNKNOWN: u32 = 0;
pub const U_PF_WINDOWS: u32 = 1000;
pub const U_PLATFORM: u32 = 1000;
pub const U_PLATFORM_HAS_WIN32_API: u32 = 1;
pub const U_PLATFORM_IMPLEMENTS_POSIX: u32 = 0;
pub const U_PLATFORM_IS_DARWIN_BASED: u32 = 0;
pub const U_PLATFORM_IS_LINUX_BASED: u32 = 0;
pub const U_PLATFORM_USES_ONLY_WIN32_API: u32 = 1;
pub const U_PLUGIN_CHANGED_LEVEL_WARNING: UErrorCode = -120;
pub const U_PLUGIN_DIDNT_SET_LEVEL: UErrorCode = 66817;
pub const U_PLUGIN_ERROR_START: UErrorCode = 66816;
pub const U_PLUGIN_TOO_HIGH: UErrorCode = 66816;
pub const U_POP_DIRECTIONAL_FORMAT: UCharDirection = 16;
pub const U_POP_DIRECTIONAL_ISOLATE: UCharDirection = 22;
pub const U_PRIMARY_TOO_LONG_ERROR: UErrorCode = 22;
pub const U_PRIVATE_USE_CHAR: UCharCategory = 17;
pub const U_REGEX_BAD_ESCAPE_SEQUENCE: UErrorCode = 66307;
pub const U_REGEX_BAD_INTERVAL: UErrorCode = 66312;
pub const U_REGEX_ERROR_START: UErrorCode = 66304;
pub const U_REGEX_INTERNAL_ERROR: UErrorCode = 66304;
pub const U_REGEX_INVALID_BACK_REF: UErrorCode = 66314;
pub const U_REGEX_INVALID_CAPTURE_GROUP_NAME: UErrorCode = 66325;
pub const U_REGEX_INVALID_FLAG: UErrorCode = 66315;
pub const U_REGEX_INVALID_RANGE: UErrorCode = 66320;
pub const U_REGEX_INVALID_STATE: UErrorCode = 66306;
pub const U_REGEX_LOOK_BEHIND_LIMIT: UErrorCode = 66316;
pub const U_REGEX_MAX_LT_MIN: UErrorCode = 66313;
pub const U_REGEX_MISMATCHED_PAREN: UErrorCode = 66310;
pub const U_REGEX_MISSING_CLOSE_BRACKET: UErrorCode = 66319;
pub const U_REGEX_NUMBER_TOO_BIG: UErrorCode = 66311;
pub const U_REGEX_PATTERN_TOO_BIG: UErrorCode = 66324;
pub const U_REGEX_PROPERTY_SYNTAX: UErrorCode = 66308;
pub const U_REGEX_RULE_SYNTAX: UErrorCode = 66305;
pub const U_REGEX_SET_CONTAINS_STRING: UErrorCode = 66317;
pub const U_REGEX_STACK_OVERFLOW: UErrorCode = 66321;
pub const U_REGEX_STOPPED_BY_CALLER: UErrorCode = 66323;
pub const U_REGEX_TIME_OUT: UErrorCode = 66322;
pub const U_REGEX_UNIMPLEMENTED: UErrorCode = 66309;
pub const U_RESOURCE_TYPE_MISMATCH: UErrorCode = 17;
pub const U_RIGHT_TO_LEFT: UCharDirection = 1;
pub const U_RIGHT_TO_LEFT_ARABIC: UCharDirection = 13;
pub const U_RIGHT_TO_LEFT_EMBEDDING: UCharDirection = 14;
pub const U_RIGHT_TO_LEFT_ISOLATE: UCharDirection = 21;
pub const U_RIGHT_TO_LEFT_OVERRIDE: UCharDirection = 15;
pub const U_RULE_MASK_ERROR: UErrorCode = 65557;
pub const U_SAFECLONE_ALLOCATED_WARNING: UErrorCode = -126;
pub const U_SB_ATERM: USentenceBreak = 1;
pub const U_SB_CLOSE: USentenceBreak = 2;
pub const U_SB_CR: USentenceBreak = 11;
pub const U_SB_EXTEND: USentenceBreak = 12;
pub const U_SB_FORMAT: USentenceBreak = 3;
pub const U_SB_LF: USentenceBreak = 13;
pub const U_SB_LOWER: USentenceBreak = 4;
pub const U_SB_NUMERIC: USentenceBreak = 5;
pub const U_SB_OLETTER: USentenceBreak = 6;
pub const U_SB_OTHER: USentenceBreak = 0;
pub const U_SB_SCONTINUE: USentenceBreak = 14;
pub const U_SB_SEP: USentenceBreak = 7;
pub const U_SB_SP: USentenceBreak = 8;
pub const U_SB_STERM: USentenceBreak = 9;
pub const U_SB_UPPER: USentenceBreak = 10;
pub const U_SEGMENT_SEPARATOR: UCharDirection = 8;
pub const U_SENTINEL: i32 = -1;
pub const U_SHAPE_AGGREGATE_TASHKEEL: u32 = 16384;
pub const U_SHAPE_AGGREGATE_TASHKEEL_MASK: u32 = 16384;
pub const U_SHAPE_AGGREGATE_TASHKEEL_NOOP: u32 = 0;
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_AL: u32 = 128;
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_LR: u32 = 96;
pub const U_SHAPE_DIGITS_AN2EN: u32 = 64;
pub const U_SHAPE_DIGITS_EN2AN: u32 = 32;
pub const U_SHAPE_DIGITS_MASK: u32 = 224;
pub const U_SHAPE_DIGITS_NOOP: u32 = 0;
pub const U_SHAPE_DIGITS_RESERVED: u32 = 160;
pub const U_SHAPE_DIGIT_TYPE_AN: u32 = 0;
pub const U_SHAPE_DIGIT_TYPE_AN_EXTENDED: u32 = 256;
pub const U_SHAPE_DIGIT_TYPE_MASK: u32 = 768;
pub const U_SHAPE_DIGIT_TYPE_RESERVED: u32 = 512;
pub const U_SHAPE_LAMALEF_AUTO: u32 = 65536;
pub const U_SHAPE_LAMALEF_BEGIN: u32 = 3;
pub const U_SHAPE_LAMALEF_END: u32 = 2;
pub const U_SHAPE_LAMALEF_MASK: u32 = 65539;
pub const U_SHAPE_LAMALEF_NEAR: u32 = 1;
pub const U_SHAPE_LAMALEF_RESIZE: u32 = 0;
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_BEGINNING: u32 = 3;
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_END: u32 = 2;
pub const U_SHAPE_LENGTH_FIXED_SPACES_NEAR: u32 = 1;
pub const U_SHAPE_LENGTH_GROW_SHRINK: u32 = 0;
pub const U_SHAPE_LENGTH_MASK: u32 = 65539;
pub const U_SHAPE_LETTERS_MASK: u32 = 24;
pub const U_SHAPE_LETTERS_NOOP: u32 = 0;
pub const U_SHAPE_LETTERS_SHAPE: u32 = 8;
pub const U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED: u32 = 24;
pub const U_SHAPE_LETTERS_UNSHAPE: u32 = 16;
pub const U_SHAPE_PRESERVE_PRESENTATION: u32 = 32768;
pub const U_SHAPE_PRESERVE_PRESENTATION_MASK: u32 = 32768;
pub const U_SHAPE_PRESERVE_PRESENTATION_NOOP: u32 = 0;
pub const U_SHAPE_SEEN_MASK: u32 = 7340032;
pub const U_SHAPE_SEEN_TWOCELL_NEAR: u32 = 2097152;
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END: u32 = 67108864;
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_MASK: u32 = 67108864;
pub const U_SHAPE_TAIL_NEW_UNICODE: u32 = 134217728;
pub const U_SHAPE_TAIL_TYPE_MASK: u32 = 134217728;
pub const U_SHAPE_TASHKEEL_BEGIN: u32 = 262144;
pub const U_SHAPE_TASHKEEL_END: u32 = 393216;
pub const U_SHAPE_TASHKEEL_MASK: u32 = 917504;
pub const U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL: u32 = 786432;
pub const U_SHAPE_TASHKEEL_RESIZE: u32 = 524288;
pub const U_SHAPE_TEXT_DIRECTION_LOGICAL: u32 = 0;
pub const U_SHAPE_TEXT_DIRECTION_MASK: u32 = 4;
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_LTR: u32 = 4;
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_RTL: u32 = 0;
pub const U_SHAPE_YEHHAMZA_MASK: u32 = 58720256;
pub const U_SHAPE_YEHHAMZA_TWOCELL_NEAR: u32 = 16777216;
pub const U_SHORT_PROPERTY_NAME: UPropertyNameChoice = 0;
pub const U_SHOW_CPLUSPLUS_API: u32 = 0;
pub const U_SIZEOF_UCHAR: u32 = 2;
pub const U_SIZEOF_WCHAR_T: u32 = 2;
pub const U_SORT_KEY_TOO_SHORT_WARNING: UErrorCode = -123;
pub const U_SPACE_SEPARATOR: UCharCategory = 12;
pub const U_START_PUNCTUATION: UCharCategory = 20;
pub const U_STATE_OLD_WARNING: UErrorCode = -125;
pub const U_STATE_TOO_OLD_ERROR: UErrorCode = 23;
pub const U_STRINGPREP_CHECK_BIDI_ERROR: UErrorCode = 66562;
pub const U_STRINGPREP_PROHIBITED_ERROR: UErrorCode = 66560;
pub const U_STRINGPREP_UNASSIGNED_ERROR: UErrorCode = 66561;
pub const U_STRING_NOT_TERMINATED_WARNING: UErrorCode = -124;
pub const U_SURROGATE: UCharCategory = 18;
pub const U_TITLECASE_ADJUST_TO_CASED: u32 = 1024;
pub const U_TITLECASE_LETTER: UCharCategory = 3;
pub const U_TITLECASE_NO_BREAK_ADJUSTMENT: u32 = 512;
pub const U_TITLECASE_NO_LOWERCASE: u32 = 256;
pub const U_TITLECASE_SENTENCES: u32 = 64;
pub const U_TITLECASE_WHOLE_STRING: u32 = 32;
pub const U_TOO_MANY_ALIASES_ERROR: UErrorCode = 24;
pub const U_TRAILING_BACKSLASH: UErrorCode = 65552;
pub const U_TRUNCATED_CHAR_FOUND: UErrorCode = 11;
pub const U_UINT64_MAX: i32 = -1;
pub const U_UNASSIGNED: UCharCategory = 0;
pub const U_UNCLOSED_SEGMENT: UErrorCode = 65563;
pub const U_UNDEFINED_KEYWORD: UErrorCode = 65806;
pub const U_UNDEFINED_SEGMENT_REFERENCE: UErrorCode = 65553;
pub const U_UNDEFINED_VARIABLE: UErrorCode = 65554;
pub const U_UNEXPECTED_TOKEN: UErrorCode = 65792;
pub const U_UNICODE_CHAR_NAME: UCharNameChoice = 0;
pub const U_UNMATCHED_BRACES: UErrorCode = 65801;
pub const U_UNQUOTED_SPECIAL: UErrorCode = 65555;
pub const U_UNSUPPORTED_ATTRIBUTE: UErrorCode = 65803;
pub const U_UNSUPPORTED_ERROR: UErrorCode = 16;
pub const U_UNSUPPORTED_ESCAPE_SEQUENCE: UErrorCode = 19;
pub const U_UNSUPPORTED_PROPERTY: UErrorCode = 65802;
pub const U_UNTERMINATED_QUOTE: UErrorCode = 65556;
pub const U_UPPERCASE_LETTER: UCharCategory = 1;
pub const U_USELESS_COLLATOR_ERROR: UErrorCode = 29;
pub const U_USING_DEFAULT_WARNING: UErrorCode = -127;
pub const U_USING_FALLBACK_WARNING: UErrorCode = -128;
pub const U_VARIABLE_RANGE_EXHAUSTED: UErrorCode = 65565;
pub const U_VARIABLE_RANGE_OVERLAP: UErrorCode = 65566;
pub const U_VERSION_DELIMITER: u32 = 46;
pub const U_VO_ROTATED: UVerticalOrientation = 0;
pub const U_VO_TRANSFORMED_ROTATED: UVerticalOrientation = 1;
pub const U_VO_TRANSFORMED_UPRIGHT: UVerticalOrientation = 2;
pub const U_VO_UPRIGHT: UVerticalOrientation = 3;
pub const U_WB_ALETTER: UWordBreakValues = 1;
pub const U_WB_CR: UWordBreakValues = 8;
pub const U_WB_DOUBLE_QUOTE: UWordBreakValues = 16;
pub const U_WB_EXTEND: UWordBreakValues = 9;
pub const U_WB_EXTENDNUMLET: UWordBreakValues = 7;
pub const U_WB_E_BASE: UWordBreakValues = 17;
pub const U_WB_E_BASE_GAZ: UWordBreakValues = 18;
pub const U_WB_E_MODIFIER: UWordBreakValues = 19;
pub const U_WB_FORMAT: UWordBreakValues = 2;
pub const U_WB_GLUE_AFTER_ZWJ: UWordBreakValues = 20;
pub const U_WB_HEBREW_LETTER: UWordBreakValues = 14;
pub const U_WB_KATAKANA: UWordBreakValues = 3;
pub const U_WB_LF: UWordBreakValues = 10;
pub const U_WB_MIDLETTER: UWordBreakValues = 4;
pub const U_WB_MIDNUM: UWordBreakValues = 5;
pub const U_WB_MIDNUMLET: UWordBreakValues = 11;
pub const U_WB_NEWLINE: UWordBreakValues = 12;
pub const U_WB_NUMERIC: UWordBreakValues = 6;
pub const U_WB_OTHER: UWordBreakValues = 0;
pub const U_WB_REGIONAL_INDICATOR: UWordBreakValues = 13;
pub const U_WB_SINGLE_QUOTE: UWordBreakValues = 15;
pub const U_WB_WSEGSPACE: UWordBreakValues = 22;
pub const U_WB_ZWJ: UWordBreakValues = 21;
pub const U_WHITE_SPACE_NEUTRAL: UCharDirection = 9;
pub const U_ZERO_ERROR: UErrorCode = 0;
pub type u_nl_catd = *mut UResourceBundle;
