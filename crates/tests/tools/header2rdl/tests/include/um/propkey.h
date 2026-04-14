
#pragma once
//===========================================================================
//
// Copyright (c) Microsoft Corporation. All rights reserved.
//
//===========================================================================

#ifndef _INC_PROPKEY
#define _INC_PROPKEY

#ifndef DEFINE_API_PKEY
#define DEFINE_API_PKEY(name, managed_name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8, pid) \
        DEFINE_PROPERTYKEY(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8, pid)
#endif


#include <propkeydef.h>

#ifndef _WIN32_IE
#define _WIN32_IE 0x0501
#else
#if (_WIN32_IE < 0x0400) && defined(_WIN32_WINNT) && (_WIN32_WINNT >= 0x0500)
#error _WIN32_IE setting conflicts with _WIN32_WINNT setting
#endif
#endif


//-----------------------------------------------------------------------------
// Address properties

//  Name:     System.Address.Country -- PKEY_Address_Country
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C07B4199-E1DF-4493-B1E1-DE5946FB58F8}, 100
DEFINE_PROPERTYKEY(PKEY_Address_Country, 0xC07B4199, 0xE1DF, 0x4493, 0xB1, 0xE1, 0xDE, 0x59, 0x46, 0xFB, 0x58, 0xF8, 100);
#define INIT_PKEY_Address_Country { { 0xC07B4199, 0xE1DF, 0x4493, 0xB1, 0xE1, 0xDE, 0x59, 0x46, 0xFB, 0x58, 0xF8 }, 100 }

//  Name:     System.Address.CountryCode -- PKEY_Address_CountryCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C07B4199-E1DF-4493-B1E1-DE5946FB58F8}, 101
DEFINE_PROPERTYKEY(PKEY_Address_CountryCode, 0xC07B4199, 0xE1DF, 0x4493, 0xB1, 0xE1, 0xDE, 0x59, 0x46, 0xFB, 0x58, 0xF8, 101);
#define INIT_PKEY_Address_CountryCode { { 0xC07B4199, 0xE1DF, 0x4493, 0xB1, 0xE1, 0xDE, 0x59, 0x46, 0xFB, 0x58, 0xF8 }, 101 }

//  Name:     System.Address.Region -- PKEY_Address_Region
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C07B4199-E1DF-4493-B1E1-DE5946FB58F8}, 102
DEFINE_PROPERTYKEY(PKEY_Address_Region, 0xC07B4199, 0xE1DF, 0x4493, 0xB1, 0xE1, 0xDE, 0x59, 0x46, 0xFB, 0x58, 0xF8, 102);
#define INIT_PKEY_Address_Region { { 0xC07B4199, 0xE1DF, 0x4493, 0xB1, 0xE1, 0xDE, 0x59, 0x46, 0xFB, 0x58, 0xF8 }, 102 }

//  Name:     System.Address.RegionCode -- PKEY_Address_RegionCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C07B4199-E1DF-4493-B1E1-DE5946FB58F8}, 103
DEFINE_PROPERTYKEY(PKEY_Address_RegionCode, 0xC07B4199, 0xE1DF, 0x4493, 0xB1, 0xE1, 0xDE, 0x59, 0x46, 0xFB, 0x58, 0xF8, 103);
#define INIT_PKEY_Address_RegionCode { { 0xC07B4199, 0xE1DF, 0x4493, 0xB1, 0xE1, 0xDE, 0x59, 0x46, 0xFB, 0x58, 0xF8 }, 103 }

//  Name:     System.Address.Town -- PKEY_Address_Town
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C07B4199-E1DF-4493-B1E1-DE5946FB58F8}, 104
DEFINE_PROPERTYKEY(PKEY_Address_Town, 0xC07B4199, 0xE1DF, 0x4493, 0xB1, 0xE1, 0xDE, 0x59, 0x46, 0xFB, 0x58, 0xF8, 104);
#define INIT_PKEY_Address_Town { { 0xC07B4199, 0xE1DF, 0x4493, 0xB1, 0xE1, 0xDE, 0x59, 0x46, 0xFB, 0x58, 0xF8 }, 104 }

//-----------------------------------------------------------------------------
// Audio properties

//  Name:     System.Audio.ChannelCount -- PKEY_Audio_ChannelCount
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_AudioSummaryInformation) {64440490-4C8B-11D1-8B70-080036B11A03}, 7 (PIDASI_CHANNEL_COUNT)
//
//  Indicates the channel count for the audio file.  Values: 1 (mono), 2 (stereo).
DEFINE_PROPERTYKEY(PKEY_Audio_ChannelCount, 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 7);
#define INIT_PKEY_Audio_ChannelCount { { 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 7 }

// Possible discrete values for PKEY_Audio_ChannelCount are:
#define AUDIO_CHANNELCOUNT_MONO             1ul
#define AUDIO_CHANNELCOUNT_STEREO           2ul

//  Name:     System.Audio.Compression -- PKEY_Audio_Compression
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_AudioSummaryInformation) {64440490-4C8B-11D1-8B70-080036B11A03}, 10 (PIDASI_COMPRESSION)
//
//  
DEFINE_PROPERTYKEY(PKEY_Audio_Compression, 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 10);
#define INIT_PKEY_Audio_Compression { { 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 10 }

//  Name:     System.Audio.EncodingBitrate -- PKEY_Audio_EncodingBitrate
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_AudioSummaryInformation) {64440490-4C8B-11D1-8B70-080036B11A03}, 4 (PIDASI_AVG_DATA_RATE)
//
//  Indicates the average data rate in Hz for the audio file in "bits per second".
DEFINE_PROPERTYKEY(PKEY_Audio_EncodingBitrate, 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 4);
#define INIT_PKEY_Audio_EncodingBitrate { { 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 4 }

//  Name:     System.Audio.Format -- PKEY_Audio_Format
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)  Legacy code may treat this as VT_BSTR.
//  FormatID: (FMTID_AudioSummaryInformation) {64440490-4C8B-11D1-8B70-080036B11A03}, 2 (PIDASI_FORMAT)
//
//  Indicates the format of the audio file.
DEFINE_PROPERTYKEY(PKEY_Audio_Format, 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 2);
#define INIT_PKEY_Audio_Format { { 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 2 }

//  Name:     System.Audio.IsVariableBitRate -- PKEY_Audio_IsVariableBitRate
//  Type:     Boolean -- VT_BOOL
//  FormatID: {E6822FEE-8C17-4D62-823C-8E9CFCBD1D5C}, 100
DEFINE_PROPERTYKEY(PKEY_Audio_IsVariableBitRate, 0xE6822FEE, 0x8C17, 0x4D62, 0x82, 0x3C, 0x8E, 0x9C, 0xFC, 0xBD, 0x1D, 0x5C, 100);
#define INIT_PKEY_Audio_IsVariableBitRate { { 0xE6822FEE, 0x8C17, 0x4D62, 0x82, 0x3C, 0x8E, 0x9C, 0xFC, 0xBD, 0x1D, 0x5C }, 100 }

//  Name:     System.Audio.PeakValue -- PKEY_Audio_PeakValue
//  Type:     UInt32 -- VT_UI4
//  FormatID: {2579E5D0-1116-4084-BD9A-9B4F7CB4DF5E}, 100
DEFINE_PROPERTYKEY(PKEY_Audio_PeakValue, 0x2579E5D0, 0x1116, 0x4084, 0xBD, 0x9A, 0x9B, 0x4F, 0x7C, 0xB4, 0xDF, 0x5E, 100);
#define INIT_PKEY_Audio_PeakValue { { 0x2579E5D0, 0x1116, 0x4084, 0xBD, 0x9A, 0x9B, 0x4F, 0x7C, 0xB4, 0xDF, 0x5E }, 100 }

//  Name:     System.Audio.SampleRate -- PKEY_Audio_SampleRate
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_AudioSummaryInformation) {64440490-4C8B-11D1-8B70-080036B11A03}, 5 (PIDASI_SAMPLE_RATE)
//
//  Indicates the audio sample rate for the audio file in "samples per second".
DEFINE_PROPERTYKEY(PKEY_Audio_SampleRate, 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 5);
#define INIT_PKEY_Audio_SampleRate { { 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 5 }

//  Name:     System.Audio.SampleSize -- PKEY_Audio_SampleSize
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_AudioSummaryInformation) {64440490-4C8B-11D1-8B70-080036B11A03}, 6 (PIDASI_SAMPLE_SIZE)
//
//  Indicates the audio sample size for the audio file in "bits per sample".
DEFINE_PROPERTYKEY(PKEY_Audio_SampleSize, 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 6);
#define INIT_PKEY_Audio_SampleSize { { 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 6 }

//  Name:     System.Audio.StreamName -- PKEY_Audio_StreamName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_AudioSummaryInformation) {64440490-4C8B-11D1-8B70-080036B11A03}, 9 (PIDASI_STREAM_NAME)
//
//  
DEFINE_PROPERTYKEY(PKEY_Audio_StreamName, 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 9);
#define INIT_PKEY_Audio_StreamName { { 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 9 }

//  Name:     System.Audio.StreamNumber -- PKEY_Audio_StreamNumber
//  Type:     UInt16 -- VT_UI2
//  FormatID: (FMTID_AudioSummaryInformation) {64440490-4C8B-11D1-8B70-080036B11A03}, 8 (PIDASI_STREAM_NUMBER)
//
//  
DEFINE_PROPERTYKEY(PKEY_Audio_StreamNumber, 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 8);
#define INIT_PKEY_Audio_StreamNumber { { 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 8 }

//-----------------------------------------------------------------------------
// Calendar properties

//  Name:     System.Calendar.Duration -- PKEY_Calendar_Duration
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {293CA35A-09AA-4DD2-B180-1FE245728A52}, 100
//
//  The duration as specified in a string.
DEFINE_PROPERTYKEY(PKEY_Calendar_Duration, 0x293CA35A, 0x09AA, 0x4DD2, 0xB1, 0x80, 0x1F, 0xE2, 0x45, 0x72, 0x8A, 0x52, 100);
#define INIT_PKEY_Calendar_Duration { { 0x293CA35A, 0x09AA, 0x4DD2, 0xB1, 0x80, 0x1F, 0xE2, 0x45, 0x72, 0x8A, 0x52 }, 100 }

//  Name:     System.Calendar.IsOnline -- PKEY_Calendar_IsOnline
//  Type:     Boolean -- VT_BOOL
//  FormatID: {BFEE9149-E3E2-49A7-A862-C05988145CEC}, 100
//
//  Identifies if the event is an online event.
DEFINE_PROPERTYKEY(PKEY_Calendar_IsOnline, 0xBFEE9149, 0xE3E2, 0x49A7, 0xA8, 0x62, 0xC0, 0x59, 0x88, 0x14, 0x5C, 0xEC, 100);
#define INIT_PKEY_Calendar_IsOnline { { 0xBFEE9149, 0xE3E2, 0x49A7, 0xA8, 0x62, 0xC0, 0x59, 0x88, 0x14, 0x5C, 0xEC }, 100 }

//  Name:     System.Calendar.IsRecurring -- PKEY_Calendar_IsRecurring
//  Type:     Boolean -- VT_BOOL
//  FormatID: {315B9C8D-80A9-4EF9-AE16-8E746DA51D70}, 100
DEFINE_PROPERTYKEY(PKEY_Calendar_IsRecurring, 0x315B9C8D, 0x80A9, 0x4EF9, 0xAE, 0x16, 0x8E, 0x74, 0x6D, 0xA5, 0x1D, 0x70, 100);
#define INIT_PKEY_Calendar_IsRecurring { { 0x315B9C8D, 0x80A9, 0x4EF9, 0xAE, 0x16, 0x8E, 0x74, 0x6D, 0xA5, 0x1D, 0x70 }, 100 }

//  Name:     System.Calendar.Location -- PKEY_Calendar_Location
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {F6272D18-CECC-40B1-B26A-3911717AA7BD}, 100
DEFINE_PROPERTYKEY(PKEY_Calendar_Location, 0xF6272D18, 0xCECC, 0x40B1, 0xB2, 0x6A, 0x39, 0x11, 0x71, 0x7A, 0xA7, 0xBD, 100);
#define INIT_PKEY_Calendar_Location { { 0xF6272D18, 0xCECC, 0x40B1, 0xB2, 0x6A, 0x39, 0x11, 0x71, 0x7A, 0xA7, 0xBD }, 100 }

//  Name:     System.Calendar.OptionalAttendeeAddresses -- PKEY_Calendar_OptionalAttendeeAddresses
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D55BAE5A-3892-417A-A649-C6AC5AAAEAB3}, 100
DEFINE_PROPERTYKEY(PKEY_Calendar_OptionalAttendeeAddresses, 0xD55BAE5A, 0x3892, 0x417A, 0xA6, 0x49, 0xC6, 0xAC, 0x5A, 0xAA, 0xEA, 0xB3, 100);
#define INIT_PKEY_Calendar_OptionalAttendeeAddresses { { 0xD55BAE5A, 0x3892, 0x417A, 0xA6, 0x49, 0xC6, 0xAC, 0x5A, 0xAA, 0xEA, 0xB3 }, 100 }

//  Name:     System.Calendar.OptionalAttendeeNames -- PKEY_Calendar_OptionalAttendeeNames
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {09429607-582D-437F-84C3-DE93A2B24C3C}, 100
DEFINE_PROPERTYKEY(PKEY_Calendar_OptionalAttendeeNames, 0x09429607, 0x582D, 0x437F, 0x84, 0xC3, 0xDE, 0x93, 0xA2, 0xB2, 0x4C, 0x3C, 100);
#define INIT_PKEY_Calendar_OptionalAttendeeNames { { 0x09429607, 0x582D, 0x437F, 0x84, 0xC3, 0xDE, 0x93, 0xA2, 0xB2, 0x4C, 0x3C }, 100 }

//  Name:     System.Calendar.OrganizerAddress -- PKEY_Calendar_OrganizerAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {744C8242-4DF5-456C-AB9E-014EFB9021E3}, 100
//
//  Address of the organizer organizing the event.
DEFINE_PROPERTYKEY(PKEY_Calendar_OrganizerAddress, 0x744C8242, 0x4DF5, 0x456C, 0xAB, 0x9E, 0x01, 0x4E, 0xFB, 0x90, 0x21, 0xE3, 100);
#define INIT_PKEY_Calendar_OrganizerAddress { { 0x744C8242, 0x4DF5, 0x456C, 0xAB, 0x9E, 0x01, 0x4E, 0xFB, 0x90, 0x21, 0xE3 }, 100 }

//  Name:     System.Calendar.OrganizerName -- PKEY_Calendar_OrganizerName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {AAA660F9-9865-458E-B484-01BC7FE3973E}, 100
//
//  Name of the organizer organizing the event.
DEFINE_PROPERTYKEY(PKEY_Calendar_OrganizerName, 0xAAA660F9, 0x9865, 0x458E, 0xB4, 0x84, 0x01, 0xBC, 0x7F, 0xE3, 0x97, 0x3E, 100);
#define INIT_PKEY_Calendar_OrganizerName { { 0xAAA660F9, 0x9865, 0x458E, 0xB4, 0x84, 0x01, 0xBC, 0x7F, 0xE3, 0x97, 0x3E }, 100 }

//  Name:     System.Calendar.ReminderTime -- PKEY_Calendar_ReminderTime
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {72FC5BA4-24F9-4011-9F3F-ADD27AFAD818}, 100
DEFINE_PROPERTYKEY(PKEY_Calendar_ReminderTime, 0x72FC5BA4, 0x24F9, 0x4011, 0x9F, 0x3F, 0xAD, 0xD2, 0x7A, 0xFA, 0xD8, 0x18, 100);
#define INIT_PKEY_Calendar_ReminderTime { { 0x72FC5BA4, 0x24F9, 0x4011, 0x9F, 0x3F, 0xAD, 0xD2, 0x7A, 0xFA, 0xD8, 0x18 }, 100 }

//  Name:     System.Calendar.RequiredAttendeeAddresses -- PKEY_Calendar_RequiredAttendeeAddresses
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {0BA7D6C3-568D-4159-AB91-781A91FB71E5}, 100
DEFINE_PROPERTYKEY(PKEY_Calendar_RequiredAttendeeAddresses, 0x0BA7D6C3, 0x568D, 0x4159, 0xAB, 0x91, 0x78, 0x1A, 0x91, 0xFB, 0x71, 0xE5, 100);
#define INIT_PKEY_Calendar_RequiredAttendeeAddresses { { 0x0BA7D6C3, 0x568D, 0x4159, 0xAB, 0x91, 0x78, 0x1A, 0x91, 0xFB, 0x71, 0xE5 }, 100 }

//  Name:     System.Calendar.RequiredAttendeeNames -- PKEY_Calendar_RequiredAttendeeNames
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {B33AF30B-F552-4584-936C-CB93E5CDA29F}, 100
DEFINE_PROPERTYKEY(PKEY_Calendar_RequiredAttendeeNames, 0xB33AF30B, 0xF552, 0x4584, 0x93, 0x6C, 0xCB, 0x93, 0xE5, 0xCD, 0xA2, 0x9F, 100);
#define INIT_PKEY_Calendar_RequiredAttendeeNames { { 0xB33AF30B, 0xF552, 0x4584, 0x93, 0x6C, 0xCB, 0x93, 0xE5, 0xCD, 0xA2, 0x9F }, 100 }

//  Name:     System.Calendar.Resources -- PKEY_Calendar_Resources
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {00F58A38-C54B-4C40-8696-97235980EAE1}, 100
DEFINE_PROPERTYKEY(PKEY_Calendar_Resources, 0x00F58A38, 0xC54B, 0x4C40, 0x86, 0x96, 0x97, 0x23, 0x59, 0x80, 0xEA, 0xE1, 100);
#define INIT_PKEY_Calendar_Resources { { 0x00F58A38, 0xC54B, 0x4C40, 0x86, 0x96, 0x97, 0x23, 0x59, 0x80, 0xEA, 0xE1 }, 100 }

//  Name:     System.Calendar.ResponseStatus -- PKEY_Calendar_ResponseStatus
//  Type:     UInt16 -- VT_UI2
//  FormatID: {188C1F91-3C40-4132-9EC5-D8B03B72A8A2}, 100
//  
//  This property stores the status of the user responses to meetings in her calendar.
DEFINE_PROPERTYKEY(PKEY_Calendar_ResponseStatus, 0x188C1F91, 0x3C40, 0x4132, 0x9E, 0xC5, 0xD8, 0xB0, 0x3B, 0x72, 0xA8, 0xA2, 100);
#define INIT_PKEY_Calendar_ResponseStatus { { 0x188C1F91, 0x3C40, 0x4132, 0x9E, 0xC5, 0xD8, 0xB0, 0x3B, 0x72, 0xA8, 0xA2 }, 100 }

// Possible discrete values for PKEY_Calendar_ResponseStatus are:
#define CALENDAR_RESPONSESTATUS_NONE        0u
#define CALENDAR_RESPONSESTATUS_ORGANIZED   1u
#define CALENDAR_RESPONSESTATUS_TENTATIVE   2u
#define CALENDAR_RESPONSESTATUS_ACCEPTED    3u
#define CALENDAR_RESPONSESTATUS_DECLINED    4u
#define CALENDAR_RESPONSESTATUS_NOTRESPONDED 5u

//  Name:     System.Calendar.ShowTimeAs -- PKEY_Calendar_ShowTimeAs
//  Type:     UInt16 -- VT_UI2
//  FormatID: {5BF396D4-5EB2-466F-BDE9-2FB3F2361D6E}, 100
//
//  
DEFINE_PROPERTYKEY(PKEY_Calendar_ShowTimeAs, 0x5BF396D4, 0x5EB2, 0x466F, 0xBD, 0xE9, 0x2F, 0xB3, 0xF2, 0x36, 0x1D, 0x6E, 100);
#define INIT_PKEY_Calendar_ShowTimeAs { { 0x5BF396D4, 0x5EB2, 0x466F, 0xBD, 0xE9, 0x2F, 0xB3, 0xF2, 0x36, 0x1D, 0x6E }, 100 }

// Possible discrete values for PKEY_Calendar_ShowTimeAs are:
#define CALENDAR_SHOWTIMEAS_FREE            0u
#define CALENDAR_SHOWTIMEAS_TENTATIVE       1u
#define CALENDAR_SHOWTIMEAS_BUSY            2u
#define CALENDAR_SHOWTIMEAS_OOF             3u

//  Name:     System.Calendar.ShowTimeAsText -- PKEY_Calendar_ShowTimeAsText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {53DA57CF-62C0-45C4-81DE-7610BCEFD7F5}, 100
//  
//  This is the user-friendly form of System.Calendar.ShowTimeAs.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Calendar_ShowTimeAsText, 0x53DA57CF, 0x62C0, 0x45C4, 0x81, 0xDE, 0x76, 0x10, 0xBC, 0xEF, 0xD7, 0xF5, 100);
#define INIT_PKEY_Calendar_ShowTimeAsText { { 0x53DA57CF, 0x62C0, 0x45C4, 0x81, 0xDE, 0x76, 0x10, 0xBC, 0xEF, 0xD7, 0xF5 }, 100 }

//-----------------------------------------------------------------------------
// Communication properties

//  Name:     System.Communication.AccountName -- PKEY_Communication_AccountName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 9
//
//  Account Name
DEFINE_PROPERTYKEY(PKEY_Communication_AccountName, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 9);
#define INIT_PKEY_Communication_AccountName { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 9 }

//  Name:     System.Communication.DateItemExpires -- PKEY_Communication_DateItemExpires
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {428040AC-A177-4C8A-9760-F6F761227F9A}, 100
//  
//  Date the item expires due to the retention policy.
DEFINE_PROPERTYKEY(PKEY_Communication_DateItemExpires, 0x428040AC, 0xA177, 0x4C8A, 0x97, 0x60, 0xF6, 0xF7, 0x61, 0x22, 0x7F, 0x9A, 100);
#define INIT_PKEY_Communication_DateItemExpires { { 0x428040AC, 0xA177, 0x4C8A, 0x97, 0x60, 0xF6, 0xF7, 0x61, 0x22, 0x7F, 0x9A }, 100 }

//  Name:     System.Communication.Direction -- PKEY_Communication_Direction
//  Type:     UInt16 -- VT_UI2
//  FormatID: {8E531030-B960-4346-AE0D-66BC9A86FB94}, 100
//
//  Indicates whether a communication was incoming or outgoing
DEFINE_PROPERTYKEY(PKEY_Communication_Direction, 0x8E531030, 0xB960, 0x4346, 0xAE, 0x0D, 0x66, 0xBC, 0x9A, 0x86, 0xFB, 0x94, 100);
#define INIT_PKEY_Communication_Direction { { 0x8E531030, 0xB960, 0x4346, 0xAE, 0x0D, 0x66, 0xBC, 0x9A, 0x86, 0xFB, 0x94 }, 100 }

// Possible discrete values for PKEY_Communication_Direction are:
#define COMMUNICATION_DIRECTION_UNKNOWN     0u
#define COMMUNICATION_DIRECTION_INCOMING    1u
#define COMMUNICATION_DIRECTION_OUTGOING    2u

//  Name:     System.Communication.FollowupIconIndex -- PKEY_Communication_FollowupIconIndex
//  Type:     Int32 -- VT_I4
//  FormatID: {83A6347E-6FE4-4F40-BA9C-C4865240D1F4}, 100
//  
//  This is the icon index used on messages marked for followup.
DEFINE_PROPERTYKEY(PKEY_Communication_FollowupIconIndex, 0x83A6347E, 0x6FE4, 0x4F40, 0xBA, 0x9C, 0xC4, 0x86, 0x52, 0x40, 0xD1, 0xF4, 100);
#define INIT_PKEY_Communication_FollowupIconIndex { { 0x83A6347E, 0x6FE4, 0x4F40, 0xBA, 0x9C, 0xC4, 0x86, 0x52, 0x40, 0xD1, 0xF4 }, 100 }

//  Name:     System.Communication.HeaderItem -- PKEY_Communication_HeaderItem
//  Type:     Boolean -- VT_BOOL
//  FormatID: {C9C34F84-2241-4401-B607-BD20ED75AE7F}, 100
//  
//  This property will be true if the item is a header item which means the item hasn't been fully downloaded.
DEFINE_PROPERTYKEY(PKEY_Communication_HeaderItem, 0xC9C34F84, 0x2241, 0x4401, 0xB6, 0x07, 0xBD, 0x20, 0xED, 0x75, 0xAE, 0x7F, 100);
#define INIT_PKEY_Communication_HeaderItem { { 0xC9C34F84, 0x2241, 0x4401, 0xB6, 0x07, 0xBD, 0x20, 0xED, 0x75, 0xAE, 0x7F }, 100 }

//  Name:     System.Communication.PolicyTag -- PKEY_Communication_PolicyTag
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {EC0B4191-AB0B-4C66-90B6-C6637CDEBBAB}, 100
//  
//  This a string used to identify the retention policy applied to the item.
DEFINE_PROPERTYKEY(PKEY_Communication_PolicyTag, 0xEC0B4191, 0xAB0B, 0x4C66, 0x90, 0xB6, 0xC6, 0x63, 0x7C, 0xDE, 0xBB, 0xAB, 100);
#define INIT_PKEY_Communication_PolicyTag { { 0xEC0B4191, 0xAB0B, 0x4C66, 0x90, 0xB6, 0xC6, 0x63, 0x7C, 0xDE, 0xBB, 0xAB }, 100 }

//  Name:     System.Communication.SecurityFlags -- PKEY_Communication_SecurityFlags
//  Type:     Int32 -- VT_I4
//  FormatID: {8619A4B6-9F4D-4429-8C0F-B996CA59E335}, 100
//  
//  Security flags associated with the item to know if the item is encrypted, signed or DRM enabled.
DEFINE_PROPERTYKEY(PKEY_Communication_SecurityFlags, 0x8619A4B6, 0x9F4D, 0x4429, 0x8C, 0x0F, 0xB9, 0x96, 0xCA, 0x59, 0xE3, 0x35, 100);
#define INIT_PKEY_Communication_SecurityFlags { { 0x8619A4B6, 0x9F4D, 0x4429, 0x8C, 0x0F, 0xB9, 0x96, 0xCA, 0x59, 0xE3, 0x35 }, 100 }

//  Name:     System.Communication.Suffix -- PKEY_Communication_Suffix
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {807B653A-9E91-43EF-8F97-11CE04EE20C5}, 100
DEFINE_PROPERTYKEY(PKEY_Communication_Suffix, 0x807B653A, 0x9E91, 0x43EF, 0x8F, 0x97, 0x11, 0xCE, 0x04, 0xEE, 0x20, 0xC5, 100);
#define INIT_PKEY_Communication_Suffix { { 0x807B653A, 0x9E91, 0x43EF, 0x8F, 0x97, 0x11, 0xCE, 0x04, 0xEE, 0x20, 0xC5 }, 100 }

//  Name:     System.Communication.TaskStatus -- PKEY_Communication_TaskStatus
//  Type:     UInt16 -- VT_UI2
//  FormatID: {BE1A72C6-9A1D-46B7-AFE7-AFAF8CEF4999}, 100
DEFINE_PROPERTYKEY(PKEY_Communication_TaskStatus, 0xBE1A72C6, 0x9A1D, 0x46B7, 0xAF, 0xE7, 0xAF, 0xAF, 0x8C, 0xEF, 0x49, 0x99, 100);
#define INIT_PKEY_Communication_TaskStatus { { 0xBE1A72C6, 0x9A1D, 0x46B7, 0xAF, 0xE7, 0xAF, 0xAF, 0x8C, 0xEF, 0x49, 0x99 }, 100 }

// Possible discrete values for PKEY_Communication_TaskStatus are:
#define TASKSTATUS_NOTSTARTED               0u
#define TASKSTATUS_INPROGRESS               1u
#define TASKSTATUS_COMPLETE                 2u
#define TASKSTATUS_WAITING                  3u
#define TASKSTATUS_DEFERRED                 4u

//  Name:     System.Communication.TaskStatusText -- PKEY_Communication_TaskStatusText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A6744477-C237-475B-A075-54F34498292A}, 100
//  
//  This is the user-friendly form of System.Communication.TaskStatus.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Communication_TaskStatusText, 0xA6744477, 0xC237, 0x475B, 0xA0, 0x75, 0x54, 0xF3, 0x44, 0x98, 0x29, 0x2A, 100);
#define INIT_PKEY_Communication_TaskStatusText { { 0xA6744477, 0xC237, 0x475B, 0xA0, 0x75, 0x54, 0xF3, 0x44, 0x98, 0x29, 0x2A }, 100 }

//-----------------------------------------------------------------------------
// Computer properties

//  Name:     System.Computer.DecoratedFreeSpace -- PKEY_Computer_DecoratedFreeSpace
//  Type:     Multivalue UInt64 -- VT_VECTOR | VT_UI8  (For variants: VT_ARRAY | VT_UI8)
//  FormatID: (FMTID_Volume) {9B174B35-40FF-11D2-A27E-00C04FC30871}, 7  (Filesystem Volume Properties)
//
//  Free space and total space: "%s free of %s"
DEFINE_PROPERTYKEY(PKEY_Computer_DecoratedFreeSpace, 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71, 7);
#define INIT_PKEY_Computer_DecoratedFreeSpace { { 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71 }, 7 }

//-----------------------------------------------------------------------------
// Contact properties

//  Name:     System.Contact.AccountPictureDynamicVideo -- PKEY_Contact_AccountPictureDynamicVideo
//  Type:     Stream -- VT_STREAM
//  FormatID: {0B8BB018-2725-4B44-92BA-7933AEB2DDE7}, 2
//  
//  This is a stream containing the data necessary to render a contact's dynamic video account picture.
DEFINE_PROPERTYKEY(PKEY_Contact_AccountPictureDynamicVideo, 0x0B8BB018, 0x2725, 0x4B44, 0x92, 0xBA, 0x79, 0x33, 0xAE, 0xB2, 0xDD, 0xE7, 2);
#define INIT_PKEY_Contact_AccountPictureDynamicVideo { { 0x0B8BB018, 0x2725, 0x4B44, 0x92, 0xBA, 0x79, 0x33, 0xAE, 0xB2, 0xDD, 0xE7 }, 2 }

//  Name:     System.Contact.AccountPictureLarge -- PKEY_Contact_AccountPictureLarge
//  Type:     Stream -- VT_STREAM
//  FormatID: {0B8BB018-2725-4B44-92BA-7933AEB2DDE7}, 3
//  
//  This is a stream containing the data necessary to render a contact's large account picture.
DEFINE_PROPERTYKEY(PKEY_Contact_AccountPictureLarge, 0x0B8BB018, 0x2725, 0x4B44, 0x92, 0xBA, 0x79, 0x33, 0xAE, 0xB2, 0xDD, 0xE7, 3);
#define INIT_PKEY_Contact_AccountPictureLarge { { 0x0B8BB018, 0x2725, 0x4B44, 0x92, 0xBA, 0x79, 0x33, 0xAE, 0xB2, 0xDD, 0xE7 }, 3 }

//  Name:     System.Contact.AccountPictureSmall -- PKEY_Contact_AccountPictureSmall
//  Type:     Stream -- VT_STREAM
//  FormatID: {0B8BB018-2725-4B44-92BA-7933AEB2DDE7}, 4
//  
//  This is a stream containing the data necessary to render a contact's small account picture.
DEFINE_PROPERTYKEY(PKEY_Contact_AccountPictureSmall, 0x0B8BB018, 0x2725, 0x4B44, 0x92, 0xBA, 0x79, 0x33, 0xAE, 0xB2, 0xDD, 0xE7, 4);
#define INIT_PKEY_Contact_AccountPictureSmall { { 0x0B8BB018, 0x2725, 0x4B44, 0x92, 0xBA, 0x79, 0x33, 0xAE, 0xB2, 0xDD, 0xE7 }, 4 }

//  Name:     System.Contact.Anniversary -- PKEY_Contact_Anniversary
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {9AD5BADB-CEA7-4470-A03D-B84E51B9949E}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_Anniversary, 0x9AD5BADB, 0xCEA7, 0x4470, 0xA0, 0x3D, 0xB8, 0x4E, 0x51, 0xB9, 0x94, 0x9E, 100);
#define INIT_PKEY_Contact_Anniversary { { 0x9AD5BADB, 0xCEA7, 0x4470, 0xA0, 0x3D, 0xB8, 0x4E, 0x51, 0xB9, 0x94, 0x9E }, 100 }

//  Name:     System.Contact.AssistantName -- PKEY_Contact_AssistantName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {CD102C9C-5540-4A88-A6F6-64E4981C8CD1}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_AssistantName, 0xCD102C9C, 0x5540, 0x4A88, 0xA6, 0xF6, 0x64, 0xE4, 0x98, 0x1C, 0x8C, 0xD1, 100);
#define INIT_PKEY_Contact_AssistantName { { 0xCD102C9C, 0x5540, 0x4A88, 0xA6, 0xF6, 0x64, 0xE4, 0x98, 0x1C, 0x8C, 0xD1 }, 100 }

//  Name:     System.Contact.AssistantTelephone -- PKEY_Contact_AssistantTelephone
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9A93244D-A7AD-4FF8-9B99-45EE4CC09AF6}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_AssistantTelephone, 0x9A93244D, 0xA7AD, 0x4FF8, 0x9B, 0x99, 0x45, 0xEE, 0x4C, 0xC0, 0x9A, 0xF6, 100);
#define INIT_PKEY_Contact_AssistantTelephone { { 0x9A93244D, 0xA7AD, 0x4FF8, 0x9B, 0x99, 0x45, 0xEE, 0x4C, 0xC0, 0x9A, 0xF6 }, 100 }

//  Name:     System.Contact.Birthday -- PKEY_Contact_Birthday
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 47
//
//  Contact's birthday. It's recommended to use year 1601 if the birthday does not have year information.
DEFINE_PROPERTYKEY(PKEY_Contact_Birthday, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 47);
#define INIT_PKEY_Contact_Birthday { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 47 }

//  Name:     System.Contact.BusinessAddress -- PKEY_Contact_BusinessAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {730FB6DD-CF7C-426B-A03F-BD166CC9EE24}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress, 0x730FB6DD, 0xCF7C, 0x426B, 0xA0, 0x3F, 0xBD, 0x16, 0x6C, 0xC9, 0xEE, 0x24, 100);
#define INIT_PKEY_Contact_BusinessAddress { { 0x730FB6DD, 0xCF7C, 0x426B, 0xA0, 0x3F, 0xBD, 0x16, 0x6C, 0xC9, 0xEE, 0x24 }, 100 }

//  Name:     System.Contact.BusinessAddress1Country -- PKEY_Contact_BusinessAddress1Country
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 119
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress1Country, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 119);
#define INIT_PKEY_Contact_BusinessAddress1Country { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 119 }

//  Name:     System.Contact.BusinessAddress1Locality -- PKEY_Contact_BusinessAddress1Locality
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 117
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress1Locality, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 117);
#define INIT_PKEY_Contact_BusinessAddress1Locality { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 117 }

//  Name:     System.Contact.BusinessAddress1PostalCode -- PKEY_Contact_BusinessAddress1PostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 120
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress1PostalCode, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 120);
#define INIT_PKEY_Contact_BusinessAddress1PostalCode { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 120 }

//  Name:     System.Contact.BusinessAddress1Region -- PKEY_Contact_BusinessAddress1Region
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 118
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress1Region, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 118);
#define INIT_PKEY_Contact_BusinessAddress1Region { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 118 }

//  Name:     System.Contact.BusinessAddress1Street -- PKEY_Contact_BusinessAddress1Street
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 116
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress1Street, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 116);
#define INIT_PKEY_Contact_BusinessAddress1Street { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 116 }

//  Name:     System.Contact.BusinessAddress2Country -- PKEY_Contact_BusinessAddress2Country
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 124
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress2Country, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 124);
#define INIT_PKEY_Contact_BusinessAddress2Country { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 124 }

//  Name:     System.Contact.BusinessAddress2Locality -- PKEY_Contact_BusinessAddress2Locality
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 122
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress2Locality, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 122);
#define INIT_PKEY_Contact_BusinessAddress2Locality { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 122 }

//  Name:     System.Contact.BusinessAddress2PostalCode -- PKEY_Contact_BusinessAddress2PostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 125
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress2PostalCode, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 125);
#define INIT_PKEY_Contact_BusinessAddress2PostalCode { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 125 }

//  Name:     System.Contact.BusinessAddress2Region -- PKEY_Contact_BusinessAddress2Region
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 123
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress2Region, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 123);
#define INIT_PKEY_Contact_BusinessAddress2Region { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 123 }

//  Name:     System.Contact.BusinessAddress2Street -- PKEY_Contact_BusinessAddress2Street
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 121
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress2Street, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 121);
#define INIT_PKEY_Contact_BusinessAddress2Street { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 121 }

//  Name:     System.Contact.BusinessAddress3Country -- PKEY_Contact_BusinessAddress3Country
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 129
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress3Country, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 129);
#define INIT_PKEY_Contact_BusinessAddress3Country { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 129 }

//  Name:     System.Contact.BusinessAddress3Locality -- PKEY_Contact_BusinessAddress3Locality
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 127
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress3Locality, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 127);
#define INIT_PKEY_Contact_BusinessAddress3Locality { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 127 }

//  Name:     System.Contact.BusinessAddress3PostalCode -- PKEY_Contact_BusinessAddress3PostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 130
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress3PostalCode, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 130);
#define INIT_PKEY_Contact_BusinessAddress3PostalCode { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 130 }

//  Name:     System.Contact.BusinessAddress3Region -- PKEY_Contact_BusinessAddress3Region
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 128
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress3Region, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 128);
#define INIT_PKEY_Contact_BusinessAddress3Region { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 128 }

//  Name:     System.Contact.BusinessAddress3Street -- PKEY_Contact_BusinessAddress3Street
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 126
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddress3Street, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 126);
#define INIT_PKEY_Contact_BusinessAddress3Street { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 126 }

//  Name:     System.Contact.BusinessAddressCity -- PKEY_Contact_BusinessAddressCity
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {402B5934-EC5A-48C3-93E6-85E86A2D934E}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddressCity, 0x402B5934, 0xEC5A, 0x48C3, 0x93, 0xE6, 0x85, 0xE8, 0x6A, 0x2D, 0x93, 0x4E, 100);
#define INIT_PKEY_Contact_BusinessAddressCity { { 0x402B5934, 0xEC5A, 0x48C3, 0x93, 0xE6, 0x85, 0xE8, 0x6A, 0x2D, 0x93, 0x4E }, 100 }

//  Name:     System.Contact.BusinessAddressCountry -- PKEY_Contact_BusinessAddressCountry
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B0B87314-FCF6-4FEB-8DFF-A50DA6AF561C}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddressCountry, 0xB0B87314, 0xFCF6, 0x4FEB, 0x8D, 0xFF, 0xA5, 0x0D, 0xA6, 0xAF, 0x56, 0x1C, 100);
#define INIT_PKEY_Contact_BusinessAddressCountry { { 0xB0B87314, 0xFCF6, 0x4FEB, 0x8D, 0xFF, 0xA5, 0x0D, 0xA6, 0xAF, 0x56, 0x1C }, 100 }

//  Name:     System.Contact.BusinessAddressPostalCode -- PKEY_Contact_BusinessAddressPostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E1D4A09E-D758-4CD1-B6EC-34A8B5A73F80}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddressPostalCode, 0xE1D4A09E, 0xD758, 0x4CD1, 0xB6, 0xEC, 0x34, 0xA8, 0xB5, 0xA7, 0x3F, 0x80, 100);
#define INIT_PKEY_Contact_BusinessAddressPostalCode { { 0xE1D4A09E, 0xD758, 0x4CD1, 0xB6, 0xEC, 0x34, 0xA8, 0xB5, 0xA7, 0x3F, 0x80 }, 100 }

//  Name:     System.Contact.BusinessAddressPostOfficeBox -- PKEY_Contact_BusinessAddressPostOfficeBox
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {BC4E71CE-17F9-48D5-BEE9-021DF0EA5409}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddressPostOfficeBox, 0xBC4E71CE, 0x17F9, 0x48D5, 0xBE, 0xE9, 0x02, 0x1D, 0xF0, 0xEA, 0x54, 0x09, 100);
#define INIT_PKEY_Contact_BusinessAddressPostOfficeBox { { 0xBC4E71CE, 0x17F9, 0x48D5, 0xBE, 0xE9, 0x02, 0x1D, 0xF0, 0xEA, 0x54, 0x09 }, 100 }

//  Name:     System.Contact.BusinessAddressState -- PKEY_Contact_BusinessAddressState
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {446F787F-10C4-41CB-A6C4-4D0343551597}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddressState, 0x446F787F, 0x10C4, 0x41CB, 0xA6, 0xC4, 0x4D, 0x03, 0x43, 0x55, 0x15, 0x97, 100);
#define INIT_PKEY_Contact_BusinessAddressState { { 0x446F787F, 0x10C4, 0x41CB, 0xA6, 0xC4, 0x4D, 0x03, 0x43, 0x55, 0x15, 0x97 }, 100 }

//  Name:     System.Contact.BusinessAddressStreet -- PKEY_Contact_BusinessAddressStreet
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {DDD1460F-C0BF-4553-8CE4-10433C908FB0}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessAddressStreet, 0xDDD1460F, 0xC0BF, 0x4553, 0x8C, 0xE4, 0x10, 0x43, 0x3C, 0x90, 0x8F, 0xB0, 100);
#define INIT_PKEY_Contact_BusinessAddressStreet { { 0xDDD1460F, 0xC0BF, 0x4553, 0x8C, 0xE4, 0x10, 0x43, 0x3C, 0x90, 0x8F, 0xB0 }, 100 }

//  Name:     System.Contact.BusinessEmailAddresses -- PKEY_Contact_BusinessEmailAddresses
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {F271C659-7E5E-471F-BA25-7F77B286F836}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessEmailAddresses, 0xF271C659, 0x7E5E, 0x471F, 0xBA, 0x25, 0x7F, 0x77, 0xB2, 0x86, 0xF8, 0x36, 100);
#define INIT_PKEY_Contact_BusinessEmailAddresses { { 0xF271C659, 0x7E5E, 0x471F, 0xBA, 0x25, 0x7F, 0x77, 0xB2, 0x86, 0xF8, 0x36 }, 100 }

//  Name:     System.Contact.BusinessFaxNumber -- PKEY_Contact_BusinessFaxNumber
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {91EFF6F3-2E27-42CA-933E-7C999FBE310B}, 100
//
//  Business fax number of the contact.
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessFaxNumber, 0x91EFF6F3, 0x2E27, 0x42CA, 0x93, 0x3E, 0x7C, 0x99, 0x9F, 0xBE, 0x31, 0x0B, 100);
#define INIT_PKEY_Contact_BusinessFaxNumber { { 0x91EFF6F3, 0x2E27, 0x42CA, 0x93, 0x3E, 0x7C, 0x99, 0x9F, 0xBE, 0x31, 0x0B }, 100 }

//  Name:     System.Contact.BusinessHomePage -- PKEY_Contact_BusinessHomePage
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {56310920-2491-4919-99CE-EADB06FAFDB2}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessHomePage, 0x56310920, 0x2491, 0x4919, 0x99, 0xCE, 0xEA, 0xDB, 0x06, 0xFA, 0xFD, 0xB2, 100);
#define INIT_PKEY_Contact_BusinessHomePage { { 0x56310920, 0x2491, 0x4919, 0x99, 0xCE, 0xEA, 0xDB, 0x06, 0xFA, 0xFD, 0xB2 }, 100 }

//  Name:     System.Contact.BusinessTelephone -- PKEY_Contact_BusinessTelephone
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {6A15E5A0-0A1E-4CD7-BB8C-D2F1B0C929BC}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_BusinessTelephone, 0x6A15E5A0, 0x0A1E, 0x4CD7, 0xBB, 0x8C, 0xD2, 0xF1, 0xB0, 0xC9, 0x29, 0xBC, 100);
#define INIT_PKEY_Contact_BusinessTelephone { { 0x6A15E5A0, 0x0A1E, 0x4CD7, 0xBB, 0x8C, 0xD2, 0xF1, 0xB0, 0xC9, 0x29, 0xBC }, 100 }

//  Name:     System.Contact.CallbackTelephone -- PKEY_Contact_CallbackTelephone
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {BF53D1C3-49E0-4F7F-8567-5A821D8AC542}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_CallbackTelephone, 0xBF53D1C3, 0x49E0, 0x4F7F, 0x85, 0x67, 0x5A, 0x82, 0x1D, 0x8A, 0xC5, 0x42, 100);
#define INIT_PKEY_Contact_CallbackTelephone { { 0xBF53D1C3, 0x49E0, 0x4F7F, 0x85, 0x67, 0x5A, 0x82, 0x1D, 0x8A, 0xC5, 0x42 }, 100 }

//  Name:     System.Contact.CarTelephone -- PKEY_Contact_CarTelephone
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8FDC6DEA-B929-412B-BA90-397A257465FE}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_CarTelephone, 0x8FDC6DEA, 0xB929, 0x412B, 0xBA, 0x90, 0x39, 0x7A, 0x25, 0x74, 0x65, 0xFE, 100);
#define INIT_PKEY_Contact_CarTelephone { { 0x8FDC6DEA, 0xB929, 0x412B, 0xBA, 0x90, 0x39, 0x7A, 0x25, 0x74, 0x65, 0xFE }, 100 }

//  Name:     System.Contact.Children -- PKEY_Contact_Children
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D4729704-8EF1-43EF-9024-2BD381187FD5}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_Children, 0xD4729704, 0x8EF1, 0x43EF, 0x90, 0x24, 0x2B, 0xD3, 0x81, 0x18, 0x7F, 0xD5, 100);
#define INIT_PKEY_Contact_Children { { 0xD4729704, 0x8EF1, 0x43EF, 0x90, 0x24, 0x2B, 0xD3, 0x81, 0x18, 0x7F, 0xD5 }, 100 }

//  Name:     System.Contact.CompanyMainTelephone -- PKEY_Contact_CompanyMainTelephone
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8589E481-6040-473D-B171-7FA89C2708ED}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_CompanyMainTelephone, 0x8589E481, 0x6040, 0x473D, 0xB1, 0x71, 0x7F, 0xA8, 0x9C, 0x27, 0x08, 0xED, 100);
#define INIT_PKEY_Contact_CompanyMainTelephone { { 0x8589E481, 0x6040, 0x473D, 0xB1, 0x71, 0x7F, 0xA8, 0x9C, 0x27, 0x08, 0xED }, 100 }

//  Name:     System.Contact.ConnectedServiceDisplayName -- PKEY_Contact_ConnectedServiceDisplayName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {39B77F4F-A104-4863-B395-2DB2AD8F7BC1}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_ConnectedServiceDisplayName, 0x39B77F4F, 0xA104, 0x4863, 0xB3, 0x95, 0x2D, 0xB2, 0xAD, 0x8F, 0x7B, 0xC1, 100);
#define INIT_PKEY_Contact_ConnectedServiceDisplayName { { 0x39B77F4F, 0xA104, 0x4863, 0xB3, 0x95, 0x2D, 0xB2, 0xAD, 0x8F, 0x7B, 0xC1 }, 100 }

//  Name:     System.Contact.ConnectedServiceIdentities -- PKEY_Contact_ConnectedServiceIdentities
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {80F41EB8-AFC4-4208-AA5F-CCE21A627281}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_ConnectedServiceIdentities, 0x80F41EB8, 0xAFC4, 0x4208, 0xAA, 0x5F, 0xCC, 0xE2, 0x1A, 0x62, 0x72, 0x81, 100);
#define INIT_PKEY_Contact_ConnectedServiceIdentities { { 0x80F41EB8, 0xAFC4, 0x4208, 0xAA, 0x5F, 0xCC, 0xE2, 0x1A, 0x62, 0x72, 0x81 }, 100 }

//  Name:     System.Contact.ConnectedServiceName -- PKEY_Contact_ConnectedServiceName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B5C84C9E-5927-46B5-A3CC-933C21B78469}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_ConnectedServiceName, 0xB5C84C9E, 0x5927, 0x46B5, 0xA3, 0xCC, 0x93, 0x3C, 0x21, 0xB7, 0x84, 0x69, 100);
#define INIT_PKEY_Contact_ConnectedServiceName { { 0xB5C84C9E, 0x5927, 0x46B5, 0xA3, 0xCC, 0x93, 0x3C, 0x21, 0xB7, 0x84, 0x69 }, 100 }

//  Name:     System.Contact.ConnectedServiceSupportedActions -- PKEY_Contact_ConnectedServiceSupportedActions
//  Type:     UInt32 -- VT_UI4
//  FormatID: {A19FB7A9-024B-4371-A8BF-4D29C3E4E9C9}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_ConnectedServiceSupportedActions, 0xA19FB7A9, 0x024B, 0x4371, 0xA8, 0xBF, 0x4D, 0x29, 0xC3, 0xE4, 0xE9, 0xC9, 100);
#define INIT_PKEY_Contact_ConnectedServiceSupportedActions { { 0xA19FB7A9, 0x024B, 0x4371, 0xA8, 0xBF, 0x4D, 0x29, 0xC3, 0xE4, 0xE9, 0xC9 }, 100 }

//  Name:     System.Contact.DataSuppliers -- PKEY_Contact_DataSuppliers
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {9660C283-FC3A-4A08-A096-EED3AAC46DA2}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_DataSuppliers, 0x9660C283, 0xFC3A, 0x4A08, 0xA0, 0x96, 0xEE, 0xD3, 0xAA, 0xC4, 0x6D, 0xA2, 100);
#define INIT_PKEY_Contact_DataSuppliers { { 0x9660C283, 0xFC3A, 0x4A08, 0xA0, 0x96, 0xEE, 0xD3, 0xAA, 0xC4, 0x6D, 0xA2 }, 100 }

//  Name:     System.Contact.Department -- PKEY_Contact_Department
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FC9F7306-FF8F-4D49-9FB6-3FFE5C0951EC}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_Department, 0xFC9F7306, 0xFF8F, 0x4D49, 0x9F, 0xB6, 0x3F, 0xFE, 0x5C, 0x09, 0x51, 0xEC, 100);
#define INIT_PKEY_Contact_Department { { 0xFC9F7306, 0xFF8F, 0x4D49, 0x9F, 0xB6, 0x3F, 0xFE, 0x5C, 0x09, 0x51, 0xEC }, 100 }

//  Name:     System.Contact.DisplayBusinessPhoneNumbers -- PKEY_Contact_DisplayBusinessPhoneNumbers
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {364028DA-D895-41FE-A584-302B1BB70A76}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_DisplayBusinessPhoneNumbers, 0x364028DA, 0xD895, 0x41FE, 0xA5, 0x84, 0x30, 0x2B, 0x1B, 0xB7, 0x0A, 0x76, 100);
#define INIT_PKEY_Contact_DisplayBusinessPhoneNumbers { { 0x364028DA, 0xD895, 0x41FE, 0xA5, 0x84, 0x30, 0x2B, 0x1B, 0xB7, 0x0A, 0x76 }, 100 }

//  Name:     System.Contact.DisplayHomePhoneNumbers -- PKEY_Contact_DisplayHomePhoneNumbers
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {5068BCDF-D697-4D85-8C53-1F1CDAB01763}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_DisplayHomePhoneNumbers, 0x5068BCDF, 0xD697, 0x4D85, 0x8C, 0x53, 0x1F, 0x1C, 0xDA, 0xB0, 0x17, 0x63, 100);
#define INIT_PKEY_Contact_DisplayHomePhoneNumbers { { 0x5068BCDF, 0xD697, 0x4D85, 0x8C, 0x53, 0x1F, 0x1C, 0xDA, 0xB0, 0x17, 0x63 }, 100 }

//  Name:     System.Contact.DisplayMobilePhoneNumbers -- PKEY_Contact_DisplayMobilePhoneNumbers
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {9CB0C358-9D7A-46B1-B466-DCC6F1A3D93D}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_DisplayMobilePhoneNumbers, 0x9CB0C358, 0x9D7A, 0x46B1, 0xB4, 0x66, 0xDC, 0xC6, 0xF1, 0xA3, 0xD9, 0x3D, 100);
#define INIT_PKEY_Contact_DisplayMobilePhoneNumbers { { 0x9CB0C358, 0x9D7A, 0x46B1, 0xB4, 0x66, 0xDC, 0xC6, 0xF1, 0xA3, 0xD9, 0x3D }, 100 }

//  Name:     System.Contact.DisplayOtherPhoneNumbers -- PKEY_Contact_DisplayOtherPhoneNumbers
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {03089873-8EE8-4191-BD60-D31F72B7900B}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_DisplayOtherPhoneNumbers, 0x03089873, 0x8EE8, 0x4191, 0xBD, 0x60, 0xD3, 0x1F, 0x72, 0xB7, 0x90, 0x0B, 100);
#define INIT_PKEY_Contact_DisplayOtherPhoneNumbers { { 0x03089873, 0x8EE8, 0x4191, 0xBD, 0x60, 0xD3, 0x1F, 0x72, 0xB7, 0x90, 0x0B }, 100 }

//  Name:     System.Contact.EmailAddress -- PKEY_Contact_EmailAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {F8FA7FA3-D12B-4785-8A4E-691A94F7A3E7}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_EmailAddress, 0xF8FA7FA3, 0xD12B, 0x4785, 0x8A, 0x4E, 0x69, 0x1A, 0x94, 0xF7, 0xA3, 0xE7, 100);
#define INIT_PKEY_Contact_EmailAddress { { 0xF8FA7FA3, 0xD12B, 0x4785, 0x8A, 0x4E, 0x69, 0x1A, 0x94, 0xF7, 0xA3, 0xE7 }, 100 }

//  Name:     System.Contact.EmailAddress2 -- PKEY_Contact_EmailAddress2
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {38965063-EDC8-4268-8491-B7723172CF29}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_EmailAddress2, 0x38965063, 0xEDC8, 0x4268, 0x84, 0x91, 0xB7, 0x72, 0x31, 0x72, 0xCF, 0x29, 100);
#define INIT_PKEY_Contact_EmailAddress2 { { 0x38965063, 0xEDC8, 0x4268, 0x84, 0x91, 0xB7, 0x72, 0x31, 0x72, 0xCF, 0x29 }, 100 }

//  Name:     System.Contact.EmailAddress3 -- PKEY_Contact_EmailAddress3
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {644D37B4-E1B3-4BAD-B099-7E7C04966ACA}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_EmailAddress3, 0x644D37B4, 0xE1B3, 0x4BAD, 0xB0, 0x99, 0x7E, 0x7C, 0x04, 0x96, 0x6A, 0xCA, 100);
#define INIT_PKEY_Contact_EmailAddress3 { { 0x644D37B4, 0xE1B3, 0x4BAD, 0xB0, 0x99, 0x7E, 0x7C, 0x04, 0x96, 0x6A, 0xCA }, 100 }

//  Name:     System.Contact.EmailAddresses -- PKEY_Contact_EmailAddresses
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {84D8F337-981D-44B3-9615-C7596DBA17E3}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_EmailAddresses, 0x84D8F337, 0x981D, 0x44B3, 0x96, 0x15, 0xC7, 0x59, 0x6D, 0xBA, 0x17, 0xE3, 100);
#define INIT_PKEY_Contact_EmailAddresses { { 0x84D8F337, 0x981D, 0x44B3, 0x96, 0x15, 0xC7, 0x59, 0x6D, 0xBA, 0x17, 0xE3 }, 100 }

//  Name:     System.Contact.EmailName -- PKEY_Contact_EmailName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {CC6F4F24-6083-4BD4-8754-674D0DE87AB8}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_EmailName, 0xCC6F4F24, 0x6083, 0x4BD4, 0x87, 0x54, 0x67, 0x4D, 0x0D, 0xE8, 0x7A, 0xB8, 100);
#define INIT_PKEY_Contact_EmailName { { 0xCC6F4F24, 0x6083, 0x4BD4, 0x87, 0x54, 0x67, 0x4D, 0x0D, 0xE8, 0x7A, 0xB8 }, 100 }

//  Name:     System.Contact.FileAsName -- PKEY_Contact_FileAsName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {F1A24AA7-9CA7-40F6-89EC-97DEF9FFE8DB}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_FileAsName, 0xF1A24AA7, 0x9CA7, 0x40F6, 0x89, 0xEC, 0x97, 0xDE, 0xF9, 0xFF, 0xE8, 0xDB, 100);
#define INIT_PKEY_Contact_FileAsName { { 0xF1A24AA7, 0x9CA7, 0x40F6, 0x89, 0xEC, 0x97, 0xDE, 0xF9, 0xFF, 0xE8, 0xDB }, 100 }

//  Name:     System.Contact.FirstName -- PKEY_Contact_FirstName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {14977844-6B49-4AAD-A714-A4513BF60460}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_FirstName, 0x14977844, 0x6B49, 0x4AAD, 0xA7, 0x14, 0xA4, 0x51, 0x3B, 0xF6, 0x04, 0x60, 100);
#define INIT_PKEY_Contact_FirstName { { 0x14977844, 0x6B49, 0x4AAD, 0xA7, 0x14, 0xA4, 0x51, 0x3B, 0xF6, 0x04, 0x60 }, 100 }

//  Name:     System.Contact.FullName -- PKEY_Contact_FullName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {635E9051-50A5-4BA2-B9DB-4ED056C77296}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_FullName, 0x635E9051, 0x50A5, 0x4BA2, 0xB9, 0xDB, 0x4E, 0xD0, 0x56, 0xC7, 0x72, 0x96, 100);
#define INIT_PKEY_Contact_FullName { { 0x635E9051, 0x50A5, 0x4BA2, 0xB9, 0xDB, 0x4E, 0xD0, 0x56, 0xC7, 0x72, 0x96 }, 100 }

//  Name:     System.Contact.Gender -- PKEY_Contact_Gender
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {3C8CEE58-D4F0-4CF9-B756-4E5D24447BCD}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_Gender, 0x3C8CEE58, 0xD4F0, 0x4CF9, 0xB7, 0x56, 0x4E, 0x5D, 0x24, 0x44, 0x7B, 0xCD, 100);
#define INIT_PKEY_Contact_Gender { { 0x3C8CEE58, 0xD4F0, 0x4CF9, 0xB7, 0x56, 0x4E, 0x5D, 0x24, 0x44, 0x7B, 0xCD }, 100 }

//  Name:     System.Contact.GenderValue -- PKEY_Contact_GenderValue
//  Type:     UInt16 -- VT_UI2
//  FormatID: {3C8CEE58-D4F0-4CF9-B756-4E5D24447BCD}, 101
DEFINE_PROPERTYKEY(PKEY_Contact_GenderValue, 0x3C8CEE58, 0xD4F0, 0x4CF9, 0xB7, 0x56, 0x4E, 0x5D, 0x24, 0x44, 0x7B, 0xCD, 101);
#define INIT_PKEY_Contact_GenderValue { { 0x3C8CEE58, 0xD4F0, 0x4CF9, 0xB7, 0x56, 0x4E, 0x5D, 0x24, 0x44, 0x7B, 0xCD }, 101 }

// Possible discrete values for PKEY_Contact_GenderValue are:
#define GENDERVALUE_UNSPECIFIED             0u
#define GENDERVALUE_FEMALE                  1u
#define GENDERVALUE_MALE                    2u

//  Name:     System.Contact.Hobbies -- PKEY_Contact_Hobbies
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {5DC2253F-5E11-4ADF-9CFE-910DD01E3E70}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_Hobbies, 0x5DC2253F, 0x5E11, 0x4ADF, 0x9C, 0xFE, 0x91, 0x0D, 0xD0, 0x1E, 0x3E, 0x70, 100);
#define INIT_PKEY_Contact_Hobbies { { 0x5DC2253F, 0x5E11, 0x4ADF, 0x9C, 0xFE, 0x91, 0x0D, 0xD0, 0x1E, 0x3E, 0x70 }, 100 }

//  Name:     System.Contact.HomeAddress -- PKEY_Contact_HomeAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {98F98354-617A-46B8-8560-5B1B64BF1F89}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress, 0x98F98354, 0x617A, 0x46B8, 0x85, 0x60, 0x5B, 0x1B, 0x64, 0xBF, 0x1F, 0x89, 100);
#define INIT_PKEY_Contact_HomeAddress { { 0x98F98354, 0x617A, 0x46B8, 0x85, 0x60, 0x5B, 0x1B, 0x64, 0xBF, 0x1F, 0x89 }, 100 }

//  Name:     System.Contact.HomeAddress1Country -- PKEY_Contact_HomeAddress1Country
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 104
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress1Country, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 104);
#define INIT_PKEY_Contact_HomeAddress1Country { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 104 }

//  Name:     System.Contact.HomeAddress1Locality -- PKEY_Contact_HomeAddress1Locality
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 102
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress1Locality, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 102);
#define INIT_PKEY_Contact_HomeAddress1Locality { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 102 }

//  Name:     System.Contact.HomeAddress1PostalCode -- PKEY_Contact_HomeAddress1PostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 105
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress1PostalCode, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 105);
#define INIT_PKEY_Contact_HomeAddress1PostalCode { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 105 }

//  Name:     System.Contact.HomeAddress1Region -- PKEY_Contact_HomeAddress1Region
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 103
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress1Region, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 103);
#define INIT_PKEY_Contact_HomeAddress1Region { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 103 }

//  Name:     System.Contact.HomeAddress1Street -- PKEY_Contact_HomeAddress1Street
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 101
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress1Street, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 101);
#define INIT_PKEY_Contact_HomeAddress1Street { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 101 }

//  Name:     System.Contact.HomeAddress2Country -- PKEY_Contact_HomeAddress2Country
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 109
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress2Country, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 109);
#define INIT_PKEY_Contact_HomeAddress2Country { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 109 }

//  Name:     System.Contact.HomeAddress2Locality -- PKEY_Contact_HomeAddress2Locality
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 107
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress2Locality, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 107);
#define INIT_PKEY_Contact_HomeAddress2Locality { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 107 }

//  Name:     System.Contact.HomeAddress2PostalCode -- PKEY_Contact_HomeAddress2PostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 110
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress2PostalCode, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 110);
#define INIT_PKEY_Contact_HomeAddress2PostalCode { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 110 }

//  Name:     System.Contact.HomeAddress2Region -- PKEY_Contact_HomeAddress2Region
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 108
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress2Region, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 108);
#define INIT_PKEY_Contact_HomeAddress2Region { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 108 }

//  Name:     System.Contact.HomeAddress2Street -- PKEY_Contact_HomeAddress2Street
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 106
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress2Street, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 106);
#define INIT_PKEY_Contact_HomeAddress2Street { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 106 }

//  Name:     System.Contact.HomeAddress3Country -- PKEY_Contact_HomeAddress3Country
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 114
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress3Country, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 114);
#define INIT_PKEY_Contact_HomeAddress3Country { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 114 }

//  Name:     System.Contact.HomeAddress3Locality -- PKEY_Contact_HomeAddress3Locality
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 112
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress3Locality, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 112);
#define INIT_PKEY_Contact_HomeAddress3Locality { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 112 }

//  Name:     System.Contact.HomeAddress3PostalCode -- PKEY_Contact_HomeAddress3PostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 115
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress3PostalCode, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 115);
#define INIT_PKEY_Contact_HomeAddress3PostalCode { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 115 }

//  Name:     System.Contact.HomeAddress3Region -- PKEY_Contact_HomeAddress3Region
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 113
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress3Region, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 113);
#define INIT_PKEY_Contact_HomeAddress3Region { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 113 }

//  Name:     System.Contact.HomeAddress3Street -- PKEY_Contact_HomeAddress3Street
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 111
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddress3Street, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 111);
#define INIT_PKEY_Contact_HomeAddress3Street { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 111 }

//  Name:     System.Contact.HomeAddressCity -- PKEY_Contact_HomeAddressCity
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 65
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddressCity, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 65);
#define INIT_PKEY_Contact_HomeAddressCity { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 65 }

//  Name:     System.Contact.HomeAddressCountry -- PKEY_Contact_HomeAddressCountry
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {08A65AA1-F4C9-43DD-9DDF-A33D8E7EAD85}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddressCountry, 0x08A65AA1, 0xF4C9, 0x43DD, 0x9D, 0xDF, 0xA3, 0x3D, 0x8E, 0x7E, 0xAD, 0x85, 100);
#define INIT_PKEY_Contact_HomeAddressCountry { { 0x08A65AA1, 0xF4C9, 0x43DD, 0x9D, 0xDF, 0xA3, 0x3D, 0x8E, 0x7E, 0xAD, 0x85 }, 100 }

//  Name:     System.Contact.HomeAddressPostalCode -- PKEY_Contact_HomeAddressPostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8AFCC170-8A46-4B53-9EEE-90BAE7151E62}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddressPostalCode, 0x8AFCC170, 0x8A46, 0x4B53, 0x9E, 0xEE, 0x90, 0xBA, 0xE7, 0x15, 0x1E, 0x62, 100);
#define INIT_PKEY_Contact_HomeAddressPostalCode { { 0x8AFCC170, 0x8A46, 0x4B53, 0x9E, 0xEE, 0x90, 0xBA, 0xE7, 0x15, 0x1E, 0x62 }, 100 }

//  Name:     System.Contact.HomeAddressPostOfficeBox -- PKEY_Contact_HomeAddressPostOfficeBox
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {7B9F6399-0A3F-4B12-89BD-4ADC51C918AF}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddressPostOfficeBox, 0x7B9F6399, 0x0A3F, 0x4B12, 0x89, 0xBD, 0x4A, 0xDC, 0x51, 0xC9, 0x18, 0xAF, 100);
#define INIT_PKEY_Contact_HomeAddressPostOfficeBox { { 0x7B9F6399, 0x0A3F, 0x4B12, 0x89, 0xBD, 0x4A, 0xDC, 0x51, 0xC9, 0x18, 0xAF }, 100 }

//  Name:     System.Contact.HomeAddressState -- PKEY_Contact_HomeAddressState
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C89A23D0-7D6D-4EB8-87D4-776A82D493E5}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddressState, 0xC89A23D0, 0x7D6D, 0x4EB8, 0x87, 0xD4, 0x77, 0x6A, 0x82, 0xD4, 0x93, 0xE5, 100);
#define INIT_PKEY_Contact_HomeAddressState { { 0xC89A23D0, 0x7D6D, 0x4EB8, 0x87, 0xD4, 0x77, 0x6A, 0x82, 0xD4, 0x93, 0xE5 }, 100 }

//  Name:     System.Contact.HomeAddressStreet -- PKEY_Contact_HomeAddressStreet
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {0ADEF160-DB3F-4308-9A21-06237B16FA2A}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_HomeAddressStreet, 0x0ADEF160, 0xDB3F, 0x4308, 0x9A, 0x21, 0x06, 0x23, 0x7B, 0x16, 0xFA, 0x2A, 100);
#define INIT_PKEY_Contact_HomeAddressStreet { { 0x0ADEF160, 0xDB3F, 0x4308, 0x9A, 0x21, 0x06, 0x23, 0x7B, 0x16, 0xFA, 0x2A }, 100 }

//  Name:     System.Contact.HomeEmailAddresses -- PKEY_Contact_HomeEmailAddresses
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {56C90E9D-9D46-4963-886F-2E1CD9A694EF}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_HomeEmailAddresses, 0x56C90E9D, 0x9D46, 0x4963, 0x88, 0x6F, 0x2E, 0x1C, 0xD9, 0xA6, 0x94, 0xEF, 100);
#define INIT_PKEY_Contact_HomeEmailAddresses { { 0x56C90E9D, 0x9D46, 0x4963, 0x88, 0x6F, 0x2E, 0x1C, 0xD9, 0xA6, 0x94, 0xEF }, 100 }

//  Name:     System.Contact.HomeFaxNumber -- PKEY_Contact_HomeFaxNumber
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {660E04D6-81AB-4977-A09F-82313113AB26}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_HomeFaxNumber, 0x660E04D6, 0x81AB, 0x4977, 0xA0, 0x9F, 0x82, 0x31, 0x31, 0x13, 0xAB, 0x26, 100);
#define INIT_PKEY_Contact_HomeFaxNumber { { 0x660E04D6, 0x81AB, 0x4977, 0xA0, 0x9F, 0x82, 0x31, 0x31, 0x13, 0xAB, 0x26 }, 100 }

//  Name:     System.Contact.HomeTelephone -- PKEY_Contact_HomeTelephone
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 20
DEFINE_PROPERTYKEY(PKEY_Contact_HomeTelephone, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 20);
#define INIT_PKEY_Contact_HomeTelephone { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 20 }

//  Name:     System.Contact.IMAddress -- PKEY_Contact_IMAddress
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D68DBD8A-3374-4B81-9972-3EC30682DB3D}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_IMAddress, 0xD68DBD8A, 0x3374, 0x4B81, 0x99, 0x72, 0x3E, 0xC3, 0x06, 0x82, 0xDB, 0x3D, 100);
#define INIT_PKEY_Contact_IMAddress { { 0xD68DBD8A, 0x3374, 0x4B81, 0x99, 0x72, 0x3E, 0xC3, 0x06, 0x82, 0xDB, 0x3D }, 100 }

//  Name:     System.Contact.Initials -- PKEY_Contact_Initials
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {F3D8F40D-50CB-44A2-9718-40CB9119495D}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_Initials, 0xF3D8F40D, 0x50CB, 0x44A2, 0x97, 0x18, 0x40, 0xCB, 0x91, 0x19, 0x49, 0x5D, 100);
#define INIT_PKEY_Contact_Initials { { 0xF3D8F40D, 0x50CB, 0x44A2, 0x97, 0x18, 0x40, 0xCB, 0x91, 0x19, 0x49, 0x5D }, 100 }

//  Name:     System.Contact.JA.CompanyNamePhonetic -- PKEY_Contact_JA_CompanyNamePhonetic
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {897B3694-FE9E-43E6-8066-260F590C0100}, 2
//  
//  
DEFINE_PROPERTYKEY(PKEY_Contact_JA_CompanyNamePhonetic, 0x897B3694, 0xFE9E, 0x43E6, 0x80, 0x66, 0x26, 0x0F, 0x59, 0x0C, 0x01, 0x00, 2);
#define INIT_PKEY_Contact_JA_CompanyNamePhonetic { { 0x897B3694, 0xFE9E, 0x43E6, 0x80, 0x66, 0x26, 0x0F, 0x59, 0x0C, 0x01, 0x00 }, 2 }

//  Name:     System.Contact.JA.FirstNamePhonetic -- PKEY_Contact_JA_FirstNamePhonetic
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {897B3694-FE9E-43E6-8066-260F590C0100}, 3
//  
//  
DEFINE_PROPERTYKEY(PKEY_Contact_JA_FirstNamePhonetic, 0x897B3694, 0xFE9E, 0x43E6, 0x80, 0x66, 0x26, 0x0F, 0x59, 0x0C, 0x01, 0x00, 3);
#define INIT_PKEY_Contact_JA_FirstNamePhonetic { { 0x897B3694, 0xFE9E, 0x43E6, 0x80, 0x66, 0x26, 0x0F, 0x59, 0x0C, 0x01, 0x00 }, 3 }

//  Name:     System.Contact.JA.LastNamePhonetic -- PKEY_Contact_JA_LastNamePhonetic
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {897B3694-FE9E-43E6-8066-260F590C0100}, 4
//  
//  
DEFINE_PROPERTYKEY(PKEY_Contact_JA_LastNamePhonetic, 0x897B3694, 0xFE9E, 0x43E6, 0x80, 0x66, 0x26, 0x0F, 0x59, 0x0C, 0x01, 0x00, 4);
#define INIT_PKEY_Contact_JA_LastNamePhonetic { { 0x897B3694, 0xFE9E, 0x43E6, 0x80, 0x66, 0x26, 0x0F, 0x59, 0x0C, 0x01, 0x00 }, 4 }

//  Name:     System.Contact.JobInfo1CompanyAddress -- PKEY_Contact_JobInfo1CompanyAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 120
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo1CompanyAddress, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 120);
#define INIT_PKEY_Contact_JobInfo1CompanyAddress { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 120 }

//  Name:     System.Contact.JobInfo1CompanyName -- PKEY_Contact_JobInfo1CompanyName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 102
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo1CompanyName, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 102);
#define INIT_PKEY_Contact_JobInfo1CompanyName { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 102 }

//  Name:     System.Contact.JobInfo1Department -- PKEY_Contact_JobInfo1Department
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 106
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo1Department, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 106);
#define INIT_PKEY_Contact_JobInfo1Department { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 106 }

//  Name:     System.Contact.JobInfo1Manager -- PKEY_Contact_JobInfo1Manager
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 105
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo1Manager, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 105);
#define INIT_PKEY_Contact_JobInfo1Manager { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 105 }

//  Name:     System.Contact.JobInfo1OfficeLocation -- PKEY_Contact_JobInfo1OfficeLocation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 104
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo1OfficeLocation, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 104);
#define INIT_PKEY_Contact_JobInfo1OfficeLocation { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 104 }

//  Name:     System.Contact.JobInfo1Title -- PKEY_Contact_JobInfo1Title
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 103
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo1Title, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 103);
#define INIT_PKEY_Contact_JobInfo1Title { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 103 }

//  Name:     System.Contact.JobInfo1YomiCompanyName -- PKEY_Contact_JobInfo1YomiCompanyName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 101
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo1YomiCompanyName, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 101);
#define INIT_PKEY_Contact_JobInfo1YomiCompanyName { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 101 }

//  Name:     System.Contact.JobInfo2CompanyAddress -- PKEY_Contact_JobInfo2CompanyAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 121
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo2CompanyAddress, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 121);
#define INIT_PKEY_Contact_JobInfo2CompanyAddress { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 121 }

//  Name:     System.Contact.JobInfo2CompanyName -- PKEY_Contact_JobInfo2CompanyName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 108
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo2CompanyName, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 108);
#define INIT_PKEY_Contact_JobInfo2CompanyName { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 108 }

//  Name:     System.Contact.JobInfo2Department -- PKEY_Contact_JobInfo2Department
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 113
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo2Department, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 113);
#define INIT_PKEY_Contact_JobInfo2Department { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 113 }

//  Name:     System.Contact.JobInfo2Manager -- PKEY_Contact_JobInfo2Manager
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 112
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo2Manager, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 112);
#define INIT_PKEY_Contact_JobInfo2Manager { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 112 }

//  Name:     System.Contact.JobInfo2OfficeLocation -- PKEY_Contact_JobInfo2OfficeLocation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 110
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo2OfficeLocation, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 110);
#define INIT_PKEY_Contact_JobInfo2OfficeLocation { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 110 }

//  Name:     System.Contact.JobInfo2Title -- PKEY_Contact_JobInfo2Title
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 109
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo2Title, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 109);
#define INIT_PKEY_Contact_JobInfo2Title { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 109 }

//  Name:     System.Contact.JobInfo2YomiCompanyName -- PKEY_Contact_JobInfo2YomiCompanyName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 107
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo2YomiCompanyName, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 107);
#define INIT_PKEY_Contact_JobInfo2YomiCompanyName { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 107 }

//  Name:     System.Contact.JobInfo3CompanyAddress -- PKEY_Contact_JobInfo3CompanyAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 123
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo3CompanyAddress, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 123);
#define INIT_PKEY_Contact_JobInfo3CompanyAddress { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 123 }

//  Name:     System.Contact.JobInfo3CompanyName -- PKEY_Contact_JobInfo3CompanyName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 115
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo3CompanyName, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 115);
#define INIT_PKEY_Contact_JobInfo3CompanyName { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 115 }

//  Name:     System.Contact.JobInfo3Department -- PKEY_Contact_JobInfo3Department
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 119
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo3Department, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 119);
#define INIT_PKEY_Contact_JobInfo3Department { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 119 }

//  Name:     System.Contact.JobInfo3Manager -- PKEY_Contact_JobInfo3Manager
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 118
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo3Manager, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 118);
#define INIT_PKEY_Contact_JobInfo3Manager { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 118 }

//  Name:     System.Contact.JobInfo3OfficeLocation -- PKEY_Contact_JobInfo3OfficeLocation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 117
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo3OfficeLocation, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 117);
#define INIT_PKEY_Contact_JobInfo3OfficeLocation { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 117 }

//  Name:     System.Contact.JobInfo3Title -- PKEY_Contact_JobInfo3Title
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 116
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo3Title, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 116);
#define INIT_PKEY_Contact_JobInfo3Title { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 116 }

//  Name:     System.Contact.JobInfo3YomiCompanyName -- PKEY_Contact_JobInfo3YomiCompanyName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 114
DEFINE_PROPERTYKEY(PKEY_Contact_JobInfo3YomiCompanyName, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 114);
#define INIT_PKEY_Contact_JobInfo3YomiCompanyName { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 114 }

//  Name:     System.Contact.JobTitle -- PKEY_Contact_JobTitle
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 6
DEFINE_PROPERTYKEY(PKEY_Contact_JobTitle, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 6);
#define INIT_PKEY_Contact_JobTitle { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 6 }

//  Name:     System.Contact.Label -- PKEY_Contact_Label
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {97B0AD89-DF49-49CC-834E-660974FD755B}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_Label, 0x97B0AD89, 0xDF49, 0x49CC, 0x83, 0x4E, 0x66, 0x09, 0x74, 0xFD, 0x75, 0x5B, 100);
#define INIT_PKEY_Contact_Label { { 0x97B0AD89, 0xDF49, 0x49CC, 0x83, 0x4E, 0x66, 0x09, 0x74, 0xFD, 0x75, 0x5B }, 100 }

//  Name:     System.Contact.LastName -- PKEY_Contact_LastName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8F367200-C270-457C-B1D4-E07C5BCD90C7}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_LastName, 0x8F367200, 0xC270, 0x457C, 0xB1, 0xD4, 0xE0, 0x7C, 0x5B, 0xCD, 0x90, 0xC7, 100);
#define INIT_PKEY_Contact_LastName { { 0x8F367200, 0xC270, 0x457C, 0xB1, 0xD4, 0xE0, 0x7C, 0x5B, 0xCD, 0x90, 0xC7 }, 100 }

//  Name:     System.Contact.MailingAddress -- PKEY_Contact_MailingAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C0AC206A-827E-4650-95AE-77E2BB74FCC9}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_MailingAddress, 0xC0AC206A, 0x827E, 0x4650, 0x95, 0xAE, 0x77, 0xE2, 0xBB, 0x74, 0xFC, 0xC9, 100);
#define INIT_PKEY_Contact_MailingAddress { { 0xC0AC206A, 0x827E, 0x4650, 0x95, 0xAE, 0x77, 0xE2, 0xBB, 0x74, 0xFC, 0xC9 }, 100 }

//  Name:     System.Contact.MiddleName -- PKEY_Contact_MiddleName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 71
DEFINE_PROPERTYKEY(PKEY_Contact_MiddleName, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 71);
#define INIT_PKEY_Contact_MiddleName { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 71 }

//  Name:     System.Contact.MobileTelephone -- PKEY_Contact_MobileTelephone
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 35
DEFINE_PROPERTYKEY(PKEY_Contact_MobileTelephone, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 35);
#define INIT_PKEY_Contact_MobileTelephone { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 35 }

//  Name:     System.Contact.NickName -- PKEY_Contact_NickName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 74
DEFINE_PROPERTYKEY(PKEY_Contact_NickName, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 74);
#define INIT_PKEY_Contact_NickName { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 74 }

//  Name:     System.Contact.OfficeLocation -- PKEY_Contact_OfficeLocation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 7
DEFINE_PROPERTYKEY(PKEY_Contact_OfficeLocation, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 7);
#define INIT_PKEY_Contact_OfficeLocation { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 7 }

//  Name:     System.Contact.OtherAddress -- PKEY_Contact_OtherAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {508161FA-313B-43D5-83A1-C1ACCF68622C}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress, 0x508161FA, 0x313B, 0x43D5, 0x83, 0xA1, 0xC1, 0xAC, 0xCF, 0x68, 0x62, 0x2C, 100);
#define INIT_PKEY_Contact_OtherAddress { { 0x508161FA, 0x313B, 0x43D5, 0x83, 0xA1, 0xC1, 0xAC, 0xCF, 0x68, 0x62, 0x2C }, 100 }

//  Name:     System.Contact.OtherAddress1Country -- PKEY_Contact_OtherAddress1Country
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 134
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress1Country, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 134);
#define INIT_PKEY_Contact_OtherAddress1Country { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 134 }

//  Name:     System.Contact.OtherAddress1Locality -- PKEY_Contact_OtherAddress1Locality
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 132
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress1Locality, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 132);
#define INIT_PKEY_Contact_OtherAddress1Locality { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 132 }

//  Name:     System.Contact.OtherAddress1PostalCode -- PKEY_Contact_OtherAddress1PostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 135
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress1PostalCode, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 135);
#define INIT_PKEY_Contact_OtherAddress1PostalCode { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 135 }

//  Name:     System.Contact.OtherAddress1Region -- PKEY_Contact_OtherAddress1Region
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 133
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress1Region, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 133);
#define INIT_PKEY_Contact_OtherAddress1Region { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 133 }

//  Name:     System.Contact.OtherAddress1Street -- PKEY_Contact_OtherAddress1Street
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 131
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress1Street, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 131);
#define INIT_PKEY_Contact_OtherAddress1Street { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 131 }

//  Name:     System.Contact.OtherAddress2Country -- PKEY_Contact_OtherAddress2Country
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 139
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress2Country, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 139);
#define INIT_PKEY_Contact_OtherAddress2Country { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 139 }

//  Name:     System.Contact.OtherAddress2Locality -- PKEY_Contact_OtherAddress2Locality
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 137
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress2Locality, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 137);
#define INIT_PKEY_Contact_OtherAddress2Locality { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 137 }

//  Name:     System.Contact.OtherAddress2PostalCode -- PKEY_Contact_OtherAddress2PostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 140
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress2PostalCode, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 140);
#define INIT_PKEY_Contact_OtherAddress2PostalCode { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 140 }

//  Name:     System.Contact.OtherAddress2Region -- PKEY_Contact_OtherAddress2Region
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 138
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress2Region, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 138);
#define INIT_PKEY_Contact_OtherAddress2Region { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 138 }

//  Name:     System.Contact.OtherAddress2Street -- PKEY_Contact_OtherAddress2Street
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 136
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress2Street, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 136);
#define INIT_PKEY_Contact_OtherAddress2Street { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 136 }

//  Name:     System.Contact.OtherAddress3Country -- PKEY_Contact_OtherAddress3Country
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 144
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress3Country, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 144);
#define INIT_PKEY_Contact_OtherAddress3Country { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 144 }

//  Name:     System.Contact.OtherAddress3Locality -- PKEY_Contact_OtherAddress3Locality
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 142
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress3Locality, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 142);
#define INIT_PKEY_Contact_OtherAddress3Locality { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 142 }

//  Name:     System.Contact.OtherAddress3PostalCode -- PKEY_Contact_OtherAddress3PostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 145
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress3PostalCode, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 145);
#define INIT_PKEY_Contact_OtherAddress3PostalCode { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 145 }

//  Name:     System.Contact.OtherAddress3Region -- PKEY_Contact_OtherAddress3Region
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 143
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress3Region, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 143);
#define INIT_PKEY_Contact_OtherAddress3Region { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 143 }

//  Name:     System.Contact.OtherAddress3Street -- PKEY_Contact_OtherAddress3Street
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7B6F596-D678-4BC1-B05F-0203D27E8AA1}, 141
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddress3Street, 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1, 141);
#define INIT_PKEY_Contact_OtherAddress3Street { { 0xA7B6F596, 0xD678, 0x4BC1, 0xB0, 0x5F, 0x02, 0x03, 0xD2, 0x7E, 0x8A, 0xA1 }, 141 }

//  Name:     System.Contact.OtherAddressCity -- PKEY_Contact_OtherAddressCity
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {6E682923-7F7B-4F0C-A337-CFCA296687BF}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddressCity, 0x6E682923, 0x7F7B, 0x4F0C, 0xA3, 0x37, 0xCF, 0xCA, 0x29, 0x66, 0x87, 0xBF, 100);
#define INIT_PKEY_Contact_OtherAddressCity { { 0x6E682923, 0x7F7B, 0x4F0C, 0xA3, 0x37, 0xCF, 0xCA, 0x29, 0x66, 0x87, 0xBF }, 100 }

//  Name:     System.Contact.OtherAddressCountry -- PKEY_Contact_OtherAddressCountry
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8F167568-0AAE-4322-8ED9-6055B7B0E398}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddressCountry, 0x8F167568, 0x0AAE, 0x4322, 0x8E, 0xD9, 0x60, 0x55, 0xB7, 0xB0, 0xE3, 0x98, 100);
#define INIT_PKEY_Contact_OtherAddressCountry { { 0x8F167568, 0x0AAE, 0x4322, 0x8E, 0xD9, 0x60, 0x55, 0xB7, 0xB0, 0xE3, 0x98 }, 100 }

//  Name:     System.Contact.OtherAddressPostalCode -- PKEY_Contact_OtherAddressPostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {95C656C1-2ABF-4148-9ED3-9EC602E3B7CD}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddressPostalCode, 0x95C656C1, 0x2ABF, 0x4148, 0x9E, 0xD3, 0x9E, 0xC6, 0x02, 0xE3, 0xB7, 0xCD, 100);
#define INIT_PKEY_Contact_OtherAddressPostalCode { { 0x95C656C1, 0x2ABF, 0x4148, 0x9E, 0xD3, 0x9E, 0xC6, 0x02, 0xE3, 0xB7, 0xCD }, 100 }

//  Name:     System.Contact.OtherAddressPostOfficeBox -- PKEY_Contact_OtherAddressPostOfficeBox
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8B26EA41-058F-43F6-AECC-4035681CE977}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddressPostOfficeBox, 0x8B26EA41, 0x058F, 0x43F6, 0xAE, 0xCC, 0x40, 0x35, 0x68, 0x1C, 0xE9, 0x77, 100);
#define INIT_PKEY_Contact_OtherAddressPostOfficeBox { { 0x8B26EA41, 0x058F, 0x43F6, 0xAE, 0xCC, 0x40, 0x35, 0x68, 0x1C, 0xE9, 0x77 }, 100 }

//  Name:     System.Contact.OtherAddressState -- PKEY_Contact_OtherAddressState
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {71B377D6-E570-425F-A170-809FAE73E54E}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddressState, 0x71B377D6, 0xE570, 0x425F, 0xA1, 0x70, 0x80, 0x9F, 0xAE, 0x73, 0xE5, 0x4E, 100);
#define INIT_PKEY_Contact_OtherAddressState { { 0x71B377D6, 0xE570, 0x425F, 0xA1, 0x70, 0x80, 0x9F, 0xAE, 0x73, 0xE5, 0x4E }, 100 }

//  Name:     System.Contact.OtherAddressStreet -- PKEY_Contact_OtherAddressStreet
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FF962609-B7D6-4999-862D-95180D529AEA}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_OtherAddressStreet, 0xFF962609, 0xB7D6, 0x4999, 0x86, 0x2D, 0x95, 0x18, 0x0D, 0x52, 0x9A, 0xEA, 100);
#define INIT_PKEY_Contact_OtherAddressStreet { { 0xFF962609, 0xB7D6, 0x4999, 0x86, 0x2D, 0x95, 0x18, 0x0D, 0x52, 0x9A, 0xEA }, 100 }

//  Name:     System.Contact.OtherEmailAddresses -- PKEY_Contact_OtherEmailAddresses
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {11D6336B-38C4-4EC9-84D6-EB38D0B150AF}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_OtherEmailAddresses, 0x11D6336B, 0x38C4, 0x4EC9, 0x84, 0xD6, 0xEB, 0x38, 0xD0, 0xB1, 0x50, 0xAF, 100);
#define INIT_PKEY_Contact_OtherEmailAddresses { { 0x11D6336B, 0x38C4, 0x4EC9, 0x84, 0xD6, 0xEB, 0x38, 0xD0, 0xB1, 0x50, 0xAF }, 100 }

//  Name:     System.Contact.PagerTelephone -- PKEY_Contact_PagerTelephone
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {D6304E01-F8F5-4F45-8B15-D024A6296789}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_PagerTelephone, 0xD6304E01, 0xF8F5, 0x4F45, 0x8B, 0x15, 0xD0, 0x24, 0xA6, 0x29, 0x67, 0x89, 100);
#define INIT_PKEY_Contact_PagerTelephone { { 0xD6304E01, 0xF8F5, 0x4F45, 0x8B, 0x15, 0xD0, 0x24, 0xA6, 0x29, 0x67, 0x89 }, 100 }

//  Name:     System.Contact.PersonalTitle -- PKEY_Contact_PersonalTitle
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 69
DEFINE_PROPERTYKEY(PKEY_Contact_PersonalTitle, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 69);
#define INIT_PKEY_Contact_PersonalTitle { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 69 }

//  Name:     System.Contact.PhoneNumbersCanonical -- PKEY_Contact_PhoneNumbersCanonical
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D042D2A1-927E-40B5-A503-6EDBD42A517E}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_PhoneNumbersCanonical, 0xD042D2A1, 0x927E, 0x40B5, 0xA5, 0x03, 0x6E, 0xDB, 0xD4, 0x2A, 0x51, 0x7E, 100);
#define INIT_PKEY_Contact_PhoneNumbersCanonical { { 0xD042D2A1, 0x927E, 0x40B5, 0xA5, 0x03, 0x6E, 0xDB, 0xD4, 0x2A, 0x51, 0x7E }, 100 }

//  Name:     System.Contact.Prefix -- PKEY_Contact_Prefix
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 75
DEFINE_PROPERTYKEY(PKEY_Contact_Prefix, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 75);
#define INIT_PKEY_Contact_Prefix { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 75 }

//  Name:     System.Contact.PrimaryAddressCity -- PKEY_Contact_PrimaryAddressCity
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C8EA94F0-A9E3-4969-A94B-9C62A95324E0}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_PrimaryAddressCity, 0xC8EA94F0, 0xA9E3, 0x4969, 0xA9, 0x4B, 0x9C, 0x62, 0xA9, 0x53, 0x24, 0xE0, 100);
#define INIT_PKEY_Contact_PrimaryAddressCity { { 0xC8EA94F0, 0xA9E3, 0x4969, 0xA9, 0x4B, 0x9C, 0x62, 0xA9, 0x53, 0x24, 0xE0 }, 100 }

//  Name:     System.Contact.PrimaryAddressCountry -- PKEY_Contact_PrimaryAddressCountry
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E53D799D-0F3F-466E-B2FF-74634A3CB7A4}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_PrimaryAddressCountry, 0xE53D799D, 0x0F3F, 0x466E, 0xB2, 0xFF, 0x74, 0x63, 0x4A, 0x3C, 0xB7, 0xA4, 100);
#define INIT_PKEY_Contact_PrimaryAddressCountry { { 0xE53D799D, 0x0F3F, 0x466E, 0xB2, 0xFF, 0x74, 0x63, 0x4A, 0x3C, 0xB7, 0xA4 }, 100 }

//  Name:     System.Contact.PrimaryAddressPostalCode -- PKEY_Contact_PrimaryAddressPostalCode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {18BBD425-ECFD-46EF-B612-7B4A6034EDA0}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_PrimaryAddressPostalCode, 0x18BBD425, 0xECFD, 0x46EF, 0xB6, 0x12, 0x7B, 0x4A, 0x60, 0x34, 0xED, 0xA0, 100);
#define INIT_PKEY_Contact_PrimaryAddressPostalCode { { 0x18BBD425, 0xECFD, 0x46EF, 0xB6, 0x12, 0x7B, 0x4A, 0x60, 0x34, 0xED, 0xA0 }, 100 }

//  Name:     System.Contact.PrimaryAddressPostOfficeBox -- PKEY_Contact_PrimaryAddressPostOfficeBox
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {DE5EF3C7-46E1-484E-9999-62C5308394C1}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_PrimaryAddressPostOfficeBox, 0xDE5EF3C7, 0x46E1, 0x484E, 0x99, 0x99, 0x62, 0xC5, 0x30, 0x83, 0x94, 0xC1, 100);
#define INIT_PKEY_Contact_PrimaryAddressPostOfficeBox { { 0xDE5EF3C7, 0x46E1, 0x484E, 0x99, 0x99, 0x62, 0xC5, 0x30, 0x83, 0x94, 0xC1 }, 100 }

//  Name:     System.Contact.PrimaryAddressState -- PKEY_Contact_PrimaryAddressState
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {F1176DFE-7138-4640-8B4C-AE375DC70A6D}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_PrimaryAddressState, 0xF1176DFE, 0x7138, 0x4640, 0x8B, 0x4C, 0xAE, 0x37, 0x5D, 0xC7, 0x0A, 0x6D, 100);
#define INIT_PKEY_Contact_PrimaryAddressState { { 0xF1176DFE, 0x7138, 0x4640, 0x8B, 0x4C, 0xAE, 0x37, 0x5D, 0xC7, 0x0A, 0x6D }, 100 }

//  Name:     System.Contact.PrimaryAddressStreet -- PKEY_Contact_PrimaryAddressStreet
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {63C25B20-96BE-488F-8788-C09C407AD812}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_PrimaryAddressStreet, 0x63C25B20, 0x96BE, 0x488F, 0x87, 0x88, 0xC0, 0x9C, 0x40, 0x7A, 0xD8, 0x12, 100);
#define INIT_PKEY_Contact_PrimaryAddressStreet { { 0x63C25B20, 0x96BE, 0x488F, 0x87, 0x88, 0xC0, 0x9C, 0x40, 0x7A, 0xD8, 0x12 }, 100 }

//  Name:     System.Contact.PrimaryEmailAddress -- PKEY_Contact_PrimaryEmailAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 48
DEFINE_PROPERTYKEY(PKEY_Contact_PrimaryEmailAddress, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 48);
#define INIT_PKEY_Contact_PrimaryEmailAddress { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 48 }

//  Name:     System.Contact.PrimaryTelephone -- PKEY_Contact_PrimaryTelephone
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 25
DEFINE_PROPERTYKEY(PKEY_Contact_PrimaryTelephone, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 25);
#define INIT_PKEY_Contact_PrimaryTelephone { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 25 }

//  Name:     System.Contact.Profession -- PKEY_Contact_Profession
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {7268AF55-1CE4-4F6E-A41F-B6E4EF10E4A9}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_Profession, 0x7268AF55, 0x1CE4, 0x4F6E, 0xA4, 0x1F, 0xB6, 0xE4, 0xEF, 0x10, 0xE4, 0xA9, 100);
#define INIT_PKEY_Contact_Profession { { 0x7268AF55, 0x1CE4, 0x4F6E, 0xA4, 0x1F, 0xB6, 0xE4, 0xEF, 0x10, 0xE4, 0xA9 }, 100 }

//  Name:     System.Contact.SpouseName -- PKEY_Contact_SpouseName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9D2408B6-3167-422B-82B0-F583B7A7CFE3}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_SpouseName, 0x9D2408B6, 0x3167, 0x422B, 0x82, 0xB0, 0xF5, 0x83, 0xB7, 0xA7, 0xCF, 0xE3, 100);
#define INIT_PKEY_Contact_SpouseName { { 0x9D2408B6, 0x3167, 0x422B, 0x82, 0xB0, 0xF5, 0x83, 0xB7, 0xA7, 0xCF, 0xE3 }, 100 }

//  Name:     System.Contact.Suffix -- PKEY_Contact_Suffix
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {176DC63C-2688-4E89-8143-A347800F25E9}, 73
DEFINE_PROPERTYKEY(PKEY_Contact_Suffix, 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9, 73);
#define INIT_PKEY_Contact_Suffix { { 0x176DC63C, 0x2688, 0x4E89, 0x81, 0x43, 0xA3, 0x47, 0x80, 0x0F, 0x25, 0xE9 }, 73 }

//  Name:     System.Contact.TelexNumber -- PKEY_Contact_TelexNumber
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C554493C-C1F7-40C1-A76C-EF8C0614003E}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_TelexNumber, 0xC554493C, 0xC1F7, 0x40C1, 0xA7, 0x6C, 0xEF, 0x8C, 0x06, 0x14, 0x00, 0x3E, 100);
#define INIT_PKEY_Contact_TelexNumber { { 0xC554493C, 0xC1F7, 0x40C1, 0xA7, 0x6C, 0xEF, 0x8C, 0x06, 0x14, 0x00, 0x3E }, 100 }

//  Name:     System.Contact.TTYTDDTelephone -- PKEY_Contact_TTYTDDTelephone
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {AAF16BAC-2B55-45E6-9F6D-415EB94910DF}, 100
DEFINE_PROPERTYKEY(PKEY_Contact_TTYTDDTelephone, 0xAAF16BAC, 0x2B55, 0x45E6, 0x9F, 0x6D, 0x41, 0x5E, 0xB9, 0x49, 0x10, 0xDF, 100);
#define INIT_PKEY_Contact_TTYTDDTelephone { { 0xAAF16BAC, 0x2B55, 0x45E6, 0x9F, 0x6D, 0x41, 0x5E, 0xB9, 0x49, 0x10, 0xDF }, 100 }

//  Name:     System.Contact.WebPage -- PKEY_Contact_WebPage
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 18
DEFINE_PROPERTYKEY(PKEY_Contact_WebPage, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 18);
#define INIT_PKEY_Contact_WebPage { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 18 }

//  Name:     System.Contact.Webpage2 -- PKEY_Contact_Webpage2
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 124
DEFINE_PROPERTYKEY(PKEY_Contact_Webpage2, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 124);
#define INIT_PKEY_Contact_Webpage2 { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 124 }

//  Name:     System.Contact.Webpage3 -- PKEY_Contact_Webpage3
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {00F63DD8-22BD-4A5D-BA34-5CB0B9BDCB03}, 125
DEFINE_PROPERTYKEY(PKEY_Contact_Webpage3, 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03, 125);
#define INIT_PKEY_Contact_Webpage3 { { 0x00F63DD8, 0x22BD, 0x4A5D, 0xBA, 0x34, 0x5C, 0xB0, 0xB9, 0xBD, 0xCB, 0x03 }, 125 }

//-----------------------------------------------------------------------------
// Core properties

//  Name:     System.AcquisitionID -- PKEY_AcquisitionID
//  Type:     Int32 -- VT_I4
//  FormatID: {65A98875-3C80-40AB-ABBC-EFDAF77DBEE2}, 100
//
//  Hash to determine acquisition session.
DEFINE_PROPERTYKEY(PKEY_AcquisitionID, 0x65A98875, 0x3C80, 0x40AB, 0xAB, 0xBC, 0xEF, 0xDA, 0xF7, 0x7D, 0xBE, 0xE2, 100);
#define INIT_PKEY_AcquisitionID { { 0x65A98875, 0x3C80, 0x40AB, 0xAB, 0xBC, 0xEF, 0xDA, 0xF7, 0x7D, 0xBE, 0xE2 }, 100 }

//  Name:     System.ApplicationDefinedProperties -- PKEY_ApplicationDefinedProperties
//  Type:     Any -- VT_NULL  Legacy code may treat this as VT_UNKNOWN.
//  FormatID: {CDBFC167-337E-41D8-AF7C-8C09205429C7}, 100
DEFINE_PROPERTYKEY(PKEY_ApplicationDefinedProperties, 0xCDBFC167, 0x337E, 0x41D8, 0xAF, 0x7C, 0x8C, 0x09, 0x20, 0x54, 0x29, 0xC7, 100);
#define INIT_PKEY_ApplicationDefinedProperties { { 0xCDBFC167, 0x337E, 0x41D8, 0xAF, 0x7C, 0x8C, 0x09, 0x20, 0x54, 0x29, 0xC7 }, 100 }

//  Name:     System.ApplicationName -- PKEY_ApplicationName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)  Legacy code may treat this as VT_LPSTR.
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 18 (PIDSI_APPNAME)
//
//  
DEFINE_PROPERTYKEY(PKEY_ApplicationName, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 18);
#define INIT_PKEY_ApplicationName { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 18 }

//  Name:     System.AppZoneIdentifier -- PKEY_AppZoneIdentifier
//  Type:     UInt32 -- VT_UI4
//  FormatID: {502CFEAB-47EB-459C-B960-E6D8728F7701}, 102
//
//  Mark of the app container. The zone identifier as determined by the file's last writer.
DEFINE_PROPERTYKEY(PKEY_AppZoneIdentifier, 0x502CFEAB, 0x47EB, 0x459C, 0xB9, 0x60, 0xE6, 0xD8, 0x72, 0x8F, 0x77, 0x01, 102);
#define INIT_PKEY_AppZoneIdentifier { { 0x502CFEAB, 0x47EB, 0x459C, 0xB9, 0x60, 0xE6, 0xD8, 0x72, 0x8F, 0x77, 0x01 }, 102 }

//  Name:     System.Author -- PKEY_Author
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)  Legacy code may treat this as VT_LPSTR.
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 4 (PIDSI_AUTHOR)
//
//  
DEFINE_PROPERTYKEY(PKEY_Author, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 4);
#define INIT_PKEY_Author { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 4 }

//  Name:     System.CachedFileUpdaterContentIdForConflictResolution -- PKEY_CachedFileUpdaterContentIdForConflictResolution
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 114
DEFINE_PROPERTYKEY(PKEY_CachedFileUpdaterContentIdForConflictResolution, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 114);
#define INIT_PKEY_CachedFileUpdaterContentIdForConflictResolution { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 114 }

//  Name:     System.CachedFileUpdaterContentIdForStream -- PKEY_CachedFileUpdaterContentIdForStream
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 113
DEFINE_PROPERTYKEY(PKEY_CachedFileUpdaterContentIdForStream, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 113);
#define INIT_PKEY_CachedFileUpdaterContentIdForStream { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 113 }

//  Name:     System.Capacity -- PKEY_Capacity
//  Type:     UInt64 -- VT_UI8
//  FormatID: (FMTID_Volume) {9B174B35-40FF-11D2-A27E-00C04FC30871}, 3 (PID_VOLUME_CAPACITY)  (Filesystem Volume Properties)
//
//  The amount of total space in bytes.
DEFINE_PROPERTYKEY(PKEY_Capacity, 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71, 3);
#define INIT_PKEY_Capacity { { 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71 }, 3 }

//  Name:     System.Category -- PKEY_Category
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 2 (PIDDSI_CATEGORY)
//
//  Legacy code treats this as VT_LPSTR.
DEFINE_PROPERTYKEY(PKEY_Category, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 2);
#define INIT_PKEY_Category { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 2 }

//  Name:     System.Comment -- PKEY_Comment
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)  Legacy code may treat this as VT_LPSTR.
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 6 (PIDSI_COMMENTS)
//
//  Comments.
DEFINE_PROPERTYKEY(PKEY_Comment, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 6);
#define INIT_PKEY_Comment { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 6 }

//  Name:     System.Company -- PKEY_Company
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 15 (PIDDSI_COMPANY)
//
//  The company or publisher.
DEFINE_PROPERTYKEY(PKEY_Company, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 15);
#define INIT_PKEY_Company { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 15 }

//  Name:     System.ComputerName -- PKEY_ComputerName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 5 (PID_COMPUTERNAME)
//
//  
DEFINE_PROPERTYKEY(PKEY_ComputerName, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 5);
#define INIT_PKEY_ComputerName { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 5 }

//  Name:     System.ContainedItems -- PKEY_ContainedItems
//  Type:     Multivalue Guid -- VT_VECTOR | VT_CLSID  (For variants: VT_ARRAY | VT_CLSID)
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 29
//  
//  The list of type of items, this item contains. For example, this item contains urls, attachments etc.
//  This is represented as a vector array of GUIDs where each GUID represents certain type.
DEFINE_PROPERTYKEY(PKEY_ContainedItems, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 29);
#define INIT_PKEY_ContainedItems { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 29 }

//  Name:     System.ContentId -- PKEY_ContentId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 132
//
//  Durable identifier for a file that can be used to prevent duplication of activities across devices.
DEFINE_PROPERTYKEY(PKEY_ContentId, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 132);
#define INIT_PKEY_ContentId { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 132 }

//  Name:     System.ContentStatus -- PKEY_ContentStatus
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 27
DEFINE_PROPERTYKEY(PKEY_ContentStatus, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 27);
#define INIT_PKEY_ContentStatus { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 27 }

//  Name:     System.ContentType -- PKEY_ContentType
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 26
DEFINE_PROPERTYKEY(PKEY_ContentType, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 26);
#define INIT_PKEY_ContentType { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 26 }

//  Name:     System.ContentUri -- PKEY_ContentUri
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 131
//
//  Durable identifier for a file that can be used across devices to access file.
DEFINE_PROPERTYKEY(PKEY_ContentUri, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 131);
#define INIT_PKEY_ContentUri { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 131 }

//  Name:     System.Copyright -- PKEY_Copyright
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 11 (PIDMSI_COPYRIGHT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Copyright, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 11);
#define INIT_PKEY_Copyright { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 11 }

//  Name:     System.CreatorAppId -- PKEY_CreatorAppId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C2EA046E-033C-4E91-BD5B-D4942F6BBE49}, 2
//
//  The AppId of the application that created this file.
DEFINE_PROPERTYKEY(PKEY_CreatorAppId, 0xC2EA046E, 0x033C, 0x4E91, 0xBD, 0x5B, 0xD4, 0x94, 0x2F, 0x6B, 0xBE, 0x49, 2);
#define INIT_PKEY_CreatorAppId { { 0xC2EA046E, 0x033C, 0x4E91, 0xBD, 0x5B, 0xD4, 0x94, 0x2F, 0x6B, 0xBE, 0x49 }, 2 }

//  Name:     System.CreatorOpenWithUIOptions -- PKEY_CreatorOpenWithUIOptions
//  Type:     UInt32 -- VT_UI4
//  FormatID: {C2EA046E-033C-4E91-BD5B-D4942F6BBE49}, 3
//
//  Provides options that influence the behavior of UI controls that launch the file with the app specified in System.CreatorAppId.
DEFINE_PROPERTYKEY(PKEY_CreatorOpenWithUIOptions, 0xC2EA046E, 0x033C, 0x4E91, 0xBD, 0x5B, 0xD4, 0x94, 0x2F, 0x6B, 0xBE, 0x49, 3);
#define INIT_PKEY_CreatorOpenWithUIOptions { { 0xC2EA046E, 0x033C, 0x4E91, 0xBD, 0x5B, 0xD4, 0x94, 0x2F, 0x6B, 0xBE, 0x49 }, 3 }

// Possible discrete values for PKEY_CreatorOpenWithUIOptions are:
#define CREATOROPENWITHUIOPTION_HIDDEN      0ul
#define CREATOROPENWITHUIOPTION_VISIBLE     1ul

//  Name:     System.DataObjectFormat -- PKEY_DataObjectFormat
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {1E81A3F8-A30F-4247-B9EE-1D0368A9425C}, 2
//
//  Data object format. String value is the clipboard format name.
DEFINE_PROPERTYKEY(PKEY_DataObjectFormat, 0x1E81A3F8, 0xA30F, 0x4247, 0xB9, 0xEE, 0x1D, 0x03, 0x68, 0xA9, 0x42, 0x5C, 2);
#define INIT_PKEY_DataObjectFormat { { 0x1E81A3F8, 0xA30F, 0x4247, 0xB9, 0xEE, 0x1D, 0x03, 0x68, 0xA9, 0x42, 0x5C }, 2 }

//  Name:     System.DateAccessed -- PKEY_DateAccessed
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 16 (PID_STG_ACCESSTIME)
//
//  The time of the last access to the item.  The Indexing Service friendly name is 'access'.
DEFINE_PROPERTYKEY(PKEY_DateAccessed, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 16);
#define INIT_PKEY_DateAccessed { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 16 }

//  Name:     System.DateAcquired -- PKEY_DateAcquired
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {2CBAA8F5-D81F-47CA-B17A-F8D822300131}, 100
//  
//  The time the file entered the system via acquisition.  This is not the same as System.DateImported.
//  Examples are when pictures are acquired from a camera, or when music is purchased online.
DEFINE_PROPERTYKEY(PKEY_DateAcquired, 0x2CBAA8F5, 0xD81F, 0x47CA, 0xB1, 0x7A, 0xF8, 0xD8, 0x22, 0x30, 0x01, 0x31, 100);
#define INIT_PKEY_DateAcquired { { 0x2CBAA8F5, 0xD81F, 0x47CA, 0xB1, 0x7A, 0xF8, 0xD8, 0x22, 0x30, 0x01, 0x31 }, 100 }

//  Name:     System.DateArchived -- PKEY_DateArchived
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {43F8D7B7-A444-4F87-9383-52271C9B915C}, 100
DEFINE_PROPERTYKEY(PKEY_DateArchived, 0x43F8D7B7, 0xA444, 0x4F87, 0x93, 0x83, 0x52, 0x27, 0x1C, 0x9B, 0x91, 0x5C, 100);
#define INIT_PKEY_DateArchived { { 0x43F8D7B7, 0xA444, 0x4F87, 0x93, 0x83, 0x52, 0x27, 0x1C, 0x9B, 0x91, 0x5C }, 100 }

//  Name:     System.DateCompleted -- PKEY_DateCompleted
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {72FAB781-ACDA-43E5-B155-B2434F85E678}, 100
DEFINE_PROPERTYKEY(PKEY_DateCompleted, 0x72FAB781, 0xACDA, 0x43E5, 0xB1, 0x55, 0xB2, 0x43, 0x4F, 0x85, 0xE6, 0x78, 100);
#define INIT_PKEY_DateCompleted { { 0x72FAB781, 0xACDA, 0x43E5, 0xB1, 0x55, 0xB2, 0x43, 0x4F, 0x85, 0xE6, 0x78 }, 100 }

//  Name:     System.DateCreated -- PKEY_DateCreated
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 15 (PID_STG_CREATETIME)
//
//  The date and time the item was created. The Indexing Service friendly name is 'create'.
DEFINE_PROPERTYKEY(PKEY_DateCreated, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 15);
#define INIT_PKEY_DateCreated { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 15 }

//  Name:     System.DateImported -- PKEY_DateImported
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 18258
//
//  The time the file is imported into a separate database.  This is not the same as System.DateAcquired.  (Eg, 2003:05:22 13:55:04)
DEFINE_PROPERTYKEY(PKEY_DateImported, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 18258);
#define INIT_PKEY_DateImported { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 18258 }

//  Name:     System.DateModified -- PKEY_DateModified
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 14 (PID_STG_WRITETIME)
//
//  The date and time of the last write to the item. The Indexing Service friendly name is 'write'.
DEFINE_PROPERTYKEY(PKEY_DateModified, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 14);
#define INIT_PKEY_DateModified { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 14 }

//  Name:     System.DefaultSaveLocationDisplay -- PKEY_DefaultSaveLocationDisplay
//  Type:     UInt32 -- VT_UI4
//  FormatID: {5D76B67F-9B3D-44BB-B6AE-25DA4F638A67}, 10
//
//  Helps display as an icon whether or not a location is the default save location for owner and/or non-owners of a library
DEFINE_PROPERTYKEY(PKEY_DefaultSaveLocationDisplay, 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67, 10);
#define INIT_PKEY_DefaultSaveLocationDisplay { { 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67 }, 10 }

// Possible discrete values for PKEY_DefaultSaveLocationDisplay are:
#define ISDEFAULTSAVE_NONE                  0ul
#define ISDEFAULTSAVE_OWNER                 1ul
#define ISDEFAULTSAVE_NONOWNER              2ul
#define ISDEFAULTSAVE_BOTH                  3ul

//  Name:     System.DueDate -- PKEY_DueDate
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {3F8472B5-E0AF-4DB2-8071-C53FE76AE7CE}, 100
DEFINE_PROPERTYKEY(PKEY_DueDate, 0x3F8472B5, 0xE0AF, 0x4DB2, 0x80, 0x71, 0xC5, 0x3F, 0xE7, 0x6A, 0xE7, 0xCE, 100);
#define INIT_PKEY_DueDate { { 0x3F8472B5, 0xE0AF, 0x4DB2, 0x80, 0x71, 0xC5, 0x3F, 0xE7, 0x6A, 0xE7, 0xCE }, 100 }

//  Name:     System.EndDate -- PKEY_EndDate
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {C75FAA05-96FD-49E7-9CB4-9F601082D553}, 100
DEFINE_PROPERTYKEY(PKEY_EndDate, 0xC75FAA05, 0x96FD, 0x49E7, 0x9C, 0xB4, 0x9F, 0x60, 0x10, 0x82, 0xD5, 0x53, 100);
#define INIT_PKEY_EndDate { { 0xC75FAA05, 0x96FD, 0x49E7, 0x9C, 0xB4, 0x9F, 0x60, 0x10, 0x82, 0xD5, 0x53 }, 100 }

//  Name:     System.ExpandoProperties -- PKEY_ExpandoProperties
//  Type:     Any -- VT_NULL  Legacy code may treat this as VT_UNKNOWN.
//  FormatID: {6FA20DE6-D11C-4D9D-A154-64317628C12D}, 100
//
//  Properties that are not stored in the item itself, where the properties are in the form of a stream containing a SERIALIZEDPROPSTORAGE.
DEFINE_PROPERTYKEY(PKEY_ExpandoProperties, 0x6FA20DE6, 0xD11C, 0x4D9D, 0xA1, 0x54, 0x64, 0x31, 0x76, 0x28, 0xC1, 0x2D, 100);
#define INIT_PKEY_ExpandoProperties { { 0x6FA20DE6, 0xD11C, 0x4D9D, 0xA1, 0x54, 0x64, 0x31, 0x76, 0x28, 0xC1, 0x2D }, 100 }

//  Name:     System.FileAllocationSize -- PKEY_FileAllocationSize
//  Type:     UInt64 -- VT_UI8
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 18 (PID_STG_ALLOCSIZE)
//
//  
DEFINE_PROPERTYKEY(PKEY_FileAllocationSize, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 18);
#define INIT_PKEY_FileAllocationSize { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 18 }

//  Name:     System.FileAttributes -- PKEY_FileAttributes
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 13 (PID_STG_ATTRIBUTES)
//  
//  This is the WIN32_FIND_DATA dwFileAttributes for the file-based item.
DEFINE_PROPERTYKEY(PKEY_FileAttributes, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 13);
#define INIT_PKEY_FileAttributes { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 13 }

//  Name:     System.FileCount -- PKEY_FileCount
//  Type:     UInt64 -- VT_UI8
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 12
//
//  
DEFINE_PROPERTYKEY(PKEY_FileCount, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 12);
#define INIT_PKEY_FileCount { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 12 }

//  Name:     System.FileDescription -- PKEY_FileDescription
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSFMTID_VERSION) {0CEF7D53-FA64-11D1-A203-0000F81FEDEE}, 3 (PIDVSI_FileDescription)
//  
//  This is a user-friendly description of the file.
DEFINE_PROPERTYKEY(PKEY_FileDescription, 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE, 3);
#define INIT_PKEY_FileDescription { { 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE }, 3 }

//  Name:     System.FileExtension -- PKEY_FileExtension
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E4F10A3C-49E6-405D-8288-A23BD4EEAA6C}, 100
//  
//  This is the file extension of the file based item, including the leading period.  
//  
//  If System.FileName is VT_EMPTY, then this property should be too.  Otherwise, it should be derived
//  appropriately by the data source from System.FileName.  If System.FileName does not have a file 
//  extension, this value should be VT_EMPTY.
//  
//  To obtain the type of any item (including an item that is not a file), use System.ItemType.
//  
//  Example values:
//  
//      If the path is...                     The property value is...
//      -----------------                     ------------------------
//      "c:\folder\subfolder\hello.txt"       ".txt"
//      "\\server\share\mydir\goodnews.doc"   ".doc"
//      "\\server\share\numbers.xls"          ".xls"
//      "\\server\share\folder"               VT_EMPTY
//      "c:\folder\MyFolder"                  VT_EMPTY
//      [desktop]                             VT_EMPTY
DEFINE_PROPERTYKEY(PKEY_FileExtension, 0xE4F10A3C, 0x49E6, 0x405D, 0x82, 0x88, 0xA2, 0x3B, 0xD4, 0xEE, 0xAA, 0x6C, 100);
#define INIT_PKEY_FileExtension { { 0xE4F10A3C, 0x49E6, 0x405D, 0x82, 0x88, 0xA2, 0x3B, 0xD4, 0xEE, 0xAA, 0x6C }, 100 }

//  Name:     System.FileFRN -- PKEY_FileFRN
//  Type:     UInt64 -- VT_UI8
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 21 (PID_STG_FRN)
//  
//  This is the unique file ID, also known as the File Reference Number. For a given file, this is the same value
//  as is found in the structure variable FILE_ID_BOTH_DIR_INFO.FileId, via GetFileInformationByHandleEx().
DEFINE_PROPERTYKEY(PKEY_FileFRN, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 21);
#define INIT_PKEY_FileFRN { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 21 }

//  Name:     System.FileName -- PKEY_FileName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {41CF5AE0-F75A-4806-BD87-59C7D9248EB9}, 100
//  
//  This is the file name (including extension) of the file.
//  
//  It is possible that the item might not exist on a filesystem (ie, it may not be opened 
//  using CreateFile).  Nonetheless, if the item is represented as a file from the logical sense 
//  (and its name follows standard Win32 file-naming syntax), then the data source should emit this property.
//  
//  If an item is not a file, then the value for this property is VT_EMPTY.  See 
//  System.ItemNameDisplay.
//  
//  This has the same value as System.ParsingName for items that are provided by the Shell's file folder.
//  
//  Example values:
//  
//      If the path is...                     The property value is...
//      -----------------                     ------------------------
//      "c:\folder\subfolder\hello.txt"       "hello.txt"
//      "\\server\share\mydir\goodnews.doc"   "goodnews.doc"
//      "\\server\share\numbers.xls"          "numbers.xls"
//      "c:\folder\MyFolder"                  "MyFolder"
//      (email message)                       VT_EMPTY
//      (song on portable device)             "song.wma"
DEFINE_PROPERTYKEY(PKEY_FileName, 0x41CF5AE0, 0xF75A, 0x4806, 0xBD, 0x87, 0x59, 0xC7, 0xD9, 0x24, 0x8E, 0xB9, 100);
#define INIT_PKEY_FileName { { 0x41CF5AE0, 0xF75A, 0x4806, 0xBD, 0x87, 0x59, 0xC7, 0xD9, 0x24, 0x8E, 0xB9 }, 100 }

//  Name:     System.FileOfflineAvailabilityStatus -- PKEY_FileOfflineAvailabilityStatus
//  Type:     UInt32 -- VT_UI4
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 100
//  
//  Null (VT_EMPTY) indicates the normal case (file is available offline). 
//  The partial case is only for folders where some content may be available offline and some 
//  may not. The Complete cases are for folders only, where everything inside is present 
//  and/or pinned 
DEFINE_PROPERTYKEY(PKEY_FileOfflineAvailabilityStatus, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 100);
#define INIT_PKEY_FileOfflineAvailabilityStatus { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 100 }

// Possible discrete values for PKEY_FileOfflineAvailabilityStatus are:
#define FILEOFFLINEAVAILABILITYSTATUS_NOTAVAILABLEOFFLINE 0ul
#define FILEOFFLINEAVAILABILITYSTATUS_PARTIAL 1ul
#define FILEOFFLINEAVAILABILITYSTATUS_COMPLETE 2ul
#define FILEOFFLINEAVAILABILITYSTATUS_COMPLETE_PINNED 3ul
#define FILEOFFLINEAVAILABILITYSTATUS_EXCLUDED 4ul
#define FILEOFFLINEAVAILABILITYSTATUS_FOLDER_EMPTY 5ul

//  Name:     System.FileOwner -- PKEY_FileOwner
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Misc) {9B174B34-40FF-11D2-A27E-00C04FC30871}, 4 (PID_MISC_OWNER)
//  
//  This is the owner of the file, according to the file system.
DEFINE_PROPERTYKEY(PKEY_FileOwner, 0x9B174B34, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71, 4);
#define INIT_PKEY_FileOwner { { 0x9B174B34, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71 }, 4 }

//  Name:     System.FilePlaceholderStatus -- PKEY_FilePlaceholderStatus
//  Type:     UInt32 -- VT_UI4
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 2
//
//  Contains the placeholder file's status flags.
DEFINE_PROPERTYKEY(PKEY_FilePlaceholderStatus, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 2);
#define INIT_PKEY_FilePlaceholderStatus { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 2 }

//  Name:     System.FileVersion -- PKEY_FileVersion
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSFMTID_VERSION) {0CEF7D53-FA64-11D1-A203-0000F81FEDEE}, 4 (PIDVSI_FileVersion)
//
//  
DEFINE_PROPERTYKEY(PKEY_FileVersion, 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE, 4);
#define INIT_PKEY_FileVersion { { 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE }, 4 }

//  Name:     System.FindData -- PKEY_FindData
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 0 (PID_FINDDATA)
//
//  WIN32_FIND_DATAW in buffer of bytes.
DEFINE_PROPERTYKEY(PKEY_FindData, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 0);
#define INIT_PKEY_FindData { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 0 }

//  Name:     System.FlagColor -- PKEY_FlagColor
//  Type:     UInt16 -- VT_UI2
//  FormatID: {67DF94DE-0CA7-4D6F-B792-053A3E4F03CF}, 100
//
//  
DEFINE_PROPERTYKEY(PKEY_FlagColor, 0x67DF94DE, 0x0CA7, 0x4D6F, 0xB7, 0x92, 0x05, 0x3A, 0x3E, 0x4F, 0x03, 0xCF, 100);
#define INIT_PKEY_FlagColor { { 0x67DF94DE, 0x0CA7, 0x4D6F, 0xB7, 0x92, 0x05, 0x3A, 0x3E, 0x4F, 0x03, 0xCF }, 100 }

// Possible discrete values for PKEY_FlagColor are:
#define FLAGCOLOR_PURPLE                    1u
#define FLAGCOLOR_ORANGE                    2u
#define FLAGCOLOR_GREEN                     3u
#define FLAGCOLOR_YELLOW                    4u
#define FLAGCOLOR_BLUE                      5u
#define FLAGCOLOR_RED                       6u

//  Name:     System.FlagColorText -- PKEY_FlagColorText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {45EAE747-8E2A-40AE-8CBF-CA52ABA6152A}, 100
//  
//  This is the user-friendly form of System.FlagColor.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_FlagColorText, 0x45EAE747, 0x8E2A, 0x40AE, 0x8C, 0xBF, 0xCA, 0x52, 0xAB, 0xA6, 0x15, 0x2A, 100);
#define INIT_PKEY_FlagColorText { { 0x45EAE747, 0x8E2A, 0x40AE, 0x8C, 0xBF, 0xCA, 0x52, 0xAB, 0xA6, 0x15, 0x2A }, 100 }

//  Name:     System.FlagStatus -- PKEY_FlagStatus
//  Type:     Int32 -- VT_I4
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 12
//
//  Status of Flag.  Values: (0=none 1=white 2=Red).  cdoPR_FLAG_STATUS
DEFINE_PROPERTYKEY(PKEY_FlagStatus, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 12);
#define INIT_PKEY_FlagStatus { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 12 }

// Possible discrete values for PKEY_FlagStatus are:
#define FLAGSTATUS_NOTFLAGGED               0l
#define FLAGSTATUS_COMPLETED                1l
#define FLAGSTATUS_FOLLOWUP                 2l

//  Name:     System.FlagStatusText -- PKEY_FlagStatusText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {DC54FD2E-189D-4871-AA01-08C2F57A4ABC}, 100
//  
//  This is the user-friendly form of System.FlagStatus.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_FlagStatusText, 0xDC54FD2E, 0x189D, 0x4871, 0xAA, 0x01, 0x08, 0xC2, 0xF5, 0x7A, 0x4A, 0xBC, 100);
#define INIT_PKEY_FlagStatusText { { 0xDC54FD2E, 0x189D, 0x4871, 0xAA, 0x01, 0x08, 0xC2, 0xF5, 0x7A, 0x4A, 0xBC }, 100 }

//  Name:     System.FolderKind -- PKEY_FolderKind
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 101
//  
//  This property represents the types of content stored in this folder specified by the storage provider.
//  Each folder type must be one of the known value specified by System.Kind definition
//  System.FolderKind is a readonly property, it should only be updated by the storage provider.
DEFINE_PROPERTYKEY(PKEY_FolderKind, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 101);
#define INIT_PKEY_FolderKind { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 101 }

//  Name:     System.FolderNameDisplay -- PKEY_FolderNameDisplay
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 25
//  
//  This property is similar to System.ItemNameDisplay except it is only set for folders, for files it will be empty. 
//  This is useful to segregate files and folders by using this as the first sort key. When 
//  System.ItemDate is used as a second sort key it produces results with folders first ordered by name, then 
//  files ordered by date.
DEFINE_PROPERTYKEY(PKEY_FolderNameDisplay, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 25);
#define INIT_PKEY_FolderNameDisplay { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 25 }

//  Name:     System.FreeSpace -- PKEY_FreeSpace
//  Type:     UInt64 -- VT_UI8
//  FormatID: (FMTID_Volume) {9B174B35-40FF-11D2-A27E-00C04FC30871}, 2 (PID_VOLUME_FREE)  (Filesystem Volume Properties)
//
//  The amount of free space in bytes.
DEFINE_PROPERTYKEY(PKEY_FreeSpace, 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71, 2);
#define INIT_PKEY_FreeSpace { { 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71 }, 2 }

//  Name:     System.FullText -- PKEY_FullText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {1E3EE840-BC2B-476C-8237-2ACD1A839B22}, 6
//  
//  This PKEY is used to specify search terms that should be applied as broadly as possible,
//  across all valid properties for the data source(s) being searched.  It should not be
//  emitted from a data source.
DEFINE_PROPERTYKEY(PKEY_FullText, 0x1E3EE840, 0xBC2B, 0x476C, 0x82, 0x37, 0x2A, 0xCD, 0x1A, 0x83, 0x9B, 0x22, 6);
#define INIT_PKEY_FullText { { 0x1E3EE840, 0xBC2B, 0x476C, 0x82, 0x37, 0x2A, 0xCD, 0x1A, 0x83, 0x9B, 0x22 }, 6 }

//  Name:     System.HighKeywords -- PKEY_HighKeywords
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 24
//
//  The high confidence keywords for the item.
DEFINE_PROPERTYKEY(PKEY_HighKeywords, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 24);
#define INIT_PKEY_HighKeywords { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 24 }

//  Name:     System.Identity -- PKEY_Identity
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A26F4AFC-7346-4299-BE47-EB1AE613139F}, 100
DEFINE_PROPERTYKEY(PKEY_Identity, 0xA26F4AFC, 0x7346, 0x4299, 0xBE, 0x47, 0xEB, 0x1A, 0xE6, 0x13, 0x13, 0x9F, 100);
#define INIT_PKEY_Identity { { 0xA26F4AFC, 0x7346, 0x4299, 0xBE, 0x47, 0xEB, 0x1A, 0xE6, 0x13, 0x13, 0x9F }, 100 }

//  Name:     System.Identity.Blob -- PKEY_Identity_Blob
//  Type:     Blob -- VT_BLOB
//  FormatID: {8C3B93A4-BAED-1A83-9A32-102EE313F6EB}, 100
//
//  Blob used to import/export identities
DEFINE_PROPERTYKEY(PKEY_Identity_Blob, 0x8C3B93A4, 0xBAED, 0x1A83, 0x9A, 0x32, 0x10, 0x2E, 0xE3, 0x13, 0xF6, 0xEB, 100);
#define INIT_PKEY_Identity_Blob { { 0x8C3B93A4, 0xBAED, 0x1A83, 0x9A, 0x32, 0x10, 0x2E, 0xE3, 0x13, 0xF6, 0xEB }, 100 }

//  Name:     System.Identity.DisplayName -- PKEY_Identity_DisplayName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {7D683FC9-D155-45A8-BB1F-89D19BCB792F}, 100
//
//  Display Name
DEFINE_PROPERTYKEY(PKEY_Identity_DisplayName, 0x7D683FC9, 0xD155, 0x45A8, 0xBB, 0x1F, 0x89, 0xD1, 0x9B, 0xCB, 0x79, 0x2F, 100);
#define INIT_PKEY_Identity_DisplayName { { 0x7D683FC9, 0xD155, 0x45A8, 0xBB, 0x1F, 0x89, 0xD1, 0x9B, 0xCB, 0x79, 0x2F }, 100 }

//  Name:     System.Identity.InternetSid -- PKEY_Identity_InternetSid
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {6D6D5D49-265D-4688-9F4E-1FDD33E7CC83}, 100
//
//  Internet SID of an identity
DEFINE_PROPERTYKEY(PKEY_Identity_InternetSid, 0x6D6D5D49, 0x265D, 0x4688, 0x9F, 0x4E, 0x1F, 0xDD, 0x33, 0xE7, 0xCC, 0x83, 100);
#define INIT_PKEY_Identity_InternetSid { { 0x6D6D5D49, 0x265D, 0x4688, 0x9F, 0x4E, 0x1F, 0xDD, 0x33, 0xE7, 0xCC, 0x83 }, 100 }

//  Name:     System.Identity.IsMeIdentity -- PKEY_Identity_IsMeIdentity
//  Type:     Boolean -- VT_BOOL
//  FormatID: {A4108708-09DF-4377-9DFC-6D99986D5A67}, 100
//
//  Is it Me Identity
DEFINE_PROPERTYKEY(PKEY_Identity_IsMeIdentity, 0xA4108708, 0x09DF, 0x4377, 0x9D, 0xFC, 0x6D, 0x99, 0x98, 0x6D, 0x5A, 0x67, 100);
#define INIT_PKEY_Identity_IsMeIdentity { { 0xA4108708, 0x09DF, 0x4377, 0x9D, 0xFC, 0x6D, 0x99, 0x98, 0x6D, 0x5A, 0x67 }, 100 }

//  Name:     System.Identity.KeyProviderContext -- PKEY_Identity_KeyProviderContext
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A26F4AFC-7346-4299-BE47-EB1AE613139F}, 17
//
//  Identity key provider context
DEFINE_PROPERTYKEY(PKEY_Identity_KeyProviderContext, 0xA26F4AFC, 0x7346, 0x4299, 0xBE, 0x47, 0xEB, 0x1A, 0xE6, 0x13, 0x13, 0x9F, 17);
#define INIT_PKEY_Identity_KeyProviderContext { { 0xA26F4AFC, 0x7346, 0x4299, 0xBE, 0x47, 0xEB, 0x1A, 0xE6, 0x13, 0x13, 0x9F }, 17 }

//  Name:     System.Identity.KeyProviderName -- PKEY_Identity_KeyProviderName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A26F4AFC-7346-4299-BE47-EB1AE613139F}, 16
//
//  Identity key provider name
DEFINE_PROPERTYKEY(PKEY_Identity_KeyProviderName, 0xA26F4AFC, 0x7346, 0x4299, 0xBE, 0x47, 0xEB, 0x1A, 0xE6, 0x13, 0x13, 0x9F, 16);
#define INIT_PKEY_Identity_KeyProviderName { { 0xA26F4AFC, 0x7346, 0x4299, 0xBE, 0x47, 0xEB, 0x1A, 0xE6, 0x13, 0x13, 0x9F }, 16 }

//  Name:     System.Identity.LogonStatusString -- PKEY_Identity_LogonStatusString
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {F18DEDF3-337F-42C0-9E03-CEE08708A8C3}, 100
//
//  Identity User Logon Status String
DEFINE_PROPERTYKEY(PKEY_Identity_LogonStatusString, 0xF18DEDF3, 0x337F, 0x42C0, 0x9E, 0x03, 0xCE, 0xE0, 0x87, 0x08, 0xA8, 0xC3, 100);
#define INIT_PKEY_Identity_LogonStatusString { { 0xF18DEDF3, 0x337F, 0x42C0, 0x9E, 0x03, 0xCE, 0xE0, 0x87, 0x08, 0xA8, 0xC3 }, 100 }

//  Name:     System.Identity.PrimaryEmailAddress -- PKEY_Identity_PrimaryEmailAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FCC16823-BAED-4F24-9B32-A0982117F7FA}, 100
//
//  Primary Email Address
DEFINE_PROPERTYKEY(PKEY_Identity_PrimaryEmailAddress, 0xFCC16823, 0xBAED, 0x4F24, 0x9B, 0x32, 0xA0, 0x98, 0x21, 0x17, 0xF7, 0xFA, 100);
#define INIT_PKEY_Identity_PrimaryEmailAddress { { 0xFCC16823, 0xBAED, 0x4F24, 0x9B, 0x32, 0xA0, 0x98, 0x21, 0x17, 0xF7, 0xFA }, 100 }

//  Name:     System.Identity.PrimarySid -- PKEY_Identity_PrimarySid
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {2B1B801E-C0C1-4987-9EC5-72FA89814787}, 100
//
//  Primary SID of an identity
DEFINE_PROPERTYKEY(PKEY_Identity_PrimarySid, 0x2B1B801E, 0xC0C1, 0x4987, 0x9E, 0xC5, 0x72, 0xFA, 0x89, 0x81, 0x47, 0x87, 100);
#define INIT_PKEY_Identity_PrimarySid { { 0x2B1B801E, 0xC0C1, 0x4987, 0x9E, 0xC5, 0x72, 0xFA, 0x89, 0x81, 0x47, 0x87 }, 100 }

//  Name:     System.Identity.ProviderData -- PKEY_Identity_ProviderData
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A8A74B92-361B-4E9A-B722-7C4A7330A312}, 100
//
//  Provider custom data of an identity
DEFINE_PROPERTYKEY(PKEY_Identity_ProviderData, 0xA8A74B92, 0x361B, 0x4E9A, 0xB7, 0x22, 0x7C, 0x4A, 0x73, 0x30, 0xA3, 0x12, 100);
#define INIT_PKEY_Identity_ProviderData { { 0xA8A74B92, 0x361B, 0x4E9A, 0xB7, 0x22, 0x7C, 0x4A, 0x73, 0x30, 0xA3, 0x12 }, 100 }

//  Name:     System.Identity.ProviderID -- PKEY_Identity_ProviderID
//  Type:     Guid -- VT_CLSID
//  FormatID: {74A7DE49-FA11-4D3D-A006-DB7E08675916}, 100
//
//  Provider ID
DEFINE_PROPERTYKEY(PKEY_Identity_ProviderID, 0x74A7DE49, 0xFA11, 0x4D3D, 0xA0, 0x06, 0xDB, 0x7E, 0x08, 0x67, 0x59, 0x16, 100);
#define INIT_PKEY_Identity_ProviderID { { 0x74A7DE49, 0xFA11, 0x4D3D, 0xA0, 0x06, 0xDB, 0x7E, 0x08, 0x67, 0x59, 0x16 }, 100 }

//  Name:     System.Identity.QualifiedUserName -- PKEY_Identity_QualifiedUserName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {DA520E51-F4E9-4739-AC82-02E0A95C9030}, 100
//
//  Identity Qualified User Name
DEFINE_PROPERTYKEY(PKEY_Identity_QualifiedUserName, 0xDA520E51, 0xF4E9, 0x4739, 0xAC, 0x82, 0x02, 0xE0, 0xA9, 0x5C, 0x90, 0x30, 100);
#define INIT_PKEY_Identity_QualifiedUserName { { 0xDA520E51, 0xF4E9, 0x4739, 0xAC, 0x82, 0x02, 0xE0, 0xA9, 0x5C, 0x90, 0x30 }, 100 }

//  Name:     System.Identity.UniqueID -- PKEY_Identity_UniqueID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E55FC3B0-2B60-4220-918E-B21E8BF16016}, 100
//
//  Unique ID
DEFINE_PROPERTYKEY(PKEY_Identity_UniqueID, 0xE55FC3B0, 0x2B60, 0x4220, 0x91, 0x8E, 0xB2, 0x1E, 0x8B, 0xF1, 0x60, 0x16, 100);
#define INIT_PKEY_Identity_UniqueID { { 0xE55FC3B0, 0x2B60, 0x4220, 0x91, 0x8E, 0xB2, 0x1E, 0x8B, 0xF1, 0x60, 0x16 }, 100 }

//  Name:     System.Identity.UserName -- PKEY_Identity_UserName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C4322503-78CA-49C6-9ACC-A68E2AFD7B6B}, 100
//
//  Identity User Name
DEFINE_PROPERTYKEY(PKEY_Identity_UserName, 0xC4322503, 0x78CA, 0x49C6, 0x9A, 0xCC, 0xA6, 0x8E, 0x2A, 0xFD, 0x7B, 0x6B, 100);
#define INIT_PKEY_Identity_UserName { { 0xC4322503, 0x78CA, 0x49C6, 0x9A, 0xCC, 0xA6, 0x8E, 0x2A, 0xFD, 0x7B, 0x6B }, 100 }

//  Name:     System.IdentityProvider.Name -- PKEY_IdentityProvider_Name
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B96EFF7B-35CA-4A35-8607-29E3A54C46EA}, 100
//
//  Identity Provider Name
DEFINE_PROPERTYKEY(PKEY_IdentityProvider_Name, 0xB96EFF7B, 0x35CA, 0x4A35, 0x86, 0x07, 0x29, 0xE3, 0xA5, 0x4C, 0x46, 0xEA, 100);
#define INIT_PKEY_IdentityProvider_Name { { 0xB96EFF7B, 0x35CA, 0x4A35, 0x86, 0x07, 0x29, 0xE3, 0xA5, 0x4C, 0x46, 0xEA }, 100 }

//  Name:     System.IdentityProvider.Picture -- PKEY_IdentityProvider_Picture
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {2425166F-5642-4864-992F-98FD98F294C3}, 100
//
//  Picture for the Identity Provider
DEFINE_PROPERTYKEY(PKEY_IdentityProvider_Picture, 0x2425166F, 0x5642, 0x4864, 0x99, 0x2F, 0x98, 0xFD, 0x98, 0xF2, 0x94, 0xC3, 100);
#define INIT_PKEY_IdentityProvider_Picture { { 0x2425166F, 0x5642, 0x4864, 0x99, 0x2F, 0x98, 0xFD, 0x98, 0xF2, 0x94, 0xC3 }, 100 }

//  Name:     System.ImageParsingName -- PKEY_ImageParsingName
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D7750EE0-C6A4-48EC-B53E-B87B52E6D073}, 100
DEFINE_PROPERTYKEY(PKEY_ImageParsingName, 0xD7750EE0, 0xC6A4, 0x48EC, 0xB5, 0x3E, 0xB8, 0x7B, 0x52, 0xE6, 0xD0, 0x73, 100);
#define INIT_PKEY_ImageParsingName { { 0xD7750EE0, 0xC6A4, 0x48EC, 0xB5, 0x3E, 0xB8, 0x7B, 0x52, 0xE6, 0xD0, 0x73 }, 100 }

//  Name:     System.Importance -- PKEY_Importance
//  Type:     Int32 -- VT_I4
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 11
DEFINE_PROPERTYKEY(PKEY_Importance, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 11);
#define INIT_PKEY_Importance { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 11 }

// Possible range of values for PKEY_Importance are:
#define IMPORTANCE_LOW_MIN                  0l
#define IMPORTANCE_LOW_SET                  1l
#define IMPORTANCE_LOW_MAX                  1l

#define IMPORTANCE_NORMAL_MIN               2l
#define IMPORTANCE_NORMAL_SET               3l
#define IMPORTANCE_NORMAL_MAX               4l

#define IMPORTANCE_HIGH_MIN                 5l
#define IMPORTANCE_HIGH_SET                 5l
#define IMPORTANCE_HIGH_MAX                 5l


//  Name:     System.ImportanceText -- PKEY_ImportanceText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A3B29791-7713-4E1D-BB40-17DB85F01831}, 100
//  
//  This is the user-friendly form of System.Importance.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_ImportanceText, 0xA3B29791, 0x7713, 0x4E1D, 0xBB, 0x40, 0x17, 0xDB, 0x85, 0xF0, 0x18, 0x31, 100);
#define INIT_PKEY_ImportanceText { { 0xA3B29791, 0x7713, 0x4E1D, 0xBB, 0x40, 0x17, 0xDB, 0x85, 0xF0, 0x18, 0x31 }, 100 }

//  Name:     System.IsAttachment -- PKEY_IsAttachment
//  Type:     Boolean -- VT_BOOL
//  FormatID: {F23F425C-71A1-4FA8-922F-678EA4A60408}, 100
//
//  Identifies if this item is an attachment.
DEFINE_PROPERTYKEY(PKEY_IsAttachment, 0xF23F425C, 0x71A1, 0x4FA8, 0x92, 0x2F, 0x67, 0x8E, 0xA4, 0xA6, 0x04, 0x08, 100);
#define INIT_PKEY_IsAttachment { { 0xF23F425C, 0x71A1, 0x4FA8, 0x92, 0x2F, 0x67, 0x8E, 0xA4, 0xA6, 0x04, 0x08 }, 100 }

//  Name:     System.IsDefaultNonOwnerSaveLocation -- PKEY_IsDefaultNonOwnerSaveLocation
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5D76B67F-9B3D-44BB-B6AE-25DA4F638A67}, 5
//
//  Identifies the default save location for a library for non-owners of the library
DEFINE_PROPERTYKEY(PKEY_IsDefaultNonOwnerSaveLocation, 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67, 5);
#define INIT_PKEY_IsDefaultNonOwnerSaveLocation { { 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67 }, 5 }

//  Name:     System.IsDefaultSaveLocation -- PKEY_IsDefaultSaveLocation
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5D76B67F-9B3D-44BB-B6AE-25DA4F638A67}, 3
//
//  Identifies the default save location for a library for the owner of the library
DEFINE_PROPERTYKEY(PKEY_IsDefaultSaveLocation, 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67, 3);
#define INIT_PKEY_IsDefaultSaveLocation { { 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67 }, 3 }

//  Name:     System.IsDeleted -- PKEY_IsDeleted
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5CDA5FC8-33EE-4FF3-9094-AE7BD8868C4D}, 100
DEFINE_PROPERTYKEY(PKEY_IsDeleted, 0x5CDA5FC8, 0x33EE, 0x4FF3, 0x90, 0x94, 0xAE, 0x7B, 0xD8, 0x86, 0x8C, 0x4D, 100);
#define INIT_PKEY_IsDeleted { { 0x5CDA5FC8, 0x33EE, 0x4FF3, 0x90, 0x94, 0xAE, 0x7B, 0xD8, 0x86, 0x8C, 0x4D }, 100 }

//  Name:     System.IsEncrypted -- PKEY_IsEncrypted
//  Type:     Boolean -- VT_BOOL
//  FormatID: {90E5E14E-648B-4826-B2AA-ACAF790E3513}, 10
//
//  Is the item encrypted?
DEFINE_PROPERTYKEY(PKEY_IsEncrypted, 0x90E5E14E, 0x648B, 0x4826, 0xB2, 0xAA, 0xAC, 0xAF, 0x79, 0x0E, 0x35, 0x13, 10);
#define INIT_PKEY_IsEncrypted { { 0x90E5E14E, 0x648B, 0x4826, 0xB2, 0xAA, 0xAC, 0xAF, 0x79, 0x0E, 0x35, 0x13 }, 10 }

//  Name:     System.IsFlagged -- PKEY_IsFlagged
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5DA84765-E3FF-4278-86B0-A27967FBDD03}, 100
DEFINE_PROPERTYKEY(PKEY_IsFlagged, 0x5DA84765, 0xE3FF, 0x4278, 0x86, 0xB0, 0xA2, 0x79, 0x67, 0xFB, 0xDD, 0x03, 100);
#define INIT_PKEY_IsFlagged { { 0x5DA84765, 0xE3FF, 0x4278, 0x86, 0xB0, 0xA2, 0x79, 0x67, 0xFB, 0xDD, 0x03 }, 100 }

//  Name:     System.IsFlaggedComplete -- PKEY_IsFlaggedComplete
//  Type:     Boolean -- VT_BOOL
//  FormatID: {A6F360D2-55F9-48DE-B909-620E090A647C}, 100
DEFINE_PROPERTYKEY(PKEY_IsFlaggedComplete, 0xA6F360D2, 0x55F9, 0x48DE, 0xB9, 0x09, 0x62, 0x0E, 0x09, 0x0A, 0x64, 0x7C, 100);
#define INIT_PKEY_IsFlaggedComplete { { 0xA6F360D2, 0x55F9, 0x48DE, 0xB9, 0x09, 0x62, 0x0E, 0x09, 0x0A, 0x64, 0x7C }, 100 }

//  Name:     System.IsIncomplete -- PKEY_IsIncomplete
//  Type:     Boolean -- VT_BOOL
//  FormatID: {346C8BD1-2E6A-4C45-89A4-61B78E8E700F}, 100
//
//  Identifies if the message was not completely received for some error condition.
DEFINE_PROPERTYKEY(PKEY_IsIncomplete, 0x346C8BD1, 0x2E6A, 0x4C45, 0x89, 0xA4, 0x61, 0xB7, 0x8E, 0x8E, 0x70, 0x0F, 100);
#define INIT_PKEY_IsIncomplete { { 0x346C8BD1, 0x2E6A, 0x4C45, 0x89, 0xA4, 0x61, 0xB7, 0x8E, 0x8E, 0x70, 0x0F }, 100 }

//  Name:     System.IsLocationSupported -- PKEY_IsLocationSupported
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5D76B67F-9B3D-44BB-B6AE-25DA4F638A67}, 8
//
//  A bool value to know if a location is supported (locally indexable, or remotely indexed).
DEFINE_PROPERTYKEY(PKEY_IsLocationSupported, 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67, 8);
#define INIT_PKEY_IsLocationSupported { { 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67 }, 8 }

//  Name:     System.IsPinnedToNameSpaceTree -- PKEY_IsPinnedToNameSpaceTree
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5D76B67F-9B3D-44BB-B6AE-25DA4F638A67}, 2
//
//  A bool value to know if a shell folder is pinned to the navigation pane
DEFINE_PROPERTYKEY(PKEY_IsPinnedToNameSpaceTree, 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67, 2);
#define INIT_PKEY_IsPinnedToNameSpaceTree { { 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67 }, 2 }

//  Name:     System.IsRead -- PKEY_IsRead
//  Type:     Boolean -- VT_BOOL
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 10
//
//  Has the item been read?
DEFINE_PROPERTYKEY(PKEY_IsRead, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 10);
#define INIT_PKEY_IsRead { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 10 }

//  Name:     System.IsSearchOnlyItem -- PKEY_IsSearchOnlyItem
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5D76B67F-9B3D-44BB-B6AE-25DA4F638A67}, 4
//
//  Identifies if a location or a library is search only
DEFINE_PROPERTYKEY(PKEY_IsSearchOnlyItem, 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67, 4);
#define INIT_PKEY_IsSearchOnlyItem { { 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67 }, 4 }

//  Name:     System.IsSendToTarget -- PKEY_IsSendToTarget
//  Type:     Boolean -- VT_BOOL
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 33
//
//  Provided by certain shell folders. Return TRUE if the folder is a valid Send To target.
DEFINE_PROPERTYKEY(PKEY_IsSendToTarget, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 33);
#define INIT_PKEY_IsSendToTarget { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 33 }

//  Name:     System.IsShared -- PKEY_IsShared
//  Type:     Boolean -- VT_BOOL
//  FormatID: {EF884C5B-2BFE-41BB-AAE5-76EEDF4F9902}, 100
//
//  Is this item shared?  This only checks for ACLs that are not inherited.
DEFINE_PROPERTYKEY(PKEY_IsShared, 0xEF884C5B, 0x2BFE, 0x41BB, 0xAA, 0xE5, 0x76, 0xEE, 0xDF, 0x4F, 0x99, 0x02, 100);
#define INIT_PKEY_IsShared { { 0xEF884C5B, 0x2BFE, 0x41BB, 0xAA, 0xE5, 0x76, 0xEE, 0xDF, 0x4F, 0x99, 0x02 }, 100 }

//  Name:     System.ItemAuthors -- PKEY_ItemAuthors
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D0A04F0A-462A-48A4-BB2F-3706E88DBD7D}, 100
//  
//  This is the generic list of authors associated with an item. 
//  
//  For example, the artist name for a track is the item author.
DEFINE_PROPERTYKEY(PKEY_ItemAuthors, 0xD0A04F0A, 0x462A, 0x48A4, 0xBB, 0x2F, 0x37, 0x06, 0xE8, 0x8D, 0xBD, 0x7D, 100);
#define INIT_PKEY_ItemAuthors { { 0xD0A04F0A, 0x462A, 0x48A4, 0xBB, 0x2F, 0x37, 0x06, 0xE8, 0x8D, 0xBD, 0x7D }, 100 }

//  Name:     System.ItemClassType -- PKEY_ItemClassType
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {048658AD-2DB8-41A4-BBB6-AC1EF1207EB1}, 100
DEFINE_PROPERTYKEY(PKEY_ItemClassType, 0x048658AD, 0x2DB8, 0x41A4, 0xBB, 0xB6, 0xAC, 0x1E, 0xF1, 0x20, 0x7E, 0xB1, 100);
#define INIT_PKEY_ItemClassType { { 0x048658AD, 0x2DB8, 0x41A4, 0xBB, 0xB6, 0xAC, 0x1E, 0xF1, 0x20, 0x7E, 0xB1 }, 100 }

//  Name:     System.ItemDate -- PKEY_ItemDate
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {F7DB74B4-4287-4103-AFBA-F1B13DCD75CF}, 100
//  
//  This is the main date for an item. The date of interest. 
//  
//  For example, for photos this maps to System.Photo.DateTaken.
DEFINE_PROPERTYKEY(PKEY_ItemDate, 0xF7DB74B4, 0x4287, 0x4103, 0xAF, 0xBA, 0xF1, 0xB1, 0x3D, 0xCD, 0x75, 0xCF, 100);
#define INIT_PKEY_ItemDate { { 0xF7DB74B4, 0x4287, 0x4103, 0xAF, 0xBA, 0xF1, 0xB1, 0x3D, 0xCD, 0x75, 0xCF }, 100 }

//  Name:     System.ItemFolderNameDisplay -- PKEY_ItemFolderNameDisplay
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 2 (PID_STG_DIRECTORY)
//  
//  This is the user-friendly display name of the parent folder of an item.
//  
//  If System.ItemFolderPathDisplay is VT_EMPTY, then this property should be too.  Otherwise, it 
//  should be derived appropriately by the data source from System.ItemFolderPathDisplay.
//  
//  If the folder is a file folder, the value will be localized if a localized name is available.
//  
//  Example values:
//  
//      If the path is...                     The property value is...
//      -----------------                     ------------------------
//      "c:\folder\subfolder\hello.txt"       "subfolder"
//      "\\server\share\mydir\goodnews.doc"   "mydir"
//      "\\server\share\numbers.xls"          "share"
//      "c:\folder\MyFolder"                  "folder"
//      "/Mailbox Account/Inbox/'Re: Hello!'" "Inbox"
DEFINE_PROPERTYKEY(PKEY_ItemFolderNameDisplay, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 2);
#define INIT_PKEY_ItemFolderNameDisplay { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 2 }

//  Name:     System.ItemFolderPathDisplay -- PKEY_ItemFolderPathDisplay
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 6
//  
//  This is the user-friendly display path of the parent folder of an item.
//  
//  If System.ItemPathDisplay is VT_EMPTY, then this property should be too.  Otherwise, it should 
//  be derived appropriately by the data source from System.ItemPathDisplay.
//  
//  Example values:
//  
//      If the path is...                     The property value is...
//      -----------------                     ------------------------
//      "c:\folder\subfolder\hello.txt"       "c:\folder\subfolder"
//      "\\server\share\mydir\goodnews.doc"   "\\server\share\mydir"
//      "\\server\share\numbers.xls"          "\\server\share"
//      "c:\folder\MyFolder"                  "c:\folder"
//      "/Mailbox Account/Inbox/'Re: Hello!'" "/Mailbox Account/Inbox"
DEFINE_PROPERTYKEY(PKEY_ItemFolderPathDisplay, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 6);
#define INIT_PKEY_ItemFolderPathDisplay { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 6 }

//  Name:     System.ItemFolderPathDisplayNarrow -- PKEY_ItemFolderPathDisplayNarrow
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {DABD30ED-0043-4789-A7F8-D013A4736622}, 100
//  
//  This is the user-friendly display path of the parent folder of an item.  The format of the string
//  should be tailored such that the folder name comes first, to optimize for a narrow viewing column.
//  
//  If the folder is a file folder, the value includes localized names if they are present.
//  
//  If System.ItemFolderPathDisplay is VT_EMPTY, then this property should be too.  Otherwise, it should
//  be derived appropriately by the data source from System.ItemFolderPathDisplay.
//  
//  Example values:
//  
//      If the path is...                     The property value is...
//      -----------------                     ------------------------
//      "c:\folder\subfolder\hello.txt"       "subfolder (c:\folder)"
//      "\\server\share\mydir\goodnews.doc"   "mydir (\\server\share)"
//      "\\server\share\numbers.xls"          "share (\\server)"
//      "c:\folder\MyFolder"                  "folder (c:\)"
//      "/Mailbox Account/Inbox/'Re: Hello!'" "Inbox (/Mailbox Account)"
DEFINE_PROPERTYKEY(PKEY_ItemFolderPathDisplayNarrow, 0xDABD30ED, 0x0043, 0x4789, 0xA7, 0xF8, 0xD0, 0x13, 0xA4, 0x73, 0x66, 0x22, 100);
#define INIT_PKEY_ItemFolderPathDisplayNarrow { { 0xDABD30ED, 0x0043, 0x4789, 0xA7, 0xF8, 0xD0, 0x13, 0xA4, 0x73, 0x66, 0x22 }, 100 }

//  Name:     System.ItemName -- PKEY_ItemName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {6B8DA074-3B5C-43BC-886F-0A2CDCE00B6F}, 100
//  
//  This is the base-name of the System.ItemNameDisplay.
//  
//  If the item is a file this property
//  includes the extension in all cases, and will be localized if a localized name is available.
//  
//  If the item is a message, then the value of this property does not include the forwarding or
//  reply prefixes (see System.ItemNamePrefix).
DEFINE_PROPERTYKEY(PKEY_ItemName, 0x6B8DA074, 0x3B5C, 0x43BC, 0x88, 0x6F, 0x0A, 0x2C, 0xDC, 0xE0, 0x0B, 0x6F, 100);
#define INIT_PKEY_ItemName { { 0x6B8DA074, 0x3B5C, 0x43BC, 0x88, 0x6F, 0x0A, 0x2C, 0xDC, 0xE0, 0x0B, 0x6F }, 100 }

//  Name:     System.ItemNameDisplay -- PKEY_ItemNameDisplay
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 10 (PID_STG_NAME)
//  
//  This is the display name in "most complete" form.  This is the best effort unique representation
//  of the name of an item that makes sense for end users to read.  It is the concatentation of
//  System.ItemNamePrefix and System.ItemName.
//  
//  If the item is a file this property
//  includes the extension in all cases, and will be localized if a localized name is available.
//  
//  There are acceptable cases when System.FileName is not VT_EMPTY, yet the value of this property 
//  is completely different.  Email messages are a key example.  If the item is an email message, 
//  the item name is likely the subject.  In that case, the value must be the concatenation of the
//  System.ItemNamePrefix and System.ItemName.  Since the value of System.ItemNamePrefix excludes
//  any trailing whitespace, the concatenation must include a whitespace when generating System.ItemNameDisplay.
//  
//  Note that this property is not guaranteed to be unique, but the idea is to promote the most likely
//  candidate that can be unique and also makes sense for end users. For example, for documents, you
//  might think about using System.Title as the System.ItemNameDisplay, but in practice the title of
//  the documents may not be useful or unique enough to be of value as the sole System.ItemNameDisplay.  
//  Instead, providing the value of System.FileName as the value of System.ItemNameDisplay is a better
//  candidate.  In Windows Mail, the emails are stored in the file system as .eml files and the 
//  System.FileName for those files are not human-friendly as they contain GUIDs. In this example, 
//  promoting System.Subject as System.ItemNameDisplay makes more sense.
//  
//  Compatibility notes:
//  
//  Shell folder implementations on Vista: use PKEY_ItemNameDisplay for the name column when
//  you want Explorer to call ISF::GetDisplayNameOf(SHGDN_NORMAL) to get the value of the name. Use
//  another PKEY (like PKEY_ItemName) when you want Explorer to call either the folder's property store or
//  ISF2::GetDetailsEx in order to get the value of the name.
//  
//  Shell folder implementations on XP: the first column needs to be the name column, and Explorer
//  will call ISF::GetDisplayNameOf to get the value of the name.  The PKEY/SCID does not matter.
//  
//  Example values:
//  
//      File:          "hello.txt"
//      Message:       "Re: Let's talk about Tom's argyle socks!"
//      Device folder: "song.wma"
//      Folder:        "Documents"
DEFINE_PROPERTYKEY(PKEY_ItemNameDisplay, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 10);
#define INIT_PKEY_ItemNameDisplay { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 10 }

//  Name:     System.ItemNameDisplayWithoutExtension -- PKEY_ItemNameDisplayWithoutExtension
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 24
//  
//  This is similar to System.ItemNameDisplay except that it never includes a file extension.
DEFINE_PROPERTYKEY(PKEY_ItemNameDisplayWithoutExtension, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 24);
#define INIT_PKEY_ItemNameDisplayWithoutExtension { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 24 }

//  Name:     System.ItemNamePrefix -- PKEY_ItemNamePrefix
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {D7313FF1-A77A-401C-8C99-3DBDD68ADD36}, 100
//  
//  This is the prefix of an item, used for email messages.
//  where the subject begins with "Re:" which is the prefix.
//  
//  If the item is a file, then the value of this property is VT_EMPTY.
//  
//  If the item is a message, then the value of this property is the forwarding or reply 
//  prefixes (including delimiting colon, but no whitespace), or VT_EMPTY if there is no prefix.
//  
//  Example values:
//  
//  System.ItemNamePrefix    System.ItemName      System.ItemNameDisplay
//  ---------------------    -------------------  ----------------------
//  VT_EMPTY                 "Great day"          "Great day"
//  "Re:"                    "Great day"          "Re: Great day"
//  "Fwd: "                  "Monthly budget"     "Fwd: Monthly budget"
//  VT_EMPTY                 "accounts.xls"       "accounts.xls"
DEFINE_PROPERTYKEY(PKEY_ItemNamePrefix, 0xD7313FF1, 0xA77A, 0x401C, 0x8C, 0x99, 0x3D, 0xBD, 0xD6, 0x8A, 0xDD, 0x36, 100);
#define INIT_PKEY_ItemNamePrefix { { 0xD7313FF1, 0xA77A, 0x401C, 0x8C, 0x99, 0x3D, 0xBD, 0xD6, 0x8A, 0xDD, 0x36 }, 100 }

//  Name:     System.ItemNameSortOverride -- PKEY_ItemNameSortOverride
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 23
//  
//  This string should be set to the phonetic version of the display name as defined in System.ItemNameDisplay in CJK locales
//  (CHS Pinyin, JPN Hiragana, KOR Hangul, etc.). The first character of this field is also used for grouping names by first
//  letter. For most non-CJK languages, this field does not need to be set (in which case System.ItemNameDisplay will be used).
//  However, if it is desirable to override the grouping letter (for example, to remove leading articles such as "a" and "the"),
//  an alternate string can be provided here.
DEFINE_PROPERTYKEY(PKEY_ItemNameSortOverride, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 23);
#define INIT_PKEY_ItemNameSortOverride { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 23 }

//  Name:     System.ItemParticipants -- PKEY_ItemParticipants
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D4D0AA16-9948-41A4-AA85-D97FF9646993}, 100
//  
//  This is the generic list of people associated with an item and who contributed 
//  to the item. 
//  
//  For example, this is the combination of people in the To list, Cc list and 
//  sender of an email message.
DEFINE_PROPERTYKEY(PKEY_ItemParticipants, 0xD4D0AA16, 0x9948, 0x41A4, 0xAA, 0x85, 0xD9, 0x7F, 0xF9, 0x64, 0x69, 0x93, 100);
#define INIT_PKEY_ItemParticipants { { 0xD4D0AA16, 0x9948, 0x41A4, 0xAA, 0x85, 0xD9, 0x7F, 0xF9, 0x64, 0x69, 0x93 }, 100 }

//  Name:     System.ItemPathDisplay -- PKEY_ItemPathDisplay
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 7
//  
//  This is the user-friendly display path to the item.
//  
//  If the item is a file or folder this property
//  includes the extension in all cases, and will be localized if a localized name is available.
//  
//  For other items,this is the user-friendly equivalent, assuming the item exists in hierarchical storage.
//  
//  Unlike System.ItemUrl, this property value does not include the URL scheme.
//  
//  To parse an item path, use System.ItemUrl or System.ParsingPath.  To reference shell 
//  namespace items using shell APIs, use System.ParsingPath.
//  
//  Example values:
//  
//      If the path is...                     The property value is...
//      -----------------                     ------------------------
//      "c:\folder\subfolder\hello.txt"       "c:\folder\subfolder\hello.txt"
//      "\\server\share\mydir\goodnews.doc"   "\\server\share\mydir\goodnews.doc"
//      "\\server\share\numbers.xls"          "\\server\share\numbers.xls"
//      "c:\folder\MyFolder"                  "c:\folder\MyFolder"
//      "/Mailbox Account/Inbox/'Re: Hello!'" "/Mailbox Account/Inbox/'Re: Hello!'"
DEFINE_PROPERTYKEY(PKEY_ItemPathDisplay, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 7);
#define INIT_PKEY_ItemPathDisplay { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 7 }

//  Name:     System.ItemPathDisplayNarrow -- PKEY_ItemPathDisplayNarrow
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 8
//  
//  This is the user-friendly display path to the item. The format of the string should be 
//  tailored such that the name comes first, to optimize for a narrow viewing column.
//  
//  If the item is a file, the value excludes the file extension, and includes localized names if they are present.
//  If the item is a message, the value includes the System.ItemNamePrefix.
//  
//  To parse an item path, use System.ItemUrl or System.ParsingPath.
//  
//  Example values:
//  
//      If the path is...                     The property value is...
//      -----------------                     ------------------------
//      "c:\folder\subfolder\hello.txt"       "hello (c:\folder\subfolder)"
//      "\\server\share\mydir\goodnews.doc"   "goodnews (\\server\share\mydir)"
//      "\\server\share\folder"               "folder (\\server\share)"
//      "c:\folder\MyFolder"                  "MyFolder (c:\folder)"
//      "/Mailbox Account/Inbox/'Re: Hello!'" "Re: Hello! (/Mailbox Account/Inbox)"
DEFINE_PROPERTYKEY(PKEY_ItemPathDisplayNarrow, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 8);
#define INIT_PKEY_ItemPathDisplayNarrow { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 8 }

//  Name:     System.ItemSubType -- PKEY_ItemSubType
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 37
//  
//  Describes the sub-type of an item. The value is intended for display to the user.
//  
//  In contrast to System.ItemType, which is generally used to describe a class of items
//  that all have the same common content format, System.ItemSubType may differ based on
//  the individual contents or purpose of the item.
//  
//  For example, this property may be used to identify an item with System.ItemType = "jpg"
//  as System.ItemSubType = "Panorama" or System.ItemSubType = "Smart Shot".
DEFINE_PROPERTYKEY(PKEY_ItemSubType, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 37);
#define INIT_PKEY_ItemSubType { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 37 }

//  Name:     System.ItemType -- PKEY_ItemType
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 11
//  
//  This is the canonical type of the item and is intended to be programmatically
//  parsed.
//  
//  If there is no canonical type, the value is VT_EMPTY.
//  
//  If the item is a file (ie, System.FileName is not VT_EMPTY), the value is the same as
//  System.FileExtension.
//  
//  Use System.ItemTypeText when you want to display the type to end users in a view.  (If
//   the item is a file, passing the System.ItemType value to PSFormatForDisplay will
//   result in the same value as System.ItemTypeText.)
//  
//  Example values:
//  
//      If the path is...                     The property value is...
//      -----------------                     ------------------------
//      "c:\folder\subfolder\hello.txt"       ".txt"
//      "\\server\share\mydir\goodnews.doc"   ".doc"
//      "\\server\share\folder"               "Directory"
//      "c:\folder\MyFolder"                  "Directory"
//      [desktop]                             "Folder"
//      "/Mailbox Account/Inbox/'Re: Hello!'" "MAPI/IPM.Message"
DEFINE_PROPERTYKEY(PKEY_ItemType, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 11);
#define INIT_PKEY_ItemType { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 11 }

//  Name:     System.ItemTypeText -- PKEY_ItemTypeText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 4 (PID_STG_STORAGETYPE)
//  
//  This is the user friendly type name of the item.  This is not intended to be
//  programmatically parsed.
//  
//  If System.ItemType is VT_EMPTY, the value of this property is also VT_EMPTY.
//  
//  If the item is a file, the value of this property is the same as if you passed the 
//  file's System.ItemType value to PSFormatForDisplay.
//  
//  This property should not be confused with System.Kind, where System.Kind is a high-level
//  user friendly kind name. For example, for a document, System.Kind = "Document" and 
//  System.Item.Type = ".doc" and System.Item.TypeText = "Microsoft Word Document"
//  
//  Example values:
//  
//      If the path is...                     The property value is...
//      -----------------                     ------------------------
//      "c:\folder\subfolder\hello.txt"       "Text File"
//      "\\server\share\mydir\goodnews.doc"   "Microsoft Word Document"
//      "\\server\share\folder"               "File Folder"
//      "c:\folder\MyFolder"                  "File Folder"
//      "/Mailbox Account/Inbox/'Re: Hello!'" "Outlook E-Mail Message"
DEFINE_PROPERTYKEY(PKEY_ItemTypeText, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 4);
#define INIT_PKEY_ItemTypeText { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 4 }

//  Name:     System.ItemUrl -- PKEY_ItemUrl
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Query) {49691C90-7E17-101A-A91C-08002B2ECDA9}, 9 (DISPID_QUERY_VIRTUALPATH)
//  
//  This always represents a well formed URL that points to the item.  
//  
//  To reference shell namespace items using shell APIs, use System.ParsingPath.
//  
//  Example values:
//  
//      Files:    "file:///c:/folder/subfolder/hello.txt"
//                "csc://{GUID}/..."
//      Messages: "mapi://..."
DEFINE_PROPERTYKEY(PKEY_ItemUrl, 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9, 9);
#define INIT_PKEY_ItemUrl { { 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9 }, 9 }

//  Name:     System.Keywords -- PKEY_Keywords
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)  Legacy code may treat this as VT_LPSTR.
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 5 (PIDSI_KEYWORDS)
//
//  The keywords for the item.  Also referred to as tags.
DEFINE_PROPERTYKEY(PKEY_Keywords, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 5);
#define INIT_PKEY_Keywords { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 5 }

//  Name:     System.Kind -- PKEY_Kind
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {1E3EE840-BC2B-476C-8237-2ACD1A839B22}, 3
//  
//  System.Kind is used to map extensions to various .Search folders.
//  Extensions are mapped to Kinds at HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Explorer\KindMap
//  The list of kinds is not extensible.
DEFINE_PROPERTYKEY(PKEY_Kind, 0x1E3EE840, 0xBC2B, 0x476C, 0x82, 0x37, 0x2A, 0xCD, 0x1A, 0x83, 0x9B, 0x22, 3);
#define INIT_PKEY_Kind { { 0x1E3EE840, 0xBC2B, 0x476C, 0x82, 0x37, 0x2A, 0xCD, 0x1A, 0x83, 0x9B, 0x22 }, 3 }

// Possible discrete values for PKEY_Kind are:
#define KIND_CALENDAR                       L"calendar"
#define KIND_COMMUNICATION                  L"communication"
#define KIND_CONTACT                        L"contact"
#define KIND_DOCUMENT                       L"document"
#define KIND_EMAIL                          L"email"
#define KIND_FEED                           L"feed"
#define KIND_FOLDER                         L"folder"
#define KIND_GAME                           L"game"
#define KIND_INSTANTMESSAGE                 L"instantmessage"
#define KIND_JOURNAL                        L"journal"
#define KIND_LINK                           L"link"
#define KIND_MOVIE                          L"movie"
#define KIND_MUSIC                          L"music"
#define KIND_NOTE                           L"note"
#define KIND_PICTURE                        L"picture"
#define KIND_PLAYLIST                       L"playlist"
#define KIND_PROGRAM                        L"program"
#define KIND_RECORDEDTV                     L"recordedtv"
#define KIND_SEARCHFOLDER                   L"searchfolder"
#define KIND_TASK                           L"task"
#define KIND_VIDEO                          L"video"
#define KIND_WEBHISTORY                     L"webhistory"
#define KIND_UNKNOWN                        L"unknown"

//  Name:     System.KindText -- PKEY_KindText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {F04BEF95-C585-4197-A2B7-DF46FDC9EE6D}, 100
//  
//  This is the user-friendly form of System.Kind.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_KindText, 0xF04BEF95, 0xC585, 0x4197, 0xA2, 0xB7, 0xDF, 0x46, 0xFD, 0xC9, 0xEE, 0x6D, 100);
#define INIT_PKEY_KindText { { 0xF04BEF95, 0xC585, 0x4197, 0xA2, 0xB7, 0xDF, 0x46, 0xFD, 0xC9, 0xEE, 0x6D }, 100 }

//  Name:     System.Language -- PKEY_Language
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 28
//
//  
DEFINE_PROPERTYKEY(PKEY_Language, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 28);
#define INIT_PKEY_Language { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 28 }

//  Name:     System.LastSyncError -- PKEY_LastSyncError
//  Type:     UInt32 -- VT_UI4
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 107
DEFINE_PROPERTYKEY(PKEY_LastSyncError, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 107);
#define INIT_PKEY_LastSyncError { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 107 }

//  Name:     System.LastSyncWarning -- PKEY_LastSyncWarning
//  Type:     UInt32 -- VT_UI4
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 128
DEFINE_PROPERTYKEY(PKEY_LastSyncWarning, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 128);
#define INIT_PKEY_LastSyncWarning { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 128 }

//  Name:     System.LastWriterPackageFamilyName -- PKEY_LastWriterPackageFamilyName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {502CFEAB-47EB-459C-B960-E6D8728F7701}, 101
//
//  Mark of the app container. The package family name of the last app to edit the file's contents.
DEFINE_PROPERTYKEY(PKEY_LastWriterPackageFamilyName, 0x502CFEAB, 0x47EB, 0x459C, 0xB9, 0x60, 0xE6, 0xD8, 0x72, 0x8F, 0x77, 0x01, 101);
#define INIT_PKEY_LastWriterPackageFamilyName { { 0x502CFEAB, 0x47EB, 0x459C, 0xB9, 0x60, 0xE6, 0xD8, 0x72, 0x8F, 0x77, 0x01 }, 101 }

//  Name:     System.LowKeywords -- PKEY_LowKeywords
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 25
//
//  The low confidence keywords for the item.
DEFINE_PROPERTYKEY(PKEY_LowKeywords, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 25);
#define INIT_PKEY_LowKeywords { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 25 }

//  Name:     System.MediumKeywords -- PKEY_MediumKeywords
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 26
//
//  The medium confidence keywords for the item.
DEFINE_PROPERTYKEY(PKEY_MediumKeywords, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 26);
#define INIT_PKEY_MediumKeywords { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 26 }

//  Name:     System.MileageInformation -- PKEY_MileageInformation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FDF84370-031A-4ADD-9E91-0D775F1C6605}, 100
DEFINE_PROPERTYKEY(PKEY_MileageInformation, 0xFDF84370, 0x031A, 0x4ADD, 0x9E, 0x91, 0x0D, 0x77, 0x5F, 0x1C, 0x66, 0x05, 100);
#define INIT_PKEY_MileageInformation { { 0xFDF84370, 0x031A, 0x4ADD, 0x9E, 0x91, 0x0D, 0x77, 0x5F, 0x1C, 0x66, 0x05 }, 100 }

//  Name:     System.MIMEType -- PKEY_MIMEType
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {0B63E350-9CCC-11D0-BCDB-00805FCCCE04}, 5
//
//  The MIME type.  Eg, for EML files: 'message/rfc822'.
DEFINE_PROPERTYKEY(PKEY_MIMEType, 0x0B63E350, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04, 5);
#define INIT_PKEY_MIMEType { { 0x0B63E350, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04 }, 5 }

//  Name:     System.Null -- PKEY_Null
//  Type:     Null -- VT_NULL
//  FormatID: {00000000-0000-0000-0000-000000000000}, 0
DEFINE_PROPERTYKEY(PKEY_Null, 0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0);
#define INIT_PKEY_Null { { 0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 }, 0 }

//  Name:     System.OfflineAvailability -- PKEY_OfflineAvailability
//  Type:     UInt32 -- VT_UI4
//  FormatID: {A94688B6-7D9F-4570-A648-E3DFC0AB2B3F}, 100
DEFINE_PROPERTYKEY(PKEY_OfflineAvailability, 0xA94688B6, 0x7D9F, 0x4570, 0xA6, 0x48, 0xE3, 0xDF, 0xC0, 0xAB, 0x2B, 0x3F, 100);
#define INIT_PKEY_OfflineAvailability { { 0xA94688B6, 0x7D9F, 0x4570, 0xA6, 0x48, 0xE3, 0xDF, 0xC0, 0xAB, 0x2B, 0x3F }, 100 }

// Possible discrete values for PKEY_OfflineAvailability are:
#define OFFLINEAVAILABILITY_NOT_AVAILABLE   0ul
#define OFFLINEAVAILABILITY_AVAILABLE       1ul
#define OFFLINEAVAILABILITY_ALWAYS_AVAILABLE 2ul

//  Name:     System.OfflineStatus -- PKEY_OfflineStatus
//  Type:     UInt32 -- VT_UI4
//  FormatID: {6D24888F-4718-4BDA-AFED-EA0FB4386CD8}, 100
DEFINE_PROPERTYKEY(PKEY_OfflineStatus, 0x6D24888F, 0x4718, 0x4BDA, 0xAF, 0xED, 0xEA, 0x0F, 0xB4, 0x38, 0x6C, 0xD8, 100);
#define INIT_PKEY_OfflineStatus { { 0x6D24888F, 0x4718, 0x4BDA, 0xAF, 0xED, 0xEA, 0x0F, 0xB4, 0x38, 0x6C, 0xD8 }, 100 }

// Possible discrete values for PKEY_OfflineStatus are:
#define OFFLINESTATUS_ONLINE                0ul
#define OFFLINESTATUS_OFFLINE               1ul
#define OFFLINESTATUS_OFFLINE_FORCED        2ul
#define OFFLINESTATUS_OFFLINE_SLOW          3ul
#define OFFLINESTATUS_OFFLINE_ERROR         4ul
#define OFFLINESTATUS_OFFLINE_ITEM_VERSION_CONFLICT 5ul
#define OFFLINESTATUS_OFFLINE_SUSPENDED     6ul

//  Name:     System.OriginalFileName -- PKEY_OriginalFileName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSFMTID_VERSION) {0CEF7D53-FA64-11D1-A203-0000F81FEDEE}, 6
//  
//  
DEFINE_PROPERTYKEY(PKEY_OriginalFileName, 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE, 6);
#define INIT_PKEY_OriginalFileName { { 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE }, 6 }

//  Name:     System.OwnerSID -- PKEY_OwnerSID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {5D76B67F-9B3D-44BB-B6AE-25DA4F638A67}, 6
//
//  SID of the user that owns the library.
DEFINE_PROPERTYKEY(PKEY_OwnerSID, 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67, 6);
#define INIT_PKEY_OwnerSID { { 0x5D76B67F, 0x9B3D, 0x44BB, 0xB6, 0xAE, 0x25, 0xDA, 0x4F, 0x63, 0x8A, 0x67 }, 6 }

//  Name:     System.ParentalRating -- PKEY_ParentalRating
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 21 (PIDMSI_PARENTAL_RATING)
//
//  
DEFINE_PROPERTYKEY(PKEY_ParentalRating, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 21);
#define INIT_PKEY_ParentalRating { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 21 }

//  Name:     System.ParentalRatingReason -- PKEY_ParentalRatingReason
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {10984E0A-F9F2-4321-B7EF-BAF195AF4319}, 100
DEFINE_PROPERTYKEY(PKEY_ParentalRatingReason, 0x10984E0A, 0xF9F2, 0x4321, 0xB7, 0xEF, 0xBA, 0xF1, 0x95, 0xAF, 0x43, 0x19, 100);
#define INIT_PKEY_ParentalRatingReason { { 0x10984E0A, 0xF9F2, 0x4321, 0xB7, 0xEF, 0xBA, 0xF1, 0x95, 0xAF, 0x43, 0x19 }, 100 }

//  Name:     System.ParentalRatingsOrganization -- PKEY_ParentalRatingsOrganization
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A7FE0840-1344-46F0-8D37-52ED712A4BF9}, 100
DEFINE_PROPERTYKEY(PKEY_ParentalRatingsOrganization, 0xA7FE0840, 0x1344, 0x46F0, 0x8D, 0x37, 0x52, 0xED, 0x71, 0x2A, 0x4B, 0xF9, 100);
#define INIT_PKEY_ParentalRatingsOrganization { { 0xA7FE0840, 0x1344, 0x46F0, 0x8D, 0x37, 0x52, 0xED, 0x71, 0x2A, 0x4B, 0xF9 }, 100 }

//  Name:     System.ParsingBindContext -- PKEY_ParsingBindContext
//  Type:     Any -- VT_NULL  Legacy code may treat this as VT_UNKNOWN.
//  FormatID: {DFB9A04D-362F-4CA3-B30B-0254B17B5B84}, 100
//  
//  used to get the IBindCtx for an item for parsing
DEFINE_PROPERTYKEY(PKEY_ParsingBindContext, 0xDFB9A04D, 0x362F, 0x4CA3, 0xB3, 0x0B, 0x02, 0x54, 0xB1, 0x7B, 0x5B, 0x84, 100);
#define INIT_PKEY_ParsingBindContext { { 0xDFB9A04D, 0x362F, 0x4CA3, 0xB3, 0x0B, 0x02, 0x54, 0xB1, 0x7B, 0x5B, 0x84 }, 100 }

//  Name:     System.ParsingName -- PKEY_ParsingName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 24
//  
//  The shell namespace name of an item relative to a parent folder.  This name may be passed to 
//  IShellFolder::ParseDisplayName() of the parent shell folder.
DEFINE_PROPERTYKEY(PKEY_ParsingName, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 24);
#define INIT_PKEY_ParsingName { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 24 }

//  Name:     System.ParsingPath -- PKEY_ParsingPath
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 30
//  
//  This is the shell namespace path to the item.  This path may be passed to 
//  SHParseDisplayName to parse the path to the correct shell folder.
//  
//  If the item cannot be accessed through the shell namespace, this value is VT_EMPTY.
DEFINE_PROPERTYKEY(PKEY_ParsingPath, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 30);
#define INIT_PKEY_ParsingPath { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 30 }

//  Name:     System.PerceivedType -- PKEY_PerceivedType
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 9
//
//  The perceived type of a shell item, based upon its canonical type.
DEFINE_PROPERTYKEY(PKEY_PerceivedType, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 9);
#define INIT_PKEY_PerceivedType { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 9 }

// For the enumerated values of PKEY_PerceivedType, see the PERCEIVED_TYPE_* values in shtypes.idl.

//  Name:     System.PercentFull -- PKEY_PercentFull
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_Volume) {9B174B35-40FF-11D2-A27E-00C04FC30871}, 5  (Filesystem Volume Properties)
//
//  The amount filled as a percentage, multiplied by 100 (ie, the valid range is 0 through 100).
DEFINE_PROPERTYKEY(PKEY_PercentFull, 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71, 5);
#define INIT_PKEY_PercentFull { { 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71 }, 5 }

//  Name:     System.Priority -- PKEY_Priority
//  Type:     UInt16 -- VT_UI2
//  FormatID: {9C1FCF74-2D97-41BA-B4AE-CB2E3661A6E4}, 5
//
//  
DEFINE_PROPERTYKEY(PKEY_Priority, 0x9C1FCF74, 0x2D97, 0x41BA, 0xB4, 0xAE, 0xCB, 0x2E, 0x36, 0x61, 0xA6, 0xE4, 5);
#define INIT_PKEY_Priority { { 0x9C1FCF74, 0x2D97, 0x41BA, 0xB4, 0xAE, 0xCB, 0x2E, 0x36, 0x61, 0xA6, 0xE4 }, 5 }

// Possible discrete values for PKEY_Priority are:
#define PRIORITY_PROP_LOW                   0u
#define PRIORITY_PROP_NORMAL                1u
#define PRIORITY_PROP_HIGH                  2u

//  Name:     System.PriorityText -- PKEY_PriorityText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {D98BE98B-B86B-4095-BF52-9D23B2E0A752}, 100
//  
//  This is the user-friendly form of System.Priority.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_PriorityText, 0xD98BE98B, 0xB86B, 0x4095, 0xBF, 0x52, 0x9D, 0x23, 0xB2, 0xE0, 0xA7, 0x52, 100);
#define INIT_PKEY_PriorityText { { 0xD98BE98B, 0xB86B, 0x4095, 0xBF, 0x52, 0x9D, 0x23, 0xB2, 0xE0, 0xA7, 0x52 }, 100 }

//  Name:     System.Project -- PKEY_Project
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {39A7F922-477C-48DE-8BC8-B28441E342E3}, 100
DEFINE_PROPERTYKEY(PKEY_Project, 0x39A7F922, 0x477C, 0x48DE, 0x8B, 0xC8, 0xB2, 0x84, 0x41, 0xE3, 0x42, 0xE3, 100);
#define INIT_PKEY_Project { { 0x39A7F922, 0x477C, 0x48DE, 0x8B, 0xC8, 0xB2, 0x84, 0x41, 0xE3, 0x42, 0xE3 }, 100 }

//  Name:     System.ProviderItemID -- PKEY_ProviderItemID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {F21D9941-81F0-471A-ADEE-4E74B49217ED}, 100
//  
//  
DEFINE_PROPERTYKEY(PKEY_ProviderItemID, 0xF21D9941, 0x81F0, 0x471A, 0xAD, 0xEE, 0x4E, 0x74, 0xB4, 0x92, 0x17, 0xED, 100);
#define INIT_PKEY_ProviderItemID { { 0xF21D9941, 0x81F0, 0x471A, 0xAD, 0xEE, 0x4E, 0x74, 0xB4, 0x92, 0x17, 0xED }, 100 }

//  Name:     System.Rating -- PKEY_Rating
//  Type:     UInt32 -- VT_UI4
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 9 (PIDMSI_RATING)
//  
//  Indicates the users preference rating of an item on a scale of 1-99 (1-12 = One Star, 
//  13-37 = Two Stars, 38-62 = Three Stars, 63-87 = Four Stars, 88-99 = Five Stars).
DEFINE_PROPERTYKEY(PKEY_Rating, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 9);
#define INIT_PKEY_Rating { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 9 }

// Use the following constants to convert between visual stars and the ratings value:
#define RATING_ONE_STAR_MIN                 1ul
#define RATING_ONE_STAR_SET                 1ul
#define RATING_ONE_STAR_MAX                 12ul

#define RATING_TWO_STARS_MIN                13ul
#define RATING_TWO_STARS_SET                25ul
#define RATING_TWO_STARS_MAX                37ul

#define RATING_THREE_STARS_MIN              38ul
#define RATING_THREE_STARS_SET              50ul
#define RATING_THREE_STARS_MAX              62ul

#define RATING_FOUR_STARS_MIN               63ul
#define RATING_FOUR_STARS_SET               75ul
#define RATING_FOUR_STARS_MAX               87ul

#define RATING_FIVE_STARS_MIN               88ul
#define RATING_FIVE_STARS_SET               99ul
#define RATING_FIVE_STARS_MAX               99ul


//  Name:     System.RatingText -- PKEY_RatingText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {90197CA7-FD8F-4E8C-9DA3-B57E1E609295}, 100
//  
//  This is the user-friendly form of System.Rating.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_RatingText, 0x90197CA7, 0xFD8F, 0x4E8C, 0x9D, 0xA3, 0xB5, 0x7E, 0x1E, 0x60, 0x92, 0x95, 100);
#define INIT_PKEY_RatingText { { 0x90197CA7, 0xFD8F, 0x4E8C, 0x9D, 0xA3, 0xB5, 0x7E, 0x1E, 0x60, 0x92, 0x95 }, 100 }

//  Name:     System.RemoteConflictingFile -- PKEY_RemoteConflictingFile
//  Type:     Object -- VT_UNKNOWN
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 115
DEFINE_PROPERTYKEY(PKEY_RemoteConflictingFile, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 115);
#define INIT_PKEY_RemoteConflictingFile { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 115 }

//  Name:     System.Security.AllowedEnterpriseDataProtectionIdentities -- PKEY_Security_AllowedEnterpriseDataProtectionIdentities
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {38D43380-D418-4830-84D5-46935A81C5C6}, 32
//
//  Encryption options
DEFINE_PROPERTYKEY(PKEY_Security_AllowedEnterpriseDataProtectionIdentities, 0x38D43380, 0xD418, 0x4830, 0x84, 0xD5, 0x46, 0x93, 0x5A, 0x81, 0xC5, 0xC6, 32);
#define INIT_PKEY_Security_AllowedEnterpriseDataProtectionIdentities { { 0x38D43380, 0xD418, 0x4830, 0x84, 0xD5, 0x46, 0x93, 0x5A, 0x81, 0xC5, 0xC6 }, 32 }

//  Name:     System.Security.EncryptionOwners -- PKEY_Security_EncryptionOwners
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {5F5AFF6A-37E5-4780-97EA-80C7565CF535}, 34
//
//  File ownership
DEFINE_PROPERTYKEY(PKEY_Security_EncryptionOwners, 0x5F5AFF6A, 0x37E5, 0x4780, 0x97, 0xEA, 0x80, 0xC7, 0x56, 0x5C, 0xF5, 0x35, 34);
#define INIT_PKEY_Security_EncryptionOwners { { 0x5F5AFF6A, 0x37E5, 0x4780, 0x97, 0xEA, 0x80, 0xC7, 0x56, 0x5C, 0xF5, 0x35 }, 34 }

//  Name:     System.Security.EncryptionOwnersDisplay -- PKEY_Security_EncryptionOwnersDisplay
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {DE621B8F-E125-43A3-A32D-5665446D632A}, 25
//
//  File ownership
DEFINE_PROPERTYKEY(PKEY_Security_EncryptionOwnersDisplay, 0xDE621B8F, 0xE125, 0x43A3, 0xA3, 0x2D, 0x56, 0x65, 0x44, 0x6D, 0x63, 0x2A, 25);
#define INIT_PKEY_Security_EncryptionOwnersDisplay { { 0xDE621B8F, 0xE125, 0x43A3, 0xA3, 0x2D, 0x56, 0x65, 0x44, 0x6D, 0x63, 0x2A }, 25 }

//  Name:     System.Sensitivity -- PKEY_Sensitivity
//  Type:     UInt16 -- VT_UI2
//  FormatID: {F8D3F6AC-4874-42CB-BE59-AB454B30716A}, 100
//
//  
DEFINE_PROPERTYKEY(PKEY_Sensitivity, 0xF8D3F6AC, 0x4874, 0x42CB, 0xBE, 0x59, 0xAB, 0x45, 0x4B, 0x30, 0x71, 0x6A, 100);
#define INIT_PKEY_Sensitivity { { 0xF8D3F6AC, 0x4874, 0x42CB, 0xBE, 0x59, 0xAB, 0x45, 0x4B, 0x30, 0x71, 0x6A }, 100 }

// Possible discrete values for PKEY_Sensitivity are:
#define SENSITIVITY_PROP_NORMAL             0u
#define SENSITIVITY_PROP_PERSONAL           1u
#define SENSITIVITY_PROP_PRIVATE            2u
#define SENSITIVITY_PROP_CONFIDENTIAL       3u

//  Name:     System.SensitivityText -- PKEY_SensitivityText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {D0C7F054-3F72-4725-8527-129A577CB269}, 100
//  
//  This is the user-friendly form of System.Sensitivity.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_SensitivityText, 0xD0C7F054, 0x3F72, 0x4725, 0x85, 0x27, 0x12, 0x9A, 0x57, 0x7C, 0xB2, 0x69, 100);
#define INIT_PKEY_SensitivityText { { 0xD0C7F054, 0x3F72, 0x4725, 0x85, 0x27, 0x12, 0x9A, 0x57, 0x7C, 0xB2, 0x69 }, 100 }

//  Name:     System.SFGAOFlags -- PKEY_SFGAOFlags
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 25
//
//  IShellFolder::GetAttributesOf flags, with SFGAO_PKEYSFGAOMASK attributes masked out.
DEFINE_PROPERTYKEY(PKEY_SFGAOFlags, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 25);
#define INIT_PKEY_SFGAOFlags { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 25 }

//  Name:     System.SharedWith -- PKEY_SharedWith
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {EF884C5B-2BFE-41BB-AAE5-76EEDF4F9902}, 200
//
//  Who is the item shared with?
DEFINE_PROPERTYKEY(PKEY_SharedWith, 0xEF884C5B, 0x2BFE, 0x41BB, 0xAA, 0xE5, 0x76, 0xEE, 0xDF, 0x4F, 0x99, 0x02, 200);
#define INIT_PKEY_SharedWith { { 0xEF884C5B, 0x2BFE, 0x41BB, 0xAA, 0xE5, 0x76, 0xEE, 0xDF, 0x4F, 0x99, 0x02 }, 200 }

//  Name:     System.ShareUserRating -- PKEY_ShareUserRating
//  Type:     UInt32 -- VT_UI4
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 12 (PIDMSI_SHARE_USER_RATING)
//
//  
DEFINE_PROPERTYKEY(PKEY_ShareUserRating, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 12);
#define INIT_PKEY_ShareUserRating { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 12 }

//  Name:     System.SharingStatus -- PKEY_SharingStatus
//  Type:     UInt32 -- VT_UI4
//  FormatID: {EF884C5B-2BFE-41BB-AAE5-76EEDF4F9902}, 300
//
//  What is the item's sharing status (not shared, shared, everyone (homegroup or everyone), or private)?
DEFINE_PROPERTYKEY(PKEY_SharingStatus, 0xEF884C5B, 0x2BFE, 0x41BB, 0xAA, 0xE5, 0x76, 0xEE, 0xDF, 0x4F, 0x99, 0x02, 300);
#define INIT_PKEY_SharingStatus { { 0xEF884C5B, 0x2BFE, 0x41BB, 0xAA, 0xE5, 0x76, 0xEE, 0xDF, 0x4F, 0x99, 0x02 }, 300 }

// Possible discrete values for PKEY_SharingStatus are:
#define SHARINGSTATUS_NOTSHARED             0ul
#define SHARINGSTATUS_SHARED                1ul
#define SHARINGSTATUS_PRIVATE               2ul

//  Name:     System.Shell.OmitFromView -- PKEY_Shell_OmitFromView
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {DE35258C-C695-4CBC-B982-38B0AD24CED0}, 2
//  
//  Set this to a string value of 'True' to omit this item from shell views
DEFINE_PROPERTYKEY(PKEY_Shell_OmitFromView, 0xDE35258C, 0xC695, 0x4CBC, 0xB9, 0x82, 0x38, 0xB0, 0xAD, 0x24, 0xCE, 0xD0, 2);
#define INIT_PKEY_Shell_OmitFromView { { 0xDE35258C, 0xC695, 0x4CBC, 0xB9, 0x82, 0x38, 0xB0, 0xAD, 0x24, 0xCE, 0xD0 }, 2 }

//  Name:     System.SimpleRating -- PKEY_SimpleRating
//  Type:     UInt32 -- VT_UI4
//  FormatID: {A09F084E-AD41-489F-8076-AA5BE3082BCA}, 100
//  
//  Indicates the users preference rating of an item on a scale of 0-5 (0=unrated, 1=One Star, 2=Two Stars, 3=Three Stars,
//  4=Four Stars, 5=Five Stars)
DEFINE_PROPERTYKEY(PKEY_SimpleRating, 0xA09F084E, 0xAD41, 0x489F, 0x80, 0x76, 0xAA, 0x5B, 0xE3, 0x08, 0x2B, 0xCA, 100);
#define INIT_PKEY_SimpleRating { { 0xA09F084E, 0xAD41, 0x489F, 0x80, 0x76, 0xAA, 0x5B, 0xE3, 0x08, 0x2B, 0xCA }, 100 }

//  Name:     System.Size -- PKEY_Size
//  Type:     UInt64 -- VT_UI8
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 12 (PID_STG_SIZE)
//
//  
DEFINE_PROPERTYKEY(PKEY_Size, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 12);
#define INIT_PKEY_Size { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 12 }

//  Name:     System.SoftwareUsed -- PKEY_SoftwareUsed
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 305
//
//  PropertyTagSoftwareUsed
DEFINE_PROPERTYKEY(PKEY_SoftwareUsed, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 305);
#define INIT_PKEY_SoftwareUsed { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 305 }

//  Name:     System.SourceItem -- PKEY_SourceItem
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {668CDFA5-7A1B-4323-AE4B-E527393A1D81}, 100
DEFINE_PROPERTYKEY(PKEY_SourceItem, 0x668CDFA5, 0x7A1B, 0x4323, 0xAE, 0x4B, 0xE5, 0x27, 0x39, 0x3A, 0x1D, 0x81, 100);
#define INIT_PKEY_SourceItem { { 0x668CDFA5, 0x7A1B, 0x4323, 0xAE, 0x4B, 0xE5, 0x27, 0x39, 0x3A, 0x1D, 0x81 }, 100 }

//  Name:     System.SourcePackageFamilyName -- PKEY_SourcePackageFamilyName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FFAE9DB7-1C8D-43FF-818C-84403AA3732D}, 100
//
//  Package family name of the app which the storage item instance originated.
DEFINE_PROPERTYKEY(PKEY_SourcePackageFamilyName, 0xFFAE9DB7, 0x1C8D, 0x43FF, 0x81, 0x8C, 0x84, 0x40, 0x3A, 0xA3, 0x73, 0x2D, 100);
#define INIT_PKEY_SourcePackageFamilyName { { 0xFFAE9DB7, 0x1C8D, 0x43FF, 0x81, 0x8C, 0x84, 0x40, 0x3A, 0xA3, 0x73, 0x2D }, 100 }

//  Name:     System.StartDate -- PKEY_StartDate
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {48FD6EC8-8A12-4CDF-A03E-4EC5A511EDDE}, 100
DEFINE_PROPERTYKEY(PKEY_StartDate, 0x48FD6EC8, 0x8A12, 0x4CDF, 0xA0, 0x3E, 0x4E, 0xC5, 0xA5, 0x11, 0xED, 0xDE, 100);
#define INIT_PKEY_StartDate { { 0x48FD6EC8, 0x8A12, 0x4CDF, 0xA0, 0x3E, 0x4E, 0xC5, 0xA5, 0x11, 0xED, 0xDE }, 100 }

//  Name:     System.Status -- PKEY_Status
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_IntSite) {000214A1-0000-0000-C000-000000000046}, 9
DEFINE_PROPERTYKEY(PKEY_Status, 0x000214A1, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 9);
#define INIT_PKEY_Status { { 0x000214A1, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46 }, 9 }

//  Name:     System.StorageProviderCallerVersionInformation -- PKEY_StorageProviderCallerVersionInformation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 7
//  
//  The storage provider caller protocol version information.
//  The format of this property is provider specific, refer to the storage provider documentation for more information.
DEFINE_PROPERTYKEY(PKEY_StorageProviderCallerVersionInformation, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 7);
#define INIT_PKEY_StorageProviderCallerVersionInformation { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 7 }

//  Name:     System.StorageProviderCustomPrimaryIcon -- PKEY_StorageProviderCustomPrimaryIcon
//  Type:     UInt32 -- VT_UI4
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 12
//
//  The storage provider custom icon for this file.
DEFINE_PROPERTYKEY(PKEY_StorageProviderCustomPrimaryIcon, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 12);
#define INIT_PKEY_StorageProviderCustomPrimaryIcon { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 12 }

// Possible discrete values for PKEY_StorageProviderCustomPrimaryIcon are:
#define STORAGEPROVIDERCUSTOM_ICON_PHONE    0ul

//  Name:     System.StorageProviderError -- PKEY_StorageProviderError
//  Type:     UInt32 -- VT_UI4
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 109
DEFINE_PROPERTYKEY(PKEY_StorageProviderError, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 109);
#define INIT_PKEY_StorageProviderError { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 109 }

//  Name:     System.StorageProviderFileChecksum -- PKEY_StorageProviderFileChecksum
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 5
//
//  The checksum computed by the storage provider for the file. Files with the same checksum value will have the same contents.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileChecksum, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 5);
#define INIT_PKEY_StorageProviderFileChecksum { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 5 }

//  Name:     System.StorageProviderFileCreatedBy -- PKEY_StorageProviderFileCreatedBy
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 10
//
//  The display name of the user who created the file or folder specified by the storage provider.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileCreatedBy, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 10);
#define INIT_PKEY_StorageProviderFileCreatedBy { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 10 }

//  Name:     System.StorageProviderFileDateShared -- PKEY_StorageProviderFileDateShared
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 14
//
//  The most recent date and time the file or folder was shared by any user, specified by the storage provider.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileDateShared, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 14);
#define INIT_PKEY_StorageProviderFileDateShared { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 14 }

//  Name:     System.StorageProviderFileFlags -- PKEY_StorageProviderFileFlags
//  Type:     UInt32 -- VT_UI4
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 8
//
//  Information specified by the storage provider about a file or a folder.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileFlags, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 8);
#define INIT_PKEY_StorageProviderFileFlags { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 8 }

//  Name:     System.StorageProviderFileHasConflict -- PKEY_StorageProviderFileHasConflict
//  Type:     Boolean -- VT_BOOL
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 9
//
//  There is a conflict with the version of the file in the cloud.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileHasConflict, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 9);
#define INIT_PKEY_StorageProviderFileHasConflict { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 9 }

//  Name:     System.StorageProviderFileIdentifier -- PKEY_StorageProviderFileIdentifier
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 3
//
//  The storage provider identifier for this file.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileIdentifier, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 3);
#define INIT_PKEY_StorageProviderFileIdentifier { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 3 }

//  Name:     System.StorageProviderFileModifiedBy -- PKEY_StorageProviderFileModifiedBy
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 11
//
//  The display name of the user who last modified the file or folder specified by the storage provider.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileModifiedBy, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 11);
#define INIT_PKEY_StorageProviderFileModifiedBy { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 11 }

//  Name:     System.StorageProviderFileRemoteLocation -- PKEY_StorageProviderFileRemoteLocation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 16
//  
//  The user-friendly display name of the remote location where the file or folder is stored, specified by the storage provider.
//  It should not represent a full path to the item, rather a concise name of where the item is stored remotely.
//  For example, "John Doe's Contoso Drive".
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileRemoteLocation, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 16);
#define INIT_PKEY_StorageProviderFileRemoteLocation { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 16 }

//  Name:     System.StorageProviderFileRemoteUri -- PKEY_StorageProviderFileRemoteUri
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 112
//
//  The storage provider's remote Uri for this file.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileRemoteUri, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 112);
#define INIT_PKEY_StorageProviderFileRemoteUri { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 112 }

//  Name:     System.StorageProviderFileSharedBy -- PKEY_StorageProviderFileSharedBy
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 15
//
//  The display name of the user who last shared the file or folder, specified by the storage provider.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileSharedBy, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 15);
#define INIT_PKEY_StorageProviderFileSharedBy { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 15 }

//  Name:     System.StorageProviderFileVersion -- PKEY_StorageProviderFileVersion
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 4
//
//  The storage provider file version for this file.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileVersion, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 4);
#define INIT_PKEY_StorageProviderFileVersion { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 4 }

//  Name:     System.StorageProviderFileVersionWaterline -- PKEY_StorageProviderFileVersionWaterline
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 6
//
//  The storage provider computed file version waterline for this file. This value is used to detect if a file has changed.
DEFINE_PROPERTYKEY(PKEY_StorageProviderFileVersionWaterline, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 6);
#define INIT_PKEY_StorageProviderFileVersionWaterline { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 6 }

//  Name:     System.StorageProviderFullyQualifiedId -- PKEY_StorageProviderFullyQualifiedId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 119
//  
//  This property represents the entire fully-qualified provider identifier
//  "[Storage Provider ID]![Windows SID]![Account ID]".
DEFINE_PROPERTYKEY(PKEY_StorageProviderFullyQualifiedId, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 119);
#define INIT_PKEY_StorageProviderFullyQualifiedId { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 119 }

//  Name:     System.StorageProviderId -- PKEY_StorageProviderId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 108
//  
//  This property represents the [Storage Provider ID] part of the fully-qualified provider identifier
//  "[Storage Provider ID]![Windows SID]![Account ID]".
DEFINE_PROPERTYKEY(PKEY_StorageProviderId, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 108);
#define INIT_PKEY_StorageProviderId { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 108 }

//  Name:     System.StorageProviderShareStatuses -- PKEY_StorageProviderShareStatuses
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 111
//  
//  This property represents a list of share statuses for the file/folder specified by the storage provider.
//  Each share status must be one of the known value specified by the enumerations below
//  StorageProviderShareStatuses is a readonly property, it should only be updated by the storage provider.
DEFINE_PROPERTYKEY(PKEY_StorageProviderShareStatuses, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 111);
#define INIT_PKEY_StorageProviderShareStatuses { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 111 }

// Possible discrete values for PKEY_StorageProviderShareStatuses are:
#define STORAGE_PROVIDER_SHARE_STATUS_PRIVATE L"Private"
#define STORAGE_PROVIDER_SHARE_STATUS_SHARED L"Shared"
#define STORAGE_PROVIDER_SHARE_STATUS_PUBLIC L"Public"
#define STORAGE_PROVIDER_SHARE_STATUS_GROUP L"Group"
#define STORAGE_PROVIDER_SHARE_STATUS_OWNER L"Owner"

//  Name:     System.StorageProviderSharingStatus -- PKEY_StorageProviderSharingStatus
//  Type:     UInt32 -- VT_UI4
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 117
//  
//  This property represents a the most permissive share status for the file/folder specified by the storage provider.
//  The share statuses from most to least permissive are Owned > Co-owned > Public > Shared > Private.
//  StorageProviderSharingStatus is a readonly property.
DEFINE_PROPERTYKEY(PKEY_StorageProviderSharingStatus, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 117);
#define INIT_PKEY_StorageProviderSharingStatus { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 117 }

// Possible discrete values for PKEY_StorageProviderSharingStatus are:
#define STORAGE_PROVIDER_SHARINGSTATUS_NOTSHARED 0ul
#define STORAGE_PROVIDER_SHARINGSTATUS_SHARED 1ul
#define STORAGE_PROVIDER_SHARINGSTATUS_PRIVATE 2ul
#define STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC 3ul
#define STORAGE_PROVIDER_SHARINGSTATUS_SHARED_OWNED 4ul
#define STORAGE_PROVIDER_SHARINGSTATUS_SHARED_COOWNED 5ul
#define STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC_OWNED 6ul
#define STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC_COOWNED 7ul

//  Name:     System.StorageProviderStatus -- PKEY_StorageProviderStatus
//  Type:     UInt64 -- VT_UI8
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 110
DEFINE_PROPERTYKEY(PKEY_StorageProviderStatus, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 110);
#define INIT_PKEY_StorageProviderStatus { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 110 }

//  Name:     System.StorageProviderUserAccountKind -- PKEY_StorageProviderUserAccountKind
//  Type:     UInt32 -- VT_UI4
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 17
//
//  The account kind for the user who is authenticated with this storage provider.
DEFINE_PROPERTYKEY(PKEY_StorageProviderUserAccountKind, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 17);
#define INIT_PKEY_StorageProviderUserAccountKind { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 17 }

// Possible discrete values for PKEY_StorageProviderUserAccountKind are:
#define STORAGEPROVIDERUSERACCOUNTKIND_UNKNOWN 0ul
#define STORAGEPROVIDERUSERACCOUNTKIND_CONSUMER 1ul
#define STORAGEPROVIDERUSERACCOUNTKIND_BUSINESS 2ul

//  Name:     System.StorageProviderUserId -- PKEY_StorageProviderUserId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {B2F9B9D6-FEC4-4DD5-94D7-8957488C807B}, 13
//
//  The unique identifier for the user who is authenticated with this storage provider.
DEFINE_PROPERTYKEY(PKEY_StorageProviderUserId, 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B, 13);
#define INIT_PKEY_StorageProviderUserId { { 0xB2F9B9D6, 0xFEC4, 0x4DD5, 0x94, 0xD7, 0x89, 0x57, 0x48, 0x8C, 0x80, 0x7B }, 13 }

//  Name:     System.Subject -- PKEY_Subject
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 3 (PIDSI_SUBJECT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Subject, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 3);
#define INIT_PKEY_Subject { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 3 }

//  Name:     System.SyncTransferStatus -- PKEY_SyncTransferStatus
//  Type:     UInt32 -- VT_UI4
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 103
DEFINE_PROPERTYKEY(PKEY_SyncTransferStatus, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 103);
#define INIT_PKEY_SyncTransferStatus { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 103 }

//  Name:     System.Thumbnail -- PKEY_Thumbnail
//  Type:     Clipboard -- VT_CF
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 17 (PIDSI_THUMBNAIL)
//
//  A data that represents the thumbnail in VT_CF format.
DEFINE_PROPERTYKEY(PKEY_Thumbnail, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 17);
#define INIT_PKEY_Thumbnail { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 17 }

//  Name:     System.ThumbnailCacheId -- PKEY_ThumbnailCacheId
//  Type:     UInt64 -- VT_UI8
//  FormatID: {446D16B1-8DAD-4870-A748-402EA43D788C}, 100
//  
//  Unique value that can be used as a key to cache thumbnails. The value changes when the name, volume, or data modified 
//  of an item changes.
DEFINE_PROPERTYKEY(PKEY_ThumbnailCacheId, 0x446D16B1, 0x8DAD, 0x4870, 0xA7, 0x48, 0x40, 0x2E, 0xA4, 0x3D, 0x78, 0x8C, 100);
#define INIT_PKEY_ThumbnailCacheId { { 0x446D16B1, 0x8DAD, 0x4870, 0xA7, 0x48, 0x40, 0x2E, 0xA4, 0x3D, 0x78, 0x8C }, 100 }

//  Name:     System.ThumbnailStream -- PKEY_ThumbnailStream
//  Type:     Stream -- VT_STREAM
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 27
//
//  Data that represents the thumbnail in VT_STREAM format that GDI+/WindowsCodecs supports (jpg, png, etc).
DEFINE_PROPERTYKEY(PKEY_ThumbnailStream, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 27);
#define INIT_PKEY_ThumbnailStream { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 27 }

//  Name:     System.Title -- PKEY_Title
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)  Legacy code may treat this as VT_LPSTR.
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 2 (PIDSI_TITLE)
//
//  Title of item.
DEFINE_PROPERTYKEY(PKEY_Title, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 2);
#define INIT_PKEY_Title { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 2 }

//  Name:     System.TitleSortOverride -- PKEY_TitleSortOverride
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)  Legacy code may treat this as VT_LPSTR.
//  FormatID: {F0F7984D-222E-4AD2-82AB-1DD8EA40E57E}, 300 (PIDSI_TITLE_SORT_OVERRIDE)
//  
//  This optional string value allows for overriding the standard sort order of System.Title.
//  This is very important for proper sorting of music files in Japanese which cannot be
//  correctly sorted phonetically (the user-expected ordering) without this field.
//  It can also be used for customizing sorting in non-East Asian scenarios,
//  such as allowing the user to remove articles for sorting purposes.
DEFINE_PROPERTYKEY(PKEY_TitleSortOverride, 0xF0F7984D, 0x222E, 0x4AD2, 0x82, 0xAB, 0x1D, 0xD8, 0xEA, 0x40, 0xE5, 0x7E, 300);
#define INIT_PKEY_TitleSortOverride { { 0xF0F7984D, 0x222E, 0x4AD2, 0x82, 0xAB, 0x1D, 0xD8, 0xEA, 0x40, 0xE5, 0x7E }, 300 }

//  Name:     System.TotalFileSize -- PKEY_TotalFileSize
//  Type:     UInt64 -- VT_UI8
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 14
//
//  
DEFINE_PROPERTYKEY(PKEY_TotalFileSize, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 14);
#define INIT_PKEY_TotalFileSize { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 14 }

//  Name:     System.Trademarks -- PKEY_Trademarks
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSFMTID_VERSION) {0CEF7D53-FA64-11D1-A203-0000F81FEDEE}, 9 (PIDVSI_Trademarks)
//
//  
DEFINE_PROPERTYKEY(PKEY_Trademarks, 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE, 9);
#define INIT_PKEY_Trademarks { { 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE }, 9 }

//  Name:     System.TransferOrder -- PKEY_TransferOrder
//  Type:     UInt64 -- VT_UI8
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 106
DEFINE_PROPERTYKEY(PKEY_TransferOrder, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 106);
#define INIT_PKEY_TransferOrder { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 106 }

//  Name:     System.TransferPosition -- PKEY_TransferPosition
//  Type:     UInt64 -- VT_UI8
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 104
DEFINE_PROPERTYKEY(PKEY_TransferPosition, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 104);
#define INIT_PKEY_TransferPosition { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 104 }

//  Name:     System.TransferSize -- PKEY_TransferSize
//  Type:     UInt64 -- VT_UI8
//  FormatID: {FCEFF153-E839-4CF3-A9E7-EA22832094B8}, 105
DEFINE_PROPERTYKEY(PKEY_TransferSize, 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8, 105);
#define INIT_PKEY_TransferSize { { 0xFCEFF153, 0xE839, 0x4CF3, 0xA9, 0xE7, 0xEA, 0x22, 0x83, 0x20, 0x94, 0xB8 }, 105 }

//  Name:     System.VolumeId -- PKEY_VolumeId
//  Type:     Guid -- VT_CLSID
//  FormatID: {446D16B1-8DAD-4870-A748-402EA43D788C}, 104
//  
//  The GUID of the NTFS Volume.
DEFINE_PROPERTYKEY(PKEY_VolumeId, 0x446D16B1, 0x8DAD, 0x4870, 0xA7, 0x48, 0x40, 0x2E, 0xA4, 0x3D, 0x78, 0x8C, 104);
#define INIT_PKEY_VolumeId { { 0x446D16B1, 0x8DAD, 0x4870, 0xA7, 0x48, 0x40, 0x2E, 0xA4, 0x3D, 0x78, 0x8C }, 104 }

//  Name:     System.ZoneIdentifier -- PKEY_ZoneIdentifier
//  Type:     UInt32 -- VT_UI4
//  FormatID: {502CFEAB-47EB-459C-B960-E6D8728F7701}, 100
//
//  Mark of the Web zone, as URLZONE enumeration value.
DEFINE_PROPERTYKEY(PKEY_ZoneIdentifier, 0x502CFEAB, 0x47EB, 0x459C, 0xB9, 0x60, 0xE6, 0xD8, 0x72, 0x8F, 0x77, 0x01, 100);
#define INIT_PKEY_ZoneIdentifier { { 0x502CFEAB, 0x47EB, 0x459C, 0xB9, 0x60, 0xE6, 0xD8, 0x72, 0x8F, 0x77, 0x01 }, 100 }

//-----------------------------------------------------------------------------
// Devices properties

//  Name:     System.Device.PrinterURL -- PKEY_Device_PrinterURL
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {0B48F35A-BE6E-4F17-B108-3C4073D1669A}, 15
//
//  Printer information Printer URL.
DEFINE_PROPERTYKEY(PKEY_Device_PrinterURL, 0x0B48F35A, 0xBE6E, 0x4F17, 0xB1, 0x08, 0x3C, 0x40, 0x73, 0xD1, 0x66, 0x9A, 15);
#define INIT_PKEY_Device_PrinterURL { { 0x0B48F35A, 0xBE6E, 0x4F17, 0xB1, 0x08, 0x3C, 0x40, 0x73, 0xD1, 0x66, 0x9A }, 15 }

//  Name:     System.DeviceInterface.Bluetooth.DeviceAddress -- PKEY_DeviceInterface_Bluetooth_DeviceAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 1
//
//  Bluetooth device address.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Bluetooth_DeviceAddress, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 1);
#define INIT_PKEY_DeviceInterface_Bluetooth_DeviceAddress { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 1 }

//  Name:     System.DeviceInterface.Bluetooth.Flags -- PKEY_DeviceInterface_Bluetooth_Flags
//  Type:     UInt32 -- VT_UI4
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 3
//
//  Bluetooth device flags.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Bluetooth_Flags, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 3);
#define INIT_PKEY_DeviceInterface_Bluetooth_Flags { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 3 }

//  Name:     System.DeviceInterface.Bluetooth.LastConnectedTime -- PKEY_DeviceInterface_Bluetooth_LastConnectedTime
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 11
//
//  Bluetooth device last connected time.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Bluetooth_LastConnectedTime, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 11);
#define INIT_PKEY_DeviceInterface_Bluetooth_LastConnectedTime { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 11 }

//  Name:     System.DeviceInterface.Bluetooth.Manufacturer -- PKEY_DeviceInterface_Bluetooth_Manufacturer
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 4
//
//  Bluetooth device manufacturer.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Bluetooth_Manufacturer, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 4);
#define INIT_PKEY_DeviceInterface_Bluetooth_Manufacturer { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 4 }

//  Name:     System.DeviceInterface.Bluetooth.ModelNumber -- PKEY_DeviceInterface_Bluetooth_ModelNumber
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 5
//
//  Bluetooth device model number.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Bluetooth_ModelNumber, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 5);
#define INIT_PKEY_DeviceInterface_Bluetooth_ModelNumber { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 5 }

//  Name:     System.DeviceInterface.Bluetooth.ProductId -- PKEY_DeviceInterface_Bluetooth_ProductId
//  Type:     UInt16 -- VT_UI2
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 8
//
//  Bluetooth device product identifier.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Bluetooth_ProductId, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 8);
#define INIT_PKEY_DeviceInterface_Bluetooth_ProductId { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 8 }

//  Name:     System.DeviceInterface.Bluetooth.ProductVersion -- PKEY_DeviceInterface_Bluetooth_ProductVersion
//  Type:     UInt16 -- VT_UI2
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 9
//
//  Bluetooth device product version.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Bluetooth_ProductVersion, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 9);
#define INIT_PKEY_DeviceInterface_Bluetooth_ProductVersion { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 9 }

//  Name:     System.DeviceInterface.Bluetooth.ServiceGuid -- PKEY_DeviceInterface_Bluetooth_ServiceGuid
//  Type:     Guid -- VT_CLSID
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 2
//
//  Bluetooth service GUID.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Bluetooth_ServiceGuid, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 2);
#define INIT_PKEY_DeviceInterface_Bluetooth_ServiceGuid { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 2 }

//  Name:     System.DeviceInterface.Bluetooth.VendorId -- PKEY_DeviceInterface_Bluetooth_VendorId
//  Type:     UInt16 -- VT_UI2
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 7
//
//  Bluetooth device vendor identifier.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Bluetooth_VendorId, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 7);
#define INIT_PKEY_DeviceInterface_Bluetooth_VendorId { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 7 }

//  Name:     System.DeviceInterface.Bluetooth.VendorIdSource -- PKEY_DeviceInterface_Bluetooth_VendorIdSource
//  Type:     Byte -- VT_UI1
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 6
//
//  Bluetooth device vendor identifier source.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Bluetooth_VendorIdSource, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 6);
#define INIT_PKEY_DeviceInterface_Bluetooth_VendorIdSource { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 6 }

//  Name:     System.DeviceInterface.Hid.IsReadOnly -- PKEY_DeviceInterface_Hid_IsReadOnly
//  Type:     Boolean -- VT_BOOL
//  FormatID: {CBF38310-4A17-4310-A1EB-247F0B67593B}, 4
//
//  Indicates if a HID device is a Read-Only device.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Hid_IsReadOnly, 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B, 4);
#define INIT_PKEY_DeviceInterface_Hid_IsReadOnly { { 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B }, 4 }

//  Name:     System.DeviceInterface.Hid.ProductId -- PKEY_DeviceInterface_Hid_ProductId
//  Type:     UInt16 -- VT_UI2
//  FormatID: {CBF38310-4A17-4310-A1EB-247F0B67593B}, 6
//
//  HID device Product Id.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Hid_ProductId, 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B, 6);
#define INIT_PKEY_DeviceInterface_Hid_ProductId { { 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B }, 6 }

//  Name:     System.DeviceInterface.Hid.UsageId -- PKEY_DeviceInterface_Hid_UsageId
//  Type:     UInt16 -- VT_UI2
//  FormatID: {CBF38310-4A17-4310-A1EB-247F0B67593B}, 3
//
//  HID device Usage Id.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Hid_UsageId, 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B, 3);
#define INIT_PKEY_DeviceInterface_Hid_UsageId { { 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B }, 3 }

//  Name:     System.DeviceInterface.Hid.UsagePage -- PKEY_DeviceInterface_Hid_UsagePage
//  Type:     UInt16 -- VT_UI2
//  FormatID: {CBF38310-4A17-4310-A1EB-247F0B67593B}, 2
//
//  HID device Usage Page.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Hid_UsagePage, 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B, 2);
#define INIT_PKEY_DeviceInterface_Hid_UsagePage { { 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B }, 2 }

//  Name:     System.DeviceInterface.Hid.VendorId -- PKEY_DeviceInterface_Hid_VendorId
//  Type:     UInt16 -- VT_UI2
//  FormatID: {CBF38310-4A17-4310-A1EB-247F0B67593B}, 5
//
//  HID device Vendor Id.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Hid_VendorId, 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B, 5);
#define INIT_PKEY_DeviceInterface_Hid_VendorId { { 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B }, 5 }

//  Name:     System.DeviceInterface.Hid.VersionNumber -- PKEY_DeviceInterface_Hid_VersionNumber
//  Type:     UInt16 -- VT_UI2
//  FormatID: {CBF38310-4A17-4310-A1EB-247F0B67593B}, 7
//
//  HID device Version Number.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Hid_VersionNumber, 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B, 7);
#define INIT_PKEY_DeviceInterface_Hid_VersionNumber { { 0xCBF38310, 0x4A17, 0x4310, 0xA1, 0xEB, 0x24, 0x7F, 0x0B, 0x67, 0x59, 0x3B }, 7 }

//  Name:     System.DeviceInterface.PrinterDriverDirectory -- PKEY_DeviceInterface_PrinterDriverDirectory
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {847C66DE-B8D6-4AF9-ABC3-6F4F926BC039}, 14
//
//  Printer information Printer Driver Directory.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_PrinterDriverDirectory, 0x847C66DE, 0xB8D6, 0x4AF9, 0xAB, 0xC3, 0x6F, 0x4F, 0x92, 0x6B, 0xC0, 0x39, 14);
#define INIT_PKEY_DeviceInterface_PrinterDriverDirectory { { 0x847C66DE, 0xB8D6, 0x4AF9, 0xAB, 0xC3, 0x6F, 0x4F, 0x92, 0x6B, 0xC0, 0x39 }, 14 }

//  Name:     System.DeviceInterface.PrinterDriverName -- PKEY_DeviceInterface_PrinterDriverName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {AFC47170-14F5-498C-8F30-B0D19BE449C6}, 11
//
//  Printer information Driver Name.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_PrinterDriverName, 0xAFC47170, 0x14F5, 0x498C, 0x8F, 0x30, 0xB0, 0xD1, 0x9B, 0xE4, 0x49, 0xC6, 11);
#define INIT_PKEY_DeviceInterface_PrinterDriverName { { 0xAFC47170, 0x14F5, 0x498C, 0x8F, 0x30, 0xB0, 0xD1, 0x9B, 0xE4, 0x49, 0xC6 }, 11 }

//  Name:     System.DeviceInterface.PrinterEnumerationFlag -- PKEY_DeviceInterface_PrinterEnumerationFlag
//  Type:     UInt32 -- VT_UI4
//  FormatID: {A00742A1-CD8C-4B37-95AB-70755587767A}, 3
//
//  Printer information Printer Enumeration Flag.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_PrinterEnumerationFlag, 0xA00742A1, 0xCD8C, 0x4B37, 0x95, 0xAB, 0x70, 0x75, 0x55, 0x87, 0x76, 0x7A, 3);
#define INIT_PKEY_DeviceInterface_PrinterEnumerationFlag { { 0xA00742A1, 0xCD8C, 0x4B37, 0x95, 0xAB, 0x70, 0x75, 0x55, 0x87, 0x76, 0x7A }, 3 }

//  Name:     System.DeviceInterface.PrinterName -- PKEY_DeviceInterface_PrinterName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {0A7B84EF-0C27-463F-84EF-06C5070001BE}, 10
//
//  Printer information Printer Name.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_PrinterName, 0x0A7B84EF, 0x0C27, 0x463F, 0x84, 0xEF, 0x06, 0xC5, 0x07, 0x00, 0x01, 0xBE, 10);
#define INIT_PKEY_DeviceInterface_PrinterName { { 0x0A7B84EF, 0x0C27, 0x463F, 0x84, 0xEF, 0x06, 0xC5, 0x07, 0x00, 0x01, 0xBE }, 10 }

//  Name:     System.DeviceInterface.PrinterPortName -- PKEY_DeviceInterface_PrinterPortName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {EEC7B761-6F94-41B1-949F-C729720DD13C}, 12
//
//  Printer information Port Name.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_PrinterPortName, 0xEEC7B761, 0x6F94, 0x41B1, 0x94, 0x9F, 0xC7, 0x29, 0x72, 0x0D, 0xD1, 0x3C, 12);
#define INIT_PKEY_DeviceInterface_PrinterPortName { { 0xEEC7B761, 0x6F94, 0x41B1, 0x94, 0x9F, 0xC7, 0x29, 0x72, 0x0D, 0xD1, 0x3C }, 12 }

//  Name:     System.DeviceInterface.Proximity.SupportsNfc -- PKEY_DeviceInterface_Proximity_SupportsNfc
//  Type:     Boolean -- VT_BOOL
//  FormatID: {FB3842CD-9E2A-4F83-8FCC-4B0761139AE9}, 2
//
//  Indicates whether the device supports NFC communications (NDEF).
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Proximity_SupportsNfc, 0xFB3842CD, 0x9E2A, 0x4F83, 0x8F, 0xCC, 0x4B, 0x07, 0x61, 0x13, 0x9A, 0xE9, 2);
#define INIT_PKEY_DeviceInterface_Proximity_SupportsNfc { { 0xFB3842CD, 0x9E2A, 0x4F83, 0x8F, 0xCC, 0x4B, 0x07, 0x61, 0x13, 0x9A, 0xE9 }, 2 }

//  Name:     System.DeviceInterface.Serial.PortName -- PKEY_DeviceInterface_Serial_PortName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4C6BF15C-4C03-4AAC-91F5-64C0F852BCF4}, 4
//
//  Serial device friendly name
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Serial_PortName, 0x4C6BF15C, 0x4C03, 0x4AAC, 0x91, 0xF5, 0x64, 0xC0, 0xF8, 0x52, 0xBC, 0xF4, 4);
#define INIT_PKEY_DeviceInterface_Serial_PortName { { 0x4C6BF15C, 0x4C03, 0x4AAC, 0x91, 0xF5, 0x64, 0xC0, 0xF8, 0x52, 0xBC, 0xF4 }, 4 }

//  Name:     System.DeviceInterface.Serial.UsbProductId -- PKEY_DeviceInterface_Serial_UsbProductId
//  Type:     UInt16 -- VT_UI2
//  FormatID: {4C6BF15C-4C03-4AAC-91F5-64C0F852BCF4}, 3
//
//  Serial device USB Product Id
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Serial_UsbProductId, 0x4C6BF15C, 0x4C03, 0x4AAC, 0x91, 0xF5, 0x64, 0xC0, 0xF8, 0x52, 0xBC, 0xF4, 3);
#define INIT_PKEY_DeviceInterface_Serial_UsbProductId { { 0x4C6BF15C, 0x4C03, 0x4AAC, 0x91, 0xF5, 0x64, 0xC0, 0xF8, 0x52, 0xBC, 0xF4 }, 3 }

//  Name:     System.DeviceInterface.Serial.UsbVendorId -- PKEY_DeviceInterface_Serial_UsbVendorId
//  Type:     UInt16 -- VT_UI2
//  FormatID: {4C6BF15C-4C03-4AAC-91F5-64C0F852BCF4}, 2
//
//  Serial device USB Vendor Id.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_Serial_UsbVendorId, 0x4C6BF15C, 0x4C03, 0x4AAC, 0x91, 0xF5, 0x64, 0xC0, 0xF8, 0x52, 0xBC, 0xF4, 2);
#define INIT_PKEY_DeviceInterface_Serial_UsbVendorId { { 0x4C6BF15C, 0x4C03, 0x4AAC, 0x91, 0xF5, 0x64, 0xC0, 0xF8, 0x52, 0xBC, 0xF4 }, 2 }

//  Name:     System.DeviceInterface.WinUsb.DeviceInterfaceClasses -- PKEY_DeviceInterface_WinUsb_DeviceInterfaceClasses
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {95E127B5-79CC-4E83-9C9E-8422187B3E0E}, 7
//
//  WinUSB device interface GUID(s) used to open a handle to the device.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_WinUsb_DeviceInterfaceClasses, 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E, 7);
#define INIT_PKEY_DeviceInterface_WinUsb_DeviceInterfaceClasses { { 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E }, 7 }

//  Name:     System.DeviceInterface.WinUsb.UsbClass -- PKEY_DeviceInterface_WinUsb_UsbClass
//  Type:     Byte -- VT_UI1
//  FormatID: {95E127B5-79CC-4E83-9C9E-8422187B3E0E}, 4
//
//  Class value from the USB device's first USB Interface Descriptor.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_WinUsb_UsbClass, 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E, 4);
#define INIT_PKEY_DeviceInterface_WinUsb_UsbClass { { 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E }, 4 }

//  Name:     System.DeviceInterface.WinUsb.UsbProductId -- PKEY_DeviceInterface_WinUsb_UsbProductId
//  Type:     UInt16 -- VT_UI2
//  FormatID: {95E127B5-79CC-4E83-9C9E-8422187B3E0E}, 3
//
//  Product ID from the USB device's USB Device Descriptor.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_WinUsb_UsbProductId, 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E, 3);
#define INIT_PKEY_DeviceInterface_WinUsb_UsbProductId { { 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E }, 3 }

//  Name:     System.DeviceInterface.WinUsb.UsbProtocol -- PKEY_DeviceInterface_WinUsb_UsbProtocol
//  Type:     Byte -- VT_UI1
//  FormatID: {95E127B5-79CC-4E83-9C9E-8422187B3E0E}, 6
//
//  Protocol value from the USB device's first USB Interface Descriptor.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_WinUsb_UsbProtocol, 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E, 6);
#define INIT_PKEY_DeviceInterface_WinUsb_UsbProtocol { { 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E }, 6 }

//  Name:     System.DeviceInterface.WinUsb.UsbSubClass -- PKEY_DeviceInterface_WinUsb_UsbSubClass
//  Type:     Byte -- VT_UI1
//  FormatID: {95E127B5-79CC-4E83-9C9E-8422187B3E0E}, 5
//
//  SubClass value from the USB device's first USB Interface Descriptor.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_WinUsb_UsbSubClass, 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E, 5);
#define INIT_PKEY_DeviceInterface_WinUsb_UsbSubClass { { 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E }, 5 }

//  Name:     System.DeviceInterface.WinUsb.UsbVendorId -- PKEY_DeviceInterface_WinUsb_UsbVendorId
//  Type:     UInt16 -- VT_UI2
//  FormatID: {95E127B5-79CC-4E83-9C9E-8422187B3E0E}, 2
//
//  Vendor ID from the USB device's USB Device Descriptor.
DEFINE_PROPERTYKEY(PKEY_DeviceInterface_WinUsb_UsbVendorId, 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E, 2);
#define INIT_PKEY_DeviceInterface_WinUsb_UsbVendorId { { 0x95E127B5, 0x79CC, 0x4E83, 0x9C, 0x9E, 0x84, 0x22, 0x18, 0x7B, 0x3E, 0x0E }, 2 }

//  Name:     System.Devices.Aep.AepId -- PKEY_Devices_Aep_AepId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {3B2CE006-5E61-4FDE-BAB8-9B8AAC9B26DF}, 8
//
//  Identity of the Device Association Endpoint
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_AepId, 0x3B2CE006, 0x5E61, 0x4FDE, 0xBA, 0xB8, 0x9B, 0x8A, 0xAC, 0x9B, 0x26, 0xDF, 8);
#define INIT_PKEY_Devices_Aep_AepId { { 0x3B2CE006, 0x5E61, 0x4FDE, 0xBA, 0xB8, 0x9B, 0x8A, 0xAC, 0x9B, 0x26, 0xDF }, 8 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Major -- PKEY_Devices_Aep_Bluetooth_Cod_Major
//  Type:     UInt16 -- VT_UI2
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 2
//
//  Bluetooth class of device major code.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Major, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 2);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Major { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 2 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Minor -- PKEY_Devices_Aep_Bluetooth_Cod_Minor
//  Type:     UInt16 -- VT_UI2
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 3
//
//  Bluetooth class of device minor code.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Minor, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 3);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Minor { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 3 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Services.Audio -- PKEY_Devices_Aep_Bluetooth_Cod_Services_Audio
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 10
//
//  Bluetooth class of device service audio.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Services_Audio, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 10);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Services_Audio { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 10 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Services.Capturing -- PKEY_Devices_Aep_Bluetooth_Cod_Services_Capturing
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 8
//
//  Bluetooth class of device service capturing.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Services_Capturing, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 8);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Services_Capturing { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 8 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Services.Information -- PKEY_Devices_Aep_Bluetooth_Cod_Services_Information
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 12
//
//  Bluetooth class of device service information
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Services_Information, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 12);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Services_Information { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 12 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Services.LimitedDiscovery -- PKEY_Devices_Aep_Bluetooth_Cod_Services_LimitedDiscovery
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 4
//
//  Bluetooth class of device service limited discovery.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Services_LimitedDiscovery, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 4);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Services_LimitedDiscovery { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 4 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Services.Networking -- PKEY_Devices_Aep_Bluetooth_Cod_Services_Networking
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 6
//
//  Bluetooth class of device service networking.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Services_Networking, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 6);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Services_Networking { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 6 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Services.ObjectXfer -- PKEY_Devices_Aep_Bluetooth_Cod_Services_ObjectXfer
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 9
//
//  Bluetooth class of device service object transfer.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Services_ObjectXfer, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 9);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Services_ObjectXfer { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 9 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Services.Positioning -- PKEY_Devices_Aep_Bluetooth_Cod_Services_Positioning
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 5
//
//  Bluetooth class of device service positioning.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Services_Positioning, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 5);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Services_Positioning { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 5 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Services.Rendering -- PKEY_Devices_Aep_Bluetooth_Cod_Services_Rendering
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 7
//
//  Bluetooth class of device service rendering.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Services_Rendering, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 7);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Services_Rendering { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 7 }

//  Name:     System.Devices.Aep.Bluetooth.Cod.Services.Telephony -- PKEY_Devices_Aep_Bluetooth_Cod_Services_Telephony
//  Type:     Boolean -- VT_BOOL
//  FormatID: {5FBD34CD-561A-412E-BA98-478A6B0FEF1D}, 11
//
//  Bluetooth class of device service telephony.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Cod_Services_Telephony, 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D, 11);
#define INIT_PKEY_Devices_Aep_Bluetooth_Cod_Services_Telephony { { 0x5FBD34CD, 0x561A, 0x412E, 0xBA, 0x98, 0x47, 0x8A, 0x6B, 0x0F, 0xEF, 0x1D }, 11 }

//  Name:     System.Devices.Aep.Bluetooth.LastSeenTime -- PKEY_Devices_Aep_Bluetooth_LastSeenTime
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {2BD67D8B-8BEB-48D5-87E0-6CDA3428040A}, 12
//
//  The time stamp the device was last seen via an inquiry or advertisement.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_LastSeenTime, 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A, 12);
#define INIT_PKEY_Devices_Aep_Bluetooth_LastSeenTime { { 0x2BD67D8B, 0x8BEB, 0x48D5, 0x87, 0xE0, 0x6C, 0xDA, 0x34, 0x28, 0x04, 0x0A }, 12 }

//  Name:     System.Devices.Aep.Bluetooth.Le.AddressType -- PKEY_Devices_Aep_Bluetooth_Le_AddressType
//  Type:     Byte -- VT_UI1
//  FormatID: {995EF0B0-7EB3-4A8B-B9CE-068BB3F4AF69}, 4
//
//  Bluetooth LE device address type.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Le_AddressType, 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69, 4);
#define INIT_PKEY_Devices_Aep_Bluetooth_Le_AddressType { { 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69 }, 4 }

// Possible discrete values for PKEY_Devices_Aep_Bluetooth_Le_AddressType are:
#define BLUETOOTH_ADDRESS_TYPE_PUBLIC       0
#define BLUETOOTH_ADDRESS_TYPE_RANDOM       1

//  Name:     System.Devices.Aep.Bluetooth.Le.Appearance -- PKEY_Devices_Aep_Bluetooth_Le_Appearance
//  Type:     UInt16 -- VT_UI2
//  FormatID: {995EF0B0-7EB3-4A8B-B9CE-068BB3F4AF69}, 1
//
//  Bluetooth LE device appearance.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Le_Appearance, 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69, 1);
#define INIT_PKEY_Devices_Aep_Bluetooth_Le_Appearance { { 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69 }, 1 }

//  Name:     System.Devices.Aep.Bluetooth.Le.Appearance.Category -- PKEY_Devices_Aep_Bluetooth_Le_Appearance_Category
//  Type:     UInt16 -- VT_UI2
//  FormatID: {995EF0B0-7EB3-4A8B-B9CE-068BB3F4AF69}, 5
//
//  Bluetooth LE device appearance.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Le_Appearance_Category, 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69, 5);
#define INIT_PKEY_Devices_Aep_Bluetooth_Le_Appearance_Category { { 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69 }, 5 }

//  Name:     System.Devices.Aep.Bluetooth.Le.Appearance.Subcategory -- PKEY_Devices_Aep_Bluetooth_Le_Appearance_Subcategory
//  Type:     UInt16 -- VT_UI2
//  FormatID: {995EF0B0-7EB3-4A8B-B9CE-068BB3F4AF69}, 6
//
//  Bluetooth LE device appearance.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Le_Appearance_Subcategory, 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69, 6);
#define INIT_PKEY_Devices_Aep_Bluetooth_Le_Appearance_Subcategory { { 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69 }, 6 }

//  Name:     System.Devices.Aep.Bluetooth.Le.IsCallControlClient -- PKEY_Devices_Aep_Bluetooth_Le_IsCallControlClient
//  Type:     Boolean -- VT_BOOL
//  FormatID: {995EF0B0-7EB3-4A8B-B9CE-068BB3F4AF69}, 12
//
//  Whether the Bluetooth LE device is a call control client.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Le_IsCallControlClient, 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69, 12);
#define INIT_PKEY_Devices_Aep_Bluetooth_Le_IsCallControlClient { { 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69 }, 12 }

//  Name:     System.Devices.Aep.Bluetooth.Le.IsConnectable -- PKEY_Devices_Aep_Bluetooth_Le_IsConnectable
//  Type:     Boolean -- VT_BOOL
//  FormatID: {995EF0B0-7EB3-4A8B-B9CE-068BB3F4AF69}, 8
//
//  Whether the Bluetooth LE device is currently advertising a connectable advertisement.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Bluetooth_Le_IsConnectable, 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69, 8);
#define INIT_PKEY_Devices_Aep_Bluetooth_Le_IsConnectable { { 0x995EF0B0, 0x7EB3, 0x4A8B, 0xB9, 0xCE, 0x06, 0x8B, 0xB3, 0xF4, 0xAF, 0x69 }, 8 }

//  Name:     System.Devices.Aep.CanPair -- PKEY_Devices_Aep_CanPair
//  Type:     Boolean -- VT_BOOL
//  FormatID: {E7C3FB29-CAA7-4F47-8C8B-BE59B330D4C5}, 3
//
//  Whether the Device Association Endpoint can be paired with the system or not
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_CanPair, 0xE7C3FB29, 0xCAA7, 0x4F47, 0x8C, 0x8B, 0xBE, 0x59, 0xB3, 0x30, 0xD4, 0xC5, 3);
#define INIT_PKEY_Devices_Aep_CanPair { { 0xE7C3FB29, 0xCAA7, 0x4F47, 0x8C, 0x8B, 0xBE, 0x59, 0xB3, 0x30, 0xD4, 0xC5 }, 3 }

//  Name:     System.Devices.Aep.Category -- PKEY_Devices_Aep_Category
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {A35996AB-11CF-4935-8B61-A6761081ECDF}, 17
//
//  Categories the device is part of. e.g. Printer, Camera, etc.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Category, 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF, 17);
#define INIT_PKEY_Devices_Aep_Category { { 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF }, 17 }

//  Name:     System.Devices.Aep.ContainerId -- PKEY_Devices_Aep_ContainerId
//  Type:     Guid -- VT_CLSID
//  FormatID: {E7C3FB29-CAA7-4F47-8C8B-BE59B330D4C5}, 2
//
//  Device Association Endpoint's Parent Container Id
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_ContainerId, 0xE7C3FB29, 0xCAA7, 0x4F47, 0x8C, 0x8B, 0xBE, 0x59, 0xB3, 0x30, 0xD4, 0xC5, 2);
#define INIT_PKEY_Devices_Aep_ContainerId { { 0xE7C3FB29, 0xCAA7, 0x4F47, 0x8C, 0x8B, 0xBE, 0x59, 0xB3, 0x30, 0xD4, 0xC5 }, 2 }

//  Name:     System.Devices.Aep.DeviceAddress -- PKEY_Devices_Aep_DeviceAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A35996AB-11CF-4935-8B61-A6761081ECDF}, 12
//
//  Address based on the protocol of the Device Association Endpoint. IP Address for an IP device, Bluetooth address for Bluetooth device, etc.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_DeviceAddress, 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF, 12);
#define INIT_PKEY_Devices_Aep_DeviceAddress { { 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF }, 12 }

//  Name:     System.Devices.Aep.IsConnected -- PKEY_Devices_Aep_IsConnected
//  Type:     Boolean -- VT_BOOL
//  FormatID: {A35996AB-11CF-4935-8B61-A6761081ECDF}, 7
//
//  Whether the device is currently connected to the system or or not
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_IsConnected, 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF, 7);
#define INIT_PKEY_Devices_Aep_IsConnected { { 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF }, 7 }

//  Name:     System.Devices.Aep.IsPaired -- PKEY_Devices_Aep_IsPaired
//  Type:     Boolean -- VT_BOOL
//  FormatID: {A35996AB-11CF-4935-8B61-A6761081ECDF}, 16
//
//  Whether the Device Association Endpoint is paired with the system or not
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_IsPaired, 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF, 16);
#define INIT_PKEY_Devices_Aep_IsPaired { { 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF }, 16 }

//  Name:     System.Devices.Aep.IsPresent -- PKEY_Devices_Aep_IsPresent
//  Type:     Boolean -- VT_BOOL
//  FormatID: {A35996AB-11CF-4935-8B61-A6761081ECDF}, 9
//
//  Whether the device is currently present or not
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_IsPresent, 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF, 9);
#define INIT_PKEY_Devices_Aep_IsPresent { { 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF }, 9 }

//  Name:     System.Devices.Aep.Manufacturer -- PKEY_Devices_Aep_Manufacturer
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A35996AB-11CF-4935-8B61-A6761081ECDF}, 5
//
//  Device Association Endpoint's Manufacturer
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_Manufacturer, 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF, 5);
#define INIT_PKEY_Devices_Aep_Manufacturer { { 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF }, 5 }

//  Name:     System.Devices.Aep.ModelId -- PKEY_Devices_Aep_ModelId
//  Type:     Guid -- VT_CLSID
//  FormatID: {A35996AB-11CF-4935-8B61-A6761081ECDF}, 4
//
//  Device Association Endpoint's Model Id
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_ModelId, 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF, 4);
#define INIT_PKEY_Devices_Aep_ModelId { { 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF }, 4 }

//  Name:     System.Devices.Aep.ModelName -- PKEY_Devices_Aep_ModelName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A35996AB-11CF-4935-8B61-A6761081ECDF}, 3
//
//  Device Association Endpoint's Model Name
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_ModelName, 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF, 3);
#define INIT_PKEY_Devices_Aep_ModelName { { 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF }, 3 }

//  Name:     System.Devices.Aep.PointOfService.ConnectionTypes -- PKEY_Devices_Aep_PointOfService_ConnectionTypes
//  Type:     Int32 -- VT_I4
//  FormatID: {D4BF61B3-442E-4ADA-882D-FA7B70C832D9}, 6
//
//  A bit mask that specifies which connection types should be included in the search.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_PointOfService_ConnectionTypes, 0xD4BF61B3, 0x442E, 0x4ADA, 0x88, 0x2D, 0xFA, 0x7B, 0x70, 0xC8, 0x32, 0xD9, 6);
#define INIT_PKEY_Devices_Aep_PointOfService_ConnectionTypes { { 0xD4BF61B3, 0x442E, 0x4ADA, 0x88, 0x2D, 0xFA, 0x7B, 0x70, 0xC8, 0x32, 0xD9 }, 6 }

//  Name:     System.Devices.Aep.ProtocolId -- PKEY_Devices_Aep_ProtocolId
//  Type:     Guid -- VT_CLSID
//  FormatID: {3B2CE006-5E61-4FDE-BAB8-9B8AAC9B26DF}, 5
//
//  Identity of the protocol this Device Association Endpoint was discovered over
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_ProtocolId, 0x3B2CE006, 0x5E61, 0x4FDE, 0xBA, 0xB8, 0x9B, 0x8A, 0xAC, 0x9B, 0x26, 0xDF, 5);
#define INIT_PKEY_Devices_Aep_ProtocolId { { 0x3B2CE006, 0x5E61, 0x4FDE, 0xBA, 0xB8, 0x9B, 0x8A, 0xAC, 0x9B, 0x26, 0xDF }, 5 }

//  Name:     System.Devices.Aep.SignalStrength -- PKEY_Devices_Aep_SignalStrength
//  Type:     Int32 -- VT_I4
//  FormatID: {A35996AB-11CF-4935-8B61-A6761081ECDF}, 6
//
//  Signal strength of the device. Only applicable for some protocols.
DEFINE_PROPERTYKEY(PKEY_Devices_Aep_SignalStrength, 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF, 6);
#define INIT_PKEY_Devices_Aep_SignalStrength { { 0xA35996AB, 0x11CF, 0x4935, 0x8B, 0x61, 0xA6, 0x76, 0x10, 0x81, 0xEC, 0xDF }, 6 }

//  Name:     System.Devices.AepContainer.CanPair -- PKEY_Devices_AepContainer_CanPair
//  Type:     Boolean -- VT_BOOL
//  FormatID: {0BBA1EDE-7566-4F47-90EC-25FC567CED2A}, 3
//
//  Whether one of the child Device Association Endpoints can be paired with the system or not.
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_CanPair, 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A, 3);
#define INIT_PKEY_Devices_AepContainer_CanPair { { 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A }, 3 }

//  Name:     System.Devices.AepContainer.Categories -- PKEY_Devices_AepContainer_Categories
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {0BBA1EDE-7566-4F47-90EC-25FC567CED2A}, 9
//
//  Categories the device is part of. e.g. Printer, Camera, etc.
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_Categories, 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A, 9);
#define INIT_PKEY_Devices_AepContainer_Categories { { 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A }, 9 }

//  Name:     System.Devices.AepContainer.Children -- PKEY_Devices_AepContainer_Children
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {0BBA1EDE-7566-4F47-90EC-25FC567CED2A}, 2
//
//  List of child Device Association Endpoint Identities that are part of this Device Association Endpoint Container
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_Children, 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A, 2);
#define INIT_PKEY_Devices_AepContainer_Children { { 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A }, 2 }

//  Name:     System.Devices.AepContainer.ContainerId -- PKEY_Devices_AepContainer_ContainerId
//  Type:     Guid -- VT_CLSID
//  FormatID: {0BBA1EDE-7566-4F47-90EC-25FC567CED2A}, 12
//
//  Device Association Endpoint Container's Identity
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_ContainerId, 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A, 12);
#define INIT_PKEY_Devices_AepContainer_ContainerId { { 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A }, 12 }

//  Name:     System.Devices.AepContainer.DialProtocol.InstalledApplications -- PKEY_Devices_AepContainer_DialProtocol_InstalledApplications
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 6
//
//  List of applications supporting DIAL protocol on the Device Association End Point Container
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_DialProtocol_InstalledApplications, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 6);
#define INIT_PKEY_Devices_AepContainer_DialProtocol_InstalledApplications { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 6 }

//  Name:     System.Devices.AepContainer.IsPaired -- PKEY_Devices_AepContainer_IsPaired
//  Type:     Boolean -- VT_BOOL
//  FormatID: {0BBA1EDE-7566-4F47-90EC-25FC567CED2A}, 4
//
//  Whether one of the child Device Association Endpoints is paired with the system or not.
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_IsPaired, 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A, 4);
#define INIT_PKEY_Devices_AepContainer_IsPaired { { 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A }, 4 }

//  Name:     System.Devices.AepContainer.IsPresent -- PKEY_Devices_AepContainer_IsPresent
//  Type:     Boolean -- VT_BOOL
//  FormatID: {0BBA1EDE-7566-4F47-90EC-25FC567CED2A}, 11
//
//  Whether one of the Device Association Endpoints is currently present or not
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_IsPresent, 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A, 11);
#define INIT_PKEY_Devices_AepContainer_IsPresent { { 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A }, 11 }

//  Name:     System.Devices.AepContainer.Manufacturer -- PKEY_Devices_AepContainer_Manufacturer
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {0BBA1EDE-7566-4F47-90EC-25FC567CED2A}, 6
//
//  Manufacturer of the device
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_Manufacturer, 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A, 6);
#define INIT_PKEY_Devices_AepContainer_Manufacturer { { 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A }, 6 }

//  Name:     System.Devices.AepContainer.ModelIds -- PKEY_Devices_AepContainer_ModelIds
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {0BBA1EDE-7566-4F47-90EC-25FC567CED2A}, 8
//
//  List of Model Ids for the device. Each Model Id is a Guid in string form.
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_ModelIds, 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A, 8);
#define INIT_PKEY_Devices_AepContainer_ModelIds { { 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A }, 8 }

//  Name:     System.Devices.AepContainer.ModelName -- PKEY_Devices_AepContainer_ModelName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {0BBA1EDE-7566-4F47-90EC-25FC567CED2A}, 7
//
//  Model Name of the device
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_ModelName, 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A, 7);
#define INIT_PKEY_Devices_AepContainer_ModelName { { 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A }, 7 }

//  Name:     System.Devices.AepContainer.ProtocolIds -- PKEY_Devices_AepContainer_ProtocolIds
//  Type:     Multivalue Guid -- VT_VECTOR | VT_CLSID  (For variants: VT_ARRAY | VT_CLSID)
//  FormatID: {0BBA1EDE-7566-4F47-90EC-25FC567CED2A}, 13
//
//  List of Protocol Ids that have contributed to building the Device Association Endpoint Container
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_ProtocolIds, 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A, 13);
#define INIT_PKEY_Devices_AepContainer_ProtocolIds { { 0x0BBA1EDE, 0x7566, 0x4F47, 0x90, 0xEC, 0x25, 0xFC, 0x56, 0x7C, 0xED, 0x2A }, 13 }

//  Name:     System.Devices.AepContainer.SupportedUriSchemes -- PKEY_Devices_AepContainer_SupportedUriSchemes
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 5
//
//  List of Casting Uri Schemes Supported by the Device Association Endpoint Container
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportedUriSchemes, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 5);
#define INIT_PKEY_Devices_AepContainer_SupportedUriSchemes { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 5 }

//  Name:     System.Devices.AepContainer.SupportsAudio -- PKEY_Devices_AepContainer_SupportsAudio
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 2
//
//  Indicates if the Device Association Endpoint Container Supports Audio Casting
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsAudio, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 2);
#define INIT_PKEY_Devices_AepContainer_SupportsAudio { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 2 }

//  Name:     System.Devices.AepContainer.SupportsCapturing -- PKEY_Devices_AepContainer_SupportsCapturing
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 11
//
//  Indicates if the Device Association Endpoint Container Supports Capturing
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsCapturing, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 11);
#define INIT_PKEY_Devices_AepContainer_SupportsCapturing { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 11 }

//  Name:     System.Devices.AepContainer.SupportsImages -- PKEY_Devices_AepContainer_SupportsImages
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 4
//
//  Indicates if the Device Association Endpoint Container Supports Image Casting
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsImages, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 4);
#define INIT_PKEY_Devices_AepContainer_SupportsImages { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 4 }

//  Name:     System.Devices.AepContainer.SupportsInformation -- PKEY_Devices_AepContainer_SupportsInformation
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 14
//
//  Indicates if the Device Association Endpoint Container Supports Information
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsInformation, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 14);
#define INIT_PKEY_Devices_AepContainer_SupportsInformation { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 14 }

//  Name:     System.Devices.AepContainer.SupportsLimitedDiscovery -- PKEY_Devices_AepContainer_SupportsLimitedDiscovery
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 7
//
//  Indicates if the Device Association Endpoint Container Supports Limited Discovery
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsLimitedDiscovery, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 7);
#define INIT_PKEY_Devices_AepContainer_SupportsLimitedDiscovery { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 7 }

//  Name:     System.Devices.AepContainer.SupportsNetworking -- PKEY_Devices_AepContainer_SupportsNetworking
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 9
//
//  Indicates if the Device Association Endpoint Container Supports SupportsNetworking
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsNetworking, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 9);
#define INIT_PKEY_Devices_AepContainer_SupportsNetworking { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 9 }

//  Name:     System.Devices.AepContainer.SupportsObjectTransfer -- PKEY_Devices_AepContainer_SupportsObjectTransfer
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 12
//
//  Indicates if the Device Association Endpoint Container Supports Object Transfer
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsObjectTransfer, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 12);
#define INIT_PKEY_Devices_AepContainer_SupportsObjectTransfer { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 12 }

//  Name:     System.Devices.AepContainer.SupportsPositioning -- PKEY_Devices_AepContainer_SupportsPositioning
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 8
//
//  Indicates if the Device Association Endpoint Container Supports Positioning
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsPositioning, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 8);
#define INIT_PKEY_Devices_AepContainer_SupportsPositioning { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 8 }

//  Name:     System.Devices.AepContainer.SupportsRendering -- PKEY_Devices_AepContainer_SupportsRendering
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 10
//
//  Indicates if the Device Association Endpoint Container Supports Rendering
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsRendering, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 10);
#define INIT_PKEY_Devices_AepContainer_SupportsRendering { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 10 }

//  Name:     System.Devices.AepContainer.SupportsTelephony -- PKEY_Devices_AepContainer_SupportsTelephony
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 13
//
//  Indicates if the Device Association Endpoint Container Supports Telephony
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsTelephony, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 13);
#define INIT_PKEY_Devices_AepContainer_SupportsTelephony { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 13 }

//  Name:     System.Devices.AepContainer.SupportsVideo -- PKEY_Devices_AepContainer_SupportsVideo
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6AF55D45-38DB-4495-ACB0-D4728A3B8314}, 3
//
//  Indicates if the Device Association Endpoint Container Supports Video Casting
DEFINE_PROPERTYKEY(PKEY_Devices_AepContainer_SupportsVideo, 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14, 3);
#define INIT_PKEY_Devices_AepContainer_SupportsVideo { { 0x6AF55D45, 0x38DB, 0x4495, 0xAC, 0xB0, 0xD4, 0x72, 0x8A, 0x3B, 0x83, 0x14 }, 3 }

//  Name:     System.Devices.AepService.AepId -- PKEY_Devices_AepService_AepId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9C141A9-1B4C-4F17-A9D1-F298538CADB8}, 6
//
//  Device Association Endpoint Service's Parent AEP Id
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_AepId, 0xC9C141A9, 0x1B4C, 0x4F17, 0xA9, 0xD1, 0xF2, 0x98, 0x53, 0x8C, 0xAD, 0xB8, 6);
#define INIT_PKEY_Devices_AepService_AepId { { 0xC9C141A9, 0x1B4C, 0x4F17, 0xA9, 0xD1, 0xF2, 0x98, 0x53, 0x8C, 0xAD, 0xB8 }, 6 }

//  Name:     System.Devices.AepService.Bluetooth.CacheMode -- PKEY_Devices_AepService_Bluetooth_CacheMode
//  Type:     Byte -- VT_UI1
//  FormatID: {9744311E-7951-4B2E-B6F0-ECB293CAC119}, 5
//
//  Bluetooth caching mode for the query.
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_Bluetooth_CacheMode, 0x9744311E, 0x7951, 0x4B2E, 0xB6, 0xF0, 0xEC, 0xB2, 0x93, 0xCA, 0xC1, 0x19, 5);
#define INIT_PKEY_Devices_AepService_Bluetooth_CacheMode { { 0x9744311E, 0x7951, 0x4B2E, 0xB6, 0xF0, 0xEC, 0xB2, 0x93, 0xCA, 0xC1, 0x19 }, 5 }

// Possible discrete values for PKEY_Devices_AepService_Bluetooth_CacheMode are:
#define BLUETOOTH_CACHE_MODE_CACHED         0
#define BLUETOOTH_CACHED_MODE_UNCACHED      1

//  Name:     System.Devices.AepService.Bluetooth.ServiceGuid -- PKEY_Devices_AepService_Bluetooth_ServiceGuid
//  Type:     Guid -- VT_CLSID
//  FormatID: {A399AAC7-C265-474E-B073-FFCE57721716}, 2
//
//  Bluetooth service GUID.
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_Bluetooth_ServiceGuid, 0xA399AAC7, 0xC265, 0x474E, 0xB0, 0x73, 0xFF, 0xCE, 0x57, 0x72, 0x17, 0x16, 2);
#define INIT_PKEY_Devices_AepService_Bluetooth_ServiceGuid { { 0xA399AAC7, 0xC265, 0x474E, 0xB0, 0x73, 0xFF, 0xCE, 0x57, 0x72, 0x17, 0x16 }, 2 }

//  Name:     System.Devices.AepService.Bluetooth.TargetDevice -- PKEY_Devices_AepService_Bluetooth_TargetDevice
//  Type:     UInt64 -- VT_UI8
//  FormatID: {9744311E-7951-4B2E-B6F0-ECB293CAC119}, 6
//
//  Bluetooth parent device for the query.  Required for uncached queries.
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_Bluetooth_TargetDevice, 0x9744311E, 0x7951, 0x4B2E, 0xB6, 0xF0, 0xEC, 0xB2, 0x93, 0xCA, 0xC1, 0x19, 6);
#define INIT_PKEY_Devices_AepService_Bluetooth_TargetDevice { { 0x9744311E, 0x7951, 0x4B2E, 0xB6, 0xF0, 0xEC, 0xB2, 0x93, 0xCA, 0xC1, 0x19 }, 6 }

//  Name:     System.Devices.AepService.ContainerId -- PKEY_Devices_AepService_ContainerId
//  Type:     Guid -- VT_CLSID
//  FormatID: {71724756-3E74-4432-9B59-E7B2F668A593}, 4
//
//  Device Association Endpoint Service's Parent Container Id
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_ContainerId, 0x71724756, 0x3E74, 0x4432, 0x9B, 0x59, 0xE7, 0xB2, 0xF6, 0x68, 0xA5, 0x93, 4);
#define INIT_PKEY_Devices_AepService_ContainerId { { 0x71724756, 0x3E74, 0x4432, 0x9B, 0x59, 0xE7, 0xB2, 0xF6, 0x68, 0xA5, 0x93 }, 4 }

//  Name:     System.Devices.AepService.FriendlyName -- PKEY_Devices_AepService_FriendlyName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {71724756-3E74-4432-9B59-E7B2F668A593}, 2
//
//  Device Association Endpoint Service Friendly Name
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_FriendlyName, 0x71724756, 0x3E74, 0x4432, 0x9B, 0x59, 0xE7, 0xB2, 0xF6, 0x68, 0xA5, 0x93, 2);
#define INIT_PKEY_Devices_AepService_FriendlyName { { 0x71724756, 0x3E74, 0x4432, 0x9B, 0x59, 0xE7, 0xB2, 0xF6, 0x68, 0xA5, 0x93 }, 2 }

//  Name:     System.Devices.AepService.IoT.ServiceInterfaces -- PKEY_Devices_AepService_IoT_ServiceInterfaces
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {79D94E82-4D79-45AA-821A-74858B4E4CA6}, 2
//
//  List of interfaces that are available for this service.
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_IoT_ServiceInterfaces, 0x79D94E82, 0x4D79, 0x45AA, 0x82, 0x1A, 0x74, 0x85, 0x8B, 0x4E, 0x4C, 0xA6, 2);
#define INIT_PKEY_Devices_AepService_IoT_ServiceInterfaces { { 0x79D94E82, 0x4D79, 0x45AA, 0x82, 0x1A, 0x74, 0x85, 0x8B, 0x4E, 0x4C, 0xA6 }, 2 }

//  Name:     System.Devices.AepService.ParentAepIsPaired -- PKEY_Devices_AepService_ParentAepIsPaired
//  Type:     Boolean -- VT_BOOL
//  FormatID: {C9C141A9-1B4C-4F17-A9D1-F298538CADB8}, 7
//
//  Whether the parent Device Association Endpoint is paired with the system or not
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_ParentAepIsPaired, 0xC9C141A9, 0x1B4C, 0x4F17, 0xA9, 0xD1, 0xF2, 0x98, 0x53, 0x8C, 0xAD, 0xB8, 7);
#define INIT_PKEY_Devices_AepService_ParentAepIsPaired { { 0xC9C141A9, 0x1B4C, 0x4F17, 0xA9, 0xD1, 0xF2, 0x98, 0x53, 0x8C, 0xAD, 0xB8 }, 7 }

//  Name:     System.Devices.AepService.ProtocolId -- PKEY_Devices_AepService_ProtocolId
//  Type:     Guid -- VT_CLSID
//  FormatID: {C9C141A9-1B4C-4F17-A9D1-F298538CADB8}, 5
//
//  Identity of the protocol this Device Association Endpoint Service was discovered over
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_ProtocolId, 0xC9C141A9, 0x1B4C, 0x4F17, 0xA9, 0xD1, 0xF2, 0x98, 0x53, 0x8C, 0xAD, 0xB8, 5);
#define INIT_PKEY_Devices_AepService_ProtocolId { { 0xC9C141A9, 0x1B4C, 0x4F17, 0xA9, 0xD1, 0xF2, 0x98, 0x53, 0x8C, 0xAD, 0xB8 }, 5 }

//  Name:     System.Devices.AepService.ServiceClassId -- PKEY_Devices_AepService_ServiceClassId
//  Type:     Guid -- VT_CLSID
//  FormatID: {71724756-3E74-4432-9B59-E7B2F668A593}, 3
//
//  Identity of the service this Device Association Endpoint Service represents
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_ServiceClassId, 0x71724756, 0x3E74, 0x4432, 0x9B, 0x59, 0xE7, 0xB2, 0xF6, 0x68, 0xA5, 0x93, 3);
#define INIT_PKEY_Devices_AepService_ServiceClassId { { 0x71724756, 0x3E74, 0x4432, 0x9B, 0x59, 0xE7, 0xB2, 0xF6, 0x68, 0xA5, 0x93 }, 3 }

//  Name:     System.Devices.AepService.ServiceId -- PKEY_Devices_AepService_ServiceId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9C141A9-1B4C-4F17-A9D1-F298538CADB8}, 2
//
//  Device Association Endpoint Service's Id
DEFINE_PROPERTYKEY(PKEY_Devices_AepService_ServiceId, 0xC9C141A9, 0x1B4C, 0x4F17, 0xA9, 0xD1, 0xF2, 0x98, 0x53, 0x8C, 0xAD, 0xB8, 2);
#define INIT_PKEY_Devices_AepService_ServiceId { { 0xC9C141A9, 0x1B4C, 0x4F17, 0xA9, 0xD1, 0xF2, 0x98, 0x53, 0x8C, 0xAD, 0xB8 }, 2 }

//  Name:     System.Devices.AppPackageFamilyName -- PKEY_Devices_AppPackageFamilyName
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {51236583-0C4A-4FE8-B81F-166AEC13F510}, 100
//
//  The package family name registered as the app for this device.
DEFINE_PROPERTYKEY(PKEY_Devices_AppPackageFamilyName, 0x51236583, 0x0C4A, 0x4FE8, 0xB8, 0x1F, 0x16, 0x6A, 0xEC, 0x13, 0xF5, 0x10, 100);
#define INIT_PKEY_Devices_AppPackageFamilyName { { 0x51236583, 0x0C4A, 0x4FE8, 0xB8, 0x1F, 0x16, 0x6A, 0xEC, 0x13, 0xF5, 0x10 }, 100 }

//  Name:     System.Devices.AudioDevice.Microphone.EqCoefficientsDb -- PKEY_Devices_AudioDevice_Microphone_EqCoefficientsDb
//  Type:     DoubleVector -- VT_VECTOR | VT_R8  (For variants: VT_ARRAY | VT_R8)
//  FormatID: {8943B373-388C-4395-B557-BC6DBAFFAFDB}, 7
//
//  Equalization coefficients for Microphone. Array of 960 DOUBLE frequency-domain gain coefficients to be applied by effect pack processing for microphone equalization, in dB.
DEFINE_PROPERTYKEY(PKEY_Devices_AudioDevice_Microphone_EqCoefficientsDb, 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB, 7);
#define INIT_PKEY_Devices_AudioDevice_Microphone_EqCoefficientsDb { { 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB }, 7 }

//  Name:     System.Devices.AudioDevice.Microphone.IsFarField -- PKEY_Devices_AudioDevice_Microphone_IsFarField
//  Type:     Boolean -- VT_BOOL
//  FormatID: {8943B373-388C-4395-B557-BC6DBAFFAFDB}, 6
//
//  Far field capability of the microphone. If VARIANT_TRUE the microphone element will detect far field sound.
DEFINE_PROPERTYKEY(PKEY_Devices_AudioDevice_Microphone_IsFarField, 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB, 6);
#define INIT_PKEY_Devices_AudioDevice_Microphone_IsFarField { { 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB }, 6 }

//  Name:     System.Devices.AudioDevice.Microphone.SensitivityInDbfs -- PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs
//  Type:     Double -- VT_R8
//  FormatID: {8943B373-388C-4395-B557-BC6DBAFFAFDB}, 3
//
//  Sensitivity information in dBFS for a microphone device.
DEFINE_PROPERTYKEY(PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs, 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB, 3);
#define INIT_PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs { { 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB }, 3 }

//  Name:     System.Devices.AudioDevice.Microphone.SensitivityInDbfs2 -- PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs2
//  Type:     Double -- VT_R8
//  FormatID: {8943B373-388C-4395-B557-BC6DBAFFAFDB}, 5
//
//  Sensitivity information in dBFS for a microphone device, measured after fixed hardware gain (if available). Assumes 0dB software gain
DEFINE_PROPERTYKEY(PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs2, 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB, 5);
#define INIT_PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs2 { { 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB }, 5 }

//  Name:     System.Devices.AudioDevice.Microphone.SignalToNoiseRatioInDb -- PKEY_Devices_AudioDevice_Microphone_SignalToNoiseRatioInDb
//  Type:     Double -- VT_R8
//  FormatID: {8943B373-388C-4395-B557-BC6DBAFFAFDB}, 4
//
//  Signal to noise ratio information in dB for a microphone device.
DEFINE_PROPERTYKEY(PKEY_Devices_AudioDevice_Microphone_SignalToNoiseRatioInDb, 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB, 4);
#define INIT_PKEY_Devices_AudioDevice_Microphone_SignalToNoiseRatioInDb { { 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB }, 4 }

//  Name:     System.Devices.AudioDevice.RawProcessingSupported -- PKEY_Devices_AudioDevice_RawProcessingSupported
//  Type:     Boolean -- VT_BOOL
//  FormatID: {8943B373-388C-4395-B557-BC6DBAFFAFDB}, 2
//
//  Raw processing mode support for the audio device. If VARIANT_TRUE the device supports raw processing mode.
DEFINE_PROPERTYKEY(PKEY_Devices_AudioDevice_RawProcessingSupported, 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB, 2);
#define INIT_PKEY_Devices_AudioDevice_RawProcessingSupported { { 0x8943B373, 0x388C, 0x4395, 0xB5, 0x57, 0xBC, 0x6D, 0xBA, 0xFF, 0xAF, 0xDB }, 2 }

//  Name:     System.Devices.AudioDevice.SpeechProcessingSupported -- PKEY_Devices_AudioDevice_SpeechProcessingSupported
//  Type:     Boolean -- VT_BOOL
//  FormatID: {FB1DE864-E06D-47F4-82A6-8A0AEF44493C}, 2
//
//  Speech mode support for the audio device. If VARIANT_TRUE the device supports speech mode.
DEFINE_PROPERTYKEY(PKEY_Devices_AudioDevice_SpeechProcessingSupported, 0xFB1DE864, 0xE06D, 0x47F4, 0x82, 0xA6, 0x8A, 0x0A, 0xEF, 0x44, 0x49, 0x3C, 2);
#define INIT_PKEY_Devices_AudioDevice_SpeechProcessingSupported { { 0xFB1DE864, 0xE06D, 0x47F4, 0x82, 0xA6, 0x8A, 0x0A, 0xEF, 0x44, 0x49, 0x3C }, 2 }

//  Name:     System.Devices.BatteryLife -- PKEY_Devices_BatteryLife
//  Type:     Byte -- VT_UI1
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 10
//
//  Remaining battery life of the device as an integer between 0 and 100 percent.
DEFINE_PROPERTYKEY(PKEY_Devices_BatteryLife, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 10);
#define INIT_PKEY_Devices_BatteryLife { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 10 }

//  Name:     System.Devices.BatteryPlusCharging -- PKEY_Devices_BatteryPlusCharging
//  Type:     Byte -- VT_UI1
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 22
//
//  Remaining battery life of the device as an integer between 0 and 100 percent and the device's charging state.
DEFINE_PROPERTYKEY(PKEY_Devices_BatteryPlusCharging, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 22);
#define INIT_PKEY_Devices_BatteryPlusCharging { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 22 }

//  Name:     System.Devices.BatteryPlusChargingText -- PKEY_Devices_BatteryPlusChargingText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 23
//
//  Remaining battery life of the device and the device's charging state as a string.
DEFINE_PROPERTYKEY(PKEY_Devices_BatteryPlusChargingText, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 23);
#define INIT_PKEY_Devices_BatteryPlusChargingText { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 23 }

//  Name:     System.Devices.Category -- PKEY_Devices_Category
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 91
//
//  Singular form of device category.
DEFINE_PROPERTYKEY(PKEY_Devices_Category, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 91);
#define INIT_PKEY_Devices_Category { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 91 }

//  Name:     System.Devices.CategoryGroup -- PKEY_Devices_CategoryGroup
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 94
//
//  Description of the category group the device belong to.
DEFINE_PROPERTYKEY(PKEY_Devices_CategoryGroup, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 94);
#define INIT_PKEY_Devices_CategoryGroup { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 94 }

//  Name:     System.Devices.CategoryIds -- PKEY_Devices_CategoryIds
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 90
//
//  Indicates the actual raw category
DEFINE_PROPERTYKEY(PKEY_Devices_CategoryIds, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 90);
#define INIT_PKEY_Devices_CategoryIds { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 90 }

//  Name:     System.Devices.CategoryPlural -- PKEY_Devices_CategoryPlural
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 92
//
//  Plural form of device category.
DEFINE_PROPERTYKEY(PKEY_Devices_CategoryPlural, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 92);
#define INIT_PKEY_Devices_CategoryPlural { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 92 }

//  Name:     System.Devices.ChallengeAep -- PKEY_Devices_ChallengeAep
//  Type:     Boolean -- VT_BOOL
//  FormatID: {0774315E-B714-48EC-8DE8-8125C077AC11}, 2
//
//  Enable or Disable device presence challenging behavior for AEPs
DEFINE_PROPERTYKEY(PKEY_Devices_ChallengeAep, 0x0774315E, 0xB714, 0x48EC, 0x8D, 0xE8, 0x81, 0x25, 0xC0, 0x77, 0xAC, 0x11, 2);
#define INIT_PKEY_Devices_ChallengeAep { { 0x0774315E, 0xB714, 0x48EC, 0x8D, 0xE8, 0x81, 0x25, 0xC0, 0x77, 0xAC, 0x11 }, 2 }

//  Name:     System.Devices.ChargingState -- PKEY_Devices_ChargingState
//  Type:     Byte -- VT_UI1
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 11
//
//  Boolean value representing if the device is currently charging.
DEFINE_PROPERTYKEY(PKEY_Devices_ChargingState, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 11);
#define INIT_PKEY_Devices_ChargingState { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 11 }

//  Name:     System.Devices.Children -- PKEY_Devices_Children
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {4340A6C5-93FA-4706-972C-7B648008A5A7}, 9
//
//  Device instance ids of children of this device.
DEFINE_PROPERTYKEY(PKEY_Devices_Children, 0x4340A6C5, 0x93FA, 0x4706, 0x97, 0x2C, 0x7B, 0x64, 0x80, 0x08, 0xA5, 0xA7, 9);
#define INIT_PKEY_Devices_Children { { 0x4340A6C5, 0x93FA, 0x4706, 0x97, 0x2C, 0x7B, 0x64, 0x80, 0x08, 0xA5, 0xA7 }, 9 }

//  Name:     System.Devices.ClassGuid -- PKEY_Devices_ClassGuid
//  Type:     Guid -- VT_CLSID
//  FormatID: {A45C254E-DF1C-4EFD-8020-67D146A850E0}, 10
//
//  Device Class Guid.
DEFINE_PROPERTYKEY(PKEY_Devices_ClassGuid, 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0, 10);
#define INIT_PKEY_Devices_ClassGuid { { 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0 }, 10 }

//  Name:     System.Devices.CompatibleIds -- PKEY_Devices_CompatibleIds
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {A45C254E-DF1C-4EFD-8020-67D146A850E0}, 4
//
//  Compatible Ids.
DEFINE_PROPERTYKEY(PKEY_Devices_CompatibleIds, 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0, 4);
#define INIT_PKEY_Devices_CompatibleIds { { 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0 }, 4 }

//  Name:     System.Devices.Connected -- PKEY_Devices_Connected
//  Type:     Boolean -- VT_BOOL
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 55
//
//  Device connection state. If VARIANT_TRUE, indicates the device is currently connected to the computer.
DEFINE_PROPERTYKEY(PKEY_Devices_Connected, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 55);
#define INIT_PKEY_Devices_Connected { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 55 }

//  Name:     System.Devices.ContainerId -- PKEY_Devices_ContainerId
//  Type:     Guid -- VT_CLSID
//  FormatID: {8C7ED206-3F8A-4827-B3AB-AE9E1FAEFC6C}, 2
//
//  Device container ID.
DEFINE_PROPERTYKEY(PKEY_Devices_ContainerId, 0x8C7ED206, 0x3F8A, 0x4827, 0xB3, 0xAB, 0xAE, 0x9E, 0x1F, 0xAE, 0xFC, 0x6C, 2);
#define INIT_PKEY_Devices_ContainerId { { 0x8C7ED206, 0x3F8A, 0x4827, 0xB3, 0xAB, 0xAE, 0x9E, 0x1F, 0xAE, 0xFC, 0x6C }, 2 }

//  Name:     System.Devices.DefaultTooltip -- PKEY_Devices_DefaultTooltip
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {880F70A2-6082-47AC-8AAB-A739D1A300C3}, 153
//
//  Tooltip for default state
DEFINE_PROPERTYKEY(PKEY_Devices_DefaultTooltip, 0x880F70A2, 0x6082, 0x47AC, 0x8A, 0xAB, 0xA7, 0x39, 0xD1, 0xA3, 0x00, 0xC3, 153);
#define INIT_PKEY_Devices_DefaultTooltip { { 0x880F70A2, 0x6082, 0x47AC, 0x8A, 0xAB, 0xA7, 0x39, 0xD1, 0xA3, 0x00, 0xC3 }, 153 }

//  Name:     System.Devices.DeviceCapabilities -- PKEY_Devices_DeviceCapabilities
//  Type:     UInt32 -- VT_UI4
//  FormatID: {A45C254E-DF1C-4EFD-8020-67D146A850E0}, 17
//
//  Device Capabilities.
DEFINE_PROPERTYKEY(PKEY_Devices_DeviceCapabilities, 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0, 17);
#define INIT_PKEY_Devices_DeviceCapabilities { { 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0 }, 17 }

//  Name:     System.Devices.DeviceCharacteristics -- PKEY_Devices_DeviceCharacteristics
//  Type:     UInt32 -- VT_UI4
//  FormatID: {A45C254E-DF1C-4EFD-8020-67D146A850E0}, 29
//
//  Device Characteristics.
DEFINE_PROPERTYKEY(PKEY_Devices_DeviceCharacteristics, 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0, 29);
#define INIT_PKEY_Devices_DeviceCharacteristics { { 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0 }, 29 }

//  Name:     System.Devices.DeviceDescription1 -- PKEY_Devices_DeviceDescription1
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 81
//
//  First line of descriptive text about the device.
DEFINE_PROPERTYKEY(PKEY_Devices_DeviceDescription1, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 81);
#define INIT_PKEY_Devices_DeviceDescription1 { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 81 }

//  Name:     System.Devices.DeviceDescription2 -- PKEY_Devices_DeviceDescription2
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 82
//
//  Second line of descriptive text about the device.
DEFINE_PROPERTYKEY(PKEY_Devices_DeviceDescription2, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 82);
#define INIT_PKEY_Devices_DeviceDescription2 { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 82 }

//  Name:     System.Devices.DeviceHasProblem -- PKEY_Devices_DeviceHasProblem
//  Type:     Boolean -- VT_BOOL
//  FormatID: {540B947E-8B40-45BC-A8A2-6A0B894CBDA2}, 6
//
//  Device has a problem.
DEFINE_PROPERTYKEY(PKEY_Devices_DeviceHasProblem, 0x540B947E, 0x8B40, 0x45BC, 0xA8, 0xA2, 0x6A, 0x0B, 0x89, 0x4C, 0xBD, 0xA2, 6);
#define INIT_PKEY_Devices_DeviceHasProblem { { 0x540B947E, 0x8B40, 0x45BC, 0xA8, 0xA2, 0x6A, 0x0B, 0x89, 0x4C, 0xBD, 0xA2 }, 6 }

//  Name:     System.Devices.DeviceInstanceId -- PKEY_Devices_DeviceInstanceId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 256
//
//  Device instance Id.
DEFINE_PROPERTYKEY(PKEY_Devices_DeviceInstanceId, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 256);
#define INIT_PKEY_Devices_DeviceInstanceId { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 256 }

//  Name:     System.Devices.DeviceManufacturer -- PKEY_Devices_DeviceManufacturer
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A45C254E-DF1C-4EFD-8020-67D146A850E0}, 13
//
//  Device manufacturer. Property on device object
DEFINE_PROPERTYKEY(PKEY_Devices_DeviceManufacturer, 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0, 13);
#define INIT_PKEY_Devices_DeviceManufacturer { { 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0 }, 13 }

//  Name:     System.Devices.DevObjectType -- PKEY_Devices_DevObjectType
//  Type:     UInt32 -- VT_UI4
//  FormatID: {13673F42-A3D6-49F6-B4DA-AE46E0C5237C}, 2
//
//  DevQuery Device Object Type
DEFINE_PROPERTYKEY(PKEY_Devices_DevObjectType, 0x13673F42, 0xA3D6, 0x49F6, 0xB4, 0xDA, 0xAE, 0x46, 0xE0, 0xC5, 0x23, 0x7C, 2);
#define INIT_PKEY_Devices_DevObjectType { { 0x13673F42, 0xA3D6, 0x49F6, 0xB4, 0xDA, 0xAE, 0x46, 0xE0, 0xC5, 0x23, 0x7C }, 2 }

//  Name:     System.Devices.DialProtocol.InstalledApplications -- PKEY_Devices_DialProtocol_InstalledApplications
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {6845CC72-1B71-48C3-AF86-B09171A19B14}, 3
//
//  List of applications supporting DIAL protocol on the Device Association End Point
DEFINE_PROPERTYKEY(PKEY_Devices_DialProtocol_InstalledApplications, 0x6845CC72, 0x1B71, 0x48C3, 0xAF, 0x86, 0xB0, 0x91, 0x71, 0xA1, 0x9B, 0x14, 3);
#define INIT_PKEY_Devices_DialProtocol_InstalledApplications { { 0x6845CC72, 0x1B71, 0x48C3, 0xAF, 0x86, 0xB0, 0x91, 0x71, 0xA1, 0x9B, 0x14 }, 3 }

//  Name:     System.Devices.DiscoveryMethod -- PKEY_Devices_DiscoveryMethod
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 52
//
//  Device discovery method. This indicates on what transport or physical connection the device is discovered.
DEFINE_PROPERTYKEY(PKEY_Devices_DiscoveryMethod, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 52);
#define INIT_PKEY_Devices_DiscoveryMethod { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 52 }

//  Name:     System.Devices.Dnssd.Domain -- PKEY_Devices_Dnssd_Domain
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 3
//
//  Domain portion of DNS-SD service instance name. (e.g. ".local" in "myservice._http._tcp.local")
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_Domain, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 3);
#define INIT_PKEY_Devices_Dnssd_Domain { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 3 }

//  Name:     System.Devices.Dnssd.FullName -- PKEY_Devices_Dnssd_FullName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 5
//
//  Complete DNS-SD service instance name, including instance, service, and domain.
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_FullName, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 5);
#define INIT_PKEY_Devices_Dnssd_FullName { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 5 }

//  Name:     System.Devices.Dnssd.HostName -- PKEY_Devices_Dnssd_HostName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 7
//
//  DNS name of device is hosting the service.
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_HostName, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 7);
#define INIT_PKEY_Devices_Dnssd_HostName { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 7 }

//  Name:     System.Devices.Dnssd.InstanceName -- PKEY_Devices_Dnssd_InstanceName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 4
//
//  Instance portion of DNS-SD service instance name.(e.g. "myservice" in "myservice._http._tcp.local")
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_InstanceName, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 4);
#define INIT_PKEY_Devices_Dnssd_InstanceName { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 4 }

//  Name:     System.Devices.Dnssd.NetworkAdapterId -- PKEY_Devices_Dnssd_NetworkAdapterId
//  Type:     Guid -- VT_CLSID
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 11
//
//  GUID for the network adapter on which to search for a service.
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_NetworkAdapterId, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 11);
#define INIT_PKEY_Devices_Dnssd_NetworkAdapterId { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 11 }

//  Name:     System.Devices.Dnssd.PortNumber -- PKEY_Devices_Dnssd_PortNumber
//  Type:     UInt16 -- VT_UI2
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 12
//
//  Port number on which the service is listening.
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_PortNumber, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 12);
#define INIT_PKEY_Devices_Dnssd_PortNumber { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 12 }

//  Name:     System.Devices.Dnssd.Priority -- PKEY_Devices_Dnssd_Priority
//  Type:     UInt16 -- VT_UI2
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 9
//
//  SRV record priority field.  Not usually used.
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_Priority, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 9);
#define INIT_PKEY_Devices_Dnssd_Priority { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 9 }

//  Name:     System.Devices.Dnssd.ServiceName -- PKEY_Devices_Dnssd_ServiceName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 2
//
//  Service type portion of DNS-SD service instance name. (e.g. "_http._tcp" in "myservice._http._tcp.local")
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_ServiceName, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 2);
#define INIT_PKEY_Devices_Dnssd_ServiceName { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 2 }

//  Name:     System.Devices.Dnssd.TextAttributes -- PKEY_Devices_Dnssd_TextAttributes
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 6
//
//  Text data associated with the service instance.  Each string is typically a key-value pair, separated by "=".
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_TextAttributes, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 6);
#define INIT_PKEY_Devices_Dnssd_TextAttributes { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 6 }

//  Name:     System.Devices.Dnssd.Ttl -- PKEY_Devices_Dnssd_Ttl
//  Type:     UInt32 -- VT_UI4
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 10
//
//  SRV record Time-To-Live field.  Not usually used.
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_Ttl, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 10);
#define INIT_PKEY_Devices_Dnssd_Ttl { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 10 }

//  Name:     System.Devices.Dnssd.Weight -- PKEY_Devices_Dnssd_Weight
//  Type:     UInt16 -- VT_UI2
//  FormatID: {BF79C0AB-BB74-4CEE-B070-470B5AE202EA}, 8
//
//  SRV record weight field.  Not usually used.
DEFINE_PROPERTYKEY(PKEY_Devices_Dnssd_Weight, 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA, 8);
#define INIT_PKEY_Devices_Dnssd_Weight { { 0xBF79C0AB, 0xBB74, 0x4CEE, 0xB0, 0x70, 0x47, 0x0B, 0x5A, 0xE2, 0x02, 0xEA }, 8 }

//  Name:     System.Devices.FriendlyName -- PKEY_Devices_FriendlyName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {656A3BB3-ECC0-43FD-8477-4AE0404A96CD}, 12288
//
//  Device friendly name.
DEFINE_PROPERTYKEY(PKEY_Devices_FriendlyName, 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD, 12288);
#define INIT_PKEY_Devices_FriendlyName { { 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD }, 12288 }

//  Name:     System.Devices.FunctionPaths -- PKEY_Devices_FunctionPaths
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D08DD4C0-3A9E-462E-8290-7B636B2576B9}, 3
//
//  Available functions for this device.
DEFINE_PROPERTYKEY(PKEY_Devices_FunctionPaths, 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9, 3);
#define INIT_PKEY_Devices_FunctionPaths { { 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9 }, 3 }

//  Name:     System.Devices.GlyphIcon -- PKEY_Devices_GlyphIcon
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {51236583-0C4A-4FE8-B81F-166AEC13F510}, 123
//
//  Glyph Icon Path.
DEFINE_PROPERTYKEY(PKEY_Devices_GlyphIcon, 0x51236583, 0x0C4A, 0x4FE8, 0xB8, 0x1F, 0x16, 0x6A, 0xEC, 0x13, 0xF5, 0x10, 123);
#define INIT_PKEY_Devices_GlyphIcon { { 0x51236583, 0x0C4A, 0x4FE8, 0xB8, 0x1F, 0x16, 0x6A, 0xEC, 0x13, 0xF5, 0x10 }, 123 }

//  Name:     System.Devices.HardwareIds -- PKEY_Devices_HardwareIds
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {A45C254E-DF1C-4EFD-8020-67D146A850E0}, 3
//
//  Hardware Ids.
DEFINE_PROPERTYKEY(PKEY_Devices_HardwareIds, 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0, 3);
#define INIT_PKEY_Devices_HardwareIds { { 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0 }, 3 }

//  Name:     System.Devices.Icon -- PKEY_Devices_Icon
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 57
//
//  Icon Path.
DEFINE_PROPERTYKEY(PKEY_Devices_Icon, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 57);
#define INIT_PKEY_Devices_Icon { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 57 }

//  Name:     System.Devices.InLocalMachineContainer -- PKEY_Devices_InLocalMachineContainer
//  Type:     Boolean -- VT_BOOL
//  FormatID: {8C7ED206-3F8A-4827-B3AB-AE9E1FAEFC6C}, 4
//
//  Device is in the local machine container.
DEFINE_PROPERTYKEY(PKEY_Devices_InLocalMachineContainer, 0x8C7ED206, 0x3F8A, 0x4827, 0xB3, 0xAB, 0xAE, 0x9E, 0x1F, 0xAE, 0xFC, 0x6C, 4);
#define INIT_PKEY_Devices_InLocalMachineContainer { { 0x8C7ED206, 0x3F8A, 0x4827, 0xB3, 0xAB, 0xAE, 0x9E, 0x1F, 0xAE, 0xFC, 0x6C }, 4 }

//  Name:     System.Devices.InterfaceClassGuid -- PKEY_Devices_InterfaceClassGuid
//  Type:     Guid -- VT_CLSID
//  FormatID: {026E516E-B814-414B-83CD-856D6FEF4822}, 4
//
//  Interface Class Guid.
DEFINE_PROPERTYKEY(PKEY_Devices_InterfaceClassGuid, 0x026E516E, 0xB814, 0x414B, 0x83, 0xCD, 0x85, 0x6D, 0x6F, 0xEF, 0x48, 0x22, 4);
#define INIT_PKEY_Devices_InterfaceClassGuid { { 0x026E516E, 0xB814, 0x414B, 0x83, 0xCD, 0x85, 0x6D, 0x6F, 0xEF, 0x48, 0x22 }, 4 }

//  Name:     System.Devices.InterfaceEnabled -- PKEY_Devices_InterfaceEnabled
//  Type:     Boolean -- VT_BOOL
//  FormatID: {026E516E-B814-414B-83CD-856D6FEF4822}, 3
//
//  Indicates if the interface is enabled or not.
DEFINE_PROPERTYKEY(PKEY_Devices_InterfaceEnabled, 0x026E516E, 0xB814, 0x414B, 0x83, 0xCD, 0x85, 0x6D, 0x6F, 0xEF, 0x48, 0x22, 3);
#define INIT_PKEY_Devices_InterfaceEnabled { { 0x026E516E, 0xB814, 0x414B, 0x83, 0xCD, 0x85, 0x6D, 0x6F, 0xEF, 0x48, 0x22 }, 3 }

//  Name:     System.Devices.InterfacePaths -- PKEY_Devices_InterfacePaths
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D08DD4C0-3A9E-462E-8290-7B636B2576B9}, 2
//
//  Available interfaces for this device.
DEFINE_PROPERTYKEY(PKEY_Devices_InterfacePaths, 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9, 2);
#define INIT_PKEY_Devices_InterfacePaths { { 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9 }, 2 }

//  Name:     System.Devices.IpAddress -- PKEY_Devices_IpAddress
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {656A3BB3-ECC0-43FD-8477-4AE0404A96CD}, 12297
//
//  IP address of the device.
DEFINE_PROPERTYKEY(PKEY_Devices_IpAddress, 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD, 12297);
#define INIT_PKEY_Devices_IpAddress { { 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD }, 12297 }

//  Name:     System.Devices.IsDefault -- PKEY_Devices_IsDefault
//  Type:     Boolean -- VT_BOOL
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 86
//
//  If VARIANT_TRUE, the device is the default device.
DEFINE_PROPERTYKEY(PKEY_Devices_IsDefault, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 86);
#define INIT_PKEY_Devices_IsDefault { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 86 }

//  Name:     System.Devices.IsNetworkConnected -- PKEY_Devices_IsNetworkConnected
//  Type:     Boolean -- VT_BOOL
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 85
//
//  If VARIANT_TRUE, the device is a network connected device.
DEFINE_PROPERTYKEY(PKEY_Devices_IsNetworkConnected, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 85);
#define INIT_PKEY_Devices_IsNetworkConnected { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 85 }

//  Name:     System.Devices.IsShared -- PKEY_Devices_IsShared
//  Type:     Boolean -- VT_BOOL
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 84
//
//  If VARIANT_TRUE, the device is shared.
DEFINE_PROPERTYKEY(PKEY_Devices_IsShared, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 84);
#define INIT_PKEY_Devices_IsShared { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 84 }

//  Name:     System.Devices.IsSoftwareInstalling -- PKEY_Devices_IsSoftwareInstalling
//  Type:     Boolean -- VT_BOOL
//  FormatID: {83DA6326-97A6-4088-9453-A1923F573B29}, 9
//
//  If VARIANT_TRUE, the device installer is currently installing software.
DEFINE_PROPERTYKEY(PKEY_Devices_IsSoftwareInstalling, 0x83DA6326, 0x97A6, 0x4088, 0x94, 0x53, 0xA1, 0x92, 0x3F, 0x57, 0x3B, 0x29, 9);
#define INIT_PKEY_Devices_IsSoftwareInstalling { { 0x83DA6326, 0x97A6, 0x4088, 0x94, 0x53, 0xA1, 0x92, 0x3F, 0x57, 0x3B, 0x29 }, 9 }

//  Name:     System.Devices.LaunchDeviceStageFromExplorer -- PKEY_Devices_LaunchDeviceStageFromExplorer
//  Type:     Boolean -- VT_BOOL
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 77
//
//  Indicates whether to launch Device Stage or not
DEFINE_PROPERTYKEY(PKEY_Devices_LaunchDeviceStageFromExplorer, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 77);
#define INIT_PKEY_Devices_LaunchDeviceStageFromExplorer { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 77 }

//  Name:     System.Devices.LocalMachine -- PKEY_Devices_LocalMachine
//  Type:     Boolean -- VT_BOOL
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 70
//
//  If VARIANT_TRUE, the device container represents the local machine itself.
DEFINE_PROPERTYKEY(PKEY_Devices_LocalMachine, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 70);
#define INIT_PKEY_Devices_LocalMachine { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 70 }

//  Name:     System.Devices.LocationPaths -- PKEY_Devices_LocationPaths
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {A45C254E-DF1C-4EFD-8020-67D146A850E0}, 37
//
//  Device LocationPaths.
DEFINE_PROPERTYKEY(PKEY_Devices_LocationPaths, 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0, 37);
#define INIT_PKEY_Devices_LocationPaths { { 0xA45C254E, 0xDF1C, 0x4EFD, 0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0 }, 37 }

//  Name:     System.Devices.Manufacturer -- PKEY_Devices_Manufacturer
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {656A3BB3-ECC0-43FD-8477-4AE0404A96CD}, 8192
//
//  Device manufacturer.
DEFINE_PROPERTYKEY(PKEY_Devices_Manufacturer, 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD, 8192);
#define INIT_PKEY_Devices_Manufacturer { { 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD }, 8192 }

//  Name:     System.Devices.MetadataPath -- PKEY_Devices_MetadataPath
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 71
//
//  Path to metadata for the device.
DEFINE_PROPERTYKEY(PKEY_Devices_MetadataPath, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 71);
#define INIT_PKEY_Devices_MetadataPath { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 71 }

//  Name:     System.Devices.MicrophoneArray.Geometry -- PKEY_Devices_MicrophoneArray_Geometry
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {A1829EA2-27EB-459E-935D-B2FAD7B07762}, 2
//
//  Geometry data for the microphone array.
DEFINE_PROPERTYKEY(PKEY_Devices_MicrophoneArray_Geometry, 0xA1829EA2, 0x27EB, 0x459E, 0x93, 0x5D, 0xB2, 0xFA, 0xD7, 0xB0, 0x77, 0x62, 2);
#define INIT_PKEY_Devices_MicrophoneArray_Geometry { { 0xA1829EA2, 0x27EB, 0x459E, 0x93, 0x5D, 0xB2, 0xFA, 0xD7, 0xB0, 0x77, 0x62 }, 2 }

//  Name:     System.Devices.MissedCalls -- PKEY_Devices_MissedCalls
//  Type:     Byte -- VT_UI1
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 5
//
//  Number of missed calls on the device.
DEFINE_PROPERTYKEY(PKEY_Devices_MissedCalls, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 5);
#define INIT_PKEY_Devices_MissedCalls { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 5 }

//  Name:     System.Devices.ModelId -- PKEY_Devices_ModelId
//  Type:     Guid -- VT_CLSID
//  FormatID: {80D81EA6-7473-4B0C-8216-EFC11A2C4C8B}, 2
//
//  Model Id
DEFINE_PROPERTYKEY(PKEY_Devices_ModelId, 0x80D81EA6, 0x7473, 0x4B0C, 0x82, 0x16, 0xEF, 0xC1, 0x1A, 0x2C, 0x4C, 0x8B, 2);
#define INIT_PKEY_Devices_ModelId { { 0x80D81EA6, 0x7473, 0x4B0C, 0x82, 0x16, 0xEF, 0xC1, 0x1A, 0x2C, 0x4C, 0x8B }, 2 }

//  Name:     System.Devices.ModelName -- PKEY_Devices_ModelName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {656A3BB3-ECC0-43FD-8477-4AE0404A96CD}, 8194
//
//  Model name of the device.
DEFINE_PROPERTYKEY(PKEY_Devices_ModelName, 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD, 8194);
#define INIT_PKEY_Devices_ModelName { { 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD }, 8194 }

//  Name:     System.Devices.ModelNumber -- PKEY_Devices_ModelNumber
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {656A3BB3-ECC0-43FD-8477-4AE0404A96CD}, 8195
//
//  Model number of the device.
DEFINE_PROPERTYKEY(PKEY_Devices_ModelNumber, 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD, 8195);
#define INIT_PKEY_Devices_ModelNumber { { 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD }, 8195 }

//  Name:     System.Devices.NetworkedTooltip -- PKEY_Devices_NetworkedTooltip
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {880F70A2-6082-47AC-8AAB-A739D1A300C3}, 152
//
//  Tooltip for connection state
DEFINE_PROPERTYKEY(PKEY_Devices_NetworkedTooltip, 0x880F70A2, 0x6082, 0x47AC, 0x8A, 0xAB, 0xA7, 0x39, 0xD1, 0xA3, 0x00, 0xC3, 152);
#define INIT_PKEY_Devices_NetworkedTooltip { { 0x880F70A2, 0x6082, 0x47AC, 0x8A, 0xAB, 0xA7, 0x39, 0xD1, 0xA3, 0x00, 0xC3 }, 152 }

//  Name:     System.Devices.NetworkName -- PKEY_Devices_NetworkName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 7
//
//  Name of the device's network.
DEFINE_PROPERTYKEY(PKEY_Devices_NetworkName, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 7);
#define INIT_PKEY_Devices_NetworkName { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 7 }

//  Name:     System.Devices.NetworkType -- PKEY_Devices_NetworkType
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 8
//
//  String representing the type of the device's network.
DEFINE_PROPERTYKEY(PKEY_Devices_NetworkType, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 8);
#define INIT_PKEY_Devices_NetworkType { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 8 }

//  Name:     System.Devices.NewPictures -- PKEY_Devices_NewPictures
//  Type:     UInt16 -- VT_UI2
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 4
//
//  Number of new pictures on the device.
DEFINE_PROPERTYKEY(PKEY_Devices_NewPictures, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 4);
#define INIT_PKEY_Devices_NewPictures { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 4 }

//  Name:     System.Devices.Notification -- PKEY_Devices_Notification
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {06704B0C-E830-4C81-9178-91E4E95A80A0}, 3
//
//  Device Notification Property.
DEFINE_PROPERTYKEY(PKEY_Devices_Notification, 0x06704B0C, 0xE830, 0x4C81, 0x91, 0x78, 0x91, 0xE4, 0xE9, 0x5A, 0x80, 0xA0, 3);
#define INIT_PKEY_Devices_Notification { { 0x06704B0C, 0xE830, 0x4C81, 0x91, 0x78, 0x91, 0xE4, 0xE9, 0x5A, 0x80, 0xA0 }, 3 }

//  Name:     System.Devices.Notifications.LowBattery -- PKEY_Devices_Notifications_LowBattery
//  Type:     Byte -- VT_UI1
//  FormatID: {C4C07F2B-8524-4E66-AE3A-A6235F103BEB}, 2
//
//  Device Low Battery Notification.
DEFINE_PROPERTYKEY(PKEY_Devices_Notifications_LowBattery, 0xC4C07F2B, 0x8524, 0x4E66, 0xAE, 0x3A, 0xA6, 0x23, 0x5F, 0x10, 0x3B, 0xEB, 2);
#define INIT_PKEY_Devices_Notifications_LowBattery { { 0xC4C07F2B, 0x8524, 0x4E66, 0xAE, 0x3A, 0xA6, 0x23, 0x5F, 0x10, 0x3B, 0xEB }, 2 }

//  Name:     System.Devices.Notifications.MissedCall -- PKEY_Devices_Notifications_MissedCall
//  Type:     Byte -- VT_UI1
//  FormatID: {6614EF48-4EFE-4424-9EDA-C79F404EDF3E}, 2
//
//  Device Missed Call Notification.
DEFINE_PROPERTYKEY(PKEY_Devices_Notifications_MissedCall, 0x6614EF48, 0x4EFE, 0x4424, 0x9E, 0xDA, 0xC7, 0x9F, 0x40, 0x4E, 0xDF, 0x3E, 2);
#define INIT_PKEY_Devices_Notifications_MissedCall { { 0x6614EF48, 0x4EFE, 0x4424, 0x9E, 0xDA, 0xC7, 0x9F, 0x40, 0x4E, 0xDF, 0x3E }, 2 }

//  Name:     System.Devices.Notifications.NewMessage -- PKEY_Devices_Notifications_NewMessage
//  Type:     Byte -- VT_UI1
//  FormatID: {2BE9260A-2012-4742-A555-F41B638B7DCB}, 2
//
//  Device New Message Notification.
DEFINE_PROPERTYKEY(PKEY_Devices_Notifications_NewMessage, 0x2BE9260A, 0x2012, 0x4742, 0xA5, 0x55, 0xF4, 0x1B, 0x63, 0x8B, 0x7D, 0xCB, 2);
#define INIT_PKEY_Devices_Notifications_NewMessage { { 0x2BE9260A, 0x2012, 0x4742, 0xA5, 0x55, 0xF4, 0x1B, 0x63, 0x8B, 0x7D, 0xCB }, 2 }

//  Name:     System.Devices.Notifications.NewVoicemail -- PKEY_Devices_Notifications_NewVoicemail
//  Type:     Byte -- VT_UI1
//  FormatID: {59569556-0A08-4212-95B9-FAE2AD6413DB}, 2
//
//  Device Voicemail Notification.
DEFINE_PROPERTYKEY(PKEY_Devices_Notifications_NewVoicemail, 0x59569556, 0x0A08, 0x4212, 0x95, 0xB9, 0xFA, 0xE2, 0xAD, 0x64, 0x13, 0xDB, 2);
#define INIT_PKEY_Devices_Notifications_NewVoicemail { { 0x59569556, 0x0A08, 0x4212, 0x95, 0xB9, 0xFA, 0xE2, 0xAD, 0x64, 0x13, 0xDB }, 2 }

//  Name:     System.Devices.Notifications.StorageFull -- PKEY_Devices_Notifications_StorageFull
//  Type:     UInt64 -- VT_UI8
//  FormatID: {A0E00EE1-F0C7-4D41-B8E7-26A7BD8D38B0}, 2
//
//  Device Storage Full Notification.
DEFINE_PROPERTYKEY(PKEY_Devices_Notifications_StorageFull, 0xA0E00EE1, 0xF0C7, 0x4D41, 0xB8, 0xE7, 0x26, 0xA7, 0xBD, 0x8D, 0x38, 0xB0, 2);
#define INIT_PKEY_Devices_Notifications_StorageFull { { 0xA0E00EE1, 0xF0C7, 0x4D41, 0xB8, 0xE7, 0x26, 0xA7, 0xBD, 0x8D, 0x38, 0xB0 }, 2 }

//  Name:     System.Devices.Notifications.StorageFullLinkText -- PKEY_Devices_Notifications_StorageFullLinkText
//  Type:     UInt64 -- VT_UI8
//  FormatID: {A0E00EE1-F0C7-4D41-B8E7-26A7BD8D38B0}, 3
//
//  Link Text for the Device Storage Full Notification.
DEFINE_PROPERTYKEY(PKEY_Devices_Notifications_StorageFullLinkText, 0xA0E00EE1, 0xF0C7, 0x4D41, 0xB8, 0xE7, 0x26, 0xA7, 0xBD, 0x8D, 0x38, 0xB0, 3);
#define INIT_PKEY_Devices_Notifications_StorageFullLinkText { { 0xA0E00EE1, 0xF0C7, 0x4D41, 0xB8, 0xE7, 0x26, 0xA7, 0xBD, 0x8D, 0x38, 0xB0 }, 3 }

//  Name:     System.Devices.NotificationStore -- PKEY_Devices_NotificationStore
//  Type:     Object -- VT_UNKNOWN
//  FormatID: {06704B0C-E830-4C81-9178-91E4E95A80A0}, 2
//
//  Device Notification Store.
DEFINE_PROPERTYKEY(PKEY_Devices_NotificationStore, 0x06704B0C, 0xE830, 0x4C81, 0x91, 0x78, 0x91, 0xE4, 0xE9, 0x5A, 0x80, 0xA0, 2);
#define INIT_PKEY_Devices_NotificationStore { { 0x06704B0C, 0xE830, 0x4C81, 0x91, 0x78, 0x91, 0xE4, 0xE9, 0x5A, 0x80, 0xA0 }, 2 }

//  Name:     System.Devices.NotWorkingProperly -- PKEY_Devices_NotWorkingProperly
//  Type:     Boolean -- VT_BOOL
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 83
//
//  If VARIANT_TRUE, the device is not working properly.
DEFINE_PROPERTYKEY(PKEY_Devices_NotWorkingProperly, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 83);
#define INIT_PKEY_Devices_NotWorkingProperly { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 83 }

//  Name:     System.Devices.Paired -- PKEY_Devices_Paired
//  Type:     Boolean -- VT_BOOL
//  FormatID: {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 56
//
//  Device paired state. If VARIANT_TRUE, indicates the device is not paired with the computer.
DEFINE_PROPERTYKEY(PKEY_Devices_Paired, 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57, 56);
#define INIT_PKEY_Devices_Paired { { 0x78C34FC8, 0x104A, 0x4ACA, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57 }, 56 }

//  Name:     System.Devices.Panel.PanelGroup -- PKEY_Devices_Panel_PanelGroup
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8DBC9C86-97A9-4BFF-9BC6-BFE95D3E6DAD}, 3
//
//  The group that this panel belongs to
DEFINE_PROPERTYKEY(PKEY_Devices_Panel_PanelGroup, 0x8DBC9C86, 0x97A9, 0x4BFF, 0x9B, 0xC6, 0xBF, 0xE9, 0x5D, 0x3E, 0x6D, 0xAD, 3);
#define INIT_PKEY_Devices_Panel_PanelGroup { { 0x8DBC9C86, 0x97A9, 0x4BFF, 0x9B, 0xC6, 0xBF, 0xE9, 0x5D, 0x3E, 0x6D, 0xAD }, 3 }

//  Name:     System.Devices.Panel.PanelId -- PKEY_Devices_Panel_PanelId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8DBC9C86-97A9-4BFF-9BC6-BFE95D3E6DAD}, 2
//
//  Identity of the Device Panel
DEFINE_PROPERTYKEY(PKEY_Devices_Panel_PanelId, 0x8DBC9C86, 0x97A9, 0x4BFF, 0x9B, 0xC6, 0xBF, 0xE9, 0x5D, 0x3E, 0x6D, 0xAD, 2);
#define INIT_PKEY_Devices_Panel_PanelId { { 0x8DBC9C86, 0x97A9, 0x4BFF, 0x9B, 0xC6, 0xBF, 0xE9, 0x5D, 0x3E, 0x6D, 0xAD }, 2 }

//  Name:     System.Devices.Parent -- PKEY_Devices_Parent
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4340A6C5-93FA-4706-972C-7B648008A5A7}, 8
//
//  Parent device.
DEFINE_PROPERTYKEY(PKEY_Devices_Parent, 0x4340A6C5, 0x93FA, 0x4706, 0x97, 0x2C, 0x7B, 0x64, 0x80, 0x08, 0xA5, 0xA7, 8);
#define INIT_PKEY_Devices_Parent { { 0x4340A6C5, 0x93FA, 0x4706, 0x97, 0x2C, 0x7B, 0x64, 0x80, 0x08, 0xA5, 0xA7 }, 8 }

//  Name:     System.Devices.PhoneLineTransportDevice.Connected -- PKEY_Devices_PhoneLineTransportDevice_Connected
//  Type:     Boolean -- VT_BOOL
//  FormatID: {AECF2FE8-1D00-4FEE-8A6D-A70D719B772B}, 2
//
//  Connection status of a PhoneLineTransportDevice.
DEFINE_PROPERTYKEY(PKEY_Devices_PhoneLineTransportDevice_Connected, 0xAECF2FE8, 0x1D00, 0x4FEE, 0x8A, 0x6D, 0xA7, 0x0D, 0x71, 0x9B, 0x77, 0x2B, 2);
#define INIT_PKEY_Devices_PhoneLineTransportDevice_Connected { { 0xAECF2FE8, 0x1D00, 0x4FEE, 0x8A, 0x6D, 0xA7, 0x0D, 0x71, 0x9B, 0x77, 0x2B }, 2 }

//  Name:     System.Devices.PhysicalDeviceLocation -- PKEY_Devices_PhysicalDeviceLocation
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {540B947E-8B40-45BC-A8A2-6A0B894CBDA2}, 9
//
//  ACPI _PLD data for the device.
DEFINE_PROPERTYKEY(PKEY_Devices_PhysicalDeviceLocation, 0x540B947E, 0x8B40, 0x45BC, 0xA8, 0xA2, 0x6A, 0x0B, 0x89, 0x4C, 0xBD, 0xA2, 9);
#define INIT_PKEY_Devices_PhysicalDeviceLocation { { 0x540B947E, 0x8B40, 0x45BC, 0xA8, 0xA2, 0x6A, 0x0B, 0x89, 0x4C, 0xBD, 0xA2 }, 9 }

//  Name:     System.Devices.PlaybackPositionPercent -- PKEY_Devices_PlaybackPositionPercent
//  Type:     UInt32 -- VT_UI4
//  FormatID: {3633DE59-6825-4381-A49B-9F6BA13A1471}, 5
//
//  Device playback position as a percent.
DEFINE_PROPERTYKEY(PKEY_Devices_PlaybackPositionPercent, 0x3633DE59, 0x6825, 0x4381, 0xA4, 0x9B, 0x9F, 0x6B, 0xA1, 0x3A, 0x14, 0x71, 5);
#define INIT_PKEY_Devices_PlaybackPositionPercent { { 0x3633DE59, 0x6825, 0x4381, 0xA4, 0x9B, 0x9F, 0x6B, 0xA1, 0x3A, 0x14, 0x71 }, 5 }

//  Name:     System.Devices.PlaybackState -- PKEY_Devices_PlaybackState
//  Type:     Byte -- VT_UI1
//  FormatID: {3633DE59-6825-4381-A49B-9F6BA13A1471}, 2
//
//  Device playback state.
DEFINE_PROPERTYKEY(PKEY_Devices_PlaybackState, 0x3633DE59, 0x6825, 0x4381, 0xA4, 0x9B, 0x9F, 0x6B, 0xA1, 0x3A, 0x14, 0x71, 2);
#define INIT_PKEY_Devices_PlaybackState { { 0x3633DE59, 0x6825, 0x4381, 0xA4, 0x9B, 0x9F, 0x6B, 0xA1, 0x3A, 0x14, 0x71 }, 2 }

// Possible discrete values for PKEY_Devices_PlaybackState are:
#define PLAYBACKSTATE_UNKNOWN               0
#define PLAYBACKSTATE_STOPPED               1
#define PLAYBACKSTATE_PLAYING               2
#define PLAYBACKSTATE_TRANSITIONING         3
#define PLAYBACKSTATE_PAUSED                4
#define PLAYBACKSTATE_RECORDINGPAUSED       5
#define PLAYBACKSTATE_RECORDING             6
#define PLAYBACKSTATE_NOMEDIA               7

//  Name:     System.Devices.PlaybackTitle -- PKEY_Devices_PlaybackTitle
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {3633DE59-6825-4381-A49B-9F6BA13A1471}, 3
//
//  Device current playback title.
DEFINE_PROPERTYKEY(PKEY_Devices_PlaybackTitle, 0x3633DE59, 0x6825, 0x4381, 0xA4, 0x9B, 0x9F, 0x6B, 0xA1, 0x3A, 0x14, 0x71, 3);
#define INIT_PKEY_Devices_PlaybackTitle { { 0x3633DE59, 0x6825, 0x4381, 0xA4, 0x9B, 0x9F, 0x6B, 0xA1, 0x3A, 0x14, 0x71 }, 3 }

//  Name:     System.Devices.Present -- PKEY_Devices_Present
//  Type:     Boolean -- VT_BOOL
//  FormatID: {540B947E-8B40-45BC-A8A2-6A0B894CBDA2}, 5
//
//  Device is present.
DEFINE_PROPERTYKEY(PKEY_Devices_Present, 0x540B947E, 0x8B40, 0x45BC, 0xA8, 0xA2, 0x6A, 0x0B, 0x89, 0x4C, 0xBD, 0xA2, 5);
#define INIT_PKEY_Devices_Present { { 0x540B947E, 0x8B40, 0x45BC, 0xA8, 0xA2, 0x6A, 0x0B, 0x89, 0x4C, 0xBD, 0xA2 }, 5 }

//  Name:     System.Devices.PresentationUrl -- PKEY_Devices_PresentationUrl
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {656A3BB3-ECC0-43FD-8477-4AE0404A96CD}, 8198
//
//  URL of a human readable webpage on the device.
DEFINE_PROPERTYKEY(PKEY_Devices_PresentationUrl, 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD, 8198);
#define INIT_PKEY_Devices_PresentationUrl { { 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD }, 8198 }

//  Name:     System.Devices.PrimaryCategory -- PKEY_Devices_PrimaryCategory
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {D08DD4C0-3A9E-462E-8290-7B636B2576B9}, 10
//
//  Primary category group for this device.
DEFINE_PROPERTYKEY(PKEY_Devices_PrimaryCategory, 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9, 10);
#define INIT_PKEY_Devices_PrimaryCategory { { 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9 }, 10 }

//  Name:     System.Devices.RemainingDuration -- PKEY_Devices_RemainingDuration
//  Type:     UInt64 -- VT_UI8
//  FormatID: {3633DE59-6825-4381-A49B-9F6BA13A1471}, 4
//
//  Remaining playing time in 100ns units.
DEFINE_PROPERTYKEY(PKEY_Devices_RemainingDuration, 0x3633DE59, 0x6825, 0x4381, 0xA4, 0x9B, 0x9F, 0x6B, 0xA1, 0x3A, 0x14, 0x71, 4);
#define INIT_PKEY_Devices_RemainingDuration { { 0x3633DE59, 0x6825, 0x4381, 0xA4, 0x9B, 0x9F, 0x6B, 0xA1, 0x3A, 0x14, 0x71 }, 4 }

//  Name:     System.Devices.RestrictedInterface -- PKEY_Devices_RestrictedInterface
//  Type:     Boolean -- VT_BOOL
//  FormatID: {026E516E-B814-414B-83CD-856D6FEF4822}, 6
//
//  Indicates if the interface is restricted.
DEFINE_PROPERTYKEY(PKEY_Devices_RestrictedInterface, 0x026E516E, 0xB814, 0x414B, 0x83, 0xCD, 0x85, 0x6D, 0x6F, 0xEF, 0x48, 0x22, 6);
#define INIT_PKEY_Devices_RestrictedInterface { { 0x026E516E, 0xB814, 0x414B, 0x83, 0xCD, 0x85, 0x6D, 0x6F, 0xEF, 0x48, 0x22 }, 6 }

//  Name:     System.Devices.Roaming -- PKEY_Devices_Roaming
//  Type:     Byte -- VT_UI1
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 9
//
//  Status indicator used to indicate if the device is roaming.
DEFINE_PROPERTYKEY(PKEY_Devices_Roaming, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 9);
#define INIT_PKEY_Devices_Roaming { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 9 }

//  Name:     System.Devices.SafeRemovalRequired -- PKEY_Devices_SafeRemovalRequired
//  Type:     Boolean -- VT_BOOL
//  FormatID: {AFD97640-86A3-4210-B67C-289C41AABE55}, 2
//
//  Indicates if a device requires safe removal or not
DEFINE_PROPERTYKEY(PKEY_Devices_SafeRemovalRequired, 0xAFD97640, 0x86A3, 0x4210, 0xB6, 0x7C, 0x28, 0x9C, 0x41, 0xAA, 0xBE, 0x55, 2);
#define INIT_PKEY_Devices_SafeRemovalRequired { { 0xAFD97640, 0x86A3, 0x4210, 0xB6, 0x7C, 0x28, 0x9C, 0x41, 0xAA, 0xBE, 0x55 }, 2 }

//  Name:     System.Devices.SchematicName -- PKEY_Devices_SchematicName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {026E516E-B814-414B-83CD-856D6FEF4822}, 9
//
//  Interface name for statically connected devices based on system schematics.
DEFINE_PROPERTYKEY(PKEY_Devices_SchematicName, 0x026E516E, 0xB814, 0x414B, 0x83, 0xCD, 0x85, 0x6D, 0x6F, 0xEF, 0x48, 0x22, 9);
#define INIT_PKEY_Devices_SchematicName { { 0x026E516E, 0xB814, 0x414B, 0x83, 0xCD, 0x85, 0x6D, 0x6F, 0xEF, 0x48, 0x22 }, 9 }

//  Name:     System.Devices.ServiceAddress -- PKEY_Devices_ServiceAddress
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {656A3BB3-ECC0-43FD-8477-4AE0404A96CD}, 16384
//
//  Endpoint address of the device service.
DEFINE_PROPERTYKEY(PKEY_Devices_ServiceAddress, 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD, 16384);
#define INIT_PKEY_Devices_ServiceAddress { { 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD }, 16384 }

//  Name:     System.Devices.ServiceId -- PKEY_Devices_ServiceId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {656A3BB3-ECC0-43FD-8477-4AE0404A96CD}, 16385
//
//  Identifier of the device service.
DEFINE_PROPERTYKEY(PKEY_Devices_ServiceId, 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD, 16385);
#define INIT_PKEY_Devices_ServiceId { { 0x656A3BB3, 0xECC0, 0x43FD, 0x84, 0x77, 0x4A, 0xE0, 0x40, 0x4A, 0x96, 0xCD }, 16385 }

//  Name:     System.Devices.SharedTooltip -- PKEY_Devices_SharedTooltip
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {880F70A2-6082-47AC-8AAB-A739D1A300C3}, 151
//
//  Tooltip for sharing state
DEFINE_PROPERTYKEY(PKEY_Devices_SharedTooltip, 0x880F70A2, 0x6082, 0x47AC, 0x8A, 0xAB, 0xA7, 0x39, 0xD1, 0xA3, 0x00, 0xC3, 151);
#define INIT_PKEY_Devices_SharedTooltip { { 0x880F70A2, 0x6082, 0x47AC, 0x8A, 0xAB, 0xA7, 0x39, 0xD1, 0xA3, 0x00, 0xC3 }, 151 }

//  Name:     System.Devices.SignalStrength -- PKEY_Devices_SignalStrength
//  Type:     Byte -- VT_UI1
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 2
//
//  Device signal strength.
DEFINE_PROPERTYKEY(PKEY_Devices_SignalStrength, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 2);
#define INIT_PKEY_Devices_SignalStrength { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 2 }

//  Name:     System.Devices.SmartCards.ReaderKind -- PKEY_Devices_SmartCards_ReaderKind
//  Type:     Byte -- VT_UI1
//  FormatID: {D6B5B883-18BD-4B4D-B2EC-9E38AFFEDA82}, 2
//
//  Smart card reader kind.
DEFINE_PROPERTYKEY(PKEY_Devices_SmartCards_ReaderKind, 0xD6B5B883, 0x18BD, 0x4B4D, 0xB2, 0xEC, 0x9E, 0x38, 0xAF, 0xFE, 0xDA, 0x82, 2);
#define INIT_PKEY_Devices_SmartCards_ReaderKind { { 0xD6B5B883, 0x18BD, 0x4B4D, 0xB2, 0xEC, 0x9E, 0x38, 0xAF, 0xFE, 0xDA, 0x82 }, 2 }

//  Name:     System.Devices.Status -- PKEY_Devices_Status
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D08DD4C0-3A9E-462E-8290-7B636B2576B9}, 259
//
//  Array of device status strings
DEFINE_PROPERTYKEY(PKEY_Devices_Status, 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9, 259);
#define INIT_PKEY_Devices_Status { { 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9 }, 259 }

//  Name:     System.Devices.Status1 -- PKEY_Devices_Status1
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {D08DD4C0-3A9E-462E-8290-7B636B2576B9}, 257
//
//  1st line of device status.
DEFINE_PROPERTYKEY(PKEY_Devices_Status1, 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9, 257);
#define INIT_PKEY_Devices_Status1 { { 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9 }, 257 }

//  Name:     System.Devices.Status2 -- PKEY_Devices_Status2
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {D08DD4C0-3A9E-462E-8290-7B636B2576B9}, 258
//
//  2nd line of device status.
DEFINE_PROPERTYKEY(PKEY_Devices_Status2, 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9, 258);
#define INIT_PKEY_Devices_Status2 { { 0xD08DD4C0, 0x3A9E, 0x462E, 0x82, 0x90, 0x7B, 0x63, 0x6B, 0x25, 0x76, 0xB9 }, 258 }

//  Name:     System.Devices.StorageCapacity -- PKEY_Devices_StorageCapacity
//  Type:     UInt64 -- VT_UI8
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 12
//
//  Total storage capacity of the device.
DEFINE_PROPERTYKEY(PKEY_Devices_StorageCapacity, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 12);
#define INIT_PKEY_Devices_StorageCapacity { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 12 }

//  Name:     System.Devices.StorageFreeSpace -- PKEY_Devices_StorageFreeSpace
//  Type:     UInt64 -- VT_UI8
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 13
//
//  Total free space of the storage of the device.
DEFINE_PROPERTYKEY(PKEY_Devices_StorageFreeSpace, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 13);
#define INIT_PKEY_Devices_StorageFreeSpace { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 13 }

//  Name:     System.Devices.StorageFreeSpacePercent -- PKEY_Devices_StorageFreeSpacePercent
//  Type:     UInt32 -- VT_UI4
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 14
//
//  Total free space of the storage of the device as a percentage.
DEFINE_PROPERTYKEY(PKEY_Devices_StorageFreeSpacePercent, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 14);
#define INIT_PKEY_Devices_StorageFreeSpacePercent { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 14 }

//  Name:     System.Devices.TextMessages -- PKEY_Devices_TextMessages
//  Type:     Byte -- VT_UI1
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 3
//
//  Number of unread text messages on the device.
DEFINE_PROPERTYKEY(PKEY_Devices_TextMessages, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 3);
#define INIT_PKEY_Devices_TextMessages { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 3 }

//  Name:     System.Devices.Voicemail -- PKEY_Devices_Voicemail
//  Type:     Byte -- VT_UI1
//  FormatID: {49CD1F76-5626-4B17-A4E8-18B4AA1A2213}, 6
//
//  Status indicator used to indicate if the device has voicemail.
DEFINE_PROPERTYKEY(PKEY_Devices_Voicemail, 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13, 6);
#define INIT_PKEY_Devices_Voicemail { { 0x49CD1F76, 0x5626, 0x4B17, 0xA4, 0xE8, 0x18, 0xB4, 0xAA, 0x1A, 0x22, 0x13 }, 6 }

//  Name:     System.Devices.WiaDeviceType -- PKEY_Devices_WiaDeviceType
//  Type:     UInt32 -- VT_UI4
//  FormatID: {6BDD1FC6-810F-11D0-BEC7-08002BE2092F}, 2
//
//  Windows Image Acquisition (WIA) device type.
DEFINE_PROPERTYKEY(PKEY_Devices_WiaDeviceType, 0x6BDD1FC6, 0x810F, 0x11D0, 0xBE, 0xC7, 0x08, 0x00, 0x2B, 0xE2, 0x09, 0x2F, 2);
#define INIT_PKEY_Devices_WiaDeviceType { { 0x6BDD1FC6, 0x810F, 0x11D0, 0xBE, 0xC7, 0x08, 0x00, 0x2B, 0xE2, 0x09, 0x2F }, 2 }

//  Name:     System.Devices.WiFi.InterfaceGuid -- PKEY_Devices_WiFi_InterfaceGuid
//  Type:     Guid -- VT_CLSID
//  FormatID: {EF1167EB-CBFC-4341-A568-A7C91A68982C}, 2
//
//  Wi-Fi Interface Guid
DEFINE_PROPERTYKEY(PKEY_Devices_WiFi_InterfaceGuid, 0xEF1167EB, 0xCBFC, 0x4341, 0xA5, 0x68, 0xA7, 0xC9, 0x1A, 0x68, 0x98, 0x2C, 2);
#define INIT_PKEY_Devices_WiFi_InterfaceGuid { { 0xEF1167EB, 0xCBFC, 0x4341, 0xA5, 0x68, 0xA7, 0xC9, 0x1A, 0x68, 0x98, 0x2C }, 2 }

//  Name:     System.Devices.WiFiDirect.DeviceAddress -- PKEY_Devices_WiFiDirect_DeviceAddress
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 13
//
//  Wi-Fi Direct Device Address
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_DeviceAddress, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 13);
#define INIT_PKEY_Devices_WiFiDirect_DeviceAddress { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 13 }

//  Name:     System.Devices.WiFiDirect.GroupId -- PKEY_Devices_WiFiDirect_GroupId
//  Type:     Guid -- VT_CLSID
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 4
//
//  Wi-Fi Direct Group Id
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_GroupId, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 4);
#define INIT_PKEY_Devices_WiFiDirect_GroupId { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 4 }

//  Name:     System.Devices.WiFiDirect.InformationElements -- PKEY_Devices_WiFiDirect_InformationElements
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 12
//
//  Indicates full set of IEs provided by the Wi-Fi Direct Device
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_InformationElements, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 12);
#define INIT_PKEY_Devices_WiFiDirect_InformationElements { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 12 }

//  Name:     System.Devices.WiFiDirect.InterfaceAddress -- PKEY_Devices_WiFiDirect_InterfaceAddress
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 2
//
//  Wi-Fi Direct Interface Address
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_InterfaceAddress, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 2);
#define INIT_PKEY_Devices_WiFiDirect_InterfaceAddress { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 2 }

//  Name:     System.Devices.WiFiDirect.InterfaceGuid -- PKEY_Devices_WiFiDirect_InterfaceGuid
//  Type:     Guid -- VT_CLSID
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 3
//
//  Wi-Fi Direct Interface GUID
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_InterfaceGuid, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 3);
#define INIT_PKEY_Devices_WiFiDirect_InterfaceGuid { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 3 }

//  Name:     System.Devices.WiFiDirect.IsConnected -- PKEY_Devices_WiFiDirect_IsConnected
//  Type:     Boolean -- VT_BOOL
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 5
//
//  Indicates Wi-Fi Direct Device's Connectivity State
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_IsConnected, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 5);
#define INIT_PKEY_Devices_WiFiDirect_IsConnected { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 5 }

//  Name:     System.Devices.WiFiDirect.IsLegacyDevice -- PKEY_Devices_WiFiDirect_IsLegacyDevice
//  Type:     Boolean -- VT_BOOL
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 7
//
//  Indicates if Wi-Fi Direct Device is a legacy device
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_IsLegacyDevice, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 7);
#define INIT_PKEY_Devices_WiFiDirect_IsLegacyDevice { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 7 }

//  Name:     System.Devices.WiFiDirect.IsMiracastLcpSupported -- PKEY_Devices_WiFiDirect_IsMiracastLcpSupported
//  Type:     Boolean -- VT_BOOL
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 9
//
//  Indicates if link content protection is supported by the Wi-Fi Direct Device if it is Miracast capable
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_IsMiracastLcpSupported, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 9);
#define INIT_PKEY_Devices_WiFiDirect_IsMiracastLcpSupported { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 9 }

//  Name:     System.Devices.WiFiDirect.IsVisible -- PKEY_Devices_WiFiDirect_IsVisible
//  Type:     Boolean -- VT_BOOL
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 6
//
//  Indicates Wi-Fi Direct Device's Current Visibility
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_IsVisible, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 6);
#define INIT_PKEY_Devices_WiFiDirect_IsVisible { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 6 }

//  Name:     System.Devices.WiFiDirect.MiracastVersion -- PKEY_Devices_WiFiDirect_MiracastVersion
//  Type:     UInt32 -- VT_UI4
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 8
//
//  Indicates version of Miracast protocol if Wi-Fi Direct Device is Miracast capable
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_MiracastVersion, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 8);
#define INIT_PKEY_Devices_WiFiDirect_MiracastVersion { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 8 }

//  Name:     System.Devices.WiFiDirect.Services -- PKEY_Devices_WiFiDirect_Services
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 10
//
//  Indicates services supported by the Wi-Fi Direct Device
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_Services, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 10);
#define INIT_PKEY_Devices_WiFiDirect_Services { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 10 }

//  Name:     System.Devices.WiFiDirect.SupportedChannelList -- PKEY_Devices_WiFiDirect_SupportedChannelList
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {1506935D-E3E7-450F-8637-82233EBE5F6E}, 11
//
//  Wi-Fi Direct device's channel list
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirect_SupportedChannelList, 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E, 11);
#define INIT_PKEY_Devices_WiFiDirect_SupportedChannelList { { 0x1506935D, 0xE3E7, 0x450F, 0x86, 0x37, 0x82, 0x23, 0x3E, 0xBE, 0x5F, 0x6E }, 11 }

//  Name:     System.Devices.WiFiDirectServices.AdvertisementId -- PKEY_Devices_WiFiDirectServices_AdvertisementId
//  Type:     UInt32 -- VT_UI4
//  FormatID: {31B37743-7C5E-4005-93E6-E953F92B82E9}, 5
//
//  Wi-Fi Direct Services Advertisement Id
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirectServices_AdvertisementId, 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9, 5);
#define INIT_PKEY_Devices_WiFiDirectServices_AdvertisementId { { 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9 }, 5 }

//  Name:     System.Devices.WiFiDirectServices.RequestServiceInformation -- PKEY_Devices_WiFiDirectServices_RequestServiceInformation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {31B37743-7C5E-4005-93E6-E953F92B82E9}, 7
//
//  Wi-Fi Direct Services Request Service Information
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirectServices_RequestServiceInformation, 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9, 7);
#define INIT_PKEY_Devices_WiFiDirectServices_RequestServiceInformation { { 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9 }, 7 }

//  Name:     System.Devices.WiFiDirectServices.ServiceAddress -- PKEY_Devices_WiFiDirectServices_ServiceAddress
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {31B37743-7C5E-4005-93E6-E953F92B82E9}, 2
//
//  Wi-Fi Direct Services Service Address
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirectServices_ServiceAddress, 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9, 2);
#define INIT_PKEY_Devices_WiFiDirectServices_ServiceAddress { { 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9 }, 2 }

//  Name:     System.Devices.WiFiDirectServices.ServiceConfigMethods -- PKEY_Devices_WiFiDirectServices_ServiceConfigMethods
//  Type:     UInt32 -- VT_UI4
//  FormatID: {31B37743-7C5E-4005-93E6-E953F92B82E9}, 6
//
//  Wi-Fi Direct Services Configuration Methods
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirectServices_ServiceConfigMethods, 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9, 6);
#define INIT_PKEY_Devices_WiFiDirectServices_ServiceConfigMethods { { 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9 }, 6 }

//  Name:     System.Devices.WiFiDirectServices.ServiceInformation -- PKEY_Devices_WiFiDirectServices_ServiceInformation
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {31B37743-7C5E-4005-93E6-E953F92B82E9}, 4
//
//  Wi-Fi Direct Services Service Information
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirectServices_ServiceInformation, 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9, 4);
#define INIT_PKEY_Devices_WiFiDirectServices_ServiceInformation { { 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9 }, 4 }

//  Name:     System.Devices.WiFiDirectServices.ServiceName -- PKEY_Devices_WiFiDirectServices_ServiceName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {31B37743-7C5E-4005-93E6-E953F92B82E9}, 3
//
//  Wi-Fi Direct Services Service Name
DEFINE_PROPERTYKEY(PKEY_Devices_WiFiDirectServices_ServiceName, 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9, 3);
#define INIT_PKEY_Devices_WiFiDirectServices_ServiceName { { 0x31B37743, 0x7C5E, 0x4005, 0x93, 0xE6, 0xE9, 0x53, 0xF9, 0x2B, 0x82, 0xE9 }, 3 }

//  Name:     System.Devices.WinPhone8CameraFlags -- PKEY_Devices_WinPhone8CameraFlags
//  Type:     UInt32 -- VT_UI4
//  FormatID: {B7B4D61C-5A64-4187-A52E-B1539F359099}, 2
//
//  Flags for a WP8 camera device.
DEFINE_PROPERTYKEY(PKEY_Devices_WinPhone8CameraFlags, 0xB7B4D61C, 0x5A64, 0x4187, 0xA5, 0x2E, 0xB1, 0x53, 0x9F, 0x35, 0x90, 0x99, 2);
#define INIT_PKEY_Devices_WinPhone8CameraFlags { { 0xB7B4D61C, 0x5A64, 0x4187, 0xA5, 0x2E, 0xB1, 0x53, 0x9F, 0x35, 0x90, 0x99 }, 2 }

//  Name:     System.Devices.Wwan.InterfaceGuid -- PKEY_Devices_Wwan_InterfaceGuid
//  Type:     Guid -- VT_CLSID
//  FormatID: {FF1167EB-CBFC-4341-A568-A7C91A68982C}, 2
//
//  WWAN Interface Guid
DEFINE_PROPERTYKEY(PKEY_Devices_Wwan_InterfaceGuid, 0xFF1167EB, 0xCBFC, 0x4341, 0xA5, 0x68, 0xA7, 0xC9, 0x1A, 0x68, 0x98, 0x2C, 2);
#define INIT_PKEY_Devices_Wwan_InterfaceGuid { { 0xFF1167EB, 0xCBFC, 0x4341, 0xA5, 0x68, 0xA7, 0xC9, 0x1A, 0x68, 0x98, 0x2C }, 2 }

//  Name:     System.Storage.Portable -- PKEY_Storage_Portable
//  Type:     Boolean -- VT_BOOL
//  FormatID: {4D1EBEE8-0803-4774-9842-B77DB50265E9}, 2
//
//  Indicates if the drive for the storage is portable.
DEFINE_PROPERTYKEY(PKEY_Storage_Portable, 0x4D1EBEE8, 0x0803, 0x4774, 0x98, 0x42, 0xB7, 0x7D, 0xB5, 0x02, 0x65, 0xE9, 2);
#define INIT_PKEY_Storage_Portable { { 0x4D1EBEE8, 0x0803, 0x4774, 0x98, 0x42, 0xB7, 0x7D, 0xB5, 0x02, 0x65, 0xE9 }, 2 }

//  Name:     System.Storage.RemovableMedia -- PKEY_Storage_RemovableMedia
//  Type:     Boolean -- VT_BOOL
//  FormatID: {4D1EBEE8-0803-4774-9842-B77DB50265E9}, 3
//
//  Indicates if the storage media is removable.
DEFINE_PROPERTYKEY(PKEY_Storage_RemovableMedia, 0x4D1EBEE8, 0x0803, 0x4774, 0x98, 0x42, 0xB7, 0x7D, 0xB5, 0x02, 0x65, 0xE9, 3);
#define INIT_PKEY_Storage_RemovableMedia { { 0x4D1EBEE8, 0x0803, 0x4774, 0x98, 0x42, 0xB7, 0x7D, 0xB5, 0x02, 0x65, 0xE9 }, 3 }

//  Name:     System.Storage.SystemCritical -- PKEY_Storage_SystemCritical
//  Type:     Boolean -- VT_BOOL
//  FormatID: {4D1EBEE8-0803-4774-9842-B77DB50265E9}, 4
//
//  Indicates if the storage media is system critical.
DEFINE_PROPERTYKEY(PKEY_Storage_SystemCritical, 0x4D1EBEE8, 0x0803, 0x4774, 0x98, 0x42, 0xB7, 0x7D, 0xB5, 0x02, 0x65, 0xE9, 4);
#define INIT_PKEY_Storage_SystemCritical { { 0x4D1EBEE8, 0x0803, 0x4774, 0x98, 0x42, 0xB7, 0x7D, 0xB5, 0x02, 0x65, 0xE9 }, 4 }

//-----------------------------------------------------------------------------
// Document properties

//  Name:     System.Document.ByteCount -- PKEY_Document_ByteCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 4 (PIDDSI_BYTECOUNT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_ByteCount, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 4);
#define INIT_PKEY_Document_ByteCount { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 4 }

//  Name:     System.Document.CharacterCount -- PKEY_Document_CharacterCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 16 (PIDSI_CHARCOUNT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_CharacterCount, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 16);
#define INIT_PKEY_Document_CharacterCount { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 16 }

//  Name:     System.Document.ClientID -- PKEY_Document_ClientID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {276D7BB0-5B34-4FB0-AA4B-158ED12A1809}, 100
DEFINE_PROPERTYKEY(PKEY_Document_ClientID, 0x276D7BB0, 0x5B34, 0x4FB0, 0xAA, 0x4B, 0x15, 0x8E, 0xD1, 0x2A, 0x18, 0x09, 100);
#define INIT_PKEY_Document_ClientID { { 0x276D7BB0, 0x5B34, 0x4FB0, 0xAA, 0x4B, 0x15, 0x8E, 0xD1, 0x2A, 0x18, 0x09 }, 100 }

//  Name:     System.Document.Contributor -- PKEY_Document_Contributor
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {F334115E-DA1B-4509-9B3D-119504DC7ABB}, 100
DEFINE_PROPERTYKEY(PKEY_Document_Contributor, 0xF334115E, 0xDA1B, 0x4509, 0x9B, 0x3D, 0x11, 0x95, 0x04, 0xDC, 0x7A, 0xBB, 100);
#define INIT_PKEY_Document_Contributor { { 0xF334115E, 0xDA1B, 0x4509, 0x9B, 0x3D, 0x11, 0x95, 0x04, 0xDC, 0x7A, 0xBB }, 100 }

//  Name:     System.Document.DateCreated -- PKEY_Document_DateCreated
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 12 (PIDSI_CREATE_DTM)
//  
//  This property is stored in the document, not obtained from the file system.
DEFINE_PROPERTYKEY(PKEY_Document_DateCreated, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 12);
#define INIT_PKEY_Document_DateCreated { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 12 }

//  Name:     System.Document.DatePrinted -- PKEY_Document_DatePrinted
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 11 (PIDSI_LASTPRINTED)
//
//  Legacy name: "DocLastPrinted".
DEFINE_PROPERTYKEY(PKEY_Document_DatePrinted, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 11);
#define INIT_PKEY_Document_DatePrinted { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 11 }

//  Name:     System.Document.DateSaved -- PKEY_Document_DateSaved
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 13 (PIDSI_LASTSAVE_DTM)
//
//  Legacy name: "DocLastSavedTm".
DEFINE_PROPERTYKEY(PKEY_Document_DateSaved, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 13);
#define INIT_PKEY_Document_DateSaved { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 13 }

//  Name:     System.Document.Division -- PKEY_Document_Division
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {1E005EE6-BF27-428B-B01C-79676ACD2870}, 100
DEFINE_PROPERTYKEY(PKEY_Document_Division, 0x1E005EE6, 0xBF27, 0x428B, 0xB0, 0x1C, 0x79, 0x67, 0x6A, 0xCD, 0x28, 0x70, 100);
#define INIT_PKEY_Document_Division { { 0x1E005EE6, 0xBF27, 0x428B, 0xB0, 0x1C, 0x79, 0x67, 0x6A, 0xCD, 0x28, 0x70 }, 100 }

//  Name:     System.Document.DocumentID -- PKEY_Document_DocumentID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E08805C8-E395-40DF-80D2-54F0D6C43154}, 100
DEFINE_PROPERTYKEY(PKEY_Document_DocumentID, 0xE08805C8, 0xE395, 0x40DF, 0x80, 0xD2, 0x54, 0xF0, 0xD6, 0xC4, 0x31, 0x54, 100);
#define INIT_PKEY_Document_DocumentID { { 0xE08805C8, 0xE395, 0x40DF, 0x80, 0xD2, 0x54, 0xF0, 0xD6, 0xC4, 0x31, 0x54 }, 100 }

//  Name:     System.Document.HiddenSlideCount -- PKEY_Document_HiddenSlideCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 9 (PIDDSI_HIDDENCOUNT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_HiddenSlideCount, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 9);
#define INIT_PKEY_Document_HiddenSlideCount { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 9 }

//  Name:     System.Document.LastAuthor -- PKEY_Document_LastAuthor
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 8 (PIDSI_LASTAUTHOR)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_LastAuthor, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 8);
#define INIT_PKEY_Document_LastAuthor { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 8 }

//  Name:     System.Document.LineCount -- PKEY_Document_LineCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 5 (PIDDSI_LINECOUNT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_LineCount, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 5);
#define INIT_PKEY_Document_LineCount { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 5 }

//  Name:     System.Document.Manager -- PKEY_Document_Manager
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 14 (PIDDSI_MANAGER)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_Manager, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 14);
#define INIT_PKEY_Document_Manager { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 14 }

//  Name:     System.Document.MultimediaClipCount -- PKEY_Document_MultimediaClipCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 10 (PIDDSI_MMCLIPCOUNT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_MultimediaClipCount, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 10);
#define INIT_PKEY_Document_MultimediaClipCount { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 10 }

//  Name:     System.Document.NoteCount -- PKEY_Document_NoteCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 8 (PIDDSI_NOTECOUNT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_NoteCount, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 8);
#define INIT_PKEY_Document_NoteCount { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 8 }

//  Name:     System.Document.PageCount -- PKEY_Document_PageCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 14 (PIDSI_PAGECOUNT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_PageCount, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 14);
#define INIT_PKEY_Document_PageCount { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 14 }

//  Name:     System.Document.ParagraphCount -- PKEY_Document_ParagraphCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 6 (PIDDSI_PARCOUNT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_ParagraphCount, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 6);
#define INIT_PKEY_Document_ParagraphCount { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 6 }

//  Name:     System.Document.PresentationFormat -- PKEY_Document_PresentationFormat
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 3 (PIDDSI_PRESFORMAT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_PresentationFormat, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 3);
#define INIT_PKEY_Document_PresentationFormat { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 3 }

//  Name:     System.Document.RevisionNumber -- PKEY_Document_RevisionNumber
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 9 (PIDSI_REVNUMBER)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_RevisionNumber, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 9);
#define INIT_PKEY_Document_RevisionNumber { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 9 }

//  Name:     System.Document.Security -- PKEY_Document_Security
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 19
//
//  Access control information, from SummaryInfo propset
DEFINE_PROPERTYKEY(PKEY_Document_Security, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 19);
#define INIT_PKEY_Document_Security { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 19 }

//  Name:     System.Document.SlideCount -- PKEY_Document_SlideCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 7 (PIDDSI_SLIDECOUNT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_SlideCount, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 7);
#define INIT_PKEY_Document_SlideCount { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 7 }

//  Name:     System.Document.Template -- PKEY_Document_Template
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 7 (PIDSI_TEMPLATE)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_Template, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 7);
#define INIT_PKEY_Document_Template { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 7 }

//  Name:     System.Document.TotalEditingTime -- PKEY_Document_TotalEditingTime
//  Type:     UInt64 -- VT_UI8
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 10 (PIDSI_EDITTIME)
//
//  100ns units, not milliseconds. VT_FILETIME for IPropertySetStorage handlers (legacy)
DEFINE_PROPERTYKEY(PKEY_Document_TotalEditingTime, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 10);
#define INIT_PKEY_Document_TotalEditingTime { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 10 }

//  Name:     System.Document.Version -- PKEY_Document_Version
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_DocumentSummaryInformation) {D5CDD502-2E9C-101B-9397-08002B2CF9AE}, 29
DEFINE_PROPERTYKEY(PKEY_Document_Version, 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE, 29);
#define INIT_PKEY_Document_Version { { 0xD5CDD502, 0x2E9C, 0x101B, 0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE }, 29 }

//  Name:     System.Document.WordCount -- PKEY_Document_WordCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_SummaryInformation) {F29F85E0-4FF9-1068-AB91-08002B27B3D9}, 15 (PIDSI_WORDCOUNT)
//
//  
DEFINE_PROPERTYKEY(PKEY_Document_WordCount, 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9, 15);
#define INIT_PKEY_Document_WordCount { { 0xF29F85E0, 0x4FF9, 0x1068, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9 }, 15 }

//-----------------------------------------------------------------------------
// DRM properties

//  Name:     System.DRM.DatePlayExpires -- PKEY_DRM_DatePlayExpires
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: (FMTID_DRM) {AEAC19E4-89AE-4508-B9B7-BB867ABEE2ED}, 6 (PIDDRSI_PLAYEXPIRES)
//
//  Indicates when play expires for digital rights management.
DEFINE_PROPERTYKEY(PKEY_DRM_DatePlayExpires, 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED, 6);
#define INIT_PKEY_DRM_DatePlayExpires { { 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED }, 6 }

//  Name:     System.DRM.DatePlayStarts -- PKEY_DRM_DatePlayStarts
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: (FMTID_DRM) {AEAC19E4-89AE-4508-B9B7-BB867ABEE2ED}, 5 (PIDDRSI_PLAYSTARTS)
//
//  Indicates when play starts for digital rights management.
DEFINE_PROPERTYKEY(PKEY_DRM_DatePlayStarts, 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED, 5);
#define INIT_PKEY_DRM_DatePlayStarts { { 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED }, 5 }

//  Name:     System.DRM.Description -- PKEY_DRM_Description
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_DRM) {AEAC19E4-89AE-4508-B9B7-BB867ABEE2ED}, 3 (PIDDRSI_DESCRIPTION)
//
//  Displays the description for digital rights management.
DEFINE_PROPERTYKEY(PKEY_DRM_Description, 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED, 3);
#define INIT_PKEY_DRM_Description { { 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED }, 3 }

//  Name:     System.DRM.IsDisabled -- PKEY_DRM_IsDisabled
//  Type:     Boolean -- VT_BOOL
//  FormatID: (FMTID_DRM) {AEAC19E4-89AE-4508-B9B7-BB867ABEE2ED}, 7
//
//  Indicates whether the media file has been disabled by DRM.
DEFINE_PROPERTYKEY(PKEY_DRM_IsDisabled, 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED, 7);
#define INIT_PKEY_DRM_IsDisabled { { 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED }, 7 }

//  Name:     System.DRM.IsProtected -- PKEY_DRM_IsProtected
//  Type:     Boolean -- VT_BOOL
//  FormatID: (FMTID_DRM) {AEAC19E4-89AE-4508-B9B7-BB867ABEE2ED}, 2 (PIDDRSI_PROTECTED)
//
//  
DEFINE_PROPERTYKEY(PKEY_DRM_IsProtected, 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED, 2);
#define INIT_PKEY_DRM_IsProtected { { 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED }, 2 }

//  Name:     System.DRM.PlayCount -- PKEY_DRM_PlayCount
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_DRM) {AEAC19E4-89AE-4508-B9B7-BB867ABEE2ED}, 4 (PIDDRSI_PLAYCOUNT)
//
//  Indicates the play count for digital rights management.
DEFINE_PROPERTYKEY(PKEY_DRM_PlayCount, 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED, 4);
#define INIT_PKEY_DRM_PlayCount { { 0xAEAC19E4, 0x89AE, 0x4508, 0xB9, 0xB7, 0xBB, 0x86, 0x7A, 0xBE, 0xE2, 0xED }, 4 }

//-----------------------------------------------------------------------------
// GPS properties

//  Name:     System.GPS.Altitude -- PKEY_GPS_Altitude
//  Type:     Double -- VT_R8
//  FormatID: {827EDB4F-5B73-44A7-891D-FDFFABEA35CA}, 100
//  
//  Indicates the altitude based on the reference in PKEY_GPS_AltitudeRef.  Calculated from PKEY_GPS_AltitudeNumerator and 
//  PKEY_GPS_AltitudeDenominator
DEFINE_PROPERTYKEY(PKEY_GPS_Altitude, 0x827EDB4F, 0x5B73, 0x44A7, 0x89, 0x1D, 0xFD, 0xFF, 0xAB, 0xEA, 0x35, 0xCA, 100);
#define INIT_PKEY_GPS_Altitude { { 0x827EDB4F, 0x5B73, 0x44A7, 0x89, 0x1D, 0xFD, 0xFF, 0xAB, 0xEA, 0x35, 0xCA }, 100 }

//  Name:     System.GPS.AltitudeDenominator -- PKEY_GPS_AltitudeDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {78342DCB-E358-4145-AE9A-6BFE4E0F9F51}, 100
//
//  Denominator of PKEY_GPS_Altitude
DEFINE_PROPERTYKEY(PKEY_GPS_AltitudeDenominator, 0x78342DCB, 0xE358, 0x4145, 0xAE, 0x9A, 0x6B, 0xFE, 0x4E, 0x0F, 0x9F, 0x51, 100);
#define INIT_PKEY_GPS_AltitudeDenominator { { 0x78342DCB, 0xE358, 0x4145, 0xAE, 0x9A, 0x6B, 0xFE, 0x4E, 0x0F, 0x9F, 0x51 }, 100 }

//  Name:     System.GPS.AltitudeNumerator -- PKEY_GPS_AltitudeNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {2DAD1EB7-816D-40D3-9EC3-C9773BE2AADE}, 100
//
//  Numerator of PKEY_GPS_Altitude
DEFINE_PROPERTYKEY(PKEY_GPS_AltitudeNumerator, 0x2DAD1EB7, 0x816D, 0x40D3, 0x9E, 0xC3, 0xC9, 0x77, 0x3B, 0xE2, 0xAA, 0xDE, 100);
#define INIT_PKEY_GPS_AltitudeNumerator { { 0x2DAD1EB7, 0x816D, 0x40D3, 0x9E, 0xC3, 0xC9, 0x77, 0x3B, 0xE2, 0xAA, 0xDE }, 100 }

//  Name:     System.GPS.AltitudeRef -- PKEY_GPS_AltitudeRef
//  Type:     Byte -- VT_UI1
//  FormatID: {46AC629D-75EA-4515-867F-6DC4321C5844}, 100
//
//  Indicates the reference for the altitude property. (eg: above sea level, below sea level, absolute value)
DEFINE_PROPERTYKEY(PKEY_GPS_AltitudeRef, 0x46AC629D, 0x75EA, 0x4515, 0x86, 0x7F, 0x6D, 0xC4, 0x32, 0x1C, 0x58, 0x44, 100);
#define INIT_PKEY_GPS_AltitudeRef { { 0x46AC629D, 0x75EA, 0x4515, 0x86, 0x7F, 0x6D, 0xC4, 0x32, 0x1C, 0x58, 0x44 }, 100 }

//  Name:     System.GPS.AreaInformation -- PKEY_GPS_AreaInformation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {972E333E-AC7E-49F1-8ADF-A70D07A9BCAB}, 100
//
//  Represents the name of the GPS area
DEFINE_PROPERTYKEY(PKEY_GPS_AreaInformation, 0x972E333E, 0xAC7E, 0x49F1, 0x8A, 0xDF, 0xA7, 0x0D, 0x07, 0xA9, 0xBC, 0xAB, 100);
#define INIT_PKEY_GPS_AreaInformation { { 0x972E333E, 0xAC7E, 0x49F1, 0x8A, 0xDF, 0xA7, 0x0D, 0x07, 0xA9, 0xBC, 0xAB }, 100 }

//  Name:     System.GPS.Date -- PKEY_GPS_Date
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {3602C812-0F3B-45F0-85AD-603468D69423}, 100
//
//  Date and time of the GPS record
DEFINE_PROPERTYKEY(PKEY_GPS_Date, 0x3602C812, 0x0F3B, 0x45F0, 0x85, 0xAD, 0x60, 0x34, 0x68, 0xD6, 0x94, 0x23, 100);
#define INIT_PKEY_GPS_Date { { 0x3602C812, 0x0F3B, 0x45F0, 0x85, 0xAD, 0x60, 0x34, 0x68, 0xD6, 0x94, 0x23 }, 100 }

//  Name:     System.GPS.DestBearing -- PKEY_GPS_DestBearing
//  Type:     Double -- VT_R8
//  FormatID: {C66D4B3C-E888-47CC-B99F-9DCA3EE34DEA}, 100
//  
//  Indicates the bearing to the destination point.  Calculated from PKEY_GPS_DestBearingNumerator and 
//  PKEY_GPS_DestBearingDenominator.
DEFINE_PROPERTYKEY(PKEY_GPS_DestBearing, 0xC66D4B3C, 0xE888, 0x47CC, 0xB9, 0x9F, 0x9D, 0xCA, 0x3E, 0xE3, 0x4D, 0xEA, 100);
#define INIT_PKEY_GPS_DestBearing { { 0xC66D4B3C, 0xE888, 0x47CC, 0xB9, 0x9F, 0x9D, 0xCA, 0x3E, 0xE3, 0x4D, 0xEA }, 100 }

//  Name:     System.GPS.DestBearingDenominator -- PKEY_GPS_DestBearingDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {7ABCF4F8-7C3F-4988-AC91-8D2C2E97ECA5}, 100
//
//  Denominator of PKEY_GPS_DestBearing
DEFINE_PROPERTYKEY(PKEY_GPS_DestBearingDenominator, 0x7ABCF4F8, 0x7C3F, 0x4988, 0xAC, 0x91, 0x8D, 0x2C, 0x2E, 0x97, 0xEC, 0xA5, 100);
#define INIT_PKEY_GPS_DestBearingDenominator { { 0x7ABCF4F8, 0x7C3F, 0x4988, 0xAC, 0x91, 0x8D, 0x2C, 0x2E, 0x97, 0xEC, 0xA5 }, 100 }

//  Name:     System.GPS.DestBearingNumerator -- PKEY_GPS_DestBearingNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {BA3B1DA9-86EE-4B5D-A2A4-A271A429F0CF}, 100
//
//  Numerator of PKEY_GPS_DestBearing
DEFINE_PROPERTYKEY(PKEY_GPS_DestBearingNumerator, 0xBA3B1DA9, 0x86EE, 0x4B5D, 0xA2, 0xA4, 0xA2, 0x71, 0xA4, 0x29, 0xF0, 0xCF, 100);
#define INIT_PKEY_GPS_DestBearingNumerator { { 0xBA3B1DA9, 0x86EE, 0x4B5D, 0xA2, 0xA4, 0xA2, 0x71, 0xA4, 0x29, 0xF0, 0xCF }, 100 }

//  Name:     System.GPS.DestBearingRef -- PKEY_GPS_DestBearingRef
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9AB84393-2A0F-4B75-BB22-7279786977CB}, 100
//
//  Indicates the reference used for the giving the bearing to the destination point.  (eg: true direction, magnetic direction)
DEFINE_PROPERTYKEY(PKEY_GPS_DestBearingRef, 0x9AB84393, 0x2A0F, 0x4B75, 0xBB, 0x22, 0x72, 0x79, 0x78, 0x69, 0x77, 0xCB, 100);
#define INIT_PKEY_GPS_DestBearingRef { { 0x9AB84393, 0x2A0F, 0x4B75, 0xBB, 0x22, 0x72, 0x79, 0x78, 0x69, 0x77, 0xCB }, 100 }

//  Name:     System.GPS.DestDistance -- PKEY_GPS_DestDistance
//  Type:     Double -- VT_R8
//  FormatID: {A93EAE04-6804-4F24-AC81-09B266452118}, 100
//  
//  Indicates the distance to the destination point.  Calculated from PKEY_GPS_DestDistanceNumerator and 
//  PKEY_GPS_DestDistanceDenominator.
DEFINE_PROPERTYKEY(PKEY_GPS_DestDistance, 0xA93EAE04, 0x6804, 0x4F24, 0xAC, 0x81, 0x09, 0xB2, 0x66, 0x45, 0x21, 0x18, 100);
#define INIT_PKEY_GPS_DestDistance { { 0xA93EAE04, 0x6804, 0x4F24, 0xAC, 0x81, 0x09, 0xB2, 0x66, 0x45, 0x21, 0x18 }, 100 }

//  Name:     System.GPS.DestDistanceDenominator -- PKEY_GPS_DestDistanceDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {9BC2C99B-AC71-4127-9D1C-2596D0D7DCB7}, 100
//
//  Denominator of PKEY_GPS_DestDistance
DEFINE_PROPERTYKEY(PKEY_GPS_DestDistanceDenominator, 0x9BC2C99B, 0xAC71, 0x4127, 0x9D, 0x1C, 0x25, 0x96, 0xD0, 0xD7, 0xDC, 0xB7, 100);
#define INIT_PKEY_GPS_DestDistanceDenominator { { 0x9BC2C99B, 0xAC71, 0x4127, 0x9D, 0x1C, 0x25, 0x96, 0xD0, 0xD7, 0xDC, 0xB7 }, 100 }

//  Name:     System.GPS.DestDistanceNumerator -- PKEY_GPS_DestDistanceNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {2BDA47DA-08C6-4FE1-80BC-A72FC517C5D0}, 100
//
//  Numerator of PKEY_GPS_DestDistance
DEFINE_PROPERTYKEY(PKEY_GPS_DestDistanceNumerator, 0x2BDA47DA, 0x08C6, 0x4FE1, 0x80, 0xBC, 0xA7, 0x2F, 0xC5, 0x17, 0xC5, 0xD0, 100);
#define INIT_PKEY_GPS_DestDistanceNumerator { { 0x2BDA47DA, 0x08C6, 0x4FE1, 0x80, 0xBC, 0xA7, 0x2F, 0xC5, 0x17, 0xC5, 0xD0 }, 100 }

//  Name:     System.GPS.DestDistanceRef -- PKEY_GPS_DestDistanceRef
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {ED4DF2D3-8695-450B-856F-F5C1C53ACB66}, 100
//
//  Indicates the unit used to express the distance to the destination.  (eg: kilometers, miles, knots)
DEFINE_PROPERTYKEY(PKEY_GPS_DestDistanceRef, 0xED4DF2D3, 0x8695, 0x450B, 0x85, 0x6F, 0xF5, 0xC1, 0xC5, 0x3A, 0xCB, 0x66, 100);
#define INIT_PKEY_GPS_DestDistanceRef { { 0xED4DF2D3, 0x8695, 0x450B, 0x85, 0x6F, 0xF5, 0xC1, 0xC5, 0x3A, 0xCB, 0x66 }, 100 }

//  Name:     System.GPS.DestLatitude -- PKEY_GPS_DestLatitude
//  Type:     Multivalue Double -- VT_VECTOR | VT_R8  (For variants: VT_ARRAY | VT_R8)
//  FormatID: {9D1D7CC5-5C39-451C-86B3-928E2D18CC47}, 100
//  
//  Indicates the latitude of the destination point.  This is an array of three values.  Index 0 is the degrees, index 1 
//  is the minutes, index 2 is the seconds.  Each is calculated from the values in PKEY_GPS_DestLatitudeNumerator and 
//  PKEY_GPS_DestLatitudeDenominator.
DEFINE_PROPERTYKEY(PKEY_GPS_DestLatitude, 0x9D1D7CC5, 0x5C39, 0x451C, 0x86, 0xB3, 0x92, 0x8E, 0x2D, 0x18, 0xCC, 0x47, 100);
#define INIT_PKEY_GPS_DestLatitude { { 0x9D1D7CC5, 0x5C39, 0x451C, 0x86, 0xB3, 0x92, 0x8E, 0x2D, 0x18, 0xCC, 0x47 }, 100 }

//  Name:     System.GPS.DestLatitudeDenominator -- PKEY_GPS_DestLatitudeDenominator
//  Type:     Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
//  FormatID: {3A372292-7FCA-49A7-99D5-E47BB2D4E7AB}, 100
//
//  Denominator of PKEY_GPS_DestLatitude
DEFINE_PROPERTYKEY(PKEY_GPS_DestLatitudeDenominator, 0x3A372292, 0x7FCA, 0x49A7, 0x99, 0xD5, 0xE4, 0x7B, 0xB2, 0xD4, 0xE7, 0xAB, 100);
#define INIT_PKEY_GPS_DestLatitudeDenominator { { 0x3A372292, 0x7FCA, 0x49A7, 0x99, 0xD5, 0xE4, 0x7B, 0xB2, 0xD4, 0xE7, 0xAB }, 100 }

//  Name:     System.GPS.DestLatitudeNumerator -- PKEY_GPS_DestLatitudeNumerator
//  Type:     Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
//  FormatID: {ECF4B6F6-D5A6-433C-BB92-4076650FC890}, 100
//
//  Numerator of PKEY_GPS_DestLatitude
DEFINE_PROPERTYKEY(PKEY_GPS_DestLatitudeNumerator, 0xECF4B6F6, 0xD5A6, 0x433C, 0xBB, 0x92, 0x40, 0x76, 0x65, 0x0F, 0xC8, 0x90, 100);
#define INIT_PKEY_GPS_DestLatitudeNumerator { { 0xECF4B6F6, 0xD5A6, 0x433C, 0xBB, 0x92, 0x40, 0x76, 0x65, 0x0F, 0xC8, 0x90 }, 100 }

//  Name:     System.GPS.DestLatitudeRef -- PKEY_GPS_DestLatitudeRef
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {CEA820B9-CE61-4885-A128-005D9087C192}, 100
//
//  Indicates whether the latitude destination point is north or south latitude
DEFINE_PROPERTYKEY(PKEY_GPS_DestLatitudeRef, 0xCEA820B9, 0xCE61, 0x4885, 0xA1, 0x28, 0x00, 0x5D, 0x90, 0x87, 0xC1, 0x92, 100);
#define INIT_PKEY_GPS_DestLatitudeRef { { 0xCEA820B9, 0xCE61, 0x4885, 0xA1, 0x28, 0x00, 0x5D, 0x90, 0x87, 0xC1, 0x92 }, 100 }

//  Name:     System.GPS.DestLongitude -- PKEY_GPS_DestLongitude
//  Type:     Multivalue Double -- VT_VECTOR | VT_R8  (For variants: VT_ARRAY | VT_R8)
//  FormatID: {47A96261-CB4C-4807-8AD3-40B9D9DBC6BC}, 100
//  
//  Indicates the latitude of the destination point.  This is an array of three values.  Index 0 is the degrees, index 1 
//  is the minutes, index 2 is the seconds.  Each is calculated from the values in PKEY_GPS_DestLongitudeNumerator and 
//  PKEY_GPS_DestLongitudeDenominator.
DEFINE_PROPERTYKEY(PKEY_GPS_DestLongitude, 0x47A96261, 0xCB4C, 0x4807, 0x8A, 0xD3, 0x40, 0xB9, 0xD9, 0xDB, 0xC6, 0xBC, 100);
#define INIT_PKEY_GPS_DestLongitude { { 0x47A96261, 0xCB4C, 0x4807, 0x8A, 0xD3, 0x40, 0xB9, 0xD9, 0xDB, 0xC6, 0xBC }, 100 }

//  Name:     System.GPS.DestLongitudeDenominator -- PKEY_GPS_DestLongitudeDenominator
//  Type:     Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
//  FormatID: {425D69E5-48AD-4900-8D80-6EB6B8D0AC86}, 100
//
//  Denominator of PKEY_GPS_DestLongitude
DEFINE_PROPERTYKEY(PKEY_GPS_DestLongitudeDenominator, 0x425D69E5, 0x48AD, 0x4900, 0x8D, 0x80, 0x6E, 0xB6, 0xB8, 0xD0, 0xAC, 0x86, 100);
#define INIT_PKEY_GPS_DestLongitudeDenominator { { 0x425D69E5, 0x48AD, 0x4900, 0x8D, 0x80, 0x6E, 0xB6, 0xB8, 0xD0, 0xAC, 0x86 }, 100 }

//  Name:     System.GPS.DestLongitudeNumerator -- PKEY_GPS_DestLongitudeNumerator
//  Type:     Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
//  FormatID: {A3250282-FB6D-48D5-9A89-DBCACE75CCCF}, 100
//
//  Numerator of PKEY_GPS_DestLongitude
DEFINE_PROPERTYKEY(PKEY_GPS_DestLongitudeNumerator, 0xA3250282, 0xFB6D, 0x48D5, 0x9A, 0x89, 0xDB, 0xCA, 0xCE, 0x75, 0xCC, 0xCF, 100);
#define INIT_PKEY_GPS_DestLongitudeNumerator { { 0xA3250282, 0xFB6D, 0x48D5, 0x9A, 0x89, 0xDB, 0xCA, 0xCE, 0x75, 0xCC, 0xCF }, 100 }

//  Name:     System.GPS.DestLongitudeRef -- PKEY_GPS_DestLongitudeRef
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {182C1EA6-7C1C-4083-AB4B-AC6C9F4ED128}, 100
//
//  Indicates whether the longitude destination point is east or west longitude
DEFINE_PROPERTYKEY(PKEY_GPS_DestLongitudeRef, 0x182C1EA6, 0x7C1C, 0x4083, 0xAB, 0x4B, 0xAC, 0x6C, 0x9F, 0x4E, 0xD1, 0x28, 100);
#define INIT_PKEY_GPS_DestLongitudeRef { { 0x182C1EA6, 0x7C1C, 0x4083, 0xAB, 0x4B, 0xAC, 0x6C, 0x9F, 0x4E, 0xD1, 0x28 }, 100 }

//  Name:     System.GPS.Differential -- PKEY_GPS_Differential
//  Type:     UInt16 -- VT_UI2
//  FormatID: {AAF4EE25-BD3B-4DD7-BFC4-47F77BB00F6D}, 100
//
//  Indicates whether differential correction was applied to the GPS receiver
DEFINE_PROPERTYKEY(PKEY_GPS_Differential, 0xAAF4EE25, 0xBD3B, 0x4DD7, 0xBF, 0xC4, 0x47, 0xF7, 0x7B, 0xB0, 0x0F, 0x6D, 100);
#define INIT_PKEY_GPS_Differential { { 0xAAF4EE25, 0xBD3B, 0x4DD7, 0xBF, 0xC4, 0x47, 0xF7, 0x7B, 0xB0, 0x0F, 0x6D }, 100 }

//  Name:     System.GPS.DOP -- PKEY_GPS_DOP
//  Type:     Double -- VT_R8
//  FormatID: {0CF8FB02-1837-42F1-A697-A7017AA289B9}, 100
//
//  Indicates the GPS DOP (data degree of precision).  Calculated from PKEY_GPS_DOPNumerator and PKEY_GPS_DOPDenominator
DEFINE_PROPERTYKEY(PKEY_GPS_DOP, 0x0CF8FB02, 0x1837, 0x42F1, 0xA6, 0x97, 0xA7, 0x01, 0x7A, 0xA2, 0x89, 0xB9, 100);
#define INIT_PKEY_GPS_DOP { { 0x0CF8FB02, 0x1837, 0x42F1, 0xA6, 0x97, 0xA7, 0x01, 0x7A, 0xA2, 0x89, 0xB9 }, 100 }

//  Name:     System.GPS.DOPDenominator -- PKEY_GPS_DOPDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {A0BE94C5-50BA-487B-BD35-0654BE8881ED}, 100
//
//  Denominator of PKEY_GPS_DOP
DEFINE_PROPERTYKEY(PKEY_GPS_DOPDenominator, 0xA0BE94C5, 0x50BA, 0x487B, 0xBD, 0x35, 0x06, 0x54, 0xBE, 0x88, 0x81, 0xED, 100);
#define INIT_PKEY_GPS_DOPDenominator { { 0xA0BE94C5, 0x50BA, 0x487B, 0xBD, 0x35, 0x06, 0x54, 0xBE, 0x88, 0x81, 0xED }, 100 }

//  Name:     System.GPS.DOPNumerator -- PKEY_GPS_DOPNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {47166B16-364F-4AA0-9F31-E2AB3DF449C3}, 100
//
//  Numerator of PKEY_GPS_DOP
DEFINE_PROPERTYKEY(PKEY_GPS_DOPNumerator, 0x47166B16, 0x364F, 0x4AA0, 0x9F, 0x31, 0xE2, 0xAB, 0x3D, 0xF4, 0x49, 0xC3, 100);
#define INIT_PKEY_GPS_DOPNumerator { { 0x47166B16, 0x364F, 0x4AA0, 0x9F, 0x31, 0xE2, 0xAB, 0x3D, 0xF4, 0x49, 0xC3 }, 100 }

//  Name:     System.GPS.ImgDirection -- PKEY_GPS_ImgDirection
//  Type:     Double -- VT_R8
//  FormatID: {16473C91-D017-4ED9-BA4D-B6BAA55DBCF8}, 100
//  
//  Indicates direction of the image when it was captured.  Calculated from PKEY_GPS_ImgDirectionNumerator and 
//  PKEY_GPS_ImgDirectionDenominator.
DEFINE_PROPERTYKEY(PKEY_GPS_ImgDirection, 0x16473C91, 0xD017, 0x4ED9, 0xBA, 0x4D, 0xB6, 0xBA, 0xA5, 0x5D, 0xBC, 0xF8, 100);
#define INIT_PKEY_GPS_ImgDirection { { 0x16473C91, 0xD017, 0x4ED9, 0xBA, 0x4D, 0xB6, 0xBA, 0xA5, 0x5D, 0xBC, 0xF8 }, 100 }

//  Name:     System.GPS.ImgDirectionDenominator -- PKEY_GPS_ImgDirectionDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {10B24595-41A2-4E20-93C2-5761C1395F32}, 100
//
//  Denominator of PKEY_GPS_ImgDirection
DEFINE_PROPERTYKEY(PKEY_GPS_ImgDirectionDenominator, 0x10B24595, 0x41A2, 0x4E20, 0x93, 0xC2, 0x57, 0x61, 0xC1, 0x39, 0x5F, 0x32, 100);
#define INIT_PKEY_GPS_ImgDirectionDenominator { { 0x10B24595, 0x41A2, 0x4E20, 0x93, 0xC2, 0x57, 0x61, 0xC1, 0x39, 0x5F, 0x32 }, 100 }

//  Name:     System.GPS.ImgDirectionNumerator -- PKEY_GPS_ImgDirectionNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {DC5877C7-225F-45F7-BAC7-E81334B6130A}, 100
//
//  Numerator of PKEY_GPS_ImgDirection
DEFINE_PROPERTYKEY(PKEY_GPS_ImgDirectionNumerator, 0xDC5877C7, 0x225F, 0x45F7, 0xBA, 0xC7, 0xE8, 0x13, 0x34, 0xB6, 0x13, 0x0A, 100);
#define INIT_PKEY_GPS_ImgDirectionNumerator { { 0xDC5877C7, 0x225F, 0x45F7, 0xBA, 0xC7, 0xE8, 0x13, 0x34, 0xB6, 0x13, 0x0A }, 100 }

//  Name:     System.GPS.ImgDirectionRef -- PKEY_GPS_ImgDirectionRef
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A4AAA5B7-1AD0-445F-811A-0F8F6E67F6B5}, 100
//
//  Indicates reference for giving the direction of the image when it was captured.  (eg: true direction, magnetic direction)
DEFINE_PROPERTYKEY(PKEY_GPS_ImgDirectionRef, 0xA4AAA5B7, 0x1AD0, 0x445F, 0x81, 0x1A, 0x0F, 0x8F, 0x6E, 0x67, 0xF6, 0xB5, 100);
#define INIT_PKEY_GPS_ImgDirectionRef { { 0xA4AAA5B7, 0x1AD0, 0x445F, 0x81, 0x1A, 0x0F, 0x8F, 0x6E, 0x67, 0xF6, 0xB5 }, 100 }

//  Name:     System.GPS.Latitude -- PKEY_GPS_Latitude
//  Type:     Multivalue Double -- VT_VECTOR | VT_R8  (For variants: VT_ARRAY | VT_R8)
//  FormatID: {8727CFFF-4868-4EC6-AD5B-81B98521D1AB}, 100
//  
//  Indicates the latitude.  This is an array of three values.  Index 0 is the degrees, index 1 is the minutes, index 2 
//  is the seconds.  Each is calculated from the values in PKEY_GPS_LatitudeNumerator and PKEY_GPS_LatitudeDenominator.
DEFINE_PROPERTYKEY(PKEY_GPS_Latitude, 0x8727CFFF, 0x4868, 0x4EC6, 0xAD, 0x5B, 0x81, 0xB9, 0x85, 0x21, 0xD1, 0xAB, 100);
#define INIT_PKEY_GPS_Latitude { { 0x8727CFFF, 0x4868, 0x4EC6, 0xAD, 0x5B, 0x81, 0xB9, 0x85, 0x21, 0xD1, 0xAB }, 100 }

//  Name:     System.GPS.LatitudeDecimal -- PKEY_GPS_LatitudeDecimal
//  Type:     Double -- VT_R8
//  FormatID: {0F55CDE2-4F49-450D-92C1-DCD16301B1B7}, 100
//  
//  Indicates the latitude based on the reference in PKEY_GPS_LatitudeRef.  Calculated from PKEY_GPS_LatitudeNumerator and 
//  PKEY_GPS_LatitudeDenominator
DEFINE_PROPERTYKEY(PKEY_GPS_LatitudeDecimal, 0x0F55CDE2, 0x4F49, 0x450D, 0x92, 0xC1, 0xDC, 0xD1, 0x63, 0x01, 0xB1, 0xB7, 100);
#define INIT_PKEY_GPS_LatitudeDecimal { { 0x0F55CDE2, 0x4F49, 0x450D, 0x92, 0xC1, 0xDC, 0xD1, 0x63, 0x01, 0xB1, 0xB7 }, 100 }

//  Name:     System.GPS.LatitudeDenominator -- PKEY_GPS_LatitudeDenominator
//  Type:     Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
//  FormatID: {16E634EE-2BFF-497B-BD8A-4341AD39EEB9}, 100
//
//  Denominator of PKEY_GPS_Latitude
DEFINE_PROPERTYKEY(PKEY_GPS_LatitudeDenominator, 0x16E634EE, 0x2BFF, 0x497B, 0xBD, 0x8A, 0x43, 0x41, 0xAD, 0x39, 0xEE, 0xB9, 100);
#define INIT_PKEY_GPS_LatitudeDenominator { { 0x16E634EE, 0x2BFF, 0x497B, 0xBD, 0x8A, 0x43, 0x41, 0xAD, 0x39, 0xEE, 0xB9 }, 100 }

//  Name:     System.GPS.LatitudeNumerator -- PKEY_GPS_LatitudeNumerator
//  Type:     Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
//  FormatID: {7DDAAAD1-CCC8-41AE-B750-B2CB8031AEA2}, 100
//
//  Numerator of PKEY_GPS_Latitude
DEFINE_PROPERTYKEY(PKEY_GPS_LatitudeNumerator, 0x7DDAAAD1, 0xCCC8, 0x41AE, 0xB7, 0x50, 0xB2, 0xCB, 0x80, 0x31, 0xAE, 0xA2, 100);
#define INIT_PKEY_GPS_LatitudeNumerator { { 0x7DDAAAD1, 0xCCC8, 0x41AE, 0xB7, 0x50, 0xB2, 0xCB, 0x80, 0x31, 0xAE, 0xA2 }, 100 }

//  Name:     System.GPS.LatitudeRef -- PKEY_GPS_LatitudeRef
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {029C0252-5B86-46C7-ACA0-2769FFC8E3D4}, 100
//
//  Indicates whether latitude is north or south latitude 
DEFINE_PROPERTYKEY(PKEY_GPS_LatitudeRef, 0x029C0252, 0x5B86, 0x46C7, 0xAC, 0xA0, 0x27, 0x69, 0xFF, 0xC8, 0xE3, 0xD4, 100);
#define INIT_PKEY_GPS_LatitudeRef { { 0x029C0252, 0x5B86, 0x46C7, 0xAC, 0xA0, 0x27, 0x69, 0xFF, 0xC8, 0xE3, 0xD4 }, 100 }

//  Name:     System.GPS.Longitude -- PKEY_GPS_Longitude
//  Type:     Multivalue Double -- VT_VECTOR | VT_R8  (For variants: VT_ARRAY | VT_R8)
//  FormatID: {C4C4DBB2-B593-466B-BBDA-D03D27D5E43A}, 100
//  
//  Indicates the longitude.  This is an array of three values.  Index 0 is the degrees, index 1 is the minutes, index 2 
//  is the seconds.  Each is calculated from the values in PKEY_GPS_LongitudeNumerator and PKEY_GPS_LongitudeDenominator.
DEFINE_PROPERTYKEY(PKEY_GPS_Longitude, 0xC4C4DBB2, 0xB593, 0x466B, 0xBB, 0xDA, 0xD0, 0x3D, 0x27, 0xD5, 0xE4, 0x3A, 100);
#define INIT_PKEY_GPS_Longitude { { 0xC4C4DBB2, 0xB593, 0x466B, 0xBB, 0xDA, 0xD0, 0x3D, 0x27, 0xD5, 0xE4, 0x3A }, 100 }

//  Name:     System.GPS.LongitudeDecimal -- PKEY_GPS_LongitudeDecimal
//  Type:     Double -- VT_R8
//  FormatID: {4679C1B5-844D-4590-BAF5-F322231F1B81}, 100
//  
//  Indicates the longitude based on the reference in PKEY_GPS_LongitudeRef.  Calculated from PKEY_GPS_LongitudeNumerator and 
//  PKEY_GPS_LongitudeDenominator
DEFINE_PROPERTYKEY(PKEY_GPS_LongitudeDecimal, 0x4679C1B5, 0x844D, 0x4590, 0xBA, 0xF5, 0xF3, 0x22, 0x23, 0x1F, 0x1B, 0x81, 100);
#define INIT_PKEY_GPS_LongitudeDecimal { { 0x4679C1B5, 0x844D, 0x4590, 0xBA, 0xF5, 0xF3, 0x22, 0x23, 0x1F, 0x1B, 0x81 }, 100 }

//  Name:     System.GPS.LongitudeDenominator -- PKEY_GPS_LongitudeDenominator
//  Type:     Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
//  FormatID: {BE6E176C-4534-4D2C-ACE5-31DEDAC1606B}, 100
//
//  Denominator of PKEY_GPS_Longitude
DEFINE_PROPERTYKEY(PKEY_GPS_LongitudeDenominator, 0xBE6E176C, 0x4534, 0x4D2C, 0xAC, 0xE5, 0x31, 0xDE, 0xDA, 0xC1, 0x60, 0x6B, 100);
#define INIT_PKEY_GPS_LongitudeDenominator { { 0xBE6E176C, 0x4534, 0x4D2C, 0xAC, 0xE5, 0x31, 0xDE, 0xDA, 0xC1, 0x60, 0x6B }, 100 }

//  Name:     System.GPS.LongitudeNumerator -- PKEY_GPS_LongitudeNumerator
//  Type:     Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
//  FormatID: {02B0F689-A914-4E45-821D-1DDA452ED2C4}, 100
//
//  Numerator of PKEY_GPS_Longitude
DEFINE_PROPERTYKEY(PKEY_GPS_LongitudeNumerator, 0x02B0F689, 0xA914, 0x4E45, 0x82, 0x1D, 0x1D, 0xDA, 0x45, 0x2E, 0xD2, 0xC4, 100);
#define INIT_PKEY_GPS_LongitudeNumerator { { 0x02B0F689, 0xA914, 0x4E45, 0x82, 0x1D, 0x1D, 0xDA, 0x45, 0x2E, 0xD2, 0xC4 }, 100 }

//  Name:     System.GPS.LongitudeRef -- PKEY_GPS_LongitudeRef
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {33DCF22B-28D5-464C-8035-1EE9EFD25278}, 100
//
//  Indicates whether longitude is east or west longitude
DEFINE_PROPERTYKEY(PKEY_GPS_LongitudeRef, 0x33DCF22B, 0x28D5, 0x464C, 0x80, 0x35, 0x1E, 0xE9, 0xEF, 0xD2, 0x52, 0x78, 100);
#define INIT_PKEY_GPS_LongitudeRef { { 0x33DCF22B, 0x28D5, 0x464C, 0x80, 0x35, 0x1E, 0xE9, 0xEF, 0xD2, 0x52, 0x78 }, 100 }

//  Name:     System.GPS.MapDatum -- PKEY_GPS_MapDatum
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {2CA2DAE6-EDDC-407D-BEF1-773942ABFA95}, 100
//
//  Indicates the geodetic survey data used by the GPS receiver
DEFINE_PROPERTYKEY(PKEY_GPS_MapDatum, 0x2CA2DAE6, 0xEDDC, 0x407D, 0xBE, 0xF1, 0x77, 0x39, 0x42, 0xAB, 0xFA, 0x95, 100);
#define INIT_PKEY_GPS_MapDatum { { 0x2CA2DAE6, 0xEDDC, 0x407D, 0xBE, 0xF1, 0x77, 0x39, 0x42, 0xAB, 0xFA, 0x95 }, 100 }

//  Name:     System.GPS.MeasureMode -- PKEY_GPS_MeasureMode
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A015ED5D-AAEA-4D58-8A86-3C586920EA0B}, 100
//
//  Indicates the GPS measurement mode.  (eg: 2-dimensional, 3-dimensional)
DEFINE_PROPERTYKEY(PKEY_GPS_MeasureMode, 0xA015ED5D, 0xAAEA, 0x4D58, 0x8A, 0x86, 0x3C, 0x58, 0x69, 0x20, 0xEA, 0x0B, 100);
#define INIT_PKEY_GPS_MeasureMode { { 0xA015ED5D, 0xAAEA, 0x4D58, 0x8A, 0x86, 0x3C, 0x58, 0x69, 0x20, 0xEA, 0x0B }, 100 }

//  Name:     System.GPS.ProcessingMethod -- PKEY_GPS_ProcessingMethod
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {59D49E61-840F-4AA9-A939-E2099B7F6399}, 100
//
//  Indicates the name of the method used for location finding
DEFINE_PROPERTYKEY(PKEY_GPS_ProcessingMethod, 0x59D49E61, 0x840F, 0x4AA9, 0xA9, 0x39, 0xE2, 0x09, 0x9B, 0x7F, 0x63, 0x99, 100);
#define INIT_PKEY_GPS_ProcessingMethod { { 0x59D49E61, 0x840F, 0x4AA9, 0xA9, 0x39, 0xE2, 0x09, 0x9B, 0x7F, 0x63, 0x99 }, 100 }

//  Name:     System.GPS.Satellites -- PKEY_GPS_Satellites
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {467EE575-1F25-4557-AD4E-B8B58B0D9C15}, 100
//
//  Indicates the GPS satellites used for measurements
DEFINE_PROPERTYKEY(PKEY_GPS_Satellites, 0x467EE575, 0x1F25, 0x4557, 0xAD, 0x4E, 0xB8, 0xB5, 0x8B, 0x0D, 0x9C, 0x15, 100);
#define INIT_PKEY_GPS_Satellites { { 0x467EE575, 0x1F25, 0x4557, 0xAD, 0x4E, 0xB8, 0xB5, 0x8B, 0x0D, 0x9C, 0x15 }, 100 }

//  Name:     System.GPS.Speed -- PKEY_GPS_Speed
//  Type:     Double -- VT_R8
//  FormatID: {DA5D0862-6E76-4E1B-BABD-70021BD25494}, 100
//  
//  Indicates the speed of the GPS receiver movement.  Calculated from PKEY_GPS_SpeedNumerator and 
//  PKEY_GPS_SpeedDenominator.
DEFINE_PROPERTYKEY(PKEY_GPS_Speed, 0xDA5D0862, 0x6E76, 0x4E1B, 0xBA, 0xBD, 0x70, 0x02, 0x1B, 0xD2, 0x54, 0x94, 100);
#define INIT_PKEY_GPS_Speed { { 0xDA5D0862, 0x6E76, 0x4E1B, 0xBA, 0xBD, 0x70, 0x02, 0x1B, 0xD2, 0x54, 0x94 }, 100 }

//  Name:     System.GPS.SpeedDenominator -- PKEY_GPS_SpeedDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {7D122D5A-AE5E-4335-8841-D71E7CE72F53}, 100
//
//  Denominator of PKEY_GPS_Speed
DEFINE_PROPERTYKEY(PKEY_GPS_SpeedDenominator, 0x7D122D5A, 0xAE5E, 0x4335, 0x88, 0x41, 0xD7, 0x1E, 0x7C, 0xE7, 0x2F, 0x53, 100);
#define INIT_PKEY_GPS_SpeedDenominator { { 0x7D122D5A, 0xAE5E, 0x4335, 0x88, 0x41, 0xD7, 0x1E, 0x7C, 0xE7, 0x2F, 0x53 }, 100 }

//  Name:     System.GPS.SpeedNumerator -- PKEY_GPS_SpeedNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {ACC9CE3D-C213-4942-8B48-6D0820F21C6D}, 100
//
//  Numerator of PKEY_GPS_Speed
DEFINE_PROPERTYKEY(PKEY_GPS_SpeedNumerator, 0xACC9CE3D, 0xC213, 0x4942, 0x8B, 0x48, 0x6D, 0x08, 0x20, 0xF2, 0x1C, 0x6D, 100);
#define INIT_PKEY_GPS_SpeedNumerator { { 0xACC9CE3D, 0xC213, 0x4942, 0x8B, 0x48, 0x6D, 0x08, 0x20, 0xF2, 0x1C, 0x6D }, 100 }

//  Name:     System.GPS.SpeedRef -- PKEY_GPS_SpeedRef
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {ECF7F4C9-544F-4D6D-9D98-8AD79ADAF453}, 100
//  
//  Indicates the unit used to express the speed of the GPS receiver movement.  (eg: kilometers per hour, 
//  miles per hour, knots).
DEFINE_PROPERTYKEY(PKEY_GPS_SpeedRef, 0xECF7F4C9, 0x544F, 0x4D6D, 0x9D, 0x98, 0x8A, 0xD7, 0x9A, 0xDA, 0xF4, 0x53, 100);
#define INIT_PKEY_GPS_SpeedRef { { 0xECF7F4C9, 0x544F, 0x4D6D, 0x9D, 0x98, 0x8A, 0xD7, 0x9A, 0xDA, 0xF4, 0x53 }, 100 }

//  Name:     System.GPS.Status -- PKEY_GPS_Status
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {125491F4-818F-46B2-91B5-D537753617B2}, 100
//  
//  Indicates the status of the GPS receiver when the image was recorded.  (eg: measurement in progress, 
//  measurement interoperability).
DEFINE_PROPERTYKEY(PKEY_GPS_Status, 0x125491F4, 0x818F, 0x46B2, 0x91, 0xB5, 0xD5, 0x37, 0x75, 0x36, 0x17, 0xB2, 100);
#define INIT_PKEY_GPS_Status { { 0x125491F4, 0x818F, 0x46B2, 0x91, 0xB5, 0xD5, 0x37, 0x75, 0x36, 0x17, 0xB2 }, 100 }

//  Name:     System.GPS.Track -- PKEY_GPS_Track
//  Type:     Double -- VT_R8
//  FormatID: {76C09943-7C33-49E3-9E7E-CDBA872CFADA}, 100
//  
//  Indicates the direction of the GPS receiver movement.  Calculated from PKEY_GPS_TrackNumerator and 
//  PKEY_GPS_TrackDenominator.
DEFINE_PROPERTYKEY(PKEY_GPS_Track, 0x76C09943, 0x7C33, 0x49E3, 0x9E, 0x7E, 0xCD, 0xBA, 0x87, 0x2C, 0xFA, 0xDA, 100);
#define INIT_PKEY_GPS_Track { { 0x76C09943, 0x7C33, 0x49E3, 0x9E, 0x7E, 0xCD, 0xBA, 0x87, 0x2C, 0xFA, 0xDA }, 100 }

//  Name:     System.GPS.TrackDenominator -- PKEY_GPS_TrackDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {C8D1920C-01F6-40C0-AC86-2F3A4AD00770}, 100
//
//  Denominator of PKEY_GPS_Track
DEFINE_PROPERTYKEY(PKEY_GPS_TrackDenominator, 0xC8D1920C, 0x01F6, 0x40C0, 0xAC, 0x86, 0x2F, 0x3A, 0x4A, 0xD0, 0x07, 0x70, 100);
#define INIT_PKEY_GPS_TrackDenominator { { 0xC8D1920C, 0x01F6, 0x40C0, 0xAC, 0x86, 0x2F, 0x3A, 0x4A, 0xD0, 0x07, 0x70 }, 100 }

//  Name:     System.GPS.TrackNumerator -- PKEY_GPS_TrackNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {702926F4-44A6-43E1-AE71-45627116893B}, 100
//
//  Numerator of PKEY_GPS_Track
DEFINE_PROPERTYKEY(PKEY_GPS_TrackNumerator, 0x702926F4, 0x44A6, 0x43E1, 0xAE, 0x71, 0x45, 0x62, 0x71, 0x16, 0x89, 0x3B, 100);
#define INIT_PKEY_GPS_TrackNumerator { { 0x702926F4, 0x44A6, 0x43E1, 0xAE, 0x71, 0x45, 0x62, 0x71, 0x16, 0x89, 0x3B }, 100 }

//  Name:     System.GPS.TrackRef -- PKEY_GPS_TrackRef
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {35DBE6FE-44C3-4400-AAAE-D2C799C407E8}, 100
//
//  Indicates reference for the direction of the GPS receiver movement.  (eg: true direction, magnetic direction)
DEFINE_PROPERTYKEY(PKEY_GPS_TrackRef, 0x35DBE6FE, 0x44C3, 0x4400, 0xAA, 0xAE, 0xD2, 0xC7, 0x99, 0xC4, 0x07, 0xE8, 100);
#define INIT_PKEY_GPS_TrackRef { { 0x35DBE6FE, 0x44C3, 0x4400, 0xAA, 0xAE, 0xD2, 0xC7, 0x99, 0xC4, 0x07, 0xE8 }, 100 }

//  Name:     System.GPS.VersionID -- PKEY_GPS_VersionID
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {22704DA4-C6B2-4A99-8E56-F16DF8C92599}, 100
//
//  Indicates the version of the GPS information
DEFINE_PROPERTYKEY(PKEY_GPS_VersionID, 0x22704DA4, 0xC6B2, 0x4A99, 0x8E, 0x56, 0xF1, 0x6D, 0xF8, 0xC9, 0x25, 0x99, 100);
#define INIT_PKEY_GPS_VersionID { { 0x22704DA4, 0xC6B2, 0x4A99, 0x8E, 0x56, 0xF1, 0x6D, 0xF8, 0xC9, 0x25, 0x99 }, 100 }

//-----------------------------------------------------------------------------
// History properties

//  Name:     System.History.VisitCount -- PKEY_History_VisitCount
//  Type:     Int32 -- VT_I4
//  FormatID: {5CBF2787-48CF-4208-B90E-EE5E5D420294}, 7  (PKEYs relating to URLs.  Used by IE History.)
DEFINE_PROPERTYKEY(PKEY_History_VisitCount, 0x5CBF2787, 0x48CF, 0x4208, 0xB9, 0x0E, 0xEE, 0x5E, 0x5D, 0x42, 0x02, 0x94, 7);
#define INIT_PKEY_History_VisitCount { { 0x5CBF2787, 0x48CF, 0x4208, 0xB9, 0x0E, 0xEE, 0x5E, 0x5D, 0x42, 0x02, 0x94 }, 7 }

//-----------------------------------------------------------------------------
// Image properties

//  Name:     System.Image.BitDepth -- PKEY_Image_BitDepth
//  Type:     UInt32 -- VT_UI4
//  FormatID: (PSGUID_IMAGESUMMARYINFORMATION) {6444048F-4C8B-11D1-8B70-080036B11A03}, 7 (PIDISI_BITDEPTH)
//
//  
DEFINE_PROPERTYKEY(PKEY_Image_BitDepth, 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 7);
#define INIT_PKEY_Image_BitDepth { { 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 7 }

//  Name:     System.Image.ColorSpace -- PKEY_Image_ColorSpace
//  Type:     UInt16 -- VT_UI2
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 40961
//
//  PropertyTagExifColorSpace
DEFINE_PROPERTYKEY(PKEY_Image_ColorSpace, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 40961);
#define INIT_PKEY_Image_ColorSpace { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 40961 }

// Possible discrete values for PKEY_Image_ColorSpace are:
#define IMAGE_COLORSPACE_SRGB               1u
#define IMAGE_COLORSPACE_UNCALIBRATED       0xFFFFu

//  Name:     System.Image.CompressedBitsPerPixel -- PKEY_Image_CompressedBitsPerPixel
//  Type:     Double -- VT_R8
//  FormatID: {364B6FA9-37AB-482A-BE2B-AE02F60D4318}, 100
//
//  Calculated from PKEY_Image_CompressedBitsPerPixelNumerator and PKEY_Image_CompressedBitsPerPixelDenominator.
DEFINE_PROPERTYKEY(PKEY_Image_CompressedBitsPerPixel, 0x364B6FA9, 0x37AB, 0x482A, 0xBE, 0x2B, 0xAE, 0x02, 0xF6, 0x0D, 0x43, 0x18, 100);
#define INIT_PKEY_Image_CompressedBitsPerPixel { { 0x364B6FA9, 0x37AB, 0x482A, 0xBE, 0x2B, 0xAE, 0x02, 0xF6, 0x0D, 0x43, 0x18 }, 100 }

//  Name:     System.Image.CompressedBitsPerPixelDenominator -- PKEY_Image_CompressedBitsPerPixelDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {1F8844E1-24AD-4508-9DFD-5326A415CE02}, 100
//
//  Denominator of PKEY_Image_CompressedBitsPerPixel.
DEFINE_PROPERTYKEY(PKEY_Image_CompressedBitsPerPixelDenominator, 0x1F8844E1, 0x24AD, 0x4508, 0x9D, 0xFD, 0x53, 0x26, 0xA4, 0x15, 0xCE, 0x02, 100);
#define INIT_PKEY_Image_CompressedBitsPerPixelDenominator { { 0x1F8844E1, 0x24AD, 0x4508, 0x9D, 0xFD, 0x53, 0x26, 0xA4, 0x15, 0xCE, 0x02 }, 100 }

//  Name:     System.Image.CompressedBitsPerPixelNumerator -- PKEY_Image_CompressedBitsPerPixelNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {D21A7148-D32C-4624-8900-277210F79C0F}, 100
//
//  Numerator of PKEY_Image_CompressedBitsPerPixel.
DEFINE_PROPERTYKEY(PKEY_Image_CompressedBitsPerPixelNumerator, 0xD21A7148, 0xD32C, 0x4624, 0x89, 0x00, 0x27, 0x72, 0x10, 0xF7, 0x9C, 0x0F, 100);
#define INIT_PKEY_Image_CompressedBitsPerPixelNumerator { { 0xD21A7148, 0xD32C, 0x4624, 0x89, 0x00, 0x27, 0x72, 0x10, 0xF7, 0x9C, 0x0F }, 100 }

//  Name:     System.Image.Compression -- PKEY_Image_Compression
//  Type:     UInt16 -- VT_UI2
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 259
//
//  Indicates the image compression level.  PropertyTagCompression.
DEFINE_PROPERTYKEY(PKEY_Image_Compression, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 259);
#define INIT_PKEY_Image_Compression { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 259 }

// Possible discrete values for PKEY_Image_Compression are:
#define IMAGE_COMPRESSION_UNCOMPRESSED      1u
#define IMAGE_COMPRESSION_CCITT_T3          2u
#define IMAGE_COMPRESSION_CCITT_T4          3u
#define IMAGE_COMPRESSION_CCITT_T6          4u
#define IMAGE_COMPRESSION_LZW               5u
#define IMAGE_COMPRESSION_JPEG              6u
#define IMAGE_COMPRESSION_PACKBITS          32773u

//  Name:     System.Image.CompressionText -- PKEY_Image_CompressionText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {3F08E66F-2F44-4BB9-A682-AC35D2562322}, 100
//  
//  This is the user-friendly form of System.Image.Compression.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Image_CompressionText, 0x3F08E66F, 0x2F44, 0x4BB9, 0xA6, 0x82, 0xAC, 0x35, 0xD2, 0x56, 0x23, 0x22, 100);
#define INIT_PKEY_Image_CompressionText { { 0x3F08E66F, 0x2F44, 0x4BB9, 0xA6, 0x82, 0xAC, 0x35, 0xD2, 0x56, 0x23, 0x22 }, 100 }

//  Name:     System.Image.Dimensions -- PKEY_Image_Dimensions
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_IMAGESUMMARYINFORMATION) {6444048F-4C8B-11D1-8B70-080036B11A03}, 13 (PIDISI_DIMENSIONS)
//
//  Indicates the dimensions of the image.
DEFINE_PROPERTYKEY(PKEY_Image_Dimensions, 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 13);
#define INIT_PKEY_Image_Dimensions { { 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 13 }

//  Name:     System.Image.HorizontalResolution -- PKEY_Image_HorizontalResolution
//  Type:     Double -- VT_R8
//  FormatID: (PSGUID_IMAGESUMMARYINFORMATION) {6444048F-4C8B-11D1-8B70-080036B11A03}, 5 (PIDISI_RESOLUTIONX)
//
//  
DEFINE_PROPERTYKEY(PKEY_Image_HorizontalResolution, 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 5);
#define INIT_PKEY_Image_HorizontalResolution { { 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 5 }

//  Name:     System.Image.HorizontalSize -- PKEY_Image_HorizontalSize
//  Type:     UInt32 -- VT_UI4
//  FormatID: (PSGUID_IMAGESUMMARYINFORMATION) {6444048F-4C8B-11D1-8B70-080036B11A03}, 3 (PIDISI_CX)
//
//  
DEFINE_PROPERTYKEY(PKEY_Image_HorizontalSize, 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 3);
#define INIT_PKEY_Image_HorizontalSize { { 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 3 }

//  Name:     System.Image.ImageID -- PKEY_Image_ImageID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {10DABE05-32AA-4C29-BF1A-63E2D220587F}, 100
DEFINE_PROPERTYKEY(PKEY_Image_ImageID, 0x10DABE05, 0x32AA, 0x4C29, 0xBF, 0x1A, 0x63, 0xE2, 0xD2, 0x20, 0x58, 0x7F, 100);
#define INIT_PKEY_Image_ImageID { { 0x10DABE05, 0x32AA, 0x4C29, 0xBF, 0x1A, 0x63, 0xE2, 0xD2, 0x20, 0x58, 0x7F }, 100 }

//  Name:     System.Image.ResolutionUnit -- PKEY_Image_ResolutionUnit
//  Type:     Int16 -- VT_I2
//  FormatID: {19B51FA6-1F92-4A5C-AB48-7DF0ABD67444}, 100
DEFINE_PROPERTYKEY(PKEY_Image_ResolutionUnit, 0x19B51FA6, 0x1F92, 0x4A5C, 0xAB, 0x48, 0x7D, 0xF0, 0xAB, 0xD6, 0x74, 0x44, 100);
#define INIT_PKEY_Image_ResolutionUnit { { 0x19B51FA6, 0x1F92, 0x4A5C, 0xAB, 0x48, 0x7D, 0xF0, 0xAB, 0xD6, 0x74, 0x44 }, 100 }

//  Name:     System.Image.VerticalResolution -- PKEY_Image_VerticalResolution
//  Type:     Double -- VT_R8
//  FormatID: (PSGUID_IMAGESUMMARYINFORMATION) {6444048F-4C8B-11D1-8B70-080036B11A03}, 6 (PIDISI_RESOLUTIONY)
//
//  
DEFINE_PROPERTYKEY(PKEY_Image_VerticalResolution, 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 6);
#define INIT_PKEY_Image_VerticalResolution { { 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 6 }

//  Name:     System.Image.VerticalSize -- PKEY_Image_VerticalSize
//  Type:     UInt32 -- VT_UI4
//  FormatID: (PSGUID_IMAGESUMMARYINFORMATION) {6444048F-4C8B-11D1-8B70-080036B11A03}, 4 (PIDISI_CY)
//
//  
DEFINE_PROPERTYKEY(PKEY_Image_VerticalSize, 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 4);
#define INIT_PKEY_Image_VerticalSize { { 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 4 }

//-----------------------------------------------------------------------------
// Journal properties

//  Name:     System.Journal.Contacts -- PKEY_Journal_Contacts
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {DEA7C82C-1D89-4A66-9427-A4E3DEBABCB1}, 100
DEFINE_PROPERTYKEY(PKEY_Journal_Contacts, 0xDEA7C82C, 0x1D89, 0x4A66, 0x94, 0x27, 0xA4, 0xE3, 0xDE, 0xBA, 0xBC, 0xB1, 100);
#define INIT_PKEY_Journal_Contacts { { 0xDEA7C82C, 0x1D89, 0x4A66, 0x94, 0x27, 0xA4, 0xE3, 0xDE, 0xBA, 0xBC, 0xB1 }, 100 }

//  Name:     System.Journal.EntryType -- PKEY_Journal_EntryType
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {95BEB1FC-326D-4644-B396-CD3ED90E6DDF}, 100
DEFINE_PROPERTYKEY(PKEY_Journal_EntryType, 0x95BEB1FC, 0x326D, 0x4644, 0xB3, 0x96, 0xCD, 0x3E, 0xD9, 0x0E, 0x6D, 0xDF, 100);
#define INIT_PKEY_Journal_EntryType { { 0x95BEB1FC, 0x326D, 0x4644, 0xB3, 0x96, 0xCD, 0x3E, 0xD9, 0x0E, 0x6D, 0xDF }, 100 }

//-----------------------------------------------------------------------------
// LayoutPattern properties

//  Name:     System.LayoutPattern.ContentViewModeForBrowse -- PKEY_LayoutPattern_ContentViewModeForBrowse
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 500
//  
//  Specifies the layout pattern that the content view mode should apply for this item in the context of browsing.
//  Register the regvalue under the name of "ContentViewModeLayoutPatternForBrowse".
DEFINE_PROPERTYKEY(PKEY_LayoutPattern_ContentViewModeForBrowse, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 500);
#define INIT_PKEY_LayoutPattern_ContentViewModeForBrowse { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 500 }

// Possible discrete values for PKEY_LayoutPattern_ContentViewModeForBrowse are:
#define LAYOUTPATTERN_CVMFB_ALPHA           L"alpha"
#define LAYOUTPATTERN_CVMFB_BETA            L"beta"
#define LAYOUTPATTERN_CVMFB_GAMMA           L"gamma"
#define LAYOUTPATTERN_CVMFB_DELTA           L"delta"

//  Name:     System.LayoutPattern.ContentViewModeForSearch -- PKEY_LayoutPattern_ContentViewModeForSearch
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 501
//  
//  Specifies the layout pattern that the content view mode should apply for this item in the context of searching.
//  Register the regvalue under the name of "ContentViewModeLayoutPatternForSearch".
DEFINE_PROPERTYKEY(PKEY_LayoutPattern_ContentViewModeForSearch, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 501);
#define INIT_PKEY_LayoutPattern_ContentViewModeForSearch { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 501 }

// Possible discrete values for PKEY_LayoutPattern_ContentViewModeForSearch are:
#define LAYOUTPATTERN_CVMFS_ALPHA           L"alpha"
#define LAYOUTPATTERN_CVMFS_BETA            L"beta"
#define LAYOUTPATTERN_CVMFS_GAMMA           L"gamma"
#define LAYOUTPATTERN_CVMFS_DELTA           L"delta"

//-----------------------------------------------------------------------------
// Link properties

//  Name:     System.History.SelectionCount -- PKEY_History_SelectionCount
//  Type:     UInt32 -- VT_UI4
//  FormatID: {1CE0D6BC-536C-4600-B0DD-7E0C66B350D5}, 8
//
//  The count of instances the user has selected the item.
DEFINE_PROPERTYKEY(PKEY_History_SelectionCount, 0x1CE0D6BC, 0x536C, 0x4600, 0xB0, 0xDD, 0x7E, 0x0C, 0x66, 0xB3, 0x50, 0xD5, 8);
#define INIT_PKEY_History_SelectionCount { { 0x1CE0D6BC, 0x536C, 0x4600, 0xB0, 0xDD, 0x7E, 0x0C, 0x66, 0xB3, 0x50, 0xD5 }, 8 }

//  Name:     System.History.TargetUrlHostName -- PKEY_History_TargetUrlHostName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {1CE0D6BC-536C-4600-B0DD-7E0C66B350D5}, 9
//
//  Mark of the Web zone, as URLZONE enumeration value.
DEFINE_PROPERTYKEY(PKEY_History_TargetUrlHostName, 0x1CE0D6BC, 0x536C, 0x4600, 0xB0, 0xDD, 0x7E, 0x0C, 0x66, 0xB3, 0x50, 0xD5, 9);
#define INIT_PKEY_History_TargetUrlHostName { { 0x1CE0D6BC, 0x536C, 0x4600, 0xB0, 0xDD, 0x7E, 0x0C, 0x66, 0xB3, 0x50, 0xD5 }, 9 }

//  Name:     System.Link.Arguments -- PKEY_Link_Arguments
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {436F2667-14E2-4FEB-B30A-146C53B5B674}, 100
DEFINE_PROPERTYKEY(PKEY_Link_Arguments, 0x436F2667, 0x14E2, 0x4FEB, 0xB3, 0x0A, 0x14, 0x6C, 0x53, 0xB5, 0xB6, 0x74, 100);
#define INIT_PKEY_Link_Arguments { { 0x436F2667, 0x14E2, 0x4FEB, 0xB3, 0x0A, 0x14, 0x6C, 0x53, 0xB5, 0xB6, 0x74 }, 100 }

//  Name:     System.Link.Comment -- PKEY_Link_Comment
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_LINK) {B9B4B3FC-2B51-4A42-B5D8-324146AFCF25}, 5
DEFINE_PROPERTYKEY(PKEY_Link_Comment, 0xB9B4B3FC, 0x2B51, 0x4A42, 0xB5, 0xD8, 0x32, 0x41, 0x46, 0xAF, 0xCF, 0x25, 5);
#define INIT_PKEY_Link_Comment { { 0xB9B4B3FC, 0x2B51, 0x4A42, 0xB5, 0xD8, 0x32, 0x41, 0x46, 0xAF, 0xCF, 0x25 }, 5 }

//  Name:     System.Link.DateVisited -- PKEY_Link_DateVisited
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {5CBF2787-48CF-4208-B90E-EE5E5D420294}, 23  (PKEYs relating to URLs.  Used by IE History.)
DEFINE_PROPERTYKEY(PKEY_Link_DateVisited, 0x5CBF2787, 0x48CF, 0x4208, 0xB9, 0x0E, 0xEE, 0x5E, 0x5D, 0x42, 0x02, 0x94, 23);
#define INIT_PKEY_Link_DateVisited { { 0x5CBF2787, 0x48CF, 0x4208, 0xB9, 0x0E, 0xEE, 0x5E, 0x5D, 0x42, 0x02, 0x94 }, 23 }

//  Name:     System.Link.Description -- PKEY_Link_Description
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {5CBF2787-48CF-4208-B90E-EE5E5D420294}, 21  (PKEYs relating to URLs.  Used by IE History.)
DEFINE_PROPERTYKEY(PKEY_Link_Description, 0x5CBF2787, 0x48CF, 0x4208, 0xB9, 0x0E, 0xEE, 0x5E, 0x5D, 0x42, 0x02, 0x94, 21);
#define INIT_PKEY_Link_Description { { 0x5CBF2787, 0x48CF, 0x4208, 0xB9, 0x0E, 0xEE, 0x5E, 0x5D, 0x42, 0x02, 0x94 }, 21 }

//  Name:     System.Link.FeedItemLocalId -- PKEY_Link_FeedItemLocalId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8A2F99F9-3C37-465D-A8D7-69777A246D0C}, 2
DEFINE_PROPERTYKEY(PKEY_Link_FeedItemLocalId, 0x8A2F99F9, 0x3C37, 0x465D, 0xA8, 0xD7, 0x69, 0x77, 0x7A, 0x24, 0x6D, 0x0C, 2);
#define INIT_PKEY_Link_FeedItemLocalId { { 0x8A2F99F9, 0x3C37, 0x465D, 0xA8, 0xD7, 0x69, 0x77, 0x7A, 0x24, 0x6D, 0x0C }, 2 }

//  Name:     System.Link.Status -- PKEY_Link_Status
//  Type:     Int32 -- VT_I4
//  FormatID: (PSGUID_LINK) {B9B4B3FC-2B51-4A42-B5D8-324146AFCF25}, 3 (PID_LINK_TARGET_TYPE)
//
//  
DEFINE_PROPERTYKEY(PKEY_Link_Status, 0xB9B4B3FC, 0x2B51, 0x4A42, 0xB5, 0xD8, 0x32, 0x41, 0x46, 0xAF, 0xCF, 0x25, 3);
#define INIT_PKEY_Link_Status { { 0xB9B4B3FC, 0x2B51, 0x4A42, 0xB5, 0xD8, 0x32, 0x41, 0x46, 0xAF, 0xCF, 0x25 }, 3 }

// Possible discrete values for PKEY_Link_Status are:
#define LINK_STATUS_RESOLVED                1l
#define LINK_STATUS_BROKEN                  2l

//  Name:     System.Link.TargetExtension -- PKEY_Link_TargetExtension
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {7A7D76F4-B630-4BD7-95FF-37CC51A975C9}, 2
//
//  The file extension of the link target.  See System.File.Extension
DEFINE_PROPERTYKEY(PKEY_Link_TargetExtension, 0x7A7D76F4, 0xB630, 0x4BD7, 0x95, 0xFF, 0x37, 0xCC, 0x51, 0xA9, 0x75, 0xC9, 2);
#define INIT_PKEY_Link_TargetExtension { { 0x7A7D76F4, 0xB630, 0x4BD7, 0x95, 0xFF, 0x37, 0xCC, 0x51, 0xA9, 0x75, 0xC9 }, 2 }

//  Name:     System.Link.TargetParsingPath -- PKEY_Link_TargetParsingPath
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_LINK) {B9B4B3FC-2B51-4A42-B5D8-324146AFCF25}, 2 (PID_LINK_TARGET)
//  
//  This is the shell namespace path to the target of the link item.  This path may be passed to 
//  SHParseDisplayName to parse the path to the correct shell folder.
//  
//  If the target item is a file, the value is identical to System.ItemPathDisplay.
//  
//  If the target item cannot be accessed through the shell namespace, this value is VT_EMPTY.
DEFINE_PROPERTYKEY(PKEY_Link_TargetParsingPath, 0xB9B4B3FC, 0x2B51, 0x4A42, 0xB5, 0xD8, 0x32, 0x41, 0x46, 0xAF, 0xCF, 0x25, 2);
#define INIT_PKEY_Link_TargetParsingPath { { 0xB9B4B3FC, 0x2B51, 0x4A42, 0xB5, 0xD8, 0x32, 0x41, 0x46, 0xAF, 0xCF, 0x25 }, 2 }

//  Name:     System.Link.TargetSFGAOFlags -- PKEY_Link_TargetSFGAOFlags
//  Type:     UInt32 -- VT_UI4
//  FormatID: (PSGUID_LINK) {B9B4B3FC-2B51-4A42-B5D8-324146AFCF25}, 8
//  
//  IShellFolder::GetAttributesOf flags for the target of a link, with SFGAO_PKEYSFGAOMASK 
//  attributes masked out.
DEFINE_PROPERTYKEY(PKEY_Link_TargetSFGAOFlags, 0xB9B4B3FC, 0x2B51, 0x4A42, 0xB5, 0xD8, 0x32, 0x41, 0x46, 0xAF, 0xCF, 0x25, 8);
#define INIT_PKEY_Link_TargetSFGAOFlags { { 0xB9B4B3FC, 0x2B51, 0x4A42, 0xB5, 0xD8, 0x32, 0x41, 0x46, 0xAF, 0xCF, 0x25 }, 8 }

//  Name:     System.Link.TargetUrlHostName -- PKEY_Link_TargetUrlHostName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8A2F99F9-3C37-465D-A8D7-69777A246D0C}, 5
DEFINE_PROPERTYKEY(PKEY_Link_TargetUrlHostName, 0x8A2F99F9, 0x3C37, 0x465D, 0xA8, 0xD7, 0x69, 0x77, 0x7A, 0x24, 0x6D, 0x0C, 5);
#define INIT_PKEY_Link_TargetUrlHostName { { 0x8A2F99F9, 0x3C37, 0x465D, 0xA8, 0xD7, 0x69, 0x77, 0x7A, 0x24, 0x6D, 0x0C }, 5 }

//  Name:     System.Link.TargetUrlPath -- PKEY_Link_TargetUrlPath
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {8A2F99F9-3C37-465D-A8D7-69777A246D0C}, 6
DEFINE_PROPERTYKEY(PKEY_Link_TargetUrlPath, 0x8A2F99F9, 0x3C37, 0x465D, 0xA8, 0xD7, 0x69, 0x77, 0x7A, 0x24, 0x6D, 0x0C, 6);
#define INIT_PKEY_Link_TargetUrlPath { { 0x8A2F99F9, 0x3C37, 0x465D, 0xA8, 0xD7, 0x69, 0x77, 0x7A, 0x24, 0x6D, 0x0C }, 6 }

//-----------------------------------------------------------------------------
// Media properties

//  Name:     System.Media.AuthorUrl -- PKEY_Media_AuthorUrl
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 32 (PIDMSI_AUTHOR_URL)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_AuthorUrl, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 32);
#define INIT_PKEY_Media_AuthorUrl { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 32 }

//  Name:     System.Media.AverageLevel -- PKEY_Media_AverageLevel
//  Type:     UInt32 -- VT_UI4
//  FormatID: {09EDD5B6-B301-43C5-9990-D00302EFFD46}, 100
DEFINE_PROPERTYKEY(PKEY_Media_AverageLevel, 0x09EDD5B6, 0xB301, 0x43C5, 0x99, 0x90, 0xD0, 0x03, 0x02, 0xEF, 0xFD, 0x46, 100);
#define INIT_PKEY_Media_AverageLevel { { 0x09EDD5B6, 0xB301, 0x43C5, 0x99, 0x90, 0xD0, 0x03, 0x02, 0xEF, 0xFD, 0x46 }, 100 }

//  Name:     System.Media.ClassPrimaryID -- PKEY_Media_ClassPrimaryID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 13 (PIDMSI_CLASS_PRIMARY_ID)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_ClassPrimaryID, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 13);
#define INIT_PKEY_Media_ClassPrimaryID { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 13 }

//  Name:     System.Media.ClassSecondaryID -- PKEY_Media_ClassSecondaryID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 14 (PIDMSI_CLASS_SECONDARY_ID)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_ClassSecondaryID, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 14);
#define INIT_PKEY_Media_ClassSecondaryID { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 14 }

//  Name:     System.Media.CollectionGroupID -- PKEY_Media_CollectionGroupID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 24 (PIDMSI_COLLECTION_GROUP_ID)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_CollectionGroupID, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 24);
#define INIT_PKEY_Media_CollectionGroupID { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 24 }

//  Name:     System.Media.CollectionID -- PKEY_Media_CollectionID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 25 (PIDMSI_COLLECTION_ID)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_CollectionID, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 25);
#define INIT_PKEY_Media_CollectionID { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 25 }

//  Name:     System.Media.ContentDistributor -- PKEY_Media_ContentDistributor
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 18 (PIDMSI_CONTENTDISTRIBUTOR)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_ContentDistributor, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 18);
#define INIT_PKEY_Media_ContentDistributor { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 18 }

//  Name:     System.Media.ContentID -- PKEY_Media_ContentID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 26 (PIDMSI_CONTENT_ID)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_ContentID, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 26);
#define INIT_PKEY_Media_ContentID { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 26 }

//  Name:     System.Media.CreatorApplication -- PKEY_Media_CreatorApplication
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 27 (PIDMSI_TOOL_NAME)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_CreatorApplication, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 27);
#define INIT_PKEY_Media_CreatorApplication { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 27 }

//  Name:     System.Media.CreatorApplicationVersion -- PKEY_Media_CreatorApplicationVersion
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 28 (PIDMSI_TOOL_VERSION)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_CreatorApplicationVersion, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 28);
#define INIT_PKEY_Media_CreatorApplicationVersion { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 28 }

//  Name:     System.Media.DateEncoded -- PKEY_Media_DateEncoded
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {2E4B640D-5019-46D8-8881-55414CC5CAA0}, 100
//
//  DateTime is in UTC (in the doc, not file system).
DEFINE_PROPERTYKEY(PKEY_Media_DateEncoded, 0x2E4B640D, 0x5019, 0x46D8, 0x88, 0x81, 0x55, 0x41, 0x4C, 0xC5, 0xCA, 0xA0, 100);
#define INIT_PKEY_Media_DateEncoded { { 0x2E4B640D, 0x5019, 0x46D8, 0x88, 0x81, 0x55, 0x41, 0x4C, 0xC5, 0xCA, 0xA0 }, 100 }

//  Name:     System.Media.DateReleased -- PKEY_Media_DateReleased
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {DE41CC29-6971-4290-B472-F59F2E2F31E2}, 100
DEFINE_PROPERTYKEY(PKEY_Media_DateReleased, 0xDE41CC29, 0x6971, 0x4290, 0xB4, 0x72, 0xF5, 0x9F, 0x2E, 0x2F, 0x31, 0xE2, 100);
#define INIT_PKEY_Media_DateReleased { { 0xDE41CC29, 0x6971, 0x4290, 0xB4, 0x72, 0xF5, 0x9F, 0x2E, 0x2F, 0x31, 0xE2 }, 100 }

//  Name:     System.Media.DlnaProfileID -- PKEY_Media_DlnaProfileID
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {CFA31B45-525D-4998-BB44-3F7D81542FA4}, 100
//
//  DLNA profile ID for media content, defined by DLNA standards at http://www.dlna.org
DEFINE_PROPERTYKEY(PKEY_Media_DlnaProfileID, 0xCFA31B45, 0x525D, 0x4998, 0xBB, 0x44, 0x3F, 0x7D, 0x81, 0x54, 0x2F, 0xA4, 100);
#define INIT_PKEY_Media_DlnaProfileID { { 0xCFA31B45, 0x525D, 0x4998, 0xBB, 0x44, 0x3F, 0x7D, 0x81, 0x54, 0x2F, 0xA4 }, 100 }

//  Name:     System.Media.Duration -- PKEY_Media_Duration
//  Type:     UInt64 -- VT_UI8
//  FormatID: (FMTID_AudioSummaryInformation) {64440490-4C8B-11D1-8B70-080036B11A03}, 3 (PIDASI_TIMELENGTH)
//
//  100ns units, not milliseconds
DEFINE_PROPERTYKEY(PKEY_Media_Duration, 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 3);
#define INIT_PKEY_Media_Duration { { 0x64440490, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 3 }

//  Name:     System.Media.DVDID -- PKEY_Media_DVDID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 15 (PIDMSI_DVDID)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_DVDID, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 15);
#define INIT_PKEY_Media_DVDID { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 15 }

//  Name:     System.Media.EncodedBy -- PKEY_Media_EncodedBy
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 36 (PIDMSI_ENCODED_BY)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_EncodedBy, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 36);
#define INIT_PKEY_Media_EncodedBy { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 36 }

//  Name:     System.Media.EncodingSettings -- PKEY_Media_EncodingSettings
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 37 (PIDMSI_ENCODING_SETTINGS)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_EncodingSettings, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 37);
#define INIT_PKEY_Media_EncodingSettings { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 37 }

//  Name:     System.Media.EpisodeNumber -- PKEY_Media_EpisodeNumber
//  Type:     UInt32 -- VT_UI4
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 100
//  
//  A 1 based monotonically incremented number that corresponds to the episode of the show
DEFINE_PROPERTYKEY(PKEY_Media_EpisodeNumber, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 100);
#define INIT_PKEY_Media_EpisodeNumber { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 100 }

//  Name:     System.Media.FrameCount -- PKEY_Media_FrameCount
//  Type:     UInt32 -- VT_UI4
//  FormatID: (PSGUID_IMAGESUMMARYINFORMATION) {6444048F-4C8B-11D1-8B70-080036B11A03}, 12 (PIDISI_FRAMECOUNT)
//
//  Indicates the frame count for the image.
DEFINE_PROPERTYKEY(PKEY_Media_FrameCount, 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 12);
#define INIT_PKEY_Media_FrameCount { { 0x6444048F, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 12 }

//  Name:     System.Media.MCDI -- PKEY_Media_MCDI
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 16 (PIDMSI_MCDI)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_MCDI, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 16);
#define INIT_PKEY_Media_MCDI { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 16 }

//  Name:     System.Media.MetadataContentProvider -- PKEY_Media_MetadataContentProvider
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 17 (PIDMSI_PROVIDER)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_MetadataContentProvider, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 17);
#define INIT_PKEY_Media_MetadataContentProvider { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 17 }

//  Name:     System.Media.Producer -- PKEY_Media_Producer
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 22 (PIDMSI_PRODUCER)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_Producer, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 22);
#define INIT_PKEY_Media_Producer { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 22 }

//  Name:     System.Media.PromotionUrl -- PKEY_Media_PromotionUrl
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 33 (PIDMSI_PROMOTION_URL)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_PromotionUrl, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 33);
#define INIT_PKEY_Media_PromotionUrl { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 33 }

//  Name:     System.Media.ProtectionType -- PKEY_Media_ProtectionType
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 38
//  
//  If media is protected, how is it protected?
DEFINE_PROPERTYKEY(PKEY_Media_ProtectionType, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 38);
#define INIT_PKEY_Media_ProtectionType { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 38 }

//  Name:     System.Media.ProviderRating -- PKEY_Media_ProviderRating
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 39
//  
//  Rating (0 - 99) supplied by metadata provider
DEFINE_PROPERTYKEY(PKEY_Media_ProviderRating, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 39);
#define INIT_PKEY_Media_ProviderRating { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 39 }

//  Name:     System.Media.ProviderStyle -- PKEY_Media_ProviderStyle
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 40
//  
//  Style of music or video, supplied by metadata provider
DEFINE_PROPERTYKEY(PKEY_Media_ProviderStyle, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 40);
#define INIT_PKEY_Media_ProviderStyle { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 40 }

//  Name:     System.Media.Publisher -- PKEY_Media_Publisher
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 30 (PIDMSI_PUBLISHER)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_Publisher, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 30);
#define INIT_PKEY_Media_Publisher { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 30 }

//  Name:     System.Media.SeasonNumber -- PKEY_Media_SeasonNumber
//  Type:     UInt32 -- VT_UI4
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 101
//  
//  A 1 based monotonically incremented number that corresponds to the season that the show was first presented
DEFINE_PROPERTYKEY(PKEY_Media_SeasonNumber, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 101);
#define INIT_PKEY_Media_SeasonNumber { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 101 }

//  Name:     System.Media.SeriesName -- PKEY_Media_SeriesName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 42
//  
//  A name that represents a specific series, such as a podcast or recorded television series.
DEFINE_PROPERTYKEY(PKEY_Media_SeriesName, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 42);
#define INIT_PKEY_Media_SeriesName { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 42 }

//  Name:     System.Media.SubscriptionContentId -- PKEY_Media_SubscriptionContentId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9AEBAE7A-9644-487D-A92C-657585ED751A}, 100
DEFINE_PROPERTYKEY(PKEY_Media_SubscriptionContentId, 0x9AEBAE7A, 0x9644, 0x487D, 0xA9, 0x2C, 0x65, 0x75, 0x85, 0xED, 0x75, 0x1A, 100);
#define INIT_PKEY_Media_SubscriptionContentId { { 0x9AEBAE7A, 0x9644, 0x487D, 0xA9, 0x2C, 0x65, 0x75, 0x85, 0xED, 0x75, 0x1A }, 100 }

//  Name:     System.Media.SubTitle -- PKEY_Media_SubTitle
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 38 (PIDSI_MUSIC_SUB_TITLE)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_SubTitle, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 38);
#define INIT_PKEY_Media_SubTitle { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 38 }

//  Name:     System.Media.ThumbnailLargePath -- PKEY_Media_ThumbnailLargePath
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 47
//  
//  Filesystem path to the large thumbnail representation of the media item.
DEFINE_PROPERTYKEY(PKEY_Media_ThumbnailLargePath, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 47);
#define INIT_PKEY_Media_ThumbnailLargePath { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 47 }

//  Name:     System.Media.ThumbnailLargeUri -- PKEY_Media_ThumbnailLargeUri
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 48
//  
//  Uri of the large thumbnail representation of the media item.
DEFINE_PROPERTYKEY(PKEY_Media_ThumbnailLargeUri, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 48);
#define INIT_PKEY_Media_ThumbnailLargeUri { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 48 }

//  Name:     System.Media.ThumbnailSmallPath -- PKEY_Media_ThumbnailSmallPath
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 49
//  
//  Filesystem path to the large thumbnail representation of the media item.
DEFINE_PROPERTYKEY(PKEY_Media_ThumbnailSmallPath, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 49);
#define INIT_PKEY_Media_ThumbnailSmallPath { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 49 }

//  Name:     System.Media.ThumbnailSmallUri -- PKEY_Media_ThumbnailSmallUri
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 50
//  
//  Uri of the large thumbnail representation of the media item.
DEFINE_PROPERTYKEY(PKEY_Media_ThumbnailSmallUri, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 50);
#define INIT_PKEY_Media_ThumbnailSmallUri { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 50 }

//  Name:     System.Media.UniqueFileIdentifier -- PKEY_Media_UniqueFileIdentifier
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 35 (PIDMSI_UNIQUE_FILE_IDENTIFIER)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_UniqueFileIdentifier, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 35);
#define INIT_PKEY_Media_UniqueFileIdentifier { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 35 }

//  Name:     System.Media.UserNoAutoInfo -- PKEY_Media_UserNoAutoInfo
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 41
//  
//  If true, do NOT alter this file's metadata. Set by user.
DEFINE_PROPERTYKEY(PKEY_Media_UserNoAutoInfo, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 41);
#define INIT_PKEY_Media_UserNoAutoInfo { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 41 }

//  Name:     System.Media.UserWebUrl -- PKEY_Media_UserWebUrl
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 34 (PIDMSI_USER_WEB_URL)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_UserWebUrl, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 34);
#define INIT_PKEY_Media_UserWebUrl { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 34 }

//  Name:     System.Media.Writer -- PKEY_Media_Writer
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 23 (PIDMSI_WRITER)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_Writer, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 23);
#define INIT_PKEY_Media_Writer { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 23 }

//  Name:     System.Media.Year -- PKEY_Media_Year
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 5 (PIDSI_MUSIC_YEAR)
//
//  
DEFINE_PROPERTYKEY(PKEY_Media_Year, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 5);
#define INIT_PKEY_Media_Year { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 5 }

//-----------------------------------------------------------------------------
// Message properties

//  Name:     System.Message.AttachmentContents -- PKEY_Message_AttachmentContents
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {3143BF7C-80A8-4854-8880-E2E40189BDD0}, 100
DEFINE_PROPERTYKEY(PKEY_Message_AttachmentContents, 0x3143BF7C, 0x80A8, 0x4854, 0x88, 0x80, 0xE2, 0xE4, 0x01, 0x89, 0xBD, 0xD0, 100);
#define INIT_PKEY_Message_AttachmentContents { { 0x3143BF7C, 0x80A8, 0x4854, 0x88, 0x80, 0xE2, 0xE4, 0x01, 0x89, 0xBD, 0xD0 }, 100 }

//  Name:     System.Message.AttachmentNames -- PKEY_Message_AttachmentNames
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 21
//
//  The names of the attachments in a message
DEFINE_PROPERTYKEY(PKEY_Message_AttachmentNames, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 21);
#define INIT_PKEY_Message_AttachmentNames { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 21 }

//  Name:     System.Message.BccAddress -- PKEY_Message_BccAddress
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 2
//
//  Addresses in Bcc: field
DEFINE_PROPERTYKEY(PKEY_Message_BccAddress, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 2);
#define INIT_PKEY_Message_BccAddress { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 2 }

//  Name:     System.Message.BccName -- PKEY_Message_BccName
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 3
//
//  person names in Bcc: field
DEFINE_PROPERTYKEY(PKEY_Message_BccName, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 3);
#define INIT_PKEY_Message_BccName { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 3 }

//  Name:     System.Message.CcAddress -- PKEY_Message_CcAddress
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 4
//
//  Addresses in Cc: field
DEFINE_PROPERTYKEY(PKEY_Message_CcAddress, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 4);
#define INIT_PKEY_Message_CcAddress { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 4 }

//  Name:     System.Message.CcName -- PKEY_Message_CcName
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 5
//
//  person names in Cc: field
DEFINE_PROPERTYKEY(PKEY_Message_CcName, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 5);
#define INIT_PKEY_Message_CcName { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 5 }

//  Name:     System.Message.ConversationID -- PKEY_Message_ConversationID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {DC8F80BD-AF1E-4289-85B6-3DFC1B493992}, 100
DEFINE_PROPERTYKEY(PKEY_Message_ConversationID, 0xDC8F80BD, 0xAF1E, 0x4289, 0x85, 0xB6, 0x3D, 0xFC, 0x1B, 0x49, 0x39, 0x92, 100);
#define INIT_PKEY_Message_ConversationID { { 0xDC8F80BD, 0xAF1E, 0x4289, 0x85, 0xB6, 0x3D, 0xFC, 0x1B, 0x49, 0x39, 0x92 }, 100 }

//  Name:     System.Message.ConversationIndex -- PKEY_Message_ConversationIndex
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {DC8F80BD-AF1E-4289-85B6-3DFC1B493992}, 101
//  
//  
DEFINE_PROPERTYKEY(PKEY_Message_ConversationIndex, 0xDC8F80BD, 0xAF1E, 0x4289, 0x85, 0xB6, 0x3D, 0xFC, 0x1B, 0x49, 0x39, 0x92, 101);
#define INIT_PKEY_Message_ConversationIndex { { 0xDC8F80BD, 0xAF1E, 0x4289, 0x85, 0xB6, 0x3D, 0xFC, 0x1B, 0x49, 0x39, 0x92 }, 101 }

//  Name:     System.Message.DateReceived -- PKEY_Message_DateReceived
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 20
//
//  Date and Time communication was received
DEFINE_PROPERTYKEY(PKEY_Message_DateReceived, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 20);
#define INIT_PKEY_Message_DateReceived { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 20 }

//  Name:     System.Message.DateSent -- PKEY_Message_DateSent
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 19
//
//  Date and Time communication was sent
DEFINE_PROPERTYKEY(PKEY_Message_DateSent, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 19);
#define INIT_PKEY_Message_DateSent { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 19 }

//  Name:     System.Message.Flags -- PKEY_Message_Flags
//  Type:     Int32 -- VT_I4
//  FormatID: {A82D9EE7-CA67-4312-965E-226BCEA85023}, 100
//  
//  These are flags associated with email messages to know if a read receipt is pending, etc.
//  The values stored here by Outlook are defined for PR_MESSAGE_FLAGS on MSDN. 
DEFINE_PROPERTYKEY(PKEY_Message_Flags, 0xA82D9EE7, 0xCA67, 0x4312, 0x96, 0x5E, 0x22, 0x6B, 0xCE, 0xA8, 0x50, 0x23, 100);
#define INIT_PKEY_Message_Flags { { 0xA82D9EE7, 0xCA67, 0x4312, 0x96, 0x5E, 0x22, 0x6B, 0xCE, 0xA8, 0x50, 0x23 }, 100 }

//  Name:     System.Message.FromAddress -- PKEY_Message_FromAddress
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 13
DEFINE_PROPERTYKEY(PKEY_Message_FromAddress, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 13);
#define INIT_PKEY_Message_FromAddress { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 13 }

//  Name:     System.Message.FromName -- PKEY_Message_FromName
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 14
//
//  Address in from field as person name
DEFINE_PROPERTYKEY(PKEY_Message_FromName, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 14);
#define INIT_PKEY_Message_FromName { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 14 }

//  Name:     System.Message.HasAttachments -- PKEY_Message_HasAttachments
//  Type:     Boolean -- VT_BOOL
//  FormatID: {9C1FCF74-2D97-41BA-B4AE-CB2E3661A6E4}, 8
//
//  
DEFINE_PROPERTYKEY(PKEY_Message_HasAttachments, 0x9C1FCF74, 0x2D97, 0x41BA, 0xB4, 0xAE, 0xCB, 0x2E, 0x36, 0x61, 0xA6, 0xE4, 8);
#define INIT_PKEY_Message_HasAttachments { { 0x9C1FCF74, 0x2D97, 0x41BA, 0xB4, 0xAE, 0xCB, 0x2E, 0x36, 0x61, 0xA6, 0xE4 }, 8 }

//  Name:     System.Message.IsFwdOrReply -- PKEY_Message_IsFwdOrReply
//  Type:     Int32 -- VT_I4
//  FormatID: {9A9BC088-4F6D-469E-9919-E705412040F9}, 100
DEFINE_PROPERTYKEY(PKEY_Message_IsFwdOrReply, 0x9A9BC088, 0x4F6D, 0x469E, 0x99, 0x19, 0xE7, 0x05, 0x41, 0x20, 0x40, 0xF9, 100);
#define INIT_PKEY_Message_IsFwdOrReply { { 0x9A9BC088, 0x4F6D, 0x469E, 0x99, 0x19, 0xE7, 0x05, 0x41, 0x20, 0x40, 0xF9 }, 100 }

//  Name:     System.Message.MessageClass -- PKEY_Message_MessageClass
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {CD9ED458-08CE-418F-A70E-F912C7BB9C5C}, 103
//  
//  What type of outlook msg this is (meeting, task, mail, etc.)
DEFINE_PROPERTYKEY(PKEY_Message_MessageClass, 0xCD9ED458, 0x08CE, 0x418F, 0xA7, 0x0E, 0xF9, 0x12, 0xC7, 0xBB, 0x9C, 0x5C, 103);
#define INIT_PKEY_Message_MessageClass { { 0xCD9ED458, 0x08CE, 0x418F, 0xA7, 0x0E, 0xF9, 0x12, 0xC7, 0xBB, 0x9C, 0x5C }, 103 }

//  Name:     System.Message.Participants -- PKEY_Message_Participants
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {1A9BA605-8E7C-4D11-AD7D-A50ADA18BA1B}, 2
//
//  Participants in communication.
DEFINE_PROPERTYKEY(PKEY_Message_Participants, 0x1A9BA605, 0x8E7C, 0x4D11, 0xAD, 0x7D, 0xA5, 0x0A, 0xDA, 0x18, 0xBA, 0x1B, 2);
#define INIT_PKEY_Message_Participants { { 0x1A9BA605, 0x8E7C, 0x4D11, 0xAD, 0x7D, 0xA5, 0x0A, 0xDA, 0x18, 0xBA, 0x1B }, 2 }

//  Name:     System.Message.ProofInProgress -- PKEY_Message_ProofInProgress
//  Type:     Boolean -- VT_BOOL
//  FormatID: {9098F33C-9A7D-48A8-8DE5-2E1227A64E91}, 100
//  
//  This property will be true if the message junk email proofing is still in progress.
DEFINE_PROPERTYKEY(PKEY_Message_ProofInProgress, 0x9098F33C, 0x9A7D, 0x48A8, 0x8D, 0xE5, 0x2E, 0x12, 0x27, 0xA6, 0x4E, 0x91, 100);
#define INIT_PKEY_Message_ProofInProgress { { 0x9098F33C, 0x9A7D, 0x48A8, 0x8D, 0xE5, 0x2E, 0x12, 0x27, 0xA6, 0x4E, 0x91 }, 100 }

//  Name:     System.Message.SenderAddress -- PKEY_Message_SenderAddress
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {0BE1C8E7-1981-4676-AE14-FDD78F05A6E7}, 100
DEFINE_PROPERTYKEY(PKEY_Message_SenderAddress, 0x0BE1C8E7, 0x1981, 0x4676, 0xAE, 0x14, 0xFD, 0xD7, 0x8F, 0x05, 0xA6, 0xE7, 100);
#define INIT_PKEY_Message_SenderAddress { { 0x0BE1C8E7, 0x1981, 0x4676, 0xAE, 0x14, 0xFD, 0xD7, 0x8F, 0x05, 0xA6, 0xE7 }, 100 }

//  Name:     System.Message.SenderName -- PKEY_Message_SenderName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {0DA41CFA-D224-4A18-AE2F-596158DB4B3A}, 100
DEFINE_PROPERTYKEY(PKEY_Message_SenderName, 0x0DA41CFA, 0xD224, 0x4A18, 0xAE, 0x2F, 0x59, 0x61, 0x58, 0xDB, 0x4B, 0x3A, 100);
#define INIT_PKEY_Message_SenderName { { 0x0DA41CFA, 0xD224, 0x4A18, 0xAE, 0x2F, 0x59, 0x61, 0x58, 0xDB, 0x4B, 0x3A }, 100 }

//  Name:     System.Message.Store -- PKEY_Message_Store
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 15
//
//  The store (aka protocol handler) FILE, MAIL, OUTLOOKEXPRESS
DEFINE_PROPERTYKEY(PKEY_Message_Store, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 15);
#define INIT_PKEY_Message_Store { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 15 }

//  Name:     System.Message.ToAddress -- PKEY_Message_ToAddress
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 16
//
//  Addresses in To: field
DEFINE_PROPERTYKEY(PKEY_Message_ToAddress, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 16);
#define INIT_PKEY_Message_ToAddress { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 16 }

//  Name:     System.Message.ToDoFlags -- PKEY_Message_ToDoFlags
//  Type:     Int32 -- VT_I4
//  FormatID: {1F856A9F-6900-4ABA-9505-2D5F1B4D66CB}, 100
//  
//  Flags associated with a message flagged to know if it's still active, if it was custom flagged, etc.
DEFINE_PROPERTYKEY(PKEY_Message_ToDoFlags, 0x1F856A9F, 0x6900, 0x4ABA, 0x95, 0x05, 0x2D, 0x5F, 0x1B, 0x4D, 0x66, 0xCB, 100);
#define INIT_PKEY_Message_ToDoFlags { { 0x1F856A9F, 0x6900, 0x4ABA, 0x95, 0x05, 0x2D, 0x5F, 0x1B, 0x4D, 0x66, 0xCB }, 100 }

//  Name:     System.Message.ToDoTitle -- PKEY_Message_ToDoTitle
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {BCCC8A3C-8CEF-42E5-9B1C-C69079398BC7}, 100
DEFINE_PROPERTYKEY(PKEY_Message_ToDoTitle, 0xBCCC8A3C, 0x8CEF, 0x42E5, 0x9B, 0x1C, 0xC6, 0x90, 0x79, 0x39, 0x8B, 0xC7, 100);
#define INIT_PKEY_Message_ToDoTitle { { 0xBCCC8A3C, 0x8CEF, 0x42E5, 0x9B, 0x1C, 0xC6, 0x90, 0x79, 0x39, 0x8B, 0xC7 }, 100 }

//  Name:     System.Message.ToName -- PKEY_Message_ToName
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {E3E0584C-B788-4A5A-BB20-7F5A44C9ACDD}, 17
//
//  Person names in To: field
DEFINE_PROPERTYKEY(PKEY_Message_ToName, 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD, 17);
#define INIT_PKEY_Message_ToName { { 0xE3E0584C, 0xB788, 0x4A5A, 0xBB, 0x20, 0x7F, 0x5A, 0x44, 0xC9, 0xAC, 0xDD }, 17 }

//-----------------------------------------------------------------------------
// MsGraph properties

//  Name:     System.MsGraph.ActivityType -- PKEY_MsGraph_ActivityType
//  Type:     UInt32 -- VT_UI4
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 14
//
//  Represents information about the activity type from activity json
DEFINE_PROPERTYKEY(PKEY_MsGraph_ActivityType, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 14);
#define INIT_PKEY_MsGraph_ActivityType { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 14 }

//  Name:     System.MsGraph.CompositeId -- PKEY_MsGraph_CompositeId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 2
//
//  The Microsoft Graph unique composite identifier of this item.
DEFINE_PROPERTYKEY(PKEY_MsGraph_CompositeId, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 2);
#define INIT_PKEY_MsGraph_CompositeId { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 2 }

//  Name:     System.MsGraph.DateLastShared -- PKEY_MsGraph_DateLastShared
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 9
//
//  The latest time this file was shared by any user.
DEFINE_PROPERTYKEY(PKEY_MsGraph_DateLastShared, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 9);
#define INIT_PKEY_MsGraph_DateLastShared { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 9 }

//  Name:     System.MsGraph.DriveId -- PKEY_MsGraph_DriveId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 3
//
//  The Microsoft Graph unique identifier of the drive instance that contains the item.
DEFINE_PROPERTYKEY(PKEY_MsGraph_DriveId, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 3);
#define INIT_PKEY_MsGraph_DriveId { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 3 }

//  Name:     System.MsGraph.GraphFileType -- PKEY_MsGraph_GraphFileType
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 16
//
//  A data provider defined string to represent what app (local or web) can open this graph file. Useful for telemetry if extension is not available
DEFINE_PROPERTYKEY(PKEY_MsGraph_GraphFileType, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 16);
#define INIT_PKEY_MsGraph_GraphFileType { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 16 }

//  Name:     System.MsGraph.IconUrl -- PKEY_MsGraph_IconUrl
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 15
//
//  A URL for an icon that represents the file
DEFINE_PROPERTYKEY(PKEY_MsGraph_IconUrl, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 15);
#define INIT_PKEY_MsGraph_IconUrl { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 15 }

//  Name:     System.MsGraph.ItemId -- PKEY_MsGraph_ItemId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 4
//
//  The Microsoft Graph unique identifier of this item in the drive.
DEFINE_PROPERTYKEY(PKEY_MsGraph_ItemId, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 4);
#define INIT_PKEY_MsGraph_ItemId { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 4 }

//  Name:     System.MsGraph.PrimaryActivityActorDisplayName -- PKEY_MsGraph_PrimaryActivityActorDisplayName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 13
//
//  The JSON object represents information about the Display Name of the primary actor of the activity text
DEFINE_PROPERTYKEY(PKEY_MsGraph_PrimaryActivityActorDisplayName, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 13);
#define INIT_PKEY_MsGraph_PrimaryActivityActorDisplayName { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 13 }

//  Name:     System.MsGraph.PrimaryActivityActorUpn -- PKEY_MsGraph_PrimaryActivityActorUpn
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 12
//
//  The JSON object represents information about the User Principal Name of the primary actor of the activity text
DEFINE_PROPERTYKEY(PKEY_MsGraph_PrimaryActivityActorUpn, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 12);
#define INIT_PKEY_MsGraph_PrimaryActivityActorUpn { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 12 }

//  Name:     System.MsGraph.RecommendationReason -- PKEY_MsGraph_RecommendationReason
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 8
//
//  The JSON object representing information about the reason marker for a recommended item.
DEFINE_PROPERTYKEY(PKEY_MsGraph_RecommendationReason, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 8);
#define INIT_PKEY_MsGraph_RecommendationReason { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 8 }

//  Name:     System.MsGraph.RecommendationReferenceId -- PKEY_MsGraph_RecommendationReferenceId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 5
//
//  The 3S Recommendations reference Id used as a unique Id for recommended items.
DEFINE_PROPERTYKEY(PKEY_MsGraph_RecommendationReferenceId, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 5);
#define INIT_PKEY_MsGraph_RecommendationReferenceId { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 5 }

//  Name:     System.MsGraph.RecommendationResultSourceId -- PKEY_MsGraph_RecommendationResultSourceId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 7
//
//  The recommendation result source Id.
DEFINE_PROPERTYKEY(PKEY_MsGraph_RecommendationResultSourceId, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 7);
#define INIT_PKEY_MsGraph_RecommendationResultSourceId { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 7 }

//  Name:     System.MsGraph.SharedByEmail -- PKEY_MsGraph_SharedByEmail
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 11
//
//  The display Email address of last person who shared the file.
DEFINE_PROPERTYKEY(PKEY_MsGraph_SharedByEmail, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 11);
#define INIT_PKEY_MsGraph_SharedByEmail { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 11 }

//  Name:     System.MsGraph.SharedByName -- PKEY_MsGraph_SharedByName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 10
//
//  The display name of last person who shared the file.
DEFINE_PROPERTYKEY(PKEY_MsGraph_SharedByName, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 10);
#define INIT_PKEY_MsGraph_SharedByName { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 10 }

//  Name:     System.MsGraph.WebAccountId -- PKEY_MsGraph_WebAccountId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {4F85567E-FFF0-4DF5-B1D9-98B314FF0729}, 6
//
//  The WAM account identifier associated with this item.
DEFINE_PROPERTYKEY(PKEY_MsGraph_WebAccountId, 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29, 6);
#define INIT_PKEY_MsGraph_WebAccountId { { 0x4F85567E, 0xFFF0, 0x4DF5, 0xB1, 0xD9, 0x98, 0xB3, 0x14, 0xFF, 0x07, 0x29 }, 6 }

//-----------------------------------------------------------------------------
// Music properties

//  Name:     System.Music.AlbumArtist -- PKEY_Music_AlbumArtist
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 13 (PIDSI_MUSIC_ALBUM_ARTIST)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_AlbumArtist, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 13);
#define INIT_PKEY_Music_AlbumArtist { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 13 }

//  Name:     System.Music.AlbumArtistSortOverride -- PKEY_Music_AlbumArtistSortOverride
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {F1FDB4AF-F78C-466C-BB05-56E92DB0B8EC}, 103 (PIDSI_MUSIC_ALBUM_ARTIST_SORT_OVERRIDE)
//  
//  This optional string value allows for overriding the standard sort order of System.Music.AlbumArtist.
//  This is very important for proper sorting of music files in Japanese which cannot be
//  correctly sorted phonetically (the user-expected ordering) without this field.
//  It can also be used for customizing sorting in non-East Asian scenarios,
//  such as allowing the user to remove articles for sorting purposes.
DEFINE_PROPERTYKEY(PKEY_Music_AlbumArtistSortOverride, 0xF1FDB4AF, 0xF78C, 0x466C, 0xBB, 0x05, 0x56, 0xE9, 0x2D, 0xB0, 0xB8, 0xEC, 103);
#define INIT_PKEY_Music_AlbumArtistSortOverride { { 0xF1FDB4AF, 0xF78C, 0x466C, 0xBB, 0x05, 0x56, 0xE9, 0x2D, 0xB0, 0xB8, 0xEC }, 103 }

//  Name:     System.Music.AlbumID -- PKEY_Music_AlbumID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 100
//  
//  Concatenation of System.Music.AlbumArtist and System.Music.AlbumTitle, suitable for indexing and display.
//  Used to differentiate albums with the same title from different artists.
DEFINE_PROPERTYKEY(PKEY_Music_AlbumID, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 100);
#define INIT_PKEY_Music_AlbumID { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 100 }

//  Name:     System.Music.AlbumTitle -- PKEY_Music_AlbumTitle
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 4 (PIDSI_MUSIC_ALBUM)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_AlbumTitle, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 4);
#define INIT_PKEY_Music_AlbumTitle { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 4 }

//  Name:     System.Music.AlbumTitleSortOverride -- PKEY_Music_AlbumTitleSortOverride
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {13EB7FFC-EC89-4346-B19D-CCC6F1784223}, 101 (PIDSI_MUSIC_ALBUM_TITLE_SORT_OVERRIDE)
//  
//  This optional string value allows for overriding the standard sort order of System.Music.Album.
//  This is very important for proper sorting of music files in Japanese which cannot be
//  correctly sorted phonetically (the user-expected ordering) without this field.
//  It can also be used for customizing sorting in non-East Asian scenarios,
//  such as allowing the user to remove articles for sorting purposes.
DEFINE_PROPERTYKEY(PKEY_Music_AlbumTitleSortOverride, 0x13EB7FFC, 0xEC89, 0x4346, 0xB1, 0x9D, 0xCC, 0xC6, 0xF1, 0x78, 0x42, 0x23, 101);
#define INIT_PKEY_Music_AlbumTitleSortOverride { { 0x13EB7FFC, 0xEC89, 0x4346, 0xB1, 0x9D, 0xCC, 0xC6, 0xF1, 0x78, 0x42, 0x23 }, 101 }

//  Name:     System.Music.Artist -- PKEY_Music_Artist
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 2 (PIDSI_MUSIC_ARTIST)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_Artist, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 2);
#define INIT_PKEY_Music_Artist { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 2 }

//  Name:     System.Music.ArtistSortOverride -- PKEY_Music_ArtistSortOverride
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {DEEB2DB5-0696-4CE0-94FE-A01F77A45FB5}, 102 (PIDSI_MUSIC_ARTIST_SORT_OVERRIDE)
//  
//  This optional string value allows for overriding the standard sort order of System.Music.Artist.
//  This is very important for proper sorting of music files in Japanese which cannot be
//  correctly sorted phonetically (the user-expected ordering) without this field.
//  It can also be used for customizing sorting in non-East Asian scenarios,
//  such as allowing the user to remove articles for sorting purposes.
DEFINE_PROPERTYKEY(PKEY_Music_ArtistSortOverride, 0xDEEB2DB5, 0x0696, 0x4CE0, 0x94, 0xFE, 0xA0, 0x1F, 0x77, 0xA4, 0x5F, 0xB5, 102);
#define INIT_PKEY_Music_ArtistSortOverride { { 0xDEEB2DB5, 0x0696, 0x4CE0, 0x94, 0xFE, 0xA0, 0x1F, 0x77, 0xA4, 0x5F, 0xB5 }, 102 }

//  Name:     System.Music.BeatsPerMinute -- PKEY_Music_BeatsPerMinute
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 35 (PIDSI_MUSIC_BEATS_PER_MINUTE)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_BeatsPerMinute, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 35);
#define INIT_PKEY_Music_BeatsPerMinute { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 35 }

//  Name:     System.Music.Composer -- PKEY_Music_Composer
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 19 (PIDMSI_COMPOSER)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_Composer, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 19);
#define INIT_PKEY_Music_Composer { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 19 }

//  Name:     System.Music.ComposerSortOverride -- PKEY_Music_ComposerSortOverride
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {00BC20A3-BD48-4085-872C-A88D77F5097E}, 105 (PIDSI_COMPOSER_SORT_OVERRIDE)
//  
//  This optional string value allows for overriding the standard sort order of System.Music.Composer.
//  This is very important for proper sorting of music files in Japanese which cannot be
//  correctly sorted phonetically (the user-expected ordering) without this field.
//  It can also be used for customizing sorting in non-East Asian scenarios,
//  such as allowing the user to remove articles for sorting purposes.
DEFINE_PROPERTYKEY(PKEY_Music_ComposerSortOverride, 0x00BC20A3, 0xBD48, 0x4085, 0x87, 0x2C, 0xA8, 0x8D, 0x77, 0xF5, 0x09, 0x7E, 105);
#define INIT_PKEY_Music_ComposerSortOverride { { 0x00BC20A3, 0xBD48, 0x4085, 0x87, 0x2C, 0xA8, 0x8D, 0x77, 0xF5, 0x09, 0x7E }, 105 }

//  Name:     System.Music.Conductor -- PKEY_Music_Conductor
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 36 (PIDSI_MUSIC_CONDUCTOR)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_Conductor, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 36);
#define INIT_PKEY_Music_Conductor { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 36 }

//  Name:     System.Music.ContentGroupDescription -- PKEY_Music_ContentGroupDescription
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 33 (PIDSI_MUSIC_CONTENT_GROUP_DESCRIPTION)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_ContentGroupDescription, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 33);
#define INIT_PKEY_Music_ContentGroupDescription { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 33 }

//  Name:     System.Music.DiscNumber -- PKEY_Music_DiscNumber
//  Type:     UInt32 -- VT_UI4
//  FormatID: {6AFE7437-9BCD-49C7-80FE-4A5C65FA5874}, 104
DEFINE_PROPERTYKEY(PKEY_Music_DiscNumber, 0x6AFE7437, 0x9BCD, 0x49C7, 0x80, 0xFE, 0x4A, 0x5C, 0x65, 0xFA, 0x58, 0x74, 104);
#define INIT_PKEY_Music_DiscNumber { { 0x6AFE7437, 0x9BCD, 0x49C7, 0x80, 0xFE, 0x4A, 0x5C, 0x65, 0xFA, 0x58, 0x74 }, 104 }

//  Name:     System.Music.DisplayArtist -- PKEY_Music_DisplayArtist
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FD122953-FA93-4EF7-92C3-04C946B2F7C8}, 100
//  
//  This property returns the best representation of Album Artist for a given music file
//  based upon AlbumArtist, ContributingArtist and compilation info.
DEFINE_PROPERTYKEY(PKEY_Music_DisplayArtist, 0xFD122953, 0xFA93, 0x4EF7, 0x92, 0xC3, 0x04, 0xC9, 0x46, 0xB2, 0xF7, 0xC8, 100);
#define INIT_PKEY_Music_DisplayArtist { { 0xFD122953, 0xFA93, 0x4EF7, 0x92, 0xC3, 0x04, 0xC9, 0x46, 0xB2, 0xF7, 0xC8 }, 100 }

//  Name:     System.Music.Genre -- PKEY_Music_Genre
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 11 (PIDSI_MUSIC_GENRE)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_Genre, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 11);
#define INIT_PKEY_Music_Genre { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 11 }

//  Name:     System.Music.InitialKey -- PKEY_Music_InitialKey
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 34 (PIDSI_MUSIC_INITIAL_KEY)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_InitialKey, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 34);
#define INIT_PKEY_Music_InitialKey { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 34 }

//  Name:     System.Music.IsCompilation -- PKEY_Music_IsCompilation
//  Type:     Boolean -- VT_BOOL
//  FormatID: {C449D5CB-9EA4-4809-82E8-AF9D59DED6D1}, 100
//
//  Indicates whether the file is part of a compilation.
DEFINE_PROPERTYKEY(PKEY_Music_IsCompilation, 0xC449D5CB, 0x9EA4, 0x4809, 0x82, 0xE8, 0xAF, 0x9D, 0x59, 0xDE, 0xD6, 0xD1, 100);
#define INIT_PKEY_Music_IsCompilation { { 0xC449D5CB, 0x9EA4, 0x4809, 0x82, 0xE8, 0xAF, 0x9D, 0x59, 0xDE, 0xD6, 0xD1 }, 100 }

//  Name:     System.Music.Lyrics -- PKEY_Music_Lyrics
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 12 (PIDSI_MUSIC_LYRICS)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_Lyrics, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 12);
#define INIT_PKEY_Music_Lyrics { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 12 }

//  Name:     System.Music.Mood -- PKEY_Music_Mood
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 39 (PIDSI_MUSIC_MOOD)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_Mood, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 39);
#define INIT_PKEY_Music_Mood { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 39 }

//  Name:     System.Music.PartOfSet -- PKEY_Music_PartOfSet
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 37 (PIDSI_MUSIC_PART_OF_SET)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_PartOfSet, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 37);
#define INIT_PKEY_Music_PartOfSet { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 37 }

//  Name:     System.Music.Period -- PKEY_Music_Period
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 31 (PIDMSI_PERIOD)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_Period, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 31);
#define INIT_PKEY_Music_Period { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 31 }

//  Name:     System.Music.SynchronizedLyrics -- PKEY_Music_SynchronizedLyrics
//  Type:     Blob -- VT_BLOB
//  FormatID: {6B223B6A-162E-4AA9-B39F-05D678FC6D77}, 100
DEFINE_PROPERTYKEY(PKEY_Music_SynchronizedLyrics, 0x6B223B6A, 0x162E, 0x4AA9, 0xB3, 0x9F, 0x05, 0xD6, 0x78, 0xFC, 0x6D, 0x77, 100);
#define INIT_PKEY_Music_SynchronizedLyrics { { 0x6B223B6A, 0x162E, 0x4AA9, 0xB3, 0x9F, 0x05, 0xD6, 0x78, 0xFC, 0x6D, 0x77 }, 100 }

//  Name:     System.Music.TrackNumber -- PKEY_Music_TrackNumber
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_MUSIC) {56A3372E-CE9C-11D2-9F0E-006097C686F6}, 7 (PIDSI_MUSIC_TRACK)
//
//  
DEFINE_PROPERTYKEY(PKEY_Music_TrackNumber, 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6, 7);
#define INIT_PKEY_Music_TrackNumber { { 0x56A3372E, 0xCE9C, 0x11D2, 0x9F, 0x0E, 0x00, 0x60, 0x97, 0xC6, 0x86, 0xF6 }, 7 }

//-----------------------------------------------------------------------------
// Note properties

//  Name:     System.Note.Color -- PKEY_Note_Color
//  Type:     UInt16 -- VT_UI2
//  FormatID: {4776CAFA-BCE4-4CB1-A23E-265E76D8EB11}, 100
DEFINE_PROPERTYKEY(PKEY_Note_Color, 0x4776CAFA, 0xBCE4, 0x4CB1, 0xA2, 0x3E, 0x26, 0x5E, 0x76, 0xD8, 0xEB, 0x11, 100);
#define INIT_PKEY_Note_Color { { 0x4776CAFA, 0xBCE4, 0x4CB1, 0xA2, 0x3E, 0x26, 0x5E, 0x76, 0xD8, 0xEB, 0x11 }, 100 }

// Possible discrete values for PKEY_Note_Color are:
#define NOTE_COLOR_BLUE                     0u
#define NOTE_COLOR_GREEN                    1u
#define NOTE_COLOR_PINK                     2u
#define NOTE_COLOR_YELLOW                   3u
#define NOTE_COLOR_WHITE                    4u
#define NOTE_COLOR_LIGHTGREEN               5u

//  Name:     System.Note.ColorText -- PKEY_Note_ColorText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {46B4E8DE-CDB2-440D-885C-1658EB65B914}, 100
//  
//  This is the user-friendly form of System.Note.Color.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Note_ColorText, 0x46B4E8DE, 0xCDB2, 0x440D, 0x88, 0x5C, 0x16, 0x58, 0xEB, 0x65, 0xB9, 0x14, 100);
#define INIT_PKEY_Note_ColorText { { 0x46B4E8DE, 0xCDB2, 0x440D, 0x88, 0x5C, 0x16, 0x58, 0xEB, 0x65, 0xB9, 0x14 }, 100 }

//-----------------------------------------------------------------------------
// Photo properties

//  Name:     System.Photo.Aperture -- PKEY_Photo_Aperture
//  Type:     Double -- VT_R8
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 37378
//
//  PropertyTagExifAperture.  Calculated from PKEY_Photo_ApertureNumerator and PKEY_Photo_ApertureDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_Aperture, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 37378);
#define INIT_PKEY_Photo_Aperture { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 37378 }

//  Name:     System.Photo.ApertureDenominator -- PKEY_Photo_ApertureDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {E1A9A38B-6685-46BD-875E-570DC7AD7320}, 100
//
//  Denominator of PKEY_Photo_Aperture
DEFINE_PROPERTYKEY(PKEY_Photo_ApertureDenominator, 0xE1A9A38B, 0x6685, 0x46BD, 0x87, 0x5E, 0x57, 0x0D, 0xC7, 0xAD, 0x73, 0x20, 100);
#define INIT_PKEY_Photo_ApertureDenominator { { 0xE1A9A38B, 0x6685, 0x46BD, 0x87, 0x5E, 0x57, 0x0D, 0xC7, 0xAD, 0x73, 0x20 }, 100 }

//  Name:     System.Photo.ApertureNumerator -- PKEY_Photo_ApertureNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {0337ECEC-39FB-4581-A0BD-4C4CC51E9914}, 100
//
//  Numerator of PKEY_Photo_Aperture
DEFINE_PROPERTYKEY(PKEY_Photo_ApertureNumerator, 0x0337ECEC, 0x39FB, 0x4581, 0xA0, 0xBD, 0x4C, 0x4C, 0xC5, 0x1E, 0x99, 0x14, 100);
#define INIT_PKEY_Photo_ApertureNumerator { { 0x0337ECEC, 0x39FB, 0x4581, 0xA0, 0xBD, 0x4C, 0x4C, 0xC5, 0x1E, 0x99, 0x14 }, 100 }

//  Name:     System.Photo.Brightness -- PKEY_Photo_Brightness
//  Type:     Double -- VT_R8
//  FormatID: {1A701BF6-478C-4361-83AB-3701BB053C58}, 100 (PropertyTagExifBrightness)
//  
//  This is the brightness of the photo.
//  
//  Calculated from PKEY_Photo_BrightnessNumerator and PKEY_Photo_BrightnessDenominator.
//  
//  The units are "APEX", normally in the range of -99.99 to 99.99. If the numerator of 
//  the recorded value is FFFFFFFF.H, "Unknown" should be indicated.
DEFINE_PROPERTYKEY(PKEY_Photo_Brightness, 0x1A701BF6, 0x478C, 0x4361, 0x83, 0xAB, 0x37, 0x01, 0xBB, 0x05, 0x3C, 0x58, 100);
#define INIT_PKEY_Photo_Brightness { { 0x1A701BF6, 0x478C, 0x4361, 0x83, 0xAB, 0x37, 0x01, 0xBB, 0x05, 0x3C, 0x58 }, 100 }

//  Name:     System.Photo.BrightnessDenominator -- PKEY_Photo_BrightnessDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {6EBE6946-2321-440A-90F0-C043EFD32476}, 100
//
//  Denominator of PKEY_Photo_Brightness
DEFINE_PROPERTYKEY(PKEY_Photo_BrightnessDenominator, 0x6EBE6946, 0x2321, 0x440A, 0x90, 0xF0, 0xC0, 0x43, 0xEF, 0xD3, 0x24, 0x76, 100);
#define INIT_PKEY_Photo_BrightnessDenominator { { 0x6EBE6946, 0x2321, 0x440A, 0x90, 0xF0, 0xC0, 0x43, 0xEF, 0xD3, 0x24, 0x76 }, 100 }

//  Name:     System.Photo.BrightnessNumerator -- PKEY_Photo_BrightnessNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {9E7D118F-B314-45A0-8CFB-D654B917C9E9}, 100
//
//  Numerator of PKEY_Photo_Brightness
DEFINE_PROPERTYKEY(PKEY_Photo_BrightnessNumerator, 0x9E7D118F, 0xB314, 0x45A0, 0x8C, 0xFB, 0xD6, 0x54, 0xB9, 0x17, 0xC9, 0xE9, 100);
#define INIT_PKEY_Photo_BrightnessNumerator { { 0x9E7D118F, 0xB314, 0x45A0, 0x8C, 0xFB, 0xD6, 0x54, 0xB9, 0x17, 0xC9, 0xE9 }, 100 }

//  Name:     System.Photo.CameraManufacturer -- PKEY_Photo_CameraManufacturer
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 271 (PropertyTagEquipMake)
//
//  
DEFINE_PROPERTYKEY(PKEY_Photo_CameraManufacturer, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 271);
#define INIT_PKEY_Photo_CameraManufacturer { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 271 }

//  Name:     System.Photo.CameraModel -- PKEY_Photo_CameraModel
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 272 (PropertyTagEquipModel)
//
//  
DEFINE_PROPERTYKEY(PKEY_Photo_CameraModel, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 272);
#define INIT_PKEY_Photo_CameraModel { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 272 }

//  Name:     System.Photo.CameraSerialNumber -- PKEY_Photo_CameraSerialNumber
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 273
//
//  Serial number of camera that produced this photo
DEFINE_PROPERTYKEY(PKEY_Photo_CameraSerialNumber, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 273);
#define INIT_PKEY_Photo_CameraSerialNumber { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 273 }

//  Name:     System.Photo.Contrast -- PKEY_Photo_Contrast
//  Type:     UInt32 -- VT_UI4
//  FormatID: {2A785BA9-8D23-4DED-82E6-60A350C86A10}, 100
//  
//  This indicates the direction of contrast processing applied by the camera 
//  when the image was shot.
DEFINE_PROPERTYKEY(PKEY_Photo_Contrast, 0x2A785BA9, 0x8D23, 0x4DED, 0x82, 0xE6, 0x60, 0xA3, 0x50, 0xC8, 0x6A, 0x10, 100);
#define INIT_PKEY_Photo_Contrast { { 0x2A785BA9, 0x8D23, 0x4DED, 0x82, 0xE6, 0x60, 0xA3, 0x50, 0xC8, 0x6A, 0x10 }, 100 }

// Possible discrete values for PKEY_Photo_Contrast are:
#define PHOTO_CONTRAST_NORMAL               0ul
#define PHOTO_CONTRAST_SOFT                 1ul
#define PHOTO_CONTRAST_HARD                 2ul

//  Name:     System.Photo.ContrastText -- PKEY_Photo_ContrastText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {59DDE9F2-5253-40EA-9A8B-479E96C6249A}, 100
//  
//  This is the user-friendly form of System.Photo.Contrast.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_ContrastText, 0x59DDE9F2, 0x5253, 0x40EA, 0x9A, 0x8B, 0x47, 0x9E, 0x96, 0xC6, 0x24, 0x9A, 100);
#define INIT_PKEY_Photo_ContrastText { { 0x59DDE9F2, 0x5253, 0x40EA, 0x9A, 0x8B, 0x47, 0x9E, 0x96, 0xC6, 0x24, 0x9A }, 100 }

//  Name:     System.Photo.DateTaken -- PKEY_Photo_DateTaken
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 36867
//
//  PropertyTagExifDTOrig
DEFINE_PROPERTYKEY(PKEY_Photo_DateTaken, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 36867);
#define INIT_PKEY_Photo_DateTaken { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 36867 }

//  Name:     System.Photo.DigitalZoom -- PKEY_Photo_DigitalZoom
//  Type:     Double -- VT_R8
//  FormatID: {F85BF840-A925-4BC2-B0C4-8E36B598679E}, 100
//
//  PropertyTagExifDigitalZoom.  Calculated from PKEY_Photo_DigitalZoomNumerator and PKEY_Photo_DigitalZoomDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_DigitalZoom, 0xF85BF840, 0xA925, 0x4BC2, 0xB0, 0xC4, 0x8E, 0x36, 0xB5, 0x98, 0x67, 0x9E, 100);
#define INIT_PKEY_Photo_DigitalZoom { { 0xF85BF840, 0xA925, 0x4BC2, 0xB0, 0xC4, 0x8E, 0x36, 0xB5, 0x98, 0x67, 0x9E }, 100 }

//  Name:     System.Photo.DigitalZoomDenominator -- PKEY_Photo_DigitalZoomDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {745BAF0E-E5C1-4CFB-8A1B-D031A0A52393}, 100
//
//  Denominator of PKEY_Photo_DigitalZoom
DEFINE_PROPERTYKEY(PKEY_Photo_DigitalZoomDenominator, 0x745BAF0E, 0xE5C1, 0x4CFB, 0x8A, 0x1B, 0xD0, 0x31, 0xA0, 0xA5, 0x23, 0x93, 100);
#define INIT_PKEY_Photo_DigitalZoomDenominator { { 0x745BAF0E, 0xE5C1, 0x4CFB, 0x8A, 0x1B, 0xD0, 0x31, 0xA0, 0xA5, 0x23, 0x93 }, 100 }

//  Name:     System.Photo.DigitalZoomNumerator -- PKEY_Photo_DigitalZoomNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {16CBB924-6500-473B-A5BE-F1599BCBE413}, 100
//
//  Numerator of PKEY_Photo_DigitalZoom
DEFINE_PROPERTYKEY(PKEY_Photo_DigitalZoomNumerator, 0x16CBB924, 0x6500, 0x473B, 0xA5, 0xBE, 0xF1, 0x59, 0x9B, 0xCB, 0xE4, 0x13, 100);
#define INIT_PKEY_Photo_DigitalZoomNumerator { { 0x16CBB924, 0x6500, 0x473B, 0xA5, 0xBE, 0xF1, 0x59, 0x9B, 0xCB, 0xE4, 0x13 }, 100 }

//  Name:     System.Photo.Event -- PKEY_Photo_Event
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 18248
//
//  The event at which the photo was taken
DEFINE_PROPERTYKEY(PKEY_Photo_Event, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 18248);
#define INIT_PKEY_Photo_Event { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 18248 }

//  Name:     System.Photo.EXIFVersion -- PKEY_Photo_EXIFVersion
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {D35F743A-EB2E-47F2-A286-844132CB1427}, 100
//
//  The EXIF version.
DEFINE_PROPERTYKEY(PKEY_Photo_EXIFVersion, 0xD35F743A, 0xEB2E, 0x47F2, 0xA2, 0x86, 0x84, 0x41, 0x32, 0xCB, 0x14, 0x27, 100);
#define INIT_PKEY_Photo_EXIFVersion { { 0xD35F743A, 0xEB2E, 0x47F2, 0xA2, 0x86, 0x84, 0x41, 0x32, 0xCB, 0x14, 0x27 }, 100 }

//  Name:     System.Photo.ExposureBias -- PKEY_Photo_ExposureBias
//  Type:     Double -- VT_R8
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 37380
//
//  PropertyTagExifExposureBias.  Calculated from PKEY_Photo_ExposureBiasNumerator and PKEY_Photo_ExposureBiasDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureBias, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 37380);
#define INIT_PKEY_Photo_ExposureBias { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 37380 }

//  Name:     System.Photo.ExposureBiasDenominator -- PKEY_Photo_ExposureBiasDenominator
//  Type:     Int32 -- VT_I4
//  FormatID: {AB205E50-04B7-461C-A18C-2F233836E627}, 100
//
//  Denominator of PKEY_Photo_ExposureBias
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureBiasDenominator, 0xAB205E50, 0x04B7, 0x461C, 0xA1, 0x8C, 0x2F, 0x23, 0x38, 0x36, 0xE6, 0x27, 100);
#define INIT_PKEY_Photo_ExposureBiasDenominator { { 0xAB205E50, 0x04B7, 0x461C, 0xA1, 0x8C, 0x2F, 0x23, 0x38, 0x36, 0xE6, 0x27 }, 100 }

//  Name:     System.Photo.ExposureBiasNumerator -- PKEY_Photo_ExposureBiasNumerator
//  Type:     Int32 -- VT_I4
//  FormatID: {738BF284-1D87-420B-92CF-5834BF6EF9ED}, 100
//
//  Numerator of PKEY_Photo_ExposureBias
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureBiasNumerator, 0x738BF284, 0x1D87, 0x420B, 0x92, 0xCF, 0x58, 0x34, 0xBF, 0x6E, 0xF9, 0xED, 100);
#define INIT_PKEY_Photo_ExposureBiasNumerator { { 0x738BF284, 0x1D87, 0x420B, 0x92, 0xCF, 0x58, 0x34, 0xBF, 0x6E, 0xF9, 0xED }, 100 }

//  Name:     System.Photo.ExposureIndex -- PKEY_Photo_ExposureIndex
//  Type:     Double -- VT_R8
//  FormatID: {967B5AF8-995A-46ED-9E11-35B3C5B9782D}, 100
//
//  PropertyTagExifExposureIndex.  Calculated from PKEY_Photo_ExposureIndexNumerator and PKEY_Photo_ExposureIndexDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureIndex, 0x967B5AF8, 0x995A, 0x46ED, 0x9E, 0x11, 0x35, 0xB3, 0xC5, 0xB9, 0x78, 0x2D, 100);
#define INIT_PKEY_Photo_ExposureIndex { { 0x967B5AF8, 0x995A, 0x46ED, 0x9E, 0x11, 0x35, 0xB3, 0xC5, 0xB9, 0x78, 0x2D }, 100 }

//  Name:     System.Photo.ExposureIndexDenominator -- PKEY_Photo_ExposureIndexDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {93112F89-C28B-492F-8A9D-4BE2062CEE8A}, 100
//
//  Denominator of PKEY_Photo_ExposureIndex
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureIndexDenominator, 0x93112F89, 0xC28B, 0x492F, 0x8A, 0x9D, 0x4B, 0xE2, 0x06, 0x2C, 0xEE, 0x8A, 100);
#define INIT_PKEY_Photo_ExposureIndexDenominator { { 0x93112F89, 0xC28B, 0x492F, 0x8A, 0x9D, 0x4B, 0xE2, 0x06, 0x2C, 0xEE, 0x8A }, 100 }

//  Name:     System.Photo.ExposureIndexNumerator -- PKEY_Photo_ExposureIndexNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {CDEDCF30-8919-44DF-8F4C-4EB2FFDB8D89}, 100
//
//  Numerator of PKEY_Photo_ExposureIndex
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureIndexNumerator, 0xCDEDCF30, 0x8919, 0x44DF, 0x8F, 0x4C, 0x4E, 0xB2, 0xFF, 0xDB, 0x8D, 0x89, 100);
#define INIT_PKEY_Photo_ExposureIndexNumerator { { 0xCDEDCF30, 0x8919, 0x44DF, 0x8F, 0x4C, 0x4E, 0xB2, 0xFF, 0xDB, 0x8D, 0x89 }, 100 }

//  Name:     System.Photo.ExposureProgram -- PKEY_Photo_ExposureProgram
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 34850 (PropertyTagExifExposureProg)
//
//  
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureProgram, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 34850);
#define INIT_PKEY_Photo_ExposureProgram { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 34850 }

// Possible discrete values for PKEY_Photo_ExposureProgram are:
#define PHOTO_EXPOSUREPROGRAM_UNKNOWN       0ul
#define PHOTO_EXPOSUREPROGRAM_MANUAL        1ul
#define PHOTO_EXPOSUREPROGRAM_NORMAL        2ul
#define PHOTO_EXPOSUREPROGRAM_APERTURE      3ul
#define PHOTO_EXPOSUREPROGRAM_SHUTTER       4ul
#define PHOTO_EXPOSUREPROGRAM_CREATIVE      5ul
#define PHOTO_EXPOSUREPROGRAM_ACTION        6ul
#define PHOTO_EXPOSUREPROGRAM_PORTRAIT      7ul
#define PHOTO_EXPOSUREPROGRAM_LANDSCAPE     8ul

//  Name:     System.Photo.ExposureProgramText -- PKEY_Photo_ExposureProgramText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FEC690B7-5F30-4646-AE47-4CAAFBA884A3}, 100
//  
//  This is the user-friendly form of System.Photo.ExposureProgram.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureProgramText, 0xFEC690B7, 0x5F30, 0x4646, 0xAE, 0x47, 0x4C, 0xAA, 0xFB, 0xA8, 0x84, 0xA3, 100);
#define INIT_PKEY_Photo_ExposureProgramText { { 0xFEC690B7, 0x5F30, 0x4646, 0xAE, 0x47, 0x4C, 0xAA, 0xFB, 0xA8, 0x84, 0xA3 }, 100 }

//  Name:     System.Photo.ExposureTime -- PKEY_Photo_ExposureTime
//  Type:     Double -- VT_R8
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 33434
//
//  PropertyTagExifExposureTime.  Calculated from  PKEY_Photo_ExposureTimeNumerator and PKEY_Photo_ExposureTimeDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureTime, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 33434);
#define INIT_PKEY_Photo_ExposureTime { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 33434 }

//  Name:     System.Photo.ExposureTimeDenominator -- PKEY_Photo_ExposureTimeDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {55E98597-AD16-42E0-B624-21599A199838}, 100
//
//  Denominator of PKEY_Photo_ExposureTime
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureTimeDenominator, 0x55E98597, 0xAD16, 0x42E0, 0xB6, 0x24, 0x21, 0x59, 0x9A, 0x19, 0x98, 0x38, 100);
#define INIT_PKEY_Photo_ExposureTimeDenominator { { 0x55E98597, 0xAD16, 0x42E0, 0xB6, 0x24, 0x21, 0x59, 0x9A, 0x19, 0x98, 0x38 }, 100 }

//  Name:     System.Photo.ExposureTimeNumerator -- PKEY_Photo_ExposureTimeNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {257E44E2-9031-4323-AC38-85C552871B2E}, 100
//
//  Numerator of PKEY_Photo_ExposureTime
DEFINE_PROPERTYKEY(PKEY_Photo_ExposureTimeNumerator, 0x257E44E2, 0x9031, 0x4323, 0xAC, 0x38, 0x85, 0xC5, 0x52, 0x87, 0x1B, 0x2E, 100);
#define INIT_PKEY_Photo_ExposureTimeNumerator { { 0x257E44E2, 0x9031, 0x4323, 0xAC, 0x38, 0x85, 0xC5, 0x52, 0x87, 0x1B, 0x2E }, 100 }

//  Name:     System.Photo.Flash -- PKEY_Photo_Flash
//  Type:     Byte -- VT_UI1
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 37385
//
//  PropertyTagExifFlash
DEFINE_PROPERTYKEY(PKEY_Photo_Flash, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 37385);
#define INIT_PKEY_Photo_Flash { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 37385 }

// Possible discrete values for PKEY_Photo_Flash are:
#define PHOTO_FLASH_NONE                    0
#define PHOTO_FLASH_FLASH                   1
#define PHOTO_FLASH_WITHOUTSTROBE           5
#define PHOTO_FLASH_WITHSTROBE              7
#define PHOTO_FLASH_FLASH_COMPULSORY        9
#define PHOTO_FLASH_FLASH_COMPULSORY_NORETURNLIGHT 13
#define PHOTO_FLASH_FLASH_COMPULSORY_RETURNLIGHT 15
#define PHOTO_FLASH_NONE_COMPULSORY         16
#define PHOTO_FLASH_NONE_AUTO               24
#define PHOTO_FLASH_FLASH_AUTO              25
#define PHOTO_FLASH_FLASH_AUTO_NORETURNLIGHT 29
#define PHOTO_FLASH_FLASH_AUTO_RETURNLIGHT  31
#define PHOTO_FLASH_NOFUNCTION              32
#define PHOTO_FLASH_FLASH_REDEYE            65
#define PHOTO_FLASH_FLASH_REDEYE_NORETURNLIGHT 69
#define PHOTO_FLASH_FLASH_REDEYE_RETURNLIGHT 71
#define PHOTO_FLASH_FLASH_COMPULSORY_REDEYE 73
#define PHOTO_FLASH_FLASH_COMPULSORY_REDEYE_NORETURNLIGHT 77
#define PHOTO_FLASH_FLASH_COMPULSORY_REDEYE_RETURNLIGHT 79
#define PHOTO_FLASH_FLASH_AUTO_REDEYE       89
#define PHOTO_FLASH_FLASH_AUTO_REDEYE_NORETURNLIGHT 93
#define PHOTO_FLASH_FLASH_AUTO_REDEYE_RETURNLIGHT 95

//  Name:     System.Photo.FlashEnergy -- PKEY_Photo_FlashEnergy
//  Type:     Double -- VT_R8
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 41483
//
//  PropertyTagExifFlashEnergy.  Calculated from PKEY_Photo_FlashEnergyNumerator and PKEY_Photo_FlashEnergyDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_FlashEnergy, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 41483);
#define INIT_PKEY_Photo_FlashEnergy { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 41483 }

//  Name:     System.Photo.FlashEnergyDenominator -- PKEY_Photo_FlashEnergyDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {D7B61C70-6323-49CD-A5FC-C84277162C97}, 100
//
//  Denominator of PKEY_Photo_FlashEnergy
DEFINE_PROPERTYKEY(PKEY_Photo_FlashEnergyDenominator, 0xD7B61C70, 0x6323, 0x49CD, 0xA5, 0xFC, 0xC8, 0x42, 0x77, 0x16, 0x2C, 0x97, 100);
#define INIT_PKEY_Photo_FlashEnergyDenominator { { 0xD7B61C70, 0x6323, 0x49CD, 0xA5, 0xFC, 0xC8, 0x42, 0x77, 0x16, 0x2C, 0x97 }, 100 }

//  Name:     System.Photo.FlashEnergyNumerator -- PKEY_Photo_FlashEnergyNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {FCAD3D3D-0858-400F-AAA3-2F66CCE2A6BC}, 100
//
//  Numerator of PKEY_Photo_FlashEnergy
DEFINE_PROPERTYKEY(PKEY_Photo_FlashEnergyNumerator, 0xFCAD3D3D, 0x0858, 0x400F, 0xAA, 0xA3, 0x2F, 0x66, 0xCC, 0xE2, 0xA6, 0xBC, 100);
#define INIT_PKEY_Photo_FlashEnergyNumerator { { 0xFCAD3D3D, 0x0858, 0x400F, 0xAA, 0xA3, 0x2F, 0x66, 0xCC, 0xE2, 0xA6, 0xBC }, 100 }

//  Name:     System.Photo.FlashManufacturer -- PKEY_Photo_FlashManufacturer
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {AABAF6C9-E0C5-4719-8585-57B103E584FE}, 100
DEFINE_PROPERTYKEY(PKEY_Photo_FlashManufacturer, 0xAABAF6C9, 0xE0C5, 0x4719, 0x85, 0x85, 0x57, 0xB1, 0x03, 0xE5, 0x84, 0xFE, 100);
#define INIT_PKEY_Photo_FlashManufacturer { { 0xAABAF6C9, 0xE0C5, 0x4719, 0x85, 0x85, 0x57, 0xB1, 0x03, 0xE5, 0x84, 0xFE }, 100 }

//  Name:     System.Photo.FlashModel -- PKEY_Photo_FlashModel
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {FE83BB35-4D1A-42E2-916B-06F3E1AF719E}, 100
DEFINE_PROPERTYKEY(PKEY_Photo_FlashModel, 0xFE83BB35, 0x4D1A, 0x42E2, 0x91, 0x6B, 0x06, 0xF3, 0xE1, 0xAF, 0x71, 0x9E, 100);
#define INIT_PKEY_Photo_FlashModel { { 0xFE83BB35, 0x4D1A, 0x42E2, 0x91, 0x6B, 0x06, 0xF3, 0xE1, 0xAF, 0x71, 0x9E }, 100 }

//  Name:     System.Photo.FlashText -- PKEY_Photo_FlashText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {6B8B68F6-200B-47EA-8D25-D8050F57339F}, 100
//  
//  This is the user-friendly form of System.Photo.Flash.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_FlashText, 0x6B8B68F6, 0x200B, 0x47EA, 0x8D, 0x25, 0xD8, 0x05, 0x0F, 0x57, 0x33, 0x9F, 100);
#define INIT_PKEY_Photo_FlashText { { 0x6B8B68F6, 0x200B, 0x47EA, 0x8D, 0x25, 0xD8, 0x05, 0x0F, 0x57, 0x33, 0x9F }, 100 }

//  Name:     System.Photo.FNumber -- PKEY_Photo_FNumber
//  Type:     Double -- VT_R8
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 33437
//
//  PropertyTagExifFNumber.  Calculated from PKEY_Photo_FNumberNumerator and PKEY_Photo_FNumberDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_FNumber, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 33437);
#define INIT_PKEY_Photo_FNumber { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 33437 }

//  Name:     System.Photo.FNumberDenominator -- PKEY_Photo_FNumberDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {E92A2496-223B-4463-A4E3-30EABBA79D80}, 100
//
//  Denominator of PKEY_Photo_FNumber
DEFINE_PROPERTYKEY(PKEY_Photo_FNumberDenominator, 0xE92A2496, 0x223B, 0x4463, 0xA4, 0xE3, 0x30, 0xEA, 0xBB, 0xA7, 0x9D, 0x80, 100);
#define INIT_PKEY_Photo_FNumberDenominator { { 0xE92A2496, 0x223B, 0x4463, 0xA4, 0xE3, 0x30, 0xEA, 0xBB, 0xA7, 0x9D, 0x80 }, 100 }

//  Name:     System.Photo.FNumberNumerator -- PKEY_Photo_FNumberNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {1B97738A-FDFC-462F-9D93-1957E08BE90C}, 100
//
//  Numerator of PKEY_Photo_FNumber
DEFINE_PROPERTYKEY(PKEY_Photo_FNumberNumerator, 0x1B97738A, 0xFDFC, 0x462F, 0x9D, 0x93, 0x19, 0x57, 0xE0, 0x8B, 0xE9, 0x0C, 100);
#define INIT_PKEY_Photo_FNumberNumerator { { 0x1B97738A, 0xFDFC, 0x462F, 0x9D, 0x93, 0x19, 0x57, 0xE0, 0x8B, 0xE9, 0x0C }, 100 }

//  Name:     System.Photo.FocalLength -- PKEY_Photo_FocalLength
//  Type:     Double -- VT_R8
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 37386
//
//  PropertyTagExifFocalLength.  Calculated from PKEY_Photo_FocalLengthNumerator and PKEY_Photo_FocalLengthDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_FocalLength, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 37386);
#define INIT_PKEY_Photo_FocalLength { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 37386 }

//  Name:     System.Photo.FocalLengthDenominator -- PKEY_Photo_FocalLengthDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {305BC615-DCA1-44A5-9FD4-10C0BA79412E}, 100
//
//  Denominator of PKEY_Photo_FocalLength
DEFINE_PROPERTYKEY(PKEY_Photo_FocalLengthDenominator, 0x305BC615, 0xDCA1, 0x44A5, 0x9F, 0xD4, 0x10, 0xC0, 0xBA, 0x79, 0x41, 0x2E, 100);
#define INIT_PKEY_Photo_FocalLengthDenominator { { 0x305BC615, 0xDCA1, 0x44A5, 0x9F, 0xD4, 0x10, 0xC0, 0xBA, 0x79, 0x41, 0x2E }, 100 }

//  Name:     System.Photo.FocalLengthInFilm -- PKEY_Photo_FocalLengthInFilm
//  Type:     UInt16 -- VT_UI2
//  FormatID: {A0E74609-B84D-4F49-B860-462BD9971F98}, 100
DEFINE_PROPERTYKEY(PKEY_Photo_FocalLengthInFilm, 0xA0E74609, 0xB84D, 0x4F49, 0xB8, 0x60, 0x46, 0x2B, 0xD9, 0x97, 0x1F, 0x98, 100);
#define INIT_PKEY_Photo_FocalLengthInFilm { { 0xA0E74609, 0xB84D, 0x4F49, 0xB8, 0x60, 0x46, 0x2B, 0xD9, 0x97, 0x1F, 0x98 }, 100 }

//  Name:     System.Photo.FocalLengthNumerator -- PKEY_Photo_FocalLengthNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {776B6B3B-1E3D-4B0C-9A0E-8FBAF2A8492A}, 100
//
//  Numerator of PKEY_Photo_FocalLength
DEFINE_PROPERTYKEY(PKEY_Photo_FocalLengthNumerator, 0x776B6B3B, 0x1E3D, 0x4B0C, 0x9A, 0x0E, 0x8F, 0xBA, 0xF2, 0xA8, 0x49, 0x2A, 100);
#define INIT_PKEY_Photo_FocalLengthNumerator { { 0x776B6B3B, 0x1E3D, 0x4B0C, 0x9A, 0x0E, 0x8F, 0xBA, 0xF2, 0xA8, 0x49, 0x2A }, 100 }

//  Name:     System.Photo.FocalPlaneXResolution -- PKEY_Photo_FocalPlaneXResolution
//  Type:     Double -- VT_R8
//  FormatID: {CFC08D97-C6F7-4484-89DD-EBEF4356FE76}, 100
//  
//  PropertyTagExifFocalXRes.  Calculated from PKEY_Photo_FocalPlaneXResolutionNumerator and 
//  PKEY_Photo_FocalPlaneXResolutionDenominator.
DEFINE_PROPERTYKEY(PKEY_Photo_FocalPlaneXResolution, 0xCFC08D97, 0xC6F7, 0x4484, 0x89, 0xDD, 0xEB, 0xEF, 0x43, 0x56, 0xFE, 0x76, 100);
#define INIT_PKEY_Photo_FocalPlaneXResolution { { 0xCFC08D97, 0xC6F7, 0x4484, 0x89, 0xDD, 0xEB, 0xEF, 0x43, 0x56, 0xFE, 0x76 }, 100 }

//  Name:     System.Photo.FocalPlaneXResolutionDenominator -- PKEY_Photo_FocalPlaneXResolutionDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {0933F3F5-4786-4F46-A8E8-D64DD37FA521}, 100
//
//  Denominator of PKEY_Photo_FocalPlaneXResolution
DEFINE_PROPERTYKEY(PKEY_Photo_FocalPlaneXResolutionDenominator, 0x0933F3F5, 0x4786, 0x4F46, 0xA8, 0xE8, 0xD6, 0x4D, 0xD3, 0x7F, 0xA5, 0x21, 100);
#define INIT_PKEY_Photo_FocalPlaneXResolutionDenominator { { 0x0933F3F5, 0x4786, 0x4F46, 0xA8, 0xE8, 0xD6, 0x4D, 0xD3, 0x7F, 0xA5, 0x21 }, 100 }

//  Name:     System.Photo.FocalPlaneXResolutionNumerator -- PKEY_Photo_FocalPlaneXResolutionNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {DCCB10AF-B4E2-4B88-95F9-031B4D5AB490}, 100
//
//  Numerator of PKEY_Photo_FocalPlaneXResolution
DEFINE_PROPERTYKEY(PKEY_Photo_FocalPlaneXResolutionNumerator, 0xDCCB10AF, 0xB4E2, 0x4B88, 0x95, 0xF9, 0x03, 0x1B, 0x4D, 0x5A, 0xB4, 0x90, 100);
#define INIT_PKEY_Photo_FocalPlaneXResolutionNumerator { { 0xDCCB10AF, 0xB4E2, 0x4B88, 0x95, 0xF9, 0x03, 0x1B, 0x4D, 0x5A, 0xB4, 0x90 }, 100 }

//  Name:     System.Photo.FocalPlaneYResolution -- PKEY_Photo_FocalPlaneYResolution
//  Type:     Double -- VT_R8
//  FormatID: {4FFFE4D0-914F-4AC4-8D6F-C9C61DE169B1}, 100
//  
//  PropertyTagExifFocalYRes.  Calculated from PKEY_Photo_FocalPlaneYResolutionNumerator and 
//  PKEY_Photo_FocalPlaneYResolutionDenominator.
DEFINE_PROPERTYKEY(PKEY_Photo_FocalPlaneYResolution, 0x4FFFE4D0, 0x914F, 0x4AC4, 0x8D, 0x6F, 0xC9, 0xC6, 0x1D, 0xE1, 0x69, 0xB1, 100);
#define INIT_PKEY_Photo_FocalPlaneYResolution { { 0x4FFFE4D0, 0x914F, 0x4AC4, 0x8D, 0x6F, 0xC9, 0xC6, 0x1D, 0xE1, 0x69, 0xB1 }, 100 }

//  Name:     System.Photo.FocalPlaneYResolutionDenominator -- PKEY_Photo_FocalPlaneYResolutionDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {1D6179A6-A876-4031-B013-3347B2B64DC8}, 100
//
//  Denominator of PKEY_Photo_FocalPlaneYResolution
DEFINE_PROPERTYKEY(PKEY_Photo_FocalPlaneYResolutionDenominator, 0x1D6179A6, 0xA876, 0x4031, 0xB0, 0x13, 0x33, 0x47, 0xB2, 0xB6, 0x4D, 0xC8, 100);
#define INIT_PKEY_Photo_FocalPlaneYResolutionDenominator { { 0x1D6179A6, 0xA876, 0x4031, 0xB0, 0x13, 0x33, 0x47, 0xB2, 0xB6, 0x4D, 0xC8 }, 100 }

//  Name:     System.Photo.FocalPlaneYResolutionNumerator -- PKEY_Photo_FocalPlaneYResolutionNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {A2E541C5-4440-4BA8-867E-75CFC06828CD}, 100
//
//  Numerator of PKEY_Photo_FocalPlaneYResolution
DEFINE_PROPERTYKEY(PKEY_Photo_FocalPlaneYResolutionNumerator, 0xA2E541C5, 0x4440, 0x4BA8, 0x86, 0x7E, 0x75, 0xCF, 0xC0, 0x68, 0x28, 0xCD, 100);
#define INIT_PKEY_Photo_FocalPlaneYResolutionNumerator { { 0xA2E541C5, 0x4440, 0x4BA8, 0x86, 0x7E, 0x75, 0xCF, 0xC0, 0x68, 0x28, 0xCD }, 100 }

//  Name:     System.Photo.GainControl -- PKEY_Photo_GainControl
//  Type:     Double -- VT_R8
//  FormatID: {FA304789-00C7-4D80-904A-1E4DCC7265AA}, 100 (PropertyTagExifGainControl)
//  
//  This indicates the degree of overall image gain adjustment.
//  
//  Calculated from PKEY_Photo_GainControlNumerator and PKEY_Photo_GainControlDenominator.
DEFINE_PROPERTYKEY(PKEY_Photo_GainControl, 0xFA304789, 0x00C7, 0x4D80, 0x90, 0x4A, 0x1E, 0x4D, 0xCC, 0x72, 0x65, 0xAA, 100);
#define INIT_PKEY_Photo_GainControl { { 0xFA304789, 0x00C7, 0x4D80, 0x90, 0x4A, 0x1E, 0x4D, 0xCC, 0x72, 0x65, 0xAA }, 100 }

// Possible discrete values for PKEY_Photo_GainControl are:
#define PHOTO_GAINCONTROL_NONE              0.0
#define PHOTO_GAINCONTROL_LOWGAINUP         1.0
#define PHOTO_GAINCONTROL_HIGHGAINUP        2.0
#define PHOTO_GAINCONTROL_LOWGAINDOWN       3.0
#define PHOTO_GAINCONTROL_HIGHGAINDOWN      4.0

//  Name:     System.Photo.GainControlDenominator -- PKEY_Photo_GainControlDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {42864DFD-9DA4-4F77-BDED-4AAD7B256735}, 100
//
//  Denominator of PKEY_Photo_GainControl
DEFINE_PROPERTYKEY(PKEY_Photo_GainControlDenominator, 0x42864DFD, 0x9DA4, 0x4F77, 0xBD, 0xED, 0x4A, 0xAD, 0x7B, 0x25, 0x67, 0x35, 100);
#define INIT_PKEY_Photo_GainControlDenominator { { 0x42864DFD, 0x9DA4, 0x4F77, 0xBD, 0xED, 0x4A, 0xAD, 0x7B, 0x25, 0x67, 0x35 }, 100 }

//  Name:     System.Photo.GainControlNumerator -- PKEY_Photo_GainControlNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {8E8ECF7C-B7B8-4EB8-A63F-0EE715C96F9E}, 100
//
//  Numerator of PKEY_Photo_GainControl
DEFINE_PROPERTYKEY(PKEY_Photo_GainControlNumerator, 0x8E8ECF7C, 0xB7B8, 0x4EB8, 0xA6, 0x3F, 0x0E, 0xE7, 0x15, 0xC9, 0x6F, 0x9E, 100);
#define INIT_PKEY_Photo_GainControlNumerator { { 0x8E8ECF7C, 0xB7B8, 0x4EB8, 0xA6, 0x3F, 0x0E, 0xE7, 0x15, 0xC9, 0x6F, 0x9E }, 100 }

//  Name:     System.Photo.GainControlText -- PKEY_Photo_GainControlText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C06238B2-0BF9-4279-A723-25856715CB9D}, 100
//  
//  This is the user-friendly form of System.Photo.GainControl.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_GainControlText, 0xC06238B2, 0x0BF9, 0x4279, 0xA7, 0x23, 0x25, 0x85, 0x67, 0x15, 0xCB, 0x9D, 100);
#define INIT_PKEY_Photo_GainControlText { { 0xC06238B2, 0x0BF9, 0x4279, 0xA7, 0x23, 0x25, 0x85, 0x67, 0x15, 0xCB, 0x9D }, 100 }

//  Name:     System.Photo.ISOSpeed -- PKEY_Photo_ISOSpeed
//  Type:     UInt16 -- VT_UI2
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 34855
//
//  PropertyTagExifISOSpeed
DEFINE_PROPERTYKEY(PKEY_Photo_ISOSpeed, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 34855);
#define INIT_PKEY_Photo_ISOSpeed { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 34855 }

//  Name:     System.Photo.LensManufacturer -- PKEY_Photo_LensManufacturer
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E6DDCAF7-29C5-4F0A-9A68-D19412EC7090}, 100
DEFINE_PROPERTYKEY(PKEY_Photo_LensManufacturer, 0xE6DDCAF7, 0x29C5, 0x4F0A, 0x9A, 0x68, 0xD1, 0x94, 0x12, 0xEC, 0x70, 0x90, 100);
#define INIT_PKEY_Photo_LensManufacturer { { 0xE6DDCAF7, 0x29C5, 0x4F0A, 0x9A, 0x68, 0xD1, 0x94, 0x12, 0xEC, 0x70, 0x90 }, 100 }

//  Name:     System.Photo.LensModel -- PKEY_Photo_LensModel
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {E1277516-2B5F-4869-89B1-2E585BD38B7A}, 100
DEFINE_PROPERTYKEY(PKEY_Photo_LensModel, 0xE1277516, 0x2B5F, 0x4869, 0x89, 0xB1, 0x2E, 0x58, 0x5B, 0xD3, 0x8B, 0x7A, 100);
#define INIT_PKEY_Photo_LensModel { { 0xE1277516, 0x2B5F, 0x4869, 0x89, 0xB1, 0x2E, 0x58, 0x5B, 0xD3, 0x8B, 0x7A }, 100 }

//  Name:     System.Photo.LightSource -- PKEY_Photo_LightSource
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 37384
//
//  PropertyTagExifLightSource
DEFINE_PROPERTYKEY(PKEY_Photo_LightSource, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 37384);
#define INIT_PKEY_Photo_LightSource { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 37384 }

// Possible discrete values for PKEY_Photo_LightSource are:
#define PHOTO_LIGHTSOURCE_UNKNOWN           0ul
#define PHOTO_LIGHTSOURCE_DAYLIGHT          1ul
#define PHOTO_LIGHTSOURCE_FLUORESCENT       2ul
#define PHOTO_LIGHTSOURCE_TUNGSTEN          3ul
#define PHOTO_LIGHTSOURCE_STANDARD_A        17ul
#define PHOTO_LIGHTSOURCE_STANDARD_B        18ul
#define PHOTO_LIGHTSOURCE_STANDARD_C        19ul
#define PHOTO_LIGHTSOURCE_D55               20ul
#define PHOTO_LIGHTSOURCE_D65               21ul
#define PHOTO_LIGHTSOURCE_D75               22ul

//  Name:     System.Photo.MakerNote -- PKEY_Photo_MakerNote
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: {FA303353-B659-4052-85E9-BCAC79549B84}, 100
DEFINE_PROPERTYKEY(PKEY_Photo_MakerNote, 0xFA303353, 0xB659, 0x4052, 0x85, 0xE9, 0xBC, 0xAC, 0x79, 0x54, 0x9B, 0x84, 100);
#define INIT_PKEY_Photo_MakerNote { { 0xFA303353, 0xB659, 0x4052, 0x85, 0xE9, 0xBC, 0xAC, 0x79, 0x54, 0x9B, 0x84 }, 100 }

//  Name:     System.Photo.MakerNoteOffset -- PKEY_Photo_MakerNoteOffset
//  Type:     UInt64 -- VT_UI8
//  FormatID: {813F4124-34E6-4D17-AB3E-6B1F3C2247A1}, 100
DEFINE_PROPERTYKEY(PKEY_Photo_MakerNoteOffset, 0x813F4124, 0x34E6, 0x4D17, 0xAB, 0x3E, 0x6B, 0x1F, 0x3C, 0x22, 0x47, 0xA1, 100);
#define INIT_PKEY_Photo_MakerNoteOffset { { 0x813F4124, 0x34E6, 0x4D17, 0xAB, 0x3E, 0x6B, 0x1F, 0x3C, 0x22, 0x47, 0xA1 }, 100 }

//  Name:     System.Photo.MaxAperture -- PKEY_Photo_MaxAperture
//  Type:     Double -- VT_R8
//  FormatID: {08F6D7C2-E3F2-44FC-AF1E-5AA5C81A2D3E}, 100
//
//  Calculated from PKEY_Photo_MaxApertureNumerator and PKEY_Photo_MaxApertureDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_MaxAperture, 0x08F6D7C2, 0xE3F2, 0x44FC, 0xAF, 0x1E, 0x5A, 0xA5, 0xC8, 0x1A, 0x2D, 0x3E, 100);
#define INIT_PKEY_Photo_MaxAperture { { 0x08F6D7C2, 0xE3F2, 0x44FC, 0xAF, 0x1E, 0x5A, 0xA5, 0xC8, 0x1A, 0x2D, 0x3E }, 100 }

//  Name:     System.Photo.MaxApertureDenominator -- PKEY_Photo_MaxApertureDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {C77724D4-601F-46C5-9B89-C53F93BCEB77}, 100
//
//  Denominator of PKEY_Photo_MaxAperture
DEFINE_PROPERTYKEY(PKEY_Photo_MaxApertureDenominator, 0xC77724D4, 0x601F, 0x46C5, 0x9B, 0x89, 0xC5, 0x3F, 0x93, 0xBC, 0xEB, 0x77, 100);
#define INIT_PKEY_Photo_MaxApertureDenominator { { 0xC77724D4, 0x601F, 0x46C5, 0x9B, 0x89, 0xC5, 0x3F, 0x93, 0xBC, 0xEB, 0x77 }, 100 }

//  Name:     System.Photo.MaxApertureNumerator -- PKEY_Photo_MaxApertureNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {C107E191-A459-44C5-9AE6-B952AD4B906D}, 100
//
//  Numerator of PKEY_Photo_MaxAperture
DEFINE_PROPERTYKEY(PKEY_Photo_MaxApertureNumerator, 0xC107E191, 0xA459, 0x44C5, 0x9A, 0xE6, 0xB9, 0x52, 0xAD, 0x4B, 0x90, 0x6D, 100);
#define INIT_PKEY_Photo_MaxApertureNumerator { { 0xC107E191, 0xA459, 0x44C5, 0x9A, 0xE6, 0xB9, 0x52, 0xAD, 0x4B, 0x90, 0x6D }, 100 }

//  Name:     System.Photo.MeteringMode -- PKEY_Photo_MeteringMode
//  Type:     UInt16 -- VT_UI2
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 37383
//
//  PropertyTagExifMeteringMode
DEFINE_PROPERTYKEY(PKEY_Photo_MeteringMode, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 37383);
#define INIT_PKEY_Photo_MeteringMode { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 37383 }

// Possible discrete values for PKEY_Photo_MeteringMode are:
#define PHOTO_METERINGMODE_UNKNOWN          0u
#define PHOTO_METERINGMODE_AVERAGE          1u
#define PHOTO_METERINGMODE_CENTER           2u
#define PHOTO_METERINGMODE_SPOT             3u
#define PHOTO_METERINGMODE_MULTISPOT        4u
#define PHOTO_METERINGMODE_PATTERN          5u
#define PHOTO_METERINGMODE_PARTIAL          6u

//  Name:     System.Photo.MeteringModeText -- PKEY_Photo_MeteringModeText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {F628FD8C-7BA8-465A-A65B-C5AA79263A9E}, 100
//  
//  This is the user-friendly form of System.Photo.MeteringMode.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_MeteringModeText, 0xF628FD8C, 0x7BA8, 0x465A, 0xA6, 0x5B, 0xC5, 0xAA, 0x79, 0x26, 0x3A, 0x9E, 100);
#define INIT_PKEY_Photo_MeteringModeText { { 0xF628FD8C, 0x7BA8, 0x465A, 0xA6, 0x5B, 0xC5, 0xAA, 0x79, 0x26, 0x3A, 0x9E }, 100 }

//  Name:     System.Photo.Orientation -- PKEY_Photo_Orientation
//  Type:     UInt16 -- VT_UI2
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 274 (PropertyTagOrientation)
//  
//  This is the image orientation viewed in terms of rows and columns.
DEFINE_PROPERTYKEY(PKEY_Photo_Orientation, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 274);
#define INIT_PKEY_Photo_Orientation { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 274 }

// Possible discrete values for PKEY_Photo_Orientation are:
#define PHOTO_ORIENTATION_NORMAL            1u
#define PHOTO_ORIENTATION_FLIPHORIZONTAL    2u
#define PHOTO_ORIENTATION_ROTATE180         3u
#define PHOTO_ORIENTATION_FLIPVERTICAL      4u
#define PHOTO_ORIENTATION_TRANSPOSE         5u
#define PHOTO_ORIENTATION_ROTATE270         6u
#define PHOTO_ORIENTATION_TRANSVERSE        7u
#define PHOTO_ORIENTATION_ROTATE90          8u

//  Name:     System.Photo.OrientationText -- PKEY_Photo_OrientationText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A9EA193C-C511-498A-A06B-58E2776DCC28}, 100
//  
//  This is the user-friendly form of System.Photo.Orientation.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_OrientationText, 0xA9EA193C, 0xC511, 0x498A, 0xA0, 0x6B, 0x58, 0xE2, 0x77, 0x6D, 0xCC, 0x28, 100);
#define INIT_PKEY_Photo_OrientationText { { 0xA9EA193C, 0xC511, 0x498A, 0xA0, 0x6B, 0x58, 0xE2, 0x77, 0x6D, 0xCC, 0x28 }, 100 }

//  Name:     System.Photo.PeopleNames -- PKEY_Photo_PeopleNames
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)  Legacy code may treat this as VT_LPSTR.
//  FormatID: {E8309B6E-084C-49B4-B1FC-90A80331B638}, 100
//
//  The people tags on an image.
DEFINE_PROPERTYKEY(PKEY_Photo_PeopleNames, 0xE8309B6E, 0x084C, 0x49B4, 0xB1, 0xFC, 0x90, 0xA8, 0x03, 0x31, 0xB6, 0x38, 100);
#define INIT_PKEY_Photo_PeopleNames { { 0xE8309B6E, 0x084C, 0x49B4, 0xB1, 0xFC, 0x90, 0xA8, 0x03, 0x31, 0xB6, 0x38 }, 100 }

//  Name:     System.Photo.PhotometricInterpretation -- PKEY_Photo_PhotometricInterpretation
//  Type:     UInt16 -- VT_UI2
//  FormatID: {341796F1-1DF9-4B1C-A564-91BDEFA43877}, 100
//  
//  This is the pixel composition. In JPEG compressed data, a JPEG marker is used 
//  instead of this property.
DEFINE_PROPERTYKEY(PKEY_Photo_PhotometricInterpretation, 0x341796F1, 0x1DF9, 0x4B1C, 0xA5, 0x64, 0x91, 0xBD, 0xEF, 0xA4, 0x38, 0x77, 100);
#define INIT_PKEY_Photo_PhotometricInterpretation { { 0x341796F1, 0x1DF9, 0x4B1C, 0xA5, 0x64, 0x91, 0xBD, 0xEF, 0xA4, 0x38, 0x77 }, 100 }

// Possible discrete values for PKEY_Photo_PhotometricInterpretation are:
#define PHOTO_PHOTOMETRIC_RGB               2u
#define PHOTO_PHOTOMETRIC_YCBCR             6u

//  Name:     System.Photo.PhotometricInterpretationText -- PKEY_Photo_PhotometricInterpretationText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {821437D6-9EAB-4765-A589-3B1CBBD22A61}, 100
//  
//  This is the user-friendly form of System.Photo.PhotometricInterpretation.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_PhotometricInterpretationText, 0x821437D6, 0x9EAB, 0x4765, 0xA5, 0x89, 0x3B, 0x1C, 0xBB, 0xD2, 0x2A, 0x61, 100);
#define INIT_PKEY_Photo_PhotometricInterpretationText { { 0x821437D6, 0x9EAB, 0x4765, 0xA5, 0x89, 0x3B, 0x1C, 0xBB, 0xD2, 0x2A, 0x61 }, 100 }

//  Name:     System.Photo.ProgramMode -- PKEY_Photo_ProgramMode
//  Type:     UInt32 -- VT_UI4
//  FormatID: {6D217F6D-3F6A-4825-B470-5F03CA2FBE9B}, 100
//  
//  This is the class of the program used by the camera to set exposure when the 
//  picture is taken.
DEFINE_PROPERTYKEY(PKEY_Photo_ProgramMode, 0x6D217F6D, 0x3F6A, 0x4825, 0xB4, 0x70, 0x5F, 0x03, 0xCA, 0x2F, 0xBE, 0x9B, 100);
#define INIT_PKEY_Photo_ProgramMode { { 0x6D217F6D, 0x3F6A, 0x4825, 0xB4, 0x70, 0x5F, 0x03, 0xCA, 0x2F, 0xBE, 0x9B }, 100 }

// Possible discrete values for PKEY_Photo_ProgramMode are:
#define PHOTO_PROGRAMMODE_NOTDEFINED        0ul
#define PHOTO_PROGRAMMODE_MANUAL            1ul
#define PHOTO_PROGRAMMODE_NORMAL            2ul
#define PHOTO_PROGRAMMODE_APERTURE          3ul
#define PHOTO_PROGRAMMODE_SHUTTER           4ul
#define PHOTO_PROGRAMMODE_CREATIVE          5ul
#define PHOTO_PROGRAMMODE_ACTION            6ul
#define PHOTO_PROGRAMMODE_PORTRAIT          7ul
#define PHOTO_PROGRAMMODE_LANDSCAPE         8ul

//  Name:     System.Photo.ProgramModeText -- PKEY_Photo_ProgramModeText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {7FE3AA27-2648-42F3-89B0-454E5CB150C3}, 100
//  
//  This is the user-friendly form of System.Photo.ProgramMode.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_ProgramModeText, 0x7FE3AA27, 0x2648, 0x42F3, 0x89, 0xB0, 0x45, 0x4E, 0x5C, 0xB1, 0x50, 0xC3, 100);
#define INIT_PKEY_Photo_ProgramModeText { { 0x7FE3AA27, 0x2648, 0x42F3, 0x89, 0xB0, 0x45, 0x4E, 0x5C, 0xB1, 0x50, 0xC3 }, 100 }

//  Name:     System.Photo.RelatedSoundFile -- PKEY_Photo_RelatedSoundFile
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {318A6B45-087F-4DC2-B8CC-05359551FC9E}, 100
DEFINE_PROPERTYKEY(PKEY_Photo_RelatedSoundFile, 0x318A6B45, 0x087F, 0x4DC2, 0xB8, 0xCC, 0x05, 0x35, 0x95, 0x51, 0xFC, 0x9E, 100);
#define INIT_PKEY_Photo_RelatedSoundFile { { 0x318A6B45, 0x087F, 0x4DC2, 0xB8, 0xCC, 0x05, 0x35, 0x95, 0x51, 0xFC, 0x9E }, 100 }

//  Name:     System.Photo.Saturation -- PKEY_Photo_Saturation
//  Type:     UInt32 -- VT_UI4
//  FormatID: {49237325-A95A-4F67-B211-816B2D45D2E0}, 100
//  
//  This indicates the direction of saturation processing applied by the camera when 
//  the image was shot.
DEFINE_PROPERTYKEY(PKEY_Photo_Saturation, 0x49237325, 0xA95A, 0x4F67, 0xB2, 0x11, 0x81, 0x6B, 0x2D, 0x45, 0xD2, 0xE0, 100);
#define INIT_PKEY_Photo_Saturation { { 0x49237325, 0xA95A, 0x4F67, 0xB2, 0x11, 0x81, 0x6B, 0x2D, 0x45, 0xD2, 0xE0 }, 100 }

// Possible discrete values for PKEY_Photo_Saturation are:
#define PHOTO_SATURATION_NORMAL             0ul
#define PHOTO_SATURATION_LOW                1ul
#define PHOTO_SATURATION_HIGH               2ul

//  Name:     System.Photo.SaturationText -- PKEY_Photo_SaturationText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {61478C08-B600-4A84-BBE4-E99C45F0A072}, 100
//  
//  This is the user-friendly form of System.Photo.Saturation.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_SaturationText, 0x61478C08, 0xB600, 0x4A84, 0xBB, 0xE4, 0xE9, 0x9C, 0x45, 0xF0, 0xA0, 0x72, 100);
#define INIT_PKEY_Photo_SaturationText { { 0x61478C08, 0xB600, 0x4A84, 0xBB, 0xE4, 0xE9, 0x9C, 0x45, 0xF0, 0xA0, 0x72 }, 100 }

//  Name:     System.Photo.Sharpness -- PKEY_Photo_Sharpness
//  Type:     UInt32 -- VT_UI4
//  FormatID: {FC6976DB-8349-4970-AE97-B3C5316A08F0}, 100
//  
//  This indicates the direction of sharpness processing applied by the camera when 
//  the image was shot.
DEFINE_PROPERTYKEY(PKEY_Photo_Sharpness, 0xFC6976DB, 0x8349, 0x4970, 0xAE, 0x97, 0xB3, 0xC5, 0x31, 0x6A, 0x08, 0xF0, 100);
#define INIT_PKEY_Photo_Sharpness { { 0xFC6976DB, 0x8349, 0x4970, 0xAE, 0x97, 0xB3, 0xC5, 0x31, 0x6A, 0x08, 0xF0 }, 100 }

// Possible discrete values for PKEY_Photo_Sharpness are:
#define PHOTO_SHARPNESS_NORMAL              0ul
#define PHOTO_SHARPNESS_SOFT                1ul
#define PHOTO_SHARPNESS_HARD                2ul

//  Name:     System.Photo.SharpnessText -- PKEY_Photo_SharpnessText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {51EC3F47-DD50-421D-8769-334F50424B1E}, 100
//  
//  This is the user-friendly form of System.Photo.Sharpness.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_SharpnessText, 0x51EC3F47, 0xDD50, 0x421D, 0x87, 0x69, 0x33, 0x4F, 0x50, 0x42, 0x4B, 0x1E, 100);
#define INIT_PKEY_Photo_SharpnessText { { 0x51EC3F47, 0xDD50, 0x421D, 0x87, 0x69, 0x33, 0x4F, 0x50, 0x42, 0x4B, 0x1E }, 100 }

//  Name:     System.Photo.ShutterSpeed -- PKEY_Photo_ShutterSpeed
//  Type:     Double -- VT_R8
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 37377
//
//  PropertyTagExifShutterSpeed.  Calculated from PKEY_Photo_ShutterSpeedNumerator and PKEY_Photo_ShutterSpeedDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_ShutterSpeed, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 37377);
#define INIT_PKEY_Photo_ShutterSpeed { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 37377 }

//  Name:     System.Photo.ShutterSpeedDenominator -- PKEY_Photo_ShutterSpeedDenominator
//  Type:     Int32 -- VT_I4
//  FormatID: {E13D8975-81C7-4948-AE3F-37CAE11E8FF7}, 100
//
//  Denominator of PKEY_Photo_ShutterSpeed
DEFINE_PROPERTYKEY(PKEY_Photo_ShutterSpeedDenominator, 0xE13D8975, 0x81C7, 0x4948, 0xAE, 0x3F, 0x37, 0xCA, 0xE1, 0x1E, 0x8F, 0xF7, 100);
#define INIT_PKEY_Photo_ShutterSpeedDenominator { { 0xE13D8975, 0x81C7, 0x4948, 0xAE, 0x3F, 0x37, 0xCA, 0xE1, 0x1E, 0x8F, 0xF7 }, 100 }

//  Name:     System.Photo.ShutterSpeedNumerator -- PKEY_Photo_ShutterSpeedNumerator
//  Type:     Int32 -- VT_I4
//  FormatID: {16EA4042-D6F4-4BCA-8349-7C78D30FB333}, 100
//
//  Numerator of PKEY_Photo_ShutterSpeed
DEFINE_PROPERTYKEY(PKEY_Photo_ShutterSpeedNumerator, 0x16EA4042, 0xD6F4, 0x4BCA, 0x83, 0x49, 0x7C, 0x78, 0xD3, 0x0F, 0xB3, 0x33, 100);
#define INIT_PKEY_Photo_ShutterSpeedNumerator { { 0x16EA4042, 0xD6F4, 0x4BCA, 0x83, 0x49, 0x7C, 0x78, 0xD3, 0x0F, 0xB3, 0x33 }, 100 }

//  Name:     System.Photo.SubjectDistance -- PKEY_Photo_SubjectDistance
//  Type:     Double -- VT_R8
//  FormatID: (FMTID_ImageProperties) {14B81DA1-0135-4D31-96D9-6CBFC9671A99}, 37382
//
//  PropertyTagExifSubjectDist.  Calculated from PKEY_Photo_SubjectDistanceNumerator and PKEY_Photo_SubjectDistanceDenominator
DEFINE_PROPERTYKEY(PKEY_Photo_SubjectDistance, 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99, 37382);
#define INIT_PKEY_Photo_SubjectDistance { { 0x14B81DA1, 0x0135, 0x4D31, 0x96, 0xD9, 0x6C, 0xBF, 0xC9, 0x67, 0x1A, 0x99 }, 37382 }

//  Name:     System.Photo.SubjectDistanceDenominator -- PKEY_Photo_SubjectDistanceDenominator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {0C840A88-B043-466D-9766-D4B26DA3FA77}, 100
//
//  Denominator of PKEY_Photo_SubjectDistance
DEFINE_PROPERTYKEY(PKEY_Photo_SubjectDistanceDenominator, 0x0C840A88, 0xB043, 0x466D, 0x97, 0x66, 0xD4, 0xB2, 0x6D, 0xA3, 0xFA, 0x77, 100);
#define INIT_PKEY_Photo_SubjectDistanceDenominator { { 0x0C840A88, 0xB043, 0x466D, 0x97, 0x66, 0xD4, 0xB2, 0x6D, 0xA3, 0xFA, 0x77 }, 100 }

//  Name:     System.Photo.SubjectDistanceNumerator -- PKEY_Photo_SubjectDistanceNumerator
//  Type:     UInt32 -- VT_UI4
//  FormatID: {8AF4961C-F526-43E5-AA81-DB768219178D}, 100
//
//  Numerator of PKEY_Photo_SubjectDistance
DEFINE_PROPERTYKEY(PKEY_Photo_SubjectDistanceNumerator, 0x8AF4961C, 0xF526, 0x43E5, 0xAA, 0x81, 0xDB, 0x76, 0x82, 0x19, 0x17, 0x8D, 100);
#define INIT_PKEY_Photo_SubjectDistanceNumerator { { 0x8AF4961C, 0xF526, 0x43E5, 0xAA, 0x81, 0xDB, 0x76, 0x82, 0x19, 0x17, 0x8D }, 100 }

//  Name:     System.Photo.TagViewAggregate -- PKEY_Photo_TagViewAggregate
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)  Legacy code may treat this as VT_LPSTR.
//  FormatID: {B812F15D-C2D8-4BBF-BACD-79744346113F}, 100
//
//  A read-only aggregation of tag-like properties for use in building views.
DEFINE_PROPERTYKEY(PKEY_Photo_TagViewAggregate, 0xB812F15D, 0xC2D8, 0x4BBF, 0xBA, 0xCD, 0x79, 0x74, 0x43, 0x46, 0x11, 0x3F, 100);
#define INIT_PKEY_Photo_TagViewAggregate { { 0xB812F15D, 0xC2D8, 0x4BBF, 0xBA, 0xCD, 0x79, 0x74, 0x43, 0x46, 0x11, 0x3F }, 100 }

//  Name:     System.Photo.TranscodedForSync -- PKEY_Photo_TranscodedForSync
//  Type:     Boolean -- VT_BOOL
//  FormatID: {9A8EBB75-6458-4E82-BACB-35C0095B03BB}, 100
DEFINE_PROPERTYKEY(PKEY_Photo_TranscodedForSync, 0x9A8EBB75, 0x6458, 0x4E82, 0xBA, 0xCB, 0x35, 0xC0, 0x09, 0x5B, 0x03, 0xBB, 100);
#define INIT_PKEY_Photo_TranscodedForSync { { 0x9A8EBB75, 0x6458, 0x4E82, 0xBA, 0xCB, 0x35, 0xC0, 0x09, 0x5B, 0x03, 0xBB }, 100 }

//  Name:     System.Photo.WhiteBalance -- PKEY_Photo_WhiteBalance
//  Type:     UInt32 -- VT_UI4
//  FormatID: {EE3D3D8A-5381-4CFA-B13B-AAF66B5F4EC9}, 100
//  
//  This indicates the white balance mode set when the image was shot.
DEFINE_PROPERTYKEY(PKEY_Photo_WhiteBalance, 0xEE3D3D8A, 0x5381, 0x4CFA, 0xB1, 0x3B, 0xAA, 0xF6, 0x6B, 0x5F, 0x4E, 0xC9, 100);
#define INIT_PKEY_Photo_WhiteBalance { { 0xEE3D3D8A, 0x5381, 0x4CFA, 0xB1, 0x3B, 0xAA, 0xF6, 0x6B, 0x5F, 0x4E, 0xC9 }, 100 }

// Possible discrete values for PKEY_Photo_WhiteBalance are:
#define PHOTO_WHITEBALANCE_AUTO             0ul
#define PHOTO_WHITEBALANCE_MANUAL           1ul

//  Name:     System.Photo.WhiteBalanceText -- PKEY_Photo_WhiteBalanceText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {6336B95E-C7A7-426D-86FD-7AE3D39C84B4}, 100
//  
//  This is the user-friendly form of System.Photo.WhiteBalance.  Not intended to be parsed 
//  programmatically.
DEFINE_PROPERTYKEY(PKEY_Photo_WhiteBalanceText, 0x6336B95E, 0xC7A7, 0x426D, 0x86, 0xFD, 0x7A, 0xE3, 0xD3, 0x9C, 0x84, 0xB4, 100);
#define INIT_PKEY_Photo_WhiteBalanceText { { 0x6336B95E, 0xC7A7, 0x426D, 0x86, 0xFD, 0x7A, 0xE3, 0xD3, 0x9C, 0x84, 0xB4 }, 100 }

//-----------------------------------------------------------------------------
// PropGroup properties

//  Name:     System.PropGroup.Advanced -- PKEY_PropGroup_Advanced
//  Type:     Null -- VT_NULL
//  FormatID: {900A403B-097B-4B95-8AE2-071FDAEEB118}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Advanced, 0x900A403B, 0x097B, 0x4B95, 0x8A, 0xE2, 0x07, 0x1F, 0xDA, 0xEE, 0xB1, 0x18, 100);
#define INIT_PKEY_PropGroup_Advanced { { 0x900A403B, 0x097B, 0x4B95, 0x8A, 0xE2, 0x07, 0x1F, 0xDA, 0xEE, 0xB1, 0x18 }, 100 }

//  Name:     System.PropGroup.Audio -- PKEY_PropGroup_Audio
//  Type:     Null -- VT_NULL
//  FormatID: {2804D469-788F-48AA-8570-71B9C187E138}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Audio, 0x2804D469, 0x788F, 0x48AA, 0x85, 0x70, 0x71, 0xB9, 0xC1, 0x87, 0xE1, 0x38, 100);
#define INIT_PKEY_PropGroup_Audio { { 0x2804D469, 0x788F, 0x48AA, 0x85, 0x70, 0x71, 0xB9, 0xC1, 0x87, 0xE1, 0x38 }, 100 }

//  Name:     System.PropGroup.Calendar -- PKEY_PropGroup_Calendar
//  Type:     Null -- VT_NULL
//  FormatID: {9973D2B5-BFD8-438A-BA94-5349B293181A}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Calendar, 0x9973D2B5, 0xBFD8, 0x438A, 0xBA, 0x94, 0x53, 0x49, 0xB2, 0x93, 0x18, 0x1A, 100);
#define INIT_PKEY_PropGroup_Calendar { { 0x9973D2B5, 0xBFD8, 0x438A, 0xBA, 0x94, 0x53, 0x49, 0xB2, 0x93, 0x18, 0x1A }, 100 }

//  Name:     System.PropGroup.Camera -- PKEY_PropGroup_Camera
//  Type:     Null -- VT_NULL
//  FormatID: {DE00DE32-547E-4981-AD4B-542F2E9007D8}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Camera, 0xDE00DE32, 0x547E, 0x4981, 0xAD, 0x4B, 0x54, 0x2F, 0x2E, 0x90, 0x07, 0xD8, 100);
#define INIT_PKEY_PropGroup_Camera { { 0xDE00DE32, 0x547E, 0x4981, 0xAD, 0x4B, 0x54, 0x2F, 0x2E, 0x90, 0x07, 0xD8 }, 100 }

//  Name:     System.PropGroup.Contact -- PKEY_PropGroup_Contact
//  Type:     Null -- VT_NULL
//  FormatID: {DF975FD3-250A-4004-858F-34E29A3E37AA}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Contact, 0xDF975FD3, 0x250A, 0x4004, 0x85, 0x8F, 0x34, 0xE2, 0x9A, 0x3E, 0x37, 0xAA, 100);
#define INIT_PKEY_PropGroup_Contact { { 0xDF975FD3, 0x250A, 0x4004, 0x85, 0x8F, 0x34, 0xE2, 0x9A, 0x3E, 0x37, 0xAA }, 100 }

//  Name:     System.PropGroup.Content -- PKEY_PropGroup_Content
//  Type:     Null -- VT_NULL
//  FormatID: {D0DAB0BA-368A-4050-A882-6C010FD19A4F}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Content, 0xD0DAB0BA, 0x368A, 0x4050, 0xA8, 0x82, 0x6C, 0x01, 0x0F, 0xD1, 0x9A, 0x4F, 100);
#define INIT_PKEY_PropGroup_Content { { 0xD0DAB0BA, 0x368A, 0x4050, 0xA8, 0x82, 0x6C, 0x01, 0x0F, 0xD1, 0x9A, 0x4F }, 100 }

//  Name:     System.PropGroup.Description -- PKEY_PropGroup_Description
//  Type:     Null -- VT_NULL
//  FormatID: {8969B275-9475-4E00-A887-FF93B8B41E44}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Description, 0x8969B275, 0x9475, 0x4E00, 0xA8, 0x87, 0xFF, 0x93, 0xB8, 0xB4, 0x1E, 0x44, 100);
#define INIT_PKEY_PropGroup_Description { { 0x8969B275, 0x9475, 0x4E00, 0xA8, 0x87, 0xFF, 0x93, 0xB8, 0xB4, 0x1E, 0x44 }, 100 }

//  Name:     System.PropGroup.FileSystem -- PKEY_PropGroup_FileSystem
//  Type:     Null -- VT_NULL
//  FormatID: {E3A7D2C1-80FC-4B40-8F34-30EA111BDC2E}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_FileSystem, 0xE3A7D2C1, 0x80FC, 0x4B40, 0x8F, 0x34, 0x30, 0xEA, 0x11, 0x1B, 0xDC, 0x2E, 100);
#define INIT_PKEY_PropGroup_FileSystem { { 0xE3A7D2C1, 0x80FC, 0x4B40, 0x8F, 0x34, 0x30, 0xEA, 0x11, 0x1B, 0xDC, 0x2E }, 100 }

//  Name:     System.PropGroup.General -- PKEY_PropGroup_General
//  Type:     Null -- VT_NULL
//  FormatID: {CC301630-B192-4C22-B372-9F4C6D338E07}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_General, 0xCC301630, 0xB192, 0x4C22, 0xB3, 0x72, 0x9F, 0x4C, 0x6D, 0x33, 0x8E, 0x07, 100);
#define INIT_PKEY_PropGroup_General { { 0xCC301630, 0xB192, 0x4C22, 0xB3, 0x72, 0x9F, 0x4C, 0x6D, 0x33, 0x8E, 0x07 }, 100 }

//  Name:     System.PropGroup.GPS -- PKEY_PropGroup_GPS
//  Type:     Null -- VT_NULL
//  FormatID: {F3713ADA-90E3-4E11-AAE5-FDC17685B9BE}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_GPS, 0xF3713ADA, 0x90E3, 0x4E11, 0xAA, 0xE5, 0xFD, 0xC1, 0x76, 0x85, 0xB9, 0xBE, 100);
#define INIT_PKEY_PropGroup_GPS { { 0xF3713ADA, 0x90E3, 0x4E11, 0xAA, 0xE5, 0xFD, 0xC1, 0x76, 0x85, 0xB9, 0xBE }, 100 }

//  Name:     System.PropGroup.Image -- PKEY_PropGroup_Image
//  Type:     Null -- VT_NULL
//  FormatID: {E3690A87-0FA8-4A2A-9A9F-FCE8827055AC}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Image, 0xE3690A87, 0x0FA8, 0x4A2A, 0x9A, 0x9F, 0xFC, 0xE8, 0x82, 0x70, 0x55, 0xAC, 100);
#define INIT_PKEY_PropGroup_Image { { 0xE3690A87, 0x0FA8, 0x4A2A, 0x9A, 0x9F, 0xFC, 0xE8, 0x82, 0x70, 0x55, 0xAC }, 100 }

//  Name:     System.PropGroup.Media -- PKEY_PropGroup_Media
//  Type:     Null -- VT_NULL
//  FormatID: {61872CF7-6B5E-4B4B-AC2D-59DA84459248}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Media, 0x61872CF7, 0x6B5E, 0x4B4B, 0xAC, 0x2D, 0x59, 0xDA, 0x84, 0x45, 0x92, 0x48, 100);
#define INIT_PKEY_PropGroup_Media { { 0x61872CF7, 0x6B5E, 0x4B4B, 0xAC, 0x2D, 0x59, 0xDA, 0x84, 0x45, 0x92, 0x48 }, 100 }

//  Name:     System.PropGroup.MediaAdvanced -- PKEY_PropGroup_MediaAdvanced
//  Type:     Null -- VT_NULL
//  FormatID: {8859A284-DE7E-4642-99BA-D431D044B1EC}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_MediaAdvanced, 0x8859A284, 0xDE7E, 0x4642, 0x99, 0xBA, 0xD4, 0x31, 0xD0, 0x44, 0xB1, 0xEC, 100);
#define INIT_PKEY_PropGroup_MediaAdvanced { { 0x8859A284, 0xDE7E, 0x4642, 0x99, 0xBA, 0xD4, 0x31, 0xD0, 0x44, 0xB1, 0xEC }, 100 }

//  Name:     System.PropGroup.Message -- PKEY_PropGroup_Message
//  Type:     Null -- VT_NULL
//  FormatID: {7FD7259D-16B4-4135-9F97-7C96ECD2FA9E}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Message, 0x7FD7259D, 0x16B4, 0x4135, 0x9F, 0x97, 0x7C, 0x96, 0xEC, 0xD2, 0xFA, 0x9E, 100);
#define INIT_PKEY_PropGroup_Message { { 0x7FD7259D, 0x16B4, 0x4135, 0x9F, 0x97, 0x7C, 0x96, 0xEC, 0xD2, 0xFA, 0x9E }, 100 }

//  Name:     System.PropGroup.Music -- PKEY_PropGroup_Music
//  Type:     Null -- VT_NULL
//  FormatID: {68DD6094-7216-40F1-A029-43FE7127043F}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Music, 0x68DD6094, 0x7216, 0x40F1, 0xA0, 0x29, 0x43, 0xFE, 0x71, 0x27, 0x04, 0x3F, 100);
#define INIT_PKEY_PropGroup_Music { { 0x68DD6094, 0x7216, 0x40F1, 0xA0, 0x29, 0x43, 0xFE, 0x71, 0x27, 0x04, 0x3F }, 100 }

//  Name:     System.PropGroup.Origin -- PKEY_PropGroup_Origin
//  Type:     Null -- VT_NULL
//  FormatID: {2598D2FB-5569-4367-95DF-5CD3A177E1A5}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Origin, 0x2598D2FB, 0x5569, 0x4367, 0x95, 0xDF, 0x5C, 0xD3, 0xA1, 0x77, 0xE1, 0xA5, 100);
#define INIT_PKEY_PropGroup_Origin { { 0x2598D2FB, 0x5569, 0x4367, 0x95, 0xDF, 0x5C, 0xD3, 0xA1, 0x77, 0xE1, 0xA5 }, 100 }

//  Name:     System.PropGroup.PhotoAdvanced -- PKEY_PropGroup_PhotoAdvanced
//  Type:     Null -- VT_NULL
//  FormatID: {0CB2BF5A-9EE7-4A86-8222-F01E07FDADAF}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_PhotoAdvanced, 0x0CB2BF5A, 0x9EE7, 0x4A86, 0x82, 0x22, 0xF0, 0x1E, 0x07, 0xFD, 0xAD, 0xAF, 100);
#define INIT_PKEY_PropGroup_PhotoAdvanced { { 0x0CB2BF5A, 0x9EE7, 0x4A86, 0x82, 0x22, 0xF0, 0x1E, 0x07, 0xFD, 0xAD, 0xAF }, 100 }

//  Name:     System.PropGroup.RecordedTV -- PKEY_PropGroup_RecordedTV
//  Type:     Null -- VT_NULL
//  FormatID: {E7B33238-6584-4170-A5C0-AC25EFD9DA56}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_RecordedTV, 0xE7B33238, 0x6584, 0x4170, 0xA5, 0xC0, 0xAC, 0x25, 0xEF, 0xD9, 0xDA, 0x56, 100);
#define INIT_PKEY_PropGroup_RecordedTV { { 0xE7B33238, 0x6584, 0x4170, 0xA5, 0xC0, 0xAC, 0x25, 0xEF, 0xD9, 0xDA, 0x56 }, 100 }

//  Name:     System.PropGroup.Video -- PKEY_PropGroup_Video
//  Type:     Null -- VT_NULL
//  FormatID: {BEBE0920-7671-4C54-A3EB-49FDDFC191EE}, 100
DEFINE_PROPERTYKEY(PKEY_PropGroup_Video, 0xBEBE0920, 0x7671, 0x4C54, 0xA3, 0xEB, 0x49, 0xFD, 0xDF, 0xC1, 0x91, 0xEE, 100);
#define INIT_PKEY_PropGroup_Video { { 0xBEBE0920, 0x7671, 0x4C54, 0xA3, 0xEB, 0x49, 0xFD, 0xDF, 0xC1, 0x91, 0xEE }, 100 }

//-----------------------------------------------------------------------------
// PropList properties

//  Name:     System.InfoTipText -- PKEY_InfoTipText
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 17
//  
//  The text (with formatted property values) to show in the infotip.
DEFINE_PROPERTYKEY(PKEY_InfoTipText, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 17);
#define INIT_PKEY_InfoTipText { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 17 }

//  Name:     System.PropList.ConflictPrompt -- PKEY_PropList_ConflictPrompt
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 11
//  
//  The list of properties to show in the file operation conflict resolution dialog. Properties with empty 
//  values will not be displayed. Register under the regvalue of "ConflictPrompt".
DEFINE_PROPERTYKEY(PKEY_PropList_ConflictPrompt, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 11);
#define INIT_PKEY_PropList_ConflictPrompt { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 11 }

//  Name:     System.PropList.ContentViewModeForBrowse -- PKEY_PropList_ContentViewModeForBrowse
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 13
//  
//  The list of properties to show in the content view mode of an item in the context of browsing.
//  Register the regvalue under the name of "ContentViewModeForBrowse".
DEFINE_PROPERTYKEY(PKEY_PropList_ContentViewModeForBrowse, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 13);
#define INIT_PKEY_PropList_ContentViewModeForBrowse { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 13 }

//  Name:     System.PropList.ContentViewModeForSearch -- PKEY_PropList_ContentViewModeForSearch
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 14
//  
//  The list of properties to show in the content view mode of an item in the context of searching.
//  Register the regvalue under the name of "ContentViewModeForSearch".
DEFINE_PROPERTYKEY(PKEY_PropList_ContentViewModeForSearch, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 14);
#define INIT_PKEY_PropList_ContentViewModeForSearch { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 14 }

//  Name:     System.PropList.ExtendedTileInfo -- PKEY_PropList_ExtendedTileInfo
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 9
//  
//  The list of properties to show in the listview on extended tiles. Register under the regvalue of 
//  "ExtendedTileInfo".
DEFINE_PROPERTYKEY(PKEY_PropList_ExtendedTileInfo, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 9);
#define INIT_PKEY_PropList_ExtendedTileInfo { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 9 }

//  Name:     System.PropList.FileOperationPrompt -- PKEY_PropList_FileOperationPrompt
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 10
//  
//  The list of properties to show in the file operation confirmation dialog. Properties with empty values 
//  will not be displayed. If this list is not specified, then the InfoTip property list is used instead. 
//  Register under the regvalue of "FileOperationPrompt".
DEFINE_PROPERTYKEY(PKEY_PropList_FileOperationPrompt, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 10);
#define INIT_PKEY_PropList_FileOperationPrompt { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 10 }

//  Name:     System.PropList.FullDetails -- PKEY_PropList_FullDetails
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 2
//  
//  The list of all the properties to show in the details page.  Property groups can be included in this list 
//  in order to more easily organize the UI.  Register under the regvalue of "FullDetails".
DEFINE_PROPERTYKEY(PKEY_PropList_FullDetails, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 2);
#define INIT_PKEY_PropList_FullDetails { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 2 }

//  Name:     System.PropList.InfoTip -- PKEY_PropList_InfoTip
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 4 (PID_PROPLIST_INFOTIP)
//  
//  The list of properties to show in the infotip. Properties with empty values will not be displayed. Register 
//  under the regvalue of "InfoTip".
DEFINE_PROPERTYKEY(PKEY_PropList_InfoTip, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 4);
#define INIT_PKEY_PropList_InfoTip { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 4 }

//  Name:     System.PropList.NonPersonal -- PKEY_PropList_NonPersonal
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {49D1091F-082E-493F-B23F-D2308AA9668C}, 100
//  
//  The list of properties that are considered 'non-personal'. When told to remove all non-personal properties 
//  from a given file, the system will leave these particular properties untouched. Register under the regvalue 
//  of "NonPersonal".
DEFINE_PROPERTYKEY(PKEY_PropList_NonPersonal, 0x49D1091F, 0x082E, 0x493F, 0xB2, 0x3F, 0xD2, 0x30, 0x8A, 0xA9, 0x66, 0x8C, 100);
#define INIT_PKEY_PropList_NonPersonal { { 0x49D1091F, 0x082E, 0x493F, 0xB2, 0x3F, 0xD2, 0x30, 0x8A, 0xA9, 0x66, 0x8C }, 100 }

//  Name:     System.PropList.PreviewDetails -- PKEY_PropList_PreviewDetails
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 8
//
//  The list of properties to display in the preview pane.  Register under the regvalue of "PreviewDetails".
DEFINE_PROPERTYKEY(PKEY_PropList_PreviewDetails, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 8);
#define INIT_PKEY_PropList_PreviewDetails { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 8 }

//  Name:     System.PropList.PreviewTitle -- PKEY_PropList_PreviewTitle
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 6
//  
//  The one or two properties to display in the preview pane title section.  The optional second property is 
//  displayed as a subtitle.  Register under the regvalue of "PreviewTitle".
DEFINE_PROPERTYKEY(PKEY_PropList_PreviewTitle, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 6);
#define INIT_PKEY_PropList_PreviewTitle { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 6 }

//  Name:     System.PropList.QuickTip -- PKEY_PropList_QuickTip
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 5 (PID_PROPLIST_QUICKTIP)
//  
//  The list of properties to show in the infotip when the item is on a slow network. Properties with empty 
//  values will not be displayed. Register under the regvalue of "QuickTip".
DEFINE_PROPERTYKEY(PKEY_PropList_QuickTip, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 5);
#define INIT_PKEY_PropList_QuickTip { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 5 }

//  Name:     System.PropList.TileInfo -- PKEY_PropList_TileInfo
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {C9944A21-A406-48FE-8225-AEC7E24C211B}, 3 (PID_PROPLIST_TILEINFO)
//  
//  The list of properties to show in the listview on tiles. Register under the regvalue of "TileInfo".
DEFINE_PROPERTYKEY(PKEY_PropList_TileInfo, 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B, 3);
#define INIT_PKEY_PropList_TileInfo { { 0xC9944A21, 0xA406, 0x48FE, 0x82, 0x25, 0xAE, 0xC7, 0xE2, 0x4C, 0x21, 0x1B }, 3 }

//  Name:     System.PropList.XPDetailsPanel -- PKEY_PropList_XPDetailsPanel
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_WebView) {F2275480-F782-4291-BD94-F13693513AEC}, 0 (PID_DISPLAY_PROPERTIES)
//
//  The list of properties to display in the XP webview details panel. Obsolete.
DEFINE_PROPERTYKEY(PKEY_PropList_XPDetailsPanel, 0xF2275480, 0xF782, 0x4291, 0xBD, 0x94, 0xF1, 0x36, 0x93, 0x51, 0x3A, 0xEC, 0);
#define INIT_PKEY_PropList_XPDetailsPanel { { 0xF2275480, 0xF782, 0x4291, 0xBD, 0x94, 0xF1, 0x36, 0x93, 0x51, 0x3A, 0xEC }, 0 }

//-----------------------------------------------------------------------------
// RecordedTV properties

//  Name:     System.RecordedTV.ChannelNumber -- PKEY_RecordedTV_ChannelNumber
//  Type:     UInt32 -- VT_UI4
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 7
//
//  Example: 42
DEFINE_PROPERTYKEY(PKEY_RecordedTV_ChannelNumber, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 7);
#define INIT_PKEY_RecordedTV_ChannelNumber { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 7 }

//  Name:     System.RecordedTV.Credits -- PKEY_RecordedTV_Credits
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 4
//
//  Example: "Don Messick/Frank Welker/Casey Kasem/Heather North/Nicole Jaffe;;;"
DEFINE_PROPERTYKEY(PKEY_RecordedTV_Credits, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 4);
#define INIT_PKEY_RecordedTV_Credits { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 4 }

//  Name:     System.RecordedTV.DateContentExpires -- PKEY_RecordedTV_DateContentExpires
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 15
DEFINE_PROPERTYKEY(PKEY_RecordedTV_DateContentExpires, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 15);
#define INIT_PKEY_RecordedTV_DateContentExpires { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 15 }

//  Name:     System.RecordedTV.EpisodeName -- PKEY_RecordedTV_EpisodeName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 2
//
//  Example: "Nowhere to Hyde"
DEFINE_PROPERTYKEY(PKEY_RecordedTV_EpisodeName, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 2);
#define INIT_PKEY_RecordedTV_EpisodeName { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 2 }

//  Name:     System.RecordedTV.IsATSCContent -- PKEY_RecordedTV_IsATSCContent
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 16
DEFINE_PROPERTYKEY(PKEY_RecordedTV_IsATSCContent, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 16);
#define INIT_PKEY_RecordedTV_IsATSCContent { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 16 }

//  Name:     System.RecordedTV.IsClosedCaptioningAvailable -- PKEY_RecordedTV_IsClosedCaptioningAvailable
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 12
DEFINE_PROPERTYKEY(PKEY_RecordedTV_IsClosedCaptioningAvailable, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 12);
#define INIT_PKEY_RecordedTV_IsClosedCaptioningAvailable { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 12 }

//  Name:     System.RecordedTV.IsDTVContent -- PKEY_RecordedTV_IsDTVContent
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 17
DEFINE_PROPERTYKEY(PKEY_RecordedTV_IsDTVContent, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 17);
#define INIT_PKEY_RecordedTV_IsDTVContent { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 17 }

//  Name:     System.RecordedTV.IsHDContent -- PKEY_RecordedTV_IsHDContent
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 18
DEFINE_PROPERTYKEY(PKEY_RecordedTV_IsHDContent, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 18);
#define INIT_PKEY_RecordedTV_IsHDContent { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 18 }

//  Name:     System.RecordedTV.IsRepeatBroadcast -- PKEY_RecordedTV_IsRepeatBroadcast
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 13
DEFINE_PROPERTYKEY(PKEY_RecordedTV_IsRepeatBroadcast, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 13);
#define INIT_PKEY_RecordedTV_IsRepeatBroadcast { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 13 }

//  Name:     System.RecordedTV.IsSAP -- PKEY_RecordedTV_IsSAP
//  Type:     Boolean -- VT_BOOL
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 14
DEFINE_PROPERTYKEY(PKEY_RecordedTV_IsSAP, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 14);
#define INIT_PKEY_RecordedTV_IsSAP { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 14 }

//  Name:     System.RecordedTV.NetworkAffiliation -- PKEY_RecordedTV_NetworkAffiliation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {2C53C813-FB63-4E22-A1AB-0B331CA1E273}, 100
DEFINE_PROPERTYKEY(PKEY_RecordedTV_NetworkAffiliation, 0x2C53C813, 0xFB63, 0x4E22, 0xA1, 0xAB, 0x0B, 0x33, 0x1C, 0xA1, 0xE2, 0x73, 100);
#define INIT_PKEY_RecordedTV_NetworkAffiliation { { 0x2C53C813, 0xFB63, 0x4E22, 0xA1, 0xAB, 0x0B, 0x33, 0x1C, 0xA1, 0xE2, 0x73 }, 100 }

//  Name:     System.RecordedTV.OriginalBroadcastDate -- PKEY_RecordedTV_OriginalBroadcastDate
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {4684FE97-8765-4842-9C13-F006447B178C}, 100
DEFINE_PROPERTYKEY(PKEY_RecordedTV_OriginalBroadcastDate, 0x4684FE97, 0x8765, 0x4842, 0x9C, 0x13, 0xF0, 0x06, 0x44, 0x7B, 0x17, 0x8C, 100);
#define INIT_PKEY_RecordedTV_OriginalBroadcastDate { { 0x4684FE97, 0x8765, 0x4842, 0x9C, 0x13, 0xF0, 0x06, 0x44, 0x7B, 0x17, 0x8C }, 100 }

//  Name:     System.RecordedTV.ProgramDescription -- PKEY_RecordedTV_ProgramDescription
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 3
DEFINE_PROPERTYKEY(PKEY_RecordedTV_ProgramDescription, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 3);
#define INIT_PKEY_RecordedTV_ProgramDescription { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 3 }

//  Name:     System.RecordedTV.RecordingTime -- PKEY_RecordedTV_RecordingTime
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {A5477F61-7A82-4ECA-9DDE-98B69B2479B3}, 100
DEFINE_PROPERTYKEY(PKEY_RecordedTV_RecordingTime, 0xA5477F61, 0x7A82, 0x4ECA, 0x9D, 0xDE, 0x98, 0xB6, 0x9B, 0x24, 0x79, 0xB3, 100);
#define INIT_PKEY_RecordedTV_RecordingTime { { 0xA5477F61, 0x7A82, 0x4ECA, 0x9D, 0xDE, 0x98, 0xB6, 0x9B, 0x24, 0x79, 0xB3 }, 100 }

//  Name:     System.RecordedTV.StationCallSign -- PKEY_RecordedTV_StationCallSign
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {6D748DE2-8D38-4CC3-AC60-F009B057C557}, 5
//
//  Example: "TOONP"
DEFINE_PROPERTYKEY(PKEY_RecordedTV_StationCallSign, 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57, 5);
#define INIT_PKEY_RecordedTV_StationCallSign { { 0x6D748DE2, 0x8D38, 0x4CC3, 0xAC, 0x60, 0xF0, 0x09, 0xB0, 0x57, 0xC5, 0x57 }, 5 }

//  Name:     System.RecordedTV.StationName -- PKEY_RecordedTV_StationName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {1B5439E7-EBA1-4AF8-BDD7-7AF1D4549493}, 100
DEFINE_PROPERTYKEY(PKEY_RecordedTV_StationName, 0x1B5439E7, 0xEBA1, 0x4AF8, 0xBD, 0xD7, 0x7A, 0xF1, 0xD4, 0x54, 0x94, 0x93, 100);
#define INIT_PKEY_RecordedTV_StationName { { 0x1B5439E7, 0xEBA1, 0x4AF8, 0xBD, 0xD7, 0x7A, 0xF1, 0xD4, 0x54, 0x94, 0x93 }, 100 }

//-----------------------------------------------------------------------------
// Search properties

//  Name:     System.LocationEmptyString -- PKEY_LocationEmptyString
//  Type:     Unspecified -- VT_NULL
//  FormatID: {62D2D9AB-8B64-498D-B865-402D4796F865}, 3
//
//  The display string shown when the user navigates to a location, but the location is empty.
DEFINE_PROPERTYKEY(PKEY_LocationEmptyString, 0x62D2D9AB, 0x8B64, 0x498D, 0xB8, 0x65, 0x40, 0x2D, 0x47, 0x96, 0xF8, 0x65, 3);
#define INIT_PKEY_LocationEmptyString { { 0x62D2D9AB, 0x8B64, 0x498D, 0xB8, 0x65, 0x40, 0x2D, 0x47, 0x96, 0xF8, 0x65 }, 3 }

//  Name:     System.Search.AutoCategory -- PKEY_Search_AutoCategory
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 31
//
//  This is a multi-valued property to store categories auto generated by a AI Model.
DEFINE_PROPERTYKEY(PKEY_Search_AutoCategory, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 31);
#define INIT_PKEY_Search_AutoCategory { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 31 }

//  Name:     System.Search.AutoSummary -- PKEY_Search_AutoSummary
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {560C36C0-503A-11CF-BAA1-00004C752A9A}, 2
//
//  General Summary of the document.
DEFINE_PROPERTYKEY(PKEY_Search_AutoSummary, 0x560C36C0, 0x503A, 0x11CF, 0xBA, 0xA1, 0x00, 0x00, 0x4C, 0x75, 0x2A, 0x9A, 2);
#define INIT_PKEY_Search_AutoSummary { { 0x560C36C0, 0x503A, 0x11CF, 0xBA, 0xA1, 0x00, 0x00, 0x4C, 0x75, 0x2A, 0x9A }, 2 }

//  Name:     System.Search.ContainerHash -- PKEY_Search_ContainerHash
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {BCEEE283-35DF-4D53-826A-F36A3EEFC6BE}, 100
//
//  Hash code used to identify attachments to be deleted based on a common container url
DEFINE_PROPERTYKEY(PKEY_Search_ContainerHash, 0xBCEEE283, 0x35DF, 0x4D53, 0x82, 0x6A, 0xF3, 0x6A, 0x3E, 0xEF, 0xC6, 0xBE, 100);
#define INIT_PKEY_Search_ContainerHash { { 0xBCEEE283, 0x35DF, 0x4D53, 0x82, 0x6A, 0xF3, 0x6A, 0x3E, 0xEF, 0xC6, 0xBE }, 100 }

//  Name:     System.Search.Contents -- PKEY_Search_Contents
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 19 (PID_STG_CONTENTS)
//  
//  The contents of the item. This property is for query restrictions only; it cannot be retrieved in a 
//  query result. The Indexing Service friendly name is 'contents'.
DEFINE_PROPERTYKEY(PKEY_Search_Contents, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 19);
#define INIT_PKEY_Search_Contents { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 19 }

//  Name:     System.Search.EntryID -- PKEY_Search_EntryID
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_Query) {49691C90-7E17-101A-A91C-08002B2ECDA9}, 5 (PROPID_QUERY_WORKID)
//  
//  The entry ID for an item within a given catalog in the Windows Search Index.
//  This value may be recycled, and therefore is not considered unique over time.
DEFINE_PROPERTYKEY(PKEY_Search_EntryID, 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9, 5);
#define INIT_PKEY_Search_EntryID { { 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9 }, 5 }

//  Name:     System.Search.ExtendedProperties -- PKEY_Search_ExtendedProperties
//  Type:     Blob -- VT_BLOB
//  FormatID: {7B03B546-FA4F-4A52-A2FE-03D5311E5865}, 100
DEFINE_PROPERTYKEY(PKEY_Search_ExtendedProperties, 0x7B03B546, 0xFA4F, 0x4A52, 0xA2, 0xFE, 0x03, 0xD5, 0x31, 0x1E, 0x58, 0x65, 100);
#define INIT_PKEY_Search_ExtendedProperties { { 0x7B03B546, 0xFA4F, 0x4A52, 0xA2, 0xFE, 0x03, 0xD5, 0x31, 0x1E, 0x58, 0x65 }, 100 }

//  Name:     System.Search.GatherTime -- PKEY_Search_GatherTime
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {0B63E350-9CCC-11D0-BCDB-00805FCCCE04}, 8
//
//  The Datetime that the Windows Search Gatherer process last pushed properties of this document to the Windows Search Gatherer Plugins.
DEFINE_PROPERTYKEY(PKEY_Search_GatherTime, 0x0B63E350, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04, 8);
#define INIT_PKEY_Search_GatherTime { { 0x0B63E350, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04 }, 8 }

//  Name:     System.Search.HitCount -- PKEY_Search_HitCount
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_Query) {49691C90-7E17-101A-A91C-08002B2ECDA9}, 4 (PROPID_QUERY_HITCOUNT)
//  
//  When using CONTAINS over the Windows Search Index, this is the number of matches of the term.
//  If there are multiple CONTAINS, an AND computes the min number of hits and an OR the max number of hits.
DEFINE_PROPERTYKEY(PKEY_Search_HitCount, 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9, 4);
#define INIT_PKEY_Search_HitCount { { 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9 }, 4 }

//  Name:     System.Search.IsClosedDirectory -- PKEY_Search_IsClosedDirectory
//  Type:     Boolean -- VT_BOOL
//  FormatID: {0B63E343-9CCC-11D0-BCDB-00805FCCCE04}, 23
//
//  If this property is emitted with a value of TRUE, then it indicates that this URL's last modified time applies to all of it's children, and if this URL is deleted then all of it's children are deleted as well.  For example, this would be emitted as TRUE when emitting the URL of an email so that all attachments are tied to the last modified time of that email.
DEFINE_PROPERTYKEY(PKEY_Search_IsClosedDirectory, 0x0B63E343, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04, 23);
#define INIT_PKEY_Search_IsClosedDirectory { { 0x0B63E343, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04 }, 23 }

//  Name:     System.Search.IsFullyContained -- PKEY_Search_IsFullyContained
//  Type:     Boolean -- VT_BOOL
//  FormatID: {0B63E343-9CCC-11D0-BCDB-00805FCCCE04}, 24
//
//  Any child URL of a URL which has System.Search.IsClosedDirectory=TRUE must emit System.Search.IsFullyContained=TRUE.  This ensures that the URL is not deleted at the end of a crawl because it hasn't been visited (which is the normal mechanism for detecting deletes).  For example an email attachment would emit this property
DEFINE_PROPERTYKEY(PKEY_Search_IsFullyContained, 0x0B63E343, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04, 24);
#define INIT_PKEY_Search_IsFullyContained { { 0x0B63E343, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04 }, 24 }

//  Name:     System.Search.MatchKind -- PKEY_Search_MatchKind
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_Query) {49691C90-7E17-101A-A91C-08002B2ECDA9}, 29
//
//  States the type of search used to match this file to the search query.
DEFINE_PROPERTYKEY(PKEY_Search_MatchKind, 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9, 29);
#define INIT_PKEY_Search_MatchKind { { 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9 }, 29 }

// Possible discrete values for PKEY_Search_MatchKind are:
#define MATCH_KIND_LEXICAL                  1l
#define MATCH_KIND_SEMANTIC                 2l

//  Name:     System.Search.MatchTags -- PKEY_Search_MatchTags
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_Query) {49691C90-7E17-101A-A91C-08002B2ECDA9}, 30
//
//  This multi-valued string indicates the type of result matched for the query results.
DEFINE_PROPERTYKEY(PKEY_Search_MatchTags, 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9, 30);
#define INIT_PKEY_Search_MatchTags { { 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9 }, 30 }

//  Name:     System.Search.OcrContent -- PKEY_Search_OcrContent
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Storage) {B725F130-47EF-101A-A5F1-02608C9EEBAC}, 28
//  
//  The OCR contents of the Images. This property is for query restrictions only; it cannot be retrieved in a 
//  query result.
DEFINE_PROPERTYKEY(PKEY_Search_OcrContent, 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC, 28);
#define INIT_PKEY_Search_OcrContent { { 0xB725F130, 0x47EF, 0x101A, 0xA5, 0xF1, 0x02, 0x60, 0x8C, 0x9E, 0xEB, 0xAC }, 28 }

//  Name:     System.Search.QueryFocusedSummary -- PKEY_Search_QueryFocusedSummary
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {560C36C0-503A-11CF-BAA1-00004C752A9A}, 3
//
//  Query Focused Summary of the document.
DEFINE_PROPERTYKEY(PKEY_Search_QueryFocusedSummary, 0x560C36C0, 0x503A, 0x11CF, 0xBA, 0xA1, 0x00, 0x00, 0x4C, 0x75, 0x2A, 0x9A, 3);
#define INIT_PKEY_Search_QueryFocusedSummary { { 0x560C36C0, 0x503A, 0x11CF, 0xBA, 0xA1, 0x00, 0x00, 0x4C, 0x75, 0x2A, 0x9A }, 3 }

//  Name:     System.Search.QueryFocusedSummaryWithFallback -- PKEY_Search_QueryFocusedSummaryWithFallback
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {560C36C0-503A-11CF-BAA1-00004C752A9A}, 4
//
//  Query Focused Summary of the document, if none is available it returns the AutoSummary.
DEFINE_PROPERTYKEY(PKEY_Search_QueryFocusedSummaryWithFallback, 0x560C36C0, 0x503A, 0x11CF, 0xBA, 0xA1, 0x00, 0x00, 0x4C, 0x75, 0x2A, 0x9A, 4);
#define INIT_PKEY_Search_QueryFocusedSummaryWithFallback { { 0x560C36C0, 0x503A, 0x11CF, 0xBA, 0xA1, 0x00, 0x00, 0x4C, 0x75, 0x2A, 0x9A }, 4 }

//  Name:     System.Search.QueryPropertyHits -- PKEY_Search_QueryPropertyHits
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (FMTID_Query) {49691C90-7E17-101A-A91C-08002B2ECDA9}, 21
//
//  Contains the list of matched properties from a query.
DEFINE_PROPERTYKEY(PKEY_Search_QueryPropertyHits, 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9, 21);
#define INIT_PKEY_Search_QueryPropertyHits { { 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9 }, 21 }

//  Name:     System.Search.Rank -- PKEY_Search_Rank
//  Type:     Int32 -- VT_I4
//  FormatID: (FMTID_Query) {49691C90-7E17-101A-A91C-08002B2ECDA9}, 3 (PROPID_QUERY_RANK)
//  
//  Relevance rank of row. Ranges from 0-1000. Larger numbers = better matches.  Query-time only.
DEFINE_PROPERTYKEY(PKEY_Search_Rank, 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9, 3);
#define INIT_PKEY_Search_Rank { { 0x49691C90, 0x7E17, 0x101A, 0xA9, 0x1C, 0x08, 0x00, 0x2B, 0x2E, 0xCD, 0xA9 }, 3 }

//  Name:     System.Search.Store -- PKEY_Search_Store
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {A06992B3-8CAF-4ED7-A547-B259E32AC9FC}, 100
//
//  The identifier for the protocol handler that produced this item. (E.g. MAPI, CSC, FILE etc.)
DEFINE_PROPERTYKEY(PKEY_Search_Store, 0xA06992B3, 0x8CAF, 0x4ED7, 0xA5, 0x47, 0xB2, 0x59, 0xE3, 0x2A, 0xC9, 0xFC, 100);
#define INIT_PKEY_Search_Store { { 0xA06992B3, 0x8CAF, 0x4ED7, 0xA5, 0x47, 0xB2, 0x59, 0xE3, 0x2A, 0xC9, 0xFC }, 100 }

//  Name:     System.Search.UrlToIndex -- PKEY_Search_UrlToIndex
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {0B63E343-9CCC-11D0-BCDB-00805FCCCE04}, 2
//
//  This property should be emitted by a container IFilter for each child URL within the container.  The children will eventually be crawled by the indexer if they are within scope.
DEFINE_PROPERTYKEY(PKEY_Search_UrlToIndex, 0x0B63E343, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04, 2);
#define INIT_PKEY_Search_UrlToIndex { { 0x0B63E343, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04 }, 2 }

//  Name:     System.Search.UrlToIndexWithModificationTime -- PKEY_Search_UrlToIndexWithModificationTime
//  Type:     Multivalue Any -- VT_VECTOR | VT_NULL  (For variants: VT_ARRAY | VT_NULL)
//  FormatID: {0B63E343-9CCC-11D0-BCDB-00805FCCCE04}, 12
//
//  This property is the same as System.Search.UrlToIndex except that it includes the time the URL was last modified.  This is an optimization for the indexer as it doesn't have to call back into the protocol handler to ask for this information to determine if the content needs to be indexed again.  The property is a vector with two elements, a VT_LPWSTR with the URL and a VT_FILETIME for the last modified time.
DEFINE_PROPERTYKEY(PKEY_Search_UrlToIndexWithModificationTime, 0x0B63E343, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04, 12);
#define INIT_PKEY_Search_UrlToIndexWithModificationTime { { 0x0B63E343, 0x9CCC, 0x11D0, 0xBC, 0xDB, 0x00, 0x80, 0x5F, 0xCC, 0xCE, 0x04 }, 12 }

//  Name:     System.Supplemental.Album -- PKEY_Supplemental_Album
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {0C73B141-39D6-4653-A683-CAB291EAF95B}, 6
//  
//  Contains list of the album where the item belongs or is associated with.
DEFINE_PROPERTYKEY(PKEY_Supplemental_Album, 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B, 6);
#define INIT_PKEY_Supplemental_Album { { 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B }, 6 }

//  Name:     System.Supplemental.AlbumID -- PKEY_Supplemental_AlbumID
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {0C73B141-39D6-4653-A683-CAB291EAF95B}, 2
//  
//  Contains the identifiers of the albums that the item is a member of. Can be used in conjunction with the Album item in the Content Indexer APIs 
//  to notify other apps about picture albums either the user created or apps have already created.
DEFINE_PROPERTYKEY(PKEY_Supplemental_AlbumID, 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B, 2);
#define INIT_PKEY_Supplemental_AlbumID { { 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B }, 2 }

//  Name:     System.Supplemental.Location -- PKEY_Supplemental_Location
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {0C73B141-39D6-4653-A683-CAB291EAF95B}, 5
//  
//  Contains list of location information identified by client such as Photo app.
DEFINE_PROPERTYKEY(PKEY_Supplemental_Location, 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B, 5);
#define INIT_PKEY_Supplemental_Location { { 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B }, 5 }

//  Name:     System.Supplemental.Person -- PKEY_Supplemental_Person
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {0C73B141-39D6-4653-A683-CAB291EAF95B}, 7
//  
//  Contains list of person identified by client such as Photo app.
DEFINE_PROPERTYKEY(PKEY_Supplemental_Person, 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B, 7);
#define INIT_PKEY_Supplemental_Person { { 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B }, 7 }

//  Name:     System.Supplemental.ResourceId -- PKEY_Supplemental_ResourceId
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {0C73B141-39D6-4653-A683-CAB291EAF95B}, 3
//  
//  Contains the identifier for the item on the remote sync service. Used for comparing a file on the system to ones that are available in the cloud.
DEFINE_PROPERTYKEY(PKEY_Supplemental_ResourceId, 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B, 3);
#define INIT_PKEY_Supplemental_ResourceId { { 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B }, 3 }

//  Name:     System.Supplemental.Tag -- PKEY_Supplemental_Tag
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {0C73B141-39D6-4653-A683-CAB291EAF95B}, 4
//  
//  Contains list of tag associated with the item.
DEFINE_PROPERTYKEY(PKEY_Supplemental_Tag, 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B, 4);
#define INIT_PKEY_Supplemental_Tag { { 0x0C73B141, 0x39D6, 0x4653, 0xA6, 0x83, 0xCA, 0xB2, 0x91, 0xEA, 0xF9, 0x5B }, 4 }

//-----------------------------------------------------------------------------
// Shell properties

//  Name:     System.ActivityDate -- PKEY_ActivityDate
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {30C8EEF4-A832-41E2-AB32-E3C3CA28FD29}, 23
//  
//  The date and time of when the activity described in System.ActivityInfo took place for this item.
//  If System.ActivityInfo is VT_EMPTY, then this property should be too.
DEFINE_PROPERTYKEY(PKEY_ActivityDate, 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29, 23);
#define INIT_PKEY_ActivityDate { { 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29 }, 23 }

//  Name:     System.ActivityIcon -- PKEY_ActivityIcon
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {30C8EEF4-A832-41E2-AB32-E3C3CA28FD29}, 24
//  
//  The string corresponding to a glyph in the Segoe Fluent Icons font that represents the activity described in System.ActivityInfo.
//  For example, "\xE70F" for a file that was recently edited, or "\xE716" for a file that was recently shared.
//  If System.ActivityInfo is VT_EMPTY, then this property should be too.
DEFINE_PROPERTYKEY(PKEY_ActivityIcon, 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29, 24);
#define INIT_PKEY_ActivityIcon { { 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29 }, 24 }

//  Name:     System.ActivityInfo -- PKEY_ActivityInfo
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {30C8EEF4-A832-41E2-AB32-E3C3CA28FD29}, 17
//  
//  A user-friendly description of activity that took place on this file or folder.
//  This can be used to provide context as to why an item has been recommended to the user, or to describe a recent action taken on the item.
//  For example, "You edited this" for a file the user recently edited, or "John Doe shared this with you" for a file that was recently shared.
DEFINE_PROPERTYKEY(PKEY_ActivityInfo, 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29, 17);
#define INIT_PKEY_ActivityInfo { { 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29 }, 17 }

//  Name:     System.DescriptionID -- PKEY_DescriptionID
//  Type:     Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 2 (PID_DESCRIPTIONID)
//
//  The contents of a SHDESCRIPTIONID structure as a buffer of bytes.
DEFINE_PROPERTYKEY(PKEY_DescriptionID, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 2);
#define INIT_PKEY_DescriptionID { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 2 }

//  Name:     System.Home.Grouping -- PKEY_Home_Grouping
//  Type:     UInt32 -- VT_UI4
//  FormatID: {30C8EEF4-A832-41E2-AB32-E3C3CA28FD29}, 2
//
//  Which group the element belongs to frequent, pinned, recent or recommendations
DEFINE_PROPERTYKEY(PKEY_Home_Grouping, 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29, 2);
#define INIT_PKEY_Home_Grouping { { 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29 }, 2 }

// Possible discrete values for PKEY_Home_Grouping are:
#define HOMEGROUPING_UNSPECIFIED            0ul
#define HOMEGROUPING_FREQUENT               1ul
#define HOMEGROUPING_PINNED                 2ul
#define HOMEGROUPING_RECENT                 3ul
#define HOMEGROUPING_RECOMMENDATIONS        4ul
#define HOMEGROUPING_SHARED                 5ul

//  Name:     System.Home.IsPinned -- PKEY_Home_IsPinned
//  Type:     Boolean -- VT_BOOL
//  FormatID: {30C8EEF4-A832-41E2-AB32-E3C3CA28FD29}, 4
DEFINE_PROPERTYKEY(PKEY_Home_IsPinned, 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29, 4);
#define INIT_PKEY_Home_IsPinned { { 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29 }, 4 }

//  Name:     System.Home.ItemFolderPathDisplay -- PKEY_Home_ItemFolderPathDisplay
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {30C8EEF4-A832-41E2-AB32-E3C3CA28FD29}, 6
//
//  Friendly name used for the folder path when displayed on File Explorer Home"
DEFINE_PROPERTYKEY(PKEY_Home_ItemFolderPathDisplay, 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29, 6);
#define INIT_PKEY_Home_ItemFolderPathDisplay { { 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29 }, 6 }

//  Name:     System.Home.RecommendationActivityDate -- PKEY_Home_RecommendationActivityDate
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {30C8EEF4-A832-41E2-AB32-E3C3CA28FD29}, 22
//
//  A timestamp representative of when the relevant activity took place for a given recommendation
DEFINE_PROPERTYKEY(PKEY_Home_RecommendationActivityDate, 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29, 22);
#define INIT_PKEY_Home_RecommendationActivityDate { { 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29 }, 22 }

//  Name:     System.Home.RecommendationProviderSource -- PKEY_Home_RecommendationProviderSource
//  Type:     UInt32 -- VT_UI4
//  FormatID: {5CA9B1CB-C69F-404B-ABC6-FD336793A6A7}, 22
//
//  The provider source property for items in the recommended section on FE home
DEFINE_PROPERTYKEY(PKEY_Home_RecommendationProviderSource, 0x5CA9B1CB, 0xC69F, 0x404B, 0xAB, 0xC6, 0xFD, 0x33, 0x67, 0x93, 0xA6, 0xA7, 22);
#define INIT_PKEY_Home_RecommendationProviderSource { { 0x5CA9B1CB, 0xC69F, 0x404B, 0xAB, 0xC6, 0xFD, 0x33, 0x67, 0x93, 0xA6, 0xA7 }, 22 }

//  Name:     System.Home.RecommendationReasonIcon -- PKEY_Home_RecommendationReasonIcon
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {30C8EEF4-A832-41E2-AB32-E3C3CA28FD29}, 21
//
//  The string corresponding to a glyph for the recommended section in FE home
DEFINE_PROPERTYKEY(PKEY_Home_RecommendationReasonIcon, 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29, 21);
#define INIT_PKEY_Home_RecommendationReasonIcon { { 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29 }, 21 }

//  Name:     System.Home.Recommended -- PKEY_Home_Recommended
//  Type:     Boolean -- VT_BOOL
//  FormatID: {30C8EEF4-A832-41E2-AB32-E3C3CA28FD29}, 20
DEFINE_PROPERTYKEY(PKEY_Home_Recommended, 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29, 20);
#define INIT_PKEY_Home_Recommended { { 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29 }, 20 }

//  Name:     System.InternalName -- PKEY_InternalName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSFMTID_VERSION) {0CEF7D53-FA64-11D1-A203-0000F81FEDEE}, 5 (PIDVSI_InternalName)
//
//  
DEFINE_PROPERTYKEY(PKEY_InternalName, 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE, 5);
#define INIT_PKEY_InternalName { { 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE }, 5 }

//  Name:     System.LibraryLocationsCount -- PKEY_LibraryLocationsCount
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {908696C7-8F87-44F2-80ED-A8C1C6894575}, 2
//
//  Library locations count
DEFINE_PROPERTYKEY(PKEY_LibraryLocationsCount, 0x908696C7, 0x8F87, 0x44F2, 0x80, 0xED, 0xA8, 0xC1, 0xC6, 0x89, 0x45, 0x75, 2);
#define INIT_PKEY_LibraryLocationsCount { { 0x908696C7, 0x8F87, 0x44F2, 0x80, 0xED, 0xA8, 0xC1, 0xC6, 0x89, 0x45, 0x75 }, 2 }

//  Name:     System.Link.TargetSFGAOFlagsStrings -- PKEY_Link_TargetSFGAOFlagsStrings
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D6942081-D53B-443D-AD47-5E059D9CD27A}, 3
//  
//  Expresses the SFGAO flags of a link as string values and is used as a query optimization.  See 
//  PKEY_Shell_SFGAOFlagsStrings for possible values of this.
DEFINE_PROPERTYKEY(PKEY_Link_TargetSFGAOFlagsStrings, 0xD6942081, 0xD53B, 0x443D, 0xAD, 0x47, 0x5E, 0x05, 0x9D, 0x9C, 0xD2, 0x7A, 3);
#define INIT_PKEY_Link_TargetSFGAOFlagsStrings { { 0xD6942081, 0xD53B, 0x443D, 0xAD, 0x47, 0x5E, 0x05, 0x9D, 0x9C, 0xD2, 0x7A }, 3 }

//  Name:     System.Link.TargetUrl -- PKEY_Link_TargetUrl
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {5CBF2787-48CF-4208-B90E-EE5E5D420294}, 2  (PKEYs relating to URLs.  Used by IE History.)
DEFINE_PROPERTYKEY(PKEY_Link_TargetUrl, 0x5CBF2787, 0x48CF, 0x4208, 0xB9, 0x0E, 0xEE, 0x5E, 0x5D, 0x42, 0x02, 0x94, 2);
#define INIT_PKEY_Link_TargetUrl { { 0x5CBF2787, 0x48CF, 0x4208, 0xB9, 0x0E, 0xEE, 0x5E, 0x5D, 0x42, 0x02, 0x94 }, 2 }

//  Name:     System.NamespaceCLSID -- PKEY_NamespaceCLSID
//  Type:     Guid -- VT_CLSID
//  FormatID: (FMTID_ShellDetails) {28636AA6-953D-11D2-B5D6-00C04FD918D0}, 6
//
//  The CLSID of the name space extension for an item, the object that implements IShellFolder for this item
DEFINE_PROPERTYKEY(PKEY_NamespaceCLSID, 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0, 6);
#define INIT_PKEY_NamespaceCLSID { { 0x28636AA6, 0x953D, 0x11D2, 0xB5, 0xD6, 0x00, 0xC0, 0x4F, 0xD9, 0x18, 0xD0 }, 6 }

//  Name:     System.Shell.CopilotKeyProviderFastPathMessage -- PKEY_Shell_CopilotKeyProviderFastPathMessage
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {38652BCA-4329-4E74-86F9-39CF29345EEA}, 2
//  
//  Property for Copilot Key Provider application to opt in to fastpath activation
DEFINE_PROPERTYKEY(PKEY_Shell_CopilotKeyProviderFastPathMessage, 0x38652BCA, 0x4329, 0x4E74, 0x86, 0xF9, 0x39, 0xCF, 0x29, 0x34, 0x5E, 0xEA, 2);
#define INIT_PKEY_Shell_CopilotKeyProviderFastPathMessage { { 0x38652BCA, 0x4329, 0x4E74, 0x86, 0xF9, 0x39, 0xCF, 0x29, 0x34, 0x5E, 0xEA }, 2 }

//  Name:     System.Shell.SFGAOFlagsStrings -- PKEY_Shell_SFGAOFlagsStrings
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: {D6942081-D53B-443D-AD47-5E059D9CD27A}, 2
//
//  Expresses the SFGAO flags as string values and is used as a query optimization.
DEFINE_PROPERTYKEY(PKEY_Shell_SFGAOFlagsStrings, 0xD6942081, 0xD53B, 0x443D, 0xAD, 0x47, 0x5E, 0x05, 0x9D, 0x9C, 0xD2, 0x7A, 2);
#define INIT_PKEY_Shell_SFGAOFlagsStrings { { 0xD6942081, 0xD53B, 0x443D, 0xAD, 0x47, 0x5E, 0x05, 0x9D, 0x9C, 0xD2, 0x7A }, 2 }

// Possible discrete values for PKEY_Shell_SFGAOFlagsStrings are:
#define SFGAOSTR_FILESYS                    L"filesys"               // SFGAO_FILESYSTEM
#define SFGAOSTR_FILEANC                    L"fileanc"               // SFGAO_FILESYSANCESTOR
#define SFGAOSTR_STORAGEANC                 L"storageanc"               // SFGAO_STORAGEANCESTOR
#define SFGAOSTR_STREAM                     L"stream"               // SFGAO_STREAM
#define SFGAOSTR_LINK                       L"link"               // SFGAO_LINK
#define SFGAOSTR_HIDDEN                     L"hidden"               // SFGAO_HIDDEN
#define SFGAOSTR_SUPERHIDDEN                L"superhidden"               // SFGAO_SUPERHIDDEN
#define SFGAOSTR_FOLDER                     L"folder"               // SFGAO_FOLDER
#define SFGAOSTR_NONENUM                    L"nonenum"               // SFGAO_NONENUMERATED
#define SFGAOSTR_BROWSABLE                  L"browsable"               // SFGAO_BROWSABLE
#define SFGAOSTR_SYSTEM                     L"system"               // SFGAO_SYSTEM
#define SFGAOSTR_PLACEHOLDER                L"placeholder"               // SFGAO_PLACEHOLDER

//  Name:     System.StatusBarSelectedItemCount -- PKEY_StatusBarSelectedItemCount
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {26DC287C-6E3D-4BD3-B2B0-6A26BA2E346D}, 3
//
//  Count of selected items in the view and estimated total size
DEFINE_PROPERTYKEY(PKEY_StatusBarSelectedItemCount, 0x26DC287C, 0x6E3D, 0x4BD3, 0xB2, 0xB0, 0x6A, 0x26, 0xBA, 0x2E, 0x34, 0x6D, 3);
#define INIT_PKEY_StatusBarSelectedItemCount { { 0x26DC287C, 0x6E3D, 0x4BD3, 0xB2, 0xB0, 0x6A, 0x26, 0xBA, 0x2E, 0x34, 0x6D }, 3 }

//  Name:     System.StatusBarViewItemCount -- PKEY_StatusBarViewItemCount
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {26DC287C-6E3D-4BD3-B2B0-6A26BA2E346D}, 2
//
//  Count of items in the view
DEFINE_PROPERTYKEY(PKEY_StatusBarViewItemCount, 0x26DC287C, 0x6E3D, 0x4BD3, 0xB2, 0xB0, 0x6A, 0x26, 0xBA, 0x2E, 0x34, 0x6D, 2);
#define INIT_PKEY_StatusBarViewItemCount { { 0x26DC287C, 0x6E3D, 0x4BD3, 0xB2, 0xB0, 0x6A, 0x26, 0xBA, 0x2E, 0x34, 0x6D }, 2 }

//  Name:     System.StorageProviderState -- PKEY_StorageProviderState
//  Type:     UInt32 -- VT_UI4
//  FormatID: {E77E90DF-6271-4F5B-834F-2DD1F245DDA4}, 3
//
//  Property for the cloud file state icon.
DEFINE_PROPERTYKEY(PKEY_StorageProviderState, 0xE77E90DF, 0x6271, 0x4F5B, 0x83, 0x4F, 0x2D, 0xD1, 0xF2, 0x45, 0xDD, 0xA4, 3);
#define INIT_PKEY_StorageProviderState { { 0xE77E90DF, 0x6271, 0x4F5B, 0x83, 0x4F, 0x2D, 0xD1, 0xF2, 0x45, 0xDD, 0xA4 }, 3 }

// Possible discrete values for PKEY_StorageProviderState are:
#define STORAGEPROVIDERSTATE_NONE           0ul
#define STORAGEPROVIDERSTATE_SPARSE         1ul
#define STORAGEPROVIDERSTATE_IN_SYNC        2ul
#define STORAGEPROVIDERSTATE_PINNED         3ul
#define STORAGEPROVIDERSTATE_PENDING_UPLOAD 4ul
#define STORAGEPROVIDERSTATE_PENDING_DOWNLOAD 5ul
#define STORAGEPROVIDERSTATE_TRANSFERRING   6ul
#define STORAGEPROVIDERSTATE_ERROR          7ul
#define STORAGEPROVIDERSTATE_WARNING        8ul
#define STORAGEPROVIDERSTATE_EXCLUDED       9ul
#define STORAGEPROVIDERSTATE_PENDING_UNSPECIFIED 10ul

//  Name:     System.StorageProviderTransferProgress -- PKEY_StorageProviderTransferProgress
//  Type:     Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
//  FormatID: {E77E90DF-6271-4F5B-834F-2DD1F245DDA4}, 4
//
//  An array of two UInt32 values (max value, current value).
DEFINE_PROPERTYKEY(PKEY_StorageProviderTransferProgress, 0xE77E90DF, 0x6271, 0x4F5B, 0x83, 0x4F, 0x2D, 0xD1, 0xF2, 0x45, 0xDD, 0xA4, 4);
#define INIT_PKEY_StorageProviderTransferProgress { { 0xE77E90DF, 0x6271, 0x4F5B, 0x83, 0x4F, 0x2D, 0xD1, 0xF2, 0x45, 0xDD, 0xA4 }, 4 }

//  Name:     System.WebAccountID -- PKEY_WebAccountID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {30C8EEF4-A832-41E2-AB32-E3C3CA28FD29}, 7
DEFINE_PROPERTYKEY(PKEY_WebAccountID, 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29, 7);
#define INIT_PKEY_WebAccountID { { 0x30C8EEF4, 0xA832, 0x41E2, 0xAB, 0x32, 0xE3, 0xC3, 0xCA, 0x28, 0xFD, 0x29 }, 7 }

//-----------------------------------------------------------------------------
// Software properties

//  Name:     System.AppUserModel.ExcludeFromShowInNewInstall -- PKEY_AppUserModel_ExcludeFromShowInNewInstall
//  Type:     Boolean -- VT_BOOL
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 8
DEFINE_PROPERTYKEY(PKEY_AppUserModel_ExcludeFromShowInNewInstall, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 8);
#define INIT_PKEY_AppUserModel_ExcludeFromShowInNewInstall { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 8 }

//  Name:     System.AppUserModel.ID -- PKEY_AppUserModel_ID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 5
DEFINE_PROPERTYKEY(PKEY_AppUserModel_ID, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 5);
#define INIT_PKEY_AppUserModel_ID { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 5 }

//  Name:     System.AppUserModel.IsDestListSeparator -- PKEY_AppUserModel_IsDestListSeparator
//  Type:     Boolean -- VT_BOOL
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 6
DEFINE_PROPERTYKEY(PKEY_AppUserModel_IsDestListSeparator, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 6);
#define INIT_PKEY_AppUserModel_IsDestListSeparator { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 6 }

//  Name:     System.AppUserModel.IsDualMode -- PKEY_AppUserModel_IsDualMode
//  Type:     Boolean -- VT_BOOL
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 11
//  
//  Deprecated. Indicates that an application supports dual desktop and immersive modes. In Windows 8, this property is only applicable for web browsers.
DEFINE_PROPERTYKEY(PKEY_AppUserModel_IsDualMode, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 11);
#define INIT_PKEY_AppUserModel_IsDualMode { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 11 }

//  Name:     System.AppUserModel.PreventPinning -- PKEY_AppUserModel_PreventPinning
//  Type:     Boolean -- VT_BOOL
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 9
DEFINE_PROPERTYKEY(PKEY_AppUserModel_PreventPinning, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 9);
#define INIT_PKEY_AppUserModel_PreventPinning { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 9 }

//  Name:     System.AppUserModel.RelaunchCommand -- PKEY_AppUserModel_RelaunchCommand
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 2
DEFINE_PROPERTYKEY(PKEY_AppUserModel_RelaunchCommand, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 2);
#define INIT_PKEY_AppUserModel_RelaunchCommand { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 2 }

//  Name:     System.AppUserModel.RelaunchDisplayNameResource -- PKEY_AppUserModel_RelaunchDisplayNameResource
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 4
DEFINE_PROPERTYKEY(PKEY_AppUserModel_RelaunchDisplayNameResource, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 4);
#define INIT_PKEY_AppUserModel_RelaunchDisplayNameResource { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 4 }

//  Name:     System.AppUserModel.RelaunchIconResource -- PKEY_AppUserModel_RelaunchIconResource
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 3
DEFINE_PROPERTYKEY(PKEY_AppUserModel_RelaunchIconResource, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 3);
#define INIT_PKEY_AppUserModel_RelaunchIconResource { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 3 }

//  Name:     System.AppUserModel.SettingsCommand -- PKEY_AppUserModel_SettingsCommand
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 38
//  
//  A command line that includes a fully qualified file path, which can include environment variables and arguments, that will launch the settings application corresponding to the shortcut (.lnk file) this property is included in.
//  This property may be used by components such as the Start Menu to provide a settings verb.
//  Example: "%ProgramFiles%\My App\Settings.exe"
//  Example: "%ProgramFiles%\My App\My-app.exe" /settings
DEFINE_PROPERTYKEY(PKEY_AppUserModel_SettingsCommand, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 38);
#define INIT_PKEY_AppUserModel_SettingsCommand { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 38 }

//  Name:     System.AppUserModel.StartPinOption -- PKEY_AppUserModel_StartPinOption
//  Type:     UInt32 -- VT_UI4
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 12
//  
//  Set this property on a shortcut to (1) prevent an application from being automatically pinned to Start screen upon installation; or
//  (2) indicate that an item is programmatically added to launcher via user action (which implies automatically pin to Start and delete on unpin).
DEFINE_PROPERTYKEY(PKEY_AppUserModel_StartPinOption, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 12);
#define INIT_PKEY_AppUserModel_StartPinOption { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 12 }

// Possible discrete values for PKEY_AppUserModel_StartPinOption are:
#define APPUSERMODEL_STARTPINOPTION_DEFAULT 0ul
#define APPUSERMODEL_STARTPINOPTION_NOPINONINSTALL 1ul
#define APPUSERMODEL_STARTPINOPTION_USERPINNED 2ul

//  Name:     System.AppUserModel.ToastActivatorCLSID -- PKEY_AppUserModel_ToastActivatorCLSID
//  Type:     Guid -- VT_CLSID
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 26
//  
//  Used to CoCreate an INotificationActivationCallback interface to notify about toast activations.
DEFINE_PROPERTYKEY(PKEY_AppUserModel_ToastActivatorCLSID, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 26);
#define INIT_PKEY_AppUserModel_ToastActivatorCLSID { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 26 }

//  Name:     System.AppUserModel.UninstallCommand -- PKEY_AppUserModel_UninstallCommand
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 37
//  
//  A command line that includes a fully qualified file path, which can include environment variables and arguments, that when launched will uninstall the application corresponding to the shortcut (.lnk file) this property is included in.
//  This property may be used by components such as the Start Menu to provide an uninstall verb.
//  The command should execute without displaying UI, as this avoids dismissing the Start Menu, but it may show UI if needed.
//  Example: %SystemRoot%\system32\msiexec.exe /x {A7654BA2-D4AB-4510-AADF-253EA74869C5} /q
//  Example: "%ProgramFiles%\My App\Uninstall.exe" /q /f
DEFINE_PROPERTYKEY(PKEY_AppUserModel_UninstallCommand, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 37);
#define INIT_PKEY_AppUserModel_UninstallCommand { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 37 }

//  Name:     System.AppUserModel.VisualElementsManifestHintPath -- PKEY_AppUserModel_VisualElementsManifestHintPath
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}, 31
//  
//  Suggests where to look for the VisualElementsManifest for a Win32 app
DEFINE_PROPERTYKEY(PKEY_AppUserModel_VisualElementsManifestHintPath, 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3, 31);
#define INIT_PKEY_AppUserModel_VisualElementsManifestHintPath { { 0x9F4C2855, 0x9F79, 0x4B39, 0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3 }, 31 }

//  Name:     System.EdgeGesture.DisableTouchWhenFullscreen -- PKEY_EdgeGesture_DisableTouchWhenFullscreen
//  Type:     Boolean -- VT_BOOL
//  FormatID: {32CE38B2-2C9A-41B1-9BC5-B3784394AA44}, 2
DEFINE_PROPERTYKEY(PKEY_EdgeGesture_DisableTouchWhenFullscreen, 0x32CE38B2, 0x2C9A, 0x41B1, 0x9B, 0xC5, 0xB3, 0x78, 0x43, 0x94, 0xAA, 0x44, 2);
#define INIT_PKEY_EdgeGesture_DisableTouchWhenFullscreen { { 0x32CE38B2, 0x2C9A, 0x41B1, 0x9B, 0xC5, 0xB3, 0x78, 0x43, 0x94, 0xAA, 0x44 }, 2 }

//  Name:     System.Software.DateLastUsed -- PKEY_Software_DateLastUsed
//  Type:     DateTime -- VT_FILETIME  (For variants: VT_DATE)
//  FormatID: {841E4F90-FF59-4D16-8947-E81BBFFAB36D}, 16
//  
//  
DEFINE_PROPERTYKEY(PKEY_Software_DateLastUsed, 0x841E4F90, 0xFF59, 0x4D16, 0x89, 0x47, 0xE8, 0x1B, 0xBF, 0xFA, 0xB3, 0x6D, 16);
#define INIT_PKEY_Software_DateLastUsed { { 0x841E4F90, 0xFF59, 0x4D16, 0x89, 0x47, 0xE8, 0x1B, 0xBF, 0xFA, 0xB3, 0x6D }, 16 }

//  Name:     System.Software.ProductName -- PKEY_Software_ProductName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (PSFMTID_VERSION) {0CEF7D53-FA64-11D1-A203-0000F81FEDEE}, 7
//  
//  
DEFINE_PROPERTYKEY(PKEY_Software_ProductName, 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE, 7);
#define INIT_PKEY_Software_ProductName { { 0x0CEF7D53, 0xFA64, 0x11D1, 0xA2, 0x03, 0x00, 0x00, 0xF8, 0x1F, 0xED, 0xEE }, 7 }

//-----------------------------------------------------------------------------
// Sync properties

//  Name:     System.Sync.Comments -- PKEY_Sync_Comments
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {7BD5533E-AF15-44DB-B8C8-BD6624E1D032}, 13
DEFINE_PROPERTYKEY(PKEY_Sync_Comments, 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32, 13);
#define INIT_PKEY_Sync_Comments { { 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32 }, 13 }

//  Name:     System.Sync.ConflictDescription -- PKEY_Sync_ConflictDescription
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {CE50C159-2FB8-41FD-BE68-D3E042E274BC}, 4
DEFINE_PROPERTYKEY(PKEY_Sync_ConflictDescription, 0xCE50C159, 0x2FB8, 0x41FD, 0xBE, 0x68, 0xD3, 0xE0, 0x42, 0xE2, 0x74, 0xBC, 4);
#define INIT_PKEY_Sync_ConflictDescription { { 0xCE50C159, 0x2FB8, 0x41FD, 0xBE, 0x68, 0xD3, 0xE0, 0x42, 0xE2, 0x74, 0xBC }, 4 }

//  Name:     System.Sync.ConflictFirstLocation -- PKEY_Sync_ConflictFirstLocation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {CE50C159-2FB8-41FD-BE68-D3E042E274BC}, 6
DEFINE_PROPERTYKEY(PKEY_Sync_ConflictFirstLocation, 0xCE50C159, 0x2FB8, 0x41FD, 0xBE, 0x68, 0xD3, 0xE0, 0x42, 0xE2, 0x74, 0xBC, 6);
#define INIT_PKEY_Sync_ConflictFirstLocation { { 0xCE50C159, 0x2FB8, 0x41FD, 0xBE, 0x68, 0xD3, 0xE0, 0x42, 0xE2, 0x74, 0xBC }, 6 }

//  Name:     System.Sync.ConflictSecondLocation -- PKEY_Sync_ConflictSecondLocation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {CE50C159-2FB8-41FD-BE68-D3E042E274BC}, 7
DEFINE_PROPERTYKEY(PKEY_Sync_ConflictSecondLocation, 0xCE50C159, 0x2FB8, 0x41FD, 0xBE, 0x68, 0xD3, 0xE0, 0x42, 0xE2, 0x74, 0xBC, 7);
#define INIT_PKEY_Sync_ConflictSecondLocation { { 0xCE50C159, 0x2FB8, 0x41FD, 0xBE, 0x68, 0xD3, 0xE0, 0x42, 0xE2, 0x74, 0xBC }, 7 }

//  Name:     System.Sync.HandlerCollectionID -- PKEY_Sync_HandlerCollectionID
//  Type:     Guid -- VT_CLSID
//  FormatID: {7BD5533E-AF15-44DB-B8C8-BD6624E1D032}, 2
DEFINE_PROPERTYKEY(PKEY_Sync_HandlerCollectionID, 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32, 2);
#define INIT_PKEY_Sync_HandlerCollectionID { { 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32 }, 2 }

//  Name:     System.Sync.HandlerID -- PKEY_Sync_HandlerID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {7BD5533E-AF15-44DB-B8C8-BD6624E1D032}, 3
DEFINE_PROPERTYKEY(PKEY_Sync_HandlerID, 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32, 3);
#define INIT_PKEY_Sync_HandlerID { { 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32 }, 3 }

//  Name:     System.Sync.HandlerName -- PKEY_Sync_HandlerName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {CE50C159-2FB8-41FD-BE68-D3E042E274BC}, 2
DEFINE_PROPERTYKEY(PKEY_Sync_HandlerName, 0xCE50C159, 0x2FB8, 0x41FD, 0xBE, 0x68, 0xD3, 0xE0, 0x42, 0xE2, 0x74, 0xBC, 2);
#define INIT_PKEY_Sync_HandlerName { { 0xCE50C159, 0x2FB8, 0x41FD, 0xBE, 0x68, 0xD3, 0xE0, 0x42, 0xE2, 0x74, 0xBC }, 2 }

//  Name:     System.Sync.HandlerType -- PKEY_Sync_HandlerType
//  Type:     UInt32 -- VT_UI4
//  FormatID: {7BD5533E-AF15-44DB-B8C8-BD6624E1D032}, 8
//  
//  
DEFINE_PROPERTYKEY(PKEY_Sync_HandlerType, 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32, 8);
#define INIT_PKEY_Sync_HandlerType { { 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32 }, 8 }

// Possible discrete values for PKEY_Sync_HandlerType are:
#define SYNC_HANDLERTYPE_OTHER              0ul
#define SYNC_HANDLERTYPE_PROGRAMS           1ul
#define SYNC_HANDLERTYPE_DEVICES            2ul
#define SYNC_HANDLERTYPE_FOLDERS            3ul
#define SYNC_HANDLERTYPE_WEBSERVICES        4ul
#define SYNC_HANDLERTYPE_COMPUTERS          5ul

//  Name:     System.Sync.HandlerTypeLabel -- PKEY_Sync_HandlerTypeLabel
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {7BD5533E-AF15-44DB-B8C8-BD6624E1D032}, 9
//  
//  
DEFINE_PROPERTYKEY(PKEY_Sync_HandlerTypeLabel, 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32, 9);
#define INIT_PKEY_Sync_HandlerTypeLabel { { 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32 }, 9 }

//  Name:     System.Sync.ItemID -- PKEY_Sync_ItemID
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {7BD5533E-AF15-44DB-B8C8-BD6624E1D032}, 6
DEFINE_PROPERTYKEY(PKEY_Sync_ItemID, 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32, 6);
#define INIT_PKEY_Sync_ItemID { { 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32 }, 6 }

//  Name:     System.Sync.ItemName -- PKEY_Sync_ItemName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {CE50C159-2FB8-41FD-BE68-D3E042E274BC}, 3
DEFINE_PROPERTYKEY(PKEY_Sync_ItemName, 0xCE50C159, 0x2FB8, 0x41FD, 0xBE, 0x68, 0xD3, 0xE0, 0x42, 0xE2, 0x74, 0xBC, 3);
#define INIT_PKEY_Sync_ItemName { { 0xCE50C159, 0x2FB8, 0x41FD, 0xBE, 0x68, 0xD3, 0xE0, 0x42, 0xE2, 0x74, 0xBC }, 3 }

//  Name:     System.Sync.ProgressPercentage -- PKEY_Sync_ProgressPercentage
//  Type:     UInt32 -- VT_UI4
//  FormatID: {7BD5533E-AF15-44DB-B8C8-BD6624E1D032}, 23
//
//  An integer value between 0 and 100 representing the percentage completed.
DEFINE_PROPERTYKEY(PKEY_Sync_ProgressPercentage, 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32, 23);
#define INIT_PKEY_Sync_ProgressPercentage { { 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32 }, 23 }

//  Name:     System.Sync.State -- PKEY_Sync_State
//  Type:     UInt32 -- VT_UI4
//  FormatID: {7BD5533E-AF15-44DB-B8C8-BD6624E1D032}, 24
//
//  Sync state.
DEFINE_PROPERTYKEY(PKEY_Sync_State, 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32, 24);
#define INIT_PKEY_Sync_State { { 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32 }, 24 }

// Possible discrete values for PKEY_Sync_State are:
#define SYNC_STATE_NOTSETUP                 0ul
#define SYNC_STATE_SYNCNOTRUN               1ul
#define SYNC_STATE_IDLE                     2ul
#define SYNC_STATE_ERROR                    3ul
#define SYNC_STATE_PENDING                  4ul
#define SYNC_STATE_SYNCING                  5ul

//  Name:     System.Sync.Status -- PKEY_Sync_Status
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {7BD5533E-AF15-44DB-B8C8-BD6624E1D032}, 10
DEFINE_PROPERTYKEY(PKEY_Sync_Status, 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32, 10);
#define INIT_PKEY_Sync_Status { { 0x7BD5533E, 0xAF15, 0x44DB, 0xB8, 0xC8, 0xBD, 0x66, 0x24, 0xE1, 0xD0, 0x32 }, 10 }

//-----------------------------------------------------------------------------
// Task properties

//  Name:     System.Task.BillingInformation -- PKEY_Task_BillingInformation
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {D37D52C6-261C-4303-82B3-08B926AC6F12}, 100
DEFINE_PROPERTYKEY(PKEY_Task_BillingInformation, 0xD37D52C6, 0x261C, 0x4303, 0x82, 0xB3, 0x08, 0xB9, 0x26, 0xAC, 0x6F, 0x12, 100);
#define INIT_PKEY_Task_BillingInformation { { 0xD37D52C6, 0x261C, 0x4303, 0x82, 0xB3, 0x08, 0xB9, 0x26, 0xAC, 0x6F, 0x12 }, 100 }

//  Name:     System.Task.CompletionStatus -- PKEY_Task_CompletionStatus
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {084D8A0A-E6D5-40DE-BF1F-C8820E7C877C}, 100
DEFINE_PROPERTYKEY(PKEY_Task_CompletionStatus, 0x084D8A0A, 0xE6D5, 0x40DE, 0xBF, 0x1F, 0xC8, 0x82, 0x0E, 0x7C, 0x87, 0x7C, 100);
#define INIT_PKEY_Task_CompletionStatus { { 0x084D8A0A, 0xE6D5, 0x40DE, 0xBF, 0x1F, 0xC8, 0x82, 0x0E, 0x7C, 0x87, 0x7C }, 100 }

//  Name:     System.Task.Owner -- PKEY_Task_Owner
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: {08C7CC5F-60F2-4494-AD75-55E3E0B5ADD0}, 100
DEFINE_PROPERTYKEY(PKEY_Task_Owner, 0x08C7CC5F, 0x60F2, 0x4494, 0xAD, 0x75, 0x55, 0xE3, 0xE0, 0xB5, 0xAD, 0xD0, 100);
#define INIT_PKEY_Task_Owner { { 0x08C7CC5F, 0x60F2, 0x4494, 0xAD, 0x75, 0x55, 0xE3, 0xE0, 0xB5, 0xAD, 0xD0 }, 100 }

//-----------------------------------------------------------------------------
// Video properties

//  Name:     System.Video.Compression -- PKEY_Video_Compression
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 10 (PIDVSI_COMPRESSION)
//
//  Indicates the level of compression for the video stream.  "Compression".
DEFINE_PROPERTYKEY(PKEY_Video_Compression, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 10);
#define INIT_PKEY_Video_Compression { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 10 }

//  Name:     System.Video.Director -- PKEY_Video_Director
//  Type:     Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
//  FormatID: (PSGUID_MEDIAFILESUMMARYINFORMATION) {64440492-4C8B-11D1-8B70-080036B11A03}, 20 (PIDMSI_DIRECTOR)
//
//  
DEFINE_PROPERTYKEY(PKEY_Video_Director, 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 20);
#define INIT_PKEY_Video_Director { { 0x64440492, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 20 }

//  Name:     System.Video.EncodingBitrate -- PKEY_Video_EncodingBitrate
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 8 (PIDVSI_DATA_RATE)
//
//  Indicates the data rate in "bits per second" for the video stream. "DataRate".
DEFINE_PROPERTYKEY(PKEY_Video_EncodingBitrate, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 8);
#define INIT_PKEY_Video_EncodingBitrate { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 8 }

//  Name:     System.Video.FourCC -- PKEY_Video_FourCC
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 44
//  
//  Indicates the 4CC for the video stream.
DEFINE_PROPERTYKEY(PKEY_Video_FourCC, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 44);
#define INIT_PKEY_Video_FourCC { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 44 }

//  Name:     System.Video.FrameHeight -- PKEY_Video_FrameHeight
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 4
//
//  Indicates the frame height for the video stream.
DEFINE_PROPERTYKEY(PKEY_Video_FrameHeight, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 4);
#define INIT_PKEY_Video_FrameHeight { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 4 }

//  Name:     System.Video.FrameRate -- PKEY_Video_FrameRate
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 6 (PIDVSI_FRAME_RATE)
//
//  Indicates the frame rate in "frames per millisecond" for the video stream.  "FrameRate".
DEFINE_PROPERTYKEY(PKEY_Video_FrameRate, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 6);
#define INIT_PKEY_Video_FrameRate { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 6 }

//  Name:     System.Video.FrameWidth -- PKEY_Video_FrameWidth
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 3
//
//  Indicates the frame width for the video stream.
DEFINE_PROPERTYKEY(PKEY_Video_FrameWidth, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 3);
#define INIT_PKEY_Video_FrameWidth { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 3 }

//  Name:     System.Video.HorizontalAspectRatio -- PKEY_Video_HorizontalAspectRatio
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 42
//  
//  Indicates the horizontal portion of the aspect ratio. The X portion of XX:YY,
//  like 16:9.
DEFINE_PROPERTYKEY(PKEY_Video_HorizontalAspectRatio, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 42);
#define INIT_PKEY_Video_HorizontalAspectRatio { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 42 }

//  Name:     System.Video.IsSpherical -- PKEY_Video_IsSpherical
//  Type:     Boolean -- VT_BOOL
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 100
//
//  Indicates whether the media file has a spherical video stream.
DEFINE_PROPERTYKEY(PKEY_Video_IsSpherical, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 100);
#define INIT_PKEY_Video_IsSpherical { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 100 }

//  Name:     System.Video.IsStereo -- PKEY_Video_IsStereo
//  Type:     Boolean -- VT_BOOL
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 98
//
//  Indicates whether the media file has a stereo video stream.
DEFINE_PROPERTYKEY(PKEY_Video_IsStereo, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 98);
#define INIT_PKEY_Video_IsStereo { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 98 }

//  Name:     System.Video.Orientation -- PKEY_Video_Orientation
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 99
//  
//  This is the video orientation in degrees.
DEFINE_PROPERTYKEY(PKEY_Video_Orientation, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 99);
#define INIT_PKEY_Video_Orientation { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 99 }

//  Name:     System.Video.SampleSize -- PKEY_Video_SampleSize
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 9 (PIDVSI_SAMPLE_SIZE)
//
//  Indicates the sample size in bits for the video stream.  "SampleSize".
DEFINE_PROPERTYKEY(PKEY_Video_SampleSize, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 9);
#define INIT_PKEY_Video_SampleSize { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 9 }

//  Name:     System.Video.StreamName -- PKEY_Video_StreamName
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 2 (PIDVSI_STREAM_NAME)
//
//  Indicates the name for the video stream. "StreamName".
DEFINE_PROPERTYKEY(PKEY_Video_StreamName, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 2);
#define INIT_PKEY_Video_StreamName { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 2 }

//  Name:     System.Video.StreamNumber -- PKEY_Video_StreamNumber
//  Type:     UInt16 -- VT_UI2
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 11 (PIDVSI_STREAM_NUMBER)
//
//  "Stream Number".
DEFINE_PROPERTYKEY(PKEY_Video_StreamNumber, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 11);
#define INIT_PKEY_Video_StreamNumber { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 11 }

//  Name:     System.Video.TotalBitrate -- PKEY_Video_TotalBitrate
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 43 (PIDVSI_TOTAL_BITRATE)
//
//  Indicates the total data rate in "bits per second" for all video and audio streams.
DEFINE_PROPERTYKEY(PKEY_Video_TotalBitrate, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 43);
#define INIT_PKEY_Video_TotalBitrate { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 43 }

//  Name:     System.Video.TranscodedForSync -- PKEY_Video_TranscodedForSync
//  Type:     Boolean -- VT_BOOL
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 46
DEFINE_PROPERTYKEY(PKEY_Video_TranscodedForSync, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 46);
#define INIT_PKEY_Video_TranscodedForSync { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 46 }

//  Name:     System.Video.VerticalAspectRatio -- PKEY_Video_VerticalAspectRatio
//  Type:     UInt32 -- VT_UI4
//  FormatID: (FMTID_VideoSummaryInformation) {64440491-4C8B-11D1-8B70-080036B11A03}, 45
//  
//  Indicates the vertical portion of the aspect ratio. The Y portion of 
//  XX:YY, like 16:9.
DEFINE_PROPERTYKEY(PKEY_Video_VerticalAspectRatio, 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03, 45);
#define INIT_PKEY_Video_VerticalAspectRatio { { 0x64440491, 0x4C8B, 0x11D1, 0x8B, 0x70, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03 }, 45 }

//-----------------------------------------------------------------------------
// Volume properties

//  Name:     System.Volume.FileSystem -- PKEY_Volume_FileSystem
//  Type:     String -- VT_LPWSTR  (For variants: VT_BSTR)
//  FormatID: (FMTID_Volume) {9B174B35-40FF-11D2-A27E-00C04FC30871}, 4 (PID_VOLUME_FILESYSTEM)  (Filesystem Volume Properties)
//
//  Indicates the filesystem of the volume.
DEFINE_PROPERTYKEY(PKEY_Volume_FileSystem, 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71, 4);
#define INIT_PKEY_Volume_FileSystem { { 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71 }, 4 }

//  Name:     System.Volume.IsMappedDrive -- PKEY_Volume_IsMappedDrive
//  Type:     Boolean -- VT_BOOL
//  FormatID: {149C0B69-2C2D-48FC-808F-D318D78C4636}, 2
DEFINE_PROPERTYKEY(PKEY_Volume_IsMappedDrive, 0x149C0B69, 0x2C2D, 0x48FC, 0x80, 0x8F, 0xD3, 0x18, 0xD7, 0x8C, 0x46, 0x36, 2);
#define INIT_PKEY_Volume_IsMappedDrive { { 0x149C0B69, 0x2C2D, 0x48FC, 0x80, 0x8F, 0xD3, 0x18, 0xD7, 0x8C, 0x46, 0x36 }, 2 }

//  Name:     System.Volume.IsRoot -- PKEY_Volume_IsRoot
//  Type:     Boolean -- VT_BOOL
//  FormatID: (FMTID_Volume) {9B174B35-40FF-11D2-A27E-00C04FC30871}, 10  (Filesystem Volume Properties)
//
//  
DEFINE_PROPERTYKEY(PKEY_Volume_IsRoot, 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71, 10);
#define INIT_PKEY_Volume_IsRoot { { 0x9B174B35, 0x40FF, 0x11D2, 0xA2, 0x7E, 0x00, 0xC0, 0x4F, 0xC3, 0x08, 0x71 }, 10 }

//-----------------------------------------------------------------------------
// Deprecated property key names

#define PKEY_Devices_Category_Desc_Singular            PKEY_Devices_Category
#define PKEY_Devices_CategoryGroup_Desc                PKEY_Devices_CategoryGroup
#define PKEY_Devices_Category_Desc_Plural              PKEY_Devices_CategoryPlural
#define PKEY_Devices_IsConnected                       PKEY_Devices_Connected
#define PKEY_Devices_IsDefaultDevice                   PKEY_Devices_IsDefault
#define PKEY_Devices_IsNetworkDevice                   PKEY_Devices_IsNetworkConnected
#define PKEY_Devices_IsSharedDevice                    PKEY_Devices_IsShared
#define PKEY_Devices_IsLocalMachine                    PKEY_Devices_LocalMachine
#define PKEY_Devices_Notification_LowBattery           PKEY_Devices_Notifications_LowBattery
#define PKEY_Devices_Notification_MissedCall           PKEY_Devices_Notifications_MissedCall
#define PKEY_Devices_Notification_NewMessage           PKEY_Devices_Notifications_NewMessage
#define PKEY_Devices_Notification_NewVoicemail         PKEY_Devices_Notifications_NewVoicemail
#define PKEY_Devices_Notification_StorageFull          PKEY_Devices_Notifications_StorageFull
#define PKEY_Devices_Notification_StorageFullLinkText  PKEY_Devices_Notifications_StorageFullLinkText
#define PKEY_Devices_IsNotWorkingProperly              PKEY_Devices_NotWorkingProperly
#define PKEY_Devices_IsPaired                          PKEY_Devices_Paired

#endif  /* _INC_PROPKEY */
