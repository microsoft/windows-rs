

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */


#ifndef __mddefw_h__
#define __mddefw_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mddefw_0000_0000 */
/* [local] */ 

/*++
                                                                                
Copyright (c) 1997-1999 Microsoft Corporation
                                                                                
Module Name: mddef.h
                                                                                
    Definitions for Admin Objects and Metadata
                                                                                
--*/
#ifndef _MD_DEFW_
#define _MD_DEFW_
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <mdmsg.h>
#include <mdcommsg.h>
/*                                                                              
    Error Codes                                                                 
                                                                                
        Metadata api's all return HRESULTS. Since internal results are either   
        winerrors or Metadata specific return codes (see mdmsg.h), they are     
        converted to HRESULTS using the RETURNCODETOHRESULT macro (see          
        commsg.h).                                                              
*/                                                                              
                                                                                
/*                                                                              
    Max Name Length                                                             
        The maximum number of characters in the length of a metaobject name,    
        including the terminating NULL. This refers to each node in the tree,   
        not the entire path.                                                    
        eg. strlen("Root") < METADATA_MAX_NAME_LEN                            
*/                                                                              
#define METADATA_MAX_NAME_LEN           256
/*                                                                              
   Access Permissons                                                            
       Permissions associated with handles of type METADATA_HANDLE              
                                                                                
       METADATA_PERMISSION_READ - Allows reading metadata.                      
       METADATA_PERMISSION_WRITE - Allows writing metadata.                     
*/                                                                              
#define METADATA_PERMISSION_READ        0x00000001
#define METADATA_PERMISSION_WRITE       0x00000002
/*                                                                              
    Data Types                                                                  
                                                                                
        ALL_METADATA - Used on Get/Enum/Getall api's (ComMDGetMetaData,         
            ComMDEnumMetaData, and ComMDGetAllMetaData), api's to allow getting 
            all data. Not valid on the Set api.                                 
                                                                                
        DWORD_METADATA - The data is an unsigned 32 bit value.                  
                                                                                
        STRING_METADATA - The data is a null terminated ASCII string.           
                                                                                
        BINARY_METADATA - The data is any binary value.                         
                                                                                
        EXPANDSZ_METADATA - The data is a null terminated ASCII string.         
            Clients are expected to treat this as an expandsz string.           
                                                                                
        MULTISZ_METADATA - A series of NULL terminated ASCII strings. ending    
            with 2 NULL's.                                                      
*/                                                                              

enum METADATATYPES
    {
        ALL_METADATA	= 0,
        DWORD_METADATA	= ( ALL_METADATA + 1 ) ,
        STRING_METADATA	= ( DWORD_METADATA + 1 ) ,
        BINARY_METADATA	= ( STRING_METADATA + 1 ) ,
        EXPANDSZ_METADATA	= ( BINARY_METADATA + 1 ) ,
        MULTISZ_METADATA	= ( EXPANDSZ_METADATA + 1 ) ,
        INVALID_END_METADATA	= ( MULTISZ_METADATA + 1 ) 
    } ;
/*                                                                              
    Attributes - The flags for the data.                                        
                                                                                
        METADATA_INHERIT - If set for a data item, the data item can be         
            inherited. If set on input to the Get/Enum/Getall api's,            
            (ComMDGetMetaData, ComMDEnumMetaData, and ComMDGetAllMetaData),     
            inheritable data will be returned. If not set on input to the       
            Get/Enum/Getall, inheritable data will not be returned.             
                                                                                
        METADATA_PARTIAL_PATH - If set on input to Get/Enum/Getall api's, this  
            routine will return ERROR_SUCCESS and the inherited data even if    
            the entire path is not present. Only valid if METADATA_INHERIT is   
            also set. Should not be set for data items on input the the Set api 
            (ComMDSetMetaData).                                                 
                                                                                
        METADATA_SECURE - If set for a data item, the data is stored in a       
            secure fasion. Should not be set on input to Get/Enum api's.        
                                                                                
        METADATA_REFERENCE - If set for a data item, the data item may be       
            gotten by reference. If set on input to Get/Enum/GetAll api's and   
            set on a the data item being returned, the data is returned by      
            reference. A pointer to the metadata server's copy of the data is   
            placed in the Data field of the METADATA_RECORD or                  
            METADATA_GETALL_RECORD, and the DataTag field is set. This data must
            be freed by calling ComMDReleaseReferenceData. The client must not  
            change this data. This flag must not be set on input to             
            Get/Enum/Getall api's from remote clients.                          
                                                                                
        METADATA_VOLATILE - If set for a data item, the data item will not be   
            saved to long term storage.                                         
                                                                                
        METADATA_ISINHERITED - If specified on input to one of the get api's,   
        the flag will be set on return if the data was inherited.               
                                                                                
        METADATA_INSERT_PATH - If specified on input to one of the get api's,   
        and on a string data item, the path relative to handle will replaced the
        string MD_INSERT_PATH_STRING in the string data.                        
                                                                                
        METADATA_LOCAL_MACHINE_ONLY - If set for a data item, the data item
        will not be replicated during web cluster replication.
                                                                                
        METADATA_NON_SECURE_ONLY - When using GetAll api, do not
        retrieve properties which are secure.
                                                                                
*/                                                                              
                                                                                
#define METADATA_NO_ATTRIBUTES          0                                       
#define METADATA_INHERIT                0x00000001                              
#define METADATA_PARTIAL_PATH           0x00000002                              
#define METADATA_SECURE                 0x00000004                              
#define METADATA_REFERENCE              0x00000008                              
#define METADATA_VOLATILE               0x00000010                              
#define METADATA_ISINHERITED            0x00000020                              
#define METADATA_INSERT_PATH            0x00000040                              
#define METADATA_LOCAL_MACHINE_ONLY     0x00000080                              
#define METADATA_NON_SECURE_ONLY        0x00000100                              
#define METADATA_DONT_EXPAND            0x00000200                              
                                                                                
/*                                                                              
    Backup Flags.                                                               
                                                                                
        MD_BACKUP_OVERWRITE - If set, the metabase will be backed up even if    
            a backupe with the same name and version already exists. The        
            existing backup will be overwritten.                                
                                                                                
        MD_BACKUP_SAVE_FIRST - If set backup will save the metabase prior to    
            making the backup. If the save fails, backup behavior is dependent  
            on the value of MD_BACKUP_FORCE_BACKUP.                             
                                                                                
        MD_BACKUP_FORCE_BACKUP - If set backup will proceed even if the save    
            failed. Only valid if MD_BACKUP_SAVE_FIRST is set. If the save      
            but the backup succeeds, a warning will be returned.                
*/                                                                              
                                                                                
#define MD_BACKUP_OVERWRITE             0x00000001                              
#define MD_BACKUP_SAVE_FIRST            0x00000002                              
#define MD_BACKUP_FORCE_BACKUP          0x00000004                              
/*                                                                              
    Backup Version Defines.                                                     
                                                                                
        MD_BACKUP_NEXT_VERSION - For Backup, indicates use the next available   
            backup version of the BackupLocation specified, ie. one higher than 
            the highest existing version.                                       
            Not valid for Restore or DeleteBackup.                              
                                                                                
        MD_BACKUP_HIGHEST_VERSION - For Backup, Restore, and DeleteBackup, will 
            use the highest existing backup version of the BackupLocation       
            specified.                                                          
                                                                                
        MD_BACKUP_MAX_VERSION - The highest allowed backup version number.      
                                                                                
        MD_BACKUP_MAX_LEN - The maximup length, in UNICODE characters, of the   
            BackupLocation.                                                     
*/                                                                              
                                                                                
#define MD_BACKUP_NEXT_VERSION          0xffffffff                              
#define MD_BACKUP_HIGHEST_VERSION       0xfffffffe                              
#define MD_BACKUP_MAX_VERSION           9999                                    
#define MD_BACKUP_MAX_LEN               (100)                                   
                                                                                
/*                                                                              
    Backup Location Defines.                                                    
                                                                                
        MD_DEFAULT_BACKUP_LOCATION - The default location to backup from or     
            restore to if no location is specified.                             
*/                                                                              
                                                                                
#define MD_DEFAULT_BACKUP_LOCATION TEXT("MDBackUp")                           
                                                                                
/*                                                                              
    History Flags.                                                              
                                                                                
        MD_HISTORY_LATEST - The most recent history file by timestamp           
*/                                                                              
                                                                                
#define MD_HISTORY_LATEST               0x00000001                              
                                                                                
/*                                                                              
    Export Flags.                                                               
                                                                                
        MD_EXPORT_INHERITED - If set, inherited properties will be backed up    
            to special section in output file called IIsInheritedProperties.    
            If not set, inherited properties are ignored.                       
                                                                                
        MD_EXPORT_NODE_ONLY - If set, children will not be exported.            
            If not set, children will be exported.                              
*/                                                                              
                                                                                
#define MD_EXPORT_INHERITED             0x00000001                              
#define MD_EXPORT_NODE_ONLY             0x00000002                              
                                                                                
/*                                                                              
    Import Flags.                                                               
                                                                                
        MD_IMPORT_INHERITED - If set, inherited properties will be imported.    
                                                                                
        MD_IMPORT_NODE_ONLY - If set, children will not be imported.            
            If not set, children will be imported.                              
                                                                                
        MD_IMPORT_MERGE - If set, imported settings overwrite existing          
            settings, but entire node is not overwritten.  If not set, entire   
            node is clobbered.                                                  
*/                                                                              
                                                                                
#define MD_IMPORT_INHERITED             0x00000001                              
#define MD_IMPORT_NODE_ONLY             0x00000002                              
#define MD_IMPORT_MERGE                 0x00000004                              
                                                                                
/*                                                                              
    Insert Path Defines.                                                        
*/                                                                              
                                                                                
                                                                                
#define MD_INSERT_PATH_STRINGA      "<%INSERT_PATH%>"                         
#define MD_INSERT_PATH_STRINGW      L##"<%INSERT_PATH%>"                      
#define MD_INSERT_PATH_STRING       TEXT("<%INSERT_PATH%>")                   
                                                                                
/*                                                                              
    Handle Defines.                                                             
*/                                                                              
                                                                                
#define METADATA_MASTER_ROOT_HANDLE     0
                                                                                
/*                                                                              
    METADATA_RECORD is the basic input/output parameter for the set and get     
        metadata api's. The Set api (ComMDSetMetaData) takes all fields as      
        input, except DataTag. The Get/Enum api's (ComMDGetMetadata and         
        ComMDEnumMetaData) take some of the fields as input, and fill in all    
        fields as output.                                                       
                                                                                
    Identifier - The identifier of the data.                                    
                                                                                
    Attributes - The flags for the data.                                        
                                                                                
    UserType - The User Type for the data. This is a user defined field to allow
        users to group data. If set to anything other than ALL_METADATA on input
        to Get/Set apis, only metadata of the specified User Type will be       
        returned.                                                               
                                                                                
        ALL_METADATA                                                            
                                                                                
        User Defined Values                                                     
                                                                                
    DataType - The Type of the data. Must be set to a valid value other than    
        ALL_METADATA for each data item. If set to anything other than          
        ALL_METADATA on input to Get/Set api's, only metadata of the            
        specified Data Type will be returned.                                   
             ALL_METADATA                                                       
             DWORD_METADATA                                                     
             STRING_METADATA                                                    
             BINARY_METADATA                                                    
             EXPANDSZ_METADATA                                                  
                                                                                
    DataLen - On input to the Set api, specifies the length of Data, in         
        bytes. Inprocess clients only need to specify this for binary data.     
        Remote clients need to specify this for all data types. For strings,    
        this must include the trailing '\0', eg. strlen(string) + 1.            
        On input to Get/Enum apis, specifies the size of the buffer pointed to  
        by Data. On successful output from Get/Enum API's, specifies the size of
        Data in bytes.                                                          
                                                                                
    Data - On input to the Set api, points to the data. On input to the         
        Get/Enum api's, points to a buffer to return the data in. On output     
        from the Get/Enum api's, points to the data. If the data is not         
        gotten by reference, the  pointer will be unchanged.                    
                                                                                
    DataTag - A tag for reference data. Not used in the Set api. Not used on    
        input to the Get/Enum api's. On successful return from the Get/Enum     
        api's, this is set to a nonzero tag if the data was gotten by reference,
        and set to 0 if the data was not gotten by reference.                   
*/                                                                              
typedef struct _METADATA_RECORD
    {
    DWORD dwMDIdentifier;
    DWORD dwMDAttributes;
    DWORD dwMDUserType;
    DWORD dwMDDataType;
    DWORD dwMDDataLen;
    /* [size_is][unique] */ unsigned char *pbMDData;
    DWORD dwMDDataTag;
    } 	METADATA_RECORD;

typedef struct _METADATA_RECORD *PMETADATA_RECORD;

/*                                                                              
    METADATA_GETALL_RECORD, is essentially the same as METADATA_RECORD, but is  
        used by MDGetAllMetaData. It is used the same as the corresponding      
        METADATA_RECORD values for the MDGetMetaData, with the following        
        exceptions:                                                             
                                                                                
    MDGetAllMetadata does not take the structure as input, but takes parameters 
        equivalent to Attributes, UserType, and DataType.                       
                                                                                
    On output, MDGetAllMetadata returns an array of METADATA_GETALL_RECORD.     
                                                                                
    DataOffset/Data - If the data is not returned by reference, DataOffset      
        contains the byte offset into the buffer provided. If the data is       
        returned by reference, Data contains a pointer to the data.             
                                                                                
                                                                                
    Because an opaque pointer to an array of _METADATA_GETALL_RECORD's is       
    passed on calls to GetAllData, its size must be the same on x86 and ia64.   
    So, the pbMDData member (not used by the public interface) has been         
    removed and a new structure _METADATA_GETALL_INTERNAL_RECORD has been       
    created for use by the callees of the internal interface                    
*/                                                                              
typedef struct _METADATA_GETALL_RECORD
    {
    DWORD dwMDIdentifier;
    DWORD dwMDAttributes;
    DWORD dwMDUserType;
    DWORD dwMDDataType;
    DWORD dwMDDataLen;
    DWORD dwMDDataOffset;
    DWORD dwMDDataTag;
    } 	METADATA_GETALL_RECORD;

typedef struct _METADATA_GETALL_RECORD *PMETADATA_GETALL_RECORD;

typedef struct _METADATA_GETALL_INTERNAL_RECORD
    {
    DWORD dwMDIdentifier;
    DWORD dwMDAttributes;
    DWORD dwMDUserType;
    DWORD dwMDDataType;
    DWORD dwMDDataLen;
    union 
        {
        DWORD_PTR dwMDDataOffset;
        unsigned char *pbMDData;
        } 	;
    DWORD dwMDDataTag;
    } 	METADATA_GETALL_INTERNAL_RECORD;

typedef struct _METADATA_GETALL_INTERNAL_RECORD *PMETADATA_GETALL_INTERNAL_RECORD;

typedef DWORD METADATA_HANDLE;

typedef DWORD *PMETADATA_HANDLE;

/*                                                                              
Handle Information                                                              
                                                                                
    Permissions - The permissions associated with the handle.                   
        METADATA_PERMISSION_READ                                                
        METADATA_PERMISSION_WRITE                                               
                                                                                
    SystemChangeNumber - The system change number at the time the handle was    
        allocated.                                                              
*/                                                                              
typedef struct _METADATA_HANDLE_INFO
    {
    DWORD dwMDPermissions;
    DWORD dwMDSystemChangeNumber;
    } 	METADATA_HANDLE_INFO;

typedef struct _METADATA_HANDLE_INFO *PMETADATA_HANDLE_INFO;

/*                                                                              
    Change Object - The structure passed to ComMDSinkNotify.                    
                                                                                
        Path - The path of the MetaObject modified.                             
                                                                                
        ChangeType - The types of changes made, from the flags below.           
                                                                                
        NumDataIDs - The number of data id's changed.                           
                                                                                
        DataIDs - An array of the data id's changed.                            
*/                                                                              
                                                                                
#define MD_CHANGE_OBJECT     MD_CHANGE_OBJECT_W                                 
#define PMD_CHANGE_OBJECT    PMD_CHANGE_OBJECT_W                                
typedef struct _MD_CHANGE_OBJECT_W
    {
    /* [string] */ LPWSTR pszMDPath;
    DWORD dwMDChangeType;
    DWORD dwMDNumDataIDs;
    /* [size_is][unique] */ DWORD *pdwMDDataIDs;
    } 	MD_CHANGE_OBJECT_W;

typedef struct _MD_CHANGE_OBJECT_W *PMD_CHANGE_OBJECT_W;

/*                                                                              
                                                                                
Change Types                                                                    
                                                                                
    MD_CHANGE_TYPE_DELETE_OBJECT - The Meta Object was deleted.                 
                                                                                
    MD_CHANGE_TYPE_ADD_OBJECT - The Meta Object was added.                      
                                                                                
    MD_CHANGE_TYPE_SET_DATA - A data item was set.                              
                                                                                
    MD_CHANGE_TYPE_DELETE_DATA - A data item was deleted.                       
                                                                                
    MD_CHANGE_TYPE_RENAME_OBJECT - The Meta Object was renamed.                 
                                                                                
    MD_CHANGE_TYPE_RESTORE - The Meta Base was restored.                 
                                                                                
*/                                                                              
#define MD_CHANGE_TYPE_DELETE_OBJECT   0x00000001
#define MD_CHANGE_TYPE_ADD_OBJECT      0x00000002
#define MD_CHANGE_TYPE_SET_DATA        0x00000004
#define MD_CHANGE_TYPE_DELETE_DATA     0x00000008
#define MD_CHANGE_TYPE_RENAME_OBJECT   0x00000010
#define MD_CHANGE_TYPE_RESTORE         0x00000020
/*                                                                              
                                                                                
Max Change Entries - The maximum number of change entries that will be sent on  
    a single call to IMDCOMSINK::ComMDSinkNotify. If more notifications are     
    required, IMDCOMSINK::ComMDSinkNotify will be called multiple times.        
*/                                                                              
#define MD_MAX_CHANGE_ENTRIES          100
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif


extern RPC_IF_HANDLE __MIDL_itf_mddefw_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mddefw_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


