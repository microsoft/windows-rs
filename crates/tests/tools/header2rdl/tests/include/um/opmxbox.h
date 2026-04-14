//
//---------------------------------------------------------------------------
//
//  Copyright (c) 2014  Microsoft Corporation
//
//  Abstract:
//
//    Simple OPM API for XBOX
//
//---------------------------------------------------------------------------


#pragma once

#ifdef __cplusplus 
 extern "C" {
#endif

//
// Supported HDCP version for the system
// OPM_HDCP_TYPE_0  --> all downsteam tree supports all version fo HDCP including v1.4
// OPM_HDCP_TYPE_1  --> all downsteam tree must be HDCP 2.2
//
typedef enum OPM_HDCP_TYPE
{
    OPM_HDCP_TYPE_0 = 0,
    OPM_HDCP_TYPE_1,
} OPM_HDCP_TYPE;

//
// HDCP status, on or off
//
typedef enum OPM_HDCP_STATUS
{
    OPM_HDCP_STATUS_ON = 0,
    OPM_HDCP_STATUS_OFF,    
} OPM_HDCP_STATUS;


//
// Request to Enable particular HDCP version
//
HRESULT OPMXboxEnableHDCP(
    OPM_HDCP_TYPE HDCPType);

//
// Retrieve HDCP on/off status
//
HRESULT OPMXboxGetHDCPStatus(OPM_HDCP_STATUS *pHDCPStatus);

//
// Retrieve HDCP on/off status and type
//
HRESULT OPMXboxGetHDCPStatusAndType(OPM_HDCP_STATUS *pHDCPStatus, OPM_HDCP_TYPE *pHDCPType);

#ifdef __cplusplus 
}
#endif

