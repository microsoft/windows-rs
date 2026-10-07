#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations,
    clippy::useless_transmute
)]
mod bindings;

use bindings::*;
use windows_core::Interface;

#[windows_core::implement(IActivateAudioInterfaceCompletionHandler)]
struct Completion {
    sender: std::sync::mpsc::Sender<Result<u32, String>>,
}

impl IActivateAudioInterfaceCompletionHandler_Impl for Completion_Impl {
    fn ActivateCompleted(
        &self,
        operation: windows_core::Ref<IActivateAudioInterfaceAsyncOperation>,
    ) -> windows_core::Result<()> {
        let result = (|| -> Result<u32, String> {
            unsafe {
                let mut status = E_FAIL;
                let mut activated = None;
                operation.ok().map_err(|error| error.to_string())?
                    .GetActivateResult(&mut status, &mut activated)
                    .ok().map_err(|error| format!("GetActivateResult: {error}"))?;
                status.ok().map_err(|error| format!("asynchronous activation: {error}"))?;
                let activated = activated.ok_or("GetActivateResult returned no interface")?;
                let volume = activated.cast::<IAudioEndpointVolume>()
                    .map_err(|error| format!("IAudioEndpointVolume: {error}"))?;
                volume.GetChannelCount().map_err(|error| format!("GetChannelCount: {error}"))
            }
        })();
        self.sender
            .send(result)
            .map_err(|error| {
                windows_core::Error::new(
                    E_FAIL,
                    format!("audio completion receiver closed: {error}"),
                )
            })
    }
}

fn guid(value: _GUID) -> windows_core::GUID {
    windows_core::GUID {
        data1: value.Data1,
        data2: value.Data2,
        data3: value.Data3,
        data4: value.Data4,
    }
}

fn native_guid(value: windows_core::GUID) -> _GUID {
    _GUID {
        Data1: value.data1,
        Data2: value.data2,
        Data3: value.data3,
        Data4: value.data4,
    }
}

fn activate_async() -> Result<u32, Box<dyn std::error::Error>> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let handler: IActivateAudioInterfaceCompletionHandler = Completion { sender }.into();
    unsafe {
        let mut text = windows_core::PWSTR::null();
        StringFromIID(&guid(DEVINTERFACE_AUDIO_RENDER), &mut text.0).ok()?;
        let path = text.to_string();
        CoTaskMemFree(text.0.cast());
        let path = windows_core::HSTRING::from(path?);
        let mut raw = core::ptr::null_mut();
        ActivateAudioInterfaceAsync(
            windows_core::PCWSTR(path.as_ptr()),
            &native_guid(IAudioEndpointVolume::IID),
            core::ptr::null(),
            handler.as_raw(),
            &mut raw,
        )
        .ok().map_err(|error| format!("ActivateAudioInterfaceAsync: {error}"))?;
        let operation: IActivateAudioInterfaceAsyncOperation =
            windows_core::imp::Type::from_abi(raw)?;
        let channels = receiver.recv_timeout(std::time::Duration::from_secs(20))??;
        drop(operation);
        if channels == 0 {
            return Err("asynchronous render activation returned no channels".into());
        }
        Ok(channels)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _apartment = windows_core::init_sta()?;
    // MMDeviceEnumerator's coclass UUID from the pinned mmdeviceapi.h.
    let class = windows_core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
    const INPROC_SERVER: u32 = 1;

    unsafe {
        let mut raw = core::ptr::null_mut();
        CoCreateInstance(
            &class,
            core::ptr::null_mut(),
            INPROC_SERVER,
            &IMMDeviceEnumerator::IID,
            &mut raw,
        )
        .ok()?;
        let enumerator: IMMDeviceEnumerator = windows_core::imp::Type::from_abi(raw)?;
        let devices = enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE as u32)?;
        let count = devices.GetCount()?;
        if count == 0 {
            return Err("audio smoke test requires an active render endpoint".into());
        }
        for index in 0..count {
            let device = devices.Item(index)?;
            assert_eq!(device.GetState()?, DEVICE_STATE_ACTIVE as u32);
            let id = device.GetId()?;
            assert!(!id.is_null());
            let text = id.to_string();
            CoTaskMemFree(id.0.cast());
            assert!(!text?.is_empty());

            let key = PKEY_AudioEndpoint_FormFactor;
            let key = PROPERTYKEY {
                fmtid: guid(key.fmtid),
                pid: key.pid,
            };
            let store = device.OpenPropertyStore(0)?;
            let mut value = store.GetValue(&key)?;
            let form_factor = (i32::from(value.Anonymous.Anonymous.vt) == VT_UI4)
                .then(|| value.Anonymous.Anonymous.Anonymous.ulVal);
            PropVariantClear(&mut value).ok()?;
            let form_factor = form_factor.ok_or("audio form factor property is not VT_UI4")?;
            assert!(form_factor < EndpointFormFactor_enum_count as u32);

            let iid = native_guid(IAudioEndpointVolume::IID);
            let mut raw = core::ptr::null_mut();
            device.Activate(&iid, INPROC_SERVER, None, &mut raw).ok()?;
            let volume: IAudioEndpointVolume = windows_core::imp::Type::from_abi(raw)?;
            let channels = volume.GetChannelCount()?;
            assert!(channels > 0);
            assert!((0.0..=1.0).contains(&volume.GetMasterVolumeLevelScalar()?));
            let _ = volume.GetMute()?;

            let extended = volume.cast::<IAudioEndpointVolumeEx>()?;
            let (mut min, mut max, mut step) = (0.0, 0.0, 0.0);
            extended
                .GetVolumeRangeChannel(0, &mut min, &mut max, &mut step)
                .ok()?;
            assert!(min.is_finite() && max.is_finite() && step.is_finite());
            assert!(min <= max && step >= 0.0);
            println!(
                "audio endpoint {index}: {channels} channels, form factor {form_factor}; read-only queries passed"
            );
        }
        println!("clang2 audio smoke: {count} endpoint(s) passed");
    }
    println!(
        "clang2 ordinal-17 async activation: {} render channels",
        activate_async()?
    );
    Ok(())
}
