////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation
//
// SYNOPSIS
//
//   Declares the MS IDs for the NAP system.
//
////////////////////////////////////////////////////////////

#ifndef NAPMICROSOFTVENDORIDS_H
#define NAPMICROSOFTVENDORIDS_H

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern "C"
{
   static UINT32 MicrosoftVendorId = 0x137;

   // These are the IDs used to identify internal Microsoft
   // enforcers and system health components.
   // The 4-byte values are encoded as follows - 
   //    first 3 bytes are the IETF-supplied SMI ID.
   //    last byte is used to identify the product.
   // 

   // The NAP system use the following ID.
   static UINT32 NapSystemId = 0x00013700;


   // Microsoft Enforcers use the following IDs.

   static UINT32 NapDhcpEnforcementId           = 0x00013701;
   static UINT32 NapRasEnforcementId            = 0x00013702;
   static UINT32 NapIpsecEnforcementId          = 0x00013703;
   static UINT32 Nap8021xEnforcementId          = 0x00013704;
   static UINT32 NapAnywhereAccessEnforcementId = 0x00013705;
   static UINT32 NapIsaEnforcementId            = 0x00013706;
   static UINT32 NapEapEnforcementId            = 0x00013707;


   // Microsoft System Health Components use the following IDs.
   
   static UINT32 NapOutOfBoxSystemHealthId      = 0x00013780;
   static UINT32 NapSmsSystemHealthId           = 0x00013781;
   static UINT32 NapFCSv1SystemHealthId         = 0x00013782;
   static UINT32 NapFCSv2SystemHealthId         = 0x00013783;
   static UINT32 NapTpmSystemHealthId           = 0x00013784; 
}


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // NAPMICROSOFTVENDORIDS_H
