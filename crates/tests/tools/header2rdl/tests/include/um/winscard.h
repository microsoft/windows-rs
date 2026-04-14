/*++

Copyright (c) 1996  Microsoft Corporation

Module Name:

    WinSCard

Abstract:

    This header file provides the definitions and symbols necessary for an
    Application or Smart Card Service Provider to access the Smartcard
    Subsystem.

Environment:

    Win32

Notes:

--*/

#ifndef _WINSCARD_H_
#define _WINSCARD_H_

#if defined (_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#include <wtypes.h>
#include <winioctl.h>
#include "winsmcrd.h"
#ifndef SCARD_S_SUCCESS
#include "SCardErr.h"
#endif
#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef _LPCBYTE_DEFINED
#define _LPCBYTE_DEFINED
typedef const BYTE *LPCBYTE;
#endif
#ifndef _LPCVOID_DEFINED
#define _LPCVOID_DEFINED
typedef const VOID *LPCVOID;
#endif

#ifndef WINSCARDAPI
#define WINSCARDAPI
#endif
#ifndef WINSCARDDATA
#define WINSCARDDATA __declspec(dllimport)
#endif

/* In clr:pure we cannot mark data export with dllimport.
 * We should add small functions which returns the value of
 * the global.
 */
#if !defined(_M_CEE_PURE)
WINSCARDDATA extern const SCARD_IO_REQUEST
    g_rgSCardT0Pci,
    g_rgSCardT1Pci,
    g_rgSCardRawPci;
#define SCARD_PCI_T0  (&g_rgSCardT0Pci)
#define SCARD_PCI_T1  (&g_rgSCardT1Pci)
#define SCARD_PCI_RAW (&g_rgSCardRawPci)
#endif

//
////////////////////////////////////////////////////////////////////////////////
//
//  Service Manager Access Services
//
//      The following services are used to manage user and terminal contexts for
//      Smart Cards.
//

typedef ULONG_PTR SCARDCONTEXT;
typedef SCARDCONTEXT *PSCARDCONTEXT, *LPSCARDCONTEXT;

typedef ULONG_PTR SCARDHANDLE;
typedef SCARDHANDLE *PSCARDHANDLE, *LPSCARDHANDLE;

#define SCARD_AUTOALLOCATE (DWORD)(-1)

#define SCARD_SCOPE_USER     0  // The context is a user context, and any
                                // database operations are performed within the
                                // domain of the user.
#define SCARD_SCOPE_TERMINAL 1  // The context is that of the current terminal,
                                // and any database operations are performed
                                // within the domain of that terminal.  (The
                                // calling application must have appropriate
                                // access permissions for any database actions.)
#define SCARD_SCOPE_SYSTEM    2 // The context is the system context, and any
                                // database operations are performed within the
                                // domain of the system.  (The calling
                                // application must have appropriate access
                                // permissions for any database actions.)

extern WINSCARDAPI LONG WINAPI
SCardEstablishContext(
    _In_  DWORD dwScope,
    _Reserved_  LPCVOID pvReserved1,
    _Reserved_  LPCVOID pvReserved2,
    _Out_ LPSCARDCONTEXT phContext);

extern WINSCARDAPI LONG WINAPI
SCardReleaseContext(
    _In_      SCARDCONTEXT hContext);

extern WINSCARDAPI LONG WINAPI
SCardIsValidContext(
    _In_      SCARDCONTEXT hContext);


//
////////////////////////////////////////////////////////////////////////////////
//
//  Smart Card Database Management Services
//
//      The following services provide for managing the Smart Card Database.
//

#define SCARD_ALL_READERS       TEXT("SCard$AllReaders\000")
#define SCARD_DEFAULT_READERS   TEXT("SCard$DefaultReaders\000")
#define SCARD_LOCAL_READERS     TEXT("SCard$LocalReaders\000")
#define SCARD_SYSTEM_READERS    TEXT("SCard$SystemReaders\000")

#define SCARD_PROVIDER_PRIMARY  1   // Primary Provider Id
#define SCARD_PROVIDER_CSP      2   // Crypto Service Provider Id
#define SCARD_PROVIDER_KSP      3   // Key Storage Provider Id


//
// Database Reader routines
//

extern WINSCARDAPI LONG WINAPI
SCardListReaderGroupsA(
    _In_    SCARDCONTEXT hContext,
    _Out_writes_opt_(*pcchGroups) _Post_ _NullNull_terminated_  LPSTR mszGroups,
    _Inout_ LPDWORD pcchGroups);
extern WINSCARDAPI LONG WINAPI
SCardListReaderGroupsW(
    _In_    SCARDCONTEXT hContext,
    _Out_writes_opt_(*pcchGroups) _Post_ _NullNull_terminated_  LPWSTR mszGroups,
    _Inout_ LPDWORD pcchGroups);
#ifdef UNICODE
#define SCardListReaderGroups  SCardListReaderGroupsW
#else
#define SCardListReaderGroups  SCardListReaderGroupsA
#endif // !UNICODE

_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardListReadersA(
    _In_     SCARDCONTEXT hContext,
    _In_opt_ LPCSTR mszGroups,
    _When_(_Old_(*pcchReaders) == SCARD_AUTOALLOCATE, _At_((LPSTR *)mszReaders, _Outptr_result_buffer_maybenull_(*pcchReaders) _At_(*_Curr_, _Post_z_ _Post_ _NullNull_terminated_)))
    _When_(_Old_(*pcchReaders) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcchReaders) _Post_ _NullNull_terminated_)
             LPSTR mszReaders,
    _Inout_  LPDWORD pcchReaders);
_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardListReadersW(
    _In_     SCARDCONTEXT hContext,
    _In_opt_ LPCWSTR mszGroups,
    _When_(_Old_(*pcchReaders) == SCARD_AUTOALLOCATE, _At_((LPWSTR *)mszReaders, _Outptr_result_buffer_maybenull_(*pcchReaders) _At_(*_Curr_, _Post_z_ _Post_ _NullNull_terminated_)))
    _When_(_Old_(*pcchReaders) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcchReaders) _Post_ _NullNull_terminated_)
             LPWSTR mszReaders,
    _Inout_  LPDWORD pcchReaders);
#ifdef UNICODE
#define SCardListReaders  SCardListReadersW
#else
#define SCardListReaders  SCardListReadersA
#endif // !UNICODE

_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardListCardsA(
    _In_      SCARDCONTEXT hContext,
    _In_opt_  LPCBYTE pbAtr,
    _In_reads_opt_(cguidInterfaceCount)  LPCGUID rgquidInterfaces,
    _In_      DWORD cguidInterfaceCount,
    _When_(_Old_(*pcchCards) == SCARD_AUTOALLOCATE, _At_((LPSTR *)mszCards, _Outptr_result_buffer_maybenull_(*pcchCards) _At_(*_Curr_, _Post_ _NullNull_terminated_)))
    _When_(_Old_(*pcchCards) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcchCards) _Post_ _NullNull_terminated_)
              CHAR *mszCards,
    _Inout_   LPDWORD pcchCards);
_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardListCardsW(
    _In_      SCARDCONTEXT hContext,
    _In_opt_  LPCBYTE pbAtr,
    _In_reads_opt_(cguidInterfaceCount)  LPCGUID rgquidInterfaces,
    _In_      DWORD cguidInterfaceCount,
    _When_(_Old_(*pcchCards) == SCARD_AUTOALLOCATE, _At_((LPWSTR *)mszCards, _Outptr_result_buffer_maybenull_(*pcchCards) _At_(*_Curr_, _Post_ _NullNull_terminated_)))
    _When_(_Old_(*pcchCards) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcchCards) _Post_ _NullNull_terminated_)
              WCHAR *mszCards,
    _Inout_   LPDWORD pcchCards);
#ifdef UNICODE
#define SCardListCards  SCardListCardsW
#else
#define SCardListCards  SCardListCardsA
#endif // !UNICODE
//
// NOTE:    The routine SCardListCards name differs from the PC/SC definition.
//          It should be:
//
//              extern WINSCARDAPI LONG WINAPI
//              SCardListCardTypes(
//                  _In_      SCARDCONTEXT hContext,
//                  _In_opt_  LPCBYTE pbAtr,
//                  _In_opt_  LPCGUID rgquidInterfaces,
//                  _In_      DWORD cguidInterfaceCount,
//                  _Out_opt_ LPTSTR mszCards,
//                  _Inout_   LPDWORD pcchCards);
//
//          Here's a work-around MACRO:
#define SCardListCardTypes SCardListCards

extern WINSCARDAPI LONG WINAPI
SCardListInterfacesA(
    _In_     SCARDCONTEXT hContext,
    _In_     LPCSTR szCard,
    _Out_    LPGUID pguidInterfaces,
    _Inout_  LPDWORD pcguidInterfaces);
extern WINSCARDAPI LONG WINAPI
SCardListInterfacesW(
    _In_     SCARDCONTEXT hContext,
    _In_     LPCWSTR szCard,
    _Out_    LPGUID pguidInterfaces,
    _Inout_  LPDWORD pcguidInterfaces);
#ifdef UNICODE
#define SCardListInterfaces  SCardListInterfacesW
#else
#define SCardListInterfaces  SCardListInterfacesA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardGetProviderIdA(
    _In_     SCARDCONTEXT hContext,
    _In_     LPCSTR szCard,
    _Out_    LPGUID pguidProviderId);
extern WINSCARDAPI LONG WINAPI
SCardGetProviderIdW(
    _In_     SCARDCONTEXT hContext,
    _In_     LPCWSTR szCard,
    _Out_    LPGUID pguidProviderId);
#ifdef UNICODE
#define SCardGetProviderId  SCardGetProviderIdW
#else
#define SCardGetProviderId  SCardGetProviderIdA
#endif // !UNICODE
//
// NOTE:    The routine SCardGetProviderId in this implementation uses GUIDs.
//          The PC/SC definition uses BYTEs.
//

_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardGetCardTypeProviderNameA(
    _In_      SCARDCONTEXT hContext,
    _In_      LPCSTR szCardName,
    _In_      DWORD dwProviderId,
    _When_(_Old_(*pcchProvider) == SCARD_AUTOALLOCATE, _At_((LPSTR *)szProvider, _Outptr_result_buffer_all_(*pcchProvider)))
    _When_(_Old_(*pcchProvider) != SCARD_AUTOALLOCATE, _Out_writes_to_(*pcchProvider, *pcchProvider) _Post_z_)
              CHAR *szProvider,
    _Inout_   LPDWORD pcchProvider);
_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardGetCardTypeProviderNameW(
    _In_      SCARDCONTEXT hContext,
    _In_      LPCWSTR szCardName,
    _In_      DWORD dwProviderId,
    _When_(_Old_(*pcchProvider) == SCARD_AUTOALLOCATE, _At_((LPWSTR *)szProvider, _Outptr_result_buffer_all_(*pcchProvider)))
    _When_(_Old_(*pcchProvider) != SCARD_AUTOALLOCATE, _Out_writes_to_(*pcchProvider, *pcchProvider) _Post_z_)
              WCHAR *szProvider,
    _Inout_   LPDWORD pcchProvider);
#ifdef UNICODE
#define SCardGetCardTypeProviderName  SCardGetCardTypeProviderNameW
#else
#define SCardGetCardTypeProviderName  SCardGetCardTypeProviderNameA
#endif // !UNICODE
//
// NOTE:    This routine is an extension to the PC/SC definitions.
//


//
// Database Writer routines
//

extern WINSCARDAPI LONG WINAPI
SCardIntroduceReaderGroupA(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCSTR szGroupName);
extern WINSCARDAPI LONG WINAPI
SCardIntroduceReaderGroupW(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCWSTR szGroupName);
#ifdef UNICODE
#define SCardIntroduceReaderGroup  SCardIntroduceReaderGroupW
#else
#define SCardIntroduceReaderGroup  SCardIntroduceReaderGroupA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardForgetReaderGroupA(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCSTR szGroupName);
extern WINSCARDAPI LONG WINAPI
SCardForgetReaderGroupW(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCWSTR szGroupName);
#ifdef UNICODE
#define SCardForgetReaderGroup  SCardForgetReaderGroupW
#else
#define SCardForgetReaderGroup  SCardForgetReaderGroupA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardIntroduceReaderA(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCSTR szReaderName,
    _In_ LPCSTR szDeviceName);
extern WINSCARDAPI LONG WINAPI
SCardIntroduceReaderW(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCWSTR szReaderName,
    _In_ LPCWSTR szDeviceName);
#ifdef UNICODE
#define SCardIntroduceReader  SCardIntroduceReaderW
#else
#define SCardIntroduceReader  SCardIntroduceReaderA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardForgetReaderA(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCSTR szReaderName);
extern WINSCARDAPI LONG WINAPI
SCardForgetReaderW(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCWSTR szReaderName);
#ifdef UNICODE
#define SCardForgetReader  SCardForgetReaderW
#else
#define SCardForgetReader  SCardForgetReaderA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardAddReaderToGroupA(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCSTR szReaderName,
    _In_ LPCSTR szGroupName);
extern WINSCARDAPI LONG WINAPI
SCardAddReaderToGroupW(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCWSTR szReaderName,
    _In_ LPCWSTR szGroupName);
#ifdef UNICODE
#define SCardAddReaderToGroup  SCardAddReaderToGroupW
#else
#define SCardAddReaderToGroup  SCardAddReaderToGroupA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardRemoveReaderFromGroupA(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCSTR szReaderName,
    _In_ LPCSTR szGroupName);
extern WINSCARDAPI LONG WINAPI
SCardRemoveReaderFromGroupW(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCWSTR szReaderName,
    _In_ LPCWSTR szGroupName);
#ifdef UNICODE
#define SCardRemoveReaderFromGroup  SCardRemoveReaderFromGroupW
#else
#define SCardRemoveReaderFromGroup  SCardRemoveReaderFromGroupA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardIntroduceCardTypeA(
    _In_     SCARDCONTEXT hContext,
    _In_     LPCSTR szCardName,
    _In_opt_ LPCGUID pguidPrimaryProvider,
    _In_opt_ LPCGUID rgguidInterfaces,
    _In_     DWORD dwInterfaceCount,
    _In_     LPCBYTE pbAtr,
    _In_     LPCBYTE pbAtrMask,
    _In_     DWORD cbAtrLen);
extern WINSCARDAPI LONG WINAPI
SCardIntroduceCardTypeW(
    _In_     SCARDCONTEXT hContext,
    _In_     LPCWSTR szCardName,
    _In_opt_ LPCGUID pguidPrimaryProvider,
    _In_opt_ LPCGUID rgguidInterfaces,
    _In_     DWORD dwInterfaceCount,
    _In_     LPCBYTE pbAtr,
    _In_     LPCBYTE pbAtrMask,
    _In_     DWORD cbAtrLen);
#ifdef UNICODE
#define SCardIntroduceCardType  SCardIntroduceCardTypeW
#else
#define SCardIntroduceCardType  SCardIntroduceCardTypeA
#endif // !UNICODE
//
// NOTE:    The routine SCardIntroduceCardType's parameters' order differs from
//          the PC/SC definition.  It should be:
//
//              extern WINSCARDAPI LONG WINAPI
//              SCardIntroduceCardType(
//                  _In_     SCARDCONTEXT hContext,
//                  _In_     LPCTSTR szCardName,
//                  _In_     LPCBYTE pbAtr,
//                  _In_     LPCBYTE pbAtrMask,
//                  _In_     DWORD cbAtrLen,
//                  _In_opt_ LPCGUID pguidPrimaryProvider,
//                  _In_opt_ LPCGUID rgguidInterfaces,
//                  _In_     DWORD dwInterfaceCount);
//
//          Here's a work-around MACRO:
#define PCSCardIntroduceCardType(hContext, szCardName, pbAtr, pbAtrMask, cbAtrLen, pguidPrimaryProvider, rgguidInterfaces, dwInterfaceCount) \
          SCardIntroduceCardType(hContext, szCardName, pguidPrimaryProvider, rgguidInterfaces, dwInterfaceCount, pbAtr, pbAtrMask, cbAtrLen)

extern WINSCARDAPI LONG WINAPI
SCardSetCardTypeProviderNameA(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCSTR szCardName,
    _In_ DWORD dwProviderId,
    _In_ LPCSTR szProvider);
extern WINSCARDAPI LONG WINAPI
SCardSetCardTypeProviderNameW(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCWSTR szCardName,
    _In_ DWORD dwProviderId,
    _In_ LPCWSTR szProvider);
#ifdef UNICODE
#define SCardSetCardTypeProviderName  SCardSetCardTypeProviderNameW
#else
#define SCardSetCardTypeProviderName  SCardSetCardTypeProviderNameA
#endif // !UNICODE
//
// NOTE:    This routine is an extention to the PC/SC specifications.
//

extern WINSCARDAPI LONG WINAPI
SCardForgetCardTypeA(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCSTR szCardName);
extern WINSCARDAPI LONG WINAPI
SCardForgetCardTypeW(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCWSTR szCardName);
#ifdef UNICODE
#define SCardForgetCardType  SCardForgetCardTypeW
#else
#define SCardForgetCardType  SCardForgetCardTypeA
#endif // !UNICODE


//
////////////////////////////////////////////////////////////////////////////////
//
//  Service Manager Support Routines
//
//      The following services are supplied to simplify the use of the Service
//      Manager API.
//

extern WINSCARDAPI LONG WINAPI
SCardFreeMemory(
    _In_ SCARDCONTEXT hContext,
    _In_ LPCVOID pvMem);

#if (NTDDI_VERSION >= NTDDI_WINXP)
extern WINSCARDAPI HANDLE WINAPI
SCardAccessStartedEvent(void);

extern WINSCARDAPI void WINAPI
SCardReleaseStartedEvent(void);
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

//
////////////////////////////////////////////////////////////////////////////////
//
//  Reader Services
//
//      The following services supply means for tracking cards within readers.
//

typedef struct {
    LPCSTR      szReader;       // reader name
    LPVOID      pvUserData;     // user defined data
    DWORD       dwCurrentState; // current state of reader at time of call
    DWORD       dwEventState;   // state of reader after state change
    DWORD       cbAtr;          // Number of bytes in the returned ATR.
    BYTE        rgbAtr[36];     // Atr of inserted card, (extra alignment bytes)
} SCARD_READERSTATEA, *PSCARD_READERSTATEA, *LPSCARD_READERSTATEA;
typedef struct {
    LPCWSTR     szReader;       // reader name
    LPVOID      pvUserData;     // user defined data
    DWORD       dwCurrentState; // current state of reader at time of call
    DWORD       dwEventState;   // state of reader after state change
    DWORD       cbAtr;          // Number of bytes in the returned ATR.
    BYTE        rgbAtr[36];     // Atr of inserted card, (extra alignment bytes)
} SCARD_READERSTATEW, *PSCARD_READERSTATEW, *LPSCARD_READERSTATEW;
#ifdef UNICODE
typedef SCARD_READERSTATEW SCARD_READERSTATE;
typedef PSCARD_READERSTATEW PSCARD_READERSTATE;
typedef LPSCARD_READERSTATEW LPSCARD_READERSTATE;
#else
typedef SCARD_READERSTATEA SCARD_READERSTATE;
typedef PSCARD_READERSTATEA PSCARD_READERSTATE;
typedef LPSCARD_READERSTATEA LPSCARD_READERSTATE;
#endif // UNICODE

// Backwards compatibility macros
#define SCARD_READERSTATE_A SCARD_READERSTATEA
#define SCARD_READERSTATE_W SCARD_READERSTATEW
#define PSCARD_READERSTATE_A PSCARD_READERSTATEA
#define PSCARD_READERSTATE_W PSCARD_READERSTATEW
#define LPSCARD_READERSTATE_A LPSCARD_READERSTATEA
#define LPSCARD_READERSTATE_W LPSCARD_READERSTATEW

#define SCARD_STATE_UNAWARE     0x00000000  // The application is unaware of the
                                            // current state, and would like to
                                            // know.  The use of this value
                                            // results in an immediate return
                                            // from state transition monitoring
                                            // services.  This is represented by
                                            // all bits set to zero.
#define SCARD_STATE_IGNORE      0x00000001  // The application requested that
                                            // this reader be ignored.  No other
                                            // bits will be set.
#define SCARD_STATE_CHANGED     0x00000002  // This implies that there is a
                                            // difference between the state
                                            // believed by the application, and
                                            // the state known by the Service
                                            // Manager.  When this bit is set,
                                            // the application may assume a
                                            // significant state change has
                                            // occurred on this reader.
#define SCARD_STATE_UNKNOWN     0x00000004  // This implies that the given
                                            // reader name is not recognized by
                                            // the Service Manager.  If this bit
                                            // is set, then SCARD_STATE_CHANGED
                                            // and SCARD_STATE_IGNORE will also
                                            // be set.
#define SCARD_STATE_UNAVAILABLE 0x00000008  // This implies that the actual
                                            // state of this reader is not
                                            // available.  If this bit is set,
                                            // then all the following bits are
                                            // clear.
#define SCARD_STATE_EMPTY       0x00000010  // This implies that there is not
                                            // card in the reader.  If this bit
                                            // is set, all the following bits
                                            // will be clear.
#define SCARD_STATE_PRESENT     0x00000020  // This implies that there is a card
                                            // in the reader.
#define SCARD_STATE_ATRMATCH    0x00000040  // This implies that there is a card
                                            // in the reader with an ATR
                                            // matching one of the target cards.
                                            // If this bit is set,
                                            // SCARD_STATE_PRESENT will also be
                                            // set.  This bit is only returned
                                            // on the SCardLocateCard() service.
#define SCARD_STATE_EXCLUSIVE   0x00000080  // This implies that the card in the
                                            // reader is allocated for exclusive
                                            // use by another application.  If
                                            // this bit is set,
                                            // SCARD_STATE_PRESENT will also be
                                            // set.
#define SCARD_STATE_INUSE       0x00000100  // This implies that the card in the
                                            // reader is in use by one or more
                                            // other applications, but may be
                                            // connected to in shared mode.  If
                                            // this bit is set,
                                            // SCARD_STATE_PRESENT will also be
                                            // set.
#define SCARD_STATE_MUTE        0x00000200  // This implies that the card in the
                                            // reader is unresponsive or not
                                            // supported by the reader or
                                            // software.
#define SCARD_STATE_UNPOWERED   0x00000400  // This implies that the card in the
                                            // reader has not been powered up.

extern WINSCARDAPI LONG WINAPI
SCardLocateCardsA(
    _In_    SCARDCONTEXT hContext,
    _In_    LPCSTR mszCards,
    _Inout_ LPSCARD_READERSTATEA rgReaderStates,
    _In_    DWORD cReaders);
extern WINSCARDAPI LONG WINAPI
SCardLocateCardsW(
    _In_    SCARDCONTEXT hContext,
    _In_    LPCWSTR mszCards,
    _Inout_ LPSCARD_READERSTATEW rgReaderStates,
    _In_    DWORD cReaders);
#ifdef UNICODE
#define SCardLocateCards  SCardLocateCardsW
#else
#define SCardLocateCards  SCardLocateCardsA
#endif // !UNICODE

#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef struct _SCARD_ATRMASK {
    DWORD       cbAtr;          // Number of bytes in the ATR and the mask.
    BYTE        rgbAtr[36];     // Atr of card (extra alignment bytes)
    BYTE        rgbMask[36];    // Mask for the Atr (extra alignment bytes)
} SCARD_ATRMASK, *PSCARD_ATRMASK, *LPSCARD_ATRMASK;


extern WINSCARDAPI LONG WINAPI
SCardLocateCardsByATRA(
    _In_    SCARDCONTEXT hContext,
    _In_    LPSCARD_ATRMASK rgAtrMasks,
    _In_    DWORD cAtrs,
    _Inout_ LPSCARD_READERSTATEA rgReaderStates,
    _In_    DWORD cReaders);
extern WINSCARDAPI LONG WINAPI
SCardLocateCardsByATRW(
    _In_    SCARDCONTEXT hContext,
    _In_    LPSCARD_ATRMASK rgAtrMasks,
    _In_    DWORD cAtrs,
    _Inout_ LPSCARD_READERSTATEW rgReaderStates,
    _In_    DWORD cReaders);
#ifdef UNICODE
#define SCardLocateCardsByATR  SCardLocateCardsByATRW
#else
#define SCardLocateCardsByATR  SCardLocateCardsByATRA
#endif // !UNICODE
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

extern WINSCARDAPI LONG WINAPI
SCardGetStatusChangeA(
    _In_    SCARDCONTEXT hContext,
    _In_    DWORD dwTimeout,
    _Inout_ LPSCARD_READERSTATEA rgReaderStates,
    _In_    DWORD cReaders);
extern WINSCARDAPI LONG WINAPI
SCardGetStatusChangeW(
    _In_    SCARDCONTEXT hContext,
    _In_    DWORD dwTimeout,
    _Inout_ LPSCARD_READERSTATEW rgReaderStates,
    _In_    DWORD cReaders);
#ifdef UNICODE
#define SCardGetStatusChange  SCardGetStatusChangeW
#else
#define SCardGetStatusChange  SCardGetStatusChangeA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardCancel(
    _In_    SCARDCONTEXT hContext);


//
////////////////////////////////////////////////////////////////////////////////
//
//  Card/Reader Communication Services
//
//      The following services provide means for communication with the card.
//

#define SCARD_SHARE_EXCLUSIVE 1 // This application is not willing to share this
                                // card with other applications.
#define SCARD_SHARE_SHARED    2 // This application is willing to share this
                                // card with other applications.
#define SCARD_SHARE_DIRECT    3 // This application demands direct control of
                                // the reader, so it is not available to other
                                // applications.

#define SCARD_LEAVE_CARD      0 // Don't do anything special on close
#define SCARD_RESET_CARD      1 // Reset the card on close
#define SCARD_UNPOWER_CARD    2 // Power down the card on close
#define SCARD_EJECT_CARD      3 // Eject the card on close

extern WINSCARDAPI LONG WINAPI
SCardConnectA(
    _In_    SCARDCONTEXT hContext,
    _In_    LPCSTR szReader,
    _In_    DWORD dwShareMode,
    _In_    DWORD dwPreferredProtocols,
    _Out_   LPSCARDHANDLE phCard,
    _Out_   LPDWORD pdwActiveProtocol);
extern WINSCARDAPI LONG WINAPI
SCardConnectW(
    _In_    SCARDCONTEXT hContext,
    _In_    LPCWSTR szReader,
    _In_    DWORD dwShareMode,
    _In_    DWORD dwPreferredProtocols,
    _Out_   LPSCARDHANDLE phCard,
    _Out_   LPDWORD pdwActiveProtocol);
#ifdef UNICODE
#define SCardConnect  SCardConnectW
#else
#define SCardConnect  SCardConnectA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardReconnect(
    _In_      SCARDHANDLE hCard,
    _In_      DWORD dwShareMode,
    _In_      DWORD dwPreferredProtocols,
    _In_      DWORD dwInitialization,
    _Out_opt_ LPDWORD pdwActiveProtocol);

extern WINSCARDAPI LONG WINAPI
SCardDisconnect(
    _In_    SCARDHANDLE hCard,
    _In_    DWORD dwDisposition);

extern WINSCARDAPI LONG WINAPI
SCardBeginTransaction(
    _In_    SCARDHANDLE hCard);

extern WINSCARDAPI LONG WINAPI
SCardEndTransaction(
    _In_    SCARDHANDLE hCard,
    _In_    DWORD dwDisposition);

extern WINSCARDAPI LONG WINAPI
SCardCancelTransaction(
    _In_    SCARDHANDLE hCard);
//
// NOTE:    This call corresponds to the PC/SC SCARDCOMM::Cancel routine,
//          terminating a blocked SCardBeginTransaction service.
//


extern WINSCARDAPI LONG WINAPI
SCardState(
    _In_    SCARDHANDLE hCard,
    _Out_   LPDWORD pdwState,
    _Out_   LPDWORD pdwProtocol,
    _Out_writes_bytes_(*pcbAtrLen)   LPBYTE pbAtr,
    _Inout_ LPDWORD pcbAtrLen);
//
// NOTE:    SCardState is an obsolete routine.  PC/SC has replaced it with
//          SCardStatus.
//

extern WINSCARDAPI LONG WINAPI
SCardStatusA(
    _In_        SCARDHANDLE hCard,
    _When_(_Old_(*pcchReaderLen) == SCARD_AUTOALLOCATE, _At_((LPSTR *)mszReaderNames, _Outptr_result_buffer_maybenull_(*pcchReaderLen) _At_(*_Curr_, _Post_z_ _Post_ _NullNull_terminated_)))
    _When_(_Old_(*pcchReaderLen) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcchReaderLen) _Post_ _NullNull_terminated_)
                LPSTR mszReaderNames,
    _Inout_opt_ LPDWORD pcchReaderLen,
    _Out_opt_   LPDWORD pdwState,
    _Out_opt_   LPDWORD pdwProtocol,
    _When_(_Old_(*pcbAtrLen) == SCARD_AUTOALLOCATE, _At_((LPBYTE *)pbAtr, _Outptr_result_buffer_maybenull_(*pcbAtrLen) _At_(*_Curr_, _Post_ _NullNull_terminated_)))
    _When_(_Old_(*pcbAtrLen) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcbAtrLen) _Post_ _NullNull_terminated_)
                LPBYTE pbAtr,
    _Inout_opt_ LPDWORD pcbAtrLen);
extern WINSCARDAPI LONG WINAPI
SCardStatusW(
    _In_        SCARDHANDLE hCard,
    _When_(_Old_(*pcchReaderLen) == SCARD_AUTOALLOCATE, _At_((LPWSTR *)mszReaderNames, _Outptr_result_buffer_maybenull_(*pcchReaderLen) _At_(*_Curr_, _Post_z_ _Post_ _NullNull_terminated_)))
    _When_(_Old_(*pcchReaderLen) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcchReaderLen) _Post_ _NullNull_terminated_)
                LPWSTR mszReaderNames,
    _Inout_opt_ LPDWORD pcchReaderLen,
    _Out_opt_   LPDWORD pdwState,
    _Out_opt_   LPDWORD pdwProtocol,
    _When_(_Old_(*pcbAtrLen) == SCARD_AUTOALLOCATE, _At_((LPBYTE *)pbAtr, _Outptr_result_buffer_maybenull_(*pcbAtrLen) _At_(*_Curr_, _Post_ _NullNull_terminated_)))
    _When_(_Old_(*pcbAtrLen) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcbAtrLen) _Post_ _NullNull_terminated_)
                LPBYTE pbAtr,
    _Inout_opt_ LPDWORD pcbAtrLen);
#ifdef UNICODE
#define SCardStatus  SCardStatusW
#else
#define SCardStatus  SCardStatusA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardTransmit(
    _In_        SCARDHANDLE hCard,
    _In_        LPCSCARD_IO_REQUEST pioSendPci,
    _In_reads_bytes_(cbSendLength) LPCBYTE pbSendBuffer,
    _In_        DWORD cbSendLength,
    _Inout_opt_ LPSCARD_IO_REQUEST pioRecvPci,
    _Out_writes_bytes_(*pcbRecvLength) LPBYTE pbRecvBuffer,
    _Inout_     LPDWORD pcbRecvLength);

#if (NTDDI_VERSION >= NTDDI_VISTA)
extern WINSCARDAPI LONG WINAPI
SCardGetTransmitCount(
    _In_ SCARDHANDLE hCard,
    _Out_ LPDWORD pcTransmitCount);
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//
////////////////////////////////////////////////////////////////////////////////
//
//  Reader Control Routines
//
//      The following services provide for direct, low-level manipulation of the
//      reader by the calling application allowing it control over the
//      attributes of the communications with the card.
//

extern WINSCARDAPI LONG WINAPI
SCardControl(
    _In_    SCARDHANDLE hCard,
    _In_    DWORD dwControlCode,
    _In_reads_bytes_(cbInBufferSize) LPCVOID lpInBuffer,
    _In_    DWORD cbInBufferSize,
    _Out_writes_bytes_(cbOutBufferSize) LPVOID lpOutBuffer,
    _In_    DWORD cbOutBufferSize,
    _Out_   LPDWORD lpBytesReturned);

extern WINSCARDAPI LONG WINAPI
SCardGetAttrib(
    _In_    SCARDHANDLE hCard,
    _In_    DWORD dwAttrId,
    _Out_writes_bytes_opt_(*pcbAttrLen) LPBYTE pbAttr,
    _Inout_ LPDWORD pcbAttrLen);
//
// NOTE:    The routine SCardGetAttrib's name differs from the PC/SC definition.
//          It should be:
//
//              extern WINSCARDAPI LONG WINAPI
//              SCardGetReaderCapabilities(
//                  _In_    SCARDHANDLE hCard,
//                  _In_    DWORD dwTag,
//                  _Out_   LPBYTE pbAttr,
//                  _Inout_ LPDWORD pcbAttrLen);
//
//          Here's a work-around MACRO:
#define SCardGetReaderCapabilities SCardGetAttrib

extern WINSCARDAPI LONG WINAPI
SCardSetAttrib(
    _In_ SCARDHANDLE hCard,
    _In_ DWORD dwAttrId,
    _In_reads_bytes_(cbAttrLen) LPCBYTE pbAttr,
    _In_ DWORD cbAttrLen);
//
// NOTE:    The routine SCardSetAttrib's name differs from the PC/SC definition.
//          It should be:
//
//              extern WINSCARDAPI LONG WINAPI
//              SCardSetReaderCapabilities(
//                  _In_    SCARDHANDLE hCard,
//                  _In_    DWORD dwTag,
//                  _In_    LPCBYTE pbAttr,
//                  _In_    DWORD cbAttrLen);
//
//          Here's a work-around MACRO:
#define SCardSetReaderCapabilities SCardSetAttrib


//
////////////////////////////////////////////////////////////////////////////////
//
//  Smart Card Dialog definitions
//
//      The following section contains structures and  exported function
//      declarations for the Smart Card Common Dialog dialog.
//

// Defined constants
// Flags
#define SC_DLG_MINIMAL_UI       0x01
#define SC_DLG_NO_UI            0x02
#define SC_DLG_FORCE_UI         0x04

#define SCERR_NOCARDNAME        0x4000
#define SCERR_NOGUIDS           0x8000

typedef SCARDHANDLE (WINAPI *LPOCNCONNPROCA) (_In_ SCARDCONTEXT, _In_ LPSTR, _In_ LPSTR, _In_ PVOID);
typedef SCARDHANDLE (WINAPI *LPOCNCONNPROCW) (_In_ SCARDCONTEXT, _In_ LPWSTR, _In_ LPWSTR, _In_ PVOID);
#ifdef UNICODE
#define LPOCNCONNPROC  LPOCNCONNPROCW
#else
#define LPOCNCONNPROC  LPOCNCONNPROCA
#endif // !UNICODE
typedef BOOL (WINAPI *LPOCNCHKPROC) (_In_ SCARDCONTEXT, _In_ SCARDHANDLE, _In_ PVOID);
typedef void (WINAPI *LPOCNDSCPROC) (_In_ SCARDCONTEXT, _In_ SCARDHANDLE, _In_ PVOID);


//
// OPENCARD_SEARCH_CRITERIA: In order to specify a user-extended search,
// lpfnCheck must not be NULL.  Moreover, the connection to be made to the
// card before performing the callback must be indicated by either providing
// lpfnConnect and lpfnDisconnect OR by setting dwShareMode.
// If both the connection callbacks and dwShareMode are non-NULL, the callbacks
// will be used.
//

typedef struct {
    DWORD           dwStructSize;
    LPSTR           lpstrGroupNames;        // OPTIONAL reader groups to include in
    DWORD           nMaxGroupNames;         //          search.  NULL defaults to
                                            //          SCard$DefaultReaders
    LPCGUID         rgguidInterfaces;       // OPTIONAL requested interfaces
    DWORD           cguidInterfaces;        //          supported by card's SSP
    LPSTR           lpstrCardNames;         // OPTIONAL requested card names; all cards w/
    DWORD           nMaxCardNames;          //          matching ATRs will be accepted
    LPOCNCHKPROC    lpfnCheck;              // OPTIONAL if NULL no user check will be performed.
    LPOCNCONNPROCA  lpfnConnect;            // OPTIONAL if lpfnConnect is provided,
    LPOCNDSCPROC    lpfnDisconnect;         //          lpfnDisconnect must also be set.
    LPVOID          pvUserData;             // OPTIONAL parameter to callbacks
    DWORD           dwShareMode;            // OPTIONAL must be set if lpfnCheck is not null
    DWORD           dwPreferredProtocols;   // OPTIONAL
} OPENCARD_SEARCH_CRITERIAA, *POPENCARD_SEARCH_CRITERIAA, *LPOPENCARD_SEARCH_CRITERIAA;
typedef struct {
    DWORD           dwStructSize;
    LPWSTR          lpstrGroupNames;        // OPTIONAL reader groups to include in
    DWORD           nMaxGroupNames;         //          search.  NULL defaults to
                                            //          SCard$DefaultReaders
    LPCGUID         rgguidInterfaces;       // OPTIONAL requested interfaces
    DWORD           cguidInterfaces;        //          supported by card's SSP
    LPWSTR          lpstrCardNames;         // OPTIONAL requested card names; all cards w/
    DWORD           nMaxCardNames;          //          matching ATRs will be accepted
    LPOCNCHKPROC    lpfnCheck;              // OPTIONAL if NULL no user check will be performed.
    LPOCNCONNPROCW  lpfnConnect;            // OPTIONAL if lpfnConnect is provided,
    LPOCNDSCPROC    lpfnDisconnect;         //          lpfnDisconnect must also be set.
    LPVOID          pvUserData;             // OPTIONAL parameter to callbacks
    DWORD           dwShareMode;            // OPTIONAL must be set if lpfnCheck is not null
    DWORD           dwPreferredProtocols;   // OPTIONAL
} OPENCARD_SEARCH_CRITERIAW, *POPENCARD_SEARCH_CRITERIAW, *LPOPENCARD_SEARCH_CRITERIAW;
#ifdef UNICODE
typedef OPENCARD_SEARCH_CRITERIAW OPENCARD_SEARCH_CRITERIA;
typedef POPENCARD_SEARCH_CRITERIAW POPENCARD_SEARCH_CRITERIA;
typedef LPOPENCARD_SEARCH_CRITERIAW LPOPENCARD_SEARCH_CRITERIA;
#else
typedef OPENCARD_SEARCH_CRITERIAA OPENCARD_SEARCH_CRITERIA;
typedef POPENCARD_SEARCH_CRITERIAA POPENCARD_SEARCH_CRITERIA;
typedef LPOPENCARD_SEARCH_CRITERIAA LPOPENCARD_SEARCH_CRITERIA;
#endif // UNICODE


//
// OPENCARDNAME_EX: used by SCardUIDlgSelectCard; replaces obsolete OPENCARDNAME
//

typedef struct {
    DWORD           dwStructSize;           // REQUIRED
    SCARDCONTEXT    hSCardContext;          // REQUIRED
    HWND            hwndOwner;              // OPTIONAL
    DWORD           dwFlags;                // OPTIONAL -- default is SC_DLG_MINIMAL_UI
    LPCSTR          lpstrTitle;             // OPTIONAL
    LPCSTR          lpstrSearchDesc;        // OPTIONAL (eg. "Please insert your <brandname> smart card.")
    HICON           hIcon;                  // OPTIONAL 32x32 icon for your brand insignia
    POPENCARD_SEARCH_CRITERIAA pOpenCardSearchCriteria; // OPTIONAL
    LPOCNCONNPROCA  lpfnConnect;            // OPTIONAL - performed on successful selection
    LPVOID          pvUserData;             // OPTIONAL parameter to lpfnConnect
    DWORD           dwShareMode;            // OPTIONAL - if lpfnConnect is NULL, dwShareMode and
    DWORD           dwPreferredProtocols;   // OPTIONAL dwPreferredProtocols will be used to
                                            //          connect to the selected card
    LPSTR           lpstrRdr;               // REQUIRED [IN|OUT] Name of selected reader
    DWORD           nMaxRdr;                // REQUIRED [IN|OUT]
    LPSTR           lpstrCard;              // REQUIRED [IN|OUT] Name of selected card
    DWORD           nMaxCard;               // REQUIRED [IN|OUT]
    DWORD           dwActiveProtocol;       // [OUT] set only if dwShareMode not NULL
    SCARDHANDLE     hCardHandle;            // [OUT] set if a card connection was indicated
} OPENCARDNAME_EXA, *POPENCARDNAME_EXA, *LPOPENCARDNAME_EXA;
typedef struct {
    DWORD           dwStructSize;           // REQUIRED
    SCARDCONTEXT    hSCardContext;          // REQUIRED
    HWND            hwndOwner;              // OPTIONAL
    DWORD           dwFlags;                // OPTIONAL -- default is SC_DLG_MINIMAL_UI
    LPCWSTR         lpstrTitle;             // OPTIONAL
    LPCWSTR         lpstrSearchDesc;        // OPTIONAL (eg. "Please insert your <brandname> smart card.")
    HICON           hIcon;                  // OPTIONAL 32x32 icon for your brand insignia
    POPENCARD_SEARCH_CRITERIAW pOpenCardSearchCriteria; // OPTIONAL
    LPOCNCONNPROCW  lpfnConnect;            // OPTIONAL - performed on successful selection
    LPVOID          pvUserData;             // OPTIONAL parameter to lpfnConnect
    DWORD           dwShareMode;            // OPTIONAL - if lpfnConnect is NULL, dwShareMode and
    DWORD           dwPreferredProtocols;   // OPTIONAL dwPreferredProtocols will be used to
                                            //          connect to the selected card
    LPWSTR          lpstrRdr;               // REQUIRED [IN|OUT] Name of selected reader
    DWORD           nMaxRdr;                // REQUIRED [IN|OUT]
    LPWSTR          lpstrCard;              // REQUIRED [IN|OUT] Name of selected card
    DWORD           nMaxCard;               // REQUIRED [IN|OUT]
    DWORD           dwActiveProtocol;       // [OUT] set only if dwShareMode not NULL
    SCARDHANDLE     hCardHandle;            // [OUT] set if a card connection was indicated
} OPENCARDNAME_EXW, *POPENCARDNAME_EXW, *LPOPENCARDNAME_EXW;
#ifdef UNICODE
typedef OPENCARDNAME_EXW OPENCARDNAME_EX;
typedef POPENCARDNAME_EXW POPENCARDNAME_EX;
typedef LPOPENCARDNAME_EXW LPOPENCARDNAME_EX;
#else
typedef OPENCARDNAME_EXA OPENCARDNAME_EX;
typedef POPENCARDNAME_EXA POPENCARDNAME_EX;
typedef LPOPENCARDNAME_EXA LPOPENCARDNAME_EX;
#endif // UNICODE

#define OPENCARDNAMEA_EX OPENCARDNAME_EXA
#define OPENCARDNAMEW_EX OPENCARDNAME_EXW
#define POPENCARDNAMEA_EX POPENCARDNAME_EXA
#define POPENCARDNAMEW_EX POPENCARDNAME_EXW
#define LPOPENCARDNAMEA_EX LPOPENCARDNAME_EXA
#define LPOPENCARDNAMEW_EX LPOPENCARDNAME_EXW


//
// Smart Card Reader Selection Provider
//
// Only UNICODE is supported. Invoke smart card reader selection provider by calling
// CredUIPromptForWindowsCredentials() supplying SCARD_READER_SEL_AUTH_PACKAGE as
// pulAuthPackage, an instance of READER_SEL_REQUEST as pvInAuthBuffer and setting
// CREDUIWIN_AUTHPACKAGE_ONLY in dwFlags. Upon successful return, an instance of
// READER_SEL_RESPONSE will be returned in ppvOutAuthBuffer.
//

#define SCARD_READER_SEL_AUTH_PACKAGE   ((DWORD)-629)

//
// READER_SEL_REQUEST
//     Reader selection request to reader selection provider
//
// Members:
//
// dwShareMode:
//     Share mode used by SCardConnect to connect to smart cards
// dwPreferredProtocols:
//     Acceptable protocols for SCardConnect to connect to smart cards
//
// MatchType:
//     Indicates how the caller wants the reader selection provider to verify smart
//     cards.
//
//     If MatchType is set to RSR_MATCH_TYPE_READER_AND_CONTAINER, reader selection
//     provider will match smart cards based on whether they are in the given
//     reader and have the given key container. Reader name and container name are
//     both optional. Reader name and container name, if any, need to be appended
//     after READER_SEL_REQUEST structure and set their offsets and lengths in
//     ReaderAndContainerParameter member.
//
//     If MatchType is set to RSR_MATCH_TYPE_SERIAL_NUMBER, reader selection
//     provider will match smart cards based on their serial numbers / card IDs.
//     Serial number is required. It needs to be appended after READER_SEL_REQUEST
//     structure as a byte array and set its offset and length in
//     SerialNumberParameter member.
//
//     If MatchType is set to RSR_MATCH_TYPE_ALL_CARDS, reader selection provider
//     will allow all recognized cards to be selected by user without any filtering.
//     The card may not be personalized for Base CSP / Smart Card KSP yet, or even
//     have its own CSP.
//
// ReaderAndContainerParameter.cbReaderNameOffset:
//     Byte offset of reader name UNICODE string from the beginning of
//     READER_SEL_REQUEST structure
// ReaderAndContainerParameter.cchReaderNameLength:
//     Number of characters in reader name UNICODE string including the terminating
//     NULL character
// ReaderAndContainerParameter.cbContainerNameOffset:
//     Byte offset of container name UNICODE string from the beginning of
//     READER_SEL_REQUEST structure
// ReaderAndContainerParameter.cchContainerNameLength:
//     Number of characters in container name UNICODE string including the
//     terminating NULL character
// ReaderAndContainerParameter.dwDesiredCardModuleVersion:
//     The desired smart card module version
// ReaderAndContainerParameter.dwCspFlags:
//     CSP and KSP flags to indicate how smart cards will be used
//     (Valid flags include CRYPT_NEWKEYSET, CRYPT_DELETEKEYSET, CRYPT_VERIFYCONTEXT
//     and CRYPT_DEFAULT_CONTAINER_OPTIONAL)
//
// SerialNumberParameter.cbSerialNumberOffset:
//     Byte offset of serial number byte array from the beginning of
//     READER_SEL_REQUEST structure
// SerialNumberParameter.cbSerialNumberLength:
//     Number of bytes in serial number byte array
// SerialNumberParameter.dwDesiredCardModuleVersion:
//     The desired smart card module version
//

typedef enum {
    RSR_MATCH_TYPE_READER_AND_CONTAINER = 1,
    RSR_MATCH_TYPE_SERIAL_NUMBER,
    RSR_MATCH_TYPE_ALL_CARDS
} READER_SEL_REQUEST_MATCH_TYPE;

typedef struct {
    DWORD                           dwShareMode;
    DWORD                           dwPreferredProtocols;
    READER_SEL_REQUEST_MATCH_TYPE   MatchType;
#pragma warning(push)
#pragma warning(disable:4201)
    union {
        struct {
            DWORD                   cbReaderNameOffset;
            DWORD                   cchReaderNameLength;
            DWORD                   cbContainerNameOffset;
            DWORD                   cchContainerNameLength;
            DWORD                   dwDesiredCardModuleVersion;
            DWORD                   dwCspFlags;
        } ReaderAndContainerParameter;
        struct {
            DWORD                   cbSerialNumberOffset;
            DWORD                   cbSerialNumberLength;
            DWORD                   dwDesiredCardModuleVersion;
        } SerialNumberParameter;
    } DUMMYUNIONNAME;
#pragma warning(pop)
} READER_SEL_REQUEST, *PREADER_SEL_REQUEST;

//
// READER_SEL_RESPONSE
//     Reader selection response from reader selection provider
//
// Members:
// cbReaderNameOffset:
//     Byte offset of matched reader name UNICODE string from the beginning of
//     READER_SEL_RESPONSE structure
// cchReaderNameLength:
//     Number of characters in matched reader name UNICODE string including the
//     terminating NULL character
// cbCardNameOffset:
//     Byte offset of matched card name UNICODE string from the beginning of
//     READER_SEL_RESPONSE structure
// cchCardNameLength:
//     Number of characters in matched card name UNICODE string including the
//     terminating NULL character
//

typedef struct {
    DWORD                           cbReaderNameOffset;
    DWORD                           cchReaderNameLength;
    DWORD                           cbCardNameOffset;
    DWORD                           cchCardNameLength;
} READER_SEL_RESPONSE, *PREADER_SEL_RESPONSE;


//
// SCardUIDlgSelectCard replaces GetOpenCardName
//

extern WINSCARDAPI LONG WINAPI
SCardUIDlgSelectCardA(
    LPOPENCARDNAMEA_EX);
extern WINSCARDAPI LONG WINAPI
SCardUIDlgSelectCardW(
    LPOPENCARDNAMEW_EX);
#ifdef UNICODE
#define SCardUIDlgSelectCard  SCardUIDlgSelectCardW
#else
#define SCardUIDlgSelectCard  SCardUIDlgSelectCardA
#endif // !UNICODE


//
// "Smart Card Common Dialog" definitions for backwards compatibility
//  with the Smart Card Base Services SDK version 1.0
//

typedef struct {
    DWORD           dwStructSize;
    HWND            hwndOwner;
    SCARDCONTEXT    hSCardContext;
    LPSTR           lpstrGroupNames;
    DWORD           nMaxGroupNames;
    LPSTR           lpstrCardNames;
    DWORD           nMaxCardNames;
    LPCGUID         rgguidInterfaces;
    DWORD           cguidInterfaces;
    LPSTR           lpstrRdr;
    DWORD           nMaxRdr;
    LPSTR           lpstrCard;
    DWORD           nMaxCard;
    LPCSTR          lpstrTitle;
    DWORD           dwFlags;
    LPVOID          pvUserData;
    DWORD           dwShareMode;
    DWORD           dwPreferredProtocols;
    DWORD           dwActiveProtocol;
    LPOCNCONNPROCA  lpfnConnect;
    LPOCNCHKPROC    lpfnCheck;
    LPOCNDSCPROC    lpfnDisconnect;
    SCARDHANDLE     hCardHandle;
} OPENCARDNAMEA, *POPENCARDNAMEA, *LPOPENCARDNAMEA;
typedef struct {
    DWORD           dwStructSize;
    HWND            hwndOwner;
    SCARDCONTEXT    hSCardContext;
    LPWSTR          lpstrGroupNames;
    DWORD           nMaxGroupNames;
    LPWSTR          lpstrCardNames;
    DWORD           nMaxCardNames;
    LPCGUID         rgguidInterfaces;
    DWORD           cguidInterfaces;
    LPWSTR          lpstrRdr;
    DWORD           nMaxRdr;
    LPWSTR          lpstrCard;
    DWORD           nMaxCard;
    LPCWSTR         lpstrTitle;
    DWORD           dwFlags;
    LPVOID          pvUserData;
    DWORD           dwShareMode;
    DWORD           dwPreferredProtocols;
    DWORD           dwActiveProtocol;
    LPOCNCONNPROCW  lpfnConnect;
    LPOCNCHKPROC    lpfnCheck;
    LPOCNDSCPROC    lpfnDisconnect;
    SCARDHANDLE     hCardHandle;
} OPENCARDNAMEW, *POPENCARDNAMEW, *LPOPENCARDNAMEW;
#ifdef UNICODE
typedef OPENCARDNAMEW OPENCARDNAME;
typedef POPENCARDNAMEW POPENCARDNAME;
typedef LPOPENCARDNAMEW LPOPENCARDNAME;
#else
typedef OPENCARDNAMEA OPENCARDNAME;
typedef POPENCARDNAMEA POPENCARDNAME;
typedef LPOPENCARDNAMEA LPOPENCARDNAME;
#endif // UNICODE

// Backwards compatibility macros
#define OPENCARDNAME_A OPENCARDNAMEA
#define OPENCARDNAME_W OPENCARDNAMEW
#define POPENCARDNAME_A POPENCARDNAMEA
#define POPENCARDNAME_W POPENCARDNAMEW
#define LPOPENCARDNAME_A LPOPENCARDNAMEA
#define LPOPENCARDNAME_W LPOPENCARDNAMEW

extern WINSCARDAPI LONG WINAPI
GetOpenCardNameA(
    LPOPENCARDNAMEA);
extern WINSCARDAPI LONG WINAPI
GetOpenCardNameW(
    LPOPENCARDNAMEW);
#ifdef UNICODE
#define GetOpenCardName  GetOpenCardNameW
#else
#define GetOpenCardName  GetOpenCardNameA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardDlgExtendedError (void);

#if (NTDDI_VERSION >= NTDDI_VISTA)

//
// Smartcard Caching API
//

extern WINSCARDAPI LONG WINAPI
SCardReadCacheA(
    _In_  SCARDCONTEXT hContext,
    _In_  UUID *CardIdentifier,
    _In_  DWORD FreshnessCounter,
    _In_  LPSTR LookupName,
    _Out_writes_bytes_(*DataLen) PBYTE Data,
    _Out_ DWORD *DataLen);
extern WINSCARDAPI LONG WINAPI
SCardReadCacheW(
    _In_  SCARDCONTEXT hContext,
    _In_  UUID *CardIdentifier,
    _In_  DWORD FreshnessCounter,
    _In_  LPWSTR LookupName,
    _Out_writes_bytes_(*DataLen) PBYTE Data,
    _Out_ DWORD *DataLen);
#ifdef UNICODE
#define SCardReadCache  SCardReadCacheW
#else
#define SCardReadCache  SCardReadCacheA
#endif // !UNICODE

extern WINSCARDAPI LONG WINAPI
SCardWriteCacheA(
    _In_ SCARDCONTEXT hContext,
    _In_ UUID *CardIdentifier,
    _In_ DWORD FreshnessCounter,
    _In_ LPSTR LookupName,
    _In_reads_bytes_(DataLen) PBYTE Data,
    _In_ DWORD DataLen);
extern WINSCARDAPI LONG WINAPI
SCardWriteCacheW(
    _In_ SCARDCONTEXT hContext,
    _In_ UUID *CardIdentifier,
    _In_ DWORD FreshnessCounter,
    _In_ LPWSTR LookupName,
    _In_reads_bytes_(DataLen) PBYTE Data,
    _In_ DWORD DataLen);
#ifdef UNICODE
#define SCardWriteCache  SCardWriteCacheW
#else
#define SCardWriteCache  SCardWriteCacheA
#endif // !UNICODE

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)

_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI 
SCardGetReaderIconA(
  _In_     SCARDCONTEXT hContext,
  _In_     LPCSTR szReaderName, 
  _When_(_Old_(*pcbIcon) == SCARD_AUTOALLOCATE, _At_((LPBYTE *)pbIcon, _Outptr_result_bytebuffer_all_maybenull_(*pcbIcon)))
  _When_(_Old_(*pcbIcon) != SCARD_AUTOALLOCATE, _Out_writes_bytes_to_(*pcbIcon, *pcbIcon) _Post_z_)
           LPBYTE pbIcon,
  _Inout_  LPDWORD pcbIcon);
_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI 
SCardGetReaderIconW(
  _In_     SCARDCONTEXT hContext,
  _In_     LPCWSTR szReaderName, 
  _When_(_Old_(*pcbIcon) == SCARD_AUTOALLOCATE, _At_((LPBYTE *)pbIcon, _Outptr_result_bytebuffer_all_maybenull_(*pcbIcon)))
  _When_(_Old_(*pcbIcon) != SCARD_AUTOALLOCATE, _Out_writes_bytes_to_(*pcbIcon, *pcbIcon) _Post_z_)
           LPBYTE pbIcon,
  _Inout_  LPDWORD pcbIcon);
#ifdef UNICODE
#define SCardGetReaderIcon  SCardGetReaderIconW
#else
#define SCardGetReaderIcon  SCardGetReaderIconA
#endif // !UNICODE

_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI 
SCardGetDeviceTypeIdA(
  _In_     SCARDCONTEXT hContext,
  _In_     LPCSTR szReaderName, 
  _Inout_  LPDWORD pdwDeviceTypeId);
_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI 
SCardGetDeviceTypeIdW(
  _In_     SCARDCONTEXT hContext,
  _In_     LPCWSTR szReaderName, 
  _Inout_  LPDWORD pdwDeviceTypeId);
#ifdef UNICODE
#define SCardGetDeviceTypeId  SCardGetDeviceTypeIdW
#else
#define SCardGetDeviceTypeId  SCardGetDeviceTypeIdA
#endif // !UNICODE
  
_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardGetReaderDeviceInstanceIdA(
  _In_    SCARDCONTEXT hContext,
  _In_    LPCSTR szReaderName,
  _When_(_Old_(*pcchDeviceInstanceId) == SCARD_AUTOALLOCATE, _At_((LPSTR *)szDeviceInstanceId, _Outptr_result_buffer_maybenull_(*pcchDeviceInstanceId) _At_(*_Curr_, _Post_z_ _Post_ _NullNull_terminated_)))
  _When_(_Old_(*pcchDeviceInstanceId) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcchDeviceInstanceId) _Post_ _NullNull_terminated_)
          LPSTR szDeviceInstanceId,
  _Inout_ LPDWORD pcchDeviceInstanceId);
_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardGetReaderDeviceInstanceIdW(
  _In_    SCARDCONTEXT hContext,
  _In_    LPCWSTR szReaderName,
  _When_(_Old_(*pcchDeviceInstanceId) == SCARD_AUTOALLOCATE, _At_((LPWSTR *)szDeviceInstanceId, _Outptr_result_buffer_maybenull_(*pcchDeviceInstanceId) _At_(*_Curr_, _Post_z_ _Post_ _NullNull_terminated_)))
  _When_(_Old_(*pcchDeviceInstanceId) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcchDeviceInstanceId) _Post_ _NullNull_terminated_)
          LPWSTR szDeviceInstanceId,
  _Inout_ LPDWORD pcchDeviceInstanceId);
#ifdef UNICODE
#define SCardGetReaderDeviceInstanceId  SCardGetReaderDeviceInstanceIdW
#else
#define SCardGetReaderDeviceInstanceId  SCardGetReaderDeviceInstanceIdA
#endif // !UNICODE

_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardListReadersWithDeviceInstanceIdA(
  _In_    SCARDCONTEXT hContext,
  _In_    LPCSTR szDeviceInstanceId,
  _When_(_Old_(*pcchReaders) == SCARD_AUTOALLOCATE, _At_((LPSTR *)mszReaders, _Outptr_result_buffer_maybenull_(*pcchReaders) _At_(*_Curr_, _Post_z_ _Post_ _NullNull_terminated_)))
  _When_(_Old_(*pcchReaders) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcchReaders) _Post_ _NullNull_terminated_)
          LPSTR mszReaders,
  _Inout_ LPDWORD pcchReaders);
_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardListReadersWithDeviceInstanceIdW(
  _In_    SCARDCONTEXT hContext,
  _In_    LPCWSTR szDeviceInstanceId,
  _When_(_Old_(*pcchReaders) == SCARD_AUTOALLOCATE, _At_((LPWSTR *)mszReaders, _Outptr_result_buffer_maybenull_(*pcchReaders) _At_(*_Curr_, _Post_z_ _Post_ _NullNull_terminated_)))
  _When_(_Old_(*pcchReaders) != SCARD_AUTOALLOCATE, _Out_writes_opt_(*pcchReaders) _Post_ _NullNull_terminated_)
          LPWSTR mszReaders,
  _Inout_ LPDWORD pcchReaders);
#ifdef UNICODE
#define SCardListReadersWithDeviceInstanceId  SCardListReadersWithDeviceInstanceIdW
#else
#define SCardListReadersWithDeviceInstanceId  SCardListReadersWithDeviceInstanceIdA
#endif // !UNICODE

//
////////////////////////////////////////////////////////////////////////////////
//
//  Smart Card Auditing
//

#define SCARD_AUDIT_CHV_FAILURE 0x0 // A smart card holder verification (CHV) 
                                    // attempt failed.

#define SCARD_AUDIT_CHV_SUCCESS 0x1 // A smart card holder verification (CHV)
                                    // attempt succeeded.

_Success_(return == SCARD_S_SUCCESS)
extern WINSCARDAPI LONG WINAPI
SCardAudit(
  _In_ SCARDCONTEXT hContext,
  _In_ DWORD dwEvent);

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif
#endif // _WINSCARD_H_

