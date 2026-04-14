#pragma once
/*
//-----------------------------------------------------------------------------
//
// File: msdrmgetinfo.h
//
// Copyright (C) 2001-2004 Microsoft Corporation.  All Rights Reserved.
//
//-----------------------------------------------------------------------------
*/

#ifndef __MSDRMGETINFO_H_
#define __MSDRMGETINFO_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "msdrm.h"

/*

The following codes are used to indicate where the various query strings may be used:

for example, GI(*) means that all DRMHANDLES may be asked the indicated question using DRMGetInfo &
             GI(hEnv) means on environment handle only

GI: DRMGetInfo
GULA: DRMGetUnboundLicenceAttribute
GULO: DRMGetUnboundLicenseObject
GBLA: DRMGetBoundLicenseAttribute
GBLO: DRMGetBoundLicenseObject

*/

const PWSTR     g_wszTAG_ASCII = (PWSTR)L"ASCII Tag",
                g_wszTAG_XRML = (PWSTR)L"XrML Tag",
                g_wszTAG_FILENAME = (PWSTR)L"filename",
                g_wszTAG_MSGUID = (PWSTR)L"MS-GUID",

                g_wszPLUG_STANDARDENABLINGPRINCIPAL = (PWSTR)L"UDStdPlg Enabling Principal",
                g_wszPLUG_STANDARDRIGHTSINTERPRETER = (PWSTR)L"XrMLv2a",
                g_wszPLUG_STANDARDEBDECRYPTOR = (PWSTR)L"UDStdPlg Enabling Bits Decryptor",
                g_wszPLUG_STANDARDEBENCRYPTOR = (PWSTR)L"UDStdPlg Enabling Bits Encryptor",
                g_wszPLUG_STANDARDEBCRYPTOPROVIDER = (PWSTR)L"UDStdPlg Enabling Bits Crypto Provider",
                g_wszPLUG_STANDARDLIBRARY = (PWSTR)L"UDStdPlg",

                g_wszALGORITHMID_DES = (PWSTR)L"DES",
                g_wszALGORITHMID_COCKTAIL = (PWSTR)L"COCKTAIL",
                g_wszALGORITHMID_AES = (PWSTR)L"AES",
                g_wszALGORITHMID_RC4 = (PWSTR)L"RC4",

                // QUERY CONSTANTS BELOW HERE ////////////////////////////////////////////////

                // GI(*)
                g_wszQUERY_OBJECTIDTYPE = (PWSTR)L"object-id-type",
                g_wszQUERY_OBJECTID = (PWSTR)L"object-id",

                // GBLA(on a bound right object), GULA(on a principal object, rights group, right, & work)
                g_wszQUERY_NAME = (PWSTR)L"name",

                // GBLA(on a bound license)
                g_wszQUERY_CONTENTIDTYPE = (PWSTR)L"content-id-type",
                g_wszQUERY_CONTENTIDVALUE = (PWSTR)L"content-id-value",
                g_wszQUERY_CONTENTSKUTYPE = (PWSTR)L"content-sku-type",
                g_wszQUERY_CONTENTSKUVALUE = (PWSTR)L"content-sku-value",

                // GI(hEnv)
                g_wszQUERY_MANIFESTSOURCE = (PWSTR)L"manifest-xrml",
                g_wszQUERY_MACHINECERTSOURCE = (PWSTR)L"machine-certificate-xrml",

                // GI(hEnv)
                g_wszQUERY_APIVERSION = (PWSTR)L"api-version",
                g_wszQUERY_SECREPVERSION = (PWSTR)L"secrep-version",

                // GI(hCrypto)
                g_wszQUERY_BLOCKSIZE  = (PWSTR)L"block-size",
                g_wszQUERY_SYMMETRICKEYTYPE  = (PWSTR)L"symmetric-key-type";

                // GULO(on a condition list), GBLO(on a bound right)
const PWSTR     g_wszQUERY_ACCESSCONDITION = (PWSTR)L"access-condition",

                // GULA(on a principal)
                g_wszQUERY_ADDRESSTYPE = (PWSTR)L"address-type",
                g_wszQUERY_ADDRESSVALUE = (PWSTR)L"address-value",

                g_wszQUERY_APPDATANAME = (PWSTR)L"appdata-name",
                g_wszQUERY_APPDATAVALUE = (PWSTR)L"appdata-value",

                // GULA(on a license, a work, and rights group, or a right)
                g_wszQUERY_CONDITIONLIST = (PWSTR)L"condition-list",

                // GULO(on a license or revocation condition)
                g_wszQUERY_DISTRIBUTIONPOINT = (PWSTR)L"distribution-point",

                g_wszQUERY_OBJECTTYPE = (PWSTR)L"object-type",

                // GBLA(on a bound license)
                g_wszQUERY_ENABLINGPRINCIPALIDTYPE = (PWSTR)L"enabling-principal-id-type",
                g_wszQUERY_ENABLINGPRINCIPALIDVALUE = (PWSTR)L"enabling-principal-id-value",

                // GULO(on a license)
                g_wszQUERY_GROUPIDENTITYPRINCIPAL = (PWSTR)L"group-identity-principal",

                // GULO(on an interval time condition)
                g_wszQUERY_FIRSTUSETAG = (PWSTR)L"first-use-tag",

                // GULA(on a range time condition)
                g_wszQUERY_FROMTIME = (PWSTR)L"from-time",

                // GULA(on a license, principal, or work)
                g_wszQUERY_IDTYPE = (PWSTR)L"id-type",
                g_wszQUERY_IDVALUE = (PWSTR)L"id-value",

                // GULO(on a license)
                g_wszQUERY_ISSUEDPRINCIPAL = (PWSTR)L"issued-principal",

                // GULA(on a license)
                g_wszQUERY_ISSUEDTIME = (PWSTR)L"issued-time",

                // GULO(on a license)
                g_wszQUERY_ISSUER = (PWSTR)L"issuer",

                // GULO(on a work)
                g_wszQUERY_OWNER = (PWSTR)L"owner",

                // GULO(on an access condition)
                g_wszQUERY_PRINCIPAL = (PWSTR)L"principal",

                // GI(hEnablingPrincipal)
                g_wszQUERY_PRINCIPALIDVALUE = (PWSTR)L"principal-id-value",
                g_wszQUERY_PRINCIPALIDTYPE = (PWSTR)L"principal-id-type",

                // GULO(on a condition list)
                g_wszQUERY_RANGETIMECONDITION = (PWSTR)L"rangetime-condition",
                g_wszQUERY_OSEXCLUSIONCONDITION = (PWSTR)L"os-exclusion-condition",

                // GULA
                g_wszQUERY_INTERVALTIMECONDITION = (PWSTR)L"intervaltime-condition",
                g_wszQUERY_INTERVALTIMEINTERVAL = (PWSTR)L"intervaltime-interval",
                g_wszQUERY_MAXVERSION = (PWSTR)L"max-version",
                g_wszQUERY_MINVERSION = (PWSTR)L"min-version",

                // GULA(on a revocation condition)
                g_wszQUERY_REFRESHPERIOD = (PWSTR)L"refresh-period",

                // GULO(on a condition list)
                g_wszQUERY_REVOCATIONCONDITION = (PWSTR)L"revocation-condition",

                // GULO(on a rights group), GBLO(on a bound license)
                g_wszQUERY_RIGHT = (PWSTR)L"right",

                // GULO(on a work)
                g_wszQUERY_RIGHTSGROUP = (PWSTR)L"rights-group",

                // GULA(on a right), GBLA(on a bound right)
                g_wszQUERY_RIGHTSPARAMETERNAME = (PWSTR)L"rights-parameter-name",
                g_wszQUERY_RIGHTSPARAMETERVALUE = (PWSTR)L"rights-parameter-value",

                // GULA(on a work)
                g_wszQUERY_SKUTYPE = (PWSTR)L"sku-type",
                g_wszQUERY_SKUVALUE = (PWSTR)L"sku-value",

                // GULA(on an interval time or metered time condition)
                g_wszQUERY_TIMEINTERVAL = (PWSTR)L"time-interval",

                // GULA(on a range time condition)
                g_wszQUERY_UNTILTIME = (PWSTR)L"until-time",

                // GULA(on a license)
                g_wszQUERY_VALIDITYFROMTIME = (PWSTR)L"valid-from",
                g_wszQUERY_VALIDITYUNTILTIME = (PWSTR)L"valid-until",

                // GULO(on a license)
                g_wszQUERY_WORK = (PWSTR)L"work";


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __MSDRMGETINFO_H_

