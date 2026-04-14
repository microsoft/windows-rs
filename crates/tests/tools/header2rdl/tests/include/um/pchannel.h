/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    pchannel.h

Abstract:

    Windows Terminal Server Virtual Channel protocol header

Revision History:

--*/

#ifndef _H_PCHANNEL
#define _H_PCHANNEL

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_XP)

/****************************************************************************/
// If PAL_XXX macro's aren't defined, define it just for this file
// We do this because we don't want to add TsSsytemPal.h as a dependency
// for all parts of termsrv that includes this file
/****************************************************************************/
#ifndef PAL_PACKED_STRUCT

#ifdef ENABLE_UCORE
#error PAL_PACKED_STRUCT is not defined
#endif // ENABLE_UCORE

#define __PCHANNEL_H_PAL_PACKED_STRUCT_DEFINED
#define     PAL_PACKED_STRUCT(type)     type

#endif // PAL_PACKED_STRUCT

/****************************************************************************/
/* Maximum amount of data that is sent in one operation.  Data larger than  */
/* this is segmented into chunks of this size and sent as multiple          */
/* operations.                                                              */
/****************************************************************************/
#define CHANNEL_CHUNK_LENGTH    1600
#define CHANNEL_BUFFER_SIZE     65535

#define CHANNEL_PDU_LENGTH (CHANNEL_CHUNK_LENGTH + sizeof(CHANNEL_PDU_HEADER))

/****************************************************************************/
/* Header flags (also passed to VirtualChannelOpenEventFn)                  */
/****************************************************************************/
#define CHANNEL_FLAG_FIRST      0x01
#define CHANNEL_FLAG_LAST       0x02
#define CHANNEL_FLAG_ONLY       (CHANNEL_FLAG_FIRST | CHANNEL_FLAG_LAST)
#define CHANNEL_FLAG_MIDDLE     0

#define CHANNEL_FLAG_FAIL       0x100

/****************************************************************************/
/* Virtual Channel options, passed by Client on VirtualChannelOpen          */
/****************************************************************************/

/****************************************************************************/
/* Application is initialized.  If this flag is not set, a virtual channel  */
/* is not established for this application                                  */
/****************************************************************************/
#define CHANNEL_OPTION_INITIALIZED  0x80000000

/****************************************************************************/
/* Encrypt according to RDP data encryption (ie if RDP data is encrypted,   */
/* do so for this virtual channel too)                                      */
/****************************************************************************/
#define CHANNEL_OPTION_ENCRYPT_RDP  0x40000000

/****************************************************************************/
/* Encrypt Server to Client data (ignored if CHANNEL_OPTION_ENCRYPT_RDP is  */
/* set)                                                                     */
/****************************************************************************/
#define CHANNEL_OPTION_ENCRYPT_SC   0x20000000

/****************************************************************************/
/* Encrypt Client to Server data (ignored if CHANNEL_OPTION_ENCRYPT_RDP is  */
/* set)                                                                     */
/****************************************************************************/
#define CHANNEL_OPTION_ENCRYPT_CS   0x10000000

/****************************************************************************/
/* Send data at high priority (not recommended, as this may impact RDP      */
/* performance)                                                             */
/****************************************************************************/
#define CHANNEL_OPTION_PRI_HIGH     0x08000000

/****************************************************************************/
/* Send data at medium priority                                             */
/****************************************************************************/
#define CHANNEL_OPTION_PRI_MED      0x04000000

/****************************************************************************/
/* Send data at low priority                                                */
/****************************************************************************/
#define CHANNEL_OPTION_PRI_LOW      0x02000000

/****************************************************************************/
/* Compress data in this virtual channel if RDP data compression is         */
/* configured for this connection                                           */
/****************************************************************************/
#define CHANNEL_OPTION_COMPRESS_RDP 0x00800000

/****************************************************************************/
/* Compress data in this virtual channel, irrespective of RDP data          */
/* compression (ignored if CHANNEL_OPTION_COMPRESS_RDP is set)              */
/****************************************************************************/
#define CHANNEL_OPTION_COMPRESS     0x00400000

/****************************************************************************/
/* Show Server addins the full Virtual Channel protocol.  This option       */
/* affects how data passed to VirtualChannelWrite is presented to Server    */
/* addins.                                                                  */
/*                                                                          */
/* - If this option is set, Server addins see the full Virtual Channel      */
/* protocol, including CHANNEL_PDU_HEADER (below).                          */
/*                                                                          */
/* -If this option is not set, Server addins see just the data passed to    */
/* VirtualChannelWrite                                                      */
/****************************************************************************/
#define CHANNEL_OPTION_SHOW_PROTOCOL 0x00200000

/****************************************************************************/
/* Specify that this channel is persistent accross remote control           */
/* transitions.                                                             */
/****************************************************************************/
#define CHANNEL_OPTION_REMOTE_CONTROL_PERSISTENT 0x00100000

/****************************************************************************/
/* Maximum number and size of channel names                                 */
/****************************************************************************/
#define CHANNEL_MAX_COUNT           30
#define CHANNEL_NAME_LEN             7

/****************************************************************************/
/* Structure: CHANNEL_DEF                                                   */
/*                                                                          */
/* Description: Client to Server virtual channel information                */
/* - name       channel name                                                */
/* - options    channel options (a combination of the CHANNEL_OPTION        */
/*              constants above)                                            */
/****************************************************************************/
// Turn off packing for the CHANNEL_DEF structure
#ifdef PAL_PRAGMA_PACK_PUSH
#pragma PAL_PRAGMA_PACK_PUSH(pchannel_channel_def, 1)
#else
#pragma pack(push, pchannel_channel_def, 1)
#endif

typedef struct tagCHANNEL_DEF
{
    char            name[CHANNEL_NAME_LEN + 1];
    ULONG           options;
} PAL_PACKED_STRUCT(CHANNEL_DEF);
typedef CHANNEL_DEF UNALIGNED FAR *PCHANNEL_DEF;
typedef PCHANNEL_DEF UNALIGNED FAR *PPCHANNEL_DEF;

// Restore original packing
#ifdef PAL_PRAGMA_PACK_POP
#pragma PAL_PRAGMA_PACK_POP(pchannel_channel_def)
#else
#pragma pack(pop, pchannel_channel_def)
#endif

/****************************************************************************/
/* Header of Virtual Channel PDUs                                           */
/****************************************************************************/
/****************************************************************************/
/* Structure: CHANNEL_PDU_HEADER                                            */
/*                                                                          */
/* Description: Header sent on Virtual Channel PDUs                         */
/****************************************************************************/
typedef struct tagCHANNEL_PDU_HEADER
{
    UINT32    length;                 /* Length of data excluding header    */
    UINT32    flags;                  /* CHANNEL_FLAG_xxx flags             */
} CHANNEL_PDU_HEADER, FAR * PCHANNEL_PDU_HEADER;
/****************************************************************************/

#ifdef __PCHANNEL_H_PAL_PACKED_STRUCT_DEFINED
#undef __PCHANNEL_H_PAL_PACKED_STRUCT_DEFINED
#undef PAL_PACKED_STRUCT
#endif // __PCHANNEL_H_PAL_PACKED_STRUCT_DEFINED

#endif /* (NTDDI_VERSION >= NTDDI_XP) */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _H_PCHANNEL */
