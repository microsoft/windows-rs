/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    rtinfo.h

Abstract:
	Definitions of information block structuers used to exchange
	information in router API

--*/

#ifndef __ROUTING_RTINFO_H__
#define __ROUTING_RTINFO_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Information is passed to and from the Router Managers using a set of     //
// RTR_TOC_ENTRY structures. These structures are encapsulated by an        //
// RTR_INFO_BLOCK_HEADER.                                                   //
// The general structure of this is:                                        //
//                                                                          //
//           ---    |-----------------------|   ---                         //
//            |     |                       |    |                          //
//            |     | RTR_INFO_BLOCK_HEADER |    |                          //
//            |     |                       |    |                          //
//            |     | TocEntriesCount = N   |    |                          //
//            |     |-----------------------|    |                          //
//            |     |      TocEntry[0]      |    |                          //
//            |     |                       |    |                          //
//            |     |      Offset of        |    |                          //
//            |<------  Associated Data     |    |                          //
//            |     |                       |    |                          //
//            |     |-----------------------|    |                          //
//            |     Z                       Z    |                          //
//            |     |                       |    |                          //
//            |     |-----------------------|    |                          //
//            |     |      TocEntry[N-1]    |    |                          //
//            |     |                       |    |                          //
//            |     |      Offset of        |    |                          //
//            |     |   Associated Data  ------->|                          //
//            |     |                       |    |                          //
//           ---    |-----------------------|    |                          //
//                  |  Data for TocEntry[0] |    |                          //
//                  |-----------------------|    |                          //
//                  Z                       Z    |                          //
//                  |-----------------------|   ---                         //
//                  | Data for TocEntry[N-1]|                               //
//                  |-----------------------|                               //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Each of the blocks of data must begin at a quadword aligned boundary. To //
// get QUADWORD alignment, use the following macros.                        //
//                                                                          //
// The block of data pointed to by an InfoBlock MUST be aligned.            //
// Use the alignment macro when writing the data portion  into an infobase. //
// This implies that for each ALIGN_POINTER operation done on a chunk       //
// of memory, the requested allocation must be ALIGN_SIZE greater           //
// than what is actually required (to be on the safe side)                  //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#define ALIGN_SIZE      0x00000008
#define ALIGN_SHIFT     (ALIGN_SIZE - 0x00000001)       // 0x00000007
#define ALIGN_MASK_POINTER  (~(UINT_PTR)ALIGN_SHIFT)    // 0xfffffff8
#define ALIGN_MASK_LENGTH   (~ALIGN_SHIFT)              // 0xfffffff8
#define ALIGN_MASK          (~ALIGN_SHIFT)              // 0xfffffff8


#define ALIGN_POINTER(ptr) {                                    \
    (ptr) = (PVOID)((DWORD_PTR)(ptr) + ALIGN_SHIFT);            \
    (ptr) = (PVOID)((DWORD_PTR)(ptr) & ALIGN_MASK_POINTER);     \
}

#define ALIGN_LENGTH(length) {                                  \
    (length) = (DWORD)((length) + ALIGN_SHIFT);                 \
    (length) = (DWORD)((length) & ALIGN_MASK_LENGTH);           \
}


#define IS_ALIGNED(ptr)  (((UINT_PTR)(ptr) & ALIGN_SHIFT) == 0x00000000)

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
//   			            Table of Contents Entry 	                    //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Each entry describes a structure type, location within the information   //
// block and number of entries of the same type.                            //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

typedef struct _RTR_TOC_ENTRY
{
    ULONG	    InfoType;	// Info structure type
    ULONG	    InfoSize;	// Size of the info structure
    ULONG	    Count;		// How many info structures of this type
    ULONG	    Offset;		// Offset of the first structure, from the start
							// of the info block header.
}RTR_TOC_ENTRY, *PRTR_TOC_ENTRY;

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
//   			            Info Block Header        	                    //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// All Router information blocks start with this header                     //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#define RTR_INFO_BLOCK_VERSION	1

typedef struct _RTR_INFO_BLOCK_HEADER
{
    ULONG			Version;	    // Version of the structure
    ULONG			Size;		    // size of the whole block, including version
    ULONG			TocEntriesCount;// Number of entries
    _Field_size_(TocEntriesCount) RTR_TOC_ENTRY   TocEntry[1];    // Table of content followed by the actual
                                    // information blocks
} RTR_INFO_BLOCK_HEADER, *PRTR_INFO_BLOCK_HEADER;

//
// PVOID
// GetInfoFromTocEntry(
//                     IN PRTR_INFO_BLOCK_HEADER pInfoHdr,
//                     IN PRTR_TOC_ENTRY         pToc
//                     )
//

#define GetInfoFromTocEntry(hdr,toc)            \
    (((toc)->Offset < (hdr)->Size) ? ((PVOID)(((PBYTE)(hdr)) + (toc)->Offset)) : NULL)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //__ROUTING_RTINFO_H__
