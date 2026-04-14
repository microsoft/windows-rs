//***************************************************************************
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  CHSTRING.h
//
//  Purpose: Utility library version of MFC CString
//
//***************************************************************************

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef _CHSTRING_H
#define _CHSTRING_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/////////////////////////////////////////////////////////////////////////////
#include <windows.h>
#include <limits.h>
#include <tchar.h>
#include <polarity.h>
#pragma warning( disable : 4290 ) // Ignore 'C++ Exception Specification ignored'
#include <ProvExce.h>

/////////////////////////////////////////////////////////////////////////////

struct _DOUBLE  { BYTE doubleBits[sizeof(double)]; };

#ifdef FRAMEWORK_ALLOW_DEPRECATED
void POLARITY WINAPI SetCHStringResourceHandle(HINSTANCE handle);
#endif

/////////////////////////////////////////////////////////////////////////////
// CHString formatting
/////////////////////////////////////////////////////////////////////////////
#define TCHAR_ARG   WCHAR
#define WCHAR_ARG   WCHAR
#define CHAR_ARG    char

#if defined(_68K_) || defined(_X86_)
    #define DOUBLE_ARG  _DOUBLE
#else
    #define DOUBLE_ARG  double
#endif

struct CHStringData
{
    long nRefs;
    int nDataLength;
    int nAllocLength;

    WCHAR* data()
    {
        return (WCHAR*)(this+1); 
    }
};

/////////////////////////////////////////////////////////////////////////////
class POLARITY CHString
{
    protected:

        LPWSTR m_pchData;               // pointer to ref counted string data

    protected:

                                        // implementation helpers

        CHStringData* GetData() const;  // returns data pointer
        void Init();
        void AllocCopy(CHString& dest, int nCopyLen, int nCopyIndex, int nExtraLen) const throw ( CHeap_Exception ) ;
        void AllocBuffer(int nLen) throw ( CHeap_Exception ) ;
        void AssignCopy(int nSrcLen, LPCWSTR lpszSrcData) throw ( CHeap_Exception ) ;
        void ConcatCopy(int nSrc1Len, LPCWSTR lpszSrc1Data, int nSrc2Len, LPCWSTR lpszSrc2Data) throw ( CHeap_Exception ) ;
        void ConcatInPlace(int nSrcLen, LPCWSTR lpszSrcData);
        void CopyBeforeWrite() throw ( CHeap_Exception ) ;
        void AllocBeforeWrite(int nLen) throw ( CHeap_Exception ) ;
        static inline int WINAPI SafeStrlen(LPCWSTR lpsz)   { return (lpsz == NULL) ? 0 : (int)wcslen(lpsz); }

        // Helper function used to load resource into lpszBuf buffer.
#ifdef FRAMEWORK_ALLOW_DEPRECATED
        int LoadStringW(UINT nID, _In_reads_(nMaxBuf)  LPWSTR lpszBuf, UINT nMaxBuf) throw ( CHeap_Exception ) ;
#endif

    public:

// Constructors/Destruction

        CHString();
        CHString(const CHString& stringSrc);
        CHString(WCHAR ch, int nRepeat = 1) throw ( CHeap_Exception ) ;
        CHString(LPCSTR lpsz) throw ( CHeap_Exception ) ;
        CHString(LPCWSTR lpsz) throw ( CHeap_Exception ) ;
        CHString(LPCWSTR lpch, int nLength) throw ( CHeap_Exception ) ;
        inline CHString(const unsigned char* lpsz)  { Init(); *this = (LPCSTR)lpsz; }

        ~CHString();

// Functions

        void SetAt(int nIndex, WCHAR ch) throw ( CHeap_Exception ) ;
        void Empty();    

        // inlines

        inline int GetLength() const { return GetData()->nDataLength; }
        inline BOOL IsEmpty() const  { return GetData()->nDataLength == 0; }

#if (!defined DEBUG && !defined _DEBUG)

#ifdef _PREFAST_
 #pragma prefast(push)
 #pragma prefast(disable:26018 )
#endif
        inline WCHAR GetAt(int nIndex) const{ return m_pchData[nIndex]; }
        inline WCHAR operator[](int nIndex) const{  return m_pchData[nIndex]; }
#ifdef _PREFAST_
 #pragma prefast(pop)
#endif

#else
        WCHAR GetAt(int nIndex) const;
        WCHAR operator[](int nIndex) const;
#endif
        inline operator LPCWSTR() const     { return m_pchData; }
        inline int GetAllocLength() const       { return GetData()->nAllocLength; }

// overloaded assignment

        const CHString& operator=(const CHString& stringSrc) throw ( CHeap_Exception ) ;
        const CHString& operator=(WCHAR ch) throw ( CHeap_Exception ) ;
        const CHString& operator=(LPCSTR lpsz) throw ( CHeap_Exception ) ;
        const CHString& operator=(LPCWSTR lpsz) throw ( CHeap_Exception ) ;
        inline const CHString& operator=(const unsigned char* lpsz) throw ( CHeap_Exception ) { *this = (LPCSTR)lpsz; return *this; }
        inline const CHString& operator=(CHString *p) throw ( CHeap_Exception ) { *this = *p; return *this; }
        inline const CHString& operator=(char ch) throw ( CHeap_Exception ) { *this = (WCHAR)ch; return *this; }        
        
        inline const CHString&  operator+=(char ch) throw ( CHeap_Exception ) { *this += (WCHAR)ch; return *this; }
        friend inline CHString  operator+(const CHString& string, char ch) throw ( CHeap_Exception ) { return string + (WCHAR)ch; }
        friend inline CHString  operator+(char ch, const CHString& string) throw ( CHeap_Exception ) { return (WCHAR)ch + string; }

        const CHString& operator+=(const CHString& string) throw ( CHeap_Exception ) ;
        const CHString& operator+=(WCHAR ch) throw ( CHeap_Exception ) ;
        const CHString& operator+=(LPCWSTR lpsz) throw ( CHeap_Exception ) ;

        friend CHString POLARITY WINAPI operator+(const CHString& string1,  const CHString& string2) throw ( CHeap_Exception ) ;
        friend CHString POLARITY WINAPI operator+(const CHString& string, WCHAR ch) throw ( CHeap_Exception ) ;
        friend CHString POLARITY WINAPI operator+(WCHAR ch, const CHString& string) throw ( CHeap_Exception ) ;
        friend CHString POLARITY WINAPI operator+(const CHString& string, LPCWSTR lpsz) throw ( CHeap_Exception ) ;
        friend CHString POLARITY WINAPI operator+(LPCWSTR lpsz, const CHString& string) throw ( CHeap_Exception ) ;

// string comparison

        int Compare(LPCWSTR lpsz) const;

        inline int CompareNoCase(LPCWSTR lpsz) const
        {
            // ignore case

            return _wcsicmp(m_pchData, lpsz); 

        }   // MBCS/Unicode aware

        inline int Collate(LPCWSTR lpsz) const
        {  
            // NLS aware
            // CHString::Collate is often slower than Compare but is MBSC/Unicode
            // aware as well as locale-sensitive with respect to sort order.

            return wcscoll(m_pchData, lpsz); 

        }   // locale sensitive

// Load string from resource file.

#ifdef FRAMEWORK_ALLOW_DEPRECATED
        BOOL LoadStringW(UINT nID) throw ( CHeap_Exception ) ;
#endif

// Access to string implementation buffer as "C" character array

        LPWSTR GetBuffer(int nMinBufLength) throw ( CHeap_Exception ) ;
        void ReleaseBuffer(int nNewLength = -1) throw ( CHeap_Exception ) ;
        LPWSTR GetBufferSetLength(int nNewLength) throw ( CHeap_Exception ) ;
        void FreeExtra() throw ( CHeap_Exception ) ;

// Use LockBuffer/UnlockBuffer to turn refcounting off

        LPWSTR LockBuffer() ;
        void UnlockBuffer();

// searching (return starting index, or -1 if not found)
// look for a single character match

        int Find(WCHAR ch) const;               // like "C" strchr
        int FindOneOf(LPCWSTR lpszCharSet) const;
        int ReverseFind(WCHAR ch) const;

// look for a specific sub-string

        int Find(LPCWSTR lpszSub) const;        // like "C" strstr

// upper/lower/reverse conversion

        void MakeUpper() throw ( CHeap_Exception ) ;
        void MakeLower() throw ( CHeap_Exception ) ;
        void MakeReverse() throw ( CHeap_Exception ) ;

// simple sub-string extraction

        CHString Mid(int nFirst, int nCount) const throw ( CHeap_Exception ) ;
        CHString Mid(int nFirst) const throw ( CHeap_Exception ) ;
        CHString Left(int nCount) const throw ( CHeap_Exception ) ;
        CHString Right(int nCount) const throw ( CHeap_Exception ) ;

        CHString SpanIncluding(LPCWSTR lpszCharSet) const throw ( CHeap_Exception ) ;
        CHString SpanExcluding(LPCWSTR lpszCharSet) const throw ( CHeap_Exception ) ;

// trimming whitespace (either side)

        void TrimRight() throw ( CHeap_Exception ) ;
        void TrimLeft() throw ( CHeap_Exception ) ;
    
// printf-like formatting using passed string
        void __cdecl Format(LPCWSTR lpszFormat, ...) throw ( CHeap_Exception ) ;
        void FormatV(LPCWSTR lpszFormat, va_list argList);

// printf-like formatting using referenced string resource
#ifdef FRAMEWORK_ALLOW_DEPRECATED
        void __cdecl Format(UINT nFormatID, ...) throw ( CHeap_Exception ) ;
#endif

// format using FormatMessage API on passed string
        // Warning: if you pass string inserts to this function, they must
        // be LPCSTRs on Win9x and LPCWSTRs on NT.
        void __cdecl FormatMessageW(LPCWSTR lpszFormat, ...) throw ( CHeap_Exception ) ;

// format using FormatMessage API on referenced string resource
        // Warning: if you pass string inserts to this function, they must
        // be LPCSTRs on Win9x and LPCWSTRs on NT.
#ifdef FRAMEWORK_ALLOW_DEPRECATED
        void __cdecl FormatMessageW(UINT nFormatID, ...) throw ( CHeap_Exception ) ;
#endif

#ifndef _NO_BSTR_SUPPORT

        // OLE BSTR support (use for OLE automation)

        BSTR AllocSysString() const throw ( CHeap_Exception ) ;
        static void WINAPI Release(CHStringData* pData);
        void Release();
		
#endif

};

inline BOOL operator==(const CHString& s1, const CHString& s2)  { return s1.Compare(s2) == 0; }
inline BOOL operator==(const CHString& s1, LPCWSTR s2)          { return s1.Compare(s2) == 0; }

inline BOOL operator!=(const CHString& s1, const CHString& s2)  { return s1.Compare(s2) != 0; }
inline BOOL operator!=(const CHString& s1, LPCWSTR s2)          { return s1.Compare(s2) != 0; }

inline BOOL operator<(const CHString& s1, const CHString& s2)   { return s1.Compare(s2) < 0; }
inline BOOL operator<(const CHString& s1, LPCWSTR s2)           { return s1.Compare(s2) < 0; }

inline BOOL operator>(const CHString& s1, const CHString& s2)   { return s1.Compare(s2) > 0; }
inline BOOL operator>(const CHString& s1, LPCWSTR s2)           { return s1.Compare(s2) > 0; }

inline BOOL operator<=(const CHString& s1, const CHString& s2)  { return s1.Compare(s2) <= 0; }
inline BOOL operator<=(const CHString& s1, LPCWSTR s2)          { return s1.Compare(s2) <= 0; }

inline BOOL operator>=(const CHString& s1, const CHString& s2)  { return s1.Compare(s2) >= 0; }
inline BOOL operator>=(const CHString& s1, LPCWSTR s2)          { return s1.Compare(s2) >= 0; }

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

