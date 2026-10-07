#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
mod bindings;

use bindings::*;
use windows_core::Interface;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _apartment = windows_core::init_sta()?;
    // MMDeviceEnumerator's coclass UUID from the pinned mmdeviceapi.h.
    let class = windows_core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
    const INPROC_SERVER: u32 = 1;
    const DEVICE_STATE_ACTIVE: u32 = 1;

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
        let devices = enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
        let count = devices.GetCount()?;
        if count == 0 {
            return Err("audio smoke test requires an active render endpoint".into());
        }
        for index in 0..count {
            let device = devices.Item(index)?;
            assert_eq!(device.GetState()?, DEVICE_STATE_ACTIVE);
            let id = device.GetId()?;
            assert!(!id.is_null());
            let text = id.to_string();
            CoTaskMemFree(id.0.cast());
            assert!(!text?.is_empty());

            let iid = IAudioEndpointVolume::IID;
            let iid = GUID {
                Data1: iid.data1,
                Data2: iid.data2,
                Data3: iid.data3,
                Data4: iid.data4,
            };
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
            println!("audio endpoint {index}: {channels} channels; read-only queries passed");
        }
        println!("clang2 audio smoke: {count} endpoint(s) passed");
    }
    Ok(())
}
