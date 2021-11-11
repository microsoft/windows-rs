#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ActivateActCtx();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddRefActCtx();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaB();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaGetReverseB();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaProvidedB();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByBuffers();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByHandles();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByHandlesEx();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateActCtxA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateActCtxW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaB();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileByHandles();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileByHandlesEx();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeactivateActCtx();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeltaFree();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeltaNormalizeProvidedB();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileByHandles();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionGuid();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionStringA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionStringW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentActCtx();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoB();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureB();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureByBuffer();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureByHandle();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiAdvertiseScriptA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiAdvertiseScriptW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyMultiplePatchesA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyMultiplePatchesW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyPatchA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyPatchW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiBeginTransactionA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiBeginTransactionW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiCloseAllHandles();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiCloseHandle();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCollectUserInfoA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCollectUserInfoW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureFeatureA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureFeatureW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiCreateRecord();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCreateTransformSummaryInfoA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCreateTransformSummaryInfoW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseApplyTransformA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseApplyTransformW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiDatabaseCommit();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseExportA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseExportW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGenerateTransformA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGenerateTransformW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGetPrimaryKeysA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGetPrimaryKeysW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseImportA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseImportW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseIsTablePersistentA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseIsTablePersistentW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseMergeA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseMergeW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseOpenViewA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseOpenViewW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDetermineApplicablePatchesA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDetermineApplicablePatchesW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDeterminePatchSequenceA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDeterminePatchSequenceW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDoActionA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDoActionW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnableLogA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnableLogW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiEnableUIPreview();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiEndTransaction();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentCostsA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentCostsW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentQualifiersA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentQualifiersW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumFeaturesA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumFeaturesW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumRelatedProductsA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumRelatedProductsW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEvaluateConditionA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEvaluateConditionW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiExtractPatchXMLDataA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiExtractPatchXMLDataW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiFormatRecordA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiFormatRecordW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiGetActiveDatabase();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentStateA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentStateW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiGetDatabaseState();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureCostA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureCostW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureInfoA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureInfoW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureStateA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureStateW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureUsageA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureUsageW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureValidStatesA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureValidStatesW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileHashA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileHashW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MsiGetFileSignatureInformationA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MsiGetFileSignatureInformationW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileVersionA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileVersionW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiGetLanguage();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiGetLastErrorRecord();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetMode();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchFileListA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchFileListW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductCodeA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductCodeW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoFromScriptA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoFromScriptW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductPropertyA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductPropertyW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPropertyA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPropertyW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetShortcutTargetA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetShortcutTargetW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSourcePathA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSourcePathW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSummaryInformationA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSummaryInformationW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetTargetPathA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetTargetPathW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetUserInfoA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetUserInfoW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingComponentA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingComponentW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingFileA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingFileW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallProductA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallProductW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiIsProductElevatedA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiIsProductElevatedW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiJoinTransaction();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiLocateComponentA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiLocateComponentW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiNotifySidChangeA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiNotifySidChangeW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenDatabaseA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenDatabaseW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenProductA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenProductW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewBillboardA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewBillboardW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewDialogA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewDialogW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiProcessAdvertiseScriptA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiProcessAdvertiseScriptW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiProcessMessage();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideAssemblyA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideAssemblyW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideComponentA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideComponentW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryComponentStateA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryComponentStateW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryProductStateA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryProductStateW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiRecordClearData();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiRecordDataSize();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiRecordGetFieldCount();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiRecordGetInteger();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordGetStringA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordGetStringW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordIsNull();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordReadStream();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiRecordSetInteger();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStreamA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStreamW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStringA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStringW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallFeatureA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallFeatureW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallProductA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallProductW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRemovePatchesA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRemovePatchesW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSequenceA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSequenceW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetComponentStateA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetComponentStateW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetExternalUIA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiSetExternalUIRecord();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetExternalUIW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureAttributesA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureAttributesW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureStateA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureStateW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiSetInstallLevel();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetInternalUI();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetMode();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetPropertyA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetPropertyW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetTargetPathA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetTargetPathW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddMediaDiskA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddMediaDiskW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearMediaDiskA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearMediaDiskW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearSourceA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearSourceW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumMediaDisksA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumMediaDisksW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumSourcesA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumSourcesW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListGetInfoA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListGetInfoW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListSetInfoA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListSetInfoW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoGetPropertyA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiSummaryInfoGetPropertyCount();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoGetPropertyW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiSummaryInfoPersist();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoSetPropertyA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoSetPropertyW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureExA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureExW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiVerifyDiskSpace();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiVerifyPackageA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiVerifyPackageW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiViewClose();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiViewExecute();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiViewFetch();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiViewGetColumnInfo();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiViewGetErrorA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiViewGetErrorW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiViewModify();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NormalizeFileForPatchSignature();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryActCtxSettingsW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryActCtxW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseActCtx();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfcGetNextProtectedFile();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfcIsFileProtected();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SfcIsKeyProtected();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfpVerifyFile();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileA();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileByBuffers();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileByHandles();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileW();
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ZombifyActCtx();
}
