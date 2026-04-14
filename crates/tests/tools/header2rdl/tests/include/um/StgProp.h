//+---------------------------------------------------------------------------
//
//  Copyright (C) Microsoft Corporation, 1993-1999.
//
//  File:       stgprop.h
//
//  Contents:   Standard storage provider property definitions;
//
//  History:    Jul-9-93       robertfe
//
//----------------------------------------------------------------------------

#ifndef _STGPROP_H_
#define _STGPROP_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define PSGUID_STORAGE  { 0xb725f130,           \
                          0x47ef, 0x101a,       \
                          { 0xa5, 0xf1, 0x02, 0x60, 0x8c, 0x9e, 0xeb, 0xac } }

//#define PID_STG_DICTIONARY            ((PROPID) 0x00000000) //reserved
//#define PID_STG_CODEPAGE              ((PROPID) 0x00000001) //reserved

#define PID_STG_DIRECTORY               ((PROPID) 0x00000002)

#define PID_STG_CLASSID                 ((PROPID) 0x00000003)
#define PID_STG_STORAGETYPE             ((PROPID) 0x00000004)

#define PID_STG_VOLUME_ID               ((PROPID) 0x00000005)
#define PID_STG_PARENT_WORKID           ((PROPID) 0x00000006)
#define PID_STG_SECONDARYSTORE          ((PROPID) 0x00000007)

#define PID_STG_FILEINDEX               ((PROPID) 0x00000008)
#define PID_STG_LASTCHANGEUSN           ((PROPID) 0x00000009)
#define PID_STG_NAME                    ((PROPID) 0x0000000a)
#define PID_STG_PATH                    ((PROPID) 0x0000000b)

#define PID_STG_SIZE                    ((PROPID) 0x0000000c)
#define PID_STG_ATTRIBUTES              ((PROPID) 0x0000000d)
#define PID_STG_WRITETIME               ((PROPID) 0x0000000e)
#define PID_STG_CREATETIME              ((PROPID) 0x0000000f)
#define PID_STG_ACCESSTIME              ((PROPID) 0x00000010)
#define PID_STG_CHANGETIME              ((PROPID) 0x00000011)
#define PID_STG_ALLOCSIZE               ((PROPID) 0x00000012)

#define PID_STG_CONTENTS                ((PROPID) 0x00000013)
#define PID_STG_SHORTNAME               ((PROPID) 0x00000014)

#define PID_STG_FRN                     ((PROPID) 0x00000015)
#define PID_STG_SCOPE                   ((PROPID) 0x00000016)

#define PID_STG_MAX                     PID_STG_SCOPE

#define CSTORAGEPROPERTY                0x17


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _STGPROP_H_
