/*++

Copyright (c) 2007 Microsoft Corporation

Module Name:

    cperguid.h

Abstract:

    This header file defines the GUID constants for the Common Platform Error
    Record as defined in Appendix N of the Unified Extensible Firmware Interface
    (UEFI) specification (revision 2.1).

    This specification as well as any approved errata may be obtained from
    http://www.uefi.org.

    This header file also includes Microsoft specific extensions to the Common
    Platform Error Record as allowed by Appendix N, Section 2.3 of the Unified
    Extensible Firmware Interface specification (Non-standard Section Body).

--*/
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//-------------------------------------- Standard Error Notification Type GUIDs

/* 2dce8bb1-bdd7-450e-b9ad-9cf4ebd4f890 */
DEFINE_GUID(CMC_NOTIFY_TYPE_GUID,
            0x2dce8bb1, 0xbdd7, 0x450e, 0xb9, 0xad,
            0x9c, 0xf4, 0xeb, 0xd4, 0xf8, 0x90);

/* 4e292f96-d843-4a55-a8c2-d481f27ebeee */
DEFINE_GUID(CPE_NOTIFY_TYPE_GUID,
            0x4e292f96, 0xd843, 0x4a55, 0xa8, 0xc2,
            0xd4, 0x81, 0xf2, 0x7e, 0xbe, 0xee);

/* e8f56ffe-919c-4cc5-ba88-65abe14913bb */
DEFINE_GUID(MCE_NOTIFY_TYPE_GUID,
            0xe8f56ffe, 0x919c, 0x4cc5, 0xba, 0x88,
            0x65, 0xab, 0xe1, 0x49, 0x13, 0xbb);

/* cf93c01f-1a16-4dfc-b8bc-9c4daf67c104 */
DEFINE_GUID(PCIe_NOTIFY_TYPE_GUID,
            0xcf93c01f, 0x1a16, 0x4dfc, 0xb8, 0xbc,
            0x9c, 0x4d, 0xaf, 0x67, 0xc1, 0x04);

/* cc5263e8-9308-454a-89d0-340bd39bc98e */
DEFINE_GUID(INIT_NOTIFY_TYPE_GUID,
            0xcc5263e8, 0x9308, 0x454a, 0x89, 0xd0,
            0x34, 0x0b, 0xd3, 0x9b, 0xc9, 0x8e);

/* 5bad89ff-b7e6-42c9-814a-cf2485d6e98a */
DEFINE_GUID(NMI_NOTIFY_TYPE_GUID,
            0x5bad89ff, 0xb7e6, 0x42c9, 0x81, 0x4a,
            0xcf, 0x24, 0x85, 0xd6, 0xe9, 0x8a);

/* 3d61a466-ab40-409a-a698-f362d464b38f */
DEFINE_GUID(BOOT_NOTIFY_TYPE_GUID,
            0x3d61a466, 0xab40, 0x409a, 0xa6, 0x98,
            0xf3, 0x62, 0xd4, 0x64, 0xb3, 0x8f);

/* 9a78788a-bbe8-11e4-809e-67611e5d46b0 */
DEFINE_GUID(SEA_NOTIFY_TYPE_GUID,
            0x9a78788a, 0xbbe8, 0x11e4, 0x80, 0x9e,
            0x67, 0x61, 0x1e, 0x5d, 0x46, 0xb0);

/* 5c284c81-b0ae-4e87-a322-b04c85624323 */
DEFINE_GUID(SEI_NOTIFY_TYPE_GUID,
            0x5c284c81, 0xb0ae, 0x4e87, 0xa3, 0x22,
            0xb0, 0x4c, 0x85, 0x62, 0x43, 0x23);

/* 09a9d5ac-5204-4214-96e5-94992e752bcd */
DEFINE_GUID(PEI_NOTIFY_TYPE_GUID,
            0x09a9D5ac, 0x5204, 0x4214, 0x96, 0xe5,
            0x94, 0x99, 0x2e, 0x75, 0x2b, 0xcd);

/* 487565ba-6494-4367-95ca-4eff893522f6 */
DEFINE_GUID(BMC_NOTIFY_TYPE_GUID,
            0x487565ba, 0x6494, 0x4367, 0x95, 0xca,
            0x4e, 0xff, 0x89, 0x35, 0x22, 0xf6);

/* e9d59197-94ee-4a4f-8ad8-9b7d8bd93d2e */
DEFINE_GUID(SCI_NOTIFY_TYPE_GUID,
            0xe9d59197, 0x94ee, 0x4a4f, 0x8a, 0xd8,
            0x9b, 0x7d, 0x8b, 0xd9, 0x3d, 0x2e);

/* fe84086e-b557-43cf-ac1b-17982e078470 */
DEFINE_GUID(EXTINT_NOTIFY_TYPE_GUID,
            0xfe84086e, 0xb557, 0x43cf, 0xac, 0x1b,
            0x17, 0x98, 0x2e, 0x07, 0x84, 0x70);

/* 0033f803-2e70-4e88-992c-6f26daf3db7a */
DEFINE_GUID(DEVICE_DRIVER_NOTIFY_TYPE_GUID,
            0x0033f803, 0x2e70, 0x4e88, 0x99, 0x2c,
            0x6f, 0x26, 0xda, 0xf3, 0xdb, 0x7a);

/* 919448b2-3739-4b7f-a8f1-e0062805c2a3 */
DEFINE_GUID(CMCI_NOTIFY_TYPE_GUID,
            0x919448b2, 0x3739, 0x4b7f, 0xa8, 0xf1,
            0xe0, 0x06, 0x28, 0x05, 0xc2, 0xa3);

//------------------------------------------- Summary Error Section type GUIDs

/* 990b31e9-541a-4db0-a42f-837d344f6923 */
DEFINE_GUID(WHEA_DEVICE_ERROR_SUMMARY_GUID,
            0x990b31e9, 0x541a, 0x4db0, 0xa4, 0x2f,
            0x83, 0x7d, 0x34, 0x4f, 0x69, 0x23);
            
//------------------------------------------- Standard Error Section type GUIDs

/* 9876ccad-47b4-4bdb-b65e-16f193c4f3db */
DEFINE_GUID(PROCESSOR_GENERIC_ERROR_SECTION_GUID,
            0x9876ccad, 0x47b4, 0x4bdb, 0xb6, 0x5e,
            0x16, 0xf1, 0x93, 0xc4, 0xf3, 0xdb);

/* dc3ea0b0-a144-4797-b95b-53fa242b6e1d */
DEFINE_GUID(XPF_PROCESSOR_ERROR_SECTION_GUID,
            0xdc3ea0b0, 0xa144, 0x4797, 0xb9, 0x5b,
            0x53, 0xfa, 0x24, 0x2b, 0x6e, 0x1d);

/* e429faf1-3cb7-11d4-bca7-0080c73c8881 */
DEFINE_GUID(IPF_PROCESSOR_ERROR_SECTION_GUID,
            0xe429faf1, 0x3cb7, 0x11d4, 0xbc, 0xa7,
            0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81);

/* e19e3d16-bc11-11e4-9caa-c2051d5d46b0 */
DEFINE_GUID(ARM_PROCESSOR_ERROR_SECTION_GUID,
            0xe19e3d16, 0xbc11, 0x11e4, 0x9c, 0xaa,
            0xc2, 0x05, 0x1d, 0x5d, 0x46, 0xb0);

/* a5bc1114-6f64-4ede-b863-3e83ed7c83b1 */
DEFINE_GUID(MEMORY_ERROR_SECTION_GUID,
            0xa5bc1114, 0x6f64, 0x4ede, 0xb8, 0x63,
            0x3e, 0x83, 0xed, 0x7c, 0x83, 0xb1);

/* d995e954-bbc1-430f-ad91-b44dcb3c6f35 */
DEFINE_GUID(PCIEXPRESS_ERROR_SECTION_GUID,
            0xd995e954, 0xbbc1, 0x430f, 0xad, 0x91,
            0xb4, 0x4d, 0xcb, 0x3c, 0x6f, 0x35);

/* c5753963-3b84-4095-bf78-eddad3f9c9dd */
DEFINE_GUID(PCIXBUS_ERROR_SECTION_GUID,
            0xc5753963, 0x3b84, 0x4095, 0xbf, 0x78,
            0xed, 0xda, 0xd3, 0xf9, 0xc9, 0xdd);

/* eb5e4685-ca66-4769-b6a2-26068b001326 */
DEFINE_GUID(PCIXDEVICE_ERROR_SECTION_GUID,
            0xeb5e4685, 0xca66, 0x4769, 0xb6, 0xa2,
            0x26, 0x06, 0x8b, 0x00, 0x13, 0x26);

/* 81212a96-09ed-4996-9471-8d729c8e69ed */
DEFINE_GUID(FIRMWARE_ERROR_RECORD_REFERENCE_GUID,
            0x81212a96, 0x09ed, 0x4996, 0x94, 0x71,
            0x8d, 0x72, 0x9c, 0x8e, 0x69, 0xed);

/* 81687003-dbfd-4728-9ffd-f0904f97597d */
DEFINE_GUID(PMEM_ERROR_SECTION_GUID,
            0x81687003, 0xdbfd, 0x4728, 0x9f, 0xfd,
            0xf0, 0x90, 0x4f, 0x97, 0x59, 0x7d);

/* 85183a8b-9c41-429c-939c-5c3c087ca280 */
DEFINE_GUID(MU_TELEMETRY_SECTION_GUID,
            0x85183a8b, 0x9c41, 0x429c, 0x93, 0x9c,
            0x5c, 0x3c, 0x08, 0x7c, 0xa2, 0x80);

/* c34832a1-02c3-4c52-a9f1-9f1d5d7723fc */
DEFINE_GUID(RECOVERY_INFO_SECTION_GUID,
            0xc34832a1, 0x02c3, 0x4c52, 0xa9, 0xf1,
            0x9f, 0x1d, 0x5d, 0x77, 0x23, 0xfc);

//-------------------------------------- Processor check information type GUIDs

/* a55701f5-e3ef-43de-ac72-249b573fad2c */
DEFINE_GUID(WHEA_CACHECHECK_GUID,
            0xa55701f5, 0xe3ef, 0x43de, 0xac, 0x72,
            0x24, 0x9b, 0x57, 0x3f, 0xad, 0x2c);

/* fc06b535-5e1f-4562-9f25-0a3b9adb63c3 */
DEFINE_GUID(WHEA_TLBCHECK_GUID,
            0xfc06b535, 0x5e1f, 0x4562, 0x9f, 0x25,
            0x0a, 0x3b, 0x9a, 0xdb, 0x63, 0xc3);

/* 1cf3f8b3-c5b1-49a2-aa59-5eef92ffa63c */
DEFINE_GUID(WHEA_BUSCHECK_GUID,
            0x1cf3f8b3, 0xc5b1, 0x49a2, 0xaa, 0x59,
            0x5e, 0xef, 0x92, 0xff, 0xa6, 0x3c);

/* 48ab7f57-dc34-4f6c-a7d3-b0b5b0a74314 */
DEFINE_GUID(WHEA_MSCHECK_GUID,
            0x48ab7f57, 0xdc34, 0x4f6c, 0xa7, 0xd3,
            0xb0, 0xb5, 0xb0, 0xa7, 0x43, 0x14);

//
// This is the start of the Microsoft specific extensions to the Common Platform
// Error Record specification. This is in accordance with Appendix N, section
// 2.3 of the Unified Extensible Firmware Interface specification, which allows
// the specification of non-standard section bodies.
//

//---------------------------------------------------- Empty GUID

/* 00000000-0000-0000-0000-00000000000 */
DEFINE_GUID(CPER_EMPTY_GUID, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

//---------------------------------------------------- Microsoft record creator

/* cf07c4bd-b789-4e18-b3c4-1f732cb57131 */
DEFINE_GUID(WHEA_RECORD_CREATOR_GUID,
            0xcf07c4bd,
            0xb789, 0x4e18,
            0xb3, 0xc4, 0x1f, 0x73, 0x2c, 0xb5, 0x71, 0x31);

//---------------------------------------------------- device driver record creator

/* 57217c8d-5e66-44fb-8033-9b74cacedf5b */
DEFINE_GUID(DEFAULT_DEVICE_DRIVER_CREATOR_GUID,
            0x57217c8d,
            0x5e66, 0x44fb,
            0x80, 0x33, 0x9b, 0x74, 0xca, 0xce, 0xdf, 0x5b);

//--------------------------------------- Microsoft specific notification types

/* 3e62a467-ab40-409a-a698-f362d464b38f */
DEFINE_GUID(GENERIC_NOTIFY_TYPE_GUID,
            0x3e62a467,
            0xab40, 0x409a,
            0xa6, 0x98, 0xf3, 0x62, 0xd4, 0x64, 0xb3, 0x8f);

//-------------------------------------- Microsoft specific error section types

/* 6f3380d1-6eb0-497f-a578-4d4c65a71617 */
DEFINE_GUID(IPF_SAL_RECORD_SECTION_GUID,
            0x6f3380d1,
            0x6eb0, 0x497f,
            0xa5, 0x78, 0x4d, 0x4c, 0x65, 0xa7, 0x16, 0x17);

/* 8a1e1d01-42f9-4557-9c33-565e5cc3f7e8 */
DEFINE_GUID(XPF_MCA_SECTION_GUID,
            0x8a1e1d01,
            0x42f9, 0x4557,
            0x9c, 0x33, 0x56, 0x5e, 0x5c, 0xc3, 0xf7, 0xe8);

/* e71254e7-c1b9-4940-ab76-909703a4320f */
DEFINE_GUID(NMI_SECTION_GUID,
            0xe71254e7,
            0xc1b9, 0x4940,
            0xab, 0x76, 0x90, 0x97, 0x03, 0xa4, 0x32, 0x0f);

/* e71254e8-c1b9-4940-ab76-909703a4320f */
DEFINE_GUID(GENERIC_SECTION_GUID,
            0xe71254e8,
            0xc1b9, 0x4940,
            0xab, 0x76, 0x90, 0x97, 0x03, 0xa4, 0x32, 0x0f);

/* 1c15b445-9b06-4667-ac25-33c056b88803 */
DEFINE_GUID(IPMI_MSR_DUMP_SECTION_GUID,
            0x1c15b445,
            0x9b06, 0x4667,
            0xac, 0x25, 0x33, 0xc0, 0x56, 0xb8, 0x88, 0x03);

/* e71254e9-c1b9-4940-ab76-909703a4320f */
DEFINE_GUID(WHEA_ERROR_PACKET_SECTION_GUID,
            0xe71254e9,
            0xc1b9, 0x4940,
            0xab, 0x76, 0x90, 0x97, 0x03, 0xa4, 0x32, 0x0f);

/* ec49534b-30e7-4358-972f-eca6958fae3b */
DEFINE_GUID(WHEA_DPC_CAPABILITY_SECTION_GUID,
            0xec49534b,
            0x30e7, 0x4358,
            0x97, 0x2f, 0xec, 0xa6, 0x95, 0x8f, 0xae, 0x3b);

/* e96eca99-53e2-4f52-9be7-d2dbe9508ed0 */
DEFINE_GUID(PCIE_CORRECTABLE_ERROR_SUMMARY_SECTION_GUID,
            0xe96eca99,
            0x53e2, 0x4f52,
            0x9b, 0xe7, 0xd2, 0xdb, 0xe9, 0x50, 0x8e, 0xd0);

/* 0e36c93e-ca15-4a83-ba8a-cbe80f7f0017 */
DEFINE_GUID(MEMORY_CORRECTABLE_ERROR_SUMMARY_SECTION_GUID,
            0x0e36c93e,
            0xca15, 0x4a83,
            0xba, 0x8a, 0xcb, 0xe8, 0x0f, 0x7f, 0x00, 0x17);

/* f5fe48a6-84ce-4c1e-aa64-20c9a53099f1 */
DEFINE_GUID(SEA_SECTION_GUID,
            0xf5fe48a6, 0x84ce, 0x4c1e, 0xaa, 0x64,
            0x20, 0xc9, 0xa5, 0x30, 0x99, 0xf1);

/* f2a4a152-9c6d-4020-aecf-7695b389251b */
DEFINE_GUID(SEI_SECTION_GUID,
            0xf2a4a152, 0x9c6d, 0x4020, 0xae, 0xcf,
            0x76, 0x95, 0xb3, 0x89, 0x25, 0x1b);

/* dd060800-f6e1-4204-ac27-c4bca9568402 */
DEFINE_GUID(PCI_RECOVERY_SECTION_GUID,
            0xdd060800, 0xf6e1, 0x4204, 0xac, 0x27, 
            0xc4, 0xbc, 0xa9, 0x56, 0x84, 0x02);

/* e3ebf4a2-df50-4708-b2d7-0b29ec2f7aa9 */
DEFINE_GUID(ARM_RAS_NODE_SECTION_GUID,
            0xe3ebf4a2, 0xdf50, 0x4708, 0xb2, 0xd7,
            0x0b, 0x29, 0xec, 0x2f, 0x7a, 0xa9);

/* e16edb28-6113-4263-a41d-e53f8de78751 */
DEFINE_GUID(MEMORY_ERROR_EXT_SECTION_INTEL_GUID,
            0xe16edb28, 0x6113, 0x4263, 0xa4, 0x1d,
            0xe5, 0x3f, 0x8d, 0xe7, 0x87, 0x51);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

