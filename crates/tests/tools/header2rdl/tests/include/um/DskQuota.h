/**************************************************************************
*                                                                         *
*   dskquota.h --  public header for Windows 2000 disk quota interfaces.  *
*                                                                         *
*   Copyright (c) 1991-1999, Microsoft Corp. All rights reserved.         *
*                                                                         *
**************************************************************************/
#ifndef __DSKQUOTA_H
#define __DSKQUOTA_H

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifndef _WINDOWS_
#include <windows.h>
#endif

#ifndef _OLE2_H_
#include <ole2.h>
#endif

#ifndef _OLECTL_H_
#include <olectl.h>
#endif

#ifdef INITGUIDS
#include <initguid.h>
#endif


//
// Class IDs
//
// {7988B571-EC89-11cf-9C00-00AA00A14F56}
DEFINE_GUID(CLSID_DiskQuotaControl,
0x7988b571, 0xec89, 0x11cf, 0x9c, 0x0, 0x0, 0xaa, 0x0, 0xa1, 0x4f, 0x56);

//
// Interface IDs
//
// {7988B572-EC89-11cf-9C00-00AA00A14F56}
DEFINE_GUID(IID_IDiskQuotaControl,
0x7988b572, 0xec89, 0x11cf, 0x9c, 0x0, 0x0, 0xaa, 0x0, 0xa1, 0x4f, 0x56);

// {7988B574-EC89-11cf-9C00-00AA00A14F56}
DEFINE_GUID(IID_IDiskQuotaUser,
0x7988b574, 0xec89, 0x11cf, 0x9c, 0x0, 0x0, 0xaa, 0x0, 0xa1, 0x4f, 0x56);

// {7988B576-EC89-11cf-9C00-00AA00A14F56}
DEFINE_GUID(IID_IDiskQuotaUserBatch,
0x7988b576, 0xec89, 0x11cf, 0x9c, 0x0, 0x0, 0xaa, 0x0, 0xa1, 0x4f, 0x56);

// {7988B577-EC89-11cf-9C00-00AA00A14F56}
DEFINE_GUID(IID_IEnumDiskQuotaUsers,
0x7988b577, 0xec89, 0x11cf, 0x9c, 0x0, 0x0, 0xaa, 0x0, 0xa1, 0x4f, 0x56);

// {7988B579-EC89-11cf-9C00-00AA00A14F56}
DEFINE_GUID(IID_IDiskQuotaEvents,
0x7988b579, 0xec89, 0x11cf, 0x9c, 0x0, 0x0, 0xaa, 0x0, 0xa1, 0x4f, 0x56);


//
// Definitions for value and bits in DWORD returned by 
// IDiskQuotaControl::GetQuotaState.
//
#define DISKQUOTA_STATE_DISABLED            0x00000000
#define DISKQUOTA_STATE_TRACK               0x00000001
#define DISKQUOTA_STATE_ENFORCE             0x00000002
#define DISKQUOTA_STATE_MASK                0x00000003
#define DISKQUOTA_FILESTATE_INCOMPLETE      0x00000100
#define DISKQUOTA_FILESTATE_REBUILDING      0x00000200
#define DISKQUOTA_FILESTATE_MASK            0x00000300

//
// Helper macros for setting and testing state value.
//
#define DISKQUOTA_SET_DISABLED(s) \
            ((s) &= ~DISKQUOTA_STATE_MASK)

#define DISKQUOTA_SET_TRACKED(s) \
            ((s) |= (DISKQUOTA_STATE_MASK & DISKQUOTA_STATE_TRACK))

#define DISKQUOTA_SET_ENFORCED(s) \
            ((s) |= (DISKQUOTA_STATE_ENFORCE & DISKQUOTA_STATE_ENFORCE))

#define DISKQUOTA_IS_DISABLED(s) \
            (DISKQUOTA_STATE_DISABLED == ((s) & DISKQUOTA_STATE_MASK))

#define DISKQUOTA_IS_TRACKED(s) \
            (DISKQUOTA_STATE_TRACK == ((s) & DISKQUOTA_STATE_MASK))

#define DISKQUOTA_IS_ENFORCED(s) \
            (DISKQUOTA_STATE_ENFORCE == ((s) & DISKQUOTA_STATE_MASK))
//
// These file state flags are read-only.
//
#define DISKQUOTA_FILE_INCOMPLETE(s) \
            (0 != ((s) & DISKQUOTA_FILESTATE_INCOMPLETE))

#define DISKQUOTA_FILE_REBUILDING(s) \
            (0 != ((s) & DISKQUOTA_FILESTATE_REBUILDING))


//
// Definitions for bits in DWORD returned by 
// IDiskQuotaControl::GetQuotaLogFlags.
//
#define DISKQUOTA_LOGFLAG_USER_THRESHOLD    0x00000001
#define DISKQUOTA_LOGFLAG_USER_LIMIT        0x00000002

//
// Helper macros to interrogate a log flags DWORD.
//
#define DISKQUOTA_IS_LOGGED_USER_THRESHOLD(f) \
            (0 != ((f) & DISKQUOTA_LOGFLAG_USER_THRESHOLD))

#define DISKQUOTA_IS_LOGGED_USER_LIMIT(f) \
            (0 != ((f) & DISKQUOTA_LOGFLAG_USER_LIMIT))

//
// Helper macros to set/clear bits in a log flags DWORD.
//
#define DISKQUOTA_SET_LOG_USER_THRESHOLD(f,yn) \
              ((f &= ~DISKQUOTA_LOGFLAG_USER_THRESHOLD) |= ((yn) ? DISKQUOTA_LOGFLAG_USER_THRESHOLD : 0))

#define DISKQUOTA_SET_LOG_USER_LIMIT(f,yn) \
              ((f &= ~DISKQUOTA_LOGFLAG_USER_LIMIT) |= ((yn) ? DISKQUOTA_LOGFLAG_USER_LIMIT : 0))

//
// Per-user quota information.
//
typedef struct DiskQuotaUserInformation {
    LONGLONG QuotaUsed;
    LONGLONG QuotaThreshold;
    LONGLONG QuotaLimit;
} DISKQUOTA_USER_INFORMATION, *PDISKQUOTA_USER_INFORMATION;


//
// Values for fNameResolution argument to:
//
//      IDiskQuotaControl::AddUserSid
//      IDiskQuotaControl::AddUserName
//      IDiskQuotaControl::FindUserSid
//      IDiskQuotaControl::CreateEnumUsers
//
#define DISKQUOTA_USERNAME_RESOLVE_NONE     0
#define DISKQUOTA_USERNAME_RESOLVE_SYNC     1
#define DISKQUOTA_USERNAME_RESOLVE_ASYNC    2

//
// Values for status returned by IDiskQuotaUser::GetAccountStatus.
//
#define DISKQUOTA_USER_ACCOUNT_RESOLVED     0
#define DISKQUOTA_USER_ACCOUNT_UNAVAILABLE  1
#define DISKQUOTA_USER_ACCOUNT_DELETED      2
#define DISKQUOTA_USER_ACCOUNT_INVALID      3
#define DISKQUOTA_USER_ACCOUNT_UNKNOWN      4
#define DISKQUOTA_USER_ACCOUNT_UNRESOLVED   5


//
// IDiskQuotaUser represents a single user quota record on a particular
// NTFS volume.  Objects using this interface are instantiated 
// through several IDiskQuotaControl methods.
//
#undef  INTERFACE
#define INTERFACE IDiskQuotaUser
DECLARE_INTERFACE_IID_(IDiskQuotaUser, IUnknown, "7988B574-EC89-11cf-9C00-00AA00A14F56")
{
    STDMETHOD(GetID)(THIS_
        ULONG *pulID) PURE;

    STDMETHOD(GetName)(THIS_
        LPWSTR pszAccountContainer,
        DWORD cchAccountContainer,
        LPWSTR pszLogonName,
        DWORD cchLogonName,
        LPWSTR pszDisplayName,
        DWORD cchDisplayName) PURE;

    STDMETHOD(GetSidLength)(THIS_
        LPDWORD pdwLength) PURE;

    STDMETHOD(GetSid)(THIS_
        LPBYTE pbSidBuffer,
        DWORD cbSidBuffer) PURE;

    STDMETHOD(GetQuotaThreshold)(THIS_
        PLONGLONG pllThreshold) PURE;

    STDMETHOD(GetQuotaThresholdText)(THIS_
        LPWSTR pszText,
        DWORD cchText) PURE;

    STDMETHOD(GetQuotaLimit)(THIS_
        PLONGLONG pllLimit) PURE;

    STDMETHOD(GetQuotaLimitText)(THIS_
        LPWSTR pszText,
        DWORD cchText) PURE;

    STDMETHOD(GetQuotaUsed)(THIS_
        PLONGLONG pllUsed) PURE;

    STDMETHOD(GetQuotaUsedText)(THIS_
        LPWSTR pszText,
        DWORD cchText) PURE;

    STDMETHOD(GetQuotaInformation)(THIS_
        LPVOID pbQuotaInfo,
        DWORD cbQuotaInfo) PURE;

    STDMETHOD(SetQuotaThreshold)(THIS_
        LONGLONG llThreshold,
        BOOL fWriteThrough) PURE;

    STDMETHOD(SetQuotaLimit)(THIS_
        LONGLONG llLimit,
        BOOL fWriteThrough) PURE;

    STDMETHOD(Invalidate)(THIS) PURE;

    STDMETHOD(GetAccountStatus)(THIS_
        LPDWORD pdwStatus) PURE;
};

typedef IDiskQuotaUser DISKQUOTA_USER, *PDISKQUOTA_USER;


//
// IEnumDiskQuotaUsers represents an enumerator created by 
// IDiskQuotaControl for the purpose of enumerating individual user quota
// records on a particular volume.  Each record is represented through
// the IDiskQuotaUser interface.
//
#undef  INTERFACE
#define INTERFACE IEnumDiskQuotaUsers
DECLARE_INTERFACE_IID_(IEnumDiskQuotaUsers, IUnknown, "7988B577-EC89-11cf-9C00-00AA00A14F56")
{
    STDMETHOD(Next)(THIS_
        DWORD cUsers,
        PDISKQUOTA_USER *rgUsers,
        LPDWORD pcUsersFetched) PURE;

    STDMETHOD(Skip)(THIS_
        DWORD cUsers) PURE;

    STDMETHOD(Reset)(THIS) PURE;

    STDMETHOD(Clone)(THIS_
        IEnumDiskQuotaUsers **ppEnum) PURE;
};

typedef IEnumDiskQuotaUsers ENUM_DISKQUOTA_USERS, *PENUM_DISKQUOTA_USERS;


//
// IDiskQuotaUserBatch represents a collection of IDiskQuotaUser 
// pointers for the purpose of grouping updates to quota information.
// 
#undef  INTERFACE
#define INTERFACE IDiskQuotaUserBatch
DECLARE_INTERFACE_IID_(IDiskQuotaUserBatch, IUnknown, "7988B576-EC89-11cf-9C00-00AA00A14F56")
{
    STDMETHOD(Add)(THIS_
        PDISKQUOTA_USER pUser) PURE;

    STDMETHOD(Remove)(THIS_
        PDISKQUOTA_USER pUser) PURE;

    STDMETHOD(RemoveAll)(THIS) PURE;

    STDMETHOD(FlushToDisk)(THIS) PURE;
};

typedef IDiskQuotaUserBatch DISKQUOTA_USER_BATCH, *PDISKQUOTA_USER_BATCH;


//
// IDiskQuotaControl represents a disk volume, providing query and 
// control of that volume's quota information.
//
#undef INTERFACE
#define INTERFACE IDiskQuotaControl
DECLARE_INTERFACE_IID_(IDiskQuotaControl, IConnectionPointContainer, "7988B572-EC89-11cf-9C00-00AA00A14F56")
{
    STDMETHOD(Initialize)(THIS_
        LPCWSTR pszPath,
        BOOL bReadWrite) PURE;

    STDMETHOD(SetQuotaState)(THIS_
        DWORD dwState) PURE;

    STDMETHOD(GetQuotaState)(THIS_
        LPDWORD pdwState) PURE;

    STDMETHOD(SetQuotaLogFlags)(THIS_
        DWORD dwFlags) PURE;

    STDMETHOD(GetQuotaLogFlags)(THIS_
        LPDWORD pdwFlags) PURE;

    STDMETHOD(SetDefaultQuotaThreshold)(THIS_
        LONGLONG llThreshold) PURE;

    STDMETHOD(GetDefaultQuotaThreshold)(THIS_
        PLONGLONG pllThreshold) PURE;

    STDMETHOD(GetDefaultQuotaThresholdText)(THIS_
        LPWSTR pszText,
        DWORD cchText) PURE;

    STDMETHOD(SetDefaultQuotaLimit)(THIS_
        LONGLONG llLimit) PURE;

    STDMETHOD(GetDefaultQuotaLimit)(THIS_
        PLONGLONG pllLimit) PURE;

    STDMETHOD(GetDefaultQuotaLimitText)(THIS_
        LPWSTR pszText,
        DWORD cchText) PURE;

    STDMETHOD(AddUserSid)(THIS_
        PSID pUserSid,
        DWORD fNameResolution,
        PDISKQUOTA_USER *ppUser) PURE;

    STDMETHOD(AddUserName)(THIS_
        LPCWSTR pszLogonName,
        DWORD fNameResolution,
        PDISKQUOTA_USER *ppUser) PURE;

    STDMETHOD(DeleteUser)(THIS_
        PDISKQUOTA_USER pUser) PURE;

    STDMETHOD(FindUserSid)(THIS_
        PSID pUserSid,
        DWORD fNameResolution,
        PDISKQUOTA_USER *ppUser) PURE;

    STDMETHOD(FindUserName)(THIS_
        LPCWSTR pszLogonName,
        PDISKQUOTA_USER *ppUser) PURE;

    STDMETHOD(CreateEnumUsers)(THIS_
        PSID *rgpUserSids,
        DWORD cpSids,
        DWORD fNameResolution,
        PENUM_DISKQUOTA_USERS *ppEnum) PURE;

    STDMETHOD(CreateUserBatch)(THIS_
        PDISKQUOTA_USER_BATCH *ppBatch) PURE;

    STDMETHOD(InvalidateSidNameCache)(THIS) PURE;

    STDMETHOD(GiveUserNameResolutionPriority)(THIS_
        PDISKQUOTA_USER pUser) PURE;

    STDMETHOD(ShutdownNameResolution)(THIS) PURE;
};

typedef IDiskQuotaControl DISKQUOTA_CONTROL, *PDISKQUOTA_CONTROL;



#undef  INTERFACE
#define INTERFACE IDiskQuotaEvents
DECLARE_INTERFACE_IID_(IDiskQuotaEvents, IUnknown, "7988B579-EC89-11cf-9C00-00AA00A14F56")
{
    STDMETHOD(OnUserNameChanged)(THIS_
        PDISKQUOTA_USER pUser) PURE;
};

typedef IDiskQuotaEvents DISKQUOTA_EVENTS, *PDISKQUOTA_EVENTS;




#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __DSKQUOTA_H

