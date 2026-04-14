/******************************************************************
*                                                                 *
*  dontuse.h -- This module deprecates Banned APIs                *
*                                                                 *
*  Copyright (c) Microsoft Corp.  All rights reserved.            *
*                                                                 *
******************************************************************/
#ifndef _DONTUSE_H_INCLUDED_
#define _DONTUSE_H_INCLUDED_
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



#ifdef DEPRECATE_SUPPORTED
// First all the names that are a/w variants (or shouldn't be #defined by now anyway).


//Windows
#pragma deprecated(lstrcpyA)
#pragma deprecated(lstrcpyW)
#pragma deprecated(lstrcpynA)
#pragma deprecated(lstrcpynW)
#pragma deprecated(lstrcatA)
#pragma deprecated(lstrcatW)
#pragma deprecated(lstrcatnA)
#pragma deprecated(lstrcatnW)
#pragma deprecated(wsprintfA)
#pragma deprecated(wsprintfW)
#pragma deprecated(wvsprintfA)
#pragma deprecated(wvsprintfW)

//shlwapi
#ifndef _STRSAFE_H_INCLUDED_
#pragma deprecated(StrCpyW)
#endif
#pragma deprecated(StrNCpyW)
#pragma deprecated(StrCpyNW)
#pragma deprecated(StrCatW)
#pragma deprecated(StrNCatW)
#pragma deprecated(StrCatNW)
#pragma deprecated(wnsprintf)
#ifndef _STRSAFE_H_INCLUDED_
#pragma deprecated(wnsprintfW)
#endif
#pragma deprecated(wvnsprintfW)
#pragma deprecated(StrCatBuffW)


#if defined(_MSC_VER) && (_MSC_VER < 1700)

//crt
#pragma deprecated(gets)
#pragma deprecated(_getws)
#pragma deprecated(strcpy)
#pragma deprecated(wcscpy)
#pragma deprecated(strncat)
#pragma deprecated(wcsncat)
#pragma deprecated(strcat)
#pragma deprecated(wcscat)
#pragma deprecated(strncpy)
#pragma deprecated(wcsncpy)
#pragma deprecated(_snprintf)
#pragma deprecated(_snwprintf)
#pragma deprecated(_vsnprintf)
#pragma deprecated(_vsnwprintf)
#pragma deprecated(vsprintf)
#pragma deprecated(vswprintf)
#pragma deprecated(sprintf)
#pragma deprecated(swprintf)
#pragma deprecated(_makepath)
#pragma deprecated(_wmakepath)
#pragma deprecated(_splitpath)
#pragma deprecated(_wsplitpath)

#pragma deprecated(sprintfA)
#pragma deprecated(sprintfW)
#pragma deprecated(strcatA)
#pragma deprecated(strcatW)
#pragma deprecated(strcpyA)
#pragma deprecated(strcpyW)
#pragma deprecated(nsprintf)
#pragma deprecated(IsBadCodePtr)
#pragma deprecated(IsBadHugeReadPtr)
#pragma deprecated(IsBadHugeWritePtr)
#pragma deprecated(IsBadReadPtr)
#pragma deprecated(IsBadStringPtr)
#pragma deprecated(IsBadWritePtr)
#pragma deprecated(_ftccat)
#pragma deprecated("_ftccpy")
#pragma deprecated(_tccat)
#pragma deprecated("_tccpy")
#pragma deprecated(makepath)
#pragma deprecated(StrCatChainW)
#pragma deprecated(ualstrcpyW)

#pragma deprecated(scanf)
#pragma deprecated(wscanf)
#pragma deprecated(sscanf)
#pragma deprecated(swscanf)
#pragma deprecated(_snscanf)
#pragma deprecated(_snwscanf)
#pragma deprecated(snscanf)
#pragma deprecated("swscanf")

#endif // defined(_MSC_VER) && (_MSC_VER < 1700)

// Then all the windows.h names - we need to undef and redef based on UNICODE setting
#undef lstrcat
#undef lstrcatn
#undef lstrcpy
#undef lstrcpyn
#undef wsprintf
#undef wvsprintf
#pragma deprecated(lstrcat)
#pragma deprecated(lstrcatn)
#pragma deprecated(lstrcpy)
#pragma deprecated(lstrcpyn)
#pragma deprecated(wsprintf)
#pragma deprecated(wvsprintf)
#ifdef UNICODE
#define lstrcat    lstrcatW
#define lstrcatn   lstrcatnW
#define lstrcpy    lstrcpyW
#define lstrcpyn   lstrcpynW
#define wsprintf   wsprintfW
#define wvsprintf  wvsprintfW
#else
#define lstrcat    lstrcatA
#define lstrcatn   lstrcatnA
#define lstrcpy    lstrcpyA
#define lstrcpyn   lstrcpynA
#define wsprintf   wsprintfA
#define wvsprintf  wvsprintfA
#endif



// Then the shlwapi names - they key off UNICODE also.

#undef StrCpyA
#undef StrCatA
#undef StrCpy
#undef StrCat
#undef StrNCat
#undef StrCatN
#undef StrCpyN
#undef StrCatNA
#undef StrCpyNA
#undef StrNCpy
#undef wvnsprintfA
#undef wvnsprintf
#undef wnsprintfA
#undef wnsprintf
#undef StrCatBuffA
#undef StrCatBuff
#pragma deprecated(StrCatNA)
#pragma deprecated(StrCpyNA)
#pragma deprecated(StrCpyA)
#pragma deprecated(StrCatA)
#pragma deprecated(StrCat)
#pragma deprecated(StrCpy)
#pragma deprecated(StrNCat)
#pragma deprecated(StrCatN)
#pragma deprecated(StrNCpy)
#pragma deprecated(StrCpyN)
#pragma deprecated(StrNCpyA)
#pragma deprecated(StrNCatA)
#pragma deprecated(wvnsprintfA)
#pragma deprecated(wvnsprintf)
#pragma deprecated(wnsprintfA)
#pragma deprecated("wnsprintf")
#pragma deprecated(StrCatBuffA)
#pragma deprecated(StrCatBuff)
#define StrCpyA     lstrcpyA
#define StrCatA     lstrcatA
#define StrCatN     StrNCat
#define StrCpyN	    StrNCpy
#define wvnsprintfA vsprintf
#define wnsprintfA  wsprintf
#define StrCatBuffA lstrcatA
#ifdef UNICODE
#define StrCat      StrCatW
#define StrNCat     StrNCatW
#define StrCpy      StrCpyW
#define StrNCpy     StrNCpyW
#define wvnsprintf  vswprintf
#define wnsprintf   wsprintf
#define StrCatBuff  StrCatW
#else
#define StrCat      lstrcatA
#define StrNCat     StrNCatA
#define StrCpy      lstrcpyA
#define StrNCpy	    StrNCpyA
#define wvnsprintf  vsprintf
#define wnsprintf   vsprintf
#define StrCatBuff  lstrcatA
#endif


#if defined(_MSC_VER) && (_MSC_VER < 1700)

// Then all the CRT names - we need to undef/redef based on _UNICODE value.

#undef _getts
#undef _tcscat
#undef _tcsncat
#undef _tcscpy
#undef _tcsncpy
#undef _stprintf
#undef _vsntprintf
#undef _vstprintf
#undef _ftcscpy
#undef _ftcscat
#undef _fstrcpy
#undef _fstrcat
#undef _fstrncat
#undef _fstrncpy
#undef _sntprintf
#undef _tmakepath
#undef _tsplitpath
#undef _tscanf
#undef _stscanf
#undef _sntscanf
#pragma deprecated(_getts)
#pragma deprecated(_tcscat)
#pragma deprecated(_tcsncat)
#pragma deprecated(_tcscpy)
#pragma deprecated(_tcsncpy)
#pragma deprecated(_stprintf)
#pragma deprecated(_vsntprintf)
#pragma deprecated(_vstprintf)
#pragma deprecated("_ftcscpy")
#pragma deprecated("_ftcscat")
#pragma deprecated(_fstrcpy)
#pragma deprecated(_fstrcat)
#pragma deprecated(_fstrncat)
#pragma deprecated(_fstrncpy)
#pragma deprecated(_sntprintf)
#pragma deprecated(_tmakepath)
#pragma deprecated(_tsplitpath)
#pragma deprecated(_tscanf)
#pragma deprecated(_stscanf)
#pragma deprecated(_sntscanf)
#ifdef _UNICODE
#define _getts      _getws
#define _tcscat	    wcscat
#define _tscncat    wcsncat
#define _tcscpy	    wcscpy
#define _tcsncpy    wcsncpy
#define _sntprintf  _snwprintf
#define _vsntprintf _vsnwprintf
#define _vstprintf  vswprintf
#define _ftcscpy    wcscpy
#define _ftcscat    wcscat
#define _fstrcpy    wcscpy
#define _fstrcat    wcscat
#define _fstrncpy   wcsncpy
#define _fstrncat   wcsncat
#define _fstrcpy    wcscpy
#define _stprintf   swprintf
#define _tmakepath  _wmakepath
#define _tsplitpath _wsplitpath
#define _tscanf	    wscanf
#define _stscanf    swscanf
#define _sntscanf   snwscanf
#else
#define _getts	    gets
#define _tcscat	    strcat
#define _tcsncat    strncat
#define _tcscpy     strcpy
#define _tcsncpy    strncpy
#define _sntprintf  _snprintf
#define _vsntprintf _vsnprintf
#define _vstprintf  vsprintf
#define _ftcscpy    strcpy
#define _ftcscat    strcat
#define _fstrcpy    strcpy
#define _fstrcat    strcat
#define _fstrncpy   strncpy
#define _fstrncat   strncat
#define _fstrcpy    strcpy
#define _stprintf   vsprintf
#define _tmakepath  _makepath
#define _tsplitpath _splitpath
#define _tscanf	    scanf
#define	_stscanf    sscanf
#define _sntscanf   snscanf
#endif

#endif // defined(_MSC_VER) && (_MSC_VER < 1700)

#else // DEPRECATE_SUPPORTED

// WINDOWS

#undef lstrcpy
#define lstrcpy           lstrcpy_instead_use_strcpy_s_or_StringCchCopy_or_StringCbCopy
#undef lstrcpyA
#define lstrcpyA          lstrcpyA_instead_use_strcpy_s_or_StringCchCopyA_or_StringCbCopyA
#undef lstrcpyW
#define lstrcpyW          lstrcpyW_instead_use_wcscpy_s_or_StringCchCopyW_or_StringCbCopyW

#undef lstrcpyn
#define lstrcpyn          lstrcpyn_instead_use_strncpy_s_or_StringCchCopyN_or_StringCbCopyN
#undef lstrcpynA
#define lstrcpynA         lstrcpynA_instead_use_strncpy_s_or_StringCchCopyNA_or_StringCbCopyNA
#undef lstrcpynW
#define lstrcpynW         lstrcpynW_instead_use_wcsncpy_s_or_StringCchCopyNW_or_StringCbCopyNW

#undef lstrcat
#define lstrcat           lstrcat_instead_use_strcat_s_or_StringCchCat_or_StringCbCat
#undef lstrcatA
#define lstrcatA          lstrcatA_instead_use_strcat_s_or_StringCchCatA_or_StringCbCatA
#undef lstrcatW
#define lstrcatW          lstrcatW_instead_use_wcscat_s_or_StringCchCatW_or_StringCbCatW

#undef lstrcatn
#define lstrcatn          lstrcatn_instead_use_strncat_s_or_StringCchCatN_or_StringCbCatN
#undef lstrcatnA
#define lstrcatnA         lstrcatnA_instead_use_strncat_s_or_StringCchCatNA_or_StringCbCatNA
#undef lstrcatnW
#define lstrcatnW         lstrcatnW_instead_use_wcsncat_s_or_StringCchCatNW_or_StringCbCatNW

#undef wsprintf
#define wsprintf          wsprintf_instead_use_sprintf_s_or_StringCchPrintf_or_StringCbPrintf
#undef wsprintfA
#define wsprintfA         wsprintfA_instead_use_sprintf_s_or_StringCchPrintfA_or_StringCbPrintfA
#undef wsprintfW
#define wsprintfW         wsprintfW_instead_use_sprintf_s_or_StringCchPrintfW_or_StringCbPrintfW

#undef wvsprintf
#define wvsprintf         wvsprintf_instead_use_vsprintf_s_or_StringCchVPrintf_or_StringCbVPrintf
#undef wvsprintfA
#define wvsprintfA        wvsprintfA_instead_use_vsprintf_s_or_StringCchVPrintfA_or_StringCbVPrintfA
#undef wvsprintfW
#define wvsprintfW        wvsprintfW_instead_use_swprintf_s_or_StringCchVPrintfW_or_StringCbVPrintfW



// SHLWAPI

#undef StrCpy
#define StrCpy            StrCpy_instead_use_strcpy_s_or_StringCchCopy_or_StringCbCopy
#undef StrCpyA
#define StrCpyA           StrCpyA_instead_use_strcpy_s_or_StringCchCopyA_or_StringCbCopyA
#undef StrCpyW
#define StrCpyW           StrCpyW_instead_use_wcscpy_s_or_StringCchCopyW_or_StringCbCopyW

#undef StrNCpy
#define StrNCpy           StrNCpy_instead_use_strncpy_s_or_StringCchCopyN_or_StringCbCopyN
#undef StrNCpyA
#define StrNCpyA          StrNCpyA_instead_use_strncpy_s_or_StringCchCopyNA_or_StringCbCopyNA
#undef StrNCpyW
#define StrNCpyW          StrNCpyW_instead_use_wcsncpy_s_or_StringCchCopyNW_or_StringCbCopyNW

#undef StrCpyN
#define StrCpyN           StrCpyN_instead_use_strncpy_s_or_StringCchCopyN_or_StringCbCopyN
#undef StrCpyNA
#define StrCpyNA          StrCpyNA_instead_use_strncpy_s_or_StringCchCopyNA_or_StringCbCopyNA
#undef StrCpyNW
#define StrCpyNW          StrCpyNW_instead_use_wcsncpy_s_or_StringCchCopyNW_or_StringCbCopyNW

#undef StrCat
#define StrCat            StrCat_instead_use_strcat_s_or_StringCchCat_or_StringCbCat
#undef StrCatA
#define StrCatA           StrCatA_instead_use_strcat_s_or_StringCchCatA_or_StringCbCatA
#undef StrCatW
#define StrCatW           StrCatW_instead_use_wcscat_s_or_StringCchCatW_or_StringCbCatW

#undef StrNCat
#define StrNCat           StrNCat_instead_use_strncat_s_or_StringCchCatN_or_StringCbCatN
#undef StrNCatA
#define StrNCatA          StrNCatA_instead_use_strncat_s_or_StringCchCatNA_or_StringCbCatNA
#undef StrNCatW
#define StrNCatW          StrNCatW_instead_use_wcsncat_s_or_StringCchCatNW_or_StringCbCatNW

#undef StrCatN
#define StrCatN           StrCatN_instead_use_strncat_s_or_StringCchCatN_or_StringCbCatN
#undef StrCatNA
#define StrCatNA          StrCatNA_instead_use_strncat_s_or_StringCchCatNA_or_StringCbCatNA
#undef StrCatNW
#define StrCatNW          StrCatNW_instead_use_wcsncat_s_or_StringCchCatNW_or_StringCbCatNW

#undef wnsprintf
#define wnsprintf         wnsprintf_instead_use_vsprintf_s_or_StringCchPrintf_or_StringCbPrintf
#undef wnsprintfA
#define wnsprintfA        wnsprintfA_instead_use_vsprintf_s_or_StringCchPrintfA_or_StringCbPrintfA
#undef wnsprintfW
#define wnsprintfW        wnsprintfW_instead_use_vswprintf_s_or_StringCchPrintfW_or_StringCbPrintfW

#undef wvnsprintf
#define wvnsprintf        wvnsprintf_instead_use_vsprintf_s_or_StringCchVPrintf_or_StringCbVPrintf
#undef wvnsprintfA
#define wvnsprintfA       wvnsprintfA_instead_use_vsprintf_s_or_StringCchVPrintfA_or_StringCbVPrintfA
#undef wvnsprintfW
#define wvnsprintfW       wvnsprintfW_instead_use_vswprintf_s_or_StringCchVPrintfW_or_StringCbVPrintfW

#undef StrCatBuff
#define StrCatBuff        StrCatBuff_instead_use_strcat_s_or_StringCchCat_or_StringCbCat
#undef StrCatBuffA
#define StrCatBuffA       StrCatBuffA_instead_use_strcat_s_or_StringCchCatA_or_StringCbCatA
#undef StrCatBuffW
#define StrCatBuffW       StrCatBuffW_instead_use_strcat_s_or_StringCchCatW_or_StringCbCatW


// CRT

#undef gets
#define gets              _gets_instead_use__gets_or_StringCchGets_or_StringCbGets
#undef _getws
#define _getws            _getws_instead_use__getWs_or_StringCchGetsW_or_StringCbGetsW
#undef _getts
#define _getts            _getts_instead_use__gets_or_StringCchGets_or_StringCbGets

#undef strcpy
#define strcpy            strcpy_instead_use_strcpy_s_or_StringCchCopy_or_StringCbCopy
#undef wcscpy
#define wcscpy            wcscpy_instead_use_wcscpy_s_or_StringCchCopyW_or_StringCbCopyW
#undef _tcscpy
#define _tcscpy           _tcscpy_instead_use_strcpy_s_or_StringCchCopy_or_StringCbCopy

#undef strcat
#define strcat            strcat_instead_use_strcat_s_or_StringCchCat_or_StringCbCat
#undef wcscat
#define wcscat            wcscat_instead_use_wcscat_s_or_StringCchCatW_or_StringCbCatW
#undef _tcscat
#define _tcscat           _tcscat_instead_use_strcat_s_or_StringCchCat_or_StringCbCat

#undef strncat
#define strncat           strncat_instead_use_strncat_s_or_StringCchCatN_or_StringCbCatN
#undef wcsncat
#define wcsncat           wcsncat_instead_use_wcsncat_s_or_StringCchCatNW_or_StringCbCatNW
#undef _tcsncat
#define _tcsncat          _tcsncat_instead_use_strncat_s_or_StringCchCatN_or_StringCbCatN

#undef strncpy
#define strncpy           strncpy_instead_use_strncpy_s_or_StringCchCpyN_or_StringCbCpyN
#undef wcsncpy
#define wcsncpy           wcsncpy_instead_use_wcsncpy_s_or_StringCchCpyNW_or_StringCbCpyNW
#undef _tcsncpy
#define _tcsncpy          _tcsncpy_instead_use_strncpy_s_or_StringCchCpyN_or_StringCbCpyN

#undef _snprintf
#define _snprintf         _snprintf_instead_use_snprintf_s_or_StringCchPrintf_or_StringCbPrintf
#undef _snwprintf
#define _snwprintf        _snwprintf_instead_use_snwprintf_s_or_StringCchPrintfW_or_StringCbPrintfW
#undef _sntprintf
#define _sntprintf        _sntprintf_instead_use_sprintf_s_or_StringCchPrintf_or_StringCbPrintf

#undef _vsnprintf
#define _vsnprintf        _vsnprintf_instead_use_vsnprintf_s_or_StringCchVPrintf_or_StringCbVPrintf
#undef _vsnwprintf
#define _vsnwprintf       _vsnwprintf_instead_use_vsnwprintf_s_or_StringCchVPrintfW_or_StringCbVPrintfW
#undef _vsntprintf
#define _vsntprintf       _vsntprintf_instead_use_vsprintf_s_or_StringCchVPrintf_or_StringCbVPrintf

#undef vsprintf
#define vsprintf          vsprintf_instead_use_vsprintf_s_or_StringCchVPrintf_or_StringCbVPrintf
#undef vswprintf
#define vswprintf         vswprintf_instead_use_vswprintf_s_or_StringCchVPrintfW_or_StringCbVPrintfW
#undef _vstprintf
#define _vstprintf        _vstprintf_instead_use_vsprintf_s_or_StringCchVPrintf_or_StringCbVPrintf

#undef sprintf
#define sprintf           sprintf_instead_use_sprintf_s_or_StringCchPrintf_or_StringCbPrintf
#undef swprintf
#define swprintf          swprintf_instead_use_swprintf_s_or_StringCchPrintfW_or_StringCbPrintfW
#undef _stprintf
#define _stprintf         _stprintf_instead_use_vsprintf_s_or_StringCchPrintf_or_StringCbPrintf

#undef _makepath
#define _makepath         _makepath_instead_use_makepath_s
#undef _wmakepath
#define _wmakepath        _wmakepath_instead_use_wmakepath_s
#undef _tmakepath
#define _tmakepath        _tmakepath_instead_use_tmakepath_s

#undef _splitpath
#define _splitpath        _splitpath_instead_use_splitpath_s
#undef _wsplitpath
#define _wsplitpath       _wsplitpath_instead_use_wsplitpath_s
#undef _tsplitpath
#define _tsplitpath       _tsplitpath_instead_use_tsplitpath_s

#undef scanf
#define scanf             scanf_instead_use_scanf_S
#undef wscanf
#define wscanf            wscanf_instead_use_wscanf_S
#undef _tscanf
#define _tscanf           _tscanf_instead_use_scanf_S

#undef sscanf
#define sscanf            sscanf_instead_use_sscanf_S
#undef swscanf
#define swscanf           swscanf_instead_use_swscanf_s
#undef _stscanf
#define _stscanf          _stscanf_instead_use_sscanf_S

#undef _snscanf
#define _snscanf          _snscanf_instead_use__snscanf_s
#undef _snwscanf
#define _snwscanf         _snwscanf_instead_use__snwscanf_s
#undef snscanf
#define snscanf           snscanf_instead_use__snscanf_s
#undef swscanf
#define swscanf           swscanf_instead_use__snwscanf_s
#undef _sntscanf
#define _sntscanf         _sntscanf_instead_use__snscanf_s

#undef _ftcscat
#define _ftcscat          _ftcscat_instead_use_strcat_s_or_StringCchCat_or_StringCbCat
#undef _ftcscpy
#define _ftcscpy          _ftcscpy_instead_use_strcpy_s_or_StringCchCopy_or_StringCbCopy
#undef _fstrcat
#define _fstrcat          _fstrcat_instead_use_strcat_s_or_StringCchCat_or_StringCbCat
#undef _fstrcpy
#define _fstrcpy          _fstrcpy_instead_use_strcpy_s_or_StringCchCopy_or_StringCbCopy
#undef _fstrncat
#define _fstrncat         _fstrncat_instead_use_strncat_s_or_StringCchCatN_or_StringCbCatN
#undef _fstrncpy
#define _fstrncpy         _fstrncpy_instead_use_strncpy_s_or_StringCchCpyN_or_StringCbCpyN



// MISC

#undef sprintfA
#define sprintfA          sprintfA_instead_use_sprintf_s_or_StringCchPrintfA_or_StringCbPrintfA
#undef sprintfW
#define sprintfW          sprintfW_instead_use_swprintf_s_or_StringCchPrintfW_or_StringCbPrintfW

#undef strcpyA
#define strcpyA           strcpyA_instead_use_strcpy_s_or_StringCchCopyA_or_StringCbCopyA
#undef strcpyW
#define strcpyW           strcpyW_instead_use_wcscpy_s_or_StringCchCopyW_or_StringCbCopyW

#undef strcatA
#define strcatA           strcatA_instead_use_strcat_s_or_StringCchCatA_or_StringCbCatA
#undef strcatW
#define strcatW           strcatW_instead_use_wcscat_s_or_StringCchCatW_or_StringCbCatW

#undef nsprintf
#define nsprintf          nsprintf_instead_use_vsprintf_s_or_StringCchPrintf_or_StringCbPrintf

#undef IsBadCodePtr
#define IsBadCodePtr      IsBadCodePtr_is_no_longer_supported

#undef IsBadHugeReadPtr
#define IsBadHugeReadPtr  IsBadHugeReadPtr_is_no_longer_supported
	
#undef IsBadHugeWritePtr
#define IsBadHugeWritePtr IsBadHugeWritePtr_is_no_longer_supported

#undef IsBadReadPtr
#define IsBadReadPtr      IsBadReadPtr_is_no_longer_supported

#undef IsBadStringPtr
#define IsBadStringPtr    IsBadStringPtr_is_no_longer_supported

#undef IsBadWritePtr
#define IsBadWritePtr     IsBadWritePtr_is_no_longer_supported

#undef _ftccat
#define _ftccat           _ftccat_instead_use_strcat_s_or_StringCchCat_or_StringCbCat

#undef _ftccpy
#define _ftccpy           _ftccpy_instead_use_strcpy_s_or_StringCchCopy_or_StringCbCopy

#undef _tccat
#define _tccat            _tccat_instead_use_strcat_s_or_StringCchCat_or_StringCbCat

#undef _tccpy
#define _tccpy            _tccpy_instead_use_strcpy_s_or_StringCchCopy_or_StringCbCopy

#undef makepath
#define makepath          makepath_instead_use_makepath_s_or_wmakepath_s

#undef StrCatChainW	
#define StrCatChainW      StrCatChainW_is_no_longer_supported

#undef ualstrcpyW
#define ualstrcpyW        ualstrcpyW_is_no_longer_supported


#endif  // !DEPRECATE_SUPPORTED


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // _DONTUSE_H_INCLUDED_
