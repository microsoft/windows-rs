/*++

Copyright (c) Microsoft Corporation.  All Rights Reserved

Abstract:

    This is the header file defining the data structures and APIs
    implemented by Windows Server Backup to provide 
    extensibility/integration for 3rd party online backup applications

--*/

#if (NTDDI_VERSION >= NTDDI_WIN8)

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define WSB_MAX_OB_STATUS_VALUE_TYPE_PAIR   (5)
#define WSB_MAX_OB_STATUS_ENTRY             (5)

//Enums and structures needed for UpdateOBStatusInWindowsServerBackup
typedef enum _WSB_OB_STATUS_ENTRY_PAIR_TYPE
{
    WSB_OB_ET_UNDEFINED      = 0,
    WSB_OB_ET_STRING         = 1,
    WSB_OB_ET_NUMBER         = 2,
    WSB_OB_ET_DATETIME       = 3,
    WSB_OB_ET_TIME           = 4,
    WSB_OB_ET_SIZE           = 5,
    WSB_OB_ET_MAX            = 6
} WSB_OB_STATUS_ENTRY_PAIR_TYPE;

typedef struct _WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR 
{
    LPWSTR                              m_wszObStatusEntryPairValue;
    WSB_OB_STATUS_ENTRY_PAIR_TYPE       m_ObStatusEntryPairType;
} WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR;

typedef struct _WSB_OB_STATUS_ENTRY 
{
    DWORD                   m_dwIcon;
    DWORD                   m_dwStatusEntryName;
    DWORD                   m_dwStatusEntryValue;
    ULONG                   m_cValueTypePair;
    WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR     *m_rgValueTypePair;
} WSB_OB_STATUS_ENTRY;

typedef struct _WSB_OB_STATUS_INFO 
{
    GUID                 m_guidSnapinId;
    DWORD                m_cStatusEntry;
    WSB_OB_STATUS_ENTRY *m_rgStatusEntry;
} WSB_OB_STATUS_INFO;

//Struct needed for RegisterOnlineBackupWithWindowsServerBackup
typedef struct _WSB_OB_REGISTRATION_INFO {
    LPWSTR                  m_wszResourceDLL;
    GUID                    m_guidSnapinId;
    DWORD                   m_dwProviderName;
    DWORD                   m_dwProviderIcon;
    BOOLEAN                 m_bSupportsRemoting;
} WSB_OB_REGISTRATION_INFO;

//APIs
HRESULT WINAPI RegisterOnlineBackupWithWindowsServerBackup(_In_ IN WSB_OB_REGISTRATION_INFO  *pOBRegistrationInfo);
HRESULT WINAPI DeregisterOnlineBackupFromWindowsServerBackup(_In_ IN GUID guidSnapinId);
HRESULT WINAPI UpdateOBStatusInWindowsServerBackup(_In_ IN WSB_OB_STATUS_INFO *pOBRegistrationInfo);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WIN8)


