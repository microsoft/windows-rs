/*
   Copyright (c) Microsoft Corporation

   SYNOPSIS

     Declares the management portion of the FWP API.
*/
#include <winapifamily.h>

#pragma region Desktop Family or AppRuntime Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_APPRUNTIME)



#if (NTDDI_VERSION >= NTDDI_WIN6)

///////////////////////////////////////////////////////////////////////////////
//
// GUIDs for built-in layers.
//
///////////////////////////////////////////////////////////////////////////////

// c86fd1bf-21cd-497e-a0bb-17425c885c58
DEFINE_GUID(
   FWPM_LAYER_INBOUND_IPPACKET_V4,
   0xc86fd1bf,
   0x21cd,
   0x497e,
   0xa0, 0xbb, 0x17, 0x42, 0x5c, 0x88, 0x5c, 0x58
);

// b5a230d0-a8c0-44f2-916e-991b53ded1f7
DEFINE_GUID(
   FWPM_LAYER_INBOUND_IPPACKET_V4_DISCARD,
   0xb5a230d0,
   0xa8c0,
   0x44f2,
   0x91, 0x6e, 0x99, 0x1b, 0x53, 0xde, 0xd1, 0xf7
);

// f52032cb-991c-46e7-971d-2601459a91ca
DEFINE_GUID(
   FWPM_LAYER_INBOUND_IPPACKET_V6,
   0xf52032cb,
   0x991c,
   0x46e7,
   0x97, 0x1d, 0x26, 0x01, 0x45, 0x9a, 0x91, 0xca
);

// bb24c279-93b4-47a2-83ad-ae1698b50885
DEFINE_GUID(
   FWPM_LAYER_INBOUND_IPPACKET_V6_DISCARD,
   0xbb24c279,
   0x93b4,
   0x47a2,
   0x83, 0xad, 0xae, 0x16, 0x98, 0xb5, 0x08, 0x85
);

// 1e5c9fae-8a84-4135-a331-950b54229ecd
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_IPPACKET_V4,
   0x1e5c9fae,
   0x8a84,
   0x4135,
   0xa3, 0x31, 0x95, 0x0b, 0x54, 0x22, 0x9e, 0xcd
);

// 08e4bcb5-b647-48f3-953c-e5ddbd03937e
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_IPPACKET_V4_DISCARD,
   0x08e4bcb5,
   0xb647,
   0x48f3,
   0x95, 0x3c, 0xe5, 0xdd, 0xbd, 0x03, 0x93, 0x7e
);

// a3b3ab6b-3564-488c-9117-f34e82142763
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_IPPACKET_V6,
   0xa3b3ab6b,
   0x3564,
   0x488c,
   0x91, 0x17, 0xf3, 0x4e, 0x82, 0x14, 0x27, 0x63
);

// 9513d7c4-a934-49dc-91a7-6ccb80cc02e3
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_IPPACKET_V6_DISCARD,
   0x9513d7c4,
   0xa934,
   0x49dc,
   0x91, 0xa7, 0x6c, 0xcb, 0x80, 0xcc, 0x02, 0xe3
);

// a82acc24-4ee1-4ee1-b465-fd1d25cb10a4
DEFINE_GUID(
   FWPM_LAYER_IPFORWARD_V4,
   0xa82acc24,
   0x4ee1,
   0x4ee1,
   0xb4, 0x65, 0xfd, 0x1d, 0x25, 0xcb, 0x10, 0xa4
);

// 9e9ea773-2fae-4210-8f17-34129ef369eb
DEFINE_GUID(
   FWPM_LAYER_IPFORWARD_V4_DISCARD,
   0x9e9ea773,
   0x2fae,
   0x4210,
   0x8f, 0x17, 0x34, 0x12, 0x9e, 0xf3, 0x69, 0xeb
);

// 7b964818-19c7-493a-b71f-832c3684d28c
DEFINE_GUID(
   FWPM_LAYER_IPFORWARD_V6,
   0x7b964818,
   0x19c7,
   0x493a,
   0xb7, 0x1f, 0x83, 0x2c, 0x36, 0x84, 0xd2, 0x8c
);

// 31524a5d-1dfe-472f-bb93-518ee945d8a2
DEFINE_GUID(
   FWPM_LAYER_IPFORWARD_V6_DISCARD,
   0x31524a5d,
   0x1dfe,
   0x472f,
   0xbb, 0x93, 0x51, 0x8e, 0xe9, 0x45, 0xd8, 0xa2
);

// 5926dfc8-e3cf-4426-a283-dc393f5d0f9d
DEFINE_GUID(
   FWPM_LAYER_INBOUND_TRANSPORT_V4,
   0x5926dfc8,
   0xe3cf,
   0x4426,
   0xa2, 0x83, 0xdc, 0x39, 0x3f, 0x5d, 0x0f, 0x9d
);

// ac4a9833-f69d-4648-b261-6dc84835ef39
DEFINE_GUID(
   FWPM_LAYER_INBOUND_TRANSPORT_V4_DISCARD,
   0xac4a9833,
   0xf69d,
   0x4648,
   0xb2, 0x61, 0x6d, 0xc8, 0x48, 0x35, 0xef, 0x39
);

// 634a869f-fc23-4b90-b0c1-bf620a36ae6f
DEFINE_GUID(
   FWPM_LAYER_INBOUND_TRANSPORT_V6,
   0x634a869f,
   0xfc23,
   0x4b90,
   0xb0, 0xc1, 0xbf, 0x62, 0x0a, 0x36, 0xae, 0x6f
);

// 2a6ff955-3b2b-49d2-9848-ad9d72dcaab7
DEFINE_GUID(
   FWPM_LAYER_INBOUND_TRANSPORT_V6_DISCARD,
   0x2a6ff955,
   0x3b2b,
   0x49d2,
   0x98, 0x48, 0xad, 0x9d, 0x72, 0xdc, 0xaa, 0xb7
);

// 09e61aea-d214-46e2-9b21-b26b0b2f28c8
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_TRANSPORT_V4,
   0x09e61aea,
   0xd214,
   0x46e2,
   0x9b, 0x21, 0xb2, 0x6b, 0x0b, 0x2f, 0x28, 0xc8
);

// c5f10551-bdb0-43d7-a313-50e211f4d68a
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_TRANSPORT_V4_DISCARD,
   0xc5f10551,
   0xbdb0,
   0x43d7,
   0xa3, 0x13, 0x50, 0xe2, 0x11, 0xf4, 0xd6, 0x8a
);

// e1735bde-013f-4655-b351-a49e15762df0
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_TRANSPORT_V6,
   0xe1735bde,
   0x013f,
   0x4655,
   0xb3, 0x51, 0xa4, 0x9e, 0x15, 0x76, 0x2d, 0xf0
);

// f433df69-ccbd-482e-b9b2-57165658c3b3
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_TRANSPORT_V6_DISCARD,
   0xf433df69,
   0xccbd,
   0x482e,
   0xb9, 0xb2, 0x57, 0x16, 0x56, 0x58, 0xc3, 0xb3
);

// 3b89653c-c170-49e4-b1cd-e0eeeee19a3e
DEFINE_GUID(
   FWPM_LAYER_STREAM_V4,
   0x3b89653c,
   0xc170,
   0x49e4,
   0xb1, 0xcd, 0xe0, 0xee, 0xee, 0xe1, 0x9a, 0x3e
);

// 25c4c2c2-25ff-4352-82f9-c54a4a4726dc
DEFINE_GUID(
   FWPM_LAYER_STREAM_V4_DISCARD,
   0x25c4c2c2,
   0x25ff,
   0x4352,
   0x82, 0xf9, 0xc5, 0x4a, 0x4a, 0x47, 0x26, 0xdc
);

// 47c9137a-7ec4-46b3-b6e4-48e926b1eda4
DEFINE_GUID(
   FWPM_LAYER_STREAM_V6,
   0x47c9137a,
   0x7ec4,
   0x46b3,
   0xb6, 0xe4, 0x48, 0xe9, 0x26, 0xb1, 0xed, 0xa4
);

// 10a59fc7-b628-4c41-9eb8-cf37d55103cf
DEFINE_GUID(
   FWPM_LAYER_STREAM_V6_DISCARD,
   0x10a59fc7,
   0xb628,
   0x4c41,
   0x9e, 0xb8, 0xcf, 0x37, 0xd5, 0x51, 0x03, 0xcf
);

// 3d08bf4e-45f6-4930-a922-417098e20027
DEFINE_GUID(
   FWPM_LAYER_DATAGRAM_DATA_V4,
   0x3d08bf4e,
   0x45f6,
   0x4930,
   0xa9, 0x22, 0x41, 0x70, 0x98, 0xe2, 0x00, 0x27
);

// 18e330c6-7248-4e52-aaab-472ed67704fd
DEFINE_GUID(
   FWPM_LAYER_DATAGRAM_DATA_V4_DISCARD,
   0x18e330c6,
   0x7248,
   0x4e52,
   0xaa, 0xab, 0x47, 0x2e, 0xd6, 0x77, 0x04, 0xfd
);

// fa45fe2f-3cba-4427-87fc-57b9a4b10d00
DEFINE_GUID(
   FWPM_LAYER_DATAGRAM_DATA_V6,
   0xfa45fe2f,
   0x3cba,
   0x4427,
   0x87, 0xfc, 0x57, 0xb9, 0xa4, 0xb1, 0x0d, 0x00
);

// 09d1dfe1-9b86-4a42-be9d-8c315b92a5d0
DEFINE_GUID(
   FWPM_LAYER_DATAGRAM_DATA_V6_DISCARD,
   0x09d1dfe1,
   0x9b86,
   0x4a42,
   0xbe, 0x9d, 0x8c, 0x31, 0x5b, 0x92, 0xa5, 0xd0
);

// 61499990-3cb6-4e84-b950-53b94b6964f3
DEFINE_GUID(
   FWPM_LAYER_INBOUND_ICMP_ERROR_V4,
   0x61499990,
   0x3cb6,
   0x4e84,
   0xb9, 0x50, 0x53, 0xb9, 0x4b, 0x69, 0x64, 0xf3
);

// a6b17075-ebaf-4053-a4e7-213c8121ede5
DEFINE_GUID(
   FWPM_LAYER_INBOUND_ICMP_ERROR_V4_DISCARD,
   0xa6b17075,
   0xebaf,
   0x4053,
   0xa4, 0xe7, 0x21, 0x3c, 0x81, 0x21, 0xed, 0xe5
);

// 65f9bdff-3b2d-4e5d-b8c6-c720651fe898
DEFINE_GUID(
   FWPM_LAYER_INBOUND_ICMP_ERROR_V6,
   0x65f9bdff,
   0x3b2d,
   0x4e5d,
   0xb8, 0xc6, 0xc7, 0x20, 0x65, 0x1f, 0xe8, 0x98
);

// a6e7ccc0-08fb-468d-a472-9771d5595e09
DEFINE_GUID(
   FWPM_LAYER_INBOUND_ICMP_ERROR_V6_DISCARD,
   0xa6e7ccc0,
   0x08fb,
   0x468d,
   0xa4, 0x72, 0x97, 0x71, 0xd5, 0x59, 0x5e, 0x09
);

// 41390100-564c-4b32-bc1d-718048354d7c
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_ICMP_ERROR_V4,
   0x41390100,
   0x564c,
   0x4b32,
   0xbc, 0x1d, 0x71, 0x80, 0x48, 0x35, 0x4d, 0x7c
);

// b3598d36-0561-4588-a6bf-e955e3f6264b
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_ICMP_ERROR_V4_DISCARD,
   0xb3598d36,
   0x0561,
   0x4588,
   0xa6, 0xbf, 0xe9, 0x55, 0xe3, 0xf6, 0x26, 0x4b
);

// 7fb03b60-7b8d-4dfa-badd-980176fc4e12
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_ICMP_ERROR_V6,
   0x7fb03b60,
   0x7b8d,
   0x4dfa,
   0xba, 0xdd, 0x98, 0x01, 0x76, 0xfc, 0x4e, 0x12
);

// 65f2e647-8d0c-4f47-b19b-33a4d3f1357c
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_ICMP_ERROR_V6_DISCARD,
   0x65f2e647,
   0x8d0c,
   0x4f47,
   0xb1, 0x9b, 0x33, 0xa4, 0xd3, 0xf1, 0x35, 0x7c
);

// 1247d66d-0b60-4a15-8d44-7155d0f53a0c
DEFINE_GUID(
   FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4,
   0x1247d66d,
   0x0b60,
   0x4a15,
   0x8d, 0x44, 0x71, 0x55, 0xd0, 0xf5, 0x3a, 0x0c
);

// 0b5812a2-c3ff-4eca-b88d-c79e20ac6322
DEFINE_GUID(
   FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4_DISCARD,
   0x0b5812a2,
   0xc3ff,
   0x4eca,
   0xb8, 0x8d, 0xc7, 0x9e, 0x20, 0xac, 0x63, 0x22
);

// 55a650e1-5f0a-4eca-a653-88f53b26aa8c
DEFINE_GUID(
   FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6,
   0x55a650e1,
   0x5f0a,
   0x4eca,
   0xa6, 0x53, 0x88, 0xf5, 0x3b, 0x26, 0xaa, 0x8c
);

// cbc998bb-c51f-4c1a-bb4f-9775fcacab2f
DEFINE_GUID(
   FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6_DISCARD,
   0xcbc998bb,
   0xc51f,
   0x4c1a,
   0xbb, 0x4f, 0x97, 0x75, 0xfc, 0xac, 0xab, 0x2f
);

// 88bb5dad-76d7-4227-9c71-df0a3ed7be7e
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_LISTEN_V4,
   0x88bb5dad,
   0x76d7,
   0x4227,
   0x9c, 0x71, 0xdf, 0x0a, 0x3e, 0xd7, 0xbe, 0x7e
);

// 371dfada-9f26-45fd-b4eb-c29eb212893f
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_LISTEN_V4_DISCARD,
   0x371dfada,
   0x9f26,
   0x45fd,
   0xb4, 0xeb, 0xc2, 0x9e, 0xb2, 0x12, 0x89, 0x3f
);

// 7ac9de24-17dd-4814-b4bd-a9fbc95a321b
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_LISTEN_V6,
   0x7ac9de24,
   0x17dd,
   0x4814,
   0xb4, 0xbd, 0xa9, 0xfb, 0xc9, 0x5a, 0x32, 0x1b
);

// 60703b07-63c8-48e9-ada3-12b1af40a617
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_LISTEN_V6_DISCARD,
   0x60703b07,
   0x63c8,
   0x48e9,
   0xad, 0xa3, 0x12, 0xb1, 0xaf, 0x40, 0xa6, 0x17
);

// e1cd9fe7-f4b5-4273-96c0-592e487b8650
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4,
   0xe1cd9fe7,
   0xf4b5,
   0x4273,
   0x96, 0xc0, 0x59, 0x2e, 0x48, 0x7b, 0x86, 0x50
);

// 9eeaa99b-bd22-4227-919f-0073c63357b1
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4_DISCARD,
   0x9eeaa99b,
   0xbd22,
   0x4227,
   0x91, 0x9f, 0x00, 0x73, 0xc6, 0x33, 0x57, 0xb1
);

// a3b42c97-9f04-4672-b87e-cee9c483257f
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6,
   0xa3b42c97,
   0x9f04,
   0x4672,
   0xb8, 0x7e, 0xce, 0xe9, 0xc4, 0x83, 0x25, 0x7f
);

// 89455b97-dbe1-453f-a224-13da895af396
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6_DISCARD,
   0x89455b97,
   0xdbe1,
   0x453f,
   0xa2, 0x24, 0x13, 0xda, 0x89, 0x5a, 0xf3, 0x96
);

// c38d57d1-05a7-4c33-904f-7fbceee60e82
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_CONNECT_V4,
   0xc38d57d1,
   0x05a7,
   0x4c33,
   0x90, 0x4f, 0x7f, 0xbc, 0xee, 0xe6, 0x0e, 0x82
);

// d632a801-f5ba-4ad6-96e3-607017d9836a
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_CONNECT_V4_DISCARD,
   0xd632a801,
   0xf5ba,
   0x4ad6,
   0x96, 0xe3, 0x60, 0x70, 0x17, 0xd9, 0x83, 0x6a
);

// 4a72393b-319f-44bc-84c3-ba54dcb3b6b4
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_CONNECT_V6,
   0x4a72393b,
   0x319f,
   0x44bc,
   0x84, 0xc3, 0xba, 0x54, 0xdc, 0xb3, 0xb6, 0xb4
);

// c97bc3b8-c9a3-4e33-8695-8e17aad4de09
DEFINE_GUID(
   FWPM_LAYER_ALE_AUTH_CONNECT_V6_DISCARD,
   0xc97bc3b8,
   0xc9a3,
   0x4e33,
   0x86, 0x95, 0x8e, 0x17, 0xaa, 0xd4, 0xde, 0x09
);

// af80470a-5596-4c13-9992-539e6fe57967
DEFINE_GUID(
   FWPM_LAYER_ALE_FLOW_ESTABLISHED_V4,
   0xaf80470a,
   0x5596,
   0x4c13,
   0x99, 0x92, 0x53, 0x9e, 0x6f, 0xe5, 0x79, 0x67
);

// 146ae4a9-a1d2-4d43-a31a-4c42682b8e4f
DEFINE_GUID(
   FWPM_LAYER_ALE_FLOW_ESTABLISHED_V4_DISCARD,
   0x146ae4a9,
   0xa1d2,
   0x4d43,
   0xa3, 0x1a, 0x4c, 0x42, 0x68, 0x2b, 0x8e, 0x4f
);

// 7021d2b3-dfa4-406e-afeb-6afaf7e70efd
DEFINE_GUID(
   FWPM_LAYER_ALE_FLOW_ESTABLISHED_V6,
   0x7021d2b3,
   0xdfa4,
   0x406e,
   0xaf, 0xeb, 0x6a, 0xfa, 0xf7, 0xe7, 0x0e, 0xfd
);

// 46928636-bbca-4b76-941d-0fa7f5d7d372
DEFINE_GUID(
   FWPM_LAYER_ALE_FLOW_ESTABLISHED_V6_DISCARD,
   0x46928636,
   0xbbca,
   0x4b76,
   0x94, 0x1d, 0x0f, 0xa7, 0xf5, 0xd7, 0xd3, 0x72
);

#if (NTDDI_VERSION >= NTDDI_WIN8)

// effb7edb-0055-4f9a-a23a-4ff8131ad191
DEFINE_GUID(
   FWPM_LAYER_INBOUND_MAC_FRAME_ETHERNET,
   0xeffb7edb,
   0x0055,
   0x4f9a,
   0xa2, 0x31, 0x4f, 0xf8, 0x13, 0x1a, 0xd1, 0x91
);

// 694673bc-d6db-4870-adee-0acdbdb7f4b2
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_MAC_FRAME_ETHERNET,
   0x694673bc,
   0xd6db,
   0x4870,
   0xad, 0xee, 0x0a, 0xcd, 0xbd, 0xb7, 0xf4, 0xb2
);

// d4220bd3-62ce-4f08-ae88-b56e8526df50
DEFINE_GUID(
   FWPM_LAYER_INBOUND_MAC_FRAME_NATIVE,
   0xd4220bd3,
   0x62ce,
   0x4f08,
   0xae, 0x88, 0xb5, 0x6e, 0x85, 0x26, 0xdf, 0x50
);

// 94c44912-9d6f-4ebf-b995-05ab8a088d1b
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_MAC_FRAME_NATIVE,
   0x94c44912,
   0x9d6f,
   0x4ebf,
   0xb9, 0x95, 0x05, 0xab, 0x8a, 0x08, 0x8d, 0x1b
);

// 7d98577a-9a87-41ec-9718-7cf589c9f32d
DEFINE_GUID(
   FWPM_LAYER_INGRESS_VSWITCH_ETHERNET,
   0x7d98577a,
   0x9a87,
   0x41ec,
   0x97, 0x18, 0x7c, 0xf5, 0x89, 0xc9, 0xf3, 0x2d
);

// 86c872b0-76fa-4b79-93a4-0750530ae292
DEFINE_GUID(
   FWPM_LAYER_EGRESS_VSWITCH_ETHERNET,
   0x86c872b0,
   0x76fa,
   0x4b79,
   0x93, 0xa4, 0x07, 0x50, 0x53, 0x0a, 0xe2, 0x92
);

// b2696ff6-774f-4554-9f7d-3da3945f8e85
DEFINE_GUID(
   FWPM_LAYER_INGRESS_VSWITCH_TRANSPORT_V4,
   0xb2696ff6,
   0x774f,
   0x4554,
   0x9f, 0x7d, 0x3d, 0xa3, 0x94, 0x5f, 0x8e, 0x85
);

// 5ee314fc-7d8a-47f4-b7e3-291a36da4e12
DEFINE_GUID(
   FWPM_LAYER_INGRESS_VSWITCH_TRANSPORT_V6,
   0x5ee314fc,
   0x7d8a,
   0x47f4,
   0xb7, 0xe3, 0x29, 0x1a, 0x36, 0xda, 0x4e, 0x12
);

// b92350b6-91f0-46b6-bdc4-871dfd4a7c98
DEFINE_GUID(
   FWPM_LAYER_EGRESS_VSWITCH_TRANSPORT_V4,
   0xb92350b6,
   0x91f0,
   0x46b6,
   0xbd, 0xc4, 0x87, 0x1d, 0xfd, 0x4a, 0x7c, 0x98
);

// 1b2def23-1881-40bd-82f4-4254e63141cb
DEFINE_GUID(
   FWPM_LAYER_EGRESS_VSWITCH_TRANSPORT_V6,
   0x1b2def23,
   0x1881,
   0x40bd,
   0x82, 0xf4, 0x42, 0x54, 0xe6, 0x31, 0x41, 0xcb
);

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
//e41d2719-05c7-40f0-8983-ea8d17bbc2f6
DEFINE_GUID(
   FWPM_LAYER_INBOUND_TRANSPORT_FAST,
   0xe41d2719,
   0x05c7,
   0x40f0,
   0x89, 0x83, 0xea, 0x8d, 0x17, 0xbb, 0xc2, 0xf6
);

//13ed4388-a070-4815-9935-7a9be6408b78
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_TRANSPORT_FAST,
   0x13ed4388,
   0xa070,
   0x4815,
   0x99,0x35,0x7a,0x9b,0xe6,0x40,0x8b,0x78
);

//853aaa8e-2b78-4d24-a804-36db08b29711
DEFINE_GUID(
   FWPM_LAYER_INBOUND_MAC_FRAME_NATIVE_FAST,
   0x853aaa8e,
   0x2b78,
   0x4d24,
   0xa8,0x04,0x36,0xdb,0x08,0xb2,0x97,0x11
);

//470df946-c962-486f-9446-8293cbc75eb8
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_MAC_FRAME_NATIVE_FAST,
   0x470df946,
   0xc962,
   0x486f,
   0x94,0x46,0x82,0x93,0xcb,0xc7,0x5e,0xb8
);


#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)

// f02b1526-a459-4a51-b9e3-759de52b9d2c
DEFINE_GUID(
   FWPM_LAYER_IPSEC_KM_DEMUX_V4,
   0xf02b1526,
   0xa459,
   0x4a51,
   0xb9, 0xe3, 0x75, 0x9d, 0xe5, 0x2b, 0x9d, 0x2c
);

// 2f755cf6-2fd4-4e88-b3e4-a91bca495235
DEFINE_GUID(
   FWPM_LAYER_IPSEC_KM_DEMUX_V6,
   0x2f755cf6,
   0x2fd4,
   0x4e88,
   0xb3, 0xe4, 0xa9, 0x1b, 0xca, 0x49, 0x52, 0x35
);

// eda65c74-610d-4bc5-948f-3c4f89556867
DEFINE_GUID(
   FWPM_LAYER_IPSEC_V4,
   0xeda65c74,
   0x610d,
   0x4bc5,
   0x94, 0x8f, 0x3c, 0x4f, 0x89, 0x55, 0x68, 0x67
);

// 13c48442-8d87-4261-9a29-59d2abc348b4
DEFINE_GUID(
   FWPM_LAYER_IPSEC_V6,
   0x13c48442,
   0x8d87,
   0x4261,
   0x9a, 0x29, 0x59, 0xd2, 0xab, 0xc3, 0x48, 0xb4
);

// b14b7bdb-dbbd-473e-bed4-8b4708d4f270
DEFINE_GUID(
   FWPM_LAYER_IKEEXT_V4,
   0xb14b7bdb,
   0xdbbd,
   0x473e,
   0xbe, 0xd4, 0x8b, 0x47, 0x08, 0xd4, 0xf2, 0x70
);

// b64786b3-f687-4eb9-89d2-8ef32acdabe2
DEFINE_GUID(
   FWPM_LAYER_IKEEXT_V6,
   0xb64786b3,
   0xf687,
   0x4eb9,
   0x89, 0xd2, 0x8e, 0xf3, 0x2a, 0xcd, 0xab, 0xe2
);

// 75a89dda-95e4-40f3-adc7-7688a9c847e1
DEFINE_GUID(
   FWPM_LAYER_RPC_UM,
   0x75a89dda,
   0x95e4,
   0x40f3,
   0xad, 0xc7, 0x76, 0x88, 0xa9, 0xc8, 0x47, 0xe1
);

// 9247bc61-eb07-47ee-872c-bfd78bfd1616
DEFINE_GUID(
   FWPM_LAYER_RPC_EPMAP,
   0x9247bc61,
   0xeb07,
   0x47ee,
   0x87, 0x2c, 0xbf, 0xd7, 0x8b, 0xfd, 0x16, 0x16
);

// 618dffc7-c450-4943-95db-99b4c16a55d4
DEFINE_GUID(
   FWPM_LAYER_RPC_EP_ADD,
   0x618dffc7,
   0xc450,
   0x4943,
   0x95, 0xdb, 0x99, 0xb4, 0xc1, 0x6a, 0x55, 0xd4
);

// 94a4b50b-ba5c-4f27-907a-229fac0c2a7a
DEFINE_GUID(
   FWPM_LAYER_RPC_PROXY_CONN,
   0x94a4b50b,
   0xba5c,
   0x4f27,
   0x90, 0x7a, 0x22, 0x9f, 0xac, 0x0c, 0x2a, 0x7a
);

// f8a38615-e12c-41ac-98df-121ad981aade
DEFINE_GUID(
   FWPM_LAYER_RPC_PROXY_IF,
   0xf8a38615,
   0xe12c,
   0x41ac,
   0x98, 0xdf, 0x12, 0x1a, 0xd9, 0x81, 0xaa, 0xde
);

#if (NTDDI_VERSION >= NTDDI_WIN7)

// 4aa226e9-9020-45fb-956a-c0249d841195
DEFINE_GUID(
   FWPM_LAYER_KM_AUTHORIZATION,
   0x4aa226e9,
   0x9020,
   0x45fb,
   0x95,0x6a, 0xc0, 0x24, 0x9d, 0x84, 0x11, 0x95
);

// 0c2aa681-905b-4ccd-a467-4dd811d07b7b
DEFINE_GUID(
   FWPM_LAYER_NAME_RESOLUTION_CACHE_V4,
   0x0c2aa681,
   0x905b,
   0x4ccd,
   0xa4, 0x67, 0x4d, 0xd8, 0x11, 0xd0, 0x7b, 0x7b
);

// 92d592fa-6b01-434a-9dea-d1e96ea97da9
DEFINE_GUID(
   FWPM_LAYER_NAME_RESOLUTION_CACHE_V6,
   0x92d592fa,
   0x6b01,
   0x434a,
   0x9d, 0xea, 0xd1, 0xe9, 0x6e, 0xa9, 0x7d, 0xa9
);

// 74365cce-ccb0-401a-bfc1-b89934ad7e15
DEFINE_GUID(
   FWPM_LAYER_ALE_RESOURCE_RELEASE_V4,
   0x74365cce,
   0xccb0,
   0x401a,
   0xbf, 0xc1, 0xb8, 0x99, 0x34, 0xad, 0x7e, 0x15
);

// f4e5ce80-edcc-4e13-8a2f-b91454bb057b
DEFINE_GUID(
   FWPM_LAYER_ALE_RESOURCE_RELEASE_V6,
   0xf4e5ce80,
   0xedcc,
   0x4e13,
   0x8a, 0x2f, 0xb9, 0x14, 0x54, 0xbb, 0x05, 0x7b
);

// b4766427-e2a2-467a-bd7e-dbcd1bd85a09
DEFINE_GUID(
   FWPM_LAYER_ALE_ENDPOINT_CLOSURE_V4,
   0xb4766427,
   0xe2a2,
   0x467a,
   0xbd, 0x7e, 0xdb, 0xcd, 0x1b, 0xd8, 0x5a, 0x09
);

// bb536ccd-4755-4ba9-9ff7-f9edf8699c7b
DEFINE_GUID(
   FWPM_LAYER_ALE_ENDPOINT_CLOSURE_V6,
   0xbb536ccd,
   0x4755,
   0x4ba9,
   0x9f, 0xf7, 0xf9, 0xed, 0xf8, 0x69, 0x9c, 0x7b
);

// c6e63c8c-b784-4562-aa7d-0a67cfcaf9a3
DEFINE_GUID(
   FWPM_LAYER_ALE_CONNECT_REDIRECT_V4,
   0xc6e63c8c,
   0xb784,
   0x4562,
   0xaa, 0x7d, 0x0a, 0x67, 0xcf, 0xca, 0xf9, 0xa3
);

// 587e54a7-8046-42ba-a0aa-b716250fc7fd
DEFINE_GUID(
   FWPM_LAYER_ALE_CONNECT_REDIRECT_V6,
   0x587e54a7,
   0x8046,
   0x42ba,
   0xa0, 0xaa, 0xb7, 0x16, 0x25, 0x0f, 0xc7, 0xfd
);

// 66978cad-c704-42ac-86ac-7c1a231bd253
DEFINE_GUID(
   FWPM_LAYER_ALE_BIND_REDIRECT_V4,
   0x66978cad,
   0xc704,
   0x42ac,
   0x86, 0xac, 0x7c, 0x1a, 0x23, 0x1b, 0xd2, 0x53
);

// bef02c9c-606b-4536-8c26-1c2fc7b631d4
DEFINE_GUID(
   FWPM_LAYER_ALE_BIND_REDIRECT_V6,
   0xbef02c9c,
   0x606b,
   0x4536,
   0x8c, 0x26, 0x1c, 0x2f, 0xc7, 0xb6, 0x31, 0xd4
);

// af52d8ec-cb2d-44e5-ad92-f8dc38d2eb29
DEFINE_GUID(
   FWPM_LAYER_STREAM_PACKET_V4,
   0xaf52d8ec,
   0xcb2d,
   0x44e5,
   0xad, 0x92, 0xf8, 0xdc, 0x38, 0xd2, 0xeb, 0x29
);

// 779a8ca3-f099-468f-b5d4-83535c461c02
DEFINE_GUID(
   FWPM_LAYER_STREAM_PACKET_V6,
   0x779a8ca3,
   0xf099,
   0x468f,
   0xb5, 0xd4, 0x83, 0x53, 0x5c, 0x46, 0x1c, 0x02
);

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

//f4fb8d55-c076-46d8-a2c7-6a4c722ca4ed
DEFINE_GUID(
   FWPM_LAYER_INBOUND_RESERVED2,
   0xf4fb8d55,
   0xc076,
   0x46d8,
   0xa2, 0xc7, 0x6a, 0x4c, 0x72, 0x2c, 0xa4, 0xed
);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)


#if (NTDDI_VERSION >= NTDDI_WIN10_NI)

//037f317a-d696-494a-bba5-bffc265e6052
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_NETWORK_CONNECTION_POLICY_V4,
   0x037f317a,
   0xd696,
   0x494a,
   0xbb, 0xa5, 0xbf, 0xfc, 0x26, 0x5e, 0x60, 0x52
);

//22a4fdb1-6d7e-48ae-ae77-3742525c3119
DEFINE_GUID(
   FWPM_LAYER_OUTBOUND_NETWORK_CONNECTION_POLICY_V6,
   0x22a4fdb1,
   0x6d7e,
   0x48ae,
   0xae, 0x77, 0x37, 0x42, 0x52, 0x5c, 0x31, 0x19
);

#endif //  (NTDDI_VERSION >= NTDDI_WIN10_NI)


///////////////////////////////////////////////////////////////////////////////
//
// GUIDs for built-in sublayers.
//
///////////////////////////////////////////////////////////////////////////////

// 758c84f4-fb48-4de9-9aeb-3ed9551ab1fd
DEFINE_GUID(
   FWPM_SUBLAYER_RPC_AUDIT,
   0x758c84f4,
   0xfb48,
   0x4de9,
   0x9a, 0xeb, 0x3e, 0xd9, 0x55, 0x1a, 0xb1, 0xfd
);

// 83f299ed-9ff4-4967-aff4-c309f4dab827
DEFINE_GUID(
   FWPM_SUBLAYER_IPSEC_TUNNEL,
   0x83f299ed,
   0x9ff4,
   0x4967,
   0xaf, 0xf4, 0xc3, 0x09, 0xf4, 0xda, 0xb8, 0x27
);

// eebecc03-ced4-4380-819a-2734397b2b74
DEFINE_GUID(
   FWPM_SUBLAYER_UNIVERSAL,
   0xeebecc03,
   0xced4,
   0x4380,
   0x81, 0x9a, 0x27, 0x34, 0x39, 0x7b, 0x2b, 0x74
);

// 1b75c0ce-ff60-4711-a70f-b4958cc3b2d0
DEFINE_GUID(
   FWPM_SUBLAYER_LIPS,
   0x1b75c0ce,
   0xff60,
   0x4711,
   0xa7, 0x0f, 0xb4, 0x95, 0x8c, 0xc3, 0xb2, 0xd0
);

// 15a66e17-3f3c-4f7b-aa6c-812aa613dd82
DEFINE_GUID(
   FWPM_SUBLAYER_SECURE_SOCKET,
   0x15a66e17,
   0x3f3c,
   0x4f7b,
   0xaa, 0x6c, 0x81, 0x2a, 0xa6, 0x13, 0xdd, 0x82
);

// 337608b9-b7d5-4d5f-82f9-3618618bc058
DEFINE_GUID(
   FWPM_SUBLAYER_TCP_CHIMNEY_OFFLOAD,
   0x337608b9,
   0xb7d5,
   0x4d5f,
   0x82, 0xf9, 0x36, 0x18, 0x61, 0x8b, 0xc0, 0x58
);

// 877519e1-e6a9-41a5-81b4-8c4f118e4a60
DEFINE_GUID(
   FWPM_SUBLAYER_INSPECTION,
   0x877519e1,
   0xe6a9,
   0x41a5,
   0x81, 0xb4, 0x8c, 0x4f, 0x11, 0x8e, 0x4a, 0x60
);

// ba69dc66-5176-4979-9c89-26a7b46a8327
DEFINE_GUID(
   FWPM_SUBLAYER_TEREDO,
   0xba69dc66,
   0x5176,
   0x4979,
   0x9c, 0x89, 0x26, 0xa7, 0xb4, 0x6a, 0x83, 0x27
);

#define FWPM_SUBLAYER_EDGE_TRAVERSAL FWPM_SUBLAYER_TEREDO

#if (NTDDI_VERSION >= NTDDI_WIN6SP1)

// a5082e73-8f71-4559-8a9a-101cea04ef87
DEFINE_GUID(
   FWPM_SUBLAYER_IPSEC_FORWARD_OUTBOUND_TUNNEL,
   0xa5082e73,
   0x8f71,
   0x4559,
   0x8a, 0x9a, 0x10, 0x1c, 0xea, 0x04, 0xef, 0x87
);

#endif // (NTDDI_VERSION >= NTDDI_WIN6SP1)

#if (NTDDI_VERSION >= NTDDI_WIN7)

// e076d572-5d3d-48ef-802b-909eddb098bd
DEFINE_GUID(
   FWPM_SUBLAYER_IPSEC_DOSP,
   0xe076d572,
   0x5d3d,
   0x48ef,
   0x80, 0x2b, 0x90, 0x9e, 0xdd, 0xb0, 0x98, 0xbd
);

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)

// 24421dcf-0ac5-4caa-9e14-50f6e3636af0
DEFINE_GUID(
   FWPM_SUBLAYER_TCP_TEMPLATES,
   0x24421dcf,
   0x0ac5,
   0x4caa,
   0x9e, 0x14, 0x50, 0xf6, 0xe3, 0x63, 0x6a, 0xf0
);

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

// 37a57701-5884-4964-92b8-3e704688b0ad
DEFINE_GUID(
   FWPM_SUBLAYER_IPSEC_SECURITY_REALM,
   0x37a57701,
   0x5884,
   0x4964,
   0x92, 0xb8, 0x3e, 0x70, 0x46, 0x88, 0xb0, 0xad
);

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

// B3CDD441-AF90-41BA-A745-7C6008FF2300
DEFINE_GUID(
   FWPM_SUBLAYER_MPSSVC_WSH,
   0xb3cdd441,
   0xaf90,
   0x41ba,
   0xa7, 0x45, 0x7c, 0x60, 0x08, 0xff, 0x23, 0x00
);

// B3CDD441-AF90-41BA-A745-7C6008FF2301
DEFINE_GUID(
   FWPM_SUBLAYER_MPSSVC_WF,
   0xb3cdd441,
   0xaf90,
   0x41ba,
   0xa7, 0x45, 0x7c, 0x60, 0x08, 0xff, 0x23, 0x01
);

// B3CDD441-AF90-41BA-A745-7C6008FF2302
DEFINE_GUID(
   FWPM_SUBLAYER_MPSSVC_QUARANTINE,
   0xb3cdd441,
   0xaf90,
   0x41ba,
   0xa7, 0x45, 0x7c, 0x60, 0x08, 0xff, 0x23, 0x02
);

// 09A47E38-FA97-471B-B12318BCD7E65071
DEFINE_GUID(
   FWPM_SUBLAYER_MPSSVC_EDP,
   0x09a47e38,
   0xfa97,
   0x471b,
   0xb1, 0x23, 0x18, 0xbc, 0xd7, 0xe6, 0x50, 0x71
);

// 1EC6C7E1-FDD9-478A-B55F-FF8BA1D2C17D
DEFINE_GUID(
   FWPM_SUBLAYER_MPSSVC_TENANT_RESTRICTIONS,
   0x1ec6c7e1,
   0xfdd9,
   0x478a,
   0xb5, 0x5f, 0xff, 0x8b, 0xa1, 0xd2, 0xc1, 0x7d
);

// FFE221C3-92A8-4564-A59F-DAFB70756020
DEFINE_GUID(
   FWPM_SUBLAYER_MPSSVC_APP_ISOLATION,
   0xffe221c3,
   0x92a8,
   0x4564,
   0xa5, 0x9f, 0xda, 0xfb, 0x70, 0x75, 0x60, 0x20
);

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

///////////////////////////////////////////////////////////////////////////////
//
// GUIDs for built-in conditions.
//
///////////////////////////////////////////////////////////////////////////////
#if (NTDDI_VERSION >= NTDDI_WIN10_CU)

// 81BC78FB-F28D-4886-A604-6ACC261F261B
DEFINE_GUID(
   FWPM_CONDITION_ALE_PACKAGE_FAMILY_NAME,
   0x81bc78fb,
   0xf28d,
   0x4886,
   0xa6, 0x4, 0x6a, 0xcc, 0x26, 0x1f, 0x26, 0x1B
);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_CU)

#if (NTDDI_VERSION >= NTDDI_WIN8)

// f6e63dce-1f4b-4c6b-b6ef-1165e71f8ee7
DEFINE_GUID(
   FWPM_CONDITION_INTERFACE_MAC_ADDRESS,
   0xf6e63dce,
   0x1f4b,
   0x4c6b,
   0xb6, 0xef, 0x11, 0x65, 0xe7, 0x1f, 0x8e, 0xe7
);

// d999e981-7948-4c8e-b742-c84e3b678f8f
DEFINE_GUID(
   FWPM_CONDITION_MAC_LOCAL_ADDRESS,
   0xd999e981,
   0x7948,
   0x4c83,
   0xb7, 0x42, 0xc8, 0x4e, 0x3b, 0x67, 0x8f, 0x8f
);

// 408f2ed4-3a70-4b4d-92a6-415ac20e2f12
DEFINE_GUID(
   FWPM_CONDITION_MAC_REMOTE_ADDRESS,
   0x408f2ed4,
   0x3a70,
   0x4b4d,
   0x92, 0xa6, 0x41, 0x5a, 0xc2, 0x0e, 0x2f, 0x12
);

// fd08948d-a219-4d52-bb98-1a5540ee7b4e
DEFINE_GUID(
   FWPM_CONDITION_ETHER_TYPE,
   0xfd08948d,
   0xa219,
   0x4d52,
   0xbb, 0x98, 0x1a, 0x55, 0x40, 0xee, 0x7b, 0x4e
);


// 938eab21-3618-4e64-9ca5-2141ebda1ca2
DEFINE_GUID(
   FWPM_CONDITION_VLAN_ID,
   0x938eab21,
   0x3618,
   0x4e64,
   0x9c, 0xa5, 0x21, 0x41, 0xeb, 0xda, 0x1c, 0xa2
);

// dc04843c-79e6-4e44-a025-65b9bb0f9f94
DEFINE_GUID(
   FWPM_CONDITION_VSWITCH_TENANT_NETWORK_ID,
   0xdc04843c,
   0x79e6,
   0x4e44,
   0xa0, 0x25, 0x65, 0xb9, 0xbb, 0x0f, 0x9f, 0x94
);

// db7bb42b-2dac-4cd4-a59a-e0bdce1e6834
DEFINE_GUID(
   FWPM_CONDITION_NDIS_PORT,
   0xdb7bb42b,
   0x2dac,
   0x4cd4,
   0xa5, 0x9a, 0xe0, 0xbd, 0xce, 0x1e, 0x68, 0x34
);

// cb31cef1-791d-473b-89d1-61c5984304a0
DEFINE_GUID(
   FWPM_CONDITION_NDIS_MEDIA_TYPE,
   0xcb31cef1,
   0x791d,
   0x473b,
   0x89, 0xd1, 0x61, 0xc5, 0x98, 0x43, 0x04, 0xa0
);

// 34c79823-c229-44f2-b83c-74020882ae77
DEFINE_GUID(
   FWPM_CONDITION_NDIS_PHYSICAL_MEDIA_TYPE,
   0x34c79823,
   0xc229,
   0x44f2,
   0xb8, 0x3c, 0x74, 0x02, 0x08, 0x82, 0xae, 0x77
);

// 7bc43cbf-37ba-45f1-b74a-82ff518eeb10
DEFINE_GUID(
   FWPM_CONDITION_L2_FLAGS,
   0x7bc43cbf,
   0x37ba,
   0x45f1,
   0xb7, 0x4a, 0x82, 0xff, 0x51, 0x8e, 0xeb, 0x10
);

// cc31355c-3073-4ffb-a14f-79415cb1ead1
DEFINE_GUID(
   FWPM_CONDITION_MAC_LOCAL_ADDRESS_TYPE,
   0xcc31355c,
   0x3073,
   0x4ffb,
   0xa1, 0x4f, 0x79, 0x41, 0x5c, 0xb1, 0xea, 0xd1
);

// 027fedb4-f1c1-4030-b564-ee777fd867ea
DEFINE_GUID(
   FWPM_CONDITION_MAC_REMOTE_ADDRESS_TYPE,
   0x027fedb4,
   0xf1c1,
   0x4030,
   0xb5, 0x64, 0xee, 0x77, 0x7f, 0xd8, 0x67, 0xea
);

#define FWPM_CONDITION_INTERFACE FWPM_CONDITION_IP_LOCAL_INTERFACE

// 71BC78FA-F17C-4997-A602-6ABB261F351C
DEFINE_GUID(
   FWPM_CONDITION_ALE_PACKAGE_ID,
   0x71bc78fa,
   0xf17c,
   0x4997,
   0xa6, 0x2, 0x6a, 0xbb, 0x26, 0x1f, 0x35, 0x1c
);

// 7b795451-f1f6-4d05-b7cb-21779d802336
DEFINE_GUID(
   FWPM_CONDITION_MAC_SOURCE_ADDRESS,
   0x7b795451,
   0xf1f6,
   0x4d05,
   0xb7, 0xcb, 0x21, 0x77, 0x9d, 0x80, 0x23, 0x36
);

// 04ea2a93-858c-4027-b613-b43180c7859e
DEFINE_GUID(
   FWPM_CONDITION_MAC_DESTINATION_ADDRESS,
   0x04ea2a93,
   0x858c,
   0x4027,
   0xb6, 0x13, 0xb4, 0x31, 0x80, 0xc7, 0x85, 0x9e
);

// 5c1b72e4-299e-4437-a298-bc3f014b3dc2
DEFINE_GUID(
   FWPM_CONDITION_MAC_SOURCE_ADDRESS_TYPE,
   0x5c1b72e4,
   0x299e,
   0x4437,
   0xa2, 0x98, 0xbc, 0x3f, 0x01, 0x4b, 0x3d, 0xc2
);

// ae052932-ef42-4e99-b129-f3b3139e34f7
DEFINE_GUID(
   FWPM_CONDITION_MAC_DESTINATION_ADDRESS_TYPE,
   0xae052932,
   0xef42,
   0x4e99,
   0xb1, 0x29, 0xf3, 0xb3, 0x13, 0x9e, 0x34, 0xf7
);

// a6afef91-3df4-4730-a214-f5426aebf821
DEFINE_GUID(
   FWPM_CONDITION_IP_SOURCE_PORT,
   0xa6afef91,
   0x3df4,
   0x4730,
   0xa2, 0x14, 0xf5, 0x42, 0x6a, 0xeb, 0xf8, 0x21
);

#define FWPM_CONDITION_VSWITCH_ICMP_TYPE FWPM_CONDITION_IP_SOURCE_PORT

// ce6def45-60fb-4a7b-a304-af30a117000e
DEFINE_GUID(
   FWPM_CONDITION_IP_DESTINATION_PORT,
   0xce6def45,
   0x60fb,
   0x4a7b,
   0xa3, 0x04, 0xaf, 0x30, 0xa1, 0x17, 0x00, 0x0e
);

#define FWPM_CONDITION_VSWITCH_ICMP_CODE FWPM_CONDITION_IP_DESTINATION_PORT

// c4a414ba-437b-4de6-9946-d99c1b95b312
DEFINE_GUID(
   FWPM_CONDITION_VSWITCH_ID,
   0xc4a414ba,
   0x437b,
   0x4de6,
   0x99, 0x46, 0xd9, 0x9c, 0x1b, 0x95, 0xb3, 0x12
);

// 11d48b4b-e77a-40b4-9155-392c906c2608
DEFINE_GUID(
   FWPM_CONDITION_VSWITCH_NETWORK_TYPE,
   0x11d48b4b,
   0xe77a,
   0x40b4,
   0x91, 0x55, 0x39, 0x2c, 0x90, 0x6c, 0x26, 0x08
);

// 7f4ef24b-b2c1-4938-ba33-a1ecbed512ba
DEFINE_GUID(
   FWPM_CONDITION_VSWITCH_SOURCE_INTERFACE_ID,
   0x7f4ef24b,
   0xb2c1,
   0x4938,
   0xba, 0x33, 0xa1, 0xec, 0xbe, 0xd5, 0x12, 0xba
);

// 8ed48be4-c926-49f6-a4f6-ef3030e3fc16
DEFINE_GUID(
   FWPM_CONDITION_VSWITCH_DESTINATION_INTERFACE_ID,
   0x8ed48be4,
   0xc926,
   0x49f6,
   0xa4, 0xf6, 0xef, 0x30, 0x30, 0xe3, 0xfc, 0x16
);

// 9c2a9ec2-9fc6-42bc-bdd8-406d4da0be64
DEFINE_GUID(
   FWPM_CONDITION_VSWITCH_SOURCE_VM_ID,
   0x9c2a9ec2,
   0x9fc6,
   0x42bc,
   0xbd, 0xd8, 0x40, 0x6d, 0x4d, 0xa0, 0xbe, 0x64
);

// 6106aace-4de1-4c84-9671-3637f8bcf731
DEFINE_GUID(
   FWPM_CONDITION_VSWITCH_DESTINATION_VM_ID,
   0x6106aace,
   0x4de1,
   0x4c84,
   0x96, 0x71, 0x36, 0x37, 0xf8, 0xbc, 0xf7, 0x31
);

// e6b040a2-edaf-4c36-908b-f2f58ae43807
DEFINE_GUID(
   FWPM_CONDITION_VSWITCH_SOURCE_INTERFACE_TYPE,
   0xe6b040a2,
   0xedaf,
   0x4c36,
   0x90, 0x8b, 0xf2, 0xf5, 0x8a, 0xe4, 0x38, 0x07
);

// fa9b3f06-2f1a-4c57-9e68-a7098b28dbfe
DEFINE_GUID(
   FWPM_CONDITION_VSWITCH_DESTINATION_INTERFACE_TYPE,
   0xfa9b3f06,
   0x2f1a,
   0x4c57,
   0x9e, 0x68, 0xa7, 0x09, 0x8b, 0x28, 0xdb, 0xfe
);

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

// 37a57699-5883-4963-92b8-3e704688b0ad
DEFINE_GUID(
   FWPM_CONDITION_ALE_SECURITY_ATTRIBUTE_FQBN_VALUE,
   0x37a57699,
   0x5883,
   0x4963,
   0x92, 0xb8, 0x3e, 0x70, 0x46, 0x88, 0xb0, 0xad
);

// 37a57700-5884-4964-92b8-3e704688b0ad
DEFINE_GUID(
   FWPM_CONDITION_IPSEC_SECURITY_REALM_ID,
   0x37a57700,
   0x5884,
   0x4964,
   0x92, 0xb8, 0x3e, 0x70, 0x46, 0x88, 0xb0, 0xad
);

// b1277b9a-b781-40fc-9671-e5f1b989f34e
DEFINE_GUID(
    FWPM_CONDITION_ALE_EFFECTIVE_NAME,
    0xb1277b9a,
    0xb781,
    0x40fc,
    0x96, 0x71, 0xe5, 0xf1, 0xb9, 0x89, 0xf3, 0x4e
);

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

// d9ee00de-c1ef-4617-bfe3-ffd8f5a08957
DEFINE_GUID(
   FWPM_CONDITION_IP_LOCAL_ADDRESS,
   0xd9ee00de,
   0xc1ef,
   0x4617,
   0xbf, 0xe3, 0xff, 0xd8, 0xf5, 0xa0, 0x89, 0x57
);

// b235ae9a-1d64-49b8-a44c-5ff3d9095045
DEFINE_GUID(
   FWPM_CONDITION_IP_REMOTE_ADDRESS,
   0xb235ae9a,
   0x1d64,
   0x49b8,
   0xa4, 0x4c, 0x5f, 0xf3, 0xd9, 0x09, 0x50, 0x45
);

// ae96897e-2e94-4bc9-b313-b27ee80e574d
DEFINE_GUID(
   FWPM_CONDITION_IP_SOURCE_ADDRESS,
   0xae96897e,
   0x2e94,
   0x4bc9,
   0xb3, 0x13, 0xb2, 0x7e, 0xe8, 0x0e, 0x57, 0x4d
);

// 2d79133b-b390-45c6-8699-acaceaafed33
DEFINE_GUID(
   FWPM_CONDITION_IP_DESTINATION_ADDRESS,
   0x2d79133b,
   0xb390,
   0x45c6,
   0x86, 0x99, 0xac, 0xac, 0xea, 0xaf, 0xed, 0x33
);

// 6ec7f6c4-376b-45d7-9e9c-d337cedcd237
DEFINE_GUID(
   FWPM_CONDITION_IP_LOCAL_ADDRESS_TYPE,
   0x6ec7f6c4,
   0x376b,
   0x45d7,
   0x9e, 0x9c, 0xd3, 0x37, 0xce, 0xdc, 0xd2, 0x37
);

// 1ec1b7c9-4eea-4f5e-b9ef-76beaaaf17ee
DEFINE_GUID(
   FWPM_CONDITION_IP_DESTINATION_ADDRESS_TYPE,
   0x1ec1b7c9,
   0x4eea,
   0x4f5e,
   0xb9, 0xef, 0x76, 0xbe, 0xaa, 0xaf, 0x17, 0xee
);

#if (NTDDI_VERSION >= NTDDI_WIN7)

// eabe448a-a711-4d64-85b7-3f76b65299c7
DEFINE_GUID(
   FWPM_CONDITION_IP_NEXTHOP_ADDRESS,
   0xeabe448a,
   0xa711,
   0x4d64,
   0x85, 0xb7, 0x3f, 0x76, 0xb6, 0x52, 0x99, 0xc7
);

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

// 4cd62a49-59c3-4969-b7f3-bda5d32890a4
DEFINE_GUID(
   FWPM_CONDITION_IP_LOCAL_INTERFACE,
   0x4cd62a49,
   0x59c3,
   0x4969,
   0xb7, 0xf3, 0xbd, 0xa5, 0xd3, 0x28, 0x90, 0xa4
);


#if (NTDDI_VERSION >= NTDDI_WIN6SP1)

// 618a9b6d-386b-4136-ad6e-b51587cfb1cd
DEFINE_GUID(
   FWPM_CONDITION_IP_ARRIVAL_INTERFACE,
   0x618a9b6d,
   0x386b,
   0x4136,
   0xad, 0x6e, 0xb5, 0x15, 0x87, 0xcf, 0xb1, 0xcd
);

// 89f990de-e798-4e6d-ab76-7c9558292e6f
DEFINE_GUID(
   FWPM_CONDITION_ARRIVAL_INTERFACE_TYPE,
   0x89f990de,
   0xe798,
   0x4e6d,
   0xab, 0x76, 0x7c, 0x95, 0x58, 0x29, 0x2e, 0x6f
);

// 511166dc-7a8c-4aa7-b533-95ab59fb0340
DEFINE_GUID(
   FWPM_CONDITION_ARRIVAL_TUNNEL_TYPE,
   0x511166dc,
   0x7a8c,
   0x4aa7,
   0xb5, 0x33, 0x95, 0xab, 0x59, 0xfb, 0x03, 0x40
);

// cc088db3-1792-4a71-b0f9-037d21cd828b
DEFINE_GUID(
   FWPM_CONDITION_ARRIVAL_INTERFACE_INDEX,
   0xcc088db3,
   0x1792,
   0x4a71,
   0xb0, 0xf9, 0x03, 0x7d, 0x21, 0xcd, 0x82, 0x8b
);

#if (NTDDI_VERSION >= NTDDI_WIN7)

// ef8a6122-0577-45a7-9aaf-825fbeb4fb95
DEFINE_GUID(
   FWPM_CONDITION_NEXTHOP_SUB_INTERFACE_INDEX,
   0xef8a6122,
   0x0577,
   0x45a7,
   0x9a, 0xaf, 0x82, 0x5f, 0xbe, 0xb4, 0xfb, 0x95
);

// 93ae8f5b-7f6f-4719-98c8-14e97429ef04
DEFINE_GUID(
   FWPM_CONDITION_IP_NEXTHOP_INTERFACE,
   0x93ae8f5b,
   0x7f6f,
   0x4719,
   0x98, 0xc8, 0x14, 0xe9, 0x74, 0x29, 0xef, 0x04
);

// 97537c6c-d9a3-4767-a381-e942675cd920
DEFINE_GUID(
   FWPM_CONDITION_NEXTHOP_INTERFACE_TYPE,
   0x97537c6c,
   0xd9a3,
   0x4767,
   0xa3, 0x81, 0xe9, 0x42, 0x67, 0x5c, 0xd9, 0x20
);

// 72b1a111-987b-4720-99dd-c7c576fa2d4c
DEFINE_GUID(
   FWPM_CONDITION_NEXTHOP_TUNNEL_TYPE,
   0x72b1a111,
   0x987b,
   0x4720,
   0x99, 0xdd, 0xc7, 0xc5, 0x76, 0xfa, 0x2d, 0x4c
);

// 138e6888-7ab8-4d65-9ee8-0591bcf6a494
DEFINE_GUID(
   FWPM_CONDITION_NEXTHOP_INTERFACE_INDEX,
   0x138e6888,
   0x7ab8,
   0x4d65,
   0x9e, 0xe8, 0x05, 0x91, 0xbc, 0xf6, 0xa4, 0x94
);

// 46ea1551-2255-492b-8019-aabeee349f40
DEFINE_GUID(
   FWPM_CONDITION_ORIGINAL_PROFILE_ID,
   0x46ea1551,
   0x2255,
   0x492b,
   0x80, 0x19, 0xaa, 0xbe, 0xee, 0x34, 0x9f, 0x40
);

// ab3033c9-c0e3-4759-937d-5758c65d4ae3
DEFINE_GUID(
   FWPM_CONDITION_CURRENT_PROFILE_ID,
   0xab3033c9,
   0xc0e3,
   0x4759,
   0x93, 0x7d, 0x57, 0x58, 0xc6, 0x5d, 0x4a, 0xe3
);

// 4ebf7562-9f18-4d06-9941-a7a625744d71
DEFINE_GUID(
   FWPM_CONDITION_LOCAL_INTERFACE_PROFILE_ID,
   0x4ebf7562,
   0x9f18,
   0x4d06,
   0x99, 0x41, 0xa7, 0xa6, 0x25, 0x74, 0x4d, 0x71
);

// cdfe6aab-c083-4142-8679-c08f95329c61
DEFINE_GUID(
   FWPM_CONDITION_ARRIVAL_INTERFACE_PROFILE_ID,
   0xcdfe6aab,
   0xc083,
   0x4142,
   0x86, 0x79, 0xc0, 0x8f, 0x95, 0x32, 0x9c, 0x61
);

// d7ff9a56-cdaa-472b-84db-d23963c1d1bf
DEFINE_GUID(
   FWPM_CONDITION_NEXTHOP_INTERFACE_PROFILE_ID,
   0xd7ff9a56,
   0xcdaa,
   0x472b,
   0x84, 0xdb, 0xd2, 0x39, 0x63, 0xc1, 0xd1, 0xbf
);


// 11205e8c-11ae-457a-8a44-477026dd764a
DEFINE_GUID(
   FWPM_CONDITION_REAUTHORIZE_REASON,
   0x11205e8c,
   0x11ae,
   0x457a,
   0x8a, 0x44, 0x47, 0x70, 0x26, 0xdd, 0x76, 0x4a
);

// 076dfdbe-c56c-4f72-ae8a-2cfe7e5c8286
DEFINE_GUID(
   FWPM_CONDITION_ORIGINAL_ICMP_TYPE,
   0x076dfdbe,
   0xc56c,
   0x4f72,
   0xae, 0x8a, 0x2c, 0xfe, 0x7e, 0x5c, 0x82, 0x86
);

// da50d5c8-fa0d-4c89-b032-6e62136d1e96
DEFINE_GUID(
   FWPM_CONDITION_IP_PHYSICAL_ARRIVAL_INTERFACE,
   0xda50d5c8,
   0xfa0d,
   0x4c89,
   0xb0, 0x32, 0x6e, 0x62, 0x13, 0x6d, 0x1e, 0x96
);

// f09bd5ce-5150-48be-b098-c25152fb1f92
DEFINE_GUID(
   FWPM_CONDITION_IP_PHYSICAL_NEXTHOP_INTERFACE,
   0xf09bd5ce,
   0x5150,
   0x48be,
   0xb0, 0x98, 0xc2, 0x51, 0x52, 0xfb, 0x1f, 0x92
);

// cce68d5e-053b-43a8-9a6f-33384c28e4f6
DEFINE_GUID(
   FWPM_CONDITION_INTERFACE_QUARANTINE_EPOCH,
   0xcce68d5e,
   0x053b,
   0x43a8,
   0x9a, 0x6f, 0x33, 0x38, 0x4c, 0x28, 0xe4, 0xf6
);

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#endif // (NTDDI_VERSION >= NTDDI_WIN6SP1)

// daf8cd14-e09e-4c93-a5ae-c5c13b73ffca
DEFINE_GUID(
   FWPM_CONDITION_INTERFACE_TYPE,
   0xdaf8cd14,
   0xe09e,
   0x4c93,
   0xa5, 0xae, 0xc5, 0xc1, 0x3b, 0x73, 0xff, 0xca
);

#if (NTDDI_VERSION >= NTDDI_WIN6SP1)

#define FWPM_CONDITION_LOCAL_INTERFACE_TYPE FWPM_CONDITION_INTERFACE_TYPE

#endif // (NTDDI_VERSION >= NTDDI_WIN6SP1)

// 77a40437-8779-4868-a261-f5a902f1c0cd
DEFINE_GUID(
   FWPM_CONDITION_TUNNEL_TYPE,
   0x77a40437,
   0x8779,
   0x4868,
   0xa2, 0x61, 0xf5, 0xa9, 0x02, 0xf1, 0xc0, 0xcd
);

#if (NTDDI_VERSION >= NTDDI_WIN6SP1)

#define FWPM_CONDITION_LOCAL_TUNNEL_TYPE FWPM_CONDITION_TUNNEL_TYPE

#endif // (NTDDI_VERSION >= NTDDI_WIN6SP1)

// 1076b8a5-6323-4c5e-9810-e8d3fc9e6136
DEFINE_GUID(
   FWPM_CONDITION_IP_FORWARD_INTERFACE,
   0x1076b8a5,
   0x6323,
   0x4c5e,
   0x98, 0x10, 0xe8, 0xd3, 0xfc, 0x9e, 0x61, 0x36
);

// 3971ef2b-623e-4f9a-8cb1-6e79b806b9a7
DEFINE_GUID(
   FWPM_CONDITION_IP_PROTOCOL,
   0x3971ef2b,
   0x623e,
   0x4f9a,
   0x8c, 0xb1, 0x6e, 0x79, 0xb8, 0x06, 0xb9, 0xa7
);

// 0c1ba1af-5765-453f-af22-a8f791ac775b
DEFINE_GUID(
   FWPM_CONDITION_IP_LOCAL_PORT,
   0x0c1ba1af,
   0x5765,
   0x453f,
   0xaf, 0x22, 0xa8, 0xf7, 0x91, 0xac, 0x77, 0x5b
);

#define FWPM_CONDITION_ICMP_TYPE FWPM_CONDITION_IP_LOCAL_PORT

// c35a604d-d22b-4e1a-91b4-68f674ee674b
DEFINE_GUID(
   FWPM_CONDITION_IP_REMOTE_PORT,
   0xc35a604d,
   0xd22b,
   0x4e1a,
   0x91, 0xb4, 0x68, 0xf6, 0x74, 0xee, 0x67, 0x4b
);

#define FWPM_CONDITION_ICMP_CODE FWPM_CONDITION_IP_REMOTE_PORT

// 4672a468-8a0a-4202-abb4-849e92e66809
DEFINE_GUID(
   FWPM_CONDITION_EMBEDDED_LOCAL_ADDRESS_TYPE,
   0x4672a468,
   0x8a0a,
   0x4202,
   0xab, 0xb4, 0x84, 0x9e, 0x92, 0xe6, 0x68, 0x09
);

// 77ee4b39-3273-4671-b63b-ab6feb66eeb6
DEFINE_GUID(
   FWPM_CONDITION_EMBEDDED_REMOTE_ADDRESS,
   0x77ee4b39,
   0x3273,
   0x4671,
   0xb6, 0x3b, 0xab, 0x6f, 0xeb, 0x66, 0xee, 0xb6
);

// 07784107-a29e-4c7b-9ec7-29c44afafdbc
DEFINE_GUID(
   FWPM_CONDITION_EMBEDDED_PROTOCOL,
   0x07784107,
   0xa29e,
   0x4c7b,
   0x9e, 0xc7, 0x29, 0xc4, 0x4a, 0xfa, 0xfd, 0xbc
);

// bfca394d-acdb-484e-b8e6-2aff79757345
DEFINE_GUID(
   FWPM_CONDITION_EMBEDDED_LOCAL_PORT,
   0xbfca394d,
   0xacdb,
   0x484e,
   0xb8, 0xe6, 0x2a, 0xff, 0x79, 0x75, 0x73, 0x45
);

// cae4d6a1-2968-40ed-a4ce-547160dda88d
DEFINE_GUID(
   FWPM_CONDITION_EMBEDDED_REMOTE_PORT,
   0xcae4d6a1,
   0x2968,
   0x40ed,
   0xa4, 0xce, 0x54, 0x71, 0x60, 0xdd, 0xa8, 0x8d
);

// 632ce23b-5167-435c-86d7-e903684aa80c
DEFINE_GUID(
   FWPM_CONDITION_FLAGS,
   0x632ce23b,
   0x5167,
   0x435c,
   0x86, 0xd7, 0xe9, 0x03, 0x68, 0x4a, 0xa8, 0x0c
);

// 8784c146-ca97-44d6-9fd1-19fb1840cbf7
DEFINE_GUID(
   FWPM_CONDITION_DIRECTION,
   0x8784c146,
   0xca97,
   0x44d6,
   0x9f, 0xd1, 0x19, 0xfb, 0x18, 0x40, 0xcb, 0xf7
);

// 667fd755-d695-434a-8af5-d3835a1259bc
DEFINE_GUID(
   FWPM_CONDITION_INTERFACE_INDEX,
   0x667fd755,
   0xd695,
   0x434a,
   0x8a, 0xf5, 0xd3, 0x83, 0x5a, 0x12, 0x59, 0xbc
);

#if (NTDDI_VERSION >= NTDDI_WIN6SP1)

#define FWPM_CONDITION_LOCAL_INTERFACE_INDEX FWPM_CONDITION_INTERFACE_INDEX

#endif // (NTDDI_VERSION >= NTDDI_WIN6SP1)

// 0cd42473-d621-4be3-ae8c-72a348d283e1
DEFINE_GUID(
   FWPM_CONDITION_SUB_INTERFACE_INDEX,
   0x0cd42473,
   0xd621,
   0x4be3,
   0xae, 0x8c, 0x72, 0xa3, 0x48, 0xd2, 0x83, 0xe1
);

#if (NTDDI_VERSION >= NTDDI_WIN6SP1)

#define FWPM_CONDITION_ARRIVAL_SUB_INTERFACE_INDEX \
        FWPM_CONDITION_SUB_INTERFACE_INDEX

#endif // (NTDDI_VERSION >= NTDDI_WIN6SP1)

// 2311334d-c92d-45bf-9496-edf447820e2d
DEFINE_GUID(
   FWPM_CONDITION_SOURCE_INTERFACE_INDEX,
   0x2311334d,
   0xc92d,
   0x45bf,
   0x94, 0x96, 0xed, 0xf4, 0x47, 0x82, 0x0e, 0x2d
);

// 055edd9d-acd2-4361-8dab-f9525d97662f
DEFINE_GUID(
   FWPM_CONDITION_SOURCE_SUB_INTERFACE_INDEX,
   0x055edd9d,
   0xacd2,
   0x4361,
   0x8d, 0xab, 0xf9, 0x52, 0x5d, 0x97, 0x66, 0x2f
);

// 35cf6522-4139-45ee-a0d5-67b80949d879
DEFINE_GUID(
   FWPM_CONDITION_DESTINATION_INTERFACE_INDEX,
   0x35cf6522,
   0x4139,
   0x45ee,
   0xa0, 0xd5, 0x67, 0xb8, 0x09, 0x49, 0xd8, 0x79
);

// 2b7d4399-d4c7-4738-a2f5-e994b43da388
DEFINE_GUID(
   FWPM_CONDITION_DESTINATION_SUB_INTERFACE_INDEX,
   0x2b7d4399,
   0xd4c7,
   0x4738,
   0xa2, 0xf5, 0xe9, 0x94, 0xb4, 0x3d, 0xa3, 0x88
);

// d78e1e87-8644-4ea5-9437-d809ecefc971
DEFINE_GUID(
   FWPM_CONDITION_ALE_APP_ID,
   0xd78e1e87,
   0x8644,
   0x4ea5,
   0x94, 0x37, 0xd8, 0x09, 0xec, 0xef, 0xc9, 0x71
);

#if (NTDDI_VERSION >= NTDDI_WIN8)

// 0e6cd086-e1fb-4212-842f-8a9f993fb3f6
DEFINE_GUID(
   FWPM_CONDITION_ALE_ORIGINAL_APP_ID,
   0x0e6cd086,
   0xe1fb,
   0x4212,
   0x84, 0x2f, 0x8a, 0x9f, 0x99, 0x3f, 0xb3, 0xf6
);

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

// af043a0a-b34d-4f86-979c-c90371af6e66
DEFINE_GUID(
   FWPM_CONDITION_ALE_USER_ID,
   0xaf043a0a,
   0xb34d,
   0x4f86,
   0x97, 0x9c, 0xc9, 0x03, 0x71, 0xaf, 0x6e, 0x66
);

// f63073b7-0189-4ab0-95a4-6123cbfab862
DEFINE_GUID(
   FWPM_CONDITION_ALE_REMOTE_USER_ID,
   0xf63073b7,
   0x0189,
   0x4ab0,
   0x95, 0xa4, 0x61, 0x23, 0xcb, 0xfa, 0xb8, 0x62
);

// 1aa47f51-7f93-4508-a271-81abb00c9cab
DEFINE_GUID(
   FWPM_CONDITION_ALE_REMOTE_MACHINE_ID,
   0x1aa47f51,
   0x7f93,
   0x4508,
   0xa2, 0x71, 0x81, 0xab, 0xb0, 0x0c, 0x9c, 0xab
);

// 1c974776-7182-46e9-afd3-b02910e30334
DEFINE_GUID(
   FWPM_CONDITION_ALE_PROMISCUOUS_MODE,
   0x1c974776,
   0x7182,
   0x46e9,
   0xaf, 0xd3, 0xb0, 0x29, 0x10, 0xe3, 0x03, 0x34
);

// b9f4e088-cb98-4efb-a2c7-ad07332643db
DEFINE_GUID(
   FWPM_CONDITION_ALE_SIO_FIREWALL_SYSTEM_PORT,
   0xb9f4e088,
   0xcb98,
   0x4efb,
   0xa2, 0xc7, 0xad, 0x07, 0x33, 0x26, 0x43, 0xdb
);

#if (NTDDI_VERSION >= NTDDI_WIN7)
#define FWPM_CONDITION_ALE_SIO_FIREWALL_SOCKET_PROPERTY \
        FWPM_CONDITION_ALE_SIO_FIREWALL_SYSTEM_PORT

// b482d227-1979-4a98-8044-18bbe6237542
DEFINE_GUID(
   FWPM_CONDITION_ALE_REAUTH_REASON,
   0xb482d227,
   0x1979,
   0x4a98,
   0x80, 0x44, 0x18, 0xbb, 0xe6, 0x23, 0x75, 0x42
);

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

// 46275a9d-c03f-4d77-b784-1c57f4d02753
DEFINE_GUID(
   FWPM_CONDITION_ALE_NAP_CONTEXT,
   0x46275a9d,
   0xc03f,
   0x4d77,
   0xb7, 0x84, 0x1c, 0x57, 0xf4, 0xd0, 0x27, 0x53
);

#if (NTDDI_VERSION >= NTDDI_WIN7)
// 35d0ea0e-15ca-492b-900e-97fd46352cce
DEFINE_GUID(
   FWPM_CONDITION_KM_AUTH_NAP_CONTEXT,
   0x35d0ea0e,
   0x15ca,
   0x492b,
   0x90, 0x0e, 0x97, 0xfd, 0x46, 0x35, 0x2c, 0xce
);
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

// 9bf0ee66-06c9-41b9-84da-288cb43af51f
DEFINE_GUID(
    FWPM_CONDITION_REMOTE_USER_TOKEN,
    0x9bf0ee66,
    0x06c9,
    0x41b9,
    0x84, 0xda, 0x28, 0x8c, 0xb4, 0x3a, 0xf5, 0x1f
);

// 7c9c7d9f-0075-4d35-a0d1-8311c4cf6af1
DEFINE_GUID(
   FWPM_CONDITION_RPC_IF_UUID,
   0x7c9c7d9f,
   0x0075,
   0x4d35,
   0xa0, 0xd1, 0x83, 0x11, 0xc4, 0xcf, 0x6a, 0xf1
);

// eabfd9b7-1262-4a2e-adaa-5f96f6fe326d
DEFINE_GUID(
   FWPM_CONDITION_RPC_IF_VERSION,
   0xeabfd9b7,
   0x1262,
   0x4a2e,
   0xad, 0xaa, 0x5f, 0x96, 0xf6, 0xfe, 0x32, 0x6d
);

// 238a8a32-3199-467d-871c-272621ab3896
DEFINE_GUID(
    FWPM_CONDITION_RPC_IF_FLAG,
    0x238a8a32,
    0x3199,
    0x467d,
    0x87, 0x1c, 0x27, 0x26, 0x21, 0xab, 0x38, 0x96
);

// ff2e7b4d-3112-4770-b636-4d24ae3a6af2
DEFINE_GUID(
    FWPM_CONDITION_DCOM_APP_ID,
    0xff2e7b4d,
    0x3112,
    0x4770,
    0xb6, 0x36, 0x4d, 0x24, 0xae, 0x3a, 0x6a, 0xf2
);

// d024de4d-deaa-4317-9c85-e40ef6e140c3
DEFINE_GUID(
    FWPM_CONDITION_IMAGE_NAME,
    0xd024de4d,
    0xdeaa,
    0x4317,
    0x9c, 0x85, 0xe4, 0x0e, 0xf6, 0xe1, 0x40, 0xc3
);

// 2717bc74-3a35-4ce7-b7ef-c838fabdec45
DEFINE_GUID(
    FWPM_CONDITION_RPC_PROTOCOL,
    0x2717bc74,
    0x3a35,
    0x4ce7,
    0xb7, 0xef, 0xc8, 0x38, 0xfa, 0xbd, 0xec, 0x45
);

// daba74ab-0d67-43e7-986e-75b84f82f594
DEFINE_GUID(
   FWPM_CONDITION_RPC_AUTH_TYPE,
   0xdaba74ab,
   0x0d67,
   0x43e7,
   0x98, 0x6e, 0x75, 0xb8, 0x4f, 0x82, 0xf5, 0x94
);

// e5a0aed5-59ac-46ea-be05-a5f05ecf446e
DEFINE_GUID(
   FWPM_CONDITION_RPC_AUTH_LEVEL,
   0xe5a0aed5,
   0x59ac,
   0x46ea,
   0xbe, 0x05, 0xa5, 0xf0, 0x5e, 0xcf, 0x44, 0x6e
);

// 0d306ef0-e974-4f74-b5c7-591b0da7d562
DEFINE_GUID(
   FWPM_CONDITION_SEC_ENCRYPT_ALGORITHM,
   0x0d306ef0,
   0xe974,
   0x4f74,
   0xb5, 0xc7, 0x59, 0x1b, 0x0d, 0xa7, 0xd5, 0x62
);

// 4772183b-ccf8-4aeb-bce1-c6c6161c8fe4
DEFINE_GUID(
    FWPM_CONDITION_SEC_KEY_SIZE,
    0x4772183b,
    0xccf8,
    0x4aeb,
    0xbc, 0xe1, 0xc6, 0xc6, 0x16, 0x1c, 0x8f, 0xe4
);

// 03a629cb-6e52-49f8-9c41-5709633c09cf
DEFINE_GUID(
   FWPM_CONDITION_IP_LOCAL_ADDRESS_V4,
   0x03a629cb,
   0x6e52,
   0x49f8,
   0x9c, 0x41, 0x57, 0x09, 0x63, 0x3c, 0x09, 0xcf
);

// 2381be84-7524-45b3-a05b-1e637d9c7a6a
DEFINE_GUID(
   FWPM_CONDITION_IP_LOCAL_ADDRESS_V6,
   0x2381be84,
   0x7524,
   0x45b3,
   0xa0, 0x5b, 0x1e, 0x63, 0x7d, 0x9c, 0x7a, 0x6a
);

// 1bd0741d-e3df-4e24-8634-762046eef6eb
DEFINE_GUID(
    FWPM_CONDITION_PIPE,
    0x1bd0741d,
    0xe3df,
    0x4e24,
    0x86, 0x34, 0x76, 0x20, 0x46, 0xee, 0xf6, 0xeb
);

// 1febb610-3bcc-45e1-bc36-2e067e2cb186
DEFINE_GUID(
   FWPM_CONDITION_IP_REMOTE_ADDRESS_V4,
   0x1febb610,
   0x3bcc,
   0x45e1,
   0xbc, 0x36, 0x2e, 0x06, 0x7e, 0x2c, 0xb1, 0x86
);

// 246e1d8c-8bee-4018-9b98-31d4582f3361
DEFINE_GUID(
   FWPM_CONDITION_IP_REMOTE_ADDRESS_V6,
   0x246e1d8c,
   0x8bee,
   0x4018,
   0x9b, 0x98, 0x31, 0xd4, 0x58, 0x2f, 0x33, 0x61
);

// d58efb76-aab7-4148-a87e-9581134129b9
DEFINE_GUID(
   FWPM_CONDITION_RPC_OPNUM, 
   0xd58efb76,
   0xaab7,
   0x4148,
   0xa8, 0x7e, 0x95, 0x81, 0x13, 0x41, 0x29, 0xb9
);

// e31180a8-bbbd-4d14-a65e-7157b06233bb
DEFINE_GUID(
    FWPM_CONDITION_PROCESS_WITH_RPC_IF_UUID,
    0xe31180a8,
    0xbbbd,
    0x4d14,
    0xa6, 0x5e, 0x71, 0x57, 0xb0, 0x62, 0x33, 0xbb
);

// dccea0b9-0886-4360-9c6a-ab043a24fba9
DEFINE_GUID(
    FWPM_CONDITION_RPC_EP_VALUE,
    0xdccea0b9,
    0x0886,
    0x4360,
    0x9c, 0x6a, 0xab, 0x04, 0x3a, 0x24, 0xfb, 0xa9
);

// 218b814a-0a39-49b8-8e71-c20c39c7dd2e
DEFINE_GUID(
    FWPM_CONDITION_RPC_EP_FLAGS,
    0x218b814a,
    0x0a39,
    0x49b8,
    0x8e, 0x71, 0xc2, 0x0c, 0x39, 0xc7, 0xdd, 0x2e
);

// c228fc1e-403a-4478-be05-c9baa4c05ace
DEFINE_GUID(
    FWPM_CONDITION_CLIENT_TOKEN,
    0xc228fc1e,
    0x403a,
    0x4478,
    0xbe, 0x05, 0xc9, 0xba, 0xa4, 0xc0, 0x5a, 0xce
);

// b605a225-c3b3-48c7-9833-7aefa9527546
DEFINE_GUID(
    FWPM_CONDITION_RPC_SERVER_NAME,
    0xb605a225,
    0xc3b3,
    0x48c7,
    0x98, 0x33, 0x7a, 0xef, 0xa9, 0x52, 0x75, 0x46
);

// 8090f645-9ad5-4e3b-9f9f-8023ca097909
DEFINE_GUID(
    FWPM_CONDITION_RPC_SERVER_PORT,
    0x8090f645,
    0x9ad5,
    0x4e3b,
    0x9f, 0x9f, 0x80, 0x23, 0xca, 0x09, 0x79, 0x09
);

// 40953fe2-8565-4759-8488-1771b4b4b5db
DEFINE_GUID(
    FWPM_CONDITION_RPC_PROXY_AUTH_TYPE,
    0x40953fe2,
    0x8565,
    0x4759,
    0x84, 0x88, 0x17, 0x71, 0xb4, 0xb4, 0xb5, 0xdb
);

// a3ec00c7-05f4-4df7-91f2-5f60d91ff443
DEFINE_GUID(
    FWPM_CONDITION_CLIENT_CERT_KEY_LENGTH,
    0xa3ec00c7,
    0x05f4,
    0x4df7,
    0x91, 0xf2, 0x5f, 0x60, 0xd9, 0x1f, 0xf4, 0x43
);

// c491ad5e-f882-4283-b916-436b103ff4ad
DEFINE_GUID(
    FWPM_CONDITION_CLIENT_CERT_OID,
    0xc491ad5e,
    0xf882,
    0x4283,
    0xb9, 0x16, 0x43, 0x6b, 0x10, 0x3f, 0xf4, 0xad
);

// 206e9996-490e-40cf-b831-b38641eb6fcb
DEFINE_GUID(
   FWPM_CONDITION_NET_EVENT_TYPE,
   0x206e9996,
   0x490e,
   0x40cf,
   0xb8, 0x31, 0xb3, 0x86, 0x41, 0xeb, 0x6f, 0xcb
);

#if (NTDDI_VERSION >= NTDDI_WIN7)
// 9b539082-eb90-4186-a6cc-de5b63235016
DEFINE_GUID(
   FWPM_CONDITION_PEER_NAME,
   0x9b539082,
   0xeb90,
   0x4186,
   0xa6, 0xcc, 0xde, 0x5b, 0x63, 0x23, 0x50, 0x16
);

//f68166fd-0682-4c89-b8f5-86436c7ef9b7
DEFINE_GUID(
   FWPM_CONDITION_REMOTE_ID,
   0xf68166fd,
   0x0682,
   0x4c89,
   0xb8, 0xf5, 0x86, 0x43, 0x6c, 0x7e, 0xf9, 0xb7
);

//eb458cd5-da7b-4ef9-8d43-7b0a840332f2
DEFINE_GUID(
   FWPM_CONDITION_AUTHENTICATION_TYPE,
   0xeb458cd5,
   0xda7b,
   0x4ef9,
   0x8d, 0x43, 0x7b, 0x0a, 0x84, 0x03, 0x32, 0xf2
);

//ff0f5f49-0ceb-481b-8638-1479791f3f2c
DEFINE_GUID(
   FWPM_CONDITION_KM_TYPE,
   0xff0f5f49,
   0x0ceb,
   0x481b,
   0x86, 0x38, 0x14, 0x79, 0x79, 0x1f, 0x3f, 0x2c
);

//feef4582-ef8f-4f7b-858b-9077d122de47
DEFINE_GUID(
   FWPM_CONDITION_KM_MODE,
   0xfeef4582,
   0xef8f,
   0x4f7b,
   0x85, 0x8b, 0x90, 0x77, 0xd1, 0x22, 0xde, 0x47
);

//ad37dee3-722f-45cc-a4e3-068048124452
DEFINE_GUID(
   FWPM_CONDITION_IPSEC_POLICY_KEY,
   0xad37dee3,
   0x722f,
   0x45cc,
   0xa4, 0xe3, 0x06, 0x80, 0x48, 0x12, 0x44, 0x52
);

#if (NTDDI_VERSION >= NTDDI_WIN8)
//f64fc6d1-f9cb-43d2-8a5f-e13bc894f265
DEFINE_GUID(
   FWPM_CONDITION_QM_MODE,
   0xf64fc6d1,
   0xf9cb,
   0x43d2,
   0x8a, 0x5f, 0xe1, 0x3b, 0xc8, 0x94, 0xf2, 0x65
);

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

// 35A791AB-04AC-4FF2-A6BB-DA6CFAC71806
DEFINE_GUID(
    FWPM_CONDITION_COMPARTMENT_ID,
    0x35a791ab,
    0x4ac,
    0x4ff2,
    0xa6, 0xbb, 0xda, 0x6c, 0xfa, 0xc7, 0x18, 0x6
);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

//678f4deb-45af-4882-93fe-19d4729d9834
DEFINE_GUID(
   FWPM_CONDITION_RESERVED0,
   0x678f4deb,
   0x45af,
   0x4882,
   0x93, 0xfe, 0x19, 0xd4, 0x72, 0x9d, 0x98, 0x34
   );

//d818f827-5c69-48eb-bf80-d86b17755f97
DEFINE_GUID(
   FWPM_CONDITION_RESERVED1,
   0xd818f827,
   0x5c69,
   0x48eb,
   0xbf, 0x80, 0xd8, 0x6b, 0x17, 0x75, 0x5f, 0x97
   );

//53d4123d-e15b-4e84-b7a8-dce16f7b62d9
DEFINE_GUID(
   FWPM_CONDITION_RESERVED2,
   0x53d4123d,
   0xe15b,
   0x4e84,
   0xb7, 0xa8,0xdc, 0xe1, 0x6f, 0x7b, 0x62, 0xd9
   );

//7f6e8ca3-6606-4932-97c7-e1f20710af3b
DEFINE_GUID(
   FWPM_CONDITION_RESERVED3,
   0x7f6e8ca3,
   0x6606,
   0x4932,
   0x97, 0xc7, 0xe1, 0xf2, 0x07, 0x10, 0xaf, 0x3b
   );

//5f58e642-b937-495e-a94b-f6b051a49250
DEFINE_GUID(
   FWPM_CONDITION_RESERVED4,
   0x5f58e642,
   0xb937,
   0x495e,
   0xa9, 0x4b, 0xf6, 0xb0, 0x51, 0xa4, 0x92, 0x50
);

//9ba8f6cd-f77c-43e6-8847-11939dc5db5a
DEFINE_GUID(
   FWPM_CONDITION_RESERVED5,
   0x9ba8f6cd,
   0xf77c,
   0x43e6,
   0x88, 0x47, 0x11, 0x93, 0x9d, 0xc5, 0xdb, 0x5a
);

//f13d84bd-59d5-44c4-8817-5ecdae1805bd
DEFINE_GUID(
    FWPM_CONDITION_RESERVED6,
    0xf13d84bd,
    0x59d5,
    0x44c4,
    0x88, 0x17, 0x5e, 0xcd, 0xae, 0x18, 0x05, 0xbd
);

//65a0f930-45dd-4983-aa33-efc7b611af08
DEFINE_GUID(
    FWPM_CONDITION_RESERVED7,
    0x65a0f930,
    0x45dd,
    0x4983,
    0xaa, 0x33, 0xef, 0xc7, 0xb6, 0x11, 0xaf, 0x08
);

//4f424974-0c12-4816-9b47-9a547db39a32
DEFINE_GUID(
    FWPM_CONDITION_RESERVED8,
    0x4f424974,
    0x0c12,
    0x4816,
    0x9b, 0x47, 0x9a, 0x54, 0x7d, 0xb3, 0x9a, 0x32
);

//ce78e10f-13ff-4c70-8643-36ad1879afa3
DEFINE_GUID(
    FWPM_CONDITION_RESERVED9,
    0xce78e10f,
    0x13ff,
    0x4c70,
    0x86, 0x43, 0x36, 0xad, 0x18, 0x79, 0xaf, 0xa3
);

//b979e282-d621-4c8c-b184-b105a61c36ce
DEFINE_GUID(
    FWPM_CONDITION_RESERVED10,
    0xb979e282,
    0xd621,
    0x4c8c,
    0xb1, 0x84, 0xb1, 0x05, 0xa6, 0x1c, 0x36, 0xce
);

//2d62ee4d-023d-411f-9582-43acbb795975
DEFINE_GUID(
    FWPM_CONDITION_RESERVED11,
    0x2d62ee4d,
    0x023d,
    0x411f,
    0x95, 0x82, 0x43, 0xac, 0xbb, 0x79, 0x59, 0x75
);

//a3677c32-7e35-4ddc-93da-e8c33fc923c7
DEFINE_GUID(
    FWPM_CONDITION_RESERVED12,
    0xa3677c32,
    0x7e35,
    0x4ddc,
    0x93, 0xda, 0xe8, 0xc3, 0x3f, 0xc9, 0x23, 0xc7
);

#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)

// 335a3e90-84aa-42f5-9e6f-59309536a44c
DEFINE_GUID(
    FWPM_CONDITION_RESERVED13,
    0x335a3e90,
    0x84aa,
    0x42f5,
    0x9e, 0x6f, 0x59, 0x30, 0x95, 0x36, 0xa4, 0x4c
);

// 30e44da2-2f1a-4116-a559-f907de83604a
DEFINE_GUID(
    FWPM_CONDITION_RESERVED14,
    0x30e44da2,
    0x2f1a,
    0x4116,
    0xa5, 0x59, 0xf9, 0x07, 0xde, 0x83, 0x60, 0x4a
);

// bab8340f-afe0-43d1-80d8-5ca456962de3
DEFINE_GUID(
    FWPM_CONDITION_RESERVED15,
    0xbab8340f,
    0xafe0,
    0x43d1,
    0x80, 0xd8, 0x5c, 0xa4, 0x56, 0x96, 0x2d, 0xe3
);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_19H1)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

///////////////////////////////////////////////////////////////////////////////
//
// GUIDs for built-in providers.
//
///////////////////////////////////////////////////////////////////////////////

// 10ad9216-ccde-456c-8b16-e9f04e60a90b
DEFINE_GUID(
   FWPM_PROVIDER_IKEEXT,
   0x10ad9216,
   0xccde,
   0x456c,
   0x8b, 0x16, 0xe9, 0xf0, 0x4e, 0x60, 0xa9, 0x0b
);

#if (NTDDI_VERSION >= NTDDI_WIN7)
// 3c6c05a9-c05c-4bb9-8338-2327814ce8bf
DEFINE_GUID(
   FWPM_PROVIDER_IPSEC_DOSP_CONFIG,
   0x3c6c05a9,
   0xc05c,
   0x4bb9,
   0x83, 0x38, 0x23, 0x27, 0x81, 0x4c, 0xe8, 0xbf
);
#endif

// 896aa19e-9a34-4bcb-ae79-beb9127c84b9
DEFINE_GUID(
   FWPM_PROVIDER_TCP_CHIMNEY_OFFLOAD,
   0x896aa19e,
   0x9a34,
   0x4bcb,
   0xae, 0x79, 0xbe, 0xb9, 0x12, 0x7c, 0x84, 0xb9
);

#if (NTDDI_VERSION >= NTDDI_WIN8)
// 76cfcd30-3394-432d-bed3-441ae50e63c3
DEFINE_GUID(
   FWPM_PROVIDER_TCP_TEMPLATES,
   0x76cfcd30,
   0x3394,
   0x432d,
   0xbe, 0xd3, 0x44, 0x1a, 0xe5, 0x0e, 0x63, 0xc3
);

// 4b153735-1049-4480-aab4-d1b9bdc03710
DEFINE_GUID(
   FWPM_PROVIDER_MPSSVC_WSH,
   0x4b153735,
   0x1049,
   0x4480,
   0xaa, 0xb4, 0xd1, 0xb9, 0xbd, 0xc0, 0x37, 0x10
);

// decc16ca-3f33-4346-be1e-8fb4ae0f3d62
DEFINE_GUID(
   FWPM_PROVIDER_MPSSVC_WF,
   0xdecc16ca,
   0x3f33,
   0x4346,
   0xbe, 0x1e, 0x8f, 0xb4, 0xae, 0x0f, 0x3d, 0x62
);

// a90296f7-46b8-4457-8f84-b05e05d3c622
DEFINE_GUID(
   FWPM_PROVIDER_MPSSVC_EDP,
   0xa90296f7,
   0x46b8,
   0x4457,
   0x8f, 0x84, 0xb0, 0x5e, 0x05, 0xd3, 0xc6, 0x22
);

// d0718ff9-44da-4f50-9dc2-c963a4247613
DEFINE_GUID(
   FWPM_PROVIDER_MPSSVC_TENANT_RESTRICTIONS,
   0xd0718ff9,
   0x44da,
   0x4f50,
   0x9d, 0xc2, 0xc9, 0x63, 0xa4, 0x24, 0x76, 0x13
);

// 3cc2631f-2d5d-43a0-b174-614837d863a1
DEFINE_GUID(
   FWPM_PROVIDER_MPSSVC_APP_ISOLATION,
   0x3cc2631f,
   0x2d5d,
   0x43a0,
   0xb1, 0x74, 0x61, 0x48, 0x37, 0xd8, 0x63, 0xa1
);
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

///////////////////////////////////////////////////////////////////////////////
//
// GUIDs for built-in callouts.
//
///////////////////////////////////////////////////////////////////////////////

// 5132900d-5e84-4b5f-80e4-01741e81ff10
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_INBOUND_TRANSPORT_V4,
   0x5132900d,
   0x5e84,
   0x4b5f,
   0x80, 0xe4, 0x01, 0x74, 0x1e, 0x81, 0xff, 0x10
);

// 49d3ac92-2a6c-4dcf-955f-1c3be009dd99
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_INBOUND_TRANSPORT_V6,
   0x49d3ac92,
   0x2a6c,
   0x4dcf,
   0x95, 0x5f, 0x1c, 0x3b, 0xe0, 0x09, 0xdd, 0x99
);

// 4b46bf0a-4523-4e57-aa38-a87987c910d9
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_OUTBOUND_TRANSPORT_V4,
   0x4b46bf0a,
   0x4523,
   0x4e57,
   0xaa, 0x38, 0xa8, 0x79, 0x87, 0xc9, 0x10, 0xd9
);

// 38d87722-ad83-4f11-a91f-df0fb077225b
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_OUTBOUND_TRANSPORT_V6,
   0x38d87722,
   0xad83,
   0x4f11,
   0xa9, 0x1f, 0xdf, 0x0f, 0xb0, 0x77, 0x22, 0x5b
);

// 191a8a46-0bf8-46cf-b045-4b45dfa6a324
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_V4,
   0x191a8a46,
   0x0bf8,
   0x46cf,
   0xb0, 0x45, 0x4b, 0x45, 0xdf, 0xa6, 0xa3, 0x24
);

// 80c342e3-1e53-4d6f-9b44-03df5aeee154
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_V6,
   0x80c342e3,
   0x1e53,
   0x4d6f,
   0x9b, 0x44, 0x03, 0xdf, 0x5a, 0xee, 0xe1, 0x54
);

// 70a4196c-835b-4fb0-98e8-075f4d977d46
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_OUTBOUND_TUNNEL_V4,
   0x70a4196c,
   0x835b,
   0x4fb0,
   0x98, 0xe8, 0x07, 0x5f, 0x4d, 0x97, 0x7d, 0x46
);

// f1835363-a6a5-4e62-b180-23db789d8da6
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_OUTBOUND_TUNNEL_V6,
   0xf1835363,
   0xa6a5,
   0x4e62,
   0xb1, 0x80, 0x23, 0xdb, 0x78, 0x9d, 0x8d, 0xa6
);

// 28829633-c4f0-4e66-873f-844db2a899c7
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_FORWARD_INBOUND_TUNNEL_V4,
   0x28829633,
   0xc4f0,
   0x4e66,
   0x87, 0x3f, 0x84, 0x4d, 0xb2, 0xa8, 0x99, 0xc7
);

// af50bec2-c686-429a-884d-b74443e7b0b4
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_FORWARD_INBOUND_TUNNEL_V6,
   0xaf50bec2,
   0xc686,
   0x429a,
   0x88, 0x4d, 0xb7, 0x44, 0x43, 0xe7, 0xb0, 0xb4
);

// fb532136-15cb-440b-937c-1717ca320c40
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_FORWARD_OUTBOUND_TUNNEL_V4,
   0xfb532136,
   0x15cb,
   0x440b,
   0x93, 0x7c, 0x17, 0x17, 0xca, 0x32, 0x0c, 0x40
);

// dae640cc-e021-4bee-9eb6-a48b275c8c1d
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_FORWARD_OUTBOUND_TUNNEL_V6,
   0xdae640cc,
   0xe021,
   0x4bee,
   0x9e, 0xb6, 0xa4, 0x8b, 0x27, 0x5c, 0x8c, 0x1d
);

// 7dff309b-ba7d-4aba-91aa-ae5c6640c944
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_INBOUND_INITIATE_SECURE_V4,
   0x7dff309b,
   0xba7d,
   0x4aba,
   0x91, 0xaa, 0xae, 0x5c, 0x66, 0x40, 0xc9, 0x44
);

// a9a0d6d9-c58c-474e-8aeb-3cfe99d6d53d
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_INBOUND_INITIATE_SECURE_V6,
   0xa9a0d6d9,
   0xc58c,
   0x474e,
   0x8a, 0xeb, 0x3c, 0xfe, 0x99, 0xd6, 0xd5, 0x3d
);

// 3df6e7de-fd20-48f2-9f26-f854444cba79
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_ALE_ACCEPT_V4,
   0x3df6e7de,
   0xfd20,
   0x48f2,
   0x9f, 0x26, 0xf8, 0x54, 0x44, 0x4c, 0xba, 0x79
);

// a1e392d3-72ac-47bb-87a7-0122c69434ab
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_ALE_ACCEPT_V6,
   0xa1e392d3,
   0x72ac,
   0x47bb,
   0x87, 0xa7, 0x01, 0x22, 0xc6, 0x94, 0x34, 0xab
);

// 6ac141fc-f75d-4203-b9c8-48e6149c2712
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_ALE_CONNECT_V4,
   0x6ac141fc,
   0xf75d,
   0x4203,
   0xb9,0xc8,0x48, 0xe6, 0x14, 0x9c, 0x27, 0x12
);

// 4c0dda05-e31f-4666-90b0-b3dfad34129a
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_ALE_CONNECT_V6,
   0x4c0dda05,
   0xe31f,
   0x4666,
   0x90, 0xb0, 0xb3, 0xdf, 0xad, 0x34, 0x12, 0x9a
);

#if (NTDDI_VERSION >= NTDDI_WIN7)

// 6d08a342-db9e-4fbe-9ed2-57374ce89f79
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_DOSP_FORWARD_V6,
   0x6d08a342,
   0xdb9e,
   0x4fbe,
   0x9e, 0xd2, 0x57, 0x37, 0x4c, 0xe8, 0x9f, 0x79
);

// 2fcb56ec-cd37-4b4f-b108-62c2b1850a0c
DEFINE_GUID(
   FWPM_CALLOUT_IPSEC_DOSP_FORWARD_V4,
   0x2fcb56ec,
   0xcd37,
   0x4b4f,
   0xb1, 0x08, 0x62, 0xc2, 0xb1, 0x85, 0x0a, 0x0c
);

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

// eda08606-2494-4d78-89bc-67837c03b969
DEFINE_GUID(
   FWPM_CALLOUT_WFP_TRANSPORT_LAYER_V4_SILENT_DROP,
   0xeda08606,
   0x2494,
   0x4d78,
   0x89, 0xbc, 0x67, 0x83, 0x7c, 0x03, 0xb9, 0x69
);

// 8693cc74-a075-4156-b476-9286eece814e
DEFINE_GUID(
   FWPM_CALLOUT_WFP_TRANSPORT_LAYER_V6_SILENT_DROP,
   0x8693cc74,
   0xa075,
   0x4156,
   0xb4, 0x76, 0x92, 0x86, 0xee, 0xce, 0x81, 0x4e
);

// f3e10ab3-2c25-4279-ac36-c30fc181bec4
DEFINE_GUID(
   FWPM_CALLOUT_TCP_CHIMNEY_CONNECT_LAYER_V4,
   0xf3e10ab3,
   0x2c25,
   0x4279,
   0xac, 0x36, 0xc3, 0x0f, 0xc1, 0x81, 0xbe, 0xc4
);

// 39e22085-a341-42fc-a279-aec94e689c56
DEFINE_GUID(
   FWPM_CALLOUT_TCP_CHIMNEY_CONNECT_LAYER_V6,
   0x39e22085,
   0xa341,
   0x42fc,
   0xa2, 0x79, 0xae, 0xc9, 0x4e, 0x68, 0x9c, 0x56
);

// e183ecb2-3a7f-4b54-8ad9-76050ed880ca
DEFINE_GUID(
   FWPM_CALLOUT_TCP_CHIMNEY_ACCEPT_LAYER_V4,
   0xe183ecb2,
   0x3a7f,
   0x4b54,
   0x8a, 0xd9, 0x76, 0x05, 0x0e, 0xd8, 0x80, 0xca
);

// 0378cf41-bf98-4603-81f2-7f12586079f6
DEFINE_GUID(
   FWPM_CALLOUT_TCP_CHIMNEY_ACCEPT_LAYER_V6,
   0x0378cf41,
   0xbf98,
   0x4603,
   0x81, 0xf2, 0x7f, 0x12, 0x58, 0x60, 0x79, 0xf6
);

// bc582280-1677-41e9-94ab-c2fcb15c2eeb
DEFINE_GUID(
   FWPM_CALLOUT_SET_OPTIONS_AUTH_CONNECT_LAYER_V4,
   0xbc582280,
   0x1677,
   0x41e9,
   0x94, 0xab, 0xc2, 0xfc, 0xb1, 0x5c, 0x2e, 0xeb
);

// 98e5373c-b884-490f-b65f-2f6a4a575195
DEFINE_GUID(
   FWPM_CALLOUT_SET_OPTIONS_AUTH_CONNECT_LAYER_V6,
   0x98e5373c,
   0xb884,
   0x490f,
   0xb6, 0x5f, 0x2f, 0x6a, 0x4a, 0x57, 0x51, 0x95
);

#if (NTDDI_VERSION >= NTDDI_WIN8)

// 2d55f008-0c01-4f92-b26e-a08a94569b8d
DEFINE_GUID(
   FWPM_CALLOUT_SET_OPTIONS_AUTH_RECV_ACCEPT_LAYER_V4,
   0x2d55f008,
   0x0c01,
   0x4f92,
   0xb2, 0x6e, 0xa0, 0x8a, 0x94, 0x56, 0x9b, 0x8d
);

// 63018537-f281-4dc4-83d3-8dec18b7ade2
DEFINE_GUID(
   FWPM_CALLOUT_SET_OPTIONS_AUTH_RECV_ACCEPT_LAYER_V6,
   0x63018537,
   0xf281,
   0x4dc4,
   0x83, 0xd3, 0x8d, 0xec, 0x18, 0xb7, 0xad, 0xe2
);

// 288B524D-0566-4e19-B612-8F441A2E5949
DEFINE_GUID(
   FWPM_CALLOUT_RESERVED_AUTH_CONNECT_LAYER_V4,
   0x288b524d,
   0x566,
   0x4e19,
   0xb6, 0x12, 0x8f, 0x44, 0x1a, 0x2e, 0x59, 0x49
);

// 00B84B92-2B5E-4b71-AB0E-AACA43E387E6
DEFINE_GUID(
   FWPM_CALLOUT_RESERVED_AUTH_CONNECT_LAYER_V6,
   0xb84b92,
   0x2b5e,
   0x4b71,
   0xab, 0xe, 0xaa, 0xca, 0x43, 0xe3, 0x87, 0xe6
);

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

// 31b95392-066e-42a2-b7db-92f8acdd56f9
DEFINE_GUID(
   FWPM_CALLOUT_TEREDO_ALE_RESOURCE_ASSIGNMENT_V6,
   0x31b95392,
   0x066e,
   0x42a2,
   0xb7, 0xdb, 0x92, 0xf8, 0xac, 0xdd, 0x56, 0xf9
);

#define FWPM_CALLOUT_EDGE_TRAVERSAL_ALE_RESOURCE_ASSIGNMENT_V6 \
    FWPM_CALLOUT_TEREDO_ALE_RESOURCE_ASSIGNMENT_V6

// 079b1010-f1c5-4fcd-ae05-da41107abd0b
DEFINE_GUID(
    FWPM_CALLOUT_EDGE_TRAVERSAL_ALE_RESOURCE_ASSIGNMENT_V4,
    0x079b1010,
    0xf1c5,
    0x4fcd,
    0xae, 0x05, 0xda, 0x41, 0x10, 0x7a, 0xbd, 0x0b
);

// 81a434e7-f60c-4378-bab8-c625a30f0197
DEFINE_GUID(
   FWPM_CALLOUT_TEREDO_ALE_LISTEN_V6,
   0x81a434e7,
   0xf60c,
   0x4378,
   0xba, 0xb8, 0xc6, 0x25, 0xa3, 0x0f, 0x01, 0x97
);

#define FWPM_CALLOUT_EDGE_TRAVERSAL_ALE_LISTEN_V6 \
    FWPM_CALLOUT_TEREDO_ALE_LISTEN_V6

// 33486ab5-6d5e-4e65-a00b-a7afed0ba9a1
DEFINE_GUID(
    FWPM_CALLOUT_EDGE_TRAVERSAL_ALE_LISTEN_V4,
    0x33486ab5,
    0x6d5e,
    0x4e65,
    0xa0, 0x0b, 0xa7, 0xaf, 0xed, 0x0b, 0xa9, 0xa1
);

#if (NTDDI_VERSION >= NTDDI_WIN8)

// 215a0b39-4b7e-4eda-8ce4-179679df6224
DEFINE_GUID(
   FWPM_CALLOUT_TCP_TEMPLATES_CONNECT_LAYER_V4,
   0x215a0b39,
   0x4b7e,
   0x4eda,
   0x8c, 0xe4, 0x17, 0x96, 0x79, 0xdf, 0x62, 0x24
);

// 838b37a1-5c12-4d34-8b38-078728b2d25c
DEFINE_GUID(
   FWPM_CALLOUT_TCP_TEMPLATES_CONNECT_LAYER_V6,
   0x838b37a1,
   0x5c12,
   0x4d34,
   0x8b, 0x38, 0x07, 0x87, 0x28, 0xb2, 0xd2, 0x5c
);

// 2f23f5d0-40c4-4c41-a254-46d8dba8957c
DEFINE_GUID(
   FWPM_CALLOUT_TCP_TEMPLATES_ACCEPT_LAYER_V4,
   0x2f23f5d0,
   0x40c4,
   0x4c41,
   0xa2, 0x54, 0x46, 0xd8, 0xdb, 0xa8, 0x95, 0x7c
);

// b25152f0-991c-4f53-bbe7-d24b45fe632c
DEFINE_GUID(
   FWPM_CALLOUT_TCP_TEMPLATES_ACCEPT_LAYER_V6,
   0xb25152f0,
   0x991c,
   0x4f53,
   0xbb, 0xe7, 0xd2, 0x4b, 0x45, 0xfe, 0x63, 0x2c
);

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

// 5FBFC31D-A51C-44DC-ACB6-0624A030A700
DEFINE_GUID(
    FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_CONNECT_LAYER_V4,
    0x5fbfc31d,
    0xa51c,
    0x44dc,
    0xac, 0xb6, 0x6, 0x24, 0xa0, 0x30, 0xa7, 0x00
);

// 5FBFC31D-A51C-44DC-ACB6-0624A030A701
DEFINE_GUID(
    FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_CONNECT_LAYER_V6,
    0x5fbfc31d,
    0xa51c,
    0x44dc,
    0xac, 0xb6, 0x6, 0x24, 0xa0, 0x30, 0xa7, 0x01
);

// 5FBFC31D-A51C-44DC-ACB6-0624A030A702
DEFINE_GUID(
    FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_RECV_ACCEPT_LAYER_V4,
    0x5fbfc31d,
    0xa51c,
    0x44dc,
    0xac, 0xb6, 0x6, 0x24, 0xa0, 0x30, 0xa7, 0x02
);

// 5FBFC31D-A51C-44DC-ACB6-0624A030A703
DEFINE_GUID(
    FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_RECV_ACCEPT_LAYER_V6,
    0x5fbfc31d,
    0xa51c,
    0x44dc,
    0xac, 0xb6, 0x6, 0x24, 0xa0, 0x30, 0xa7, 0x03
);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

// b3423249-8d09-4858-9210-95c7fda8e30f
DEFINE_GUID(
    FWPM_CALLOUT_HTTP_TEMPLATE_SSL_HANDSHAKE,
    0xb3423249,
    0x8d09,
    0x4858,
    0x92, 0x10, 0x95, 0xc7, 0xfd, 0xa8, 0xe3, 0x0f
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

#if (NTDDI_VERSION >= NTDDI_WIN10_NI)

//103090d4-8e28-4fd6-9894-d1d67d6b10c9
DEFINE_GUID(
    FWPM_CALLOUT_OUTBOUND_NETWORK_CONNECTION_POLICY_LAYER_V4,
    0x103090d4,
    0x8e28,
    0x4fd6,
    0x98, 0x94, 0xd1, 0xd6, 0x7d, 0x6b, 0x10, 0xc9
    );

//4ed3446d-8dc7-459b-b09f-c1cb7a8f8689
DEFINE_GUID(
    FWPM_CALLOUT_OUTBOUND_NETWORK_CONNECTION_POLICY_LAYER_V6,
    0x4ed3446d,
    0x8dc7,
    0x459b,
    0xb0, 0x9f, 0xc1, 0xcb, 0x7a, 0x8f, 0x86, 0x89
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_NI)

///////////////////////////////////////////////////////////////////////////////
//
// GUIDs for private built-in callouts.
//
///////////////////////////////////////////////////////////////////////////////
#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
// {779719a4-e695-47b6-a199-7999fec9163b}
DEFINE_GUID(FWPM_CALLOUT_BUILT_IN_RESERVED_1,
   0x779719a4,
   0xe695,
   0x47b6,
   0xa1, 0x99, 0x79, 0x99, 0xfe, 0xc9, 0x16, 0x3b
);

// {ef9661b6-7c5e-48fd-a130-96678ceacc41}
DEFINE_GUID(FWPM_CALLOUT_BUILT_IN_RESERVED_2,
   0xef9661b6,
   0x7c5e,
   0x48fd,
   0xa1, 0x30, 0x96, 0x67, 0x8c, 0xea, 0xcc, 0x41
);

// {18729c7a-2f62-4be0-966f-974b21b86df1}
DEFINE_GUID(FWPM_CALLOUT_BUILT_IN_RESERVED_3,
   0x18729c7a,
   0x2f62,
   0x4be0,
   0x96, 0x6f, 0x97, 0x4b, 0x21, 0xb8, 0x6d, 0xf1
);

// {6c3fb801-daff-40e9-91e6-f7ff7e52f7d9}
DEFINE_GUID(FWPM_CALLOUT_BUILT_IN_RESERVED_4,
   0x6c3fb801,
   0xdaff,
   0x40e9,
   0x91, 0xe6, 0xf7, 0xff, 0x7e, 0x52, 0xf7, 0xd9
);
#endif // (NTDDI_VERSION >= NTDDI_WIN10_FE)

///////////////////////////////////////////////////////////////////////////////
//
// GUIDs for built-in provider contexts.
//
///////////////////////////////////////////////////////////////////////////////

// b25ea800-0d02-46ed-92bd-7fa84bb73e9d
DEFINE_GUID(
   FWPM_PROVIDER_CONTEXT_SECURE_SOCKET_AUTHIP,
   0xb25ea800,
   0x0d02,
   0x46ed,
   0x92, 0xbd, 0x7f, 0xa8, 0x4b, 0xb7, 0x3e, 0x9d
);

// 8c2d4144-f8e0-42c0-94ce-7ccfc63b2f9b
DEFINE_GUID(
   FWPM_PROVIDER_CONTEXT_SECURE_SOCKET_IPSEC,
   0x8c2d4144,
   0xf8e0,
   0x42c0,
   0x94, 0xce, 0x7c, 0xcf, 0xc6, 0x3b, 0x2f, 0x9b
);


///////////////////////////////////////////////////////////////////////////////
//
// GUIDs for built-in keying modules.
//
///////////////////////////////////////////////////////////////////////////////

// a9bbf787-82a8-45bb-a400-5d7e5952c7a9
DEFINE_GUID(
   FWPM_KEYING_MODULE_IKE,
   0xa9bbf787,
   0x82a8,
   0x45bb,
   0xa4, 0x00, 0x5d, 0x7e, 0x59, 0x52, 0xc7, 0xa9
);

// 11e3dae0-dd26-4590-857d-ab4b28d1a095
DEFINE_GUID(
   FWPM_KEYING_MODULE_AUTHIP,
   0x11e3dae0,
   0xdd26,
   0x4590,
   0x85, 0x7d, 0xab, 0x4b, 0x28, 0xd1, 0xa0, 0x95
);

// 041792cc-8f07-419d-a394-716968cb1647
DEFINE_GUID(
   FWPM_KEYING_MODULE_IKEV2,
   0x041792cc,
   0x8f07,
   0x419d,
   0xa3, 0x94, 0x71, 0x69, 0x68, 0xcb, 0x16, 0x47
);


#ifndef GUID_DEFS_ONLY
#ifndef FWPMX_H
#define FWPMX_H

#include "fwpmtypes.h"
#include "fwpvi.h"

#ifdef __cplusplus
extern "C" {
#endif

///////////////////////////////////////////////////////////////////////////////
//
// Well-known filter weight ranges.
//
///////////////////////////////////////////////////////////////////////////////

// Number of bits used for auto-generated weights.
#define FWPM_AUTO_WEIGHT_BITS (60)
// Maximum auto-generated weight.
#define FWPM_AUTO_WEIGHT_MAX  (MAXUINT64 >> (64 - FWPM_AUTO_WEIGHT_BITS))
// Maximum allowed weight range.
#define FWPM_WEIGHT_RANGE_MAX (MAXUINT64 >> FWPM_AUTO_WEIGHT_BITS)

// IPsec policy
#define FWPM_WEIGHT_RANGE_IPSEC            (0x0)
// Filters to exempt IKE traffic from IPsec.
#define FWPM_WEIGHT_RANGE_IKE_EXEMPTIONS   (0xc)


///////////////////////////////////////////////////////////////////////////////
//
// IPsec transform constants.
//
///////////////////////////////////////////////////////////////////////////////

//////////
// Authentication transform constants
//////////

static const IPSEC_AUTH_TRANSFORM_ID0 IPSEC_AUTH_TRANSFORM_ID_HMAC_MD5_96 =
{
   IPSEC_AUTH_MD5,
   IPSEC_AUTH_CONFIG_HMAC_MD5_96
};

static const IPSEC_AUTH_TRANSFORM_ID0 IPSEC_AUTH_TRANSFORM_ID_HMAC_SHA_1_96 =
{
   IPSEC_AUTH_SHA_1,
   IPSEC_AUTH_CONFIG_HMAC_SHA_1_96
};

#if (NTDDI_VERSION >= NTDDI_WIN6SP1)

static const IPSEC_AUTH_TRANSFORM_ID0 IPSEC_AUTH_TRANSFORM_ID_HMAC_SHA_256_128 =
{
   IPSEC_AUTH_SHA_256,
   IPSEC_AUTH_CONFIG_HMAC_SHA_256_128
};

static const IPSEC_AUTH_TRANSFORM_ID0 IPSEC_AUTH_TRANSFORM_ID_GCM_AES_128 =
{
   IPSEC_AUTH_AES_128,
   IPSEC_AUTH_CONFIG_GCM_AES_128
};

static const IPSEC_AUTH_TRANSFORM_ID0 IPSEC_AUTH_TRANSFORM_ID_GCM_AES_192 =
{
   IPSEC_AUTH_AES_192,
   IPSEC_AUTH_CONFIG_GCM_AES_192
};

static const IPSEC_AUTH_TRANSFORM_ID0 IPSEC_AUTH_TRANSFORM_ID_GCM_AES_256 =
{
   IPSEC_AUTH_AES_256,
   IPSEC_AUTH_CONFIG_GCM_AES_256
};

#endif // (NTDDI_VERSION >= NTDDI_WIN6SP1)

//////////
// Cipher transform constants
//////////

static const IPSEC_CIPHER_TRANSFORM_ID0 IPSEC_CIPHER_TRANSFORM_ID_CBC_DES =
{
   IPSEC_CIPHER_TYPE_DES,
   IPSEC_CIPHER_CONFIG_CBC_DES
};

static const IPSEC_CIPHER_TRANSFORM_ID0 IPSEC_CIPHER_TRANSFORM_ID_CBC_3DES =
{
   IPSEC_CIPHER_TYPE_3DES,
   IPSEC_CIPHER_CONFIG_CBC_3DES
};

static const IPSEC_CIPHER_TRANSFORM_ID0 IPSEC_CIPHER_TRANSFORM_ID_AES_128 =
{
   IPSEC_CIPHER_TYPE_AES_128,
   IPSEC_CIPHER_CONFIG_CBC_AES_128
};

static const IPSEC_CIPHER_TRANSFORM_ID0 IPSEC_CIPHER_TRANSFORM_ID_AES_192 =
{
   IPSEC_CIPHER_TYPE_AES_192,
   IPSEC_CIPHER_CONFIG_CBC_AES_192
};

static const IPSEC_CIPHER_TRANSFORM_ID0 IPSEC_CIPHER_TRANSFORM_ID_AES_256 =
{
   IPSEC_CIPHER_TYPE_AES_256,
   IPSEC_CIPHER_CONFIG_CBC_AES_256
};

#if (NTDDI_VERSION >= NTDDI_WIN6SP1)

static const IPSEC_CIPHER_TRANSFORM_ID0 IPSEC_CIPHER_TRANSFORM_ID_GCM_AES_128 =
{
   IPSEC_CIPHER_TYPE_AES_128,
   IPSEC_CIPHER_CONFIG_GCM_AES_128
};

static const IPSEC_CIPHER_TRANSFORM_ID0 IPSEC_CIPHER_TRANSFORM_ID_GCM_AES_192 =
{
   IPSEC_CIPHER_TYPE_AES_192,
   IPSEC_CIPHER_CONFIG_GCM_AES_192
};

static const IPSEC_CIPHER_TRANSFORM_ID0 IPSEC_CIPHER_TRANSFORM_ID_GCM_AES_256 =
{
   IPSEC_CIPHER_TYPE_AES_256,
   IPSEC_CIPHER_CONFIG_GCM_AES_256
};

#endif // (NTDDI_VERSION >= NTDDI_WIN6SP1)


///////////////////////////////////////////////////////////////////////////////
//
// Well-known filter contexts.
//
///////////////////////////////////////////////////////////////////////////////

// IPsec transport filter contexts in inbound layer
#define FWPM_CONTEXT_IPSEC_INBOUND_PASSTHRU (0x1ui64)
#define FWPM_CONTEXT_IPSEC_INBOUND_PERSIST_CONNECTION_SECURITY (0x2ui64)
#define FWPM_CONTEXT_IPSEC_INBOUND_RESERVED (0xff00000000000000ui64)
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define FWPM_CONTEXT_IPSEC_INBOUND_SECURITY_REALM_ID (0x4ui64)
#endif

// IPsec transport filter contexts in outbound layer
#define FWPM_CONTEXT_IPSEC_OUTBOUND_NEGOTIATE_DISCOVER (0x1ui64)
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define FWPM_CONTEXT_IPSEC_OUTBOUND_SUPPRESS_NEGOTIATION (0x2ui64)
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define FWPM_CONTEXT_IPSEC_OUTBOUNDBOUND_SECURITY_REALM_ID (0x4ui64)
#endif
#endif

// Filter contexts used in the ALE connect layer
#define FWPM_CONTEXT_ALE_SET_CONNECTION_REQUIRE_IPSEC_SECURITY (0x2ui64)
#define FWPM_CONTEXT_ALE_SET_CONNECTION_LAZY_SD_EVALUATION (0x4ui64)

// Filter contexts used in the ALE connect or accept layer
#define FWPM_CONTEXT_ALE_SET_CONNECTION_REQUIRE_IPSEC_ENCRYPTION (0x8ui64)
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define FWPM_CONTEXT_ALE_SET_CONNECTION_ALLOW_FIRST_INBOUND_PKT_UNENCRYPTED (0x10ui64)
// FWPM_CONTEXT_ALE_ALLOW_AUTH_FW modifies configurations that require IPsec
// security. Hence, at connect, this is only valid in combination with
// FWPM_CONTEXT_ALE_SET_CONNECTION_REQUIRE_IPSEC_SECURITY.
#define FWPM_CONTEXT_ALE_ALLOW_AUTH_FW (0x20ui64)
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

// Contexts used by the TCP Chimney Offload callouts.
#define FWPM_CONTEXT_TCP_CHIMNEY_OFFLOAD_ENABLE (0x1ui64)
#define FWPM_CONTEXT_TCP_CHIMNEY_OFFLOAD_DISABLE (0x2ui64)

// Contexts used in the RPC audit sublayer
#define FWPM_CONTEXT_RPC_AUDIT_ENABLED (0x1ui64)
#define FWPM_CONTEXT_RPC_AUDIT_BUFFER_ENABLED (0x2ui64)

///////////////////////////////////////////////////////////////////////////////
//
// Access rights
//
///////////////////////////////////////////////////////////////////////////////

// Specific access rights.
#define FWPM_ACTRL_ADD               (0x00000001)
#define FWPM_ACTRL_ADD_LINK          (0x00000002)
#define FWPM_ACTRL_BEGIN_READ_TXN    (0x00000004)
#define FWPM_ACTRL_BEGIN_WRITE_TXN   (0x00000008)
#define FWPM_ACTRL_CLASSIFY          (0x00000010)
#define FWPM_ACTRL_ENUM              (0x00000020)
#define FWPM_ACTRL_OPEN              (0x00000040)
#define FWPM_ACTRL_READ              (0x00000080)
#define FWPM_ACTRL_READ_STATS        (0x00000100)
#define FWPM_ACTRL_SUBSCRIBE         (0x00000200)
#define FWPM_ACTRL_WRITE             (0x00000400)

// Generic access rights.
#define FWPM_GENERIC_READ \
      ( STANDARD_RIGHTS_READ       | \
        FWPM_ACTRL_BEGIN_READ_TXN  | \
        FWPM_ACTRL_CLASSIFY        | \
        FWPM_ACTRL_OPEN            | \
        FWPM_ACTRL_READ            | \
        FWPM_ACTRL_READ_STATS      )

#define FWPM_GENERIC_EXECUTE \
      ( STANDARD_RIGHTS_EXECUTE    | \
        FWPM_ACTRL_ENUM            | \
        FWPM_ACTRL_SUBSCRIBE       )

#define FWPM_GENERIC_WRITE \
      ( STANDARD_RIGHTS_WRITE      | \
        DELETE                     | \
        FWPM_ACTRL_ADD             | \
        FWPM_ACTRL_ADD_LINK        | \
        FWPM_ACTRL_BEGIN_WRITE_TXN | \
        FWPM_ACTRL_WRITE           )

#define FWPM_GENERIC_ALL \
      ( STANDARD_RIGHTS_REQUIRED   | \
        FWPM_ACTRL_ADD             | \
        FWPM_ACTRL_ADD_LINK        | \
        FWPM_ACTRL_BEGIN_READ_TXN  | \
        FWPM_ACTRL_BEGIN_WRITE_TXN | \
        FWPM_ACTRL_CLASSIFY        | \
        FWPM_ACTRL_ENUM            | \
        FWPM_ACTRL_OPEN            | \
        FWPM_ACTRL_READ            | \
        FWPM_ACTRL_READ_STATS      | \
        FWPM_ACTRL_SUBSCRIBE       | \
        FWPM_ACTRL_WRITE           )


///////////////////////////////////////////////////////////////////////////////
//
// Common utility functions.
//
///////////////////////////////////////////////////////////////////////////////

void WINAPI FwpmFreeMemory0(_Inout_ void** p);


///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing the engine.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
FwpmEngineOpen0(
   _In_opt_ const wchar_t* serverName,
   _In_ UINT32 authnService,
   _In_opt_ SEC_WINNT_AUTH_IDENTITY_W* authIdentity,
   _In_opt_ const FWPM_SESSION0* session,
   _Out_ HANDLE* engineHandle
   );

DWORD
WINAPI
FwpmEngineClose0(_Inout_ HANDLE engineHandle);

DWORD
WINAPI
FwpmEngineGetOption0(
   _In_ HANDLE engineHandle,
   _In_ FWPM_ENGINE_OPTION option,
   _Outptr_ FWP_VALUE0** value
   );

DWORD
WINAPI
FwpmEngineSetOption0(
   _In_ HANDLE engineHandle,
   _In_ FWPM_ENGINE_OPTION option,
   _In_ const FWP_VALUE0* newValue
   );

DWORD
WINAPI
FwpmEngineGetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
FwpmEngineSetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );

DWORD
WINAPI
FwpmSessionCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const FWPM_SESSION_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
FwpmSessionEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_SESSION0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

DWORD
WINAPI
FwpmSessionDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );


///////////////////////////////////////////////////////////////////////////////
//
// Functions for explicit transactions.
//
///////////////////////////////////////////////////////////////////////////////

#define FWPM_TXN_READ_ONLY (0x00000001)

DWORD
WINAPI
FwpmTransactionBegin0(
   _In_ HANDLE engineHandle,
   _In_ UINT32 flags
   );

DWORD
WINAPI
FwpmTransactionCommit0(_In_ HANDLE engineHandle);

DWORD
WINAPI
FwpmTransactionAbort0(_In_ HANDLE engineHandle);


///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing providers.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
FwpmProviderAdd0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_PROVIDER0* provider,
   _In_opt_ PSECURITY_DESCRIPTOR sd
   );

DWORD
WINAPI
FwpmProviderDeleteByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key
   );

DWORD
WINAPI
FwpmProviderGetByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key,
   _Outptr_ FWPM_PROVIDER0** provider
   );

DWORD
WINAPI
FwpmProviderCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const FWPM_PROVIDER_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
FwpmProviderEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_PROVIDER0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

DWORD
WINAPI
FwpmProviderDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );

DWORD
WINAPI
FwpmProviderGetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
FwpmProviderSetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );

typedef void (CALLBACK *FWPM_PROVIDER_CHANGE_CALLBACK0)(
                           _Inout_ void* context,
                           _In_ const FWPM_PROVIDER_CHANGE0* change
                           );

DWORD
WINAPI
FwpmProviderSubscribeChanges0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_PROVIDER_SUBSCRIPTION0* subscription,
   _In_ FWPM_PROVIDER_CHANGE_CALLBACK0 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* changeHandle
   );

DWORD
WINAPI
FwpmProviderUnsubscribeChanges0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE changeHandle
   );

DWORD
WINAPI
FwpmProviderSubscriptionsGet0(
   _In_ HANDLE engineHandle,
   _Outptr_result_buffer_(*numEntries)
       FWPM_PROVIDER_SUBSCRIPTION0*** entries,
   _Out_ UINT32* numEntries
   );


///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing provider contexts.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
FwpmProviderContextAdd0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_PROVIDER_CONTEXT0* providerContext,
   _In_opt_ PSECURITY_DESCRIPTOR sd,
   _Out_opt_ UINT64* id
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
FwpmProviderContextAdd1(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_PROVIDER_CONTEXT1* providerContext,
   _In_opt_ PSECURITY_DESCRIPTOR sd,
   _Out_opt_ UINT64* id
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
FwpmProviderContextAdd2(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_PROVIDER_CONTEXT2* providerContext,
   _In_opt_ PSECURITY_DESCRIPTOR sd,
   _Out_opt_ UINT64* id
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
DWORD
WINAPI
FwpmProviderContextAdd3(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_PROVIDER_CONTEXT3* providerContext,
   _In_opt_ PSECURITY_DESCRIPTOR sd,
   _Out_opt_ UINT64* id
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)


DWORD
WINAPI
FwpmProviderContextDeleteById0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id
   );

DWORD
WINAPI
FwpmProviderContextDeleteByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key
   );

DWORD
WINAPI
FwpmProviderContextGetById0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _Outptr_ FWPM_PROVIDER_CONTEXT0** providerContext
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
FwpmProviderContextGetById1(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _Outptr_ FWPM_PROVIDER_CONTEXT1** providerContext
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
FwpmProviderContextGetById2(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _Outptr_ FWPM_PROVIDER_CONTEXT2** providerContext
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
DWORD
WINAPI
FwpmProviderContextGetById3(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _Outptr_ FWPM_PROVIDER_CONTEXT3** providerContext
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)


DWORD
WINAPI
FwpmProviderContextGetByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key,
   _Outptr_ FWPM_PROVIDER_CONTEXT0** providerContext
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
FwpmProviderContextGetByKey1(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key,
   _Outptr_ FWPM_PROVIDER_CONTEXT1** providerContext
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
FwpmProviderContextGetByKey2(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key,
   _Outptr_ FWPM_PROVIDER_CONTEXT2** providerContext
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
DWORD
WINAPI
FwpmProviderContextGetByKey3(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key,
   _Outptr_ FWPM_PROVIDER_CONTEXT3** providerContext
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

DWORD
WINAPI
FwpmProviderContextCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
FwpmProviderContextEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned)
      FWPM_PROVIDER_CONTEXT0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
FwpmProviderContextEnum1(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned)
      FWPM_PROVIDER_CONTEXT1*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
FwpmProviderContextEnum2(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned)
      FWPM_PROVIDER_CONTEXT2*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
DWORD
WINAPI
FwpmProviderContextEnum3(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned)
      FWPM_PROVIDER_CONTEXT3*** entries,
   _Out_ UINT32* numEntriesReturned
   );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

DWORD
WINAPI
FwpmProviderContextDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );

DWORD
WINAPI
FwpmProviderContextGetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
FwpmProviderContextSetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );

typedef void (CALLBACK *FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0)(
                           _Inout_ void* context,
                           _In_ const FWPM_PROVIDER_CONTEXT_CHANGE0* change
                           );

DWORD
WINAPI
FwpmProviderContextSubscribeChanges0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0* subscription,
   _In_ FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* changeHandle
   );

DWORD
WINAPI
FwpmProviderContextUnsubscribeChanges0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE changeHandle
   );

DWORD
WINAPI
FwpmProviderContextSubscriptionsGet0(
   _In_ HANDLE engineHandle,
   _Outptr_result_buffer_(*numEntries)
       FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0*** entries,
   _Out_ UINT32* numEntries
   );


///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing sub-layers.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
FwpmSubLayerAdd0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_SUBLAYER0* subLayer,
   _In_opt_ PSECURITY_DESCRIPTOR sd
   );

DWORD
WINAPI
FwpmSubLayerDeleteByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key
   );

DWORD
WINAPI
FwpmSubLayerGetByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key,
   _Outptr_ FWPM_SUBLAYER0** subLayer
   );

DWORD
WINAPI
FwpmSubLayerCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const FWPM_SUBLAYER_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
FwpmSubLayerEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_SUBLAYER0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

DWORD
WINAPI
FwpmSubLayerDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );

DWORD
WINAPI
FwpmSubLayerGetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
FwpmSubLayerSetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );

typedef void (CALLBACK *FWPM_SUBLAYER_CHANGE_CALLBACK0)(
                           _Inout_ void* context,
                           _In_ const FWPM_SUBLAYER_CHANGE0* change
                           );

DWORD
WINAPI
FwpmSubLayerSubscribeChanges0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_SUBLAYER_SUBSCRIPTION0* subscription,
   _In_ FWPM_SUBLAYER_CHANGE_CALLBACK0 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* changeHandle
   );

DWORD
WINAPI
FwpmSubLayerUnsubscribeChanges0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE changeHandle
   );

DWORD
WINAPI
FwpmSubLayerSubscriptionsGet0(
   _In_ HANDLE engineHandle,
   _Outptr_result_buffer_(*numEntries)
       FWPM_SUBLAYER_SUBSCRIPTION0*** entries,
   _Out_ UINT32* numEntries
   );


///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing layers.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
FwpmLayerGetById0(
   _In_ HANDLE engineHandle,
   _In_ UINT16 id,
   _Outptr_ FWPM_LAYER0** layer
   );

DWORD
WINAPI
FwpmLayerGetByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key,
   _Outptr_ FWPM_LAYER0** layer
   );

DWORD
WINAPI
FwpmLayerCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const FWPM_LAYER_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
FwpmLayerEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_LAYER0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

DWORD
WINAPI
FwpmLayerDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );

DWORD
WINAPI
FwpmLayerGetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
FwpmLayerSetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );


///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing callouts.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
FwpmCalloutAdd0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_CALLOUT0* callout,
   _In_opt_ PSECURITY_DESCRIPTOR sd,
   _Out_opt_ UINT32* id
   );

DWORD
WINAPI
FwpmCalloutDeleteById0(
   _In_ HANDLE engineHandle,
   _In_ UINT32 id
   );

DWORD
WINAPI
FwpmCalloutDeleteByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key
   );

DWORD
WINAPI
FwpmCalloutGetById0(
   _In_ HANDLE engineHandle,
   _In_ UINT32 id,
   _Outptr_ FWPM_CALLOUT0** callout
   );

DWORD
WINAPI
FwpmCalloutGetByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key,
   _Outptr_ FWPM_CALLOUT0** callout
   );

DWORD
WINAPI
FwpmCalloutCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const FWPM_CALLOUT_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
FwpmCalloutEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_CALLOUT0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

DWORD
WINAPI
FwpmCalloutDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );

DWORD
WINAPI
FwpmCalloutGetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
FwpmCalloutSetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );

typedef void (CALLBACK *FWPM_CALLOUT_CHANGE_CALLBACK0)(
                           _Inout_ void* context,
                           _In_ const FWPM_CALLOUT_CHANGE0* change
                           );

DWORD
WINAPI
FwpmCalloutSubscribeChanges0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_CALLOUT_SUBSCRIPTION0* subscription,
   _In_ FWPM_CALLOUT_CHANGE_CALLBACK0 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* changeHandle
   );

DWORD
WINAPI
FwpmCalloutUnsubscribeChanges0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE changeHandle
   );

DWORD
WINAPI
FwpmCalloutSubscriptionsGet0(
   _In_ HANDLE engineHandle,
   _Outptr_result_buffer_(*numEntries)
       FWPM_CALLOUT_SUBSCRIPTION0*** entries,
   _Out_ UINT32* numEntries
   );


///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing filters.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
FwpmFilterAdd0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_FILTER0* filter,
   _In_opt_ PSECURITY_DESCRIPTOR sd,
   _Out_opt_ UINT64* id
   );

DWORD
WINAPI
FwpmFilterDeleteById0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id
   );

DWORD
WINAPI
FwpmFilterDeleteByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key
   );

DWORD
WINAPI
FwpmFilterGetById0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _Outptr_ FWPM_FILTER0** filter
   );

DWORD
WINAPI
FwpmFilterGetByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key,
   _Outptr_ FWPM_FILTER0** filter
   );

DWORD
WINAPI
FwpmFilterCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const FWPM_FILTER_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
FwpmFilterEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_FILTER0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

DWORD
WINAPI
FwpmFilterDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );

DWORD
WINAPI
FwpmFilterGetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
FwpmFilterSetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _In_opt_ const GUID* key,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );

typedef void (CALLBACK *FWPM_FILTER_CHANGE_CALLBACK0)(
                           _Inout_ void* context,
                           _In_ const FWPM_FILTER_CHANGE0* change
                           );

DWORD
WINAPI
FwpmFilterSubscribeChanges0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_FILTER_SUBSCRIPTION0* subscription,
   _In_ FWPM_FILTER_CHANGE_CALLBACK0 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* changeHandle
   );

DWORD
WINAPI
FwpmFilterUnsubscribeChanges0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE changeHandle
   );

DWORD
WINAPI
FwpmFilterSubscriptionsGet0(
   _In_ HANDLE engineHandle,
   _Outptr_result_buffer_(*numEntries)
       FWPM_FILTER_SUBSCRIPTION0*** entries,
   _Out_ UINT32* numEntries
   );

DWORD
WINAPI
FwpmGetAppIdFromFileName0(
   _In_ PCWSTR fileName,
   _Outptr_ FWP_BYTE_BLOB** appId
   );

///////////////////////////////////////////////////////////////////////////////
//
// Helper functions for configuring an IPsec tunnel.
//
///////////////////////////////////////////////////////////////////////////////

// Create a point-to-point tunnel.
#define FWPM_TUNNEL_FLAG_POINT_TO_POINT (0x00000001)
#if (NTDDI_VERSION >= NTDDI_WIN7)
// Enable Virtual interface based IPsec tunnel mode.
#define FWPM_TUNNEL_FLAG_ENABLE_VIRTUAL_IF_TUNNELING (0x00000002)
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
// Reserved flags.
#define FWPM_TUNNEL_FLAG_RESERVED0 (0x00000004)
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

DWORD
WINAPI
FwpmIPsecTunnelAdd0(
   _In_ HANDLE engineHandle,
   _In_ UINT32 flags,
   _In_opt_ const FWPM_PROVIDER_CONTEXT0* mainModePolicy,
   _In_ const FWPM_PROVIDER_CONTEXT0* tunnelPolicy,
   _In_ UINT32 numFilterConditions,
   _In_reads_(numFilterConditions)
      const FWPM_FILTER_CONDITION0* filterConditions,
   _In_opt_ PSECURITY_DESCRIPTOR sd
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
FwpmIPsecTunnelAdd1(
   _In_ HANDLE engineHandle,
   _In_ UINT32 flags,
   _In_opt_ const FWPM_PROVIDER_CONTEXT1* mainModePolicy,
   _In_ const FWPM_PROVIDER_CONTEXT1* tunnelPolicy,
   _In_ UINT32 numFilterConditions,
   _In_reads_(numFilterConditions)
      const FWPM_FILTER_CONDITION0* filterConditions,
   _In_opt_ const GUID* keyModKey,
   _In_opt_ PSECURITY_DESCRIPTOR sd
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
FwpmIPsecTunnelAdd2(
   _In_ HANDLE engineHandle,
   _In_ UINT32 flags,
   _In_opt_ const FWPM_PROVIDER_CONTEXT2* mainModePolicy,
   _In_ const FWPM_PROVIDER_CONTEXT2* tunnelPolicy,
   _In_ UINT32 numFilterConditions,
   _In_reads_(numFilterConditions)
      const FWPM_FILTER_CONDITION0* filterConditions,
   _In_opt_ const GUID* keyModKey,
   _In_opt_ PSECURITY_DESCRIPTOR sd
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
DWORD
WINAPI
FwpmIPsecTunnelAdd3(
   _In_ HANDLE engineHandle,
   _In_ UINT32 flags,
   _In_opt_ const FWPM_PROVIDER_CONTEXT3* mainModePolicy,
   _In_ const FWPM_PROVIDER_CONTEXT3* tunnelPolicy,
   _In_ UINT32 numFilterConditions,
   _In_reads_(numFilterConditions)
      const FWPM_FILTER_CONDITION0* filterConditions,
   _In_opt_ const GUID* keyModKey,
   _In_opt_ PSECURITY_DESCRIPTOR sd
   );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

DWORD
WINAPI
FwpmIPsecTunnelDeleteByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key
   );


///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing IPsec.
//
///////////////////////////////////////////////////////////////////////////////


DWORD
WINAPI
IPsecGetStatistics0(
   _In_ HANDLE engineHandle,
   _Out_ IPSEC_STATISTICS0* ipsecStatistics
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
IPsecGetStatistics1(
   _In_ HANDLE engineHandle,
   _Out_ IPSEC_STATISTICS1* ipsecStatistics
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

DWORD
WINAPI
IPsecSaContextCreate0(
   _In_ HANDLE engineHandle,
   _In_ const IPSEC_TRAFFIC0* outboundTraffic,
   _Out_opt_ UINT64* inboundFilterId,
   _Out_ UINT64* id
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
IPsecSaContextCreate1(
   _In_ HANDLE engineHandle,
   _In_ const IPSEC_TRAFFIC1* outboundTraffic,
   _In_opt_ const IPSEC_VIRTUAL_IF_TUNNEL_INFO0* virtualIfTunnelInfo,
   _Out_opt_ UINT64* inboundFilterId,
   _Out_ UINT64* id
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

DWORD
WINAPI
IPsecSaContextDeleteById0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id
   );

DWORD
WINAPI
IPsecSaContextGetById0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _Outptr_ IPSEC_SA_CONTEXT0** saContext
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
IPsecSaContextGetById1(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _Outptr_ IPSEC_SA_CONTEXT1** saContext
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

DWORD
WINAPI
IPsecSaContextGetSpi0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _In_ const IPSEC_GETSPI0* getSpi,
   _Out_ IPSEC_SA_SPI* inboundSpi
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
IPsecSaContextGetSpi1(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _In_ const IPSEC_GETSPI1* getSpi,
   _Out_ IPSEC_SA_SPI* inboundSpi
   );

DWORD
WINAPI
IPsecSaContextSetSpi0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _In_ const IPSEC_GETSPI1* getSpi,
   _In_ IPSEC_SA_SPI inboundSpi
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

DWORD
WINAPI
IPsecSaContextAddInbound0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _In_ const IPSEC_SA_BUNDLE0* inboundBundle
   );

DWORD
WINAPI
IPsecSaContextAddOutbound0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _In_ const IPSEC_SA_BUNDLE0* outboundBundle
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
IPsecSaContextAddInbound1(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _In_ const IPSEC_SA_BUNDLE1* inboundBundle
   );

DWORD
WINAPI
IPsecSaContextAddOutbound1(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _In_ const IPSEC_SA_BUNDLE1* outboundBundle
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

DWORD
WINAPI
IPsecSaContextExpire0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
// Flags indicating the specific field in the IPSEC_SA_CONTEXT data type that is
// being updated.
//
#define IPSEC_SA_DETAILS_UPDATE_TRAFFIC                (0x01ui64)
#define IPSEC_SA_DETAILS_UPDATE_UDP_ENCAPSULATION      (0x02ui64)
#define IPSEC_SA_BUNDLE_UPDATE_FLAGS                   (0x04ui64)
#define IPSEC_SA_BUNDLE_UPDATE_NAP_CONTEXT             (0x08ui64)
#define IPSEC_SA_BUNDLE_UPDATE_KEY_MODULE_STATE        (0x10ui64)
#define IPSEC_SA_BUNDLE_UPDATE_PEER_V4_PRIVATE_ADDRESS (0x20ui64)
#define IPSEC_SA_BUNDLE_UPDATE_MM_SA_ID                (0x40ui64)

DWORD
WINAPI
IPsecSaContextUpdate0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 flags,
   _In_ const IPSEC_SA_CONTEXT1* newValues
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

DWORD
WINAPI
IPsecSaContextCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const IPSEC_SA_CONTEXT_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
IPsecSaContextEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) IPSEC_SA_CONTEXT0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
IPsecSaContextEnum1(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) IPSEC_SA_CONTEXT1*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

DWORD
WINAPI
IPsecSaContextDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );


#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef void (CALLBACK *IPSEC_SA_CONTEXT_CALLBACK0)(
  _Inout_ void *context,
  _In_ const IPSEC_SA_CONTEXT_CHANGE0 *change
);

DWORD
WINAPI
IPsecSaContextSubscribe0(
  _In_ HANDLE engineHandle,
  _In_ const IPSEC_SA_CONTEXT_SUBSCRIPTION0 *subscription,
  _In_ IPSEC_SA_CONTEXT_CALLBACK0 callback,
  _In_opt_ void *context,
  _Out_ HANDLE* eventsHandle
);

DWORD
WINAPI
IPsecSaContextUnsubscribe0(
  _In_ HANDLE engineHandle,
  _Inout_ HANDLE eventsHandle
);

DWORD
WINAPI
IPsecSaContextSubscriptionsGet0(
   _In_ HANDLE engineHandle,
   _Outptr_result_buffer_(*numEntries)
       IPSEC_SA_CONTEXT_SUBSCRIPTION0*** entries,
   _Out_ UINT32* numEntries
   );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)


DWORD
WINAPI
IPsecSaCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const IPSEC_SA_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
IPsecSaEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) IPSEC_SA_DETAILS0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
IPsecSaEnum1(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) IPSEC_SA_DETAILS1*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

DWORD
WINAPI
IPsecSaDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );

DWORD
WINAPI
IPsecSaDbGetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
IPsecSaDbSetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)

///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing IPsec DoS Protection.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
IPsecDospGetStatistics0(
   _In_ HANDLE engineHandle,
   _Out_ IPSEC_DOSP_STATISTICS0* idpStatistics
   );

DWORD
WINAPI
IPsecDospStateCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const IPSEC_DOSP_STATE_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
IPsecDospStateEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntries) IPSEC_DOSP_STATE0*** entries,
   _Out_ UINT32* numEntries
   );

DWORD
WINAPI
IPsecDospStateDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );

DWORD
WINAPI
IPsecDospGetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
IPsecDospSetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );

#endif // (NTDDI_VERSION >= NTDDI_WIN7)


///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing key managers.
//
///////////////////////////////////////////////////////////////////////////////

#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef void ( CALLBACK * IPSEC_KEY_MANAGER_KEY_DICTATION_CHECK0)(
                             _In_ const IKEEXT_TRAFFIC0* ikeTraffic,
                             _Out_ BOOL* willDictateKey,
                             _Out_ UINT32* weight
                             );

typedef DWORD ( CALLBACK * IPSEC_KEY_MANAGER_DICTATE_KEY0)(
                              _Inout_ IPSEC_SA_DETAILS1* inboundSaDetails,
                              _Inout_ IPSEC_SA_DETAILS1* outboundSaDetails,
                              _Out_ BOOL* keyingModuleGenKey
                              );

typedef void ( CALLBACK * IPSEC_KEY_MANAGER_NOTIFY_KEY0)(
                              _In_ const IPSEC_SA_DETAILS1* inboundSa,
                              _In_ const IPSEC_SA_DETAILS1* outboundSa
                              );

typedef struct _IPSEC_KEY_MANAGER_CALLBACKS0 {
   GUID reserved;
   UINT32 flags;
   IPSEC_KEY_MANAGER_KEY_DICTATION_CHECK0 keyDictationCheck;
   IPSEC_KEY_MANAGER_DICTATE_KEY0 keyDictation;
   IPSEC_KEY_MANAGER_NOTIFY_KEY0 keyNotify;
} IPSEC_KEY_MANAGER_CALLBACKS0;

DWORD
WINAPI
IPsecKeyManagerAddAndRegister0(
   _In_ HANDLE engineHandle,
   _In_ const IPSEC_KEY_MANAGER0* keyManager,
   _In_ const IPSEC_KEY_MANAGER_CALLBACKS0* keyManagerCallbacks,
   _Out_ HANDLE* keyMgmtHandle
);

DWORD
WINAPI
IPsecKeyManagerUnregisterAndDelete0(
  _In_ HANDLE engineHandle,
  _In_ HANDLE keyMgmtHandle
);

DWORD
WINAPI
IPsecKeyManagersGet0(
   _In_ HANDLE engineHandle,
   _Outptr_result_buffer_(*numEntries) IPSEC_KEY_MANAGER0*** entries,
   _Out_ UINT32* numEntries
);

DWORD
WINAPI
IPsecKeyManagerGetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _Reserved_ const void* reserved,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
IPsecKeyManagerSetSecurityInfoByKey0(
   _In_ HANDLE engineHandle,
   _Reserved_ const void* reserved,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

///////////////////////////////////////////////////////////////////////////////
//
// Functions for managing IKE, Authip.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
IkeextGetStatistics0(
   _In_ HANDLE engineHandle,
   _Out_ IKEEXT_STATISTICS0* ikeextStatistics
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
IkeextGetStatistics1(
   _In_ HANDLE engineHandle,
   _Out_ IKEEXT_STATISTICS1* ikeextStatistics
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

DWORD
WINAPI
IkeextSaDeleteById0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id
   );

DWORD
WINAPI
IkeextSaGetById0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _Outptr_ IKEEXT_SA_DETAILS0** sa
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
IkeextSaGetById1(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _In_opt_ GUID* saLookupContext,
   _Outptr_ IKEEXT_SA_DETAILS1** sa
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
IkeextSaGetById2(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _In_opt_ GUID* saLookupContext,
   _Outptr_ IKEEXT_SA_DETAILS2** sa
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

DWORD
WINAPI
IkeextSaCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const IKEEXT_SA_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
IkeextSaEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) IKEEXT_SA_DETAILS0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
IkeextSaEnum1(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) IKEEXT_SA_DETAILS1*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
IkeextSaEnum2(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) IKEEXT_SA_DETAILS2*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)


DWORD
WINAPI
IkeextSaDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );

DWORD
WINAPI
IkeextSaDbGetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
IkeextSaDbSetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );

///////////////////////////////////////////////////////////////////////////////
//
// Functions for diagnostics.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
FwpmNetEventCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const FWPM_NET_EVENT_ENUM_TEMPLATE0* enumTemplate,
   _Out_ HANDLE* enumHandle
   );

DWORD
WINAPI
FwpmNetEventEnum0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_NET_EVENT0*** entries,
   _Out_ UINT32* numEntriesReturned
   );

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
FwpmNetEventEnum1(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_NET_EVENT1*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
FwpmNetEventEnum2(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_NET_EVENT2*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
DWORD
WINAPI
FwpmNetEventEnum3(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_NET_EVENT3*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
DWORD
WINAPI
FwpmNetEventEnum4(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_NET_EVENT4*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS4)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5) //NTDDI_WIN10_RS5
DWORD
WINAPI
FwpmNetEventEnum5(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle,
   _In_ UINT32 numEntriesRequested,
   _Outptr_result_buffer_(*numEntriesReturned) FWPM_NET_EVENT5*** entries,
   _Out_ UINT32* numEntriesReturned
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

DWORD
WINAPI
FwpmNetEventDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE enumHandle
   );

DWORD
WINAPI
FwpmNetEventsGetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
FwpmNetEventsSetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );
#if (NTDDI_VERSION >= NTDDI_WIN7)

typedef void (CALLBACK *FWPM_NET_EVENT_CALLBACK0)(
                           _Inout_ void* context,
                           _In_ const FWPM_NET_EVENT1* event
                           );

DWORD
WINAPI
FwpmNetEventSubscribe0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_NET_EVENT_SUBSCRIPTION0* subscription,
   _In_ FWPM_NET_EVENT_CALLBACK0 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* eventsHandle
   );

DWORD
WINAPI
FwpmNetEventUnsubscribe0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE eventsHandle
   );

DWORD
WINAPI
FwpmNetEventSubscriptionsGet0(
   _In_ HANDLE engineHandle,
   _Outptr_result_buffer_(*numEntries)
       FWPM_NET_EVENT_SUBSCRIPTION0*** entries,
   _Out_ UINT32* numEntries
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef void (CALLBACK *FWPM_NET_EVENT_CALLBACK1)(
                           _Inout_ void* context,
                           _In_ const FWPM_NET_EVENT2* event
                           );

DWORD
WINAPI
FwpmNetEventSubscribe1(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_NET_EVENT_SUBSCRIPTION0* subscription,
   _In_ FWPM_NET_EVENT_CALLBACK1 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* eventsHandle
   );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

typedef void (CALLBACK *FWPM_NET_EVENT_CALLBACK2)(
                           _Inout_ void* context,
                           _In_ const FWPM_NET_EVENT3* event
                           );

DWORD
WINAPI
FwpmNetEventSubscribe2(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_NET_EVENT_SUBSCRIPTION0* subscription,
   _In_ FWPM_NET_EVENT_CALLBACK2 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* eventsHandle
   );

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

typedef void (CALLBACK *FWPM_NET_EVENT_CALLBACK3)(
                           _Inout_ void* context,
                           _In_ const FWPM_NET_EVENT4* event
                           );

DWORD
WINAPI
FwpmNetEventSubscribe3(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_NET_EVENT_SUBSCRIPTION0* subscription,
   _In_ FWPM_NET_EVENT_CALLBACK3 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* eventsHandle
   );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS4)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5) //NTDDI_WIN10_RS5

typedef void (CALLBACK *FWPM_NET_EVENT_CALLBACK4)(
                           _Inout_ void* context,
                           _In_ const FWPM_NET_EVENT5* event
                           );

DWORD
WINAPI
FwpmNetEventSubscribe4(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_NET_EVENT_SUBSCRIPTION0* subscription,
   _In_ FWPM_NET_EVENT_CALLBACK4 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* eventsHandle
   );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)


#if (NTDDI_VERSION >= NTDDI_WIN10_CO)
///////////////////////////////////////////////////////////////////////////////
//
// Functions for Tracking Dynamic Keyword Changes
//
///////////////////////////////////////////////////////////////////////////////
#define FWPM_NOTIFY_ADDRESSES_AUTO_RESOLVE 0x01ui64
#define FWPM_NOTIFY_ADDRESSES_NON_AUTO_RESOLVE 0x02ui64
#define FWPM_NOTIFY_ADDRESSES_ALL (FWPM_NOTIFY_ADDRESSES_AUTO_RESOLVE | FWPM_NOTIFY_ADDRESSES_NON_AUTO_RESOLVE)
#define FWPM_NOTIFY_GRANULAR 0x04ui64

typedef void (CALLBACK *FWPM_DYNAMIC_KEYWORD_CALLBACK0)(
                           _Inout_ void* notification,
                           _Inout_ void* context
                           );

DWORD
WINAPI
FwpmDynamicKeywordSubscribe0(
   _In_ DWORD flags,
   _In_ FWPM_DYNAMIC_KEYWORD_CALLBACK0 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* subscriptionHandle
   );

DWORD
WINAPI
FwpmDynamicKeywordUnsubscribe0(
   _In_ HANDLE subscriptionHandle
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN10_CO)


#if (NTDDI_VERSION >= NTDDI_WIN7)

///////////////////////////////////////////////////////////////////////////////
//
// Functions for tracking system ports.
//
///////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
FwpmSystemPortsGet0(
   _In_opt_ HANDLE engineHandle,
   _Outptr_ FWPM_SYSTEM_PORTS0** sysPorts
   );

typedef void (CALLBACK *FWPM_SYSTEM_PORTS_CALLBACK0)(
                           _Inout_ void* context,
                           _In_ const FWPM_SYSTEM_PORTS0* sysPorts
                           );

DWORD
WINAPI
FwpmSystemPortsSubscribe0(
   _In_opt_ HANDLE engineHandle,
   _Reserved_ void* reserved,
   _In_ FWPM_SYSTEM_PORTS_CALLBACK0 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* sysPortsHandle
   );

DWORD
WINAPI
FwpmSystemPortsUnsubscribe0(
   _In_opt_ HANDLE engineHandle,
   _Inout_ HANDLE sysPortsHandle
   );

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN8)

DWORD
WINAPI
FwpmConnectionGetById0(
   _In_ HANDLE engineHandle,
   _In_ UINT64 id,
   _Outptr_ FWPM_CONNECTION0** connection
   );

DWORD
WINAPI
FwpmConnectionEnum0(
    _In_ HANDLE engineHandle,
    _In_ HANDLE enumHandle,
    _In_ UINT32 numEntriesRequested,
    _Outptr_result_buffer_(*numEntriesReturned) FWPM_CONNECTION0*** entries,
    _Out_ UINT32 *numEntriesReturned
    );

DWORD
WINAPI
FwpmConnectionCreateEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_opt_ const FWPM_CONNECTION_ENUM_TEMPLATE0 *enumTemplate,
   _Out_ HANDLE *enumHandle
   );

DWORD
WINAPI
FwpmConnectionDestroyEnumHandle0(
   _In_ HANDLE engineHandle,
   _In_ HANDLE enumHandle
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)


DWORD
WINAPI
FwpmConnectionGetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
FwpmConnectionSetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );


#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef void (CALLBACK *FWPM_CONNECTION_CALLBACK0)(
   _Inout_ void* context,
   _In_ FWPM_CONNECTION_EVENT_TYPE eventType,
   _In_ const FWPM_CONNECTION0* connection
);

DWORD
WINAPI
FwpmConnectionSubscribe0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_CONNECTION_SUBSCRIPTION0* subscription,
   _In_ FWPM_CONNECTION_CALLBACK0 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* eventsHandle
   );

DWORD
WINAPI
FwpmConnectionUnsubscribe0(
    _In_ HANDLE engineHandle,
    _Inout_ HANDLE eventsHandle
    );

DWORD
WINAPI
FwpmConnectionSubscriptionsGet0(
   _In_ HANDLE engineHandle,
   _Outptr_result_buffer_(*numEntries)
       FWPM_CONNECTION_SUBSCRIPTION0*** entries,
   _Out_ UINT32* numEntries
   );

///////////////////////////////////////////////////////////////////////////////
//
// Functions for vSwitch events.
//
///////////////////////////////////////////////////////////////////////////////

typedef DWORD (CALLBACK *FWPM_VSWITCH_EVENT_CALLBACK0)(
                           _Inout_ void* context,
                           _In_ const FWPM_VSWITCH_EVENT0* vSwitchEvent
                           );

DWORD
WINAPI
FwpmvSwitchEventSubscribe0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_VSWITCH_EVENT_SUBSCRIPTION0* subscription,
   _In_ FWPM_VSWITCH_EVENT_CALLBACK0 callback,
   _In_opt_ void* context,
   _Out_ HANDLE* subscriptionHandle
   );

DWORD
WINAPI
FwpmvSwitchEventUnsubscribe0(
   _In_ HANDLE engineHandle,
   _Inout_ HANDLE subscriptionHandle
   );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
FwpmvSwitchEventsGetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _Outptr_result_maybenull_ PSID* sidOwner,
   _Outptr_result_maybenull_ PSID* sidGroup,
   _Outptr_result_maybenull_ PACL* dacl,
   _Outptr_result_maybenull_ PACL* sacl,
   _Outptr_ PSECURITY_DESCRIPTOR* securityDescriptor
   );

DWORD
WINAPI
FwpmvSwitchEventsSetSecurityInfo0(
   _In_ HANDLE engineHandle,
   _In_ SECURITY_INFORMATION securityInfo,
   _In_opt_ const SID* sidOwner,
   _In_opt_ const SID* sidGroup,
   _In_opt_ const ACL* dacl,
   _In_opt_ const ACL* sacl
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

////////////////////////////////////////////////////////////////
//
// Functions for managing connection policy.
//
////////////////////////////////////////////////////////////////

#if (NTDDI_VERSION >= NTDDI_WIN11_ZN)
DWORD
WINAPI
FwpmConnectionPolicyAdd0(
   _In_ HANDLE engineHandle,
   _In_ const FWPM_PROVIDER_CONTEXT3* connectionPolicy,
   _In_ FWP_IP_VERSION ipVersion,
   _In_ UINT64 weight,
   _In_ UINT32 numFilterConditions,
   _In_reads_(numFilterConditions)
      const FWPM_FILTER_CONDITION0* filterConditions,
   _In_opt_ PSECURITY_DESCRIPTOR sd
   );

DWORD
WINAPI
FwpmConnectionPolicyDeleteByKey0(
   _In_ HANDLE engineHandle,
   _In_ const GUID* key
   );
#endif // (NTDDI_VERSION >= NTDDI_WIN11_ZN)

#ifdef __cplusplus
}
#endif
#endif // FWPMX_H
#endif // GUID_DEFS_ONLY
#endif // (NTDDI_VERSION >= NTDDI_WIN6)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_APPRUNTIME) */
#pragma endregion

