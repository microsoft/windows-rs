void __stdcall CloseDriver(int p0, int p1, int p2) {}
void __stdcall DefDriverProc(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall DriverCallback(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall DrvGetModuleHandle(int p0) {}
void __stdcall GetDriverModuleHandle(int p0) {}
void __stdcall OpenDriver(int p0, int p1, int p2) {}
void __stdcall PlaySoundA(int p0, int p1, int p2) {}
void __stdcall PlaySoundW(int p0, int p1, int p2) {}
void __stdcall SendDriverMessage(int p0, int p1, int p2, int p3) {}
void __stdcall auxGetDevCapsA(int p0, int p1, int p2) {}
void __stdcall auxGetDevCapsW(int p0, int p1, int p2) {}
void __stdcall auxGetNumDevs() {}
void __stdcall auxGetVolume(int p0, int p1) {}
void __stdcall auxOutMessage(int p0, int p1, int p2, int p3) {}
void __stdcall auxSetVolume(int p0, int p1) {}
void __stdcall joyConfigChanged(int p0) {}
void __stdcall joyGetDevCapsA(int p0, int p1, int p2) {}
void __stdcall joyGetDevCapsW(int p0, int p1, int p2) {}
void __stdcall joyGetNumDevs() {}
void __stdcall joyGetPos(int p0, int p1) {}
void __stdcall joyGetPosEx(int p0, int p1) {}
void __stdcall joyGetThreshold(int p0, int p1) {}
void __stdcall joyReleaseCapture(int p0) {}
void __stdcall joySetCapture(int p0, int p1, int p2, int p3) {}
void __stdcall joySetThreshold(int p0, int p1) {}
void __stdcall mciDriverNotify(int p0, int p1, int p2) {}
void __stdcall mciDriverYield(int p0) {}
void __stdcall mciFreeCommandResource(int p0) {}
void __stdcall mciGetCreatorTask(int p0) {}
void __stdcall mciGetDeviceIDA(int p0) {}
void __stdcall mciGetDeviceIDFromElementIDA(int p0, int p1) {}
void __stdcall mciGetDeviceIDFromElementIDW(int p0, int p1) {}
void __stdcall mciGetDeviceIDW(int p0) {}
void __stdcall mciGetDriverData(int p0) {}
void __stdcall mciGetErrorStringA(int p0, int p1, int p2) {}
void __stdcall mciGetErrorStringW(int p0, int p1, int p2) {}
void __stdcall mciGetYieldProc(int p0, int p1) {}
void __stdcall mciLoadCommandResource(int p0, int p1, int p2) {}
void __stdcall mciSendCommandA(int p0, int p1, int p2, int p3) {}
void __stdcall mciSendCommandW(int p0, int p1, int p2, int p3) {}
void __stdcall mciSendStringA(int p0, int p1, int p2, int p3) {}
void __stdcall mciSendStringW(int p0, int p1, int p2, int p3) {}
void __stdcall mciSetDriverData(int p0, int p1) {}
void __stdcall mciSetYieldProc(int p0, int p1, int p2) {}
void __stdcall midiConnect(int p0, int p1, int p2) {}
void __stdcall midiDisconnect(int p0, int p1, int p2) {}
void __stdcall midiInAddBuffer(int p0, int p1, int p2) {}
void __stdcall midiInClose(int p0) {}
void __stdcall midiInGetDevCapsA(int p0, int p1, int p2) {}
void __stdcall midiInGetDevCapsW(int p0, int p1, int p2) {}
void __stdcall midiInGetErrorTextA(int p0, int p1, int p2) {}
void __stdcall midiInGetErrorTextW(int p0, int p1, int p2) {}
void __stdcall midiInGetID(int p0, int p1) {}
void __stdcall midiInGetNumDevs() {}
void __stdcall midiInMessage(int p0, int p1, int p2, int p3) {}
void __stdcall midiInOpen(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall midiInPrepareHeader(int p0, int p1, int p2) {}
void __stdcall midiInReset(int p0) {}
void __stdcall midiInStart(int p0) {}
void __stdcall midiInStop(int p0) {}
void __stdcall midiInUnprepareHeader(int p0, int p1, int p2) {}
void __stdcall midiOutCacheDrumPatches(int p0, int p1, int p2, int p3) {}
void __stdcall midiOutCachePatches(int p0, int p1, int p2, int p3) {}
void __stdcall midiOutClose(int p0) {}
void __stdcall midiOutGetDevCapsA(int p0, int p1, int p2) {}
void __stdcall midiOutGetDevCapsW(int p0, int p1, int p2) {}
void __stdcall midiOutGetErrorTextA(int p0, int p1, int p2) {}
void __stdcall midiOutGetErrorTextW(int p0, int p1, int p2) {}
void __stdcall midiOutGetID(int p0, int p1) {}
void __stdcall midiOutGetNumDevs() {}
void __stdcall midiOutGetVolume(int p0, int p1) {}
void __stdcall midiOutLongMsg(int p0, int p1, int p2) {}
void __stdcall midiOutMessage(int p0, int p1, int p2, int p3) {}
void __stdcall midiOutOpen(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall midiOutPrepareHeader(int p0, int p1, int p2) {}
void __stdcall midiOutReset(int p0) {}
void __stdcall midiOutSetVolume(int p0, int p1) {}
void __stdcall midiOutShortMsg(int p0, int p1) {}
void __stdcall midiOutUnprepareHeader(int p0, int p1, int p2) {}
void __stdcall midiStreamClose(int p0) {}
void __stdcall midiStreamOpen(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall midiStreamOut(int p0, int p1, int p2) {}
void __stdcall midiStreamPause(int p0) {}
void __stdcall midiStreamPosition(int p0, int p1, int p2) {}
void __stdcall midiStreamProperty(int p0, int p1, int p2) {}
void __stdcall midiStreamRestart(int p0) {}
void __stdcall midiStreamStop(int p0) {}
void __stdcall mixerClose(int p0) {}
void __stdcall mixerGetControlDetailsA(int p0, int p1, int p2) {}
void __stdcall mixerGetControlDetailsW(int p0, int p1, int p2) {}
void __stdcall mixerGetDevCapsA(int p0, int p1, int p2) {}
void __stdcall mixerGetDevCapsW(int p0, int p1, int p2) {}
void __stdcall mixerGetID(int p0, int p1, int p2) {}
void __stdcall mixerGetLineControlsA(int p0, int p1, int p2) {}
void __stdcall mixerGetLineControlsW(int p0, int p1, int p2) {}
void __stdcall mixerGetLineInfoA(int p0, int p1, int p2) {}
void __stdcall mixerGetLineInfoW(int p0, int p1, int p2) {}
void __stdcall mixerGetNumDevs() {}
void __stdcall mixerMessage(int p0, int p1, int p2, int p3) {}
void __stdcall mixerOpen(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall mixerSetControlDetails(int p0, int p1, int p2) {}
void __stdcall mmDrvInstall(int p0, int p1, int p2, int p3) {}
void __stdcall mmGetCurrentTask() {}
void __stdcall mmTaskBlock(int p0) {}
void __stdcall mmTaskCreate(int p0, int p1, int p2) {}
void __stdcall mmTaskSignal(int p0) {}
void __stdcall mmTaskYield() {}
void __stdcall mmioAdvance(int p0, int p1, int p2) {}
void __stdcall mmioAscend(int p0, int p1, int p2) {}
void __stdcall mmioClose(int p0, int p1) {}
void __stdcall mmioCreateChunk(int p0, int p1, int p2) {}
void __stdcall mmioDescend(int p0, int p1, int p2, int p3) {}
void __stdcall mmioFlush(int p0, int p1) {}
void __stdcall mmioGetInfo(int p0, int p1, int p2) {}
void __stdcall mmioInstallIOProcA(int p0, int p1, int p2) {}
void __stdcall mmioInstallIOProcW(int p0, int p1, int p2) {}
void __stdcall mmioOpenA(int p0, int p1, int p2) {}
void __stdcall mmioOpenW(int p0, int p1, int p2) {}
void __stdcall mmioRead(int p0, int p1, int p2) {}
void __stdcall mmioRenameA(int p0, int p1, int p2, int p3) {}
void __stdcall mmioRenameW(int p0, int p1, int p2, int p3) {}
void __stdcall mmioSeek(int p0, int p1, int p2) {}
void __stdcall mmioSendMessage(int p0, int p1, int p2, int p3) {}
void __stdcall mmioSetBuffer(int p0, int p1, int p2, int p3) {}
void __stdcall mmioSetInfo(int p0, int p1, int p2) {}
void __stdcall mmioStringToFOURCCA(int p0, int p1) {}
void __stdcall mmioStringToFOURCCW(int p0, int p1) {}
void __stdcall mmioWrite(int p0, int p1, int p2) {}
void __stdcall sndPlaySoundA(int p0, int p1) {}
void __stdcall sndPlaySoundW(int p0, int p1) {}
void __stdcall timeBeginPeriod(int p0) {}
void __stdcall timeEndPeriod(int p0) {}
void __stdcall timeGetDevCaps(int p0, int p1) {}
void __stdcall timeGetSystemTime(int p0, int p1) {}
void __stdcall timeGetTime() {}
void __stdcall timeKillEvent(int p0) {}
void __stdcall timeSetEvent(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall waveInAddBuffer(int p0, int p1, int p2) {}
void __stdcall waveInClose(int p0) {}
void __stdcall waveInGetDevCapsA(int p0, int p1, int p2) {}
void __stdcall waveInGetDevCapsW(int p0, int p1, int p2) {}
void __stdcall waveInGetErrorTextA(int p0, int p1, int p2) {}
void __stdcall waveInGetErrorTextW(int p0, int p1, int p2) {}
void __stdcall waveInGetID(int p0, int p1) {}
void __stdcall waveInGetNumDevs() {}
void __stdcall waveInGetPosition(int p0, int p1, int p2) {}
void __stdcall waveInMessage(int p0, int p1, int p2, int p3) {}
void __stdcall waveInOpen(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall waveInPrepareHeader(int p0, int p1, int p2) {}
void __stdcall waveInReset(int p0) {}
void __stdcall waveInStart(int p0) {}
void __stdcall waveInStop(int p0) {}
void __stdcall waveInUnprepareHeader(int p0, int p1, int p2) {}
void __stdcall waveOutBreakLoop(int p0) {}
void __stdcall waveOutClose(int p0) {}
void __stdcall waveOutGetDevCapsA(int p0, int p1, int p2) {}
void __stdcall waveOutGetDevCapsW(int p0, int p1, int p2) {}
void __stdcall waveOutGetErrorTextA(int p0, int p1, int p2) {}
void __stdcall waveOutGetErrorTextW(int p0, int p1, int p2) {}
void __stdcall waveOutGetID(int p0, int p1) {}
void __stdcall waveOutGetNumDevs() {}
void __stdcall waveOutGetPitch(int p0, int p1) {}
void __stdcall waveOutGetPlaybackRate(int p0, int p1) {}
void __stdcall waveOutGetPosition(int p0, int p1, int p2) {}
void __stdcall waveOutGetVolume(int p0, int p1) {}
void __stdcall waveOutMessage(int p0, int p1, int p2, int p3) {}
void __stdcall waveOutOpen(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall waveOutPause(int p0) {}
void __stdcall waveOutPrepareHeader(int p0, int p1, int p2) {}
void __stdcall waveOutReset(int p0) {}
void __stdcall waveOutRestart(int p0) {}
void __stdcall waveOutSetPitch(int p0, int p1) {}
void __stdcall waveOutSetPlaybackRate(int p0, int p1) {}
void __stdcall waveOutSetVolume(int p0, int p1) {}
void __stdcall waveOutUnprepareHeader(int p0, int p1, int p2) {}
void __stdcall waveOutWrite(int p0, int p1, int p2) {}
