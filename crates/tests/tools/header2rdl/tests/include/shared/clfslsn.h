#include <winapifamily.h>

/*=============================================================================

    Copyright (c) 1998  Microsoft Corporation

    Module Name:

        clfslsn.h

    Abstract:

        Header file containing the private definition for the common log
        file system's log sequence number structure.

    Author:

        Dexter Bradshaw    [DexterB]   09-Dec-1998


    Revision History:

=============================================================================*/

#ifndef _CLFS_LSN_H_
#define _CLFS_LSN_H_

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_RECORD_INDEX
//
// Log record offset on container file.  The log record offset consists of a block
// offset in the container and a bucket identifier indexing the records in the block.
// Declared up here because including clfs_x.h will try to define the LSN, which needs
// this.
//

typedef UINT32                      CLFS_RECORD_INDEX;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLS_LSN
//
// The log sequence number (LSN) is a valid log file address.  The LSN consists of
// three (3) parts: (a) a log identifier to identify which physical log the log record
// belongs to, (b) a container index identifying the log container where the log record
// lies, and (c) a record offset identified by the offset of the block in the container
// and an ordinal number for the record within the container.
//
//
// The structure of the LSN poses some inherent limitations of the number of logs,
// the number of containers, the size of a container, and the number of log records in
// a log block.
//
//          Maximum number of physical log files is 64K.
//          Maximum number of container identifiers is 4G.
//          Maximum size of a container is 4G.
//          Maximum number of sector-aligned log blocks is 8M
//          Maximum number of record buckets in a log block is 512
//

typedef union _CLS_LSN
{
    //
    // Container identifier
    //

    struct
    {
       CLFS_RECORD_INDEX   idxRecord;      // Record offset on container.
       CLFS_CONTAINER_ID   cidContainer;   // Container identifier.
    } offset;
    
    __volatile ULONGLONG               ullOffset;      // Sequence number within physical log.

} CLS_LSN, *PCLS_LSN, **PPCLS_LSN;

//
// Alias CLS prefixed types with CLFS prefixes.
//

typedef CLS_LSN CLFS_LSN;
typedef CLFS_LSN *PCLFS_LSN, **PPCLFS_LSN;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
