pub trait IXAPOImpl: Sized {
    fn GetRegistrationProperties();
    fn IsInputFormatSupported();
    fn IsOutputFormatSupported();
    fn Initialize();
    fn Reset();
    fn LockForProcess();
    fn UnlockForProcess();
    fn Process();
    fn CalcInputFrames();
    fn CalcOutputFrames();
}
pub trait IXAPOHrtfParametersImpl: Sized {
    fn SetSourcePosition();
    fn SetSourceOrientation();
    fn SetSourceGain();
    fn SetEnvironment();
}
pub trait IXAPOParametersImpl: Sized {
    fn SetParameters();
    fn GetParameters();
}
pub trait IXAudio2Impl: Sized {
    fn RegisterForCallbacks();
    fn UnregisterForCallbacks();
    fn CreateSourceVoice();
    fn CreateSubmixVoice();
    fn CreateMasteringVoice();
    fn StartEngine();
    fn StopEngine();
    fn CommitChanges();
    fn GetPerformanceData();
    fn SetDebugConfiguration();
}
pub trait IXAudio2EngineCallbackImpl: Sized {
    fn OnProcessingPassStart();
    fn OnProcessingPassEnd();
    fn OnCriticalError();
}
pub trait IXAudio2ExtensionImpl: Sized {
    fn GetProcessingQuantum();
    fn GetProcessor();
}
pub trait IXAudio2MasteringVoiceImpl: Sized + IXAudio2VoiceImpl {
    fn GetChannelMask();
}
pub trait IXAudio2SourceVoiceImpl: Sized + IXAudio2VoiceImpl {
    fn Start();
    fn Stop();
    fn SubmitSourceBuffer();
    fn FlushSourceBuffers();
    fn Discontinuity();
    fn ExitLoop();
    fn GetState();
    fn SetFrequencyRatio();
    fn GetFrequencyRatio();
    fn SetSourceSampleRate();
}
pub trait IXAudio2SubmixVoiceImpl: Sized + IXAudio2VoiceImpl {}
pub trait IXAudio2VoiceImpl: Sized {
    fn GetVoiceDetails();
    fn SetOutputVoices();
    fn SetEffectChain();
    fn EnableEffect();
    fn DisableEffect();
    fn GetEffectState();
    fn SetEffectParameters();
    fn GetEffectParameters();
    fn SetFilterParameters();
    fn GetFilterParameters();
    fn SetOutputFilterParameters();
    fn GetOutputFilterParameters();
    fn SetVolume();
    fn GetVolume();
    fn SetChannelVolumes();
    fn GetChannelVolumes();
    fn SetOutputMatrix();
    fn GetOutputMatrix();
    fn DestroyVoice();
}
pub trait IXAudio2VoiceCallbackImpl: Sized {
    fn OnVoiceProcessingPassStart();
    fn OnVoiceProcessingPassEnd();
    fn OnStreamEnd();
    fn OnBufferStart();
    fn OnBufferEnd();
    fn OnLoopEnd();
    fn OnVoiceError();
}
