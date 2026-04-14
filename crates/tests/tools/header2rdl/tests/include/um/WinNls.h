/*++

Copyright (c) Microsoft Corporation  All rights reserved.

Module Name:

    winnls.h

Abstract:

    Procedure declarations, constant definitions, and macros for the
    NLS component.

--*/


#ifndef _WINNLS_
#define _WINNLS_

#include <winapifamily.h>


#ifdef __cplusplus
extern "C" {
#endif



#ifndef NOAPISET
#include <datetimeapi.h>  // Datetime APISET dependencies
#include <libloaderapi.h> // LibLoader Apiset dependencies
#endif


#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

//
// Deprecated attribute support for NLS
//
#pragma push_macro("DEPRECATED")
#undef DEPRECATED

// Disable NLS deprecation for the moment
#if !defined(DISABLE_NLS_DEPRECATION)
#define DISABLE_NLS_DEPRECATION
#endif

// Deprecated NLS types/functions
#if !defined(DISABLE_NLS_DEPRECATION)
#if defined(__cplusplus)
#if __cplusplus >= 201402
#define DEPRECATED(x) [[deprecated(x)]]
#elif defined(_MSC_VER)
#if _MSC_VER > 1900
#define DEPRECATED(x) [[deprecated(x)]]
#else
#define DEPRECATED(x) __declspec(deprecated(x))
#endif // _MSC_VER > 1900
#else // Not Standard C++ or MSVC, ignore the construct.
#define DEPRECATED(x)
#endif  // C++ deprecation
#else // C - disable deprecation
#define DEPRECATED(x)
#endif
#else // Deprecation is disabled
#define DEPRECATED(x)
#endif  /* DISABLE_NLS_DEPRECATION */


#ifndef NONLS

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef _MAC
#include <macwin32.h>
#endif

#if !defined(_NORMALIZE_)
#define WINNORMALIZEAPI DECLSPEC_IMPORT
#else
#define WINNORMALIZEAPI
#endif



////////////////////////////////////////////////////////////////////////////
//
//  Constants
//
//  Define all constants for the NLS component here.
//
////////////////////////////////////////////////////////////////////////////

//
//  String Length Maximums.
//
#define MAX_LEADBYTES             12          // 5 ranges, 2 bytes ea., 0 term.
#define MAX_DEFAULTCHAR           2           // single or double byte

//
//  Surrogate pairs
//
//  Conversion examples:
//
//  A) The first character in the Surrogate range (D800, DC00) as UTF-32:
//
//  1.  D800: binary 1101100000000000  (lower ten bits: 0000000000)
//  2.  DC00: binary 1101110000000000  (lower ten bits: 0000000000)
//  3.  Concatenate 0000000000+0000000000 = 0x0000
//  4.  Add 0x10000
//
//  Result: U+10000. This is correct, since the first character in the Supplementary character
//  range immediately follows the last code point in the 16-bit UTF-16 range (U+FFFF)
//
//  B) A UTF-32 code point such as U+2040A (this a CJK character in CJK Extension B), and wish
//  to convert it in UTF-16:
//
//  1.  Subtract 0x10000 - Result: 0x1040A
//  2.  Split into two ten-bit pieces: 0001000001 0000001010
//  3.  Add 1101100000000000 (0xD800) to the high 10 bits piece (0001000001) - Result: 1101100001000001 (0xD841)
//  4.  Add 1101110000000000 (0xDC00) to the low 10 bits piece (0000001010) - Result: 1101110000001010 (0xDC0A)
//
//  RESULT: The surrogate pair: U+D841, U+DC0A
//
//  Special Unicode code point values, for use with UTF-16 surrogate pairs.
//
#define HIGH_SURROGATE_START  0xd800
#define HIGH_SURROGATE_END    0xdbff
#define LOW_SURROGATE_START   0xdc00
#define LOW_SURROGATE_END     0xdfff


//
//  MBCS and Unicode Translation Flags.
//  Please use Unicode, either UTF-16 (WCHAR) or UTF-8 (CP_UTF8)
//
// MB_PRECOMPOSED and MB_COMPOSITE are deprecated, not recommended, and
// provide out-of-date behavior.
// Windows typically uses Unicode Normalization Form C type sequences,
// If explicit normalization forms are required, please use NormalizeString.
#define MB_PRECOMPOSED            0x00000001  // DEPRECATED: use single precomposed characters when possible.
#define MB_COMPOSITE              0x00000002  // DEPRECATED: use multiple discrete characters when possible.
#define MB_USEGLYPHCHARS          0x00000004  // DEPRECATED: use glyph chars, not ctrl chars
#define MB_ERR_INVALID_CHARS      0x00000008  // error for invalid chars

// WC_COMPOSITECHECK, WC_DISCARDNS and WC_SEPCHARS are deprecated, not recommended,
// and provide out-of-date behavior.
// Windows typically uses Unicode Normalization Form C type sequences,
// If explicit normalization forms are required, please use NormalizeString.
#define WC_COMPOSITECHECK         0x00000200  // convert composite to precomposed
#define WC_DISCARDNS              0x00000010  // discard non-spacing chars          // Used with WC_COMPOSITECHECK
#define WC_SEPCHARS               0x00000020  // generate separate chars            // Used with WC_COMPOSITECHECK
#define WC_DEFAULTCHAR            0x00000040  // replace w/ default char            // Used with WC_COMPOSITECHECK
#if (WINVER >= 0x0600)
#define WC_ERR_INVALID_CHARS      0x00000080  // error for invalid chars
#endif

#if(WINVER >= 0x0500)
#define WC_NO_BEST_FIT_CHARS      0x00000400  // do not use best fit chars
#endif /* WINVER >= 0x0500 */


//
//  Character Type Flags.
//
#define CT_CTYPE1                 0x00000001  // ctype 1 information
#define CT_CTYPE2                 0x00000002  // ctype 2 information
#define CT_CTYPE3                 0x00000004  // ctype 3 information

//
//  CType 1 Flag Bits.
//
#define C1_UPPER                  0x0001      // upper case
#define C1_LOWER                  0x0002      // lower case
#define C1_DIGIT                  0x0004      // decimal digits
#define C1_SPACE                  0x0008      // spacing characters
#define C1_PUNCT                  0x0010      // punctuation characters
#define C1_CNTRL                  0x0020      // control characters
#define C1_BLANK                  0x0040      // blank characters
#define C1_XDIGIT                 0x0080      // other digits
#define C1_ALPHA                  0x0100      // any linguistic character
#define C1_DEFINED                0x0200      // defined character

//
//  CType 2 Flag Bits.
//
#define C2_LEFTTORIGHT            0x0001      // left to right
#define C2_RIGHTTOLEFT            0x0002      // right to left

#define C2_EUROPENUMBER           0x0003      // European number, digit
#define C2_EUROPESEPARATOR        0x0004      // European numeric separator
#define C2_EUROPETERMINATOR       0x0005      // European numeric terminator
#define C2_ARABICNUMBER           0x0006      // Arabic number
#define C2_COMMONSEPARATOR        0x0007      // common numeric separator

#define C2_BLOCKSEPARATOR         0x0008      // block separator
#define C2_SEGMENTSEPARATOR       0x0009      // segment separator
#define C2_WHITESPACE             0x000A      // white space
#define C2_OTHERNEUTRAL           0x000B      // other neutrals

#define C2_NOTAPPLICABLE          0x0000      // no implicit directionality

//
//  CType 3 Flag Bits.
//
#define C3_NONSPACING             0x0001      // nonspacing character
#define C3_DIACRITIC              0x0002      // diacritic mark
#define C3_VOWELMARK              0x0004      // vowel mark
#define C3_SYMBOL                 0x0008      // symbols

#define C3_KATAKANA               0x0010      // katakana character
#define C3_HIRAGANA               0x0020      // hiragana character
#define C3_HALFWIDTH              0x0040      // half width character
#define C3_FULLWIDTH              0x0080      // full width character
#define C3_IDEOGRAPH              0x0100      // ideographic character
#define C3_KASHIDA                0x0200      // Arabic kashida character
#define C3_LEXICAL                0x0400      // lexical character
#define C3_HIGHSURROGATE          0x0800      // high surrogate code unit
#define C3_LOWSURROGATE           0x1000      // low surrogate code unit

#define C3_ALPHA                  0x8000      // any linguistic char (C1_ALPHA)

#define C3_NOTAPPLICABLE          0x0000      // ctype 3 is not applicable


//
//  String Flags.
//
#define NORM_IGNORECASE           0x00000001  // ignore case
#define NORM_IGNORENONSPACE       0x00000002  // ignore nonspacing chars
#define NORM_IGNORESYMBOLS        0x00000004  // ignore symbols

#define LINGUISTIC_IGNORECASE     0x00000010  // linguistically appropriate 'ignore case'
#define LINGUISTIC_IGNOREDIACRITIC 0x00000020  // linguistically appropriate 'ignore nonspace'

#define NORM_IGNOREKANATYPE       0x00010000  // ignore kanatype
#define NORM_IGNOREWIDTH          0x00020000  // ignore width
#define NORM_LINGUISTIC_CASING    0x08000000  // use linguistic rules for casing


//
//  Locale Independent Mapping Flags.
//
#define MAP_FOLDCZONE             0x00000010  // fold compatibility zone chars
#define MAP_PRECOMPOSED           0x00000020  // convert to precomposed chars
#define MAP_COMPOSITE             0x00000040  // convert to composite chars
#define MAP_FOLDDIGITS            0x00000080  // all digits to ASCII 0-9

#if(WINVER >= 0x0500)
#define MAP_EXPAND_LIGATURES      0x00002000  // expand all ligatures
#endif /* WINVER >= 0x0500 */

//
//  Locale Dependent Mapping Flags.
//
#define LCMAP_LOWERCASE           0x00000100  // lower case letters
#define LCMAP_UPPERCASE           0x00000200  // UPPER CASE LETTERS
#if (WINVER >= _WIN32_WINNT_WIN7)
#define LCMAP_TITLECASE           0x00000300  // Title Case Letters
#endif // (WINVER >= _WIN32_WINNT_WIN7)

#define LCMAP_SORTKEY             0x00000400  // WC sort key (normalize)
#define LCMAP_BYTEREV             0x00000800  // byte reversal

#define LCMAP_HIRAGANA            0x00100000  // map katakana to hiragana
#define LCMAP_KATAKANA            0x00200000  // map hiragana to katakana
#define LCMAP_HALFWIDTH           0x00400000  // map double byte to single byte
#define LCMAP_FULLWIDTH           0x00800000  // map single byte to double byte

#define LCMAP_LINGUISTIC_CASING   0x01000000  // use linguistic rules for casing

#define LCMAP_SIMPLIFIED_CHINESE  0x02000000  // map traditional chinese to simplified chinese
#define LCMAP_TRADITIONAL_CHINESE 0x04000000  // map simplified chinese to traditional chinese


#if (WINVER >= _WIN32_WINNT_WIN8)
#define LCMAP_SORTHANDLE   0x20000000
#define LCMAP_HASH         0x00040000
#endif // (WINVER >= _WIN32_WINNT_WIN7)

//
//  Search Flags
//
#define FIND_STARTSWITH           0x00100000  // see if value is at the beginning of source
#define FIND_ENDSWITH             0x00200000  // see if value is at the end of source
#define FIND_FROMSTART            0x00400000  // look for value in source, starting at the beginning
#define FIND_FROMEND              0x00800000  // look for value in source, starting at the end


//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
// Language Group Enumeration Flags.
//
// The "Language Group" concept is an obsolete concept.
// The groups returned are not well defined, arbitrary, inconsistent, inaccurate,
// no longer maintained, and no longer supported.
//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
#define LGRPID_INSTALLED          0x00000001  // installed language group ids
#define LGRPID_SUPPORTED          0x00000002  // supported language group ids


//
//  Locale Enumeration Flags.
//
#define LCID_INSTALLED            0x00000001  // installed locale ids
#define LCID_SUPPORTED            0x00000002  // supported locale ids
#define LCID_ALTERNATE_SORTS      0x00000004  // alternate sort locale ids


#if (WINVER >= _WIN32_WINNT_VISTA)
//
//  Named based enumeration flags.
//
#define LOCALE_ALL                  0                     // enumerate all named based locales
#define LOCALE_WINDOWS              0x00000001            // shipped locales and/or replacements for them
#define LOCALE_SUPPLEMENTAL         0x00000002            // supplemental locales only
#define LOCALE_ALTERNATE_SORTS      0x00000004            // alternate sort locales
#define LOCALE_REPLACEMENT          0x00000008            // locales that replace shipped locales (callback flag only)
#endif // (WINVER >= _WIN32_WINNT_VISTA)
#if (WINVER >= _WIN32_WINNT_WIN7)
#define LOCALE_NEUTRALDATA          0x00000010            // Locales that are "neutral" (language only, region data is default)
#define LOCALE_SPECIFICDATA         0x00000020            // Locales that contain language and region data
#endif // (WINVER >= _WIN32_WINNT_WIN7)


//
//  Code Page Enumeration Flags.
//
#define CP_INSTALLED              0x00000001  // installed code page ids
#define CP_SUPPORTED              0x00000002  // supported code page ids


//
//  Sorting Flags.
//
//    WORD Sort:    culturally correct sort
//                  hyphen and apostrophe are special cased
//                  example: "coop" and "co-op" will sort together in a list
//
//                        co_op     <-------  underscore (symbol)
//                        coat
//                        comb
//                        coop
//                        co-op     <-------  hyphen (punctuation)
//                        cork
//                        went
//                        were
//                        we're     <-------  apostrophe (punctuation)
//
//
//    STRING Sort:  hyphen and apostrophe will sort with all other symbols
//
//                        co-op     <-------  hyphen (punctuation)
//                        co_op     <-------  underscore (symbol)
//                        coat
//                        comb
//                        coop
//                        cork
//                        we're     <-------  apostrophe (punctuation)
//                        went
//                        were
//
#define SORT_STRINGSORT           0x00001000  // use string sort method

//  Sort digits as numbers (ie: 2 comes before 10)
#if (WINVER >= _WIN32_WINNT_WIN7)
#define SORT_DIGITSASNUMBERS      0x00000008  // use digits as numbers sort method
#endif // (WINVER >= _WIN32_WINNT_WIN7)


//
//  Compare String Return Values.
//
#define CSTR_LESS_THAN            1           // string 1 less than string 2
#define CSTR_EQUAL                2           // string 1 equal to string 2
#define CSTR_GREATER_THAN         3           // string 1 greater than string 2


//
//  Code Page Default Values.
//  Please Use Unicode, either UTF-16 (as in WCHAR) or UTF-8 (code page CP_ACP)
//
#define CP_ACP                    0           // default to ANSI code page
#define CP_OEMCP                  1           // default to OEM  code page
#define CP_MACCP                  2           // default to MAC  code page
#define CP_THREAD_ACP             3           // current thread's ANSI code page
#define CP_SYMBOL                 42          // SYMBOL translations

#define CP_UTF7                   65000       // UTF-7 translation
#define CP_UTF8                   65001       // UTF-8 translation

//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
//  Country/Region Codes.
//
//  DEPRECATED: The GEOID  concept is deprecated, please use
//  Country/Region Names instead, eg: "US" instead of a GEOID like 244.
//  See the documentation for GetGeoInfoEx.
//
//  WARNING: These values are arbitrarily assigned values, please use
//           standard country/region names instead, such as "US".
//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
#define CTRY_DEFAULT              0

#define CTRY_ALBANIA              355         // Albania
#define CTRY_ALGERIA              213         // Algeria
#define CTRY_ARGENTINA            54          // Argentina
#define CTRY_ARMENIA              374         // Armenia
#define CTRY_AUSTRALIA            61          // Australia
#define CTRY_AUSTRIA              43          // Austria
#define CTRY_AZERBAIJAN           994         // Azerbaijan
#define CTRY_BAHRAIN              973         // Bahrain
#define CTRY_BELARUS              375         // Belarus
#define CTRY_BELGIUM              32          // Belgium
#define CTRY_BELIZE               501         // Belize
#define CTRY_BOLIVIA              591         // Bolivia
#define CTRY_BRAZIL               55          // Brazil
#define CTRY_BRUNEI_DARUSSALAM    673         // Brunei Darussalam
#define CTRY_BULGARIA             359         // Bulgaria
#define CTRY_CANADA               2           // Canada
#define CTRY_CARIBBEAN            1           // Caribbean
#define CTRY_CHILE                56          // Chile
#define CTRY_COLOMBIA             57          // Colombia
#define CTRY_COSTA_RICA           506         // Costa Rica
#define CTRY_CROATIA              385         // Croatia
#define CTRY_CZECH                420         // Czech Republic
#define CTRY_DENMARK              45          // Denmark
#define CTRY_DOMINICAN_REPUBLIC   1           // Dominican Republic
#define CTRY_ECUADOR              593         // Ecuador
#define CTRY_EGYPT                20          // Egypt
#define CTRY_EL_SALVADOR          503         // El Salvador
#define CTRY_ESTONIA              372         // Estonia
#define CTRY_FAEROE_ISLANDS       298         // Faeroe Islands
#define CTRY_FINLAND              358         // Finland
#define CTRY_FRANCE               33          // France
#define CTRY_GEORGIA              995         // Georgia
#define CTRY_GERMANY              49          // Germany
#define CTRY_GREECE               30          // Greece
#define CTRY_GUATEMALA            502         // Guatemala
#define CTRY_HONDURAS             504         // Honduras
#define CTRY_HONG_KONG            852         // Hong Kong S.A.R., P.R.C.
#define CTRY_HUNGARY              36          // Hungary
#define CTRY_ICELAND              354         // Iceland
#define CTRY_INDIA                91          // India
#define CTRY_INDONESIA            62          // Indonesia
#define CTRY_IRAN                 981         // Iran
#define CTRY_IRAQ                 964         // Iraq
#define CTRY_IRELAND              353         // Ireland
#define CTRY_ISRAEL               972         // Israel
#define CTRY_ITALY                39          // Italy
#define CTRY_JAMAICA              1           // Jamaica
#define CTRY_JAPAN                81          // Japan
#define CTRY_JORDAN               962         // Jordan
#define CTRY_KAZAKSTAN            7           // Kazakstan
#define CTRY_KENYA                254         // Kenya
#define CTRY_KUWAIT               965         // Kuwait
#define CTRY_KYRGYZSTAN           996         // Kyrgyzstan
#define CTRY_LATVIA               371         // Latvia
#define CTRY_LEBANON              961         // Lebanon
#define CTRY_LIBYA                218         // Libya
#define CTRY_LIECHTENSTEIN        41          // Liechtenstein
#define CTRY_LITHUANIA            370         // Lithuania
#define CTRY_LUXEMBOURG           352         // Luxembourg
#define CTRY_MACAU                853         // Macao SAR, PRC
#define CTRY_MACEDONIA            389         // Former Yugoslav Republic of Macedonia
#define CTRY_MALAYSIA             60          // Malaysia
#define CTRY_MALDIVES             960         // Maldives
#define CTRY_MEXICO               52          // Mexico
#define CTRY_MONACO               33          // Principality of Monaco
#define CTRY_MONGOLIA             976         // Mongolia
#define CTRY_MOROCCO              212         // Morocco
#define CTRY_NETHERLANDS          31          // Netherlands
#define CTRY_NEW_ZEALAND          64          // New Zealand
#define CTRY_NICARAGUA            505         // Nicaragua
#define CTRY_NORWAY               47          // Norway
#define CTRY_OMAN                 968         // Oman
#define CTRY_PAKISTAN             92          // Islamic Republic of Pakistan
#define CTRY_PANAMA               507         // Panama
#define CTRY_PARAGUAY             595         // Paraguay
#define CTRY_PERU                 51          // Peru
#define CTRY_PHILIPPINES          63          // Republic of the Philippines
#define CTRY_POLAND               48          // Poland
#define CTRY_PORTUGAL             351         // Portugal
#define CTRY_PRCHINA              86          // People's Republic of China
#define CTRY_PUERTO_RICO          1           // Puerto Rico
#define CTRY_QATAR                974         // Qatar
#define CTRY_ROMANIA              40          // Romania
#define CTRY_RUSSIA               7           // Russia
#define CTRY_SAUDI_ARABIA         966         // Saudi Arabia
#define CTRY_SERBIA               381         // Serbia
#define CTRY_SINGAPORE            65          // Singapore
#define CTRY_SLOVAK               421         // Slovak Republic
#define CTRY_SLOVENIA             386         // Slovenia
#define CTRY_SOUTH_AFRICA         27          // South Africa
#define CTRY_SOUTH_KOREA          82          // Korea
#define CTRY_SPAIN                34          // Spain
#define CTRY_SWEDEN               46          // Sweden
#define CTRY_SWITZERLAND          41          // Switzerland
#define CTRY_SYRIA                963         // Syria
#define CTRY_TAIWAN               886         // Taiwan
#define CTRY_TATARSTAN            7           // Tatarstan
#define CTRY_THAILAND             66          // Thailand
#define CTRY_TRINIDAD_Y_TOBAGO    1           // Trinidad y Tobago
#define CTRY_TUNISIA              216         // Tunisia
#define CTRY_TURKEY               90          // Turkey
#define CTRY_UAE                  971         // U.A.E.
#define CTRY_UKRAINE              380         // Ukraine
#define CTRY_UNITED_KINGDOM       44          // United Kingdom
#define CTRY_UNITED_STATES        1           // United States
#define CTRY_URUGUAY              598         // Uruguay
#define CTRY_UZBEKISTAN           7           // Uzbekistan
#define CTRY_VENEZUELA            58          // Venezuela
#define CTRY_VIET_NAM             84          // Viet Nam
#define CTRY_YEMEN                967         // Yemen
#define CTRY_ZIMBABWE             263         // Zimbabwe


//
//  Locale Types.
//
//  These types are used for the GetLocaleInfo NLS API routine.
//  Some of these types are also used for the SetLocaleInfo NLS API routine.
//

//
//  The following LCTypes may be used in combination with any other LCTypes.
//
//    LOCALE_NOUSEROVERRIDE is also used in GetTimeFormat and
//    GetDateFormat.
//
//    LOCALE_RETURN_NUMBER will return the result from GetLocaleInfo as a
//    number instead of a string.  This flag is only valid for the LCTypes
//    beginning with LOCALE_I.
//
//    DEPRECATED: LOCALE_USE_CP_ACP is used in many of the A (Ansi) apis that need
//                to do string translation.  Callers are encouraged to use the W
//                (WCHAR/Unicode) apis instead.
//
#define LOCALE_NOUSEROVERRIDE         0x80000000   // Not Recommended - do not use user overrides
#define LOCALE_USE_CP_ACP             0x40000000   // DEPRECATED, call Unicode APIs instead: use the system ACP

#if(WINVER >= 0x0400)
#define LOCALE_RETURN_NUMBER          0x20000000   // return number instead of string
#endif /* WINVER >= 0x0400 */

#if (WINVER >= _WIN32_WINNT_WIN7)
#define LOCALE_RETURN_GENITIVE_NAMES  0x10000000   //Flag to return the Genitive forms of month names
#define LOCALE_ALLOW_NEUTRAL_NAMES    0x08000000   //Flag to allow returning neutral names/lcids for name conversion
#endif //(WINVER >= _WIN32_WINNT_WIN7)


//
//  The following LCTypes are mutually exclusive in that they may NOT
//  be used in combination with each other.
//

//
// These are the various forms of the name of the locale:
//
#define LOCALE_SLOCALIZEDDISPLAYNAME  0x00000002   // localized name of locale, eg "German (Germany)" in UI language
#if (WINVER >= _WIN32_WINNT_WIN7)
#define LOCALE_SENGLISHDISPLAYNAME    0x00000072   // Display name (language + country/region usually) in English, eg "German (Germany)"
#define LOCALE_SNATIVEDISPLAYNAME     0x00000073   // Display name in native locale language, eg "Deutsch (Deutschland)
#endif //(WINVER >= _WIN32_WINNT_WIN7)

#if (WINVER >= _WIN32_WINNT_VISTA)
#define LOCALE_SLOCALIZEDLANGUAGENAME 0x0000006f   // Language Display Name for a language, eg "German" in UI language
#endif //(WINVER >= _WIN32_WINNT_VISTA)
#define LOCALE_SENGLISHLANGUAGENAME   0x00001001   // English name of language, eg "German"
#define LOCALE_SNATIVELANGUAGENAME    0x00000004   // native name of language, eg "Deutsch"

#define LOCALE_SLOCALIZEDCOUNTRYNAME  0x00000006   // localized name of country/region, eg "Germany" in UI language
#define LOCALE_SENGLISHCOUNTRYNAME    0x00001002   // English name of country/region, eg "Germany"
#define LOCALE_SNATIVECOUNTRYNAME     0x00000008   // native name of country/region, eg "Deutschland"

// Additional LCTypes
#define LOCALE_IDIALINGCODE           0x00000005   // country/region dialing code, example: en-US and en-CA return 1.

#define LOCALE_SLIST                  0x0000000C   // list item separator, eg "," for "1,2,3,4"
#define LOCALE_IMEASURE               0x0000000D   // 0 = metric, 1 = US measurement system

#define LOCALE_SDECIMAL               0x0000000E   // decimal separator, eg "." for 1,234.00
#define LOCALE_STHOUSAND              0x0000000F   // thousand separator, eg "," for 1,234.00
#define LOCALE_SGROUPING              0x00000010   // digit grouping, eg "3;0" for 1,000,000
#define LOCALE_IDIGITS                0x00000011   // number of fractional digits eg 2 for 1.00
#define LOCALE_ILZERO                 0x00000012   // leading zeros for decimal, 0 for .97, 1 for 0.97
#define LOCALE_INEGNUMBER             0x00001010   // negative number mode, 0-4, see documentation
#define LOCALE_SNATIVEDIGITS          0x00000013   // native digits for 0-9, eg "0123456789"

#define LOCALE_SCURRENCY              0x00000014   // local monetary symbol, eg "$"
#define LOCALE_SINTLSYMBOL            0x00000015   // intl monetary symbol, eg "USD"
#define LOCALE_SMONDECIMALSEP         0x00000016   // monetary decimal separator, eg "." for $1,234.00
#define LOCALE_SMONTHOUSANDSEP        0x00000017   // monetary thousand separator, eg "," for $1,234.00
#define LOCALE_SMONGROUPING           0x00000018   // monetary grouping, eg "3;0" for $1,000,000.00
#define LOCALE_ICURRDIGITS            0x00000019   // # local monetary digits, eg 2 for $1.00
#define LOCALE_ICURRENCY              0x0000001B   // positive currency mode, 0-3, see documentation
#define LOCALE_INEGCURR               0x0000001C   // negative currency mode, 0-15, see documentation

#define LOCALE_SSHORTDATE             0x0000001F   // short date format string, eg "MM/dd/yyyy"
#define LOCALE_SLONGDATE              0x00000020   // long date format string, eg "dddd, MMMM dd, yyyy"
#define LOCALE_STIMEFORMAT            0x00001003   // time format string, eg "HH:mm:ss"
#define LOCALE_SAM                    0x00000028   // AM designator, eg "AM"
#define LOCALE_SPM                    0x00000029   // PM designator, eg "PM"

#define LOCALE_ICALENDARTYPE          0x00001009   // type of calendar specifier, eg CAL_GREGORIAN
#define LOCALE_IOPTIONALCALENDAR      0x0000100B   // additional calendar types specifier, eg CAL_GREGORIAN_US
#define LOCALE_IFIRSTDAYOFWEEK        0x0000100C   // first day of week specifier, 0-6, 0=Monday, 6=Sunday
#define LOCALE_IFIRSTWEEKOFYEAR       0x0000100D   // first week of year specifier, 0-2, see documentation

#define LOCALE_SDAYNAME1              0x0000002A   // long name for Monday
#define LOCALE_SDAYNAME2              0x0000002B   // long name for Tuesday
#define LOCALE_SDAYNAME3              0x0000002C   // long name for Wednesday
#define LOCALE_SDAYNAME4              0x0000002D   // long name for Thursday
#define LOCALE_SDAYNAME5              0x0000002E   // long name for Friday
#define LOCALE_SDAYNAME6              0x0000002F   // long name for Saturday
#define LOCALE_SDAYNAME7              0x00000030   // long name for Sunday
#define LOCALE_SABBREVDAYNAME1        0x00000031   // abbreviated name for Monday
#define LOCALE_SABBREVDAYNAME2        0x00000032   // abbreviated name for Tuesday
#define LOCALE_SABBREVDAYNAME3        0x00000033   // abbreviated name for Wednesday
#define LOCALE_SABBREVDAYNAME4        0x00000034   // abbreviated name for Thursday
#define LOCALE_SABBREVDAYNAME5        0x00000035   // abbreviated name for Friday
#define LOCALE_SABBREVDAYNAME6        0x00000036   // abbreviated name for Saturday
#define LOCALE_SABBREVDAYNAME7        0x00000037   // abbreviated name for Sunday
#define LOCALE_SMONTHNAME1            0x00000038   // long name for January
#define LOCALE_SMONTHNAME2            0x00000039   // long name for February
#define LOCALE_SMONTHNAME3            0x0000003A   // long name for March
#define LOCALE_SMONTHNAME4            0x0000003B   // long name for April
#define LOCALE_SMONTHNAME5            0x0000003C   // long name for May
#define LOCALE_SMONTHNAME6            0x0000003D   // long name for June
#define LOCALE_SMONTHNAME7            0x0000003E   // long name for July
#define LOCALE_SMONTHNAME8            0x0000003F   // long name for August
#define LOCALE_SMONTHNAME9            0x00000040   // long name for September
#define LOCALE_SMONTHNAME10           0x00000041   // long name for October
#define LOCALE_SMONTHNAME11           0x00000042   // long name for November
#define LOCALE_SMONTHNAME12           0x00000043   // long name for December
#define LOCALE_SMONTHNAME13           0x0000100E   // long name for 13th month (if exists)
#define LOCALE_SABBREVMONTHNAME1      0x00000044   // abbreviated name for January
#define LOCALE_SABBREVMONTHNAME2      0x00000045   // abbreviated name for February
#define LOCALE_SABBREVMONTHNAME3      0x00000046   // abbreviated name for March
#define LOCALE_SABBREVMONTHNAME4      0x00000047   // abbreviated name for April
#define LOCALE_SABBREVMONTHNAME5      0x00000048   // abbreviated name for May
#define LOCALE_SABBREVMONTHNAME6      0x00000049   // abbreviated name for June
#define LOCALE_SABBREVMONTHNAME7      0x0000004A   // abbreviated name for July
#define LOCALE_SABBREVMONTHNAME8      0x0000004B   // abbreviated name for August
#define LOCALE_SABBREVMONTHNAME9      0x0000004C   // abbreviated name for September
#define LOCALE_SABBREVMONTHNAME10     0x0000004D   // abbreviated name for October
#define LOCALE_SABBREVMONTHNAME11     0x0000004E   // abbreviated name for November
#define LOCALE_SABBREVMONTHNAME12     0x0000004F   // abbreviated name for December
#define LOCALE_SABBREVMONTHNAME13     0x0000100F   // abbreviated name for 13th month (if exists)

#define LOCALE_SPOSITIVESIGN          0x00000050   // positive sign, eg ""
#define LOCALE_SNEGATIVESIGN          0x00000051   // negative sign, eg "-"
#define LOCALE_IPOSSIGNPOSN           0x00000052   // positive sign position (derived from INEGCURR)
#define LOCALE_INEGSIGNPOSN           0x00000053   // negative sign position (derived from INEGCURR)
#define LOCALE_IPOSSYMPRECEDES        0x00000054   // mon sym precedes pos amt (derived from ICURRENCY)
#define LOCALE_IPOSSEPBYSPACE         0x00000055   // mon sym sep by space from pos amt (derived from ICURRENCY)
#define LOCALE_INEGSYMPRECEDES        0x00000056   // mon sym precedes neg amt (derived from INEGCURR)
#define LOCALE_INEGSEPBYSPACE         0x00000057   // mon sym sep by space from neg amt (derived from INEGCURR)

#if(WINVER >= 0x0400)
#define LOCALE_FONTSIGNATURE          0x00000058   // font signature
#define LOCALE_SISO639LANGNAME        0x00000059   // ISO abbreviated language name, eg "en"
#define LOCALE_SISO3166CTRYNAME       0x0000005A   // ISO abbreviated country/region name, eg "US"
#endif /* WINVER >= 0x0400 */

#if(WINVER >= 0x0500)
#define LOCALE_IPAPERSIZE             0x0000100A   // 1 = letter, 5 = legal, 8 = a3, 9 = a4
#define LOCALE_SENGCURRNAME           0x00001007   // english name of currency, eg "Euro"
#define LOCALE_SNATIVECURRNAME        0x00001008   // native name of currency, eg "euro"
#define LOCALE_SYEARMONTH             0x00001006   // year month format string, eg "MM/yyyy"
#define LOCALE_SSORTNAME              0x00001013   // sort name, usually "", eg "Dictionary" in UI Language
#define LOCALE_IDIGITSUBSTITUTION     0x00001014   // 0 = context, 1 = none, 2 = national

#endif /* WINVER >= 0x0500 */

#if (WINVER >= 0x0600)
#define LOCALE_SNAME                  0x0000005c   // locale name (ie: en-us)
#define LOCALE_SDURATION              0x0000005d   // time duration format, eg "hh:mm:ss"
#define LOCALE_SSHORTESTDAYNAME1      0x00000060   // Shortest day name for Monday
#define LOCALE_SSHORTESTDAYNAME2      0x00000061   // Shortest day name for Tuesday
#define LOCALE_SSHORTESTDAYNAME3      0x00000062   // Shortest day name for Wednesday
#define LOCALE_SSHORTESTDAYNAME4      0x00000063   // Shortest day name for Thursday
#define LOCALE_SSHORTESTDAYNAME5      0x00000064   // Shortest day name for Friday
#define LOCALE_SSHORTESTDAYNAME6      0x00000065   // Shortest day name for Saturday
#define LOCALE_SSHORTESTDAYNAME7      0x00000066   // Shortest day name for Sunday
#define LOCALE_SISO639LANGNAME2       0x00000067   // 3 character ISO abbreviated language name, eg "eng"
#define LOCALE_SISO3166CTRYNAME2      0x00000068   // 3 character ISO country/region name, eg "USA"
#define LOCALE_SNAN                   0x00000069   // Not a Number, eg "NaN"
#define LOCALE_SPOSINFINITY           0x0000006a   // + Infinity, eg "infinity"
#define LOCALE_SNEGINFINITY           0x0000006b   // - Infinity, eg "-infinity"
#define LOCALE_SSCRIPTS               0x0000006c   // Typical scripts in the locale: ; delimited script codes, eg "Latn;"
#define LOCALE_SPARENT                0x0000006d   // Fallback name for resources, eg "en" for "en-US"
#define LOCALE_SCONSOLEFALLBACKNAME   0x0000006e   // Fallback name for within the console for Unicode Only locales, eg "en" for bn-IN
#endif //(WINVER >= 0x0600)

#if (WINVER >= _WIN32_WINNT_WIN7)
#define LOCALE_IREADINGLAYOUT         0x00000070   // Returns one of the following 4 reading layout values:
                                                   // 0 - Left to right (eg en-US)
                                                   // 1 - Right to left (eg arabic locales)
                                                   // 2 - Vertical top to bottom with columns to the left and also left to right (ja-JP locales)
                                                   // 3 - Vertical top to bottom with columns proceeding to the right
#define LOCALE_INEUTRAL               0x00000071   // Returns 0 for specific cultures, 1 for neutral cultures.
#define LOCALE_INEGATIVEPERCENT       0x00000074   // Returns 0-11 for the negative percent format
#define LOCALE_IPOSITIVEPERCENT       0x00000075   // Returns 0-3 for the positive percent formatIPOSITIVEPERCENT
#define LOCALE_SPERCENT               0x00000076   // Returns the percent symbol
#define LOCALE_SPERMILLE              0x00000077   // Returns the permille (U+2030) symbol
#define LOCALE_SMONTHDAY              0x00000078   // Returns the preferred month/day format
#define LOCALE_SSHORTTIME             0x00000079   // Returns the preferred short time format (ie: no seconds, just h:mm)
#define LOCALE_SOPENTYPELANGUAGETAG   0x0000007a   // Open type language tag, eg: "latn" or "dflt"
#define LOCALE_SSORTLOCALE            0x0000007b   // Name of locale to use for sorting/collation/casing behavior.
#endif //(WINVER >= _WIN32_WINNT_WIN7)

#if (WINVER >= _WIN32_WINNT_WIN8)
#define LOCALE_SRELATIVELONGDATE      0x0000007c   // Long date without year, day of week, month, date, eg: for lock screen
#endif
#if (WINVER >= _WIN32_WINNT_WINBLUE)
#define LOCALE_ICONSTRUCTEDLOCALE     0x0000007d   // Flags if this locale is constructed.  Avoid using.
#endif

#if (WINVER >= _WIN32_WINNT_WIN10)
#define LOCALE_SSHORTESTAM            0x0000007e   // Shortest AM designator, eg "A"
#define LOCALE_SSHORTESTPM            0x0000007f   // Shortest PM designator, eg "P"
#endif


#if (NTDDI_VERSION >= NTDDI_WIN10_MN)
#define LOCALE_IUSEUTF8LEGACYACP     0x00000666   // default ansi code page (use of Unicode is recommended instead)
#define LOCALE_IUSEUTF8LEGACYOEMCP   0x00000999   // default oem code page (use of Unicode is recommended instead)
#endif

//
// DEPRECATED LCTYPEs
//

// DEPRECATED LCTYPEs for Code Pages
// Applications are strongly encouraged to Use Unicode, such as UTF-16 (WCHAR type)
// or the CP_UTF8 Code Page.  Legacy encodings are unable to represent the full
// set of scripts/language and characters (& emoji!) available on modern computers.
// Use of legacy code pages (encodings) is a leading cause of data loss and corruption.
#define LOCALE_IDEFAULTCODEPAGE       0x0000000B   // default oem code page for locale (user may configure as UTF-8, use of Unicode is recommended instead)
#define LOCALE_IDEFAULTANSICODEPAGE   0x00001004   // default ansi code page for locale (user may configure as UTF-8, use of Unicode is recommended instead)
#define LOCALE_IDEFAULTMACCODEPAGE    0x00001011   // default mac code page for locale (user may configure as UTF-8, use of Unicode is recommended instead)
#if(WINVER >= 0x0500)
#define LOCALE_IDEFAULTEBCDICCODEPAGE 0x00001012   // default ebcdic code page for a locale (use of Unicode is recommended instead)
#endif /* WINVER >= 0x0500 */

// LCTYPEs using out-of-date concepts
#define LOCALE_ILANGUAGE              0x00000001   // DEPRECATED language id (LCID), LOCALE_SNAME preferred
#define LOCALE_SABBREVLANGNAME        0x00000003   // DEPRECATED arbitrary abbreviated language name, LOCALE_SISO639LANGNAME instead.
#define LOCALE_SABBREVCTRYNAME        0x00000007   // DEPRECATED arbitrary abbreviated country/region name, LOCALE_SISO3166CTRYNAME instead.
#define LOCALE_IGEOID                 0x0000005B   // DEPRECATED geographical location id, use LOCALE_SISO3166CTRYNAME instead.
#define LOCALE_IDEFAULTLANGUAGE       0x00000009   // DEPRECATED default language id, deprecated
#define LOCALE_IDEFAULTCOUNTRY        0x0000000A   // DEPRECATED default country/region code, deprecated
#define LOCALE_IINTLCURRDIGITS        0x0000001A   // DEPRECATED, use LOCALE_ICURRDIGITS # intl monetary digits, eg 2 for $1.00

// Derived legacy date & time values for compatibility only.
// Please use the appropriate date or time pattern instead.
// These can be misleading, for example a locale configured as 12h24m52s could have a time separator of "h".
#define LOCALE_SDATE                  0x0000001D   // DEPRECATED date separator (derived from LOCALE_SSHORTDATE, use that instead)
#define LOCALE_STIME                  0x0000001E   // DEPRECATED time separator (derived from LOCALE_STIMEFORMAT, use that instead)
#define LOCALE_IDATE                  0x00000021   // DEPRECATED short date format ordering (derived from LOCALE_SSHORTDATE, use that instead)
#define LOCALE_ILDATE                 0x00000022   // DEPRECATED long date format ordering (derived from LOCALE_SLONGDATE, use that instead)
#define LOCALE_ITIME                  0x00000023   // DEPRECATED time format specifier (derived from LOCALE_STIMEFORMAT, use that instead)
#define LOCALE_ITIMEMARKPOSN          0x00001005   // DEPRECATED time marker position (derived from LOCALE_STIMEFORMAT, use that instead)
#define LOCALE_ICENTURY               0x00000024   // DEPRECATED century format specifier (short date, LOCALE_SSHORTDATE is preferred)
#define LOCALE_ITLZERO                0x00000025   // DEPRECATED leading zeros in time field (derived from LOCALE_STIMEFORMAT, use that instead)
#define LOCALE_IDAYLZERO              0x00000026   // DEPRECATED leading zeros in day field (short date, LOCALE_SSHORTDATE is preferred)
#define LOCALE_IMONLZERO              0x00000027   // DEPRECATED leading zeros in month field (short date, LOCALE_SSHORTDATE is preferred)

#if(WINVER >= 0x0600)
#define LOCALE_SKEYBOARDSTOINSTALL    0x0000005e   // Used internally, see GetKeyboardLayoutName() function
#endif /* WINVER >= 0x0600 */

// LCTYPEs which have been renamed to enable more understandable source code.
#define LOCALE_SLANGUAGE              LOCALE_SLOCALIZEDDISPLAYNAME   // DEPRECATED as new name is more readable.
#if (WINVER >= _WIN32_WINNT_VISTA)
#define LOCALE_SLANGDISPLAYNAME       LOCALE_SLOCALIZEDLANGUAGENAME  // DEPRECATED as new name is more readable.
#endif //(WINVER >= _WIN32_WINNT_VISTA)
#define LOCALE_SENGLANGUAGE           LOCALE_SENGLISHLANGUAGENAME    // DEPRECATED as new name is more readable.
#define LOCALE_SNATIVELANGNAME        LOCALE_SNATIVELANGUAGENAME     // DEPRECATED as new name is more readable.
#define LOCALE_SCOUNTRY               LOCALE_SLOCALIZEDCOUNTRYNAME   // DEPRECATED as new name is more readable.
#define LOCALE_SENGCOUNTRY            LOCALE_SENGLISHCOUNTRYNAME     // DEPRECATED as new name is more readable.
#define LOCALE_SNATIVECTRYNAME        LOCALE_SNATIVECOUNTRYNAME      // DEPRECATED as new name is more readable.
// DEPRECATED: Use LOCALE_SISO3166CTRYNAME to query for a region identifier, LOCALE_ICOUNTRY is not a region identifier.
#define LOCALE_ICOUNTRY               LOCALE_IDIALINGCODE   // Deprecated synonym for LOCALE_IDIALINGCODE
#define LOCALE_S1159                  LOCALE_SAM   // DEPRECATED: Please use LOCALE_SAM, which is more readable.
#define LOCALE_S2359                  LOCALE_SPM   // DEPRECATED: Please use LOCALE_SPM, which is more readable.


//
//  Time Flags for GetTimeFormat.
//
#define TIME_NOMINUTESORSECONDS   0x00000001  // do not use minutes or seconds
#define TIME_NOSECONDS            0x00000002  // do not use seconds
#define TIME_NOTIMEMARKER         0x00000004  // do not use time marker
#define TIME_FORCE24HOURFORMAT    0x00000008  // always use 24 hour format


//
//  Date Flags for GetDateFormat.
//
#define DATE_SHORTDATE            0x00000001  // use short date picture
#define DATE_LONGDATE             0x00000002  // use long date picture
#define DATE_USE_ALT_CALENDAR     0x00000004  // use alternate calendar (if any)

#if(WINVER >= 0x0500)
#define DATE_YEARMONTH            0x00000008  // use year month picture
#define DATE_LTRREADING           0x00000010  // add marks for left to right reading order layout
#define DATE_RTLREADING           0x00000020  // add marks for right to left reading order layout
#endif /* WINVER >= 0x0500 */

#if (WINVER >= _WIN32_WINNT_WIN7)
#define DATE_AUTOLAYOUT           0x00000040  // add appropriate marks for left-to-right or right-to-left reading order layout
#endif //(WINVER >= _WIN32_WINNT_WIN7)

#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD)
#define DATE_MONTHDAY             0x00000080  // include month day pictures
#endif //(WINVER >= _WIN32_WINNT_WINTHRESHOLD)

//
//  Calendar Types.
//
//  These types are used for the EnumCalendarInfo and GetCalendarInfo
//  NLS API routines.
//  Some of these types are also used for the SetCalendarInfo NLS API
//  routine.
//

//
//  The following CalTypes may be used in combination with any other CalTypes.
//
//    CAL_NOUSEROVERRIDE
//
//    CAL_RETURN_NUMBER will return the result from GetCalendarInfo as a
//    number instead of a string.  This flag is only valid for the CalTypes
//    beginning with CAL_I.
//
//    DEPRECATED: CAL_USE_CP_ACP is used in many of the A (Ansi) apis that need
//                to do string translation.  Callers are encouraged to use the W
//                (WCHAR/Unicode) apis instead.
//
#if(WINVER >= 0x0500)
#define CAL_NOUSEROVERRIDE        LOCALE_NOUSEROVERRIDE         // Not Recommended - do not use user overrides
#define CAL_USE_CP_ACP            LOCALE_USE_CP_ACP             // DEPRECATED, call Unicode APIs instead: use the system ACP
#define CAL_RETURN_NUMBER         LOCALE_RETURN_NUMBER          // return number instead of string
#endif /* WINVER >= 0x0500 */


#if (WINVER >= _WIN32_WINNT_WIN7)
#define CAL_RETURN_GENITIVE_NAMES LOCALE_RETURN_GENITIVE_NAMES  // return genitive forms of month names
#endif // winver >= windows 7

//
//  The following CalTypes are mutually exclusive in that they may NOT
//  be used in combination with each other.
//
#define CAL_ICALINTVALUE          0x00000001  // calendar type
#define CAL_SCALNAME              0x00000002  // native name of calendar
#define CAL_IYEAROFFSETRANGE      0x00000003  // starting years of eras
#define CAL_SERASTRING            0x00000004  // era name for IYearOffsetRanges, eg A.D.
#define CAL_SSHORTDATE            0x00000005  // short date format string
#define CAL_SLONGDATE             0x00000006  // long date format string
#define CAL_SDAYNAME1             0x00000007  // native name for Monday
#define CAL_SDAYNAME2             0x00000008  // native name for Tuesday
#define CAL_SDAYNAME3             0x00000009  // native name for Wednesday
#define CAL_SDAYNAME4             0x0000000a  // native name for Thursday
#define CAL_SDAYNAME5             0x0000000b  // native name for Friday
#define CAL_SDAYNAME6             0x0000000c  // native name for Saturday
#define CAL_SDAYNAME7             0x0000000d  // native name for Sunday
#define CAL_SABBREVDAYNAME1       0x0000000e  // abbreviated name for Mon
#define CAL_SABBREVDAYNAME2       0x0000000f  // abbreviated name for Tue
#define CAL_SABBREVDAYNAME3       0x00000010  // abbreviated name for Wed
#define CAL_SABBREVDAYNAME4       0x00000011  // abbreviated name for Thu
#define CAL_SABBREVDAYNAME5       0x00000012  // abbreviated name for Fri
#define CAL_SABBREVDAYNAME6       0x00000013  // abbreviated name for Sat
#define CAL_SABBREVDAYNAME7       0x00000014  // abbreviated name for Sun
// Note that in the hebrew calendar the leap month name is always returned as the 7th month
#define CAL_SMONTHNAME1           0x00000015  // native name for January
#define CAL_SMONTHNAME2           0x00000016  // native name for February
#define CAL_SMONTHNAME3           0x00000017  // native name for March
#define CAL_SMONTHNAME4           0x00000018  // native name for April
#define CAL_SMONTHNAME5           0x00000019  // native name for May
#define CAL_SMONTHNAME6           0x0000001a  // native name for June
#define CAL_SMONTHNAME7           0x0000001b  // native name for July
#define CAL_SMONTHNAME8           0x0000001c  // native name for August
#define CAL_SMONTHNAME9           0x0000001d  // native name for September
#define CAL_SMONTHNAME10          0x0000001e  // native name for October
#define CAL_SMONTHNAME11          0x0000001f  // native name for November
#define CAL_SMONTHNAME12          0x00000020  // native name for December
#define CAL_SMONTHNAME13          0x00000021  // native name for 13th month (if any)
#define CAL_SABBREVMONTHNAME1     0x00000022  // abbreviated name for Jan
#define CAL_SABBREVMONTHNAME2     0x00000023  // abbreviated name for Feb
#define CAL_SABBREVMONTHNAME3     0x00000024  // abbreviated name for Mar
#define CAL_SABBREVMONTHNAME4     0x00000025  // abbreviated name for Apr
#define CAL_SABBREVMONTHNAME5     0x00000026  // abbreviated name for May
#define CAL_SABBREVMONTHNAME6     0x00000027  // abbreviated name for Jun
#define CAL_SABBREVMONTHNAME7     0x00000028  // abbreviated name for July
#define CAL_SABBREVMONTHNAME8     0x00000029  // abbreviated name for Aug
#define CAL_SABBREVMONTHNAME9     0x0000002a  // abbreviated name for Sep
#define CAL_SABBREVMONTHNAME10    0x0000002b  // abbreviated name for Oct
#define CAL_SABBREVMONTHNAME11    0x0000002c  // abbreviated name for Nov
#define CAL_SABBREVMONTHNAME12    0x0000002d  // abbreviated name for Dec
#define CAL_SABBREVMONTHNAME13    0x0000002e  // abbreviated name for 13th month (if any)

#if(WINVER >= 0x0500)
#define CAL_SYEARMONTH            0x0000002f  // year month format string
#define CAL_ITWODIGITYEARMAX      0x00000030  // two digit year max
#endif /* WINVER >= 0x0500 */

#if (WINVER >= 0x0600)
#define CAL_SSHORTESTDAYNAME1     0x00000031  // Shortest day name for Mo
#define CAL_SSHORTESTDAYNAME2     0x00000032  // Shortest day name for Tu
#define CAL_SSHORTESTDAYNAME3     0x00000033  // Shortest day name for We
#define CAL_SSHORTESTDAYNAME4     0x00000034  // Shortest day name for Th
#define CAL_SSHORTESTDAYNAME5     0x00000035  // Shortest day name for Fr
#define CAL_SSHORTESTDAYNAME6     0x00000036  // Shortest day name for Sa
#define CAL_SSHORTESTDAYNAME7     0x00000037  // Shortest day name for Su
#endif //(WINVER >= 0x0600)

#if (WINVER >= _WIN32_WINNT_WIN7)
#define CAL_SMONTHDAY             0x00000038  // Month/day format
#define CAL_SABBREVERASTRING      0x00000039  // Abbreviated era string (eg: AD)
#endif // winver >= windows 7

#if (WINVER >= _WIN32_WINNT_WIN8)
#define CAL_SRELATIVELONGDATE     0x0000003a   // Long date without year, day of week, month, date, eg: for lock screen
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#define CAL_SENGLISHERANAME       0x0000003b   // Japanese calendar only: return the English era names for .Net compatibility
#define CAL_SENGLISHABBREVERANAME 0x0000003c   // Japanese calendar only: return the English Abbreviated era names for .Net compatibility
#endif

// CAL_SJAPANESEERAFIRSTYEAR is only supported on machines with updates to support the "gannen" era first year behavior
// Machines without that update will return 0 and ERROR_INVALID_FLAGS, in which case ichinen is presumed.
#define CAL_SJAPANESEERAFIRSTYEAR 0x0000003d   // Japanese calendar only: return ichinen or gannen first year

//
//  Calendar Enumeration Value.
//
#define ENUM_ALL_CALENDARS        0xffffffff  // enumerate all calendars

//
//  Calendar ID Values.
//
#define CAL_GREGORIAN                  1      // Gregorian (localized) calendar
#define CAL_GREGORIAN_US               2      // Gregorian (U.S.) calendar
#define CAL_JAPAN                      3      // Japanese Emperor Era calendar
#define CAL_TAIWAN                     4      // Taiwan calendar
#define CAL_KOREA                      5      // Korean Tangun Era calendar
#define CAL_HIJRI                      6      // Hijri (Arabic Lunar) calendar
#define CAL_THAI                       7      // Thai calendar
#define CAL_HEBREW                     8      // Hebrew (Lunar) calendar
#define CAL_GREGORIAN_ME_FRENCH        9      // Gregorian Middle East French calendar
#define CAL_GREGORIAN_ARABIC           10     // Gregorian Arabic calendar
#define CAL_GREGORIAN_XLIT_ENGLISH     11     // Gregorian Transliterated English calendar
#define CAL_GREGORIAN_XLIT_FRENCH      12     // Gregorian Transliterated French calendar
#define CAL_PERSIAN                    22     // Persian (Solar Hijri) calendar
#define CAL_UMALQURA                   23     // UmAlQura Hijri (Arabic Lunar) calendar

//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
//  Language Group ID Values
//
// The "Language Group" concept is an obsolete concept.
// The groups returned are not well defined, arbitrary, inconsistent, inaccurate,
// no longer maintained, and no longer supported.
//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
#define LGRPID_WESTERN_EUROPE        0x0001   // Western Europe & U.S.
#define LGRPID_CENTRAL_EUROPE        0x0002   // Central Europe
#define LGRPID_BALTIC                0x0003   // Baltic
#define LGRPID_GREEK                 0x0004   // Greek
#define LGRPID_CYRILLIC              0x0005   // Cyrillic
#define LGRPID_TURKIC                0x0006   // Turkic
#define LGRPID_TURKISH               0x0006   // Turkish
#define LGRPID_JAPANESE              0x0007   // Japanese
#define LGRPID_KOREAN                0x0008   // Korean
#define LGRPID_TRADITIONAL_CHINESE   0x0009   // Traditional Chinese
#define LGRPID_SIMPLIFIED_CHINESE    0x000a   // Simplified Chinese
#define LGRPID_THAI                  0x000b   // Thai
#define LGRPID_HEBREW                0x000c   // Hebrew
#define LGRPID_ARABIC                0x000d   // Arabic
#define LGRPID_VIETNAMESE            0x000e   // Vietnamese
#define LGRPID_INDIC                 0x000f   // Indic
#define LGRPID_GEORGIAN              0x0010   // Georgian
#define LGRPID_ARMENIAN              0x0011   // Armenian


#if (WINVER >= 0x0600)
//
//  MUI function flag values
//
#define MUI_LANGUAGE_ID                     0x4      // Use traditional language ID convention
#define MUI_LANGUAGE_NAME                   0x8      // Use ISO language (culture) name convention
#define MUI_MERGE_SYSTEM_FALLBACK           0x10     // GetThreadPreferredUILanguages merges in parent and base languages
#define MUI_MERGE_USER_FALLBACK             0x20     // GetThreadPreferredUILanguages merges in user preferred languages
#define MUI_UI_FALLBACK                     MUI_MERGE_SYSTEM_FALLBACK | MUI_MERGE_USER_FALLBACK
#define MUI_THREAD_LANGUAGES                0x40     // GetThreadPreferredUILanguages merges in user preferred languages
#define MUI_CONSOLE_FILTER                  0x100    // SetThreadPreferredUILanguages takes on console specific behavior
#define MUI_COMPLEX_SCRIPT_FILTER           0x200    // SetThreadPreferredUILanguages takes on complex script specific behavior
#define MUI_RESET_FILTERS                   0x001    // Reset MUI_CONSOLE_FILTER and MUI_COMPLEX_SCRIPT_FILTER
#define MUI_USER_PREFERRED_UI_LANGUAGES     0x10     // GetFileMUIPath returns the MUI files for the languages in the fallback list
#define MUI_USE_INSTALLED_LANGUAGES         0x20     // GetFileMUIPath returns all the MUI files installed in the machine
#define MUI_USE_SEARCH_ALL_LANGUAGES        0x40     // GetFileMUIPath returns all the MUI files irrespective of whether language is installed
#define MUI_LANG_NEUTRAL_PE_FILE            0x100    // GetFileMUIPath returns target file with .mui extension
#define MUI_NON_LANG_NEUTRAL_FILE           0x200    // GetFileMUIPath returns target file with same name as source
#define MUI_MACHINE_LANGUAGE_SETTINGS       0x400
#define MUI_FILETYPE_NOT_LANGUAGE_NEUTRAL   0x001   // GetFileMUIInfo found a non-split resource file
#define MUI_FILETYPE_LANGUAGE_NEUTRAL_MAIN  0x002   // GetFileMUIInfo found a LN main module resource file
#define MUI_FILETYPE_LANGUAGE_NEUTRAL_MUI   0x004   // GetFileMUIInfo found a LN MUI module resource file
#define MUI_QUERY_TYPE                      0x001   // GetFileMUIInfo will look for the type of the resource file
#define MUI_QUERY_CHECKSUM                  0x002   // GetFileMUIInfo will look for the checksum of the resource file
#define MUI_QUERY_LANGUAGE_NAME             0x004   // GetFileMUIInfo will look for the culture of the resource file
#define MUI_QUERY_RESOURCE_TYPES            0x008   // GetFileMUIInfo will look for the resource types of the resource file
#define MUI_FILEINFO_VERSION                0x001   // Version of FILEMUIINFO structure used with GetFileMUIInfo

#define MUI_FULL_LANGUAGE               0x01
#define MUI_PARTIAL_LANGUAGE            0x02
#define MUI_LIP_LANGUAGE                0x04
#define MUI_LANGUAGE_INSTALLED          0x20
#define MUI_LANGUAGE_LICENSED           0x40

//
// MUI_CALLBACK_FLAG defines are duplicated in rtlmui.h
//

#define MUI_CALLBACK_ALL_FLAGS                        MUI_CALLBACK_FLAG_UPGRADED_INSTALLATION // OR all other flags when defined.

//
// MUI_CALLBACK_ flags are duplicated in rtlmui.h
//

#endif

////////////////////////////////////////////////////////////////////////////
//
//  Typedefs
//
//  Define all types for the NLS component here.
//
////////////////////////////////////////////////////////////////////////////

//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
//  Language Group ID 
//
// The "Language Group" concept is an obsolete concept.
// The groups returned are not well defined, arbitrary, inconsistent, inaccurate,
// no longer maintained, and no longer supported.
//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
typedef DWORD LGRPID;

//
//  Locale type constant.
//
typedef DWORD LCTYPE;

//
//  Calendar type constant.
//
typedef DWORD CALTYPE;


//
//  Calendar ID.
//
typedef DWORD CALID;


//
//  CP Info.
//
// Deprecated.  Applications should use Unicode (WCHAR / UTF-16 or UTF-8)
//
// WARNING: These structures fail for some encodings, including UTF-8, which
//          do not fit into the assumptions of these APIs.
//

DEPRECATED("Use Unicode. The information in this structure cannot represent all encodings accurately and may be unreliable on many machines. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef struct _cpinfo {
    UINT    MaxCharSize;                    // max length (in bytes) of a char
    BYTE    DefaultChar[MAX_DEFAULTCHAR];   // default character
    BYTE    LeadByte[MAX_LEADBYTES];        // lead byte ranges
} CPINFO, *LPCPINFO;

//
//  GEO defines
//
typedef DWORD   GEOTYPE;
typedef DWORD   GEOCLASS;

//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
//  DEPRECATED: The GEOID  concept is deprecated, please use
//  Country/Region Names instead, eg: "US" instead of a GEOID like 244.
//  See the documentation for GetGeoInfoEx.
//
//  WARNING: These values are arbitrarily assigned values, please use
//           standard country/region names instead, such as "US".
//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
typedef LONG    GEOID;

#define GEOID_NOT_AVAILABLE -1

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


DEPRECATED("Use Unicode. The information in this structure cannot represent all encodings accurately and may be unreliable on many machines. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef struct _cpinfoexA {
    UINT    MaxCharSize;                    // max length (in bytes) of a char
    BYTE    DefaultChar[MAX_DEFAULTCHAR];   // default character (MB)
    BYTE    LeadByte[MAX_LEADBYTES];        // lead byte ranges
    WCHAR   UnicodeDefaultChar;             // default character (Unicode)
    UINT    CodePage;                       // code page id
    CHAR    CodePageName[MAX_PATH];         // code page name (Unicode)
} CPINFOEXA, *LPCPINFOEXA;
DEPRECATED("Use Unicode. The information in this structure cannot represent all encodings accurately and may be unreliable on many machines. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef struct _cpinfoexW {
    UINT    MaxCharSize;                    // max length (in bytes) of a char
    BYTE    DefaultChar[MAX_DEFAULTCHAR];   // default character (MB)
    BYTE    LeadByte[MAX_LEADBYTES];        // lead byte ranges
    WCHAR   UnicodeDefaultChar;             // default character (Unicode)
    UINT    CodePage;                       // code page id
    WCHAR   CodePageName[MAX_PATH];         // code page name (Unicode)
} CPINFOEXW, *LPCPINFOEXW;
#ifdef UNICODE
typedef CPINFOEXW CPINFOEX;
typedef LPCPINFOEXW LPCPINFOEX;
#else
typedef CPINFOEXA CPINFOEX;
typedef LPCPINFOEXA LPCPINFOEX;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
//
//  Number format.
//

typedef struct _numberfmtA {
    UINT    NumDigits;                 // number of decimal digits
    UINT    LeadingZero;               // if leading zero in decimal fields
    UINT    Grouping;                  // group size left of decimal
    LPSTR   lpDecimalSep;              // ptr to decimal separator string
    LPSTR   lpThousandSep;             // ptr to thousand separator string
    UINT    NegativeOrder;             // negative number ordering
} NUMBERFMTA, *LPNUMBERFMTA;
typedef struct _numberfmtW {
    UINT    NumDigits;                 // number of decimal digits
    UINT    LeadingZero;               // if leading zero in decimal fields
    UINT    Grouping;                  // group size left of decimal
    LPWSTR  lpDecimalSep;              // ptr to decimal separator string
    LPWSTR  lpThousandSep;             // ptr to thousand separator string
    UINT    NegativeOrder;             // negative number ordering
} NUMBERFMTW, *LPNUMBERFMTW;
#ifdef UNICODE
typedef NUMBERFMTW NUMBERFMT;
typedef LPNUMBERFMTW LPNUMBERFMT;
#else
typedef NUMBERFMTA NUMBERFMT;
typedef LPNUMBERFMTA LPNUMBERFMT;
#endif // UNICODE


//
//  Currency format.
//

typedef struct _currencyfmtA {
    UINT    NumDigits;                 // number of decimal digits
    UINT    LeadingZero;               // if leading zero in decimal fields
    UINT    Grouping;                  // group size left of decimal
    LPSTR   lpDecimalSep;              // ptr to decimal separator string
    LPSTR   lpThousandSep;             // ptr to thousand separator string
    UINT    NegativeOrder;             // negative currency ordering
    UINT    PositiveOrder;             // positive currency ordering
    LPSTR   lpCurrencySymbol;          // ptr to currency symbol string
} CURRENCYFMTA, *LPCURRENCYFMTA;
typedef struct _currencyfmtW {
    UINT    NumDigits;                 // number of decimal digits
    UINT    LeadingZero;               // if leading zero in decimal fields
    UINT    Grouping;                  // group size left of decimal
    LPWSTR  lpDecimalSep;              // ptr to decimal separator string
    LPWSTR  lpThousandSep;             // ptr to thousand separator string
    UINT    NegativeOrder;             // negative currency ordering
    UINT    PositiveOrder;             // positive currency ordering
    LPWSTR  lpCurrencySymbol;          // ptr to currency symbol string
} CURRENCYFMTW, *LPCURRENCYFMTW;
#ifdef UNICODE
typedef CURRENCYFMTW CURRENCYFMT;
typedef LPCURRENCYFMTW LPCURRENCYFMT;
#else
typedef CURRENCYFMTA CURRENCYFMT;
typedef LPCURRENCYFMTA LPCURRENCYFMT;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
//
//  NLS function capabilities
//

enum SYSNLS_FUNCTION {
    COMPARE_STRING          = 0x0001,   // Collation version for NLS
};
typedef DWORD NLS_FUNCTION;


//
//  NLS version structure.
//

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (WINVER >= _WIN32_WINNT_WIN8)
//
// New structures are the same
//

// The combination of dwNLSVersion, and guidCustomVersion
// identify specific sort behavior, persist those to ensure identical
// behavior in the future.
typedef struct _nlsversioninfo{
    DWORD dwNLSVersionInfoSize;     // sizeof(NLSVERSIONINFO) == 32 bytes
    DWORD dwNLSVersion;
    DWORD dwDefinedVersion;         // Deprecated, use dwNLSVersion instead
    DWORD dwEffectiveId;            // Deprecated, use guidCustomVerison instead
    GUID  guidCustomVersion;        // Explicit sort version
} NLSVERSIONINFO, *LPNLSVERSIONINFO;
#else
// 
// Windows 7 and below had different sizes
//

// This is to be deprecated, please use the NLSVERSIONINFOEX
// structure below in the future.  The difference is that
// guidCustomversion is required to uniquely identify a sort
typedef struct _nlsversioninfo{		// Use NLSVERSIONINFOEX instead
    DWORD dwNLSVersionInfoSize;     // 12 bytes
    DWORD dwNLSVersion;
    DWORD dwDefinedVersion;         // Deprecated, use dwNLSVersion instead
} NLSVERSIONINFO, *LPNLSVERSIONINFO;
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// The combination of dwNLSVersion, and guidCustomVersion
// identify specific sort behavior, persist those to ensure identical
// behavior in the future.
typedef struct _nlsversioninfoex{
    DWORD dwNLSVersionInfoSize;     // sizeof(NLSVERSIONINFOEX) == 32 bytes
    DWORD dwNLSVersion;
    DWORD dwDefinedVersion;         // Deprecated, use dwNLSVersion instead
    DWORD dwEffectiveId;            // Deprecated, use guidCustomVerison instead
    GUID  guidCustomVersion;        // Explicit sort version
} NLSVERSIONINFOEX, *LPNLSVERSIONINFOEX;

#if (NTDDI_VERSION >= NTDDI_WIN10_MN)
#define SORTING_PARADIGM_NLS 0x00000000     // NLS style sorting
#define SORTING_PARADIGM_ICU 0x01000000     // ICU style sorting
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
#define GEO_NAME_USER_DEFAULT NULL
#endif

//
//  GEO information types for clients to query
//
// Please use GetGeoInfoEx and query by country/region name instead of GEOID (eg: "US" instead of 244)
enum SYSGEOTYPE {
    GEO_NATION      =       0x0001, // DEPRECATED Not used by name API
    GEO_LATITUDE    =       0x0002,
    GEO_LONGITUDE   =       0x0003,
    GEO_ISO2        =       0x0004,
    GEO_ISO3        =       0x0005,
    GEO_RFC1766     =       0x0006, // DEPRECATED and misleading, not used by name API
    GEO_LCID        =       0x0007, // DEPRECATED Not used by name API
    GEO_FRIENDLYNAME=       0x0008,
    GEO_OFFICIALNAME=       0x0009,
    GEO_TIMEZONES   =       0x000A, // Not implemented
    GEO_OFFICIALLANGUAGES = 0x000B, // Not implemented
    GEO_ISO_UN_NUMBER =     0x000C,
    GEO_PARENT      =       0x000D,
    GEO_DIALINGCODE =       0x000E,
    GEO_CURRENCYCODE=       0x000F, // eg: USD
    GEO_CURRENCYSYMBOL=     0x0010, // eg: $
#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
    GEO_NAME        =       0x0011, // Name, eg: US or 001
    GEO_ID          =       0x0012  // DEPRECATED - For compatibility, please avoid
#endif
};

//
//  More GEOCLASS defines will be listed here
//
DEPRECATED("The Geo Class concept is obsolete and no longer supported.  GetGeoInfoEx is preferred. Set DISABLE_NLS_DEPRECATION to disable this warning.")
enum SYSGEOCLASS {
    GEOCLASS_NATION  = 16,
    GEOCLASS_REGION  = 14,          // DEPRECATED - Never used
    GEOCLASS_ALL = 0
};

#ifdef STRICT

typedef BOOL (CALLBACK* LOCALE_ENUMPROCA)(LPSTR);                                           // Deprecated, please use Unicode
typedef BOOL (CALLBACK* LOCALE_ENUMPROCW)(LPWSTR);                                          // DEPRECATED: please use LOCALE_ENUMPROCEX

#endif // STRICT

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (WINVER >= 0x0600)
//
//  Normalization forms
//

typedef enum _NORM_FORM {
    NormalizationOther  = 0,       // Not supported
    NormalizationC      = 0x1,     // Each base plus combining characters to the canonical precomposed equivalent.
    NormalizationD      = 0x2,     // Each precomposed character to its canonical decomposed equivalent.
    NormalizationKC     = 0x5,     // Each base plus combining characters to the canonical precomposed
                                   //   equivalents and all compatibility characters to their equivalents.
    NormalizationKD     = 0x6      // Each precomposed character to its canonical decomposed equivalent
                                   //   and all compatibility characters to their equivalents.
} NORM_FORM;


//
// IDN (International Domain Name) Flags
//
#define IDN_ALLOW_UNASSIGNED        0x01  // Allow unassigned "query" behavior per RFC 3454
#define IDN_USE_STD3_ASCII_RULES    0x02  // Enforce STD3 ASCII restrictions for legal characters
#define IDN_EMAIL_ADDRESS           0x04  // Enable EAI algorithmic fallback for email local parts behavior
#define IDN_RAW_PUNYCODE            0x08  // Disable validation and mapping of punycode.   

#define VS_ALLOW_LATIN              0x0001  // Allow Latin in test script even if not present in locale script

#define GSS_ALLOW_INHERITED_COMMON  0x0001  // Output script ids for inherited and common character types if present
#endif //(WINVER >= 0x0600)

//
//  Enumeration function constants.
//

#ifdef STRICT

DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef BOOL (CALLBACK* LANGUAGEGROUP_ENUMPROCA)(LGRPID, LPSTR, LPSTR, DWORD, LONG_PTR);    // Deprecated, please use Unicode
DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef BOOL (CALLBACK* LANGGROUPLOCALE_ENUMPROCA)(LGRPID, LCID, LPSTR, LONG_PTR);          // Deprecated, please use Unicode
typedef BOOL (CALLBACK* UILANGUAGE_ENUMPROCA)(LPSTR, LONG_PTR);                             // Deprecated, please use Unicode
typedef BOOL (CALLBACK* CODEPAGE_ENUMPROCA)(LPSTR);                                         // Deprecated, please use Unicode
typedef BOOL (CALLBACK* DATEFMT_ENUMPROCA)(LPSTR);                                          // Deprecated, please use Unicode
typedef BOOL (CALLBACK* DATEFMT_ENUMPROCEXA)(LPSTR, CALID);                                 // Deprecated, please use Unicode
typedef BOOL (CALLBACK* TIMEFMT_ENUMPROCA)(LPSTR);                                          // Deprecated, please use Unicode
typedef BOOL (CALLBACK* CALINFO_ENUMPROCA)(LPSTR);                                          // Deprecated, please use Unicode
typedef BOOL (CALLBACK* CALINFO_ENUMPROCEXA)(LPSTR, CALID);                                 // Deprecated, please use Unicode

DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef BOOL (CALLBACK* LANGUAGEGROUP_ENUMPROCW)(LGRPID, LPWSTR, LPWSTR, DWORD, LONG_PTR);  // DEPRECATED: Language groups are no longer supported
DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef BOOL (CALLBACK* LANGGROUPLOCALE_ENUMPROCW)(LGRPID, LCID, LPWSTR, LONG_PTR);         // DEPRECATED: Language groups are no longer supported
typedef BOOL (CALLBACK* UILANGUAGE_ENUMPROCW)(LPWSTR, LONG_PTR);
typedef BOOL (CALLBACK* CODEPAGE_ENUMPROCW)(LPWSTR);            // Please use Unicode / UTF-8
typedef BOOL (CALLBACK* DATEFMT_ENUMPROCW)(LPWSTR);
typedef BOOL (CALLBACK* DATEFMT_ENUMPROCEXW)(LPWSTR, CALID);
typedef BOOL (CALLBACK* TIMEFMT_ENUMPROCW)(LPWSTR);
typedef BOOL (CALLBACK* CALINFO_ENUMPROCW)(LPWSTR);
typedef BOOL (CALLBACK* CALINFO_ENUMPROCEXW)(LPWSTR, CALID);
typedef BOOL (CALLBACK* GEO_ENUMPROC)(GEOID);                   // DEPRECATED, use GEO_ENUMNAMEPROC instead
#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
typedef BOOL (CALLBACK* GEO_ENUMNAMEPROC)(PWSTR, LPARAM);
#endif

#else // !STRICT

DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef FARPROC LANGUAGEGROUP_ENUMPROCA;       // Deprecated, please use Unicode
DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef FARPROC LANGGROUPLOCALE_ENUMPROCA;     // Deprecated, please use Unicode
typedef FARPROC UILANGUAGE_ENUMPROCA;          // Deprecated, please use Unicode
typedef FARPROC CODEPAGE_ENUMPROCA;            // Deprecated, please use Unicode
typedef FARPROC DATEFMT_ENUMPROCA;             // Deprecated, please use Unicode
typedef FARPROC DATEFMT_ENUMPROCEXA;           // Deprecated, please use Unicode
typedef FARPROC TIMEFMT_ENUMPROCA;             // Deprecated, please use Unicode
typedef FARPROC CALINFO_ENUMPROCA;             // Deprecated, please use Unicode
typedef FARPROC CALINFO_ENUMPROCEXA;           // Deprecated, please use Unicode
typedef FARPROC GEO_ENUMPROC;                  // DEPRECATED, use GEO_ENUMNAMEPROC instead

#endif // !STRICT

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifndef STRICT

typedef FARPROC LOCALE_ENUMPROCA;              // Deprecated, please use Unicode
typedef FARPROC LOCALE_ENUMPROCW;              // DEPRECATED: please use LOCALE_ENUMPROCEX

#endif // !STRICT

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion


#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#ifndef STRICT

DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef FARPROC LANGUAGEGROUP_ENUMPROCW;       // DEPRECATED: Language groups are no longer supported
DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
typedef FARPROC LANGGROUPLOCALE_ENUMPROCW;     // DEPRECATED: Language groups are no longer supported
typedef FARPROC UILANGUAGE_ENUMPROCW;
typedef FARPROC CODEPAGE_ENUMPROCW;            // Please use Unicode / UTF-8
typedef FARPROC DATEFMT_ENUMPROCW;
typedef FARPROC DATEFMT_ENUMPROCEXW;
typedef FARPROC TIMEFMT_ENUMPROCW;
typedef FARPROC CALINFO_ENUMPROCW;
typedef FARPROC CALINFO_ENUMPROCEXW;

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
typedef FARPROC GEO_ENUMNAMEPROC;
#endif

#endif // !STRICT

#ifdef UNICODE

// DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
#define LANGUAGEGROUP_ENUMPROC    LANGUAGEGROUP_ENUMPROCW
// DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
#define LANGGROUPLOCALE_ENUMPROC  LANGGROUPLOCALE_ENUMPROCW
#define UILANGUAGE_ENUMPROC       UILANGUAGE_ENUMPROCW
#define CODEPAGE_ENUMPROC         CODEPAGE_ENUMPROCW
#define DATEFMT_ENUMPROC          DATEFMT_ENUMPROCW
#define DATEFMT_ENUMPROCEX        DATEFMT_ENUMPROCEXW
#define TIMEFMT_ENUMPROC          TIMEFMT_ENUMPROCW
#define CALINFO_ENUMPROC          CALINFO_ENUMPROCW
#define CALINFO_ENUMPROCEX        CALINFO_ENUMPROCEXW
#define LOCALE_ENUMPROC           LOCALE_ENUMPROCW

#else

// DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
#define LANGUAGEGROUP_ENUMPROC    LANGUAGEGROUP_ENUMPROCA
// DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
#define LANGGROUPLOCALE_ENUMPROC  LANGGROUPLOCALE_ENUMPROCA
#define UILANGUAGE_ENUMPROC       UILANGUAGE_ENUMPROCA
#define CODEPAGE_ENUMPROC         CODEPAGE_ENUMPROCA
#define DATEFMT_ENUMPROC          DATEFMT_ENUMPROCA
#define DATEFMT_ENUMPROCEX        DATEFMT_ENUMPROCEXA
#define TIMEFMT_ENUMPROC          TIMEFMT_ENUMPROCA
#define CALINFO_ENUMPROC          CALINFO_ENUMPROCA
#define CALINFO_ENUMPROCEX        CALINFO_ENUMPROCEXA
#define LOCALE_ENUMPROC           LOCALE_ENUMPROCA

#endif // !UNICODE

//
// Information about a MUI file, used as input/output in GetFileMUIInfo
// All offsets are relative to start of the structure. Offsets with value 0 mean empty field.
//

typedef struct _FILEMUIINFO {
    DWORD       dwSize;                 // Size of the structure including buffer size [in]
    DWORD       dwVersion;              // Version of the structure [in]
    DWORD       dwFileType;             // Type of the file [out]
    BYTE        pChecksum[16];          // Checksum of the file [out]
    BYTE        pServiceChecksum[16];   // Checksum of the file [out]
    DWORD       dwLanguageNameOffset;   // Language name of the file [out]
    DWORD       dwTypeIDMainSize;       // Number of TypeIDs in main module [out]
    DWORD       dwTypeIDMainOffset;     // Array of TypeIDs (DWORD) in main module [out]
    DWORD       dwTypeNameMainOffset;   // Multistring array of TypeNames in main module [out]
    DWORD       dwTypeIDMUISize;        // Number of TypeIDs in MUI module [out]
    DWORD       dwTypeIDMUIOffset;      // Array of TypeIDs (DWORD) in MUI module [out]
    DWORD       dwTypeNameMUIOffset;    // Multistring array of TypeNames in MUI module [out]
    BYTE        abBuffer[8];             // Buffer for extra data [in] (Size 4 is for padding)
} FILEMUIINFO, *PFILEMUIINFO;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifndef NOAPISET
#include <stringapiset.h>    // String APISET dependencies
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

////////////////////////////////////////////////////////////////////////////
//
//  Macros
//
//  Define all macros for the NLS component here.
//
////////////////////////////////////////////////////////////////////////////

//
//  Macros to determine whether a character is a high or low surrogate,
//  and whether two code points make up a surrogate pair (a high surrogate
//  and a low surrogate).
//
#define IS_HIGH_SURROGATE(wch) (((wch) >= HIGH_SURROGATE_START) && ((wch) <= HIGH_SURROGATE_END))
#define IS_LOW_SURROGATE(wch)  (((wch) >= LOW_SURROGATE_START) && ((wch) <= LOW_SURROGATE_END))
#define IS_SURROGATE_PAIR(hs, ls) (IS_HIGH_SURROGATE(hs) && IS_LOW_SURROGATE(ls))

// ----------------------------------------------------------------------
// The following macros retrieve information from a MUIFILEINFO structure
//
// Gets the culture name (LPWSTR), NULL if not initialized
#define FILEMUIINFO_GET_CULTURE(pInfo)          \
    ((LPWSTR)((pInfo->dwLanguageNameOffset>0)?(ULONG_PTR)pInfo+pInfo->dwLanguageNameOffset:NULL))
//
// Gets the main module types array (DWORD[]), NULL if not initialized
#define FILEMUIINFO_GET_MAIN_TYPEIDS(pInfo)       \
    ((DWORD*)((pInfo->dwTypeIDMainOffset>0)?(ULONG_PTR)pInfo+pInfo->dwTypeIDMainOffset:NULL))
//
// Gets the main module type array element iType (DWORD), the array is not initialized or index is out of bounds
#define FILEMUIINFO_GET_MAIN_TYPEID(pInfo,iType)  \
    (((iType<pInfo->dwTypeIDMainSize)&&(pInfo->dwTypeIDMainOffset>0))?*((DWORD*)((ULONG_PTR)pInfo+pInfo->dwTypeIDMainOffset)+iType):0)
//
// Gets the main module names multistring array (LPWSTR), NULL if not initialized
#define FILEMUIINFO_GET_MAIN_TYPENAMES(pInfo)       \
    ((LPWSTR)((pInfo->dwTypeNameMainOffset>0)?(ULONG_PTR)pInfo+pInfo->dwTypeNameMainOffset:NULL))
//
// Gets the mui module types array (DWORD[]), NULL if not initialized
#define FILEMUIINFO_GET_MUI_TYPEIDS(pInfo)        \
    ((DWORD*)((pInfo->dwTypeIDMUIOffset>0)?(ULONG_PTR)pInfo+pInfo->dwTypeIDMUIOffset:NULL))
//
// Gets the mui module type array element iType (DWORD), the array is not initialized or index is out of bounds
#define FILEMUIINFO_GET_MUI_TYPEID(pInfo,iType)   \
    (((iType<pInfo->dwTypeIDMUISize)&&(pInfo->dwTypeIDMUIOffset>0))?*((DWORD*)((ULONG_PTR)pInfo+pInfo->dwTypeIDMUIOffset)+iType):0)
//
// Gets the mui module names multistring array (LPWSTR), NULL if not initialized
#define FILEMUIINFO_GET_MUI_TYPENAMES(pInfo)        \
    ((LPWSTR)((pInfo->dwTypeNameMUIOffset>0)?(ULONG_PTR)pInfo+pInfo->dwTypeNameMUIOffset:NULL))
// ------------------------------------------------------------------------



////////////////////////////////////////////////////////////////////////////
//
//  Function Prototypes
//
//  Only prototypes for the NLS APIs should go here.
//
////////////////////////////////////////////////////////////////////////////

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
//  Code Page Dependent APIs.
//
//  Applications should use Unicode (WCHAR / UTF-16 &/or UTF-8)
//

WINBASEAPI
BOOL
WINAPI
IsValidCodePage(
    _In_ UINT  CodePage);

WINBASEAPI
UINT
WINAPI
GetACP(void);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
UINT
WINAPI
GetOEMCP(void);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop or Pc Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP |WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

DEPRECATED("Use Unicode. The information in this structure cannot represent all encodings accuratedly and may be unreliable on many machines. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
BOOL
WINAPI
GetCPInfo(
    _In_ UINT       CodePage,
    _Out_ LPCPINFO  lpCPInfo);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP |WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or Pc Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP |WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

DEPRECATED("Use Unicode. The information in this structure cannot represent all encodings accurately and may be unreliable on many machines. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
BOOL
WINAPI
GetCPInfoExA(
    _In_ UINT          CodePage,
    _In_ DWORD         dwFlags,
    _Out_ LPCPINFOEXA  lpCPInfoEx);
DEPRECATED("Use Unicode. The information in this structure cannot represent all encodings accurately and may be unreliable on many machines. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
BOOL
WINAPI
GetCPInfoExW(
    _In_ UINT          CodePage,
    _In_ DWORD         dwFlags,
    _Out_ LPCPINFOEXW  lpCPInfoEx);
#ifdef UNICODE
#define GetCPInfoEx  GetCPInfoExW
#else
#define GetCPInfoEx  GetCPInfoExA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP |WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

//
//  Locale Dependent APIs.
//

#pragma region Desktop or OneCore or Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

DEPRECATED("CompareStringEx is preferred. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
int
WINAPI
CompareStringA(
    _In_ LCID     Locale,
    _In_ DWORD    dwCmpFlags,
    _In_reads_(cchCount1) PCNZCH lpString1,
    _In_ int      cchCount1,
    _In_reads_(cchCount2) PCNZCH  lpString2,
    _In_ int      cchCount2);

#ifndef UNICODE
#define CompareString  CompareStringA
#endif // !UNICODE

#if defined(_M_CEE)
#undef CompareString
DEPRECATED("CompareStringEx is preferred. Set DISABLE_NLS_DEPRECATION to disable this warning.")
__inline
int
CompareString(
    LCID     Locale,
    DWORD    dwCmpFlags,
    LPCTSTR  lpString1,
    int      cchCount1,
    LPCTSTR  lpString2,
    int      cchCount2
    )
{
#ifdef UNICODE
    return CompareStringW(
#else
    return CompareStringA(
#endif
        Locale,
        dwCmpFlags,
        lpString1,
        cchCount1,
        lpString2,
        cchCount2
        );
}
#endif	/* _M_CEE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (WINVER >= 0x0600)

// DEPRECATED: FindNLSStringEx is preferred
WINBASEAPI
int
WINAPI
FindNLSString(
    _In_                    LCID Locale,
    _In_                    DWORD dwFindNLSStringFlags,
    _In_reads_(cchSource)  LPCWSTR lpStringSource,
    _In_                    int cchSource,
    _In_reads_(cchValue)   LPCWSTR lpStringValue,
    _In_                    int cchValue,
    _Out_opt_               LPINT pcchFound);

#endif //(WINVER >= 0x0600)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// DEPRECATED: LCMapStringEx is preferred
WINBASEAPI
int
WINAPI
LCMapStringW(
    _In_ LCID     Locale,
    _In_ DWORD    dwMapFlags,
    _In_reads_(cchSrc) LPCWSTR  lpSrcStr,
    _In_ int      cchSrc,
    _Out_writes_opt_(_Inexpressible_(cchDest)) LPWSTR  lpDestStr,
    _In_ int      cchDest);
#ifdef UNICODE
#define LCMapString  LCMapStringW
#endif

// DEPRECATED: Use Unicode, LCMapStringEx is preferred
WINBASEAPI
int
WINAPI
LCMapStringA(
    _In_ LCID     Locale,
    _In_ DWORD    dwMapFlags,
    _In_reads_(cchSrc) LPCSTR  lpSrcStr,
    _In_ int      cchSrc,
    _Out_writes_opt_(_Inexpressible_(cchDest)) LPSTR  lpDestStr,
    _In_ int      cchDest);
#ifndef UNICODE
#define LCMapString  LCMapStringA
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// DEPRECATED: GetLocaleInfoEx is preferred
WINBASEAPI
int
WINAPI
GetLocaleInfoW(
    _In_ LCID     Locale,
    _In_ LCTYPE   LCType,
    _Out_writes_opt_(cchData) LPWSTR lpLCData,
    _In_ int      cchData);
    
#ifdef UNICODE
#define GetLocaleInfo  GetLocaleInfoW
#endif

// DEPRECATED: Use Unicode. GetLocaleInfoEx is preferred
WINBASEAPI
int 
WINAPI 
GetLocaleInfoA(
    _In_ LCID Locale,
    _In_ LCTYPE LCType,
    _Out_writes_opt_(cchData) LPSTR lpLCData,
    _In_ int cchData
    );

#ifndef UNICODE    
#define GetLocaleInfo GetLocaleInfoA
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
SetLocaleInfoA(
    _In_ LCID     Locale,
    _In_ LCTYPE   LCType,
    _In_ LPCSTR  lpLCData);
WINBASEAPI
BOOL
WINAPI
SetLocaleInfoW(
    _In_ LCID     Locale,
    _In_ LCTYPE   LCType,
    _In_ LPCWSTR  lpLCData);
#ifdef UNICODE
#define SetLocaleInfo  SetLocaleInfoW
#else
#define SetLocaleInfo  SetLocaleInfoA
#endif // !UNICODE

#if (WINVER >= 0x040A)

// DEPRECATED: GetCalendarInfoEx is preferred
WINBASEAPI
int
WINAPI
GetCalendarInfoA(
    _In_ LCID     Locale,
    _In_ CALID    Calendar,
    _In_ CALTYPE  CalType,
    _Out_writes_opt_(cchData) LPSTR   lpCalData,
    _In_ int      cchData,
    _Out_opt_ LPDWORD  lpValue);
// DEPRECATED: GetCalendarInfoEx is preferred
WINBASEAPI
int
WINAPI
GetCalendarInfoW(
    _In_ LCID     Locale,
    _In_ CALID    Calendar,
    _In_ CALTYPE  CalType,
    _Out_writes_opt_(cchData) LPWSTR   lpCalData,
    _In_ int      cchData,
    _Out_opt_ LPDWORD  lpValue);
#ifdef UNICODE
#define GetCalendarInfo  GetCalendarInfoW
#else
#define GetCalendarInfo  GetCalendarInfoA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
SetCalendarInfoA(
    _In_ LCID     Locale,
    _In_ CALID    Calendar,
    _In_ CALTYPE  CalType,
    _In_ LPCSTR  lpCalData);
WINBASEAPI
BOOL
WINAPI
SetCalendarInfoW(
    _In_ LCID     Locale,
    _In_ CALID    Calendar,
    _In_ CALTYPE  CalType,
    _In_ LPCWSTR  lpCalData);
#ifdef UNICODE
#define SetCalendarInfo  SetCalendarInfoW
#else
#define SetCalendarInfo  SetCalendarInfoA
#endif // !UNICODE
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (WINVER >= _WIN32_WINNT_WIN7)
//                                        
// Flags used by LoadStringByReference    
//                                        
#define MUI_FORMAT_REG_COMPAT  0x0001     
#define MUI_FORMAT_INF_COMPAT  0x0002     
#define MUI_VERIFY_FILE_EXISTS 0x0004     
#define MUI_SKIP_STRING_CACHE  0x0008     
#define MUI_IMMUTABLE_LOOKUP   0x0010     
        
WINBASEAPI                                
BOOL                                      
WINAPI                                    
LoadStringByReference(                    
    _In_       DWORD   Flags,             
    _In_opt_       PCWSTR  Language,      
    _In_       PCWSTR  SourceString,      
    _Out_writes_opt_(cchBuffer)   PWSTR   Buffer,     
    _In_       ULONG  cchBuffer,                      
    _In_opt_   PCWSTR  Directory,                     
    _Out_opt_  PULONG  pcchBufferOut                  
    );                                                
#endif // (WINVER >= _WIN32_WINNT_WIN7)	    


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

DEPRECATED("Use Unicode. The information provided by this structure is inaccurate for some encodings and may be unreliable on many machines.")
WINBASEAPI         
BOOL               
WINAPI             
IsDBCSLeadByte(    
    _In_ BYTE  TestChar 
    );

DEPRECATED("Use Unicode. The information provided by this structure is inaccurate for some encodings and may be unreliable on many machines.")
WINBASEAPI
BOOL
WINAPI
IsDBCSLeadByteEx(
    _In_ UINT  CodePage,
    _In_ BYTE  TestChar
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (WINVER >= 0x0600)

// Use of Locale Names is preferred, LCIDs are deprecated.
// This function is provided to enable compatibility with legacy data sets only.
WINBASEAPI
LCID
WINAPI
LocaleNameToLCID(
    _In_ LPCWSTR lpName,
    _In_ DWORD dwFlags);

// Use of Locale Names is preferred, LCIDs are deprecated.
// This function is provided to enable compatibility with legacy data sets only.
WINBASEAPI
int
WINAPI
LCIDToLocaleName(
    _In_ LCID     Locale,
    _Out_writes_opt_(cchName) LPWSTR  lpName,
    _In_ int      cchName,
    _In_ DWORD    dwFlags);

#endif  // (WINVER >= 0x0600)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// DEPRECATED: GetDurationFormatEx is preferred
#if (WINVER >= 0x0600)
WINBASEAPI
int
WINAPI
GetDurationFormat(
    _In_ LCID             Locale,
    _In_ DWORD            dwFlags,
    _In_opt_ CONST SYSTEMTIME *lpDuration,
    _In_ ULONGLONG ullDuration,
    _In_opt_ LPCWSTR          lpFormat,
    _Out_writes_opt_(cchDuration) LPWSTR          lpDurationStr,
    _In_ int              cchDuration);
#endif //(WINVER >= 0x0600)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// DEPRECATED: GetNumberFormatEx is preferred
WINBASEAPI
int
WINAPI
GetNumberFormatA(
    _In_ LCID             Locale,
    _In_ DWORD            dwFlags,
    _In_ LPCSTR          lpValue,
    _In_opt_ CONST NUMBERFMTA *lpFormat,
    _Out_writes_opt_(cchNumber) LPSTR          lpNumberStr,
    _In_ int              cchNumber);
// DEPRECATED: GetNumberFormatEx is preferred
WINBASEAPI
int
WINAPI
GetNumberFormatW(
    _In_ LCID             Locale,
    _In_ DWORD            dwFlags,
    _In_ LPCWSTR          lpValue,
    _In_opt_ CONST NUMBERFMTW *lpFormat,
    _Out_writes_opt_(cchNumber) LPWSTR          lpNumberStr,
    _In_ int              cchNumber);
#ifdef UNICODE
#define GetNumberFormat  GetNumberFormatW
#else
#define GetNumberFormat  GetNumberFormatA
#endif // !UNICODE

// DEPRECATED: GetCurrencyFormatEx is preferred
WINBASEAPI
int
WINAPI
GetCurrencyFormatA(
    _In_ LCID               Locale,
    _In_ DWORD              dwFlags,
    _In_ LPCSTR            lpValue,
    _In_opt_ CONST CURRENCYFMTA *lpFormat,
    _Out_writes_opt_(cchCurrency) LPSTR            lpCurrencyStr,
    _In_ int                cchCurrency);
// DEPRECATED: GetCurrencyFormatEx is preferred
WINBASEAPI
int
WINAPI
GetCurrencyFormatW(
    _In_ LCID               Locale,
    _In_ DWORD              dwFlags,
    _In_ LPCWSTR            lpValue,
    _In_opt_ CONST CURRENCYFMTW *lpFormat,
    _Out_writes_opt_(cchCurrency) LPWSTR            lpCurrencyStr,
    _In_ int                cchCurrency);
#ifdef UNICODE
#define GetCurrencyFormat  GetCurrencyFormatW
#else
#define GetCurrencyFormat  GetCurrencyFormatA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

// DEPRECATED: EnumCalendarInfoExEx is preferred
WINBASEAPI
BOOL
WINAPI
EnumCalendarInfoA(
    _In_ CALINFO_ENUMPROCA lpCalInfoEnumProc,
    _In_ LCID              Locale,
    _In_ CALID             Calendar,
    _In_ CALTYPE           CalType);
// DEPRECATED: EnumCalendarInfoExEx is preferred
WINBASEAPI
BOOL
WINAPI
EnumCalendarInfoW(
    _In_ CALINFO_ENUMPROCW lpCalInfoEnumProc,
    _In_ LCID              Locale,
    _In_ CALID             Calendar,
    _In_ CALTYPE           CalType);
#ifdef UNICODE
#define EnumCalendarInfo  EnumCalendarInfoW
#else
#define EnumCalendarInfo  EnumCalendarInfoA
#endif // !UNICODE

#if(WINVER >= 0x0500)
// DEPRECATED: EnumCalendarInfoExEx is preferred
WINBASEAPI
BOOL
WINAPI
EnumCalendarInfoExA(
    _In_ CALINFO_ENUMPROCEXA lpCalInfoEnumProcEx,
    _In_ LCID                Locale,
    _In_ CALID               Calendar,
    _In_ CALTYPE             CalType);
// DEPRECATED: EnumCalendarInfoExEx is preferred
WINBASEAPI
BOOL
WINAPI
EnumCalendarInfoExW(
    _In_ CALINFO_ENUMPROCEXW lpCalInfoEnumProcEx,
    _In_ LCID                Locale,
    _In_ CALID               Calendar,
    _In_ CALTYPE             CalType);
#ifdef UNICODE
#define EnumCalendarInfoEx  EnumCalendarInfoExW
#else
#define EnumCalendarInfoEx  EnumCalendarInfoExA
#endif // !UNICODE
#endif /* WINVER >= 0x0500 */

// DEPRECATED: EnumTimeFormatsEx is preferred
WINBASEAPI
BOOL
WINAPI
EnumTimeFormatsA(
    _In_ TIMEFMT_ENUMPROCA lpTimeFmtEnumProc,
    _In_ LCID              Locale,
    _In_ DWORD             dwFlags);
// DEPRECATED: EnumTimeFormatsEx is preferred
WINBASEAPI
BOOL
WINAPI
EnumTimeFormatsW(
    _In_ TIMEFMT_ENUMPROCW lpTimeFmtEnumProc,
    _In_ LCID              Locale,
    _In_ DWORD             dwFlags);
#ifdef UNICODE
#define EnumTimeFormats  EnumTimeFormatsW
#else
#define EnumTimeFormats  EnumTimeFormatsA
#endif // !UNICODE

// DEPRECATED: EnumDateFormatsExEx is preferred
WINBASEAPI
BOOL
WINAPI
EnumDateFormatsA(
    _In_ DATEFMT_ENUMPROCA lpDateFmtEnumProc,
    _In_ LCID              Locale,
    _In_ DWORD             dwFlags);
// DEPRECATED: EnumDateFormatsExEx is preferred
WINBASEAPI
BOOL
WINAPI
EnumDateFormatsW(
    _In_ DATEFMT_ENUMPROCW lpDateFmtEnumProc,
    _In_ LCID              Locale,
    _In_ DWORD             dwFlags);
#ifdef UNICODE
#define EnumDateFormats  EnumDateFormatsW
#else
#define EnumDateFormats  EnumDateFormatsA
#endif // !UNICODE

#if(WINVER >= 0x0500)
// DEPRECATED: EnumDateFormatsExEx is preferred
WINBASEAPI
BOOL
WINAPI
EnumDateFormatsExA(
    _In_ DATEFMT_ENUMPROCEXA lpDateFmtEnumProcEx,
    _In_ LCID                Locale,
    _In_ DWORD               dwFlags);
// DEPRECATED: EnumDateFormatsExEx is preferred
WINBASEAPI
BOOL
WINAPI
EnumDateFormatsExW(
    _In_ DATEFMT_ENUMPROCEXW lpDateFmtEnumProcEx,
    _In_ LCID                Locale,
    _In_ DWORD               dwFlags);
#ifdef UNICODE
#define EnumDateFormatsEx  EnumDateFormatsExW
#else
#define EnumDateFormatsEx  EnumDateFormatsExA
#endif // !UNICODE
#endif /* WINVER >= 0x0500 */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if(WINVER >= 0x0500)
DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
BOOL
WINAPI
IsValidLanguageGroup(
    _In_ LGRPID  LanguageGroup,
    _In_ DWORD   dwFlags);
#endif /* WINVER >= 0x0500 */

// DEPRECATED: GetNLSVersionEx is preferred
WINBASEAPI
BOOL
WINAPI
GetNLSVersion(
    _In_    NLS_FUNCTION     Function,
    _In_    LCID             Locale,
    _Inout_ LPNLSVERSIONINFO lpVersionInformation);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

// DEPRECATED: IsValidLocaleName is preferred
WINBASEAPI
BOOL
WINAPI
IsValidLocale(
    _In_ LCID   Locale,
    _In_ DWORD  dwFlags);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// GetGeoInfoEx is preferred where available
DEPRECATED("The GeoID concept is obsolete. Use GetGeoInfoEx instead. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
int
WINAPI
GetGeoInfoA(
    _In_ GEOID       Location,
    _In_ GEOTYPE     GeoType,
    _Out_writes_opt_(cchData) LPSTR     lpGeoData,
    _In_ int         cchData,
    _In_ LANGID      LangId);
// GetGeoInfoEx is preferred where available
DEPRECATED("The GeoID concept is obsolete. Use GetGeoInfoEx instead. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
int
WINAPI
GetGeoInfoW(
    _In_ GEOID       Location,
    _In_ GEOTYPE     GeoType,
    _Out_writes_opt_(cchData) LPWSTR     lpGeoData,
    _In_ int         cchData,
    _In_ LANGID      LangId);
#ifdef UNICODE
#define GetGeoInfo  GetGeoInfoW
#else
#define GetGeoInfo  GetGeoInfoA
#endif // !UNICODE

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
WINBASEAPI
int
WINAPI
GetGeoInfoEx(
    _In_ PWSTR       location,
    _In_ GEOTYPE     geoType,
    _Out_writes_opt_(geoDataCount) PWSTR    geoData,
    _In_ int         geoDataCount);
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or PC Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM)

// EnumSystemGeoNames is preferred where available
DEPRECATED("The GeoID concept is obsolete. Use EnumSystemGoNames instead. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
BOOL
WINAPI
EnumSystemGeoID(
    _In_ GEOCLASS        GeoClass,
    _In_ GEOID           ParentGeoId,
    _In_ GEO_ENUMPROC    lpGeoEnumProc);

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
WINBASEAPI
BOOL
WINAPI
EnumSystemGeoNames(
    _In_ GEOCLASS            geoClass,
    _In_ GEO_ENUMNAMEPROC    geoEnumProc,
    _In_opt_ LPARAM          data);
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// GetUserDefaultGeoName is preferred where available
DEPRECATED("The GeoID concept is obsolete. Use GetUserDefaultGeoName instead. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
GEOID
WINAPI
GetUserGeoID(
    _In_ GEOCLASS    GeoClass);

/**
 * Note: This API was added in the Windows 10 Fall Creators Update.
 * (Please use this API instead of calling GetUserGeoID.)
 *
 */
WINBASEAPI
int
WINAPI
GetUserDefaultGeoName(
    _Out_writes_z_(geoNameCount) LPWSTR geoName,
    _In_ int geoNameCount
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

// GetUserDefaultGeoName is preferred where available
// Applications are recommended to not change user settings themselves.
DEPRECATED("The GeoID concept is obsolete. Use SetUserGeoName instead. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
BOOL
WINAPI
SetUserGeoID(
    _In_ GEOID       GeoId);

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
// Applications are recommended to not change user settings themselves.
WINBASEAPI
BOOL
WINAPI
SetUserGeoName(
    _In_ PWSTR       geoName);
#endif

// DEPRECATED: Please use ResolveLocaleName
WINBASEAPI
LCID
WINAPI
ConvertDefaultLocale(
    _In_ LCID   Locale);

#if(WINVER >= 0x0500)
// DEPRECATED: Please use the user's language profile.
WINBASEAPI
LANGID
WINAPI
GetSystemDefaultUILanguage(void);

#endif /* WINVER >= 0x0500 */
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#if(WINVER >= 0x0500)

WINBASEAPI
LCID
WINAPI
GetThreadLocale(void);

WINBASEAPI
BOOL
WINAPI
SetThreadLocale(
    _In_ LCID  Locale
    );

#endif /* WINVER >= 0x0500 */
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#if(WINVER >= 0x0500)
// DEPRECATED: Please use the user's language profile.
WINBASEAPI
LANGID
WINAPI
GetUserDefaultUILanguage(void);
#endif /* WINVER >= 0x0500 */

// DEPRECATED: Please use GetUserDefaultLocaleName
WINBASEAPI
LANGID
WINAPI
GetUserDefaultLangID(void);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

// DEPRECATED: Please use GetUserDefaultLocaleName or the user's Language Profile
WINBASEAPI
LANGID
WINAPI
GetSystemDefaultLangID(void);

// DEPRECATED: Please use GetUserDefaultLocaleName or the user's Language Profile
WINBASEAPI
LCID
WINAPI
GetSystemDefaultLCID(void);

// DEPRECATED: Please use GetUserDefaultLocaleName
WINBASEAPI
LCID
WINAPI
GetUserDefaultLCID(void);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
LANGID
WINAPI
SetThreadUILanguage(_In_ LANGID LangId);


#if (WINVER >= 0x0600)

WINBASEAPI
LANGID
WINAPI
GetThreadUILanguage(void);

WINBASEAPI
BOOL
WINAPI
GetProcessPreferredUILanguages(
    _In_ DWORD dwFlags,
    _Out_ PULONG pulNumLanguages,
    _Out_writes_opt_(*pcchLanguagesBuffer) PZZWSTR pwszLanguagesBuffer,
    _Inout_ PULONG pcchLanguagesBuffer
);


WINBASEAPI
BOOL
WINAPI
SetProcessPreferredUILanguages(
    _In_        DWORD dwFlags,
    _In_opt_    PCZZWSTR pwszLanguagesBuffer,
    _Out_opt_   PULONG pulNumLanguages
);

#endif /* WINVER >= 0x0600 */
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or Phone Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PHONE_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if(WINVER >= 0x0600)
WINBASEAPI
BOOL
WINAPI
GetUserPreferredUILanguages (
    _In_ DWORD dwFlags,
    _Out_ PULONG pulNumLanguages,
    _Out_writes_opt_(*pcchLanguagesBuffer) PZZWSTR pwszLanguagesBuffer,
    _Inout_ PULONG pcchLanguagesBuffer
);

#endif /* WINVER >= 0x0600 */

#endif  /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PHONE_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if(WINVER >= 0x0600)

WINBASEAPI
BOOL
WINAPI
GetSystemPreferredUILanguages (
    _In_ DWORD dwFlags,
    _Out_ PULONG pulNumLanguages,
    _Out_writes_opt_(*pcchLanguagesBuffer) PZZWSTR pwszLanguagesBuffer,
    _Inout_ PULONG pcchLanguagesBuffer
);


WINBASEAPI
BOOL
WINAPI
GetThreadPreferredUILanguages(
    _In_ DWORD dwFlags,
    _Out_ PULONG pulNumLanguages,
    _Out_writes_opt_(*pcchLanguagesBuffer) PZZWSTR pwszLanguagesBuffer,
    _Inout_ PULONG pcchLanguagesBuffer
);


WINBASEAPI
BOOL
WINAPI
SetThreadPreferredUILanguages(
    _In_        DWORD dwFlags,
    _In_opt_    PCZZWSTR pwszLanguagesBuffer,
    _Out_opt_   PULONG pulNumLanguages
);

WINBASEAPI
_Success_(return!=FALSE)
BOOL
WINAPI
GetFileMUIInfo(
                        DWORD           dwFlags,
    _In_                PCWSTR          pcwszFilePath,
    _Inout_updates_bytes_to_opt_(*pcbFileMUIInfo,*pcbFileMUIInfo) PFILEMUIINFO    pFileMUIInfo,
    _Inout_             DWORD*          pcbFileMUIInfo);

WINBASEAPI
BOOL
WINAPI
GetFileMUIPath(
    _In_ DWORD      dwFlags,
    _In_ PCWSTR     pcwszFilePath ,
    _Inout_updates_opt_    (*pcchLanguage)   PWSTR pwszLanguage,
    _Inout_ PULONG  pcchLanguage,
    _Out_writes_opt_    (*pcchFileMUIPath) PWSTR pwszFileMUIPath,
    _Inout_         PULONG pcchFileMUIPath,
    _Inout_         PULONGLONG pululEnumerator
);


WINBASEAPI
BOOL
WINAPI
GetUILanguageInfo(
    _In_ DWORD dwFlags,
    _In_ PCZZWSTR pwmszLanguage,
    _Out_writes_opt_(*pcchFallbackLanguages) PZZWSTR pwszFallbackLanguages,
    _Inout_opt_ PDWORD pcchFallbackLanguages,
    _Out_ PDWORD pAttributes
);

#endif /* WINVER >= 0x0600 */

#endif  /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WIN10_VB) 

DECLARE_HANDLE(HSAVEDUILANGUAGES);

WINBASEAPI
BOOL
WINAPI
SetThreadPreferredUILanguages2(
    _In_       ULONG flags,
    _In_opt_   PCZZWSTR languages,
    _Out_opt_  PULONG numLanguagesSet,
    _Out_opt_  HSAVEDUILANGUAGES* snapshot);

WINBASEAPI
void
WINAPI
RestoreThreadPreferredUILanguages(_In_ const HSAVEDUILANGUAGES snapshot);

#endif /* NTDDI_VERSION >= NTDDI_WIN10_VB */

#endif  /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if(WINVER >= 0x0600)

WINBASEAPI
BOOL
WINAPI
NotifyUILanguageChange(
    _In_        DWORD dwFlags,
    _In_opt_    PCWSTR pcwstrNewLanguage,
    _In_opt_    PCWSTR pcwstrPreviousLanguage,
    _In_        DWORD dwReserved,
    _Out_opt_   PDWORD pdwStatusRtrn
);

#endif /* WINVER >= 0x0600 */

#endif  /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


//
//  Locale Independent APIs.
//

#pragma region Desktop or OneCore or Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
GetStringTypeExA(
    _In_                 LCID       Locale,
    _In_                 DWORD      dwInfoType,
    _In_reads_(cchSrc)   LPCSTR   lpSrcStr,
    _In_                 int        cchSrc,
    _Out_writes_(cchSrc) LPWORD     lpCharType);
#ifndef UNICODE
#define GetStringTypeEx  GetStringTypeExA
#endif


//
//  NOTE: The parameters for GetStringTypeA and GetStringTypeW are
//        NOT the same.  The W version was shipped in NT 3.1.  The
//        A version was then shipped in 16-bit OLE with the wrong
//        parameters (ported from Win95).  To be compatible, we
//        must break the relationship between the A and W versions
//        of GetStringType.  There will be NO function call for the
//        generic GetStringType.
//
//        GetStringTypeEx (above) should be used instead.
//
WINBASEAPI
BOOL
WINAPI
GetStringTypeA(
    _In_ LCID     Locale,
    _In_ DWORD    dwInfoType,
    _In_reads_(cchSrc) LPCSTR   lpSrcStr,
    _In_ int      cchSrc,
    _Out_ LPWORD  lpCharType);

WINBASEAPI
int
WINAPI
FoldStringA(
    _In_ DWORD    dwMapFlags,
    _In_reads_(cchSrc) LPCSTR  lpSrcStr,
    _In_ int      cchSrc,
    _Out_writes_opt_(cchDest) LPSTR  lpDestStr,
    _In_ int      cchDest);
#ifndef UNICODE
#define FoldString  FoldStringA
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if(WINVER >= 0x0500)

// DEPRECATED, please use Locale Names and call EnumSystemLocalesEx
WINBASEAPI
BOOL
WINAPI
EnumSystemLocalesA(
    _In_ LOCALE_ENUMPROCA lpLocaleEnumProc,
    _In_ DWORD            dwFlags);
// DEPRECATED, please use Locale Names and call EnumSystemLocalesEx
WINBASEAPI
BOOL
WINAPI
EnumSystemLocalesW(
    _In_ LOCALE_ENUMPROCW lpLocaleEnumProc,
    _In_ DWORD            dwFlags);
#ifdef UNICODE
#define EnumSystemLocales  EnumSystemLocalesW
#else
#define EnumSystemLocales  EnumSystemLocalesA
#endif // !UNICODE

#endif /* WINVER >= 0x0500 */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if(WINVER >= 0x0500)

DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
BOOL
WINAPI
EnumSystemLanguageGroupsA(
    _In_ LANGUAGEGROUP_ENUMPROCA lpLanguageGroupEnumProc,
    _In_ DWORD                   dwFlags,
    _In_ LONG_PTR                lParam);
DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
BOOL
WINAPI
EnumSystemLanguageGroupsW(
    _In_ LANGUAGEGROUP_ENUMPROCW lpLanguageGroupEnumProc,
    _In_ DWORD                   dwFlags,
    _In_ LONG_PTR                lParam);
#ifdef UNICODE
#define EnumSystemLanguageGroups  EnumSystemLanguageGroupsW
#else
#define EnumSystemLanguageGroups  EnumSystemLanguageGroupsA
#endif // !UNICODE

DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
BOOL
WINAPI
EnumLanguageGroupLocalesA(
    _In_ LANGGROUPLOCALE_ENUMPROCA lpLangGroupLocaleEnumProc,
    _In_ LGRPID                    LanguageGroup,
    _In_ DWORD                     dwFlags,
    _In_ LONG_PTR                  lParam);
DEPRECATED("The Language Group concept is obsolete and no longer supported. Set DISABLE_NLS_DEPRECATION to disable this warning.")
WINBASEAPI
BOOL
WINAPI
EnumLanguageGroupLocalesW(
    _In_ LANGGROUPLOCALE_ENUMPROCW lpLangGroupLocaleEnumProc,
    _In_ LGRPID                    LanguageGroup,
    _In_ DWORD                     dwFlags,
    _In_ LONG_PTR                  lParam);
#ifdef UNICODE
#define EnumLanguageGroupLocales  EnumLanguageGroupLocalesW
#else
#define EnumLanguageGroupLocales  EnumLanguageGroupLocalesA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

// DEPRECATED: use the user language profile instead.
WINBASEAPI
BOOL
WINAPI
EnumUILanguagesA(
    _In_ UILANGUAGE_ENUMPROCA lpUILanguageEnumProc,
    _In_ DWORD                dwFlags,
    _In_ LONG_PTR             lParam);
// DEPRECATED: use the user language profile instead.
WINBASEAPI
BOOL
WINAPI
EnumUILanguagesW(
    _In_ UILANGUAGE_ENUMPROCW lpUILanguageEnumProc,
    _In_ DWORD                dwFlags,
    _In_ LONG_PTR             lParam);
#ifdef UNICODE
#define EnumUILanguages  EnumUILanguagesW
#else
#define EnumUILanguages  EnumUILanguagesA
#endif // !UNICODE

#endif /* WINVER >= 0x0500 */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop or PC Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP  | WINAPI_PARTITION_SYSTEM)

// Please use Unicode instead.  Use of other code pages/encodings is discouraged.
WINBASEAPI
BOOL
WINAPI
EnumSystemCodePagesA(
    _In_ CODEPAGE_ENUMPROCA lpCodePageEnumProc,
    _In_ DWORD              dwFlags);
// Please use Unicode instead.  Use of other code pages/encodings is discouraged.
WINBASEAPI
BOOL
WINAPI
EnumSystemCodePagesW(
    _In_ CODEPAGE_ENUMPROCW lpCodePageEnumProc,
    _In_ DWORD              dwFlags);
#ifdef UNICODE
#define EnumSystemCodePages  EnumSystemCodePagesW
#else
#define EnumSystemCodePages  EnumSystemCodePagesA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP  | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

//
// Windows API Normalization Functions
//

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (WINVER >= 0x0600)

//
// IDN (International Domain Name) Functions
//
WINNORMALIZEAPI
int
WINAPI IdnToAscii(_In_                           DWORD    dwFlags,
                  _In_reads_(cchUnicodeChar) 	 LPCWSTR  lpUnicodeCharStr,
                  _In_                        	 int      cchUnicodeChar,
                  _Out_writes_opt_(cchASCIIChar) LPWSTR   lpASCIICharStr,
                  _In_                        	 int      cchASCIIChar);

WINNORMALIZEAPI
int
WINAPI IdnToUnicode(_In_                         	 DWORD   dwFlags,
                    _In_reads_(cchASCIIChar)    	 LPCWSTR lpASCIICharStr,
                    _In_                         	 int     cchASCIIChar,
                    _Out_writes_opt_(cchUnicodeChar) LPWSTR  lpUnicodeCharStr,
                    _In_                         	 int     cchUnicodeChar);

WINNORMALIZEAPI
int
WINAPI IdnToNameprepUnicode(_In_                            	DWORD   dwFlags,
                            _In_reads_(cchUnicodeChar)     	LPCWSTR lpUnicodeCharStr,
                            _In_                            	int     cchUnicodeChar,
                            _Out_writes_opt_(cchNameprepChar)   LPWSTR  lpNameprepCharStr,
                            _In_                            	int     cchNameprepChar);

#endif //(WINVER >= 0x0600)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (WINVER >= 0x0600)

WINNORMALIZEAPI
int
WINAPI NormalizeString( _In_                          NORM_FORM NormForm,
                        _In_reads_(cwSrcLength)      LPCWSTR   lpSrcString,
                        _In_                          int       cwSrcLength,
                        _Out_writes_opt_(cwDstLength) LPWSTR    lpDstString,
                        _In_                          int       cwDstLength );

WINNORMALIZEAPI
BOOL
WINAPI IsNormalizedString( _In_                   NORM_FORM NormForm,
                           _In_reads_(cwLength)  LPCWSTR   lpString,
                           _In_                   int       cwLength );

WINBASEAPI
BOOL
WINAPI VerifyScripts(
    _In_    DWORD   dwFlags,            // optional behavior flags
    _In_    LPCWSTR lpLocaleScripts,    // Locale list of scripts string
    _In_    int     cchLocaleScripts,   // size of locale script list string
    _In_    LPCWSTR lpTestScripts,      // test scripts string
    _In_    int     cchTestScripts);    // size of test list string

WINBASEAPI
int
WINAPI GetStringScripts(
        _In_                         DWORD   dwFlags,        // optional behavior flags
        _In_                         LPCWSTR lpString,       // Unicode character input string
        _In_                         int     cchString,      // size of input string
        _Out_writes_opt_(cchScripts) LPWSTR  lpScripts,      // Script list output string
        _In_                         int     cchScripts);    // size of output string

#endif //(WINVER >= 0x0600)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if (WINVER >= 0x0600)

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// String based NLS APIs
//

#define LOCALE_NAME_USER_DEFAULT            NULL
#define LOCALE_NAME_INVARIANT               L""
#define LOCALE_NAME_SYSTEM_DEFAULT          L"!x-sys-default-locale"

WINBASEAPI
int
WINAPI
GetLocaleInfoEx(
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ LCTYPE LCType,
    _Out_writes_to_opt_(cchData, return) LPWSTR lpLCData,
    _In_ int cchData
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or PC Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
int
WINAPI
GetCalendarInfoEx(
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ CALID Calendar,
    _In_opt_ LPCWSTR lpReserved,
    _In_ CALTYPE CalType,
    _Out_writes_opt_(cchData) LPWSTR lpCalData,
    _In_ int cchData,
    _Out_opt_ LPDWORD lpValue
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#ifndef GetDurationFormatEx_DEFINED
WINBASEAPI
int
WINAPI
GetDurationFormatEx(
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ DWORD dwFlags,
    _In_opt_ CONST SYSTEMTIME *lpDuration,
    _In_ ULONGLONG ullDuration,
    _In_opt_ LPCWSTR lpFormat,
    _Out_writes_opt_(cchDuration) LPWSTR lpDurationStr,
    _In_ int cchDuration
);
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
int
WINAPI
GetNumberFormatEx(
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ DWORD dwFlags,
    _In_ LPCWSTR lpValue,
    _In_opt_ CONST NUMBERFMTW *lpFormat,
    _Out_writes_opt_(cchNumber) LPWSTR lpNumberStr,
    _In_ int cchNumber
);

WINBASEAPI
int
WINAPI
GetCurrencyFormatEx(
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ DWORD dwFlags,
    _In_ LPCWSTR lpValue,
    _In_opt_ CONST CURRENCYFMTW *lpFormat,
    _Out_writes_opt_(cchCurrency) LPWSTR lpCurrencyStr,
    _In_ int cchCurrency
);

WINBASEAPI
int
WINAPI
GetUserDefaultLocaleName(
    _Out_writes_(cchLocaleName) LPWSTR lpLocaleName,
    _In_ int cchLocaleName
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or PC Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
int
WINAPI
GetSystemDefaultLocaleName(
    _Out_writes_(cchLocaleName) LPWSTR lpLocaleName,
    _In_ int cchLocaleName
);

WINBASEAPI
BOOL
WINAPI
IsNLSDefinedString(
    _In_ NLS_FUNCTION     Function,
    _In_ DWORD            dwFlags,
    _In_ LPNLSVERSIONINFO lpVersionInformation,
    _In_reads_(cchStr) LPCWSTR          lpString,
    _In_ INT              cchStr);

WINBASEAPI
BOOL
WINAPI
GetNLSVersionEx(
    _In_	    NLS_FUNCTION function,
    _In_opt_    LPCWSTR lpLocaleName,
    _Inout_	    LPNLSVERSIONINFOEX lpVersionInformation
);

#if (WINVER >= _WIN32_WINNT_WIN8)
WINBASEAPI
DWORD
WINAPI
IsValidNLSVersion(
    _In_        NLS_FUNCTION function,
    _In_opt_    LPCWSTR lpLocaleName,
    _In_        LPNLSVERSIONINFOEX lpVersionInformation
);
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore or Gamaes Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
int
WINAPI
FindNLSStringEx(
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ DWORD dwFindNLSStringFlags,
    _In_reads_(cchSource) LPCWSTR lpStringSource,
    _In_ int cchSource,
    _In_reads_(cchValue) LPCWSTR lpStringValue,
    _In_ int cchValue,
    _Out_opt_ LPINT pcchFound,
    _In_opt_ LPNLSVERSIONINFO lpVersionInformation,
    _In_opt_ LPVOID lpReserved,
    _In_opt_ LPARAM sortHandle
);

#if (WINVER >= _WIN32_WINNT_WIN8)
_When_((dwMapFlags & (LCMAP_SORTKEY | LCMAP_BYTEREV | LCMAP_HASH | LCMAP_SORTHANDLE)) != 0, _At_((LPBYTE) lpDestStr, _Out_writes_bytes_opt_(cchDest)))
#else
_When_((dwMapFlags & (LCMAP_SORTKEY | LCMAP_BYTEREV)) != 0, _At_((LPBYTE) lpDestStr, _Out_writes_bytes_opt_(cchDest)))
#endif
_When_(cchSrc != -1,  _At_((WCHAR *) lpSrcStr, _Out_writes_opt_(cchSrc)))
_When_(cchDest != -1, _At_((WCHAR *) lpDestStr, _Out_writes_opt_(cchDest)))
WINBASEAPI
int
WINAPI
LCMapStringEx(
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ DWORD dwMapFlags,
    _In_reads_(cchSrc) LPCWSTR lpSrcStr,
    _In_ int cchSrc,
    _Out_writes_opt_(cchDest) LPWSTR lpDestStr,
    _In_ int cchDest,
    _In_opt_ LPNLSVERSIONINFO lpVersionInformation,
    _In_opt_ LPVOID lpReserved,
    _In_opt_ LPARAM sortHandle
);

WINBASEAPI
BOOL
WINAPI
IsValidLocaleName(
    _In_ LPCWSTR lpLocaleName
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or PC Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM)

typedef BOOL (CALLBACK* CALINFO_ENUMPROCEXEX)(LPWSTR, CALID, LPWSTR, LPARAM);

WINBASEAPI
BOOL
WINAPI
EnumCalendarInfoExEx(
    _In_ CALINFO_ENUMPROCEXEX pCalInfoEnumProcExEx,
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ CALID Calendar,
    _In_opt_ LPCWSTR lpReserved,
    _In_ CALTYPE CalType,
    _In_ LPARAM lParam
);

typedef BOOL (CALLBACK* DATEFMT_ENUMPROCEXEX)(LPWSTR, CALID, LPARAM);

WINBASEAPI
BOOL
WINAPI
EnumDateFormatsExEx(
    _In_ DATEFMT_ENUMPROCEXEX lpDateFmtEnumProcExEx,
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ DWORD dwFlags,
    _In_ LPARAM lParam
);

typedef BOOL (CALLBACK* TIMEFMT_ENUMPROCEX)(LPWSTR, LPARAM);

WINBASEAPI
BOOL
WINAPI
EnumTimeFormatsEx(
    _In_ TIMEFMT_ENUMPROCEX lpTimeFmtEnumProcEx,
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ DWORD dwFlags,
    _In_ LPARAM lParam
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop or PC Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef BOOL (CALLBACK* LOCALE_ENUMPROCEX)(LPWSTR, DWORD, LPARAM);

WINBASEAPI
BOOL
WINAPI
EnumSystemLocalesEx(
    _In_ LOCALE_ENUMPROCEX lpLocaleEnumProcEx,
    _In_ DWORD dwFlags,
    _In_ LPARAM lParam,
    _In_opt_ LPVOID lpReserved
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif //(WINVER >= 0x0600)

#if (WINVER >= _WIN32_WINNT_WIN7)

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
int
WINAPI
ResolveLocaleName(
    _In_opt_                        LPCWSTR lpNameToResolve,
    _Out_writes_opt_(cchLocaleName) LPWSTR  lpLocaleName,
    _In_                            int     cchLocaleName
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // (WINVER >= _WIN32_WINNT_WIN7)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

    
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // NONLS


// Restore the original value of the 'DEPRECATED' macro");
#pragma pop_macro("DEPRECATED")

#if _MSC_VER >= 1200 
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif


#endif // _WINNLS_
