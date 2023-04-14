void __stdcall ClfsAddLogContainer(int p0, int p1, int p2) {}
void __stdcall ClfsAddLogContainerSet(int p0, int p1, int p2, int p3) {}
void __stdcall ClfsAdvanceLogBase(int p0, int p1, int p2) {}
void __stdcall ClfsAlignReservedLog(int p0, int p1, int p2, int p3) {}
void __stdcall ClfsAllocReservedLog(int p0, int p1, int p2) {}
void __stdcall ClfsCloseAndResetLogFile(int p0) {}
void __stdcall ClfsCloseLogFileObject(int p0) {}
void __stdcall ClfsCreateLogFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall ClfsCreateMarshallingArea(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall ClfsCreateMarshallingAreaEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall ClfsCreateScanContext(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ClfsDeleteLogByPointer(int p0) {}
void __stdcall ClfsDeleteLogFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ClfsDeleteMarshallingArea(int p0) {}
void __stdcall ClfsEarlierLsn(int p0) {}
void __stdcall ClfsFinalize() {}
void __stdcall ClfsFlushBuffers(int p0) {}
void __stdcall ClfsFlushToLsn(int p0, int p1, int p2) {}
void __stdcall ClfsFreeReservedLog(int p0, int p1, int p2) {}
void __stdcall ClfsGetContainerName(int p0, int p1, int p2, int p3) {}
void __stdcall ClfsGetIoStatistics(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ClfsGetLogFileInformation(int p0, int p1, int p2) {}
void __stdcall ClfsInitialize() {}
void __stdcall ClfsLaterLsn(int p0) {}
void __stdcall ClfsLsnBlockOffset(int p0) {}
void __stdcall ClfsLsnContainer(int p0) {}
void __stdcall ClfsLsnCreate(int p0, int p1, int p2) {}
void __stdcall ClfsLsnDifference(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ClfsLsnEqual(int p0, int p1) {}
void __stdcall ClfsLsnGreater(int p0, int p1) {}
void __stdcall ClfsLsnInvalid(int p0) {}
void __stdcall ClfsLsnLess(int p0, int p1) {}
void __stdcall ClfsLsnNull(int p0) {}
void __stdcall ClfsLsnRecordSequence(int p0) {}
void __stdcall ClfsMgmtDeregisterManagedClient(int p0) {}
void __stdcall ClfsMgmtHandleLogFileFull(int p0) {}
void __stdcall ClfsMgmtInstallPolicy(int p0, int p1, int p2) {}
void __stdcall ClfsMgmtQueryPolicy(int p0, int p1, int p2, int p3) {}
void __stdcall ClfsMgmtRegisterManagedClient(int p0, int p1, int p2) {}
void __stdcall ClfsMgmtRemovePolicy(int p0, int p1) {}
void __stdcall ClfsMgmtSetLogFileSize(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ClfsMgmtSetLogFileSizeAsClient(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ClfsMgmtTailAdvanceFailure(int p0, int p1) {}
void __stdcall ClfsQueryLogFileInformation(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ClfsReadLogRecord(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall ClfsReadNextLogRecord(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall ClfsReadPreviousRestartArea(int p0, int p1, int p2, int p3) {}
void __stdcall ClfsReadRestartArea(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ClfsRemoveLogContainer(int p0, int p1, int p2) {}
void __stdcall ClfsRemoveLogContainerSet(int p0, int p1, int p2, int p3) {}
void __stdcall ClfsReserveAndAppendLog(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall ClfsReserveAndAppendLogAligned(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall ClfsScanLogContainers(int p0, int p1) {}
void __stdcall ClfsSetArchiveTail(int p0, int p1) {}
void __stdcall ClfsSetEndOfLog(int p0, int p1) {}
void __stdcall ClfsSetLogFileInformation(int p0, int p1, int p2, int p3) {}
void __stdcall ClfsTerminateReadLog(int p0) {}
void __stdcall ClfsWriteRestartArea(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
