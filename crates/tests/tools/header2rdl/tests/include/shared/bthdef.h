/*++

Copyright (c) 2000  Microsoft Corporation

Module Name:

   bthdef.h

Abstract:

    This module contains the Bluetooth common structures and definitions

Author:

Notes:

Environment:

    Kernel mode only

Revision History:

  --*/

#ifndef   __BTHDEF_H__
#define   __BTHDEF_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// Bluetooth 2.1 support added in KB942567
//
#if (NTDDI_VERSION > NTDDI_VISTASP1 || \
    (NTDDI_VERSION == NTDDI_VISTASP1 && defined(VISTA_KB942567)))

  #define BTH_MAJORVERSION 2
  #define BTH_MINORVERSION 1

#elif(NTDDI_VERSION >= NTDDI_WINXPSP2)

  #define BTH_MAJORVERSION 2
  #define BTH_MINORVERSION 0

#endif // >= SP1+KB942567

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201) // nameless struct/union

#if (NTDDI_VERSION >= NTDDI_WINXPSP2)

#ifndef GUID_DEFS_ONLY
  #ifndef NO_BTHSDPDEF_INC
    #include "bthsdpdef.h"
  #endif // NO_BTHSDPDEF_INC
#endif // GUID_DEFS_ONLY

#ifndef NO_GUID_DEFS

// {0850302A-B344-4fda-9BE9-90576B8D46F0}
DEFINE_GUID(GUID_BTHPORT_DEVICE_INTERFACE,              0x850302a, 0xb344, 0x4fda, 0x9b, 0xe9, 0x90, 0x57, 0x6b, 0x8d, 0x46, 0xf0);

// RFCOMM device interface GUID for RFCOMM services
// {b142fc3e-fa4e-460b-8abc-072b628b3c70}
DEFINE_GUID(GUID_BTH_RFCOMM_SERVICE_DEVICE_INTERFACE,   0xb142fc3e, 0xfa4e, 0x460b, 0x8a, 0xbc, 0x07, 0x2b, 0x62, 0x8b, 0x3c, 0x70);

// {EA3B5B82-26EE-450E-B0D8-D26FE30A3869}
DEFINE_GUID(GUID_BLUETOOTH_RADIO_IN_RANGE,              0xea3b5b82, 0x26ee, 0x450e, 0xb0, 0xd8, 0xd2, 0x6f, 0xe3, 0x0a, 0x38, 0x69);

// {E28867C9-C2AA-4CED-B969-4570866037C4}
DEFINE_GUID(GUID_BLUETOOTH_RADIO_OUT_OF_RANGE,          0xe28867c9, 0xc2aa, 0x4ced, 0xb9, 0x69, 0x45, 0x70, 0x86, 0x60, 0x37, 0xc4);

// {7EAE4030-B709-4AA8-AC55-E953829C9DAA}
DEFINE_GUID(GUID_BLUETOOTH_L2CAP_EVENT,                 0x7eae4030, 0xb709, 0x4aa8, 0xac, 0x55, 0xe9, 0x53, 0x82, 0x9c, 0x9d, 0xaa);

// {FC240062-1541-49BE-B463-84C4DCD7BF7F}
DEFINE_GUID(GUID_BLUETOOTH_HCI_EVENT,                   0xfc240062, 0x1541, 0x49be, 0xb4, 0x63, 0x84, 0xc4, 0xdc, 0xd7, 0xbf, 0x7f);

//
// Support added in KB942567
//
#if (NTDDI_VERSION > NTDDI_VISTASP1 || \
    (NTDDI_VERSION == NTDDI_VISTASP1 && defined(VISTA_KB942567)))

// {5DC9136D-996C-46DB-84F5-32C0A3F47352}
DEFINE_GUID(GUID_BLUETOOTH_AUTHENTICATION_REQUEST,      0x5DC9136D, 0x996C, 0x46DB, 0x84, 0xF5, 0x32, 0xC0, 0xA3, 0xF4, 0x73, 0x52);

// {D668DFCD-0F4E-4EFC-BFE0-392EEEC5109C}
DEFINE_GUID(GUID_BLUETOOTH_KEYPRESS_EVENT,              0xD668DFCD, 0x0F4E, 0x4EFC, 0xBF, 0xE0, 0x39, 0x2E, 0xEE, 0xC5, 0x10, 0x9C);

// {547247e6-45bb-4c33-af8c-c00efe15a71d}
DEFINE_GUID(GUID_BLUETOOTH_HCI_VENDOR_EVENT,            0x547247e6, 0x45bb, 0x4c33, 0xaf, 0x8c, 0xc0, 0x0e, 0xfe, 0x15, 0xa7, 0x1d);

#endif // >= SP1+KB942567

//
// Bluetooth base UUID for service discovery
//
DEFINE_GUID(Bluetooth_Base_UUID,   0x00000000, 0x0000, 0x1000, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB);

//
// Creates the 128-bit UUID from a short id by using the Bluetooth base UUID
// {<short id>-0000-1000-8000-00805F9B34FB}
//
#define DEFINE_BLUETOOTH_UUID128(name,shortId) \
    DEFINE_GUID(name,shortId,0x0000,0x1000,0x80,0x00,0x00,0x80,0x5F,0x9B,0x34,0xFB)


//
// UUIDs for the Protocol Identifiers, Service Discovery Assigned Numbers
//
#define SDP_PROTOCOL_UUID16                 (0x0001)
#define UDP_PROTOCOL_UUID16                 (0x0002)
#define RFCOMM_PROTOCOL_UUID16              (0x0003)
#define TCP_PROTOCOL_UUID16                 (0x0004)
#define TCSBIN_PROTOCOL_UUID16              (0x0005)
#define TCSAT_PROTOCOL_UUID16               (0x0006)
#define ATT_PROTOCOL_UUID16                 (0x0007)
#define OBEX_PROTOCOL_UUID16                (0x0008)
#define IP_PROTOCOL_UUID16                  (0x0009)
#define FTP_PROTOCOL_UUID16                 (0x000A)
#define HTTP_PROTOCOL_UUID16                (0x000C)
#define WSP_PROTOCOL_UUID16                 (0x000E)
#define BNEP_PROTOCOL_UUID16                (0x000F)
#define UPNP_PROTOCOL_UUID16                (0x0010)
#define HID_PROTOCOL_UUID16                 (0x0011)
#define HCCC_PROTOCOL_UUID16                (0x0012)
#define HCDC_PROTOCOL_UUID16                (0x0014)
#define HCN_PROTOCOL_UUID16                 (0x0016)
#define AVCTP_PROTOCOL_UUID16               (0x0017)
#define AVDTP_PROTOCOL_UUID16               (0x0019)
#define CMPT_PROTOCOL_UUID16                (0x001B)
#define UDI_C_PLANE_PROTOCOL_UUID16         (0x001D)
#define L2CAP_PROTOCOL_UUID16               (0x0100)

DEFINE_BLUETOOTH_UUID128(SDP_PROTOCOL_UUID,         SDP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(UDP_PROTOCOL_UUID,         UDP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(RFCOMM_PROTOCOL_UUID,      RFCOMM_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(TCP_PROTOCOL_UUID,         TCP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(TCSBIN_PROTOCOL_UUID,      TCSBIN_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(TCSAT_PROTOCOL_UUID,       TCSAT_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(ATT_PROTOCOL_UUID,         ATT_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(OBEX_PROTOCOL_UUID,        OBEX_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(IP_PROTOCOL_UUID,          IP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(FTP_PROTOCOL_UUID,         FTP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(HTTP_PROTOCOL_UUID,        HTTP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(WSP_PROTOCOL_UUID,         WSP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(BNEP_PROTOCOL_UUID,        BNEP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(UPNP_PROTOCOL_UUID,        UPNP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(HID_PROTOCOL_UUID,         HID_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(HCCC_PROTOCOL_UUID,        HCCC_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(HCDC_PROTOCOL_UUID,        HCDC_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(HCN_PROTOCOL_UUID,         HCN_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(AVCTP_PROTOCOL_UUID,       AVCTP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(AVDTP_PROTOCOL_UUID,       AVDTP_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(CMPT_PROTOCOL_UUID,        CMPT_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(UDI_C_PLANE_PROTOCOL_UUID, UDI_C_PLANE_PROTOCOL_UUID16);
DEFINE_BLUETOOTH_UUID128(L2CAP_PROTOCOL_UUID,       L2CAP_PROTOCOL_UUID16);

//
// UUIDs for Service Class IDs, Service Discovery Assigned Numbers
//
#define ServiceDiscoveryServerServiceClassID_UUID16     (0x1000)
#define BrowseGroupDescriptorServiceClassID_UUID16      (0x1001)
#define PublicBrowseGroupServiceClassID_UUID16          (0x1002)

#define SerialPortServiceClassID_UUID16                 (0x1101)
#define LANAccessUsingPPPServiceClassID_UUID16          (0x1102)
#define DialupNetworkingServiceClassID_UUID16           (0x1103)
#define IrMCSyncServiceClassID_UUID16                   (0x1104)
#define OBEXObjectPushServiceClassID_UUID16             (0x1105)
#define OBEXFileTransferServiceClassID_UUID16           (0x1106)
#define IrMcSyncCommandServiceClassID_UUID16            (0x1107)
#define HeadsetServiceClassID_UUID16                    (0x1108)
#define CordlessTelephonyServiceClassID_UUID16          (0x1109)
#define AudioSourceServiceClassID_UUID16                (0x110A)
#define AudioSinkServiceClassID_UUID16                  (0x110B)
#define AVRemoteControlTargetServiceClassID_UUID16      (0x110C)
#define AVRemoteControlServiceClassID_UUID16            (0x110E)
#define AVRemoteControlControllerServiceClass_UUID16    (0x110F)
#define IntercomServiceClassID_UUID16                   (0x1110)
#define FaxServiceClassID_UUID16                        (0x1111)
#define HeadsetAudioGatewayServiceClassID_UUID16        (0x1112)
#define WAPServiceClassID_UUID16                        (0x1113)
#define WAPClientServiceClassID_UUID16                  (0x1114)
#define PANUServiceClassID_UUID16                       (0x1115)
#define NAPServiceClassID_UUID16                        (0x1116)
#define GNServiceClassID_UUID16                         (0x1117)
#define DirectPrintingServiceClassID_UUID16             (0x1118)
#define ReferencePrintingServiceClassID_UUID16          (0x1119)
#define ImagingResponderServiceClassID_UUID16           (0x111B)
#define ImagingAutomaticArchiveServiceClassID_UUID16    (0x111C)
#define ImagingReferenceObjectsServiceClassID_UUID16    (0x111D)
#define HandsfreeServiceClassID_UUID16                  (0x111E)
#define HandsfreeAudioGatewayServiceClassID_UUID16      (0x111F)
#define DirectPrintingReferenceObjectsServiceClassID_UUID16 \
                                                        (0x1120)
#define ReflectsUIServiceClassID_UUID16                 (0x1121)
#define PrintingStatusServiceClassID_UUID16             (0x1123)
#define HumanInterfaceDeviceServiceClassID_UUID16       (0x1124)
#define HCRPrintServiceClassID_UUID16                   (0x1126)
#define HCRScanServiceClassID_UUID16                    (0x1127)
#define CommonISDNAccessServiceClassID_UUID16           (0x1128)
#define VideoConferencingGWServiceClassID_UUID16        (0x1129)
#define UDIMTServiceClassID_UUID16                      (0x112A)
#define UDITAServiceClassID_UUID16                      (0x112B)
#define AudioVideoServiceClassID_UUID16                 (0x112C)
#define SimAccessServiceClassID_UUID16                  (0x112D)
#define PhonebookAccessPceServiceClassID_UUID16         (0x112E)
#define PhonebookAccessPseServiceClassID_UUID16         (0x112F)

#define HeadsetHSServiceClassID_UUID16                  (0x1131)
#define MessageAccessServerServiceClassID_UUID16        (0x1132)
#define MessageNotificationServerServiceClassID_UUID16  (0x1133)

#define GNSSServerServiceClassID_UUID16                 (0x1136)
#define ThreeDimensionalDisplayServiceClassID_UUID16    (0x1137)
#define ThreeDimensionalGlassesServiceClassID_UUID16    (0x1138)

#define MPSServiceClassID_UUID16                        (0x113B)
#define CTNAccessServiceClassID_UUID16                  (0x113C)
#define CTNNotificationServiceClassID_UUID16            (0x113D)

#define PnPInformationServiceClassID_UUID16             (0x1200)
#define GenericNetworkingServiceClassID_UUID16          (0x1201)
#define GenericFileTransferServiceClassID_UUID16        (0x1202)
#define GenericAudioServiceClassID_UUID16               (0x1203)
#define GenericTelephonyServiceClassID_UUID16           (0x1204)
#define UPnpServiceClassID_UUID16                       (0x1205)
#define UPnpIpServiceClassID_UUID16                     (0x1206)

#define ESdpUpnpIpPanServiceClassID_UUID16              (0x1300)
#define ESdpUpnpIpLapServiceClassID_UUID16              (0x1301)
#define ESdpUpnpL2capServiceClassID_UUID16              (0x1302)
#define VideoSourceServiceClassID_UUID16                (0x1303)
#define VideoSinkServiceClassID_UUID16                  (0x1304)

#define HealthDeviceProfileSourceServiceClassID_UUID16  (0x1401)
#define HealthDeviceProfileSinkServiceClassID_UUID16    (0x1402)

DEFINE_BLUETOOTH_UUID128(ServiceDiscoveryServerServiceClassID_UUID,         ServiceDiscoveryServerServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(BrowseGroupDescriptorServiceClassID_UUID,          BrowseGroupDescriptorServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(PublicBrowseGroupServiceClass_UUID,                PublicBrowseGroupServiceClassID_UUID16);

DEFINE_BLUETOOTH_UUID128(SerialPortServiceClass_UUID,                       SerialPortServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(LANAccessUsingPPPServiceClass_UUID,                LANAccessUsingPPPServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(DialupNetworkingServiceClass_UUID,                 DialupNetworkingServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(IrMCSyncServiceClass_UUID,                         IrMCSyncServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(OBEXObjectPushServiceClass_UUID,                   OBEXObjectPushServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(OBEXFileTransferServiceClass_UUID,                 OBEXFileTransferServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(IrMCSyncCommandServiceClass_UUID,                  IrMcSyncCommandServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(HeadsetServiceClass_UUID,                          HeadsetServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(CordlessTelephonyServiceClass_UUID,                CordlessTelephonyServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(AudioSourceServiceClass_UUID,                      AudioSourceServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(AudioSinkServiceClass_UUID,                        AudioSinkServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(AVRemoteControlTargetServiceClass_UUID,            AVRemoteControlTargetServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(AVRemoteControlServiceClass_UUID,                  AVRemoteControlServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(AVRemoteControlControllerServiceClass_UUID,        AVRemoteControlControllerServiceClass_UUID16);
DEFINE_BLUETOOTH_UUID128(IntercomServiceClass_UUID,                         IntercomServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(FaxServiceClass_UUID,                              FaxServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(HeadsetAudioGatewayServiceClass_UUID,              HeadsetAudioGatewayServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(WAPServiceClass_UUID,                              WAPServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(WAPClientServiceClass_UUID,                        WAPClientServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(PANUServiceClass_UUID,                             PANUServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(NAPServiceClass_UUID,                              NAPServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(GNServiceClass_UUID,                               GNServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(DirectPrintingServiceClass_UUID,                   DirectPrintingServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(ReferencePrintingServiceClass_UUID,                ReferencePrintingServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(ImagingResponderServiceClass_UUID,                 ImagingResponderServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(ImagingAutomaticArchiveServiceClass_UUID,          ImagingAutomaticArchiveServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(ImagingReferenceObjectsServiceClass_UUID,          ImagingReferenceObjectsServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(HandsfreeServiceClass_UUID,                        HandsfreeServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(HandsfreeAudioGatewayServiceClass_UUID,            HandsfreeAudioGatewayServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(DirectPrintingReferenceObjectsServiceClass_UUID,   DirectPrintingReferenceObjectsServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(ReflectedUIServiceClass_UUID,                      ReflectsUIServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(PrintingStatusServiceClass_UUID,                   PrintingStatusServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(HumanInterfaceDeviceServiceClass_UUID,             HumanInterfaceDeviceServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(HCRPrintServiceClass_UUID,                         HCRPrintServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(HCRScanServiceClass_UUID,                          HCRScanServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(CommonISDNAccessServiceClass_UUID,                 CommonISDNAccessServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(VideoConferencingGWServiceClass_UUID,              VideoConferencingGWServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(UDIMTServiceClass_UUID,                            UDIMTServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(UDITAServiceClass_UUID,                            UDITAServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(AudioVideoServiceClass_UUID,                       AudioVideoServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(SimAccessServiceClass_UUID,                        SimAccessServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(PhonebookAccessPceServiceClass_UUID,               PhonebookAccessPceServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(PhonebookAccessPseServiceClass_UUID,               PhonebookAccessPseServiceClassID_UUID16);

DEFINE_BLUETOOTH_UUID128(HeadsetHSServiceClass_UUID,                        HeadsetHSServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(MessageAccessServerServiceClass_UUID,              MessageAccessServerServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(MessageNotificationServerServiceClass_UUID,        MessageNotificationServerServiceClassID_UUID16);

DEFINE_BLUETOOTH_UUID128(GNSSServerServiceClass_UUID,                       GNSSServerServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(ThreeDimensionalDisplayServiceClass_UUID,          ThreeDimensionalDisplayServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(ThreeDimensionalGlassesServiceClass_UUID,          ThreeDimensionalGlassesServiceClassID_UUID16);

DEFINE_BLUETOOTH_UUID128(MPSServiceClass_UUID,                              MPSServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(CTNAccessServiceClass_UUID,                        CTNAccessServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(CTNNotificationServiceClass_UUID,                  CTNNotificationServiceClassID_UUID16);

DEFINE_BLUETOOTH_UUID128(PnPInformationServiceClass_UUID,                   PnPInformationServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(GenericNetworkingServiceClass_UUID,                GenericNetworkingServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(GenericFileTransferServiceClass_UUID,              GenericFileTransferServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(GenericAudioServiceClass_UUID,                     GenericAudioServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(GenericTelephonyServiceClass_UUID,                 GenericTelephonyServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(UPnpServiceClass_UUID,                             UPnpServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(UPnpIpServiceClass_UUID,                           UPnpIpServiceClassID_UUID16);

DEFINE_BLUETOOTH_UUID128(ESdpUpnpIpPanServiceClass_UUID,                    ESdpUpnpIpPanServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(ESdpUpnpIpLapServiceClass_UUID,                    ESdpUpnpIpLapServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(ESdpUpnpL2capServiceClass_UUID,                    ESdpUpnpL2capServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(VideoSourceServiceClass_UUID,                      VideoSourceServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(VideoSinkServiceClass_UUID,                        VideoSinkServiceClassID_UUID16);

DEFINE_BLUETOOTH_UUID128(HealthDeviceProfileSourceServiceClass_UUID,        HealthDeviceProfileSourceServiceClassID_UUID16);
DEFINE_BLUETOOTH_UUID128(HealthDeviceProfileSinkServiceClass_UUID,          HealthDeviceProfileSinkServiceClassID_UUID16);

//
// UUIDs for SIG defined profiles, Service Discovery Assigned Numbers
//
#define AdvancedAudioDistributionProfileID_UUID16       (0x110D)
#define ImagingServiceProfileID_UUID16                  (0x111A)
#define BasicPrintingProfileID_UUID16                   (0x1122)
#define HardcopyCableReplacementProfileID_UUID16        (0x1125)
#define PhonebookAccessProfileID_UUID16                 (0x1130)
#define MessageAccessProfileID_UUID16                   (0x1134)
#define GNSSProfileID_UUID16                            (0x1135)
#define ThreeDimensionalSynchronizationProfileID_UUID16 (0x1139)
#define MPSProfileID_UUID16                             (0x113A)
#define CTNProfileID_UUID16                             (0x113E)
#define VideoDistributionProfileID_UUID16               (0x1305)
#define HealthDeviceProfileID_UUID16                    (0x1400)

DEFINE_BLUETOOTH_UUID128(AdvancedAudioDistributionProfile_UUID,             AdvancedAudioDistributionProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(ImagingServiceProfile_UUID,                        ImagingServiceProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(BasicPrintingProfile_UUID,                         BasicPrintingProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(HardcopyCableReplacementProfile_UUID,              HardcopyCableReplacementProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(PhonebookAccessProfile_UUID,                       PhonebookAccessProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(MessageAccessProfile_UUID,                         MessageAccessProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(GNSSProfile_UUID,                                  GNSSProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(ThreeDimensionalSynchronizationProfile_UUID,       ThreeDimensionalSynchronizationProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(MPSProfile_UUID,                                   MPSProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(CTNProfile_UUID,                                   CTNProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(VideoDistributionProfile_UUID,                     VideoDistributionProfileID_UUID16);
DEFINE_BLUETOOTH_UUID128(HealthDeviceProfile_UUID,                          HealthDeviceProfileID_UUID16);

//
// The SIG renamed the uuid for VideoConferencingServiceClass
//
#define VideoConferencingServiceClass_UUID              AVRemoteControlControllerServiceClass_UUID
#define VideoConferencingServiceClassID_UUID16          AVRemoteControlControllerServiceClass_UUID16

//
// Fixing typos introduced in previous releases
//
#define HN_PROTOCOL_UUID HCN_PROTOCOL_UUID
#define BasicPringingServiceClass_UUID BasicPrintingProfile_UUID

//
// Fixing naming inconsistencies in UUID16 list
//
#define CommonISDNAccessServiceClass_UUID16                 CommonISDNAccessServiceClassID_UUID16
#define VideoConferencingGWServiceClass_UUID16              VideoConferencingGWServiceClassID_UUID16
#define UDIMTServiceClass_UUID16                            UDIMTServiceClassID_UUID16
#define UDITAServiceClass_UUID16                            UDITAServiceClassID_UUID16
#define AudioVideoServiceClass_UUID16                       AudioVideoServiceClassID_UUID16

//
// Fixing naming inconsistencies in profile list
//
#define CordlessServiceClassID_UUID16                       CordlessTelephonyServiceClassID_UUID16
#define AudioSinkSourceServiceClassID_UUID16                AudioSinkServiceClassID_UUID16
#define AdvancedAudioDistributionServiceClassID_UUID16      AdvancedAudioDistributionProfileID_UUID16
#define ImagingServiceClassID_UUID16                        ImagingServiceProfileID_UUID16
#define BasicPrintingServiceClassID_UUID16                  BasicPrintingProfileID_UUID16
#define HardcopyCableReplacementServiceClassID_UUID16       HardcopyCableReplacementProfileID_UUID16

#define AdvancedAudioDistributionServiceClass_UUID          AdvancedAudioDistributionProfile_UUID 
#define ImagingServiceClass_UUID                            ImagingServiceProfile_UUID
#define BasicPrintingServiceClass_UUID                      BasicPrintingProfile_UUID
#define HardcopyCableReplacementServiceClass_UUID           HardcopyCableReplacementProfile_UUID
#define VideoDistributionServiceClass_UUID                  VideoDistributionProfile_UUID

#endif //  NO_GUID_DEFS

#ifndef GUID_DEFS_ONLY

//
// max length of device friendly name.
//
#define BTH_MAX_NAME_SIZE          (248)

#define BTH_MAX_PIN_SIZE            (16)
#define BTH_LINK_KEY_LENGTH         (16)

#define BTH_MFG_ERICSSON        (0)
#define BTH_MFG_NOKIA           (1)
#define BTH_MFG_INTEL           (2)
#define BTH_MFG_IBM             (3)
#define BTH_MFG_TOSHIBA         (4)
#define BTH_MFG_3COM            (5)
#define BTH_MFG_MICROSOFT       (6)
#define BTH_MFG_LUCENT          (7)
#define BTH_MFG_MOTOROLA        (8)
#define BTH_MFG_INFINEON        (9)
#define BTH_MFG_CSR             (10)
#define BTH_MFG_SILICONWAVE     (11)
#define BTH_MFG_DIGIANSWER      (12)
#define BTH_MFG_TI              (13)
#define BTH_MFG_PARTHUS         (14)
#define BTH_MFG_BROADCOM        (15)
#define BTH_MFG_MITEL           (16)
#define BTH_MFG_WIDCOMM         (17)
#define BTH_MFG_ZEEVO           (18)
#define BTH_MFG_ATMEL           (19)
#define BTH_MFG_MITSIBUSHI      (20)
#define BTH_MFG_RTX_TELECOM     (21)
#define BTH_MFG_KC_TECHNOLOGY   (22)
#define BTH_MFG_NEWLOGIC        (23)
#define BTH_MFG_TRANSILICA      (24)
#define BTH_MFG_ROHDE_SCHWARZ   (25)
#define BTH_MFG_TTPCOM          (26)
#define BTH_MFG_SIGNIA          (27)
#define BTH_MFG_CONEXANT        (28)
#define BTH_MFG_QUALCOMM        (29)
#define BTH_MFG_INVENTEL        (30)
#define BTH_MFG_AVM_BERLIN      (31)
#define BTH_MFG_BANDSPEED       (32)
#define BTH_MFG_MANSELLA        (33)
#define BTH_MFG_NEC             (34)
#define BTH_MFG_WAVEPLUS_TECHNOLOGY_CO (35)
#define BTH_MFG_ALCATEL         (36)
#define BTH_MFG_PHILIPS_SEMICONDUCTOR  (37)
#define BTH_MFG_C_TECHNOLOGIES  (38)
#define BTH_MFG_OPEN_INTERFACE  (39)
#define BTH_MFG_RF_MICRO_DEVICES (40)
#define BTH_MFG_HITACHI        (41)
#define BTH_MFG_SYMBOL_TECHNOLOGIES (42)
#define BTH_MFG_TENOVIS        (43)
#define BTH_MFG_MACRONIX_INTERNATIONAL (44)
#define BTH_MFG_MARVELL        (72)
#define BTH_MFG_APPLE          (76)
#define BTH_MFG_NORDIC_SEMICONDUCTORS_ASA (89)
#define BTH_MFG_ARUBA_NETWORKS (283)
#define BTH_MFG_INTERNAL_USE    (65535)

typedef ULONGLONG BTH_ADDR, *PBTH_ADDR;
typedef ULONG BTH_COD, *PBTH_COD;
typedef ULONG BTH_LAP, *PBTH_LAP;

#define BTH_ADDR_NULL            ((ULONGLONG) 0x0000000000000000)

#define NAP_MASK                ((ULONGLONG) 0xFFFF00000000)
#define SAP_MASK                ((ULONGLONG) 0x0000FFFFFFFF)

#define NAP_BIT_OFFSET          (8 * 4)
#define SAP_BIT_OFFSET          (0)

#define GET_NAP(_bth_addr)  ((USHORT) (((_bth_addr) & NAP_MASK) >> NAP_BIT_OFFSET))
#define GET_SAP(_bth_addr)  ((ULONG)  (((_bth_addr) & SAP_MASK) >> SAP_BIT_OFFSET))

#define SET_NAP(_nap) (((ULONGLONG) ((USHORT) (_nap))) << NAP_BIT_OFFSET)
#define SET_SAP(_sap) (((ULONGLONG) ((ULONG)  (_sap))) << SAP_BIT_OFFSET)

#define SET_NAP_SAP(_nap, _sap) (SET_NAP(_nap) | SET_SAP(_sap))

#define COD_FORMAT_BIT_OFFSET   (0)
#define COD_MINOR_BIT_OFFSET    (2)
#define COD_MAJOR_BIT_OFFSET    (8 * 1)
#define COD_SERVICE_BIT_OFFSET  (8 * 1 + 5)

#define COD_FORMAT_MASK         (0x000003)
#define COD_MINOR_MASK          (0x0000FC)
#define COD_MAJOR_MASK          (0x001F00)
#define COD_SERVICE_MASK        (0xFFE000)

#define GET_COD_FORMAT(_cod)    ( (_cod) & COD_FORMAT_MASK   >> COD_FORMAT_BIT_OFFSET)
#define GET_COD_MINOR(_cod)     (((_cod) & COD_MINOR_MASK)   >> COD_MINOR_BIT_OFFSET)
#define GET_COD_MAJOR(_cod)     (((_cod) & COD_MAJOR_MASK)   >> COD_MAJOR_BIT_OFFSET)
#define GET_COD_SERVICE(_cod)   (((_cod) & COD_SERVICE_MASK) >> COD_SERVICE_BIT_OFFSET)

#define SET_COD_MINOR(_cod, _minor) (_cod) = ((_cod) & ~COD_MINOR_MASK) | ((_minor) << COD_MINOR_BIT_OFFSET)
#define SET_COD_MAJOR(_cod, _major) (_cod) = ((_cod) & ~COD_MAJOR_MASK) | ((_major) << COD_MAJOR_BIT_OFFSET)
#define SET_COD_SERVICE(_cod, _service) (_cod) = ((_cod) & ~COD_SERVICE_MASK) | ((_service) << COD_SERVICE_BIT_OFFSET)

#define COD_VERSION                            (0x0)

#define COD_SERVICE_LIMITED                 (0x0001)
#define COD_SERVICE_LE_AUDIO                (0x0002)
#define COD_SERVICE_POSITIONING             (0x0008)
#define COD_SERVICE_NETWORKING              (0x0010)
#define COD_SERVICE_RENDERING               (0x0020)
#define COD_SERVICE_CAPTURING               (0x0040)
#define COD_SERVICE_OBJECT_XFER             (0x0080)
#define COD_SERVICE_AUDIO                   (0x0100)
#define COD_SERVICE_TELEPHONY               (0x0200)
#define COD_SERVICE_INFORMATION             (0x0400)

#define COD_SERVICE_VALID_MASK              (COD_SERVICE_LIMITED        | \
                                             COD_SERVICE_LE_AUDIO       | \
                                             COD_SERVICE_POSITIONING    | \
                                             COD_SERVICE_NETWORKING     | \
                                             COD_SERVICE_RENDERING      | \
                                             COD_SERVICE_CAPTURING      | \
                                             COD_SERVICE_OBJECT_XFER    |\
                                             COD_SERVICE_AUDIO          |\
                                             COD_SERVICE_TELEPHONY      |\
                                             COD_SERVICE_INFORMATION)

#define COD_SERVICE_MAX_COUNT               (10)

//
// Major class codes
//
#define COD_MAJOR_MISCELLANEOUS             (0x00)
#define COD_MAJOR_COMPUTER                  (0x01)
#define COD_MAJOR_PHONE                     (0x02)
#define COD_MAJOR_LAN_ACCESS                (0x03)
#define COD_MAJOR_AUDIO                     (0x04)
#define COD_MAJOR_PERIPHERAL                (0x05)
#define COD_MAJOR_IMAGING                   (0x06)
#define COD_MAJOR_WEARABLE                  (0x07)
#define COD_MAJOR_TOY                       (0x08)
#define COD_MAJOR_HEALTH                    (0x09)
#define COD_MAJOR_UNCLASSIFIED              (0x1F)

//
// Minor class codes specific to each major class
//
#define COD_COMPUTER_MINOR_UNCLASSIFIED     (0x00)
#define COD_COMPUTER_MINOR_DESKTOP          (0x01)
#define COD_COMPUTER_MINOR_SERVER           (0x02)
#define COD_COMPUTER_MINOR_LAPTOP           (0x03)
#define COD_COMPUTER_MINOR_HANDHELD         (0x04)
#define COD_COMPUTER_MINOR_PALM             (0x05)
#define COD_COMPUTER_MINOR_WEARABLE         (0x06)

#define COD_PHONE_MINOR_UNCLASSIFIED        (0x00)
#define COD_PHONE_MINOR_CELLULAR            (0x01)
#define COD_PHONE_MINOR_CORDLESS            (0x02)
#define COD_PHONE_MINOR_SMART               (0x03)
#define COD_PHONE_MINOR_WIRED_MODEM         (0x04)

#define COD_AUDIO_MINOR_UNCLASSIFIED        (0x00)
#define COD_AUDIO_MINOR_HEADSET             (0x01)
#define COD_AUDIO_MINOR_HANDS_FREE          (0x02)
#define COD_AUDIO_MINOR_HEADSET_HANDS_FREE  (0x03)
#define COD_AUDIO_MINOR_MICROPHONE          (0x04)
#define COD_AUDIO_MINOR_LOUDSPEAKER         (0x05)
#define COD_AUDIO_MINOR_HEADPHONES          (0x06)
#define COD_AUDIO_MINOR_PORTABLE_AUDIO      (0x07)
#define COD_AUDIO_MINOR_CAR_AUDIO           (0x08)
#define COD_AUDIO_MINOR_SET_TOP_BOX         (0x09)
#define COD_AUDIO_MINOR_HIFI_AUDIO          (0x0A)
#define COD_AUDIO_MINOR_VCR                 (0x0B)
#define COD_AUDIO_MINOR_VIDEO_CAMERA        (0x0C)
#define COD_AUDIO_MINOR_CAMCORDER           (0x0D)
#define COD_AUDIO_MINOR_VIDEO_MONITOR       (0x0E)
#define COD_AUDIO_MINOR_VIDEO_DISPLAY_LOUDSPEAKER \
                                            (0x0F)
#define COD_AUDIO_MINOR_VIDEO_DISPLAY_CONFERENCING \
                                            (0x10)
// #define COD_AUDIO_MINOR_RESERVED            (0x11)
#define COD_AUDIO_MINOR_GAMING_TOY          (0x12)

#define COD_PERIPHERAL_MINOR_KEYBOARD_MASK  (0x10)
#define COD_PERIPHERAL_MINOR_POINTER_MASK   (0x20)

#define COD_PERIPHERAL_MINOR_NO_CATEGORY    (0x00)
#define COD_PERIPHERAL_MINOR_JOYSTICK       (0x01)
#define COD_PERIPHERAL_MINOR_GAMEPAD        (0x02)
#define COD_PERIPHERAL_MINOR_REMOTE_CONTROL (0x03)
#define COD_PERIPHERAL_MINOR_SENSING        (0x04)

#define COD_IMAGING_MINOR_DISPLAY_MASK      (0x04)
#define COD_IMAGING_MINOR_CAMERA_MASK       (0x08)
#define COD_IMAGING_MINOR_SCANNER_MASK      (0x10)
#define COD_IMAGING_MINOR_PRINTER_MASK      (0x20)

#define COD_WEARABLE_MINOR_WRIST_WATCH      (0x01)
#define COD_WEARABLE_MINOR_PAGER            (0x02)
#define COD_WEARABLE_MINOR_JACKET           (0x03)
#define COD_WEARABLE_MINOR_HELMET           (0x04)
#define COD_WEARABLE_MINOR_GLASSES          (0x05)

#define COD_TOY_MINOR_ROBOT                 (0x01)
#define COD_TOY_MINOR_VEHICLE               (0x02)
#define COD_TOY_MINOR_DOLL_ACTION_FIGURE    (0x03)
#define COD_TOY_MINOR_CONTROLLER            (0x04)
#define COD_TOY_MINOR_GAME                  (0x05)

#define COD_HEALTH_MINOR_BLOOD_PRESSURE_MONITOR \
                                            (0x01)
#define COD_HEALTH_MINOR_THERMOMETER        (0x02)
#define COD_HEALTH_MINOR_WEIGHING_SCALE     (0x03)
#define COD_HEALTH_MINOR_GLUCOSE_METER      (0x04)
#define COD_HEALTH_MINOR_PULSE_OXIMETER     (0x05)
#define COD_HEALTH_MINOR_HEART_PULSE_MONITOR \
                                            (0x06)
#define COD_HEALTH_MINOR_HEALTH_DATA_DISPLAY \
                                            (0x07)
#define COD_HEALTH_MINOR_STEP_COUNTER       (0x08)

//
// Cannot use GET_COD_MINOR for this b/c it is embedded in a different manner
// than the rest of the major classes
//

#define COD_LAN_ACCESS_BIT_OFFSET (5)

#define COD_LAN_MINOR_MASK      (0x00001C)
#define COD_LAN_ACCESS_MASK     (0x0000E0)

#define GET_COD_LAN_MINOR(_cod)         (((_cod) & COD_LAN_MINOR_MASK) >> COD_MINOR_BIT_OFFSET)
#define GET_COD_LAN_ACCESS(_cod)         (((_cod) & COD_LAN_ACCESS_MASK) >> COD_LAN_ACCESS_BIT_OFFSET)

//
// LAN access percent usage subcodes
//
#define COD_LAN_MINOR_UNCLASSIFIED    (0x00)

#define COD_LAN_ACCESS_0_USED         (0x00)
#define COD_LAN_ACCESS_17_USED        (0x01)
#define COD_LAN_ACCESS_33_USED        (0x02)
#define COD_LAN_ACCESS_50_USED        (0x03)
#define COD_LAN_ACCESS_67_USED        (0x04)
#define COD_LAN_ACCESS_83_USED        (0x05)
#define COD_LAN_ACCESS_99_USED        (0x06)
#define COD_LAN_ACCESS_FULL           (0x07)

//
// Extended Inquiry Response (EIR) defines.
//
#define BTH_EIR_FLAGS_ID                (0x01)
#define BTH_EIR_16_UUIDS_PARTIAL_ID     (0x02)
#define BTH_EIR_16_UUIDS_COMPLETE_ID    (0x03)
#define BTH_EIR_32_UUIDS_PARTIAL_ID     (0x04)
#define BTH_EIR_32_UUIDS_COMPLETE_ID    (0x05)
#define BTH_EIR_128_UUIDS_PARTIAL_ID    (0x06)
#define BTH_EIR_128_UUIDS_COMPLETE_ID   (0x07)
#define BTH_EIR_LOCAL_NAME_PARTIAL_ID   (0x08)
#define BTH_EIR_LOCAL_NAME_COMPLETE_ID  (0x09)
#define BTH_EIR_TX_POWER_LEVEL_ID       (0x0A)
#define BTH_EIR_OOB_OPT_DATA_LEN_ID     (0x0B) // OOB only.
#define BTH_EIR_OOB_BD_ADDR_ID          (0x0C) // OOB only.
#define BTH_EIR_OOB_COD_ID              (0x0D) // OOB only.
#define BTH_EIR_OOB_SP_HASH_ID          (0x0E) // OOB only.
#define BTH_EIR_OOB_SP_RANDOMIZER_ID    (0x0F) // OOB only.
#define BTH_EIR_MANUFACTURER_ID         (0xFF)

//
// Extended Inquiry Response (EIR) size.
//
#define BTH_EIR_SIZE                    (240)

//
// Used as an initializer of LAP_DATA
//
#define LAP_GIAC_INIT                   { 0x33, 0x8B, 0x9E }
#define LAP_LIAC_INIT                   { 0x00, 0x8B, 0x9E }

//
// General Inquiry Access Code.
//
#define LAP_GIAC_VALUE                  (0x009E8B33)

//
// Limited Inquiry Access Code.
//
#define LAP_LIAC_VALUE                  (0x009E8B00)

#define BTH_ADDR_IAC_FIRST               (0x9E8B00)
#define BTH_ADDR_IAC_LAST                (0x9E8B3f)
#define BTH_ADDR_LIAC                    (0x9E8B00)
#define BTH_ADDR_GIAC                    (0x9E8B33)

typedef UCHAR BTHSTATUS, *PBTHSTATUS;

#define BTH_ERROR(_btStatus)   ((_btStatus) != BTH_ERROR_SUCCESS)
#define BTH_SUCCESS(_btStatus) ((_btStatus) == BTH_ERROR_SUCCESS)

#define BTH_ERROR_SUCCESS                           (0x00)
#define BTH_ERROR_UNKNOWN_HCI_COMMAND               (0x01)
#define BTH_ERROR_NO_CONNECTION                     (0x02)
#define BTH_ERROR_HARDWARE_FAILURE                  (0x03)
#define BTH_ERROR_PAGE_TIMEOUT                      (0x04)
#define BTH_ERROR_AUTHENTICATION_FAILURE            (0x05)
#define BTH_ERROR_KEY_MISSING                       (0x06)
#define BTH_ERROR_MEMORY_FULL                       (0x07)
#define BTH_ERROR_CONNECTION_TIMEOUT                (0x08)
#define BTH_ERROR_MAX_NUMBER_OF_CONNECTIONS         (0x09)
#define BTH_ERROR_MAX_NUMBER_OF_SCO_CONNECTIONS     (0x0a)
#define BTH_ERROR_ACL_CONNECTION_ALREADY_EXISTS     (0x0b)
#define BTH_ERROR_COMMAND_DISALLOWED                (0x0c)
#define BTH_ERROR_HOST_REJECTED_LIMITED_RESOURCES   (0x0d)
#define BTH_ERROR_HOST_REJECTED_SECURITY_REASONS    (0x0e)
#define BTH_ERROR_HOST_REJECTED_PERSONAL_DEVICE     (0x0f)
#define BTH_ERROR_HOST_TIMEOUT                      (0x10)
#define BTH_ERROR_UNSUPPORTED_FEATURE_OR_PARAMETER  (0x11)
#define BTH_ERROR_INVALID_HCI_PARAMETER             (0x12)
#define BTH_ERROR_REMOTE_USER_ENDED_CONNECTION      (0x13)
#define BTH_ERROR_REMOTE_LOW_RESOURCES              (0x14)
#define BTH_ERROR_REMOTE_POWERING_OFF               (0x15)
#define BTH_ERROR_LOCAL_HOST_TERMINATED_CONNECTION  (0x16)
#define BTH_ERROR_REPEATED_ATTEMPTS                 (0x17)
#define BTH_ERROR_PAIRING_NOT_ALLOWED               (0x18)
#define BTH_ERROR_UKNOWN_LMP_PDU                    (0x19)
#define BTH_ERROR_UNSUPPORTED_REMOTE_FEATURE        (0x1a)
#define BTH_ERROR_SCO_OFFSET_REJECTED               (0x1b)
#define BTH_ERROR_SCO_INTERVAL_REJECTED             (0x1c)
#define BTH_ERROR_SCO_AIRMODE_REJECTED              (0x1d)
#define BTH_ERROR_INVALID_LMP_PARAMETERS            (0x1e)
#define BTH_ERROR_UNSPECIFIED_ERROR                 (0x1f)
#define BTH_ERROR_UNSUPPORTED_LMP_PARM_VALUE        (0x20)
#define BTH_ERROR_ROLE_CHANGE_NOT_ALLOWED           (0x21)
#define BTH_ERROR_LMP_RESPONSE_TIMEOUT              (0x22)
#define BTH_ERROR_LMP_TRANSACTION_COLLISION         (0x23)
#define BTH_ERROR_LMP_PDU_NOT_ALLOWED               (0x24)
#define BTH_ERROR_ENCRYPTION_MODE_NOT_ACCEPTABLE    (0x25)
#define BTH_ERROR_UNIT_KEY_NOT_USED                 (0x26)
#define BTH_ERROR_QOS_IS_NOT_SUPPORTED              (0x27)
#define BTH_ERROR_INSTANT_PASSED                    (0x28)
#define BTH_ERROR_PAIRING_WITH_UNIT_KEY_NOT_SUPPORTED (0x29)
#define BTH_ERROR_DIFFERENT_TRANSACTION_COLLISION   (0x2a)
#define BTH_ERROR_QOS_UNACCEPTABLE_PARAMETER        (0x2c)
#define BTH_ERROR_QOS_REJECTED                      (0x2d)
#define BTH_ERROR_CHANNEL_CLASSIFICATION_NOT_SUPPORTED (0x2e)
#define BTH_ERROR_INSUFFICIENT_SECURITY             (0x2f)
#define BTH_ERROR_PARAMETER_OUT_OF_MANDATORY_RANGE  (0x30)
#define BTH_ERROR_ROLE_SWITCH_PENDING               (0x32)
#define BTH_ERROR_RESERVED_SLOT_VIOLATION           (0x34)
#define BTH_ERROR_ROLE_SWITCH_FAILED                (0x35)
#define BTH_ERROR_EXTENDED_INQUIRY_RESPONSE_TOO_LARGE (0x36)
#define BTH_ERROR_SECURE_SIMPLE_PAIRING_NOT_SUPPORTED_BY_HOST (0x37)
#define BTH_ERROR_HOST_BUSY_PAIRING                 (0x38)
#define BTH_ERROR_CONNECTION_REJECTED_DUE_TO_NO_SUITABLE_CHANNEL_FOUND \
                                                    (0x39)
#define BTH_ERROR_CONTROLLER_BUSY                   (0x3a)
#define BTH_ERROR_UNACCEPTABLE_CONNECTION_INTERVAL  (0x3b)
#define BTH_ERROR_DIRECTED_ADVERTISING_TIMEOUT      (0x3c)
#define BTH_ERROR_CONNECTION_TERMINATED_DUE_TO_MIC_FAILURE \
                                                    (0x3d)
#define BTH_ERROR_CONNECTION_FAILED_TO_BE_ESTABLISHED (0x3e)
#define BTH_ERROR_MAC_CONNECTION_FAILED             (0x3f)
#define BTH_ERROR_COARSE_CLOCK_ADJUSTMENT_REJECTED  (0x40)
#define BTH_ERROR_TYPE_0_SUBMAP_NOT_DEFINED         (0x41)
#define BTH_ERROR_UNKNOWN_ADVERTISING_IDENTIFIER    (0x42)
#define BTH_ERROR_LIMIT_REACHED                     (0x43)
#define BTH_ERROR_OPERATION_CANCELLED_BY_HOST       (0X44)
#define BTH_ERROR_PACKET_TOO_LONG                   (0x45)

#define BTH_ERROR_UNSPECIFIED                       (0xFF)

//
// Min, max, and default L2cap MTU.
//
#define L2CAP_MIN_MTU                       (48)
#define L2CAP_MAX_MTU                       (0xFFFF)
#define L2CAP_DEFAULT_MTU                   (672)

//
// Max l2cap signal size (48) - size of signal header (4)
//
#define MAX_L2CAP_PING_DATA_LENGTH            (44)
#define MAX_L2CAP_INFO_DATA_LENGTH            (44)

//
// the following structures provide information about
// disocvered remote radios.
//

#define BDIF_ADDRESS            (0x00000001)
#define BDIF_COD                (0x00000002)
#define BDIF_NAME               (0x00000004)
#define BDIF_PAIRED             (0x00000008)
#define BDIF_PERSONAL           (0x00000010)
#define BDIF_CONNECTED          (0x00000020)

//
// Support added in KB942567
//
#if (NTDDI_VERSION > NTDDI_VISTASP1 || \
    (NTDDI_VERSION == NTDDI_VISTASP1 && defined(VISTA_KB942567)))

#define BDIF_SHORT_NAME         (0x00000040)
#define BDIF_VISIBLE            (0x00000080)
#define BDIF_SSP_SUPPORTED      (0x00000100)
#define BDIF_SSP_PAIRED         (0x00000200)
#define BDIF_SSP_MITM_PROTECTED (0x00000400)
#define BDIF_RSSI               (0x00001000)
#define BDIF_EIR                (0x00002000)

#if (NTDDI_VERSION >= NTDDI_WIN8) // >= WIN8

#define BDIF_BR                 (0x00004000)
#define BDIF_LE                 (0x00008000)
#define BDIF_LE_PAIRED          (0x00010000)
#define BDIF_LE_PERSONAL        (0x00020000)
#define BDIF_LE_MITM_PROTECTED  (0x00040000)
#define BDIF_LE_PRIVACY_ENABLED (0x00080000)
#define BDIF_LE_RANDOM_ADDRESS_TYPE \
                                (0x00100000)

#if (NTDDI_VERSION >= NTDDI_WIN10) // >= WIN10

#define BDIF_LE_DISCOVERABLE    (0x00200000)
#define BDIF_LE_NAME            (0x00400000)
#define BDIF_LE_VISIBLE         (0x00800000)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2) // >= WIN10_RS2

#define BDIF_LE_CONNECTED       (0x01000000)
#define BDIF_LE_CONNECTABLE     (0x02000000)
#define BDIF_BR_SECURE_CONNECTION_PAIRED \
                                (0x08000000)
#define BDIF_LE_SECURE_CONNECTION_PAIRED \
                                (0x10000000)

#define BDIF_DEBUGKEY           (0x20000000)
#define BDIF_LE_DEBUGKEY        (0x40000000)

#if (NTDDI_VERSION >= NTDDI_WIN10_19H1) // >= NTDDI_WIN10_19H1

#define BDIF_TX_POWER           (0x80000000)


#define BDIF_VALID_FLAGS \
    (BDIF_ADDRESS | BDIF_COD | BDIF_NAME | BDIF_PAIRED | BDIF_PERSONAL | \
     BDIF_CONNECTED | BDIF_SHORT_NAME | BDIF_VISIBLE | BDIF_RSSI | BDIF_EIR | BDIF_SSP_PAIRED | BDIF_SSP_MITM_PROTECTED | \
     BDIF_BR | BDIF_LE | BDIF_LE_PAIRED | BDIF_LE_PERSONAL | BDIF_LE_MITM_PROTECTED | BDIF_LE_PRIVACY_ENABLED | BDIF_LE_RANDOM_ADDRESS_TYPE | \
     BDIF_LE_DISCOVERABLE | BDIF_LE_NAME | BDIF_LE_VISIBLE | BDIF_LE_CONNECTED | BDIF_LE_CONNECTABLE | \
     BDIF_BR_SECURE_CONNECTION_PAIRED | BDIF_LE_SECURE_CONNECTION_PAIRED | BDIF_DEBUGKEY | BDIF_LE_DEBUGKEY | BDIF_TX_POWER)

#else

#define BDIF_VALID_FLAGS \
    (BDIF_ADDRESS | BDIF_COD | BDIF_NAME | BDIF_PAIRED | BDIF_PERSONAL | \
     BDIF_CONNECTED | BDIF_SHORT_NAME | BDIF_VISIBLE | BDIF_RSSI | BDIF_EIR | BDIF_SSP_PAIRED | BDIF_SSP_MITM_PROTECTED | \
     BDIF_BR | BDIF_LE | BDIF_LE_PAIRED | BDIF_LE_PERSONAL | BDIF_LE_MITM_PROTECTED | BDIF_LE_PRIVACY_ENABLED | BDIF_LE_RANDOM_ADDRESS_TYPE | \
     BDIF_LE_DISCOVERABLE | BDIF_LE_NAME | BDIF_LE_VISIBLE | BDIF_LE_CONNECTED | BDIF_LE_CONNECTABLE | \
     BDIF_BR_SECURE_CONNECTION_PAIRED | BDIF_LE_SECURE_CONNECTION_PAIRED | BDIF_DEBUGKEY | BDIF_LE_DEBUGKEY)

#endif  // >= NTDDI_WIN10_19H1

#else

#define BDIF_VALID_FLAGS                                                    \
    (BDIF_ADDRESS | BDIF_COD | BDIF_NAME | BDIF_PAIRED | BDIF_PERSONAL |    \
     BDIF_CONNECTED | BDIF_SHORT_NAME | BDIF_VISIBLE | BDIF_RSSI | BDIF_EIR | BDIF_SSP_PAIRED | BDIF_SSP_MITM_PROTECTED | \
     BDIF_BR | BDIF_LE | BDIF_LE_PAIRED | BDIF_LE_PERSONAL | BDIF_LE_MITM_PROTECTED | BDIF_LE_PRIVACY_ENABLED | BDIF_LE_RANDOM_ADDRESS_TYPE | \
     BDIF_LE_DISCOVERABLE | BDIF_LE_NAME | BDIF_LE_VISIBLE)

#endif // >= WIN10_RS2

#else

#define BDIF_VALID_FLAGS                                                    \
    (BDIF_ADDRESS | BDIF_COD | BDIF_NAME | BDIF_PAIRED | BDIF_PERSONAL |    \
     BDIF_CONNECTED | BDIF_SHORT_NAME | BDIF_VISIBLE | BDIF_RSSI | BDIF_EIR | BDIF_SSP_PAIRED | BDIF_SSP_MITM_PROTECTED | \
     BDIF_BR | BDIF_LE | BDIF_LE_PAIRED | BDIF_LE_PERSONAL | BDIF_LE_MITM_PROTECTED | BDIF_LE_PRIVACY_ENABLED | BDIF_LE_RANDOM_ADDRESS_TYPE)

#endif // >= WIN10

#else

#define BDIF_VALID_FLAGS                                                    \
    (BDIF_ADDRESS | BDIF_COD | BDIF_NAME | BDIF_PAIRED | BDIF_PERSONAL |    \
     BDIF_CONNECTED | BDIF_SHORT_NAME | BDIF_VISIBLE | BDIF_RSSI | BDIF_EIR | BDIF_SSP_PAIRED | BDIF_SSP_MITM_PROTECTED)

#endif // >= WIN8

#else // <= SP1

#define BDIF_VALID_FLAGS                                                    \
    (BDIF_ADDRESS | BDIF_COD | BDIF_NAME | BDIF_PAIRED | BDIF_PERSONAL |    \
     BDIF_CONNECTED)

#endif // >= SP1+KB942567

typedef struct _BTH_DEVICE_INFO {
    //
    // Combination BDIF_Xxx flags
    //
    ULONG flags;

    //
    // Address of remote device.
    //
    BTH_ADDR address;

    //
    // Class Of Device.
    //
    BTH_COD classOfDevice;

    //
    // name of the device
    //
    CHAR name[BTH_MAX_NAME_SIZE];

} BTH_DEVICE_INFO, *PBTH_DEVICE_INFO;

//
// Buffer associated with GUID_BLUETOOTH_RADIO_IN_RANGE
//

typedef struct _BTH_RADIO_IN_RANGE {
    //
    // Information about the remote radio
    //
    BTH_DEVICE_INFO deviceInfo;

    //
    // The previous flags value for the BTH_DEVICE_INFO.  The receiver of this
    // notification can compare the deviceInfo.flags and previousDeviceFlags
    // to determine what has changed about this remote radio.
    //
    // For instance, if BDIF_NAME is set in deviceInfo.flags and not in
    // previousDeviceFlags, the remote radio's has just been retrieved.
    //
    ULONG previousDeviceFlags;

} BTH_RADIO_IN_RANGE, *PBTH_RADIO_IN_RANGE;

//
// Buffer associated with GUID_BLUETOOTH_L2CAP_EVENT
//
typedef struct _BTH_L2CAP_EVENT_INFO {
    //
    // Remote radio address which the L2CAP event is associated with
    //
    BTH_ADDR bthAddress;

    //
    // The PSM that is either being connected to or disconnected from
    //
    USHORT psm;

    //
    // If != 0, then the channel has just been established.  If == 0, then the
    // channel has been destroyed.  Notifications for a destroyed channel will
    // only be sent for channels successfully established.
    //
    UCHAR connected;

    //
    // If != 0, then the local host iniated the l2cap connection.  If == 0, then
    // the remote host initated the connection.  This field is only valid if
    // connect is != 0.
    //
    UCHAR initiated;

} BTH_L2CAP_EVENT_INFO, *PBTH_L2CAP_EVENT_INFO;

#define HCI_CONNECTION_TYPE_ACL    (1)
#define HCI_CONNECTION_TYPE_SCO    (2)
#define HCI_CONNECTION_TYPE_LE     (3)

//
// Fix typos
//
#define HCI_CONNNECTION_TYPE_ACL    HCI_CONNECTION_TYPE_ACL
#define HCI_CONNNECTION_TYPE_SCO    HCI_CONNECTION_TYPE_SCO

//
// Buffer associated with GUID_BLUETOOTH_HCI_EVENT
//
typedef struct _BTH_HCI_EVENT_INFO {
    //
    // Remote radio address which the HCI event is associated with
    //
    BTH_ADDR bthAddress;

    //
    // HCI_CONNNECTION_TYPE_XXX value
    //
    UCHAR connectionType;

    //
    // If != 0, then the underlying connection to the remote radio has just
    // been estrablished.  If == 0, then the underlying conneciton has just been
    // destroyed.
    //
    UCHAR  connected;

} BTH_HCI_EVENT_INFO, *PBTH_HCI_EVENT_INFO;

//
// Support added in KB942567
//
#if (NTDDI_VERSION > NTDDI_VISTASP1 || \
    (NTDDI_VERSION == NTDDI_VISTASP1 && defined(VISTA_KB942567)))

typedef enum _IO_CAPABILITY {
    IoCaps_DisplayOnly     = 0x00,
    IoCaps_DisplayYesNo    = 0x01,
    IoCaps_KeyboardOnly    = 0x02,
    IoCaps_NoInputNoOutput = 0x03,
    IoCaps_Undefined       = 0xff
}IO_CAPABILITY;

typedef enum _AUTHENTICATION_REQUIREMENTS {
    MITMProtectionNotRequired               = 0x00,
    MITMProtectionRequired                  = 0x01,
    MITMProtectionNotRequiredBonding        = 0x02,
    MITMProtectionRequiredBonding           = 0x03,
    MITMProtectionNotRequiredGeneralBonding = 0x04,
    MITMProtectionRequiredGeneralBonding    = 0x05,
    MITMProtectionNotDefined                = 0xff
} AUTHENTICATION_REQUIREMENTS;

#define IsMITMProtectionRequired(requirements) \
        ((MITMProtectionRequired == requirements) || (MITMProtectionRequiredBonding == requirements) || (MITMProtectionRequiredGeneralBonding == requirements))

#endif // >= SP1+KB942567

//
// Max length we allow for ServiceName in the remote SDP records
//
#define BTH_MAX_SERVICE_NAME_SIZE   (256)

#define MAX_UUIDS_IN_QUERY  (12)

#define BTH_VID_DEFAULT_VALUE                   (0xFFFF)

#define SDP_ERROR_INVALID_SDP_VERSION           (0x0001)
#define SDP_ERROR_INVALID_RECORD_HANDLE         (0x0002)
#define SDP_ERROR_INVALID_REQUEST_SYNTAX        (0x0003)
#define SDP_ERROR_INVALID_PDU_SIZE              (0x0004)
#define SDP_ERROR_INVALID_CONTINUATION_STATE    (0x0005)
#define SDP_ERROR_INSUFFICIENT_RESOURCES        (0x0006)

//
// Defined by windows to handle server errors that are not described by the
// above errors.  Start at 0x0100 so we don't go anywhere near the spec
// defined values.
//

//
// Success, nothing went wrong
//
#define SDP_ERROR_SUCCESS                       ((SDP_ERROR) 0x0000)

//
// The SDP PDU or parameters other than the SDP stream response was not correct
//
#define SDP_ERROR_SERVER_INVALID_RESPONSE       ((SDP_ERROR) 0x0100)

//
// The SDP response stream did not parse correctly.
//
#define SDP_ERROR_SERVER_RESPONSE_DID_NOT_PARSE ((SDP_ERROR) 0x0200)

//
// The SDP response stream was successfully parsed, but did not match the
// required format for the query.
//
#define SDP_ERROR_SERVER_BAD_FORMAT             ((SDP_ERROR) 0x0300)

//
// SDP was unable to send a continued query back to the server
//
#define SDP_ERROR_COULD_NOT_SEND_CONTINUE       ((SDP_ERROR) 0x0400)

//
// Server sent a response that was too large to fit in the caller's buffer.
//
#define SDP_ERROR_RESPONSE_TOO_LARGE            ((SDP_ERROR) 0x0500)

#define SDP_ATTRIB_RECORD_HANDLE            (0x0000)
#define SDP_ATTRIB_CLASS_ID_LIST            (0x0001)
#define SDP_ATTRIB_RECORD_STATE             (0x0002)
#define SDP_ATTRIB_SERVICE_ID               (0x0003)
#define SDP_ATTRIB_PROTOCOL_DESCRIPTOR_LIST (0x0004)
#define SDP_ATTRIB_BROWSE_GROUP_LIST        (0x0005)
#define SDP_ATTRIB_LANG_BASE_ATTRIB_ID_LIST (0x0006)
#define SDP_ATTRIB_INFO_TIME_TO_LIVE        (0x0007)
#define SDP_ATTRIB_AVAILABILITY             (0x0008)
#define SDP_ATTRIB_PROFILE_DESCRIPTOR_LIST  (0x0009)
#define SDP_ATTRIB_DOCUMENTATION_URL        (0x000A)
#define SDP_ATTRIB_CLIENT_EXECUTABLE_URL    (0x000B)
#define SDP_ATTRIB_ICON_URL                 (0x000C)
#define SDP_ATTRIB_ADDITIONAL_PROTOCOL_DESCRIPTOR_LIST \
                                            (0x000D)

//
// Attribute IDs in the range of 0x000D - 0x01FF are reserved for future use
//
#define SDP_ATTRIB_PROFILE_SPECIFIC                     (0x0200)

#define LANG_BASE_LANGUAGE_INDEX                        (0x0000)
#define LANG_BASE_ENCODING_INDEX                        (0x0001)
#define LANG_BASE_OFFSET_INDEX                          (0x0002)
#define LANG_DEFAULT_ID                                 (0x0100)

#define LANGUAGE_EN_US                                  (0x656E)
#define ENCODING_UTF_8                                  (0x006A)

#define STRING_NAME_OFFSET                              (0x0000)
#define STRING_DESCRIPTION_OFFSET                       (0x0001)
#define STRING_PROVIDER_NAME_OFFSET                     (0x0002)

#define SDP_ATTRIB_SDP_VERSION_NUMBER_LIST              (0x0200)
#define SDP_ATTRIB_SDP_DATABASE_STATE                   (0x0201)

#define SDP_ATTRIB_BROWSE_GROUP_ID                      (0x0200)

#define SDP_ATTRIB_CORDLESS_EXTERNAL_NETWORK            (0x0301)

#define SDP_ATTRIB_FAX_CLASS_1_SUPPORT                  (0x0302)
#define SDP_ATTRIB_FAX_CLASS_2_0_SUPPORT                (0x0303)
#define SDP_ATTRIB_FAX_CLASS_2_SUPPORT                  (0x0304)
#define SDP_ATTRIB_FAX_AUDIO_FEEDBACK_SUPPORT           (0x0305)

#define SDP_ATTRIB_HEADSET_REMOTE_AUDIO_VOLUME_CONTROL  (0x0302)

#define SDP_ATTRIB_LAN_LPSUBNET                         (0x0200)

#define SDP_ATTRIB_OBJECT_PUSH_SUPPORTED_FORMATS_LIST   (0x0303)

#define SDP_ATTRIB_SYNCH_SUPPORTED_DATA_STORES_LIST     (0x0301)

//  this is in the assigned numbers doc, but it does not show up in any profile
#define SDP_ATTRIB_SERVICE_VERSION                      (0x0300)

#define SDP_ATTRIB_PAN_NETWORK_ADDRESS                  (0x0306)
#define SDP_ATTRIB_PAN_WAP_GATEWAY                      (0x0307)
#define SDP_ATTRIB_PAN_HOME_PAGE_URL                    (0x0308)
#define SDP_ATTRIB_PAN_WAP_STACK_TYPE                   (0x0309)
#define SDP_ATTRIB_PAN_SECURITY_DESCRIPTION             (0x030A)
#define SDP_ATTRIB_PAN_NET_ACCESS_TYPE                  (0x030B)
#define SDP_ATTRIB_PAN_MAX_NET_ACCESS_RATE              (0x030C)

#define SDP_ATTRIB_IMAGING_SUPPORTED_CAPABILITIES       (0x0310)
#define SDP_ATTRIB_IMAGING_SUPPORTED_FEATURES           (0x0311)
#define SDP_ATTRIB_IMAGING_SUPPORTED_FUNCTIONS          (0x0312)
#define SDP_ATTRIB_IMAGING_TOTAL_DATA_CAPACITY          (0x0313)

#define SDP_ATTRIB_DI_SPECIFICATION_ID                  (0x0200)
#define SDP_ATTRIB_DI_VENDOR_ID                         (0x0201)
#define SDP_ATTRIB_DI_PRODUCT_ID                        (0x0202)
#define SDP_ATTRIB_DI_VERSION                           (0x0203)
#define SDP_ATTRIB_DI_PRIMARY_RECORD                    (0x0204)
#define SDP_ATTRIB_DI_VENDOR_ID_SOURCE                  (0x0205)

#define SDP_ATTRIB_HID_DEVICE_RELEASE_NUMBER            (0x0200)
#define SDP_ATTRIB_HID_PARSER_VERSION                   (0x0201)
#define SDP_ATTRIB_HID_DEVICE_SUBCLASS                  (0x0202)
#define SDP_ATTRIB_HID_COUNTRY_CODE                     (0x0203)
#define SDP_ATTRIB_HID_VIRTUAL_CABLE                    (0x0204)
#define SDP_ATTRIB_HID_RECONNECT_INITIATE               (0x0205)
#define SDP_ATTRIB_HID_DESCRIPTOR_LIST                  (0x0206)
#define SDP_ATTRIB_HID_LANG_ID_BASE_LIST                (0x0207)
#define SDP_ATTRIB_HID_SDP_DISABLE                      (0x0208)
#define SDP_ATTRIB_HID_BATTERY_POWER                    (0x0209)
#define SDP_ATTRIB_HID_REMOTE_WAKE                      (0x020A)
#define SDP_ATTRIB_HID_PROFILE_VERSION                  (0x020B)
#define SDP_ATTRIB_HID_SUPERVISION_TIMEOUT              (0x020C)
#define SDP_ATTRIB_HID_NORMALLY_CONNECTABLE             (0x020D)
#define SDP_ATTRIB_HID_BOOT_DEVICE                      (0x020E)
#define SDP_ATTRIB_HID_SSR_HOST_MAX_LATENCY             (0x020F)
#define SDP_ATTRIB_HID_SSR_HOST_MIN_TIMEOUT             (0x0210)

#define SDP_ATTRIB_A2DP_SUPPORTED_FEATURES              (0x0311)

#define SDP_ATTRIB_AVRCP_SUPPORTED_FEATURES             (0x0311)

#define SDP_ATTRIB_HFP_SUPPORTED_FEATURES               (0x0311)

//
// Profile specific values
//
#define AVRCP_SUPPORTED_FEATURES_CATEGORY_1                         (0x0001)
#define AVRCP_SUPPORTED_FEATURES_CATEGORY_2                         (0x0002)
#define AVRCP_SUPPORTED_FEATURES_CATEGORY_3                         (0x0004)
#define AVRCP_SUPPORTED_FEATURES_CATEGORY_4                         (0x0008)
#define AVRCP_SUPPORTED_FEATURES_CT_BROWSING                        (0x0040)
#define AVRCP_SUPPORTED_FEATURES_CT_COVER_ART_IMAGE_PROPERTIES      (0X0080)
#define AVRCP_SUPPORTED_FEATURES_CT_COVER_ART_IMAGE                 (0X0100)
#define AVRCP_SUPPORTED_FEATURES_CT_COVER_ART_LINKED_THUMBNAIL      (0x0200)
#define AVRCP_SUPPORTED_FEATURES_TG_PLAYER_APPLICATION_SETTINGS     (0x0010)
#define AVRCP_SUPPORTED_FEATURES_TG_GROUP_NAVIGATION                (0x0020)
#define AVRCP_SUPPORTED_FEATURES_TG_BROWSING                        (0x0040)
#define AVRCP_SUPPORTED_FEATURES_TG_MULTIPLE_PLAYER_APPLICATIONS    (0x0080)
#define AVRCP_SUPPORTED_FEATURES_TG_COVER_ART                       (0x0100)

#define A2DP_SINK_SUPPORTED_FEATURES_HEADPHONE      (0x0001)
#define A2DP_SINK_SUPPORTED_FEATURES_SPEAKER        (0x0002)
#define A2DP_SINK_SUPPORTED_FEATURES_RECORDER       (0x0004)
#define A2DP_SINK_SUPPORTED_FEATURES_AMPLIFIER      (0x0008)

#define A2DP_SOURCE_SUPPORTED_FEATURES_PLAYER       (0x0001)
#define A2DP_SOURCE_SUPPORTED_FEATURES_MICROPHONE   (0x0002)
#define A2DP_SOURCE_SUPPORTED_FEATURES_TUNER        (0x0004)
#define A2DP_SOURCE_SUPPORTED_FEATURES_MIXER        (0x0008)

#define CORDLESS_EXTERNAL_NETWORK_PSTN              (0x01)
#define CORDLESS_EXTERNAL_NETWORK_ISDN              (0x02)
#define CORDLESS_EXTERNAL_NETWORK_GSM               (0x03)
#define CORDLESS_EXTERNAL_NETWORK_CDMA              (0x04)
#define CORDLESS_EXTERNAL_NETWORK_ANALOG_CELLULAR   (0x05)
#define CORDLESS_EXTERNAL_NETWORK_PACKET_SWITCHED   (0x06)
#define CORDLESS_EXTERNAL_NETWORK_OTHER             (0x07)

#define OBJECT_PUSH_FORMAT_VCARD_2_1                (0x01)
#define OBJECT_PUSH_FORMAT_VCARD_3_0                (0x02)
#define OBJECT_PUSH_FORMAT_VCAL_1_0                 (0x03)
#define OBJECT_PUSH_FORMAT_ICAL_2_0                 (0x04)
#define OBJECT_PUSH_FORMAT_VNOTE                    (0x05)
#define OBJECT_PUSH_FORMAT_VMESSAGE                 (0x06)
#define OBJECT_PUSH_FORMAT_ANY                      (0xFF)

#define SYNCH_DATA_STORE_PHONEBOOK                  (0x01)
#define SYNCH_DATA_STORE_CALENDAR                   (0x03)
#define SYNCH_DATA_STORE_NOTES                      (0x05)
#define SYNCH_DATA_STORE_MESSAGES                   (0x06)

#define DI_VENDOR_ID_SOURCE_BLUETOOTH_SIG           (0x0001)
#define DI_VENDOR_ID_SOURCE_USB_IF                  (0x0002)

#define PSM_SDP                 (0x0001)
#define PSM_RFCOMM              (0x0003)
#define PSM_TCS_BIN             (0x0005)
#define PSM_TCS_BIN_CORDLESS    (0x0007)
#define PSM_BNEP                (0x000F)
#define PSM_HID_CONTROL         (0x0011)
#define PSM_HID_INTERRUPT       (0x0013)
#define PSM_UPNP                (0x0015)
#define PSM_AVCTP               (0x0017)
#define PSM_AVDTP               (0x0019)
#define PSM_AVCTP_BROWSE        (0x001B)
#define PSM_UDI_C_PLANE         (0x001D)
#define PSM_ATT                 (0x001F)
#define PSM_3DSP                (0x0021)
#define PSM_LE_IPSP             (0x0023)

//
// Strings
//
#define STR_ADDR_FMTA                   "(%02x:%02x:%02x:%02x:%02x:%02x)"
#define STR_ADDR_FMTW                   L"(%02x:%02x:%02x:%02x:%02x:%02x)"

#define STR_ADDR_SHORT_FMTA             "%04x%08x"
#define STR_ADDR_SHORT_FMTW             L"%04x%08x"

#define STR_USBHCI_CLASS_HARDWAREIDA    "USB\\Class_E0&SubClass_01&Prot_01"
#define STR_USBHCI_CLASS_HARDWAREIDW    L"USB\\Class_E0&SubClass_01&Prot_01"

#if defined(UNICODE) || defined(BTH_KERN)

#define STR_ADDR_FMT                    STR_ADDR_FMTW
#define STR_ADDR_SHORT_FMT              STR_ADDR_SHORT_FMTW

#define STR_USBHCI_CLASS_HARDWAREID     STR_USBHCI_CLASS_HARDWAREIDW

#else // UNICODE

#define STR_ADDR_FMT                    STR_ADDR_FMTA
#define STR_ADDR_SHORT_FMT              STR_ADDR_SHORT_FMTA

#define STR_USBHCI_CLASS_HARDWAREID     STR_USBHCI_CLASS_HARDWAREIDA

#endif // UNICODE

#define GET_BITS(field,offset,mask)         ( ( (field) >> (offset) ) & (mask) )
#define GET_BIT(field,offset)               ( GET_BITS(field,offset,0x1) )

#define LMP_3_SLOT_PACKETS(x)               (GET_BIT(x, 0))
#define LMP_5_SLOT_PACKETS(x)               (GET_BIT(x, 1))
#define LMP_ENCRYPTION(x)                   (GET_BIT(x, 2))
#define LMP_SLOT_OFFSET(x)                  (GET_BIT(x, 3))
#define LMP_TIMING_ACCURACY(x)              (GET_BIT(x, 4))
#define LMP_SWITCH(x)                       (GET_BIT(x, 5))
#define LMP_HOLD_MODE(x)                    (GET_BIT(x, 6))
#define LMP_SNIFF_MODE(x)                   (GET_BIT(x, 7))
#define LMP_PARK_MODE(x)                    (GET_BIT(x, 8))
#define LMP_RSSI(x)                         (GET_BIT(x, 9))
#define LMP_CHANNEL_QUALITY_DRIVEN_MODE(x)  (GET_BIT(x,10))
#define LMP_SCO_LINK(x)                     (GET_BIT(x,11))
#define LMP_HV2_PACKETS(x)                  (GET_BIT(x,12))
#define LMP_HV3_PACKETS(x)                  (GET_BIT(x,13))
#define LMP_MU_LAW_LOG(x)                   (GET_BIT(x,14))
#define LMP_A_LAW_LOG(x)                    (GET_BIT(x,15))
#define LMP_CVSD(x)                         (GET_BIT(x,16))
#define LMP_PAGING_SCHEME(x)                (GET_BIT(x,17))
#define LMP_POWER_CONTROL(x)                (GET_BIT(x,18))
#define LMP_TRANSPARENT_SCO_DATA(x)         (GET_BIT(x,19))
#define LMP_FLOW_CONTROL_LAG(x)             (GET_BITS(x,20,0x3))
#define LMP_BROADCAST_ENCRYPTION(x)         (GET_BIT(x,23))
#define LMP_ENHANCED_DATA_RATE_ACL_2MBPS_MODE(x) (GET_BIT(x,25))
#define LMP_ENHANCED_DATA_RATE_ACL_3MBPS_MODE(x) (GET_BIT(x,26))
#define LMP_ENHANCED_INQUIRY_SCAN(x)        (GET_BIT(x,27))
#define LMP_INTERLACED_INQUIRY_SCAN(x)      (GET_BIT(x,28))
#define LMP_INTERLACED_PAGE_SCAN(x)         (GET_BIT(x,29))
#define LMP_RSSI_WITH_INQUIRY_RESULTS(x)    (GET_BIT(x,30))
#define LMP_ESCO_LINK(x)                    (GET_BIT(x,31))
#define LMP_EV4_PACKETS(x)                  (GET_BIT(x,32))
#define LMP_EV5_PACKETS(x)                  (GET_BIT(x,33))
#define LMP_AFH_CAPABLE_SLAVE(x)            (GET_BIT(x,35))
#define LMP_AFH_CLASSIFICATION_SLAVE(x)     (GET_BIT(x,36))
#define LMP_BR_EDR_NOT_SUPPORTED(x)         (GET_BIT(x,37))
#define LMP_LE_SUPPORTED(x)                 (GET_BIT(x,38))
#define LMP_3SLOT_EDR_ACL_PACKETS(x)        (GET_BIT(x,39))
#define LMP_5SLOT_EDR_ACL_PACKETS(x)        (GET_BIT(x,40))
#define LMP_SNIFF_SUBRATING(x)              (GET_BIT(x,41))
#define LMP_PAUSE_ENCRYPTION(x)             (GET_BIT(x,42))
#define LMP_AFH_CAPABLE_MASTER(x)           (GET_BIT(x,43))
#define LMP_AFH_CLASSIFICATION_MASTER(x)    (GET_BIT(x,44))
#define LMP_EDR_ESCO_2MBPS_MODE(x)          (GET_BIT(x,45))
#define LMP_EDR_ESCO_3MBPS_MODE(x)          (GET_BIT(x,46))
#define LMP_3SLOT_EDR_ESCO_PACKETS(x)       (GET_BIT(x,47))
#define LMP_EXTENDED_INQUIRY_RESPONSE(x)    (GET_BIT(x,48))
#define LMP_SIMULT_LE_BR_TO_SAME_DEV(x)     (GET_BIT(x,49))
#define LMP_SECURE_SIMPLE_PAIRING(x)        (GET_BIT(x,51))
#define LMP_ENCAPSULATED_PDU(x)             (GET_BIT(x,52))
#define LMP_ERRONEOUS_DATA_REPORTING(x)     (GET_BIT(x,53))
#define LMP_NON_FLUSHABLE_PACKET_BOUNDARY_FLAG(x) (GET_BIT(x,54))
#define LMP_LINK_SUPERVISION_TIMEOUT_CHANGED_EVENT(x) (GET_BIT(x,56))
#define LMP_INQUIRY_RESPONSE_TX_POWER_LEVEL(x)(GET_BIT(x,57))
#define LMP_EXTENDED_FEATURES(x)            (GET_BIT(x,63))

#endif // GUID_DEFS_ONLY

#endif // (NTDDI_VERSION >= NTDDI_WINXPSP2)

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __BTHDEF_H__

