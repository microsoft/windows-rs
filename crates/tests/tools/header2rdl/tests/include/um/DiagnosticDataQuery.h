// Copyright (c) Microsoft Corporation

#pragma once

#include <diagnosticdataquerytypes.h>

DECLARE_HANDLE(HDIAGNOSTIC_DATA_QUERY_SESSION);
DECLARE_HANDLE(HDIAGNOSTIC_REPORT);
DECLARE_HANDLE(HDIAGNOSTIC_EVENT_TAG_DESCRIPTION);
DECLARE_HANDLE(HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION);
DECLARE_HANDLE(HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION);
DECLARE_HANDLE(HDIAGNOSTIC_RECORD);

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

// Caller is expected to destroy hSession with DdqCloseSession
STDAPI DdqCreateSession(
    _In_ DdqAccessLevel accessLevel,
    _Out_ HDIAGNOSTIC_DATA_QUERY_SESSION* hSession);

STDAPI DdqCloseSession(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession);

STDAPI DdqGetSessionAccessLevel(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _Out_ DdqAccessLevel* accessLevel);

STDAPI DdqGetDiagnosticDataAccessLevelAllowed(
    _Out_ DdqAccessLevel* accessLevel);

STDAPI DdqGetDiagnosticRecordStats(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_ DIAGNOSTIC_DATA_SEARCH_CRITERIA const* searchCriteria,
    _Out_ UINT32* recordCount,
    _Out_ INT64* minRowId,
    _Out_ INT64* maxRowId);

// Caller is expected to free payload with CoTaskMemFree()
STDAPI DdqGetDiagnosticRecordPayload(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_ INT64 rowId,
    _Out_ PCWSTR* payload);

// Invoke DdqFreeDiagnosticRecordLocaleTags() to free tagDescription
STDAPI DdqGetDiagnosticRecordLocaleTags(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_ PCWSTR locale,
    _Out_ HDIAGNOSTIC_EVENT_TAG_DESCRIPTION* hTagDescription);

STDAPI DdqFreeDiagnosticRecordLocaleTags(
    _In_ HDIAGNOSTIC_EVENT_TAG_DESCRIPTION hTagDescription);

STDAPI DdqGetDiagnosticRecordLocaleTagAtIndex(
    _In_ HDIAGNOSTIC_EVENT_TAG_DESCRIPTION hTagDescription,
    _In_ UINT32 index,
    _Out_ DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION* tagDescription);

STDAPI DdqGetDiagnosticRecordLocaleTagCount(
    _In_ HDIAGNOSTIC_EVENT_TAG_DESCRIPTION hTagDescription,
    _Out_ UINT32* tagDescriptionCount);

// Invoke DdqFreeDiagnosticRecordProducers() to free producerDescription
STDAPI DdqGetDiagnosticRecordProducers(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _Out_ HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION* hProducerDescription);

STDAPI DdqFreeDiagnosticRecordProducers(
    _In_ HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION hProducerDescription);

STDAPI DdqGetDiagnosticRecordProducerAtIndex(
    _In_ HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION hProducerDescription,
    _In_ UINT32 index,
    _Out_ DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION* producerDescription);

STDAPI DdqGetDiagnosticRecordProducerCount(
    _In_ HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION hProducerDescription,
    _Out_ UINT32* producerDescriptionCount);

// Invoke DdqFreeDiagnosticRecordProducerCategories() to free categoryDescription
STDAPI DdqGetDiagnosticRecordProducerCategories(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_ PCWSTR producerName,
    _Out_ HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION* hCategoryDescription);

STDAPI DdqFreeDiagnosticRecordProducerCategories(
    _In_ HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION hCategoryDescription);

STDAPI DdqGetDiagnosticRecordCategoryAtIndex(
    _In_ HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION hCategoryDescription,
    _In_ UINT32 index,
    _Out_ DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION* categoryDescription);

STDAPI DdqGetDiagnosticRecordCategoryCount(
    _In_ HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION hCategoryDescription,
    _Out_ UINT32* categoryDescriptionCount);

STDAPI DdqIsDiagnosticRecordSampledIn(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_ const GUID* providerGroup,
    _In_opt_ const GUID* providerId,
    _In_ PCWSTR providerName,
    _In_opt_ const UINT32* eventId,
    _In_ PCWSTR eventName,
    _In_opt_ const UINT32* eventVersion,
    _In_opt_ const UINT64* eventKeywords,
    _Out_ BOOL* isSampledIn);

// Invoke DdqFreeDiagnosticRecordPage() to free record
STDAPI DdqGetDiagnosticRecordPage(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_ DIAGNOSTIC_DATA_SEARCH_CRITERIA* const searchCriteria,
    _In_ UINT32 offset,
    _In_ UINT32 pageRecordCount,
    _In_ INT64 baseRowId,
    _Out_ HDIAGNOSTIC_RECORD* hRecord);

STDAPI DdqFreeDiagnosticRecordPage(
    _In_ HDIAGNOSTIC_RECORD hRecord);

STDAPI DdqGetDiagnosticRecordAtIndex(
    _In_ HDIAGNOSTIC_RECORD hRecord,
    _In_ UINT32 index,
    _Out_ DIAGNOSTIC_DATA_RECORD* record);

STDAPI DdqGetDiagnosticRecordCount(
    _In_ HDIAGNOSTIC_RECORD hRecord,
    _Out_ UINT32* recordCount);

STDAPI DdqGetDiagnosticReportStoreReportCount(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_ UINT32 reportStoreType,
    _Out_ UINT32* reportCount);

STDAPI DdqCancelDiagnosticRecordOperation(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession);

// Invoke DdqFreeDiagnosticReport() to free report
STDAPI DdqGetDiagnosticReport(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_ UINT32 reportStoreType,
    _Out_ HDIAGNOSTIC_REPORT* hReport);

STDAPI DdqFreeDiagnosticReport(
    _In_ HDIAGNOSTIC_REPORT hReport);

STDAPI DdqGetDiagnosticReportAtIndex(
    _In_ HDIAGNOSTIC_REPORT hReport,
    _In_ UINT32 index,
    _Out_ DIAGNOSTIC_REPORT_DATA* report);

STDAPI DdqGetDiagnosticReportCount(
    _In_ HDIAGNOSTIC_REPORT hReport,
    _Out_ UINT32* reportCount);

STDAPI DdqExtractDiagnosticReport(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_ UINT32 reportStoreType,
    _In_ PCWSTR reportKey,
    _In_ PCWSTR destinationPath);

// Caller is expected to free tagStats with CoTaskMemFree()
STDAPI DdqGetDiagnosticRecordTagDistribution(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_reads_(producerNameCount) PCWSTR* producerNames,
    _In_ UINT32 producerNameCount,
    _Outptr_result_buffer_all_(*statCount) DIAGNOSTIC_DATA_EVENT_TAG_STATS** tagStats,
    _Out_ UINT32* statCount);

// Caller is expected to free binaryStats with CoTaskMemFree()
STDAPI DdqGetDiagnosticRecordBinaryDistribution(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_reads_(producerNameCount) PCWSTR* producerNames,
    _In_ UINT32 producerNameCount,
    _In_ UINT32 topNBinaries,
    _Outptr_result_buffer_all_(*statCount) DIAGNOSTIC_DATA_EVENT_BINARY_STATS** binaryStats,
    _Out_ UINT32* statCount);

STDAPI DdqGetDiagnosticRecordSummary(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_reads_(producerNameCount) const PCWSTR* producerNames,
    _In_ UINT32 producerNameCount,
    _Out_ DIAGNOSTIC_DATA_GENERAL_STATS* generalStats);

STDAPI DdqSetTranscriptConfiguration(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _In_ const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION* desiredConfig);

STDAPI DdqGetTranscriptConfiguration(
    _In_ HDIAGNOSTIC_DATA_QUERY_SESSION hSession,
    _Out_ DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION* currentConfig);

#ifdef __cplusplus
}
#endif //__cplusplus
