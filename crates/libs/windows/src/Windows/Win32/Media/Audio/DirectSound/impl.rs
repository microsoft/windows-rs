pub trait IDirectSoundImpl: Sized {
    fn CreateSoundBuffer();
    fn GetCaps();
    fn DuplicateSoundBuffer();
    fn SetCooperativeLevel();
    fn Compact();
    fn GetSpeakerConfig();
    fn SetSpeakerConfig();
    fn Initialize();
}
pub trait IDirectSound3DBufferImpl: Sized {
    fn GetAllParameters();
    fn GetConeAngles();
    fn GetConeOrientation();
    fn GetConeOutsideVolume();
    fn GetMaxDistance();
    fn GetMinDistance();
    fn GetMode();
    fn GetPosition();
    fn GetVelocity();
    fn SetAllParameters();
    fn SetConeAngles();
    fn SetConeOrientation();
    fn SetConeOutsideVolume();
    fn SetMaxDistance();
    fn SetMinDistance();
    fn SetMode();
    fn SetPosition();
    fn SetVelocity();
}
pub trait IDirectSound3DListenerImpl: Sized {
    fn GetAllParameters();
    fn GetDistanceFactor();
    fn GetDopplerFactor();
    fn GetOrientation();
    fn GetPosition();
    fn GetRolloffFactor();
    fn GetVelocity();
    fn SetAllParameters();
    fn SetDistanceFactor();
    fn SetDopplerFactor();
    fn SetOrientation();
    fn SetPosition();
    fn SetRolloffFactor();
    fn SetVelocity();
    fn CommitDeferredSettings();
}
pub trait IDirectSound8Impl: Sized + IDirectSoundImpl {
    fn VerifyCertification();
}
pub trait IDirectSoundBufferImpl: Sized {
    fn GetCaps();
    fn GetCurrentPosition();
    fn GetFormat();
    fn GetVolume();
    fn GetPan();
    fn GetFrequency();
    fn GetStatus();
    fn Initialize();
    fn Lock();
    fn Play();
    fn SetCurrentPosition();
    fn SetFormat();
    fn SetVolume();
    fn SetPan();
    fn SetFrequency();
    fn Stop();
    fn Unlock();
    fn Restore();
}
pub trait IDirectSoundBuffer8Impl: Sized + IDirectSoundBufferImpl {
    fn SetFX();
    fn AcquireResources();
    fn GetObjectInPath();
}
pub trait IDirectSoundCaptureImpl: Sized {
    fn CreateCaptureBuffer();
    fn GetCaps();
    fn Initialize();
}
pub trait IDirectSoundCaptureBufferImpl: Sized {
    fn GetCaps();
    fn GetCurrentPosition();
    fn GetFormat();
    fn GetStatus();
    fn Initialize();
    fn Lock();
    fn Start();
    fn Stop();
    fn Unlock();
}
pub trait IDirectSoundCaptureBuffer8Impl: Sized + IDirectSoundCaptureBufferImpl {
    fn GetObjectInPath();
    fn GetFXStatus();
}
pub trait IDirectSoundCaptureFXAecImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
    fn GetStatus();
    fn Reset();
}
pub trait IDirectSoundCaptureFXNoiseSuppressImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
    fn Reset();
}
pub trait IDirectSoundFXChorusImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
pub trait IDirectSoundFXCompressorImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
pub trait IDirectSoundFXDistortionImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
pub trait IDirectSoundFXEchoImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
pub trait IDirectSoundFXFlangerImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
pub trait IDirectSoundFXGargleImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
pub trait IDirectSoundFXI3DL2ReverbImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
    fn SetPreset();
    fn GetPreset();
    fn SetQuality();
    fn GetQuality();
}
pub trait IDirectSoundFXParamEqImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
pub trait IDirectSoundFXWavesReverbImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
pub trait IDirectSoundFullDuplexImpl: Sized {
    fn Initialize();
}
pub trait IDirectSoundNotifyImpl: Sized {
    fn SetNotificationPositions();
}
