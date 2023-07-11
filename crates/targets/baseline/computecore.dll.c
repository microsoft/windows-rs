void __stdcall HcsAddResourceToOperation(int p0, int p1, int p2, int p3) {}
void __stdcall HcsCancelOperation(int p0) {}
void __stdcall HcsCloseComputeSystem(int p0) {}
void __stdcall HcsCloseOperation(int p0) {}
void __stdcall HcsCloseProcess(int p0) {}
void __stdcall HcsCrashComputeSystem(int p0, int p1, int p2) {}
void __stdcall HcsCreateComputeSystem(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall HcsCreateComputeSystemInNamespace(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall HcsCreateEmptyGuestStateFile(int p0) {}
void __stdcall HcsCreateEmptyRuntimeStateFile(int p0) {}
void __stdcall HcsCreateOperation(int p0, int p1) {}
void __stdcall HcsCreateOperationWithNotifications(int p0, int p1, int p2) {}
void __stdcall HcsCreateProcess(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall HcsEnumerateComputeSystems(int p0, int p1) {}
void __stdcall HcsEnumerateComputeSystemsInNamespace(int p0, int p1, int p2) {}
void __stdcall HcsGetComputeSystemFromOperation(int p0) {}
void __stdcall HcsGetComputeSystemProperties(int p0, int p1, int p2) {}
void __stdcall HcsGetOperationContext(int p0) {}
void __stdcall HcsGetOperationId(int p0) {}
void __stdcall HcsGetOperationResult(int p0, int p1) {}
void __stdcall HcsGetOperationResultAndProcessInfo(int p0, int p1, int p2) {}
void __stdcall HcsGetOperationType(int p0) {}
void __stdcall HcsGetProcessFromOperation(int p0) {}
void __stdcall HcsGetProcessInfo(int p0, int p1) {}
void __stdcall HcsGetProcessProperties(int p0, int p1, int p2) {}
void __stdcall HcsGetProcessorCompatibilityFromSavedState(int p0, int p1) {}
void __stdcall HcsGetServiceProperties(int p0, int p1) {}
void __stdcall HcsGrantVmAccess(int p0, int p1) {}
void __stdcall HcsGrantVmGroupAccess(int p0) {}
void __stdcall HcsModifyComputeSystem(int p0, int p1, int p2, int p3) {}
void __stdcall HcsModifyProcess(int p0, int p1, int p2) {}
void __stdcall HcsModifyServiceSettings(int p0, int p1) {}
void __stdcall HcsOpenComputeSystem(int p0, int p1, int p2) {}
void __stdcall HcsOpenComputeSystemInNamespace(int p0, int p1, int p2, int p3) {}
void __stdcall HcsOpenProcess(int p0, int p1, int p2, int p3) {}
void __stdcall HcsPauseComputeSystem(int p0, int p1, int p2) {}
void __stdcall HcsResumeComputeSystem(int p0, int p1, int p2) {}
void __stdcall HcsRevokeVmAccess(int p0, int p1) {}
void __stdcall HcsRevokeVmGroupAccess(int p0) {}
void __stdcall HcsSaveComputeSystem(int p0, int p1, int p2) {}
void __stdcall HcsSetComputeSystemCallback(int p0, int p1, int p2, int p3) {}
void __stdcall HcsSetOperationCallback(int p0, int p1, int p2) {}
void __stdcall HcsSetOperationContext(int p0, int p1) {}
void __stdcall HcsSetProcessCallback(int p0, int p1, int p2, int p3) {}
void __stdcall HcsShutDownComputeSystem(int p0, int p1, int p2) {}
void __stdcall HcsSignalProcess(int p0, int p1, int p2) {}
void __stdcall HcsStartComputeSystem(int p0, int p1, int p2) {}
void __stdcall HcsSubmitWerReport(int p0) {}
void __stdcall HcsTerminateComputeSystem(int p0, int p1, int p2) {}
void __stdcall HcsTerminateProcess(int p0, int p1, int p2) {}
void __stdcall HcsWaitForComputeSystemExit(int p0, int p1, int p2) {}
void __stdcall HcsWaitForOperationResult(int p0, int p1, int p2) {}
void __stdcall HcsWaitForOperationResultAndProcessInfo(int p0, int p1, int p2, int p3) {}
void __stdcall HcsWaitForProcessExit(int p0, int p1, int p2) {}
