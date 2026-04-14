//+---------------------------------------------------------------------------
//
//  Copyright (C) Microsoft Corporation. All Rights Reserved
//
//
//  File:       mapiUnicodeHelp.h
//
//  Contents:   This file provides a function MAPISendMailHelper which takes mail message information
//              as Unicode and will pass to the registered mail provider, converting to ANSI/UTF8 if necessary.
//              This function will work on Windows 7 and below, on later platforms mapi32!MAPISendMailW can be called directly.
//
//----------------------------------------------------------------------------

#pragma once
#include <new>
#include <winapifamily.h>
#include <Shellapi.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// Main helper function declaration:
ULONG MAPISendMailHelper(_In_  LHANDLE lhSession, _In_  ULONG_PTR ulUIParam, _In_  lpMapiMessageW lpMessage, _In_  FLAGS flFlags, _In_  ULONG ulReserved);

// Declarations of other functions used:
HMODULE LoadDefaultMailProvider(_Out_ bool *pfSupportUTF8);
HRESULT ConvertMessageFromUnicode(_In_ const MapiMessageW *pMessageIn, _Outptr_ MapiMessage **ppMessageOut, bool fSupportUTF8);
void FreeMessage(_Inout_opt_ MapiMessage* pMessage);


__inline ULONG MAPISendMailHelper(
  _In_  LHANDLE lhSession,
  _In_  ULONG_PTR ulUIParam,
  _In_  lpMapiMessageW lpMessage,
  _In_  FLAGS flFlags,
  _In_  ULONG ulReserved)
{
    ULONG ulRet = MAPI_E_FAILURE; // Default return code

    // Load the MAPI stub dll, which itself delegates to the default mail provider:
    HMODULE hMapi32 = LoadLibraryW(L"mapi32.dll");
    if (hMapi32)
    {
        LPMAPISENDMAILW pfnMapiSendMailW = (LPMAPISENDMAILW)GetProcAddress(hMapi32, "MAPISendMailW");
        if (pfnMapiSendMailW)
        {
            // Mapi32 supports MAPISendMailW directly - call it:
            ulRet = pfnMapiSendMailW(lhSession, ulUIParam, lpMessage, flFlags, ulReserved);
        }
        else
        {
            // Mapi32 does not support MAPISendMailW, see if the mapi provider {mail client} does:
            bool fMadeCall = false;
            bool fSupportsUTF8 = false;
            HMODULE hMapiProvider = LoadDefaultMailProvider(&fSupportsUTF8);
            if (hMapiProvider)
            {
                LPMAPISENDMAILW pfnMapiSendMailProviderW = (LPMAPISENDMAILW)GetProcAddress(hMapiProvider, "MAPISendMailW");
                if (pfnMapiSendMailProviderW)
                {
                    // Mapi provider supports MAPISendMailW - call it:
                    ulRet = pfnMapiSendMailProviderW(lhSession, ulUIParam, lpMessage, flFlags, ulReserved);
                    fMadeCall = true;
                }
                FreeLibrary(hMapiProvider);
            }
            // If we failed to load the provider or it did not support MAPISendMailW, then convert to ansi and call the mapi32 MAPISendMail directly.
            if (!fMadeCall && ((flFlags & MAPI_FORCE_UNICODE) != MAPI_FORCE_UNICODE))
            {
                MapiMessage *pMapiMessageA;
                if (SUCCEEDED(ConvertMessageFromUnicode(lpMessage, &pMapiMessageA, fSupportsUTF8)))
                {
                    LPMAPISENDMAIL pfnMapiSendMailA = (LPMAPISENDMAIL)GetProcAddress(hMapi32, "MAPISendMail");
                    if (pfnMapiSendMailA)
                    {
                        ulRet = pfnMapiSendMailA(lhSession, ulUIParam, pMapiMessageA, flFlags, ulReserved); 
                    }
                    FreeMessage(pMapiMessageA);
                }
            }
        }

        FreeLibrary(hMapi32);
    }

    return ulRet;
}


// Allocates space for nFileDesc MapiFileDesc structures and zeros out their memory.
__inline HRESULT AllocFileDesc(ULONG nFileDesc, _Outptr_result_buffer_(nFileDesc) MapiFileDesc** ppFileDesc)
{
    HRESULT hr = E_INVALIDARG;
    if (ppFileDesc && (nFileDesc > 0))
    {
        *ppFileDesc = NULL;

        MapiFileDesc* pNewFileDesc = new (std::nothrow) MapiFileDesc[nFileDesc];
        if (pNewFileDesc == NULL)
        {
            hr = E_OUTOFMEMORY;
        }
        else
        {
            ZeroMemory(pNewFileDesc, nFileDesc * sizeof(MapiFileDesc));
            *ppFileDesc = pNewFileDesc;
             hr = S_OK;
        }
    }

    return hr;
}

// Frees memory in embedded pointers in a MapiFileDesc array.
__inline void FreeFileDesc(_Inout_updates_opt_(nFileDesc) MapiFileDesc* pFileDesc, ULONG nFileDesc)
{
    if (pFileDesc)
    {
        for (ULONG i = 0; i < nFileDesc; i++)
        {
            delete[] pFileDesc[i].lpszPathName;
            delete[] pFileDesc[i].lpszFileName;

            pFileDesc[i].ulReserved = 0;
            pFileDesc[i].flFlags = 0;
            pFileDesc[i].nPosition = 0;
            pFileDesc[i].lpszPathName = 0;
            pFileDesc[i].lpszFileName = 0;
            pFileDesc[i].lpFileType = NULL; // Don't free this as not allocated initially
        }
    }
}

// Allocates space for nRecipDesc MapiRecipDesc structures and zeros out their memory.
__inline HRESULT AllocRecipDesc(ULONG nRecipDesc, _Outptr_result_buffer_(nRecipDesc) MapiRecipDesc** ppRecipDesc)
{
    HRESULT hr = E_INVALIDARG;

    if (ppRecipDesc && (nRecipDesc > 0))
    {
        *ppRecipDesc = NULL;

        MapiRecipDesc* pNewRecipDesc = new (std::nothrow) MapiRecipDesc[nRecipDesc];

        if (pNewRecipDesc == NULL)
        {
            hr = E_OUTOFMEMORY;
        }
        else
        {
            ZeroMemory(pNewRecipDesc, nRecipDesc * sizeof(MapiRecipDesc));
            *ppRecipDesc = pNewRecipDesc;
            hr = S_OK;
        }
    }
    return hr;
}

// Frees memory in embedded pointers in a MapiRecipDesc array.
__inline void FreeRecipDesc(_Inout_updates_opt_(nRecipDesc) MapiRecipDesc* pRecipDesc, ULONG nRecipDesc)
{
    if (pRecipDesc)
    {
        for (ULONG i = 0; i < nRecipDesc; i++)
        {
            delete[] pRecipDesc[i].lpszName;
            delete[] pRecipDesc[i].lpszAddress;
            delete[] static_cast<BYTE*>(pRecipDesc[i].lpEntryID);

            pRecipDesc[i].ulReserved = 0;
            pRecipDesc[i].ulRecipClass = 0;
            pRecipDesc[i].lpszName = 0;
            pRecipDesc[i].lpszAddress = 0;
            pRecipDesc[i].ulEIDSize = 0;
            pRecipDesc[i].lpEntryID = 0;
        }
    }
}

// Frees MapiMessage object and embedded pointers. Can be passed NULL.
__inline void FreeMessage(_Inout_opt_ MapiMessage* pMessage)
{
    if (pMessage)
    {
        FreeRecipDesc(pMessage->lpOriginator, 1);
        FreeRecipDesc(pMessage->lpRecips, pMessage->nRecipCount);
        FreeFileDesc(pMessage->lpFiles, pMessage->nFileCount);

        delete[] pMessage->lpszSubject;
        delete[] pMessage->lpszNoteText;
        delete[] pMessage->lpszMessageType;
        delete[] pMessage->lpszDateReceived;
        delete[] pMessage->lpszConversationID;
        delete[] pMessage->lpOriginator;
        delete[] pMessage->lpRecips;
        delete[] pMessage->lpFiles;

        pMessage->ulReserved = 0;
        pMessage->lpszSubject = 0;
        pMessage->lpszNoteText = 0;
        pMessage->lpszMessageType = 0;
        pMessage->lpszDateReceived = 0;
        pMessage->lpszConversationID = 0;
        pMessage->flFlags = 0;
        pMessage->lpOriginator = 0;
        pMessage->nRecipCount = 0;
        pMessage->lpRecips = 0;
        pMessage->nFileCount = 0;
        pMessage->lpFiles = 0;

        delete pMessage;
    }
}

// Convert Unicode string to ANSI/UTF8.
__inline HRESULT ConvertStringToMultiByteEx(PCWSTR pszIn, _Outptr_ PSTR *ppszOut, bool fSupportUTF8, _Out_opt_ bool *pfUsedDefaultChar)
{
    HRESULT hr = S_OK;
    *ppszOut = NULL;
    if (pfUsedDefaultChar)
    {
        *pfUsedDefaultChar = false;
    }

    if (pszIn)
    {
        BOOL fUsedDefaultChar = FALSE;
        int iLen = WideCharToMultiByte(fSupportUTF8 ? CP_UTF8 : CP_ACP, fSupportUTF8 ? WC_ERR_INVALID_CHARS : 0, pszIn, -1, NULL, 0, NULL, fSupportUTF8 ? NULL : &fUsedDefaultChar);

        if (iLen)
        {
            *ppszOut = new (std::nothrow) char[iLen];
            if (*ppszOut)
            {
                if (0 == WideCharToMultiByte(fSupportUTF8 ? CP_UTF8 : CP_ACP, fSupportUTF8 ? WC_ERR_INVALID_CHARS : 0, pszIn, -1, *ppszOut, iLen, NULL, fSupportUTF8 ? NULL : &fUsedDefaultChar))
                {
                    delete[] *ppszOut;
                    *ppszOut = NULL;
                    hr = HRESULT_FROM_WIN32(GetLastError());
                }
                else if (pfUsedDefaultChar)
                {
                    *pfUsedDefaultChar = (fUsedDefaultChar != FALSE);
                }
            }
            else
            {
                hr = E_OUTOFMEMORY;
            }
        }
        else
        {
            hr = HRESULT_FROM_WIN32(GetLastError());
        }
    }

    return hr;
}

// Convert Unicode string to ANSI.
__inline HRESULT ConvertStringToMultiByte(PCWSTR pszIn, _Outptr_result_maybenull_ PSTR *ppszOut)
{
    return ConvertStringToMultiByteEx(pszIn, ppszOut, false, NULL);
}

// Convert a file path from Unicode to ANSI.
// If the conversion cannot be performed then try converting to the short-filename representation first.
__inline HRESULT ConvertFilePathFromUnicode(PCWSTR pszIn, _Outptr_result_maybenull_ PSTR *ppszOut)
{
    *ppszOut = NULL;

    bool fUsedDefaultChar = false;
    HRESULT hr = ConvertStringToMultiByteEx(pszIn, ppszOut, false, &fUsedDefaultChar);
    if (SUCCEEDED(hr) && fUsedDefaultChar)
    {
        // Invalid conversion - try using GetShortPathName:
        delete[] *ppszOut;
        *ppszOut = NULL;
        wchar_t wzTemp[MAX_PATH];
        DWORD dwRes = GetShortPathNameW(pszIn, wzTemp, ARRAYSIZE(wzTemp));
        if (dwRes >= ARRAYSIZE(wzTemp))
        {
            hr = E_INVALIDARG;
        }
        else if (dwRes == 0)
        {
            hr = HRESULT_FROM_WIN32(GetLastError());
        }
        else
        {
            hr = ConvertStringToMultiByteEx(wzTemp, ppszOut, false, &fUsedDefaultChar);
            if (SUCCEEDED(hr) && fUsedDefaultChar)
            {
                // Invalid conversion - fail here rather than passing an invalid path:
                delete[] *ppszOut;
                *ppszOut = NULL;
                hr = E_INVALIDARG;
            }
        }
    }
    return hr;
}

__inline HRESULT ConvertFileDescArrayFromUnicode(_In_reads_(ulFileDesc) const MapiFileDescW *pFileIn, ULONG ulFileDesc, _Outptr_result_buffer_(ulFileDesc) MapiFileDesc **ppFileOut)
{
    HRESULT hr = S_OK;
    *ppFileOut = NULL;
    if (pFileIn && ulFileDesc)
    {
        MapiFileDesc *pFileOut = NULL;
        hr = AllocFileDesc(ulFileDesc, &pFileOut);
        if (SUCCEEDED(hr))
        {
            for (ULONG ul = 0; SUCCEEDED(hr) && ul < ulFileDesc; ul++)
            {
                hr = ConvertFilePathFromUnicode(pFileIn[ul].lpszPathName, &pFileOut[ul].lpszPathName);
                if (SUCCEEDED(hr))
                {
                    hr = ConvertStringToMultiByte(pFileIn[ul].lpszFileName, &pFileOut[ul].lpszFileName);
                }
                if (SUCCEEDED(hr))
                {
                    pFileOut[ul].ulReserved = pFileIn[ul].ulReserved;
                    pFileOut[ul].flFlags = pFileIn[ul].flFlags;
                    pFileOut[ul].nPosition = pFileIn[ul].nPosition;
                    pFileOut[ul].lpFileType = pFileIn[ul].lpFileType; // This entry has no parts which need converting, copy pointer directly
                }
            }
            if (SUCCEEDED(hr))
            {
                *ppFileOut = pFileOut;
            }
            else
            {
                FreeFileDesc(pFileOut, ulFileDesc); // Will delete partially constructed MapiFileDesc's
                delete[] pFileOut;
            }
        }
    }

    return hr;
}

__inline HRESULT ConvertRecipDescArrayFromUnicode(_In_reads_(ulRecipDesc) const MapiRecipDescW *pRecipIn, ULONG ulRecipDesc, _Outptr_result_buffer_(ulRecipDesc) MapiRecipDesc **ppRecipOut)
{
    HRESULT hr = S_OK;
    *ppRecipOut = NULL;
    if (pRecipIn && ulRecipDesc)
    {
        MapiRecipDesc *pRecipOut = NULL;
        hr = AllocRecipDesc(ulRecipDesc, &pRecipOut);
        if (SUCCEEDED(hr))
        {
            for (ULONG ul = 0; SUCCEEDED(hr) && ul < ulRecipDesc; ul++)
            {
                hr = ConvertStringToMultiByte(pRecipIn[ul].lpszName, &pRecipOut[ul].lpszName);
                if (SUCCEEDED(hr))
                {
                    hr = ConvertStringToMultiByte(pRecipIn[ul].lpszAddress, &pRecipOut[ul].lpszAddress);
                }
                if (SUCCEEDED(hr) && pRecipIn[ul].ulEIDSize && pRecipIn[ul].lpEntryID)
                {
                    pRecipOut[ul].lpEntryID = new (std::nothrow) BYTE[pRecipIn[ul].ulEIDSize];
                    if (pRecipOut[ul].lpEntryID)
                    {
                        memcpy(pRecipOut[ul].lpEntryID, pRecipIn[ul].lpEntryID, pRecipIn[ul].ulEIDSize);
                    }
                    else
                    {
                        hr = E_OUTOFMEMORY;
                    }
                }
                if (SUCCEEDED(hr))
                {
                    pRecipOut[ul].ulReserved = pRecipIn[ul].ulReserved;
                    pRecipOut[ul].ulRecipClass = pRecipIn[ul].ulRecipClass;
                    pRecipOut[ul].ulEIDSize = pRecipIn[ul].ulEIDSize;
                }
            }
            if (SUCCEEDED(hr))
            {
                *ppRecipOut = pRecipOut;
            }
            else
            {
                FreeRecipDesc(pRecipOut, ulRecipDesc); // Will delete partially constructed MapiRecipDesc's
                delete[] pRecipOut;
            }
        }
    }

    return hr;
}

// Convert a complete Unicode MapiMessageW to a MapiMessage.
// Take account of whether the mapi provider supports the message body being passed as UTF8 in this scenario.
__inline HRESULT ConvertMessageFromUnicode(_In_ const MapiMessageW *pMessageIn, _Outptr_ MapiMessage **ppMessageOut, bool fSupportUTF8)
{
    HRESULT hr = S_OK;
    *ppMessageOut = NULL;

    if (pMessageIn)
    {
        MapiMessage *pMessageOut = new (std::nothrow) MapiMessage;
        if (pMessageOut == NULL)
        {
           hr = E_OUTOFMEMORY;
        }
        else
        {
            ZeroMemory(pMessageOut, sizeof(MapiMessage));

            hr = ConvertStringToMultiByteEx(pMessageIn->lpszSubject, &pMessageOut->lpszSubject, fSupportUTF8, NULL); // Can be passed as UTF8 if provider supports it.
            if (SUCCEEDED(hr))
            {
                hr = ConvertStringToMultiByteEx(pMessageIn->lpszNoteText, &pMessageOut->lpszNoteText, fSupportUTF8, NULL); // Can be passed as UTF8 if provider supports it.
                if (SUCCEEDED(hr))
                {
                    hr = ConvertStringToMultiByte(pMessageIn->lpszMessageType, &pMessageOut->lpszMessageType);
                    if (SUCCEEDED(hr))
                    {
                        hr = ConvertStringToMultiByte(pMessageIn->lpszDateReceived, &pMessageOut->lpszDateReceived);
                        if (SUCCEEDED(hr))
                        {
                            hr = ConvertStringToMultiByte(pMessageIn->lpszConversationID, &pMessageOut->lpszConversationID);
                            if (SUCCEEDED(hr))
                            {
                                hr = ConvertRecipDescArrayFromUnicode(pMessageIn->lpOriginator, 1, &pMessageOut->lpOriginator);
                                if (SUCCEEDED(hr))
                                {
                                    hr = ConvertRecipDescArrayFromUnicode(pMessageIn->lpRecips, pMessageIn->nRecipCount, &pMessageOut->lpRecips);
                                    if (SUCCEEDED(hr))
                                    {
                                        hr = ConvertFileDescArrayFromUnicode(pMessageIn->lpFiles, pMessageIn->nFileCount, &pMessageOut->lpFiles);
                                        if (SUCCEEDED(hr))
                                        {
                                            pMessageOut->ulReserved = fSupportUTF8 ? CP_UTF8 : pMessageIn->ulReserved;
                                            pMessageOut->flFlags = pMessageIn->flFlags;
                                            pMessageOut->nRecipCount = pMessageIn->nRecipCount;
                                            pMessageOut->nFileCount = pMessageIn->nFileCount;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if (SUCCEEDED(hr))
            {
                *ppMessageOut = pMessageOut;
            }
            else
            {
                 FreeMessage(pMessageOut);
            }
        }
    }
    return hr;
}



// Methods used to obtain the default MAPI provider, including looking this up in Windows Installer:

// Function pointer typedefs for Windows Installer MsiProvideQualifiedComponentW function. 
typedef UINT (WINAPI MSIPROVIDEQUALIFIEDCOMPONENTW)(PCWSTR, PCWSTR, DWORD, PWSTR, DWORD*);
typedef MSIPROVIDEQUALIFIEDCOMPONENTW * PMSIPROVIDEQUALIFIEDCOMPONENTW;

static const wchar_t s_wszLcid[] = L"Software\\";
static const wchar_t s_wszPolicy[] = L"Software\\Policy\\";

__inline bool GetComponentPathOrInstall(HINSTANCE hinstMSI, PCWSTR szCategory, DWORD dwLcid, _Out_writes_(*pcchPath) PWSTR szPath, _Inout_ DWORD *pcchPath, bool fInstall)
{
    UINT uiRet = ERROR_INDEX_ABSENT; // Default value, means did not find component
    wchar_t szQualifier[16];  // "{lcid}\NT"
    PMSIPROVIDEQUALIFIEDCOMPONENTW pfnMsiProvideQualifiedComponentW;
    const DWORD cchPathOrig = *pcchPath;

    // Get MsiProvideQualifiedComponentW()
    pfnMsiProvideQualifiedComponentW = (PMSIPROVIDEQUALIFIEDCOMPONENTW)GetProcAddress(hinstMSI, "MsiProvideQualifiedComponentW");
    if (pfnMsiProvideQualifiedComponentW)
    {
        szPath[0] = L'\0';
        szQualifier[0] = L'\0';

        // 1. Try "dddd\NT" qualifier
        swprintf_s(szQualifier, ARRAYSIZE(szQualifier), L"%lu\\%s", dwLcid, L"NT");

        uiRet = pfnMsiProvideQualifiedComponentW(szCategory, szQualifier, static_cast<DWORD>(-1), szPath, pcchPath);
        if (uiRet == ERROR_FILE_NOT_FOUND || uiRet == ERROR_INDEX_ABSENT)
        {
            *pcchPath = cchPathOrig;
            if (fInstall && uiRet == ERROR_FILE_NOT_FOUND)
            {
                // If component not found, try installing it:
                uiRet = pfnMsiProvideQualifiedComponentW(szCategory, szQualifier, 0, szPath, pcchPath);
            }
            else
            {
                // 2. Try "dddd" qualifier
                swprintf_s(szQualifier, ARRAYSIZE(szQualifier), L"%lu", dwLcid);
                uiRet = pfnMsiProvideQualifiedComponentW(szCategory, szQualifier, static_cast<DWORD>(-1), szPath, pcchPath);
                if (fInstall && uiRet == ERROR_FILE_NOT_FOUND)
                {
                    *pcchPath = cchPathOrig;
                    uiRet = pfnMsiProvideQualifiedComponentW(szCategory, szQualifier, 0, szPath, pcchPath);
                }
            }
        }
    }

    return (uiRet != ERROR_INDEX_ABSENT); // Continue searching if we didn't find the component
}

__inline bool GetComponentPath(_In_opt_ PWSTR szComponent, _In_opt_ PZZWSTR szzQualifier, _Out_writes_(cchDllPath) PWSTR szDllPath, DWORD cchDllPath, bool fInstall)
{
    szDllPath[0] = L'\0';
    HINSTANCE hinstMSI = NULL;
    size_t cb;
    PWSTR szLcid = NULL;
    HKEY hkeyLcid = NULL;
    PWSTR szPolicy = NULL;
    HKEY hkeyPolicy = NULL;
    bool fDone = false;
    PWSTR szName;
    DWORD dwLcid;
    const DWORD cchDllPathOrig = cchDllPath;

    hinstMSI = ::LoadLibraryW(L"MSI.DLL");
    if (hinstMSI)
    {
        // Use defaults if no szzQualifier
        if (szzQualifier == NULL || szzQualifier[0] == L'\0')
        {
            fDone = true;
            if (!GetComponentPathOrInstall(hinstMSI, szComponent, GetUserDefaultLCID(), szDllPath, &cchDllPath, fInstall))
            {
                cchDllPath = cchDllPathOrig;
                if (!GetComponentPathOrInstall(hinstMSI, szComponent, GetSystemDefaultLCID(), szDllPath, &cchDllPath, fInstall))
                {
                    cchDllPath = cchDllPathOrig;
                    if (!GetComponentPathOrInstall(hinstMSI, szComponent, 1033, szDllPath, &cchDllPath, fInstall))
                    {
                        fDone = false;
                    }
                }
            }
        }
        else
        {
            // Open the Policy key
            cb = (wcslen(s_wszPolicy) + wcslen(szzQualifier) + 1) * sizeof(wchar_t);
            szPolicy = (PWSTR) HeapAlloc(GetProcessHeap(), 0, cb);
            if (szPolicy)
            {
                wcscpy_s(szPolicy, cb / sizeof(wchar_t), s_wszPolicy);
                wcscat_s(szPolicy, cb / sizeof(wchar_t), szzQualifier);
                RegOpenKeyExW(HKEY_CURRENT_USER, szPolicy, 0, KEY_READ, &hkeyPolicy);
            }

            // Open the Lcid key
            cb = (wcslen(s_wszLcid) + wcslen(szzQualifier) + 1) * sizeof(wchar_t);
            szLcid = (LPWSTR) HeapAlloc(GetProcessHeap(), 0, cb);
            if (szLcid)
            {
                wcscpy_s(szLcid, cb / sizeof(wchar_t), s_wszLcid);
                wcscat_s(szLcid, cb / sizeof(wchar_t), szzQualifier);
                RegOpenKeyExW(HKEY_CURRENT_USER, szLcid, 0, KEY_READ, &hkeyLcid);
            }

            // Get first registry value name
            szName = &szzQualifier[wcslen(szzQualifier) + 1];

            // Loop till component found or we're out of registry value names
            while (szName[0] != L'\0' && !fDone)
            {
                DWORD dwType;
                DWORD dwSize = sizeof(dwLcid);
                DWORD dwSize1 = dwSize;

                if ((hkeyPolicy && RegQueryValueExW(hkeyPolicy, szName, 0, &dwType, (LPBYTE) &dwLcid, &dwSize) == ERROR_SUCCESS && dwType == REG_DWORD) ||
                    (hkeyLcid && RegQueryValueExW(hkeyLcid, szName, 0, &dwType, (LPBYTE) &dwLcid, &dwSize1) == ERROR_SUCCESS && dwType == REG_DWORD))
                {
                    cchDllPath = cchDllPathOrig;
                    fDone = GetComponentPathOrInstall(hinstMSI, szComponent, dwLcid, szDllPath, &cchDllPath, fInstall);
                }

                szName = &szName[wcslen(szName) + 1];  // Next registry value name
            }
        }
    }

    if (hkeyPolicy) { RegCloseKey(hkeyPolicy); }
    if (szPolicy) { HeapFree(GetProcessHeap(), 0, (LPVOID) szPolicy); }
    if (hkeyLcid) { RegCloseKey(hkeyLcid); }
    if (szLcid) { HeapFree(GetProcessHeap(), 0, (LPVOID) szLcid); }
    if (hinstMSI) { ::FreeLibrary(hinstMSI); }

    return fDone;
}

// Applications or DLLs can specify they must always use a certain mail provider in the registry.
// Check this for the current process and see if a certain provider is required:
__inline bool AlwaysNeedsMSMAPI(_Out_writes_opt_(cchDllPath) PWSTR szDllPath, DWORD cchDllPath)
{
    bool fNeedsMSMAPI = false;
    HKEY hkeyRoot = NULL;

    long lResult = ::RegOpenKeyExW(HKEY_LOCAL_MACHINE, L"SOFTWARE\\Microsoft\\Windows Messaging Subsystem\\MSMapiApps", 0, KEY_READ, &hkeyRoot);
    if (ERROR_SUCCESS == lResult)
    {
        wchar_t szValueName[256];

        DWORD dwSize;
        DWORD dwIndex = 0;
        DWORD dwType;
        DWORD cbBuffer;

        // Loop through the list of entries specified in the registry:
        do
        {
            dwSize = ARRAYSIZE(szValueName);

            // Either check for the mail provider specified for each entry, or just check for the existence of the entry:
            if (NULL == szDllPath || 0 == cchDllPath)
            {
                lResult = ::RegEnumValueW(hkeyRoot, dwIndex++, szValueName, &dwSize, NULL, &dwType, NULL, NULL);
            }
            else
            {
                szDllPath[cchDllPath - 1] = L'\0';
                cbBuffer = (cchDllPath - 1) * sizeof(wchar_t);
                lResult = ::RegEnumValueW(hkeyRoot, dwIndex++, szValueName, &dwSize, NULL, &dwType, (LPBYTE) szDllPath, &cbBuffer);
            }

            if (ERROR_SUCCESS == lResult)
            {
                // Check if the registry entry is the current process, or if it is a dll loaded in the current process:
                if (NULL != GetModuleHandleW(szValueName))
                {
                    fNeedsMSMAPI = true;
                    break;
                }
            }
        } while (ERROR_SUCCESS == lResult);

        ::RegCloseKey(hkeyRoot);
    }

    return fNeedsMSMAPI;
}

__inline bool IsTerminalServer(void)
{    
    OSVERSIONINFOEXW osVersionInfo = {0};
    DWORDLONG dwlConditionMask = 0;
    osVersionInfo.dwOSVersionInfoSize = sizeof(OSVERSIONINFOEXW);
    osVersionInfo.wSuiteMask = VER_SUITE_TERMINAL;
    VER_SET_CONDITION(dwlConditionMask, VER_SUITENAME, VER_AND);

    return (VerifyVersionInfoW(&osVersionInfo, VER_SUITENAME, dwlConditionMask) != 0);
}

// Find the registry key to the default Simple MAPI provider, from either HKEY_CURRENT_USER and HKEY_LOCAL_MACHINE.
__inline LRESULT GetMailClientRegKey(HKEY hkeyRoot, _Out_ HKEY* hkeyDefaultMail)
{
    HKEY hkey = nullptr;

    // Open the key to find out what the default mail program is:
    LRESULT lResult = ::RegOpenKeyExW(hkeyRoot, L"Software\\Clients\\Mail", 0, KEY_READ, &hkey);

    if (ERROR_SUCCESS == lResult)
    {
        wchar_t mailAppName[MAX_PATH] = L"";
        wchar_t defaultMailAppName[MAX_PATH];

        if (AlwaysNeedsMSMAPI(mailAppName, ARRAYSIZE(mailAppName)))
        {
            if (mailAppName[0] != L'\0')
            {
                // Mail client supplied
                wcscpy_s(defaultMailAppName, ARRAYSIZE(defaultMailAppName), mailAppName);
                lResult = ERROR_SUCCESS;
            }
            else
            {
                // No mail client, use mapi32x.dll
                lResult = ERROR_PATH_NOT_FOUND;
            }
        }
        else
        {
            // Find out what the default mail program is.
            DWORD sizeInBytes = (ARRAYSIZE(defaultMailAppName) - 1) * sizeof(wchar_t);
            defaultMailAppName[ARRAYSIZE(defaultMailAppName) - 1] = L'\0';
            lResult = ::RegQueryValueExW(hkey, nullptr, nullptr, nullptr, (LPBYTE)defaultMailAppName, &sizeInBytes);
        }

        if (ERROR_SUCCESS == lResult)
        {
            // Open the key for the default mail program to see where the dll is:
            lResult = ::RegOpenKeyExW(hkey, defaultMailAppName, 0, KEY_READ, hkeyDefaultMail);

            //In case of getting default mail client value from HKCU, look at both HKCU/HKLM
            //for finding the dllPath for the mail client
            if (ERROR_SUCCESS != lResult && hkeyRoot == HKEY_CURRENT_USER)
            {
                wchar_t DefaultMailAppKeyPath[MAX_PATH];
                wcscpy_s(DefaultMailAppKeyPath, ARRAYSIZE(DefaultMailAppKeyPath), L"Software\\Clients\\Mail\\");
                wcscat_s(DefaultMailAppKeyPath, ARRAYSIZE(DefaultMailAppKeyPath), defaultMailAppName);
                lResult = ::RegOpenKeyExW(HKEY_LOCAL_MACHINE, DefaultMailAppKeyPath, 0, KEY_READ, hkeyDefaultMail);
            }
        }
        ::RegCloseKey(hkey);
    }
    return lResult;
}

// Find the path to the default Simple MAPI provider, from either HKEY_CURRENT_USER and HKEY_LOCAL_MACHINE.
// If the provider was installed by Windows Installer then the MSI itself can be checked for the location,
// and if neccessary install on demand:
__inline bool GetMapiDllPath(HKEY hkeyRoot, _Out_writes_(cchDllPath) PWSTR szDllPath, DWORD cchDllPath, _Out_ bool *pfSupportUTF8)
{
    szDllPath[0] = L'\0';
    *pfSupportUTF8 = false;

    DWORD cbBufferSizeT;
    DWORD dwType;

    HKEY hkeyDefaultMail = NULL;

    // Open the key for the default mail program to see where the dll is:
    LRESULT lResult = GetMailClientRegKey(hkeyRoot, &hkeyDefaultMail);

    if (ERROR_SUCCESS == lResult)
    {
        // See if provider supports messages as UTF8:
        DWORD dwSupportUTF8 = 0;
        cbBufferSizeT = sizeof(dwSupportUTF8);
        if (ERROR_SUCCESS == ::RegQueryValueExW(hkeyDefaultMail, L"SupportUTF8", NULL, &dwType, (LPBYTE)&dwSupportUTF8, &cbBufferSizeT))
        {
            if (REG_DWORD == dwType && dwSupportUTF8 != 0)
            {
                *pfSupportUTF8 = true;
            }
        } // If reg value not present *pfSupportUTF8 remains as false, not an error

        wchar_t szComponent[39 + 1] = {0};    // wcslen(GUID)

        DWORD dwMSIInstallOnWTS;
        PWSTR szMSIOfficeLCID = NULL;
        PWSTR szMSIApplicationLCID = NULL;

        // Get MSIInstallOnWTS, 0 means don't demand-install on terminal server machines:
        cbBufferSizeT = sizeof(dwMSIInstallOnWTS);
        lResult = ::RegQueryValueExW(hkeyDefaultMail, L"MSIInstallOnWTS", NULL, &dwType, (LPBYTE) &dwMSIInstallOnWTS, &cbBufferSizeT);
                
        if (ERROR_SUCCESS == lResult && REG_DWORD == dwType)
        {
            // Use what is returned
        }
        else
        {
            dwMSIInstallOnWTS = 1;  // Default is true
        }

        // Get MSIApplicationLCID
        lResult = ::RegQueryValueExW(hkeyDefaultMail, L"MSIApplicationLCID", NULL, &dwType, NULL, &cbBufferSizeT);

        if (ERROR_SUCCESS == lResult && REG_MULTI_SZ == dwType)
        {
            szMSIApplicationLCID = (PWSTR)HeapAlloc(GetProcessHeap(), 0, cbBufferSizeT + sizeof(wchar_t));

            if (szMSIApplicationLCID)
            {
                szMSIApplicationLCID[cbBufferSizeT / sizeof(wchar_t)] = L'\0';
                // Note: Could return ERROR_MORE_DATA if the size of the value changes between here 
                //       and the call above to get the size.
                lResult = ::RegQueryValueExW(hkeyDefaultMail, L"MSIApplicationLCID", NULL, &dwType, (LPBYTE) szMSIApplicationLCID, &cbBufferSizeT);
            }
        }

        // Get MSIOfficeLCID
        if (ERROR_SUCCESS == lResult)
        {
            lResult = ::RegQueryValueExW(hkeyDefaultMail, L"MSIOfficeLCID", NULL, &dwType, NULL, &cbBufferSizeT);
        }

        if (ERROR_SUCCESS == lResult && REG_MULTI_SZ == dwType)
        {
            szMSIOfficeLCID = (PWSTR)HeapAlloc(GetProcessHeap(), 0, cbBufferSizeT + sizeof(wchar_t));

            if (szMSIOfficeLCID)
            {
                szMSIOfficeLCID[cbBufferSizeT / sizeof(wchar_t)] = L'\0';
                // Note: Could return ERROR_MORE_DATA if the size of the value changes between here 
                //       and the call above to get the size.
                lResult = ::RegQueryValueExW(hkeyDefaultMail, L"MSIOfficeLCID", NULL, &dwType, (LPBYTE) szMSIOfficeLCID, &cbBufferSizeT);
            }
        }

        // Find out what the component is.
        bool fDone = false;
        cbBufferSizeT = (ARRAYSIZE(szComponent) - 1) * sizeof(wchar_t);
        szComponent[ARRAYSIZE(szComponent) - 1] = L'\0';
        if (ERROR_SUCCESS == lResult)
        {
            lResult = ::RegQueryValueExW(hkeyDefaultMail, L"MSIComponentID", NULL, &dwType, (LPBYTE) szComponent, &cbBufferSizeT);
        }

        if (ERROR_SUCCESS == lResult && REG_SZ == dwType)
        {
            bool fInstall = dwMSIInstallOnWTS || !IsTerminalServer();

            // First try Application's LCID(s)
            if (szMSIApplicationLCID && GetComponentPath(szComponent, szMSIApplicationLCID, szDllPath, cchDllPath, fInstall))
            {
                fDone = true;
            }
            else if (szMSIOfficeLCID && GetComponentPath(szComponent, szMSIOfficeLCID, szDllPath, cchDllPath, fInstall))
            {
                // Then try Office's LCID(s)
                fDone = true;
            }
            else if (GetComponentPath(szComponent, NULL, szDllPath, cchDllPath, fInstall))
            {
                // Finally try the defaults
                fDone = true;
            }
        }

        if (!fDone)
        {
            // Find out what the dll is from direct registry lookup:
            cbBufferSizeT = (cchDllPath - 1) * sizeof(wchar_t);
            szDllPath[cchDllPath - 1] = L'\0';
            lResult = ::RegQueryValueExW(hkeyDefaultMail, L"DLLPath", NULL, &dwType, (LPBYTE) szDllPath, &cbBufferSizeT);
            if (ERROR_SUCCESS != lResult)
            {
                szDllPath[0] = L'\0';
            }
            else if (REG_EXPAND_SZ == dwType)
            {
                wchar_t szExpandedPath[MAX_PATH];
                lResult = ExpandEnvironmentStringsW(szDllPath, szExpandedPath, MAX_PATH);
                if (lResult > 0 && lResult < MAX_PATH)
                {
                    wcscpy_s(szDllPath, cchDllPath, szExpandedPath);
                }
                else
                {
                    szDllPath[0] = L'\0';
                }
            }
        }

        if (szMSIApplicationLCID)
        {
            HeapFree(GetProcessHeap(), 0, (LPVOID) szMSIApplicationLCID);
        }

        if (szMSIOfficeLCID)
        {
            HeapFree(GetProcessHeap(), 0, (LPVOID) szMSIOfficeLCID);
        }

        ::RegCloseKey(hkeyDefaultMail);
    }

    return (L'\0' != szDllPath[0]); // return true if we found a dll path
}


// Find the path of the executable for the Default Mail client. Note it's the responsibility of the caller 
// to free exePath using CoTaskMemFree when it's no longer needed.
__inline bool GetDefaultMailClientExePath(HKEY hkeyRoot, _Out_ PWSTR* exePath)
{
    *exePath = nullptr;
    HKEY hkeyDefaultMail = nullptr;

    // Open the key for the default mail program to see where the EXE is:
    LRESULT lResult = GetMailClientRegKey(hkeyRoot, &hkeyDefaultMail);

    if (ERROR_SUCCESS == lResult)
    {
        // Find out what the exe is from direct registry lookup:
        HKEY hkeyShellCommand = nullptr;
        lResult = ::RegOpenKeyExW(hkeyDefaultMail, L"shell\\open\\command", 0, KEY_READ, &hkeyShellCommand);
        if (ERROR_SUCCESS == lResult)
        {
            wchar_t shellCommand[MAX_PATH];
            wchar_t expandedPath[MAX_PATH];

            DWORD bufferSizeInBytes = ARRAYSIZE(shellCommand) * sizeof(wchar_t);
            DWORD valueType;

            lResult = ::RegQueryValueExW(hkeyShellCommand, L"", NULL, &valueType, (LPBYTE)shellCommand, &bufferSizeInBytes);
            
            if (ERROR_SUCCESS == lResult)
            {
                if (REG_EXPAND_SZ == valueType)
                {
                    lResult = ExpandEnvironmentStringsW(shellCommand, expandedPath, ARRAYSIZE(expandedPath));
                    if (lResult > 0 && lResult <= static_cast<LRESULT>(ARRAYSIZE(shellCommand)))
                    {
                        wcscpy_s(shellCommand, ARRAYSIZE(shellCommand), expandedPath);
                    }
                }

                if (FAILED(SHEvaluateSystemCommandTemplate(shellCommand, exePath, nullptr, nullptr)))
                {
                    *exePath = nullptr;
                    lResult = ERROR_PATH_NOT_FOUND;
                }
            }
            ::RegCloseKey(hkeyShellCommand);
        }

        ::RegCloseKey(hkeyDefaultMail);
    }
    return (lResult == ERROR_SUCCESS); // return true if we found an exe path
}

__inline HMODULE LoadDefaultMailProvider(_Out_ bool *pfSupportUTF8)
{
    HMODULE hMod = NULL;
    wchar_t szDllPath[MAX_PATH] = L"";
    *pfSupportUTF8 = false;

    // Get path to Simple MAPI provider implementation and whether it supports UTF8 messages.
    // Try both HKEY_CURRENT_USER and HKEY_LOCAL_MACHINE:
    if (!GetMapiDllPath(HKEY_CURRENT_USER, szDllPath, ARRAYSIZE(szDllPath), pfSupportUTF8) &&
        !GetMapiDllPath(HKEY_LOCAL_MACHINE, szDllPath, ARRAYSIZE(szDllPath), pfSupportUTF8))
    {
        // Otherwise default to trying to load mapi32x.dll:
        wcscpy_s(szDllPath, ARRAYSIZE(szDllPath), L"mapi32x.dll");
    }

    OFSTRUCT ofs;
    // If the proposed dll is mapi32.dll, then don't load as this would lead to an infinite loop
    if (_wcsicmp(szDllPath, L"mapi32.dll") != 0)
    {
        if (_wcsicmp(szDllPath, L"mapi32x.dll") != 0)
        {
            // Specific mapi client dll has been specified - load it
            hMod = LoadLibraryW(szDllPath);
        }
        else if ((OpenFile("mapisvc.inf", &ofs, OF_EXIST) != HFILE_ERROR) || AlwaysNeedsMSMAPI(NULL, 0))
        {
            // Fallback to mapi32x.dll if no mapi client has been specified - ensure always loading from system directory
            UINT uiRes = GetSystemDirectoryW(szDllPath, ARRAYSIZE(szDllPath));
            if ((uiRes > 0) && (uiRes < ARRAYSIZE(szDllPath)))
            {
                if (wcscat_s(szDllPath, ARRAYSIZE(szDllPath), L"\\mapi32x.dll") == 0)
                {
                    hMod = LoadLibraryW(szDllPath);
                }
            }
        }
    }

    return hMod;
}



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

