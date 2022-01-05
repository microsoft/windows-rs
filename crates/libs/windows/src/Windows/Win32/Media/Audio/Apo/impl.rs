pub trait IApoAcousticEchoCancellationImpl: Sized {}
pub trait IApoAuxiliaryInputConfigurationImpl: Sized {
    fn AddAuxiliaryInput();
    fn RemoveAuxiliaryInput();
    fn IsInputFormatSupported();
}
pub trait IApoAuxiliaryInputRTImpl: Sized {
    fn AcceptInput();
}
pub trait IAudioDeviceModulesClientImpl: Sized {
    fn SetAudioDeviceModulesManager();
}
pub trait IAudioMediaTypeImpl: Sized {
    fn IsCompressedFormat();
    fn IsEqual();
    fn GetAudioFormat();
    fn GetUncompressedAudioFormat();
}
pub trait IAudioProcessingObjectImpl: Sized {
    fn Reset();
    fn GetLatency();
    fn GetRegistrationProperties();
    fn Initialize();
    fn IsInputFormatSupported();
    fn IsOutputFormatSupported();
    fn GetInputChannelCount();
}
pub trait IAudioProcessingObjectConfigurationImpl: Sized {
    fn LockForProcess();
    fn UnlockForProcess();
}
pub trait IAudioProcessingObjectLoggingServiceImpl: Sized {
    fn ApoLog();
}
pub trait IAudioProcessingObjectNotificationsImpl: Sized {
    fn GetApoNotificationRegistrationInfo();
    fn HandleNotification();
}
pub trait IAudioProcessingObjectRTImpl: Sized {
    fn APOProcess();
    fn CalcInputFrames();
    fn CalcOutputFrames();
}
pub trait IAudioProcessingObjectRTQueueServiceImpl: Sized {
    fn GetRealTimeWorkQueue();
}
pub trait IAudioProcessingObjectVBRImpl: Sized {
    fn CalcMaxInputFrames();
    fn CalcMaxOutputFrames();
}
pub trait IAudioSystemEffectsImpl: Sized {}
pub trait IAudioSystemEffects2Impl: Sized + IAudioSystemEffectsImpl {
    fn GetEffectsList();
}
pub trait IAudioSystemEffects3Impl: Sized + IAudioSystemEffects2Impl + IAudioSystemEffectsImpl {
    fn GetControllableSystemEffectsList();
    fn SetAudioSystemEffectState();
}
pub trait IAudioSystemEffectsCustomFormatsImpl: Sized {
    fn GetFormatCount();
    fn GetFormat();
    fn GetFormatRepresentation();
}
