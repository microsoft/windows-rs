void __stdcall AddLogContainer(int p0, int p1, int p2, int p3) {}
void __stdcall AddLogContainerSet(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall AdvanceLogBase(int p0, int p1, int p2, int p3) {}
void __stdcall AlignReservedLog(int p0, int p1, int p2, int p3) {}
void __stdcall AllocReservedLog(int p0, int p1, int p2) {}
void __stdcall CloseAndResetLogFile(int p0) {}
void __stdcall CreateLogContainerScanContext(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CreateLogFile(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CreateLogMarshallingArea(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall DeleteLogByHandle(int p0) {}
void __stdcall DeleteLogFile(int p0, int p1) {}
void __stdcall DeleteLogMarshallingArea(int p0) {}
void __stdcall DeregisterManageableLogClient(int p0) {}
void __stdcall FlushLogBuffers(int p0, int p1) {}
void __stdcall FlushLogToLsn(int p0, int p1, int p2, int p3) {}
void __stdcall FreeReservedLog(int p0, int p1, int p2) {}
void __stdcall GetLogContainerName(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall GetLogFileInformation(int p0, int p1, int p2) {}
void __stdcall GetLogIoStatistics(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall GetLogReservationInfo(int p0, int p1, int p2, int p3) {}
void __stdcall GetNextLogArchiveExtent(int p0, int p1, int p2, int p3) {}
void __stdcall HandleLogFull(int p0) {}
void __stdcall InstallLogPolicy(int p0, int p1) {}
void __stdcall LogTailAdvanceFailure(int p0, int p1) {}
void __stdcall LsnBlockOffset(int p0) {}
void __stdcall LsnContainer(int p0) {}
void __stdcall LsnCreate(int p0, int p1, int p2) {}
void __stdcall LsnEqual(int p0, int p1) {}
void __stdcall LsnGreater(int p0, int p1) {}
void __stdcall LsnIncrement(int p0) {}
void __stdcall LsnInvalid(int p0) {}
void __stdcall LsnLess(int p0, int p1) {}
void __stdcall LsnNull(int p0) {}
void __stdcall LsnRecordSequence(int p0) {}
void __stdcall PrepareLogArchive(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall QueryLogPolicy(int p0, int p1, int p2, int p3) {}
void __stdcall ReadLogArchiveMetadata(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ReadLogNotification(int p0, int p1, int p2) {}
void __stdcall ReadLogRecord(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall ReadLogRestartArea(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ReadNextLogRecord(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall ReadPreviousLogRestartArea(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RegisterForLogWriteNotification(int p0, int p1, int p2) {}
void __stdcall RegisterManageableLogClient(int p0, int p1) {}
void __stdcall RemoveLogContainer(int p0, int p1, int p2, int p3) {}
void __stdcall RemoveLogContainerSet(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RemoveLogPolicy(int p0, int p1) {}
void __stdcall ReserveAndAppendLog(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall ReserveAndAppendLogAligned(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall ScanLogContainers(int p0, int p1, int p2) {}
void __stdcall SetEndOfLog(int p0, int p1, int p2) {}
void __stdcall SetLogArchiveMode(int p0, int p1) {}
void __stdcall SetLogArchiveTail(int p0, int p1, int p2) {}
void __stdcall SetLogFileSizeWithPolicy(int p0, int p1, int p2) {}
void __stdcall TerminateLogArchive(int p0) {}
void __stdcall TerminateReadLog(int p0) {}
void __stdcall TruncateLog(int p0, int p1, int p2) {}
void __stdcall ValidateLog(int p0, int p1, int p2, int p3) {}
void __stdcall WriteLogRestartArea(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
