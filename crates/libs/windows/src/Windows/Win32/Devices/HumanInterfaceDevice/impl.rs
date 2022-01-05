pub trait IDirectInput2AImpl: Sized + IDirectInputAImpl {
    fn FindDevice();
}
pub trait IDirectInput2WImpl: Sized + IDirectInputWImpl {
    fn FindDevice();
}
pub trait IDirectInput7AImpl: Sized + IDirectInput2AImpl + IDirectInputAImpl {
    fn CreateDeviceEx();
}
pub trait IDirectInput7WImpl: Sized + IDirectInput2WImpl + IDirectInputWImpl {
    fn CreateDeviceEx();
}
pub trait IDirectInput8AImpl: Sized {
    fn CreateDevice();
    fn EnumDevices();
    fn GetDeviceStatus();
    fn RunControlPanel();
    fn Initialize();
    fn FindDevice();
    fn EnumDevicesBySemantics();
    fn ConfigureDevices();
}
pub trait IDirectInput8WImpl: Sized {
    fn CreateDevice();
    fn EnumDevices();
    fn GetDeviceStatus();
    fn RunControlPanel();
    fn Initialize();
    fn FindDevice();
    fn EnumDevicesBySemantics();
    fn ConfigureDevices();
}
pub trait IDirectInputAImpl: Sized {
    fn CreateDevice();
    fn EnumDevices();
    fn GetDeviceStatus();
    fn RunControlPanel();
    fn Initialize();
}
pub trait IDirectInputDevice2AImpl: Sized + IDirectInputDeviceAImpl {
    fn CreateEffect();
    fn EnumEffects();
    fn GetEffectInfo();
    fn GetForceFeedbackState();
    fn SendForceFeedbackCommand();
    fn EnumCreatedEffectObjects();
    fn Escape();
    fn Poll();
    fn SendDeviceData();
}
pub trait IDirectInputDevice2WImpl: Sized + IDirectInputDeviceWImpl {
    fn CreateEffect();
    fn EnumEffects();
    fn GetEffectInfo();
    fn GetForceFeedbackState();
    fn SendForceFeedbackCommand();
    fn EnumCreatedEffectObjects();
    fn Escape();
    fn Poll();
    fn SendDeviceData();
}
pub trait IDirectInputDevice7AImpl: Sized + IDirectInputDevice2AImpl + IDirectInputDeviceAImpl {
    fn EnumEffectsInFile();
    fn WriteEffectToFile();
}
pub trait IDirectInputDevice7WImpl: Sized + IDirectInputDevice2WImpl + IDirectInputDeviceWImpl {
    fn EnumEffectsInFile();
    fn WriteEffectToFile();
}
pub trait IDirectInputDevice8AImpl: Sized {
    fn GetCapabilities();
    fn EnumObjects();
    fn GetProperty();
    fn SetProperty();
    fn Acquire();
    fn Unacquire();
    fn GetDeviceState();
    fn GetDeviceData();
    fn SetDataFormat();
    fn SetEventNotification();
    fn SetCooperativeLevel();
    fn GetObjectInfo();
    fn GetDeviceInfo();
    fn RunControlPanel();
    fn Initialize();
    fn CreateEffect();
    fn EnumEffects();
    fn GetEffectInfo();
    fn GetForceFeedbackState();
    fn SendForceFeedbackCommand();
    fn EnumCreatedEffectObjects();
    fn Escape();
    fn Poll();
    fn SendDeviceData();
    fn EnumEffectsInFile();
    fn WriteEffectToFile();
    fn BuildActionMap();
    fn SetActionMap();
    fn GetImageInfo();
}
pub trait IDirectInputDevice8WImpl: Sized {
    fn GetCapabilities();
    fn EnumObjects();
    fn GetProperty();
    fn SetProperty();
    fn Acquire();
    fn Unacquire();
    fn GetDeviceState();
    fn GetDeviceData();
    fn SetDataFormat();
    fn SetEventNotification();
    fn SetCooperativeLevel();
    fn GetObjectInfo();
    fn GetDeviceInfo();
    fn RunControlPanel();
    fn Initialize();
    fn CreateEffect();
    fn EnumEffects();
    fn GetEffectInfo();
    fn GetForceFeedbackState();
    fn SendForceFeedbackCommand();
    fn EnumCreatedEffectObjects();
    fn Escape();
    fn Poll();
    fn SendDeviceData();
    fn EnumEffectsInFile();
    fn WriteEffectToFile();
    fn BuildActionMap();
    fn SetActionMap();
    fn GetImageInfo();
}
pub trait IDirectInputDeviceAImpl: Sized {
    fn GetCapabilities();
    fn EnumObjects();
    fn GetProperty();
    fn SetProperty();
    fn Acquire();
    fn Unacquire();
    fn GetDeviceState();
    fn GetDeviceData();
    fn SetDataFormat();
    fn SetEventNotification();
    fn SetCooperativeLevel();
    fn GetObjectInfo();
    fn GetDeviceInfo();
    fn RunControlPanel();
    fn Initialize();
}
pub trait IDirectInputDeviceWImpl: Sized {
    fn GetCapabilities();
    fn EnumObjects();
    fn GetProperty();
    fn SetProperty();
    fn Acquire();
    fn Unacquire();
    fn GetDeviceState();
    fn GetDeviceData();
    fn SetDataFormat();
    fn SetEventNotification();
    fn SetCooperativeLevel();
    fn GetObjectInfo();
    fn GetDeviceInfo();
    fn RunControlPanel();
    fn Initialize();
}
pub trait IDirectInputEffectImpl: Sized {
    fn Initialize();
    fn GetEffectGuid();
    fn GetParameters();
    fn SetParameters();
    fn Start();
    fn Stop();
    fn GetEffectStatus();
    fn Download();
    fn Unload();
    fn Escape();
}
pub trait IDirectInputEffectDriverImpl: Sized {
    fn DeviceID();
    fn GetVersions();
    fn Escape();
    fn SetGain();
    fn SendForceFeedbackCommand();
    fn GetForceFeedbackState();
    fn DownloadEffect();
    fn DestroyEffect();
    fn StartEffect();
    fn StopEffect();
    fn GetEffectStatus();
}
pub trait IDirectInputJoyConfigImpl: Sized {
    fn Acquire();
    fn Unacquire();
    fn SetCooperativeLevel();
    fn SendNotify();
    fn EnumTypes();
    fn GetTypeInfo();
    fn SetTypeInfo();
    fn DeleteType();
    fn GetConfig();
    fn SetConfig();
    fn DeleteConfig();
    fn GetUserValues();
    fn SetUserValues();
    fn AddNewHardware();
    fn OpenTypeKey();
    fn OpenConfigKey();
}
pub trait IDirectInputJoyConfig8Impl: Sized {
    fn Acquire();
    fn Unacquire();
    fn SetCooperativeLevel();
    fn SendNotify();
    fn EnumTypes();
    fn GetTypeInfo();
    fn SetTypeInfo();
    fn DeleteType();
    fn GetConfig();
    fn SetConfig();
    fn DeleteConfig();
    fn GetUserValues();
    fn SetUserValues();
    fn AddNewHardware();
    fn OpenTypeKey();
    fn OpenAppStatusKey();
}
pub trait IDirectInputWImpl: Sized {
    fn CreateDevice();
    fn EnumDevices();
    fn GetDeviceStatus();
    fn RunControlPanel();
    fn Initialize();
}
