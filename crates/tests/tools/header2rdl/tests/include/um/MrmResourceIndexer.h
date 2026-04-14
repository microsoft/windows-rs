/*
* Copyright (c) Microsoft Corporation. All rights reserved.
*/

#pragma once
#ifndef MRMRESOURCEINDEXER_H
#define MRMRESOURCEINDEXER_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <windows.h>

#ifdef __cplusplus
extern "C" {
#endif

enum MrmPlatformVersion
{
    MrmPlatformVersion_Default = 0,
    MrmPlatformVersion_Windows10_0_0_0 = 0x010A0000,
    MrmPlatformVersion_Windows10_0_0_5 = 0x010A0005
};

struct MrmResourceIndexerHandle
{
    PVOID handle;
};

enum MrmPackagingMode
{
    MrmPackagingModeStandaloneFile, /* Produce a single PRI file. */
    MrmPackagingModeAutoSplit,      /* Produce multiple PRI files split automatically by all supported qualifiers. */
    MrmPackagingModeResourcePack    /* Produce an add-on satellite PRI file. */
};

enum MrmPackagingOptions
{
    MrmPackagingOptionsNone                         = 0x00,
    MrmPackagingOptionsOmitSchemaFromResourcePacks  = 0x01,
    MrmPackagingOptionsSplitLanguageVariants        = 0x02,
};

enum MrmDumpType
{
    MrmDumpType_Basic = 0,
    MrmDumpType_Detailed = 1,
    MrmDumpType_Schema = 2
};

enum MrmResourceIndexerMessageSeverity
{
    MrmResourceIndexerMessageSeverityVerbose,
    MrmResourceIndexerMessageSeverityInfo,
    MrmResourceIndexerMessageSeverityWarning,
    MrmResourceIndexerMessageSeverityError
};

enum MrmIndexerFlags
{
    MrmIndexerFlagsNone = 0x0,
    MrmIndexerFlagsAutoMerge = 0x1,
    MrmIndexerFlagsCreateContentChecksum = 0x2
};

struct MrmResourceIndexerMessage
{
    enum MrmResourceIndexerMessageSeverity severity;
    ULONG id;
    PCWSTR text;
};

/* Create resource Indexer, return handle. Example qualifiers string: L"Language-en-US_Scale-100_Contrast-standard" */
STDAPI MrmCreateResourceIndexer(
    _In_opt_ PCWSTR packageFamilyName,
    _In_ PCWSTR projectRoot,
    _In_ enum MrmPlatformVersion platformVersion,
    _In_opt_ PCWSTR defaultQualifiers,
    _Inout_ struct MrmResourceIndexerHandle* indexer);

/* Create resource Indexer from previous schema file. Example qualifiers string: L"Language-en-US_Scale-100_Contrast-standard" */
STDAPI MrmCreateResourceIndexerFromPreviousSchemaFile(
    _In_ PCWSTR projectRoot,
    _In_ enum MrmPlatformVersion platformVersion,
    _In_opt_ PCWSTR defaultQualifiers,
    _In_ PCWSTR schemaFile,
    _Inout_ struct MrmResourceIndexerHandle* indexer);

/* Create resource Indexer from previous PRI file. Example qualifiers string: L"Language-en-US_Scale-100_Contrast-standard" */
STDAPI MrmCreateResourceIndexerFromPreviousPriFile(
    _In_ PCWSTR projectRoot,
    _In_ enum MrmPlatformVersion platformVersion,
    _In_opt_ PCWSTR defaultQualifiers,
    _In_ PCWSTR priFile,
    _Inout_ struct MrmResourceIndexerHandle* indexer);

/* Create resource Indexer from previous schema file text cached in memory. Example qualifiers string: "Language-en_Scale-100_Contrast-standard" */
STDAPI MrmCreateResourceIndexerFromPreviousSchemaData(
    _In_ PCWSTR projectRoot,
    _In_ enum MrmPlatformVersion platformVersion,
    _In_opt_ PCWSTR defaultQualifiers,
    _In_reads_bytes_(schemaXmlSize) BYTE* schemaXmlData,
    _In_ ULONG schemaXmlSize,
    _Inout_ struct MrmResourceIndexerHandle* indexer);

/* Create resource Indexer from previous PRI file data. Example qualifiers string: "Language-en_Scale-100_Contrast-standard" */
STDAPI MrmCreateResourceIndexerFromPreviousPriData(
    _In_ PCWSTR projectRoot,
    _In_ enum MrmPlatformVersion platformVersion,
    _In_opt_ PCWSTR defaultQualifiers,
    _In_reads_bytes_(priSize) BYTE* priData,
    _In_ ULONG priSize,
    _Inout_ struct MrmResourceIndexerHandle* indexer);

/* Create resource Indexer with flags, return handle. Example qualifiers string: L"Language-en-US_Scale-100_Contrast-standard" */
STDAPI MrmCreateResourceIndexerWithFlags(
    _In_opt_ PCWSTR packageFamilyName,
    _In_ PCWSTR projectRoot,
    _In_ enum MrmPlatformVersion platformVersion,
    _In_opt_ PCWSTR defaultQualifiers,
    _In_ enum MrmIndexerFlags flags,
    _Inout_ struct MrmResourceIndexerHandle* indexer);

/* Index a single string resource, with qualifiers string. Eg.: L"Language-en-US_Scale-100_Contrast-standard" */
/* Empty string or nullptr for qualifiers indicate a neutral resource */
STDAPI MrmIndexString(
    _In_ struct MrmResourceIndexerHandle indexer,
    _In_ PCWSTR resourceUri,
    _In_ PCWSTR resourceString,
    _In_opt_ PCWSTR qualifiers);

/* Index a single embeddeddata resource, with qualifiers string. Eg.: "Language-en_Scale-100_Contrast-standard" */
/* Empty string or nullptr for qualifiers indicate a neutral resource */
STDAPI MrmIndexEmbeddedData(
    _In_ struct MrmResourceIndexerHandle indexer,
    _In_ PCWSTR resourceUri,
    _In_reads_bytes_(embeddedDataSize) const BYTE* embeddedData,
    _In_ ULONG embeddedDataSize,
    _In_opt_ PCWSTR qualifiers);

/* Index a single file resource, with qualifiers string. Eg.: L"Language-en-US_Scale-100_Contrast-standard". */
/* Empty string or nullptr for qualifiers indicate a neutral resource */
STDAPI MrmIndexFile(
    _In_ struct MrmResourceIndexerHandle indexer,
    _In_ PCWSTR resourceUri,
    _In_ PCWSTR filePath,
    _In_opt_ PCWSTR qualifiers);

/* Index a single file resource. Use the file name as resource name, and derive qualifiers from the path.
Eg.: "Images\en-US\scale-100\background.png" will add a resource named "background.png" and with qualifiers: "langugage-en-US" and "scale-100" */
STDAPI MrmIndexFileAutoQualifiers(
    _In_ struct MrmResourceIndexerHandle indexer,
    _In_opt_ PCWSTR filePath);

/* Index resource container: resw, resjson, priinfo, prifile. Qualifier will be derived from containerPath. */
STDAPI MrmIndexResourceContainerAutoQualifiers(
    _In_ struct MrmResourceIndexerHandle indexer,
    _In_ PCWSTR containerPath);

/* Create the PRI file on disk to the given directory, output file name will be "resources.pri" */
STDAPI MrmCreateResourceFile(
    _In_ struct MrmResourceIndexerHandle indexer,
    _In_ enum MrmPackagingMode packagingMode,
    _In_ enum MrmPackagingOptions packagingOptions,
    _In_ PCWSTR outputDirectory);

/* Create the PRI file on disk to the given directory, output file name will be "resources.pri" */
/* The PRI file will be stamped with the provided checksum */
STDAPI MrmCreateResourceFileWithChecksum(
    _In_ struct MrmResourceIndexerHandle indexer,
    _In_ enum MrmPackagingMode packagingMode,
    _In_ enum MrmPackagingOptions packagingOptions,
    _In_ ULONG checksum,
    _In_ PCWSTR outputDirectory);

/* Create the PRI file in memory, return data blob, does not support MrmPackagingModeAutoSplit */
/* Caller needs to call MrmFreeMemory to free the outputPriData */
STDAPI MrmCreateResourceFileInMemory(
    _In_ struct MrmResourceIndexerHandle indexer,
    _In_ enum MrmPackagingMode packagingMode,
    _In_ enum MrmPackagingOptions packagingOptions,
    _Outptr_result_bytebuffer_(*outputPriSize) BYTE** outputPriData,
    _Out_ ULONG* outputPriSize);

/* Peek the error message, user does not own the memory, do not free. */
STDAPI MrmPeekResourceIndexerMessages(
    _In_ struct MrmResourceIndexerHandle handle,
    _Out_writes_(*numMsgs) struct MrmResourceIndexerMessage** messages,
    _Out_ ULONG* numMsgs);

/* Destroy the handle, free the indexer, delete the messages,  All other memory should be deleted by the caller using the MrmFreeMemory API. */
STDAPI MrmDestroyIndexerAndMessages(_In_ struct MrmResourceIndexerHandle indexer);

/* Free memories generated by:
    MrmCreateResourceFileInMemory,
    MrmCreateConfigInMemory,
    MrmDumpPriFileInMemory,
    MrmDumpPriDataInMemory,
*/
STDAPI MrmFreeMemory(_In_ BYTE* data);

/* makepri dump: file -> file */
STDAPI MrmDumpPriFile(
    _In_ PCWSTR indexFileName,
    _In_opt_ PCWSTR schemaPriFile,
    _In_ enum MrmDumpType dumpType,
    _In_ PCWSTR outputXmlFile);

/* makepri dump: file -> blob */
/* Caller need to call MrmFreeMemory to free the allocated memory: outputXmlData */
STDAPI MrmDumpPriFileInMemory(
    _In_ PCWSTR indexFileName,
    _In_opt_ PCWSTR schemaPriFile,
    _In_ enum MrmDumpType dumpType,
    _Outptr_result_bytebuffer_(*outputXmlSize) BYTE** outputXmlData,
    _Out_ ULONG* outputXmlSize);

/* makepri dump: blob -> blob */
/* Caller need to call MrmFreeMemory to free the allocated memory: outputXmlData */
STDAPI MrmDumpPriDataInMemory(
    _In_reads_bytes_(inputPriSize) BYTE* inputPriData,
    _In_ ULONG inputPriSize,
    _In_reads_bytes_opt_(schemaPriSize) BYTE* schemaPriData,
    _In_ ULONG schemaPriSize,
    _In_ enum MrmDumpType dumpType,
    _Outptr_result_bytebuffer_(*outputXmlSize) BYTE** outputXmlData,
    _Out_ ULONG* outputXmlSize);

/* makepri createconfig as a file */
STDAPI MrmCreateConfig(
    _In_ enum MrmPlatformVersion platformVersion,
    _In_opt_ PCWSTR defaultQualifiers,
    _In_ PCWSTR outputXmlFile);

/* makepri createconfig as a blob */
/* Caller need to call MrmFreeMemory to free the allocated memory: outputXmlData */
STDAPI MrmCreateConfigInMemory(
    _In_ enum MrmPlatformVersion platformVersion,
    _In_opt_ PCWSTR defaultQualifiers,
    _Outptr_result_bytebuffer_(*outputXmlSize) BYTE** outputXmlData,
    _Out_ ULONG* outputXmlSize);

/* Get content checksum of a PRI file */
STDAPI MrmGetPriFileContentChecksum(
    _In_ PCWSTR priFile,
    _Out_ ULONG* checksum);

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* !MRMRESOURCEINDEXER_H */
