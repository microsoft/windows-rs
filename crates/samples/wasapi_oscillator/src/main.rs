use std::{env, fmt::Debug, mem};
use windows::{
    core::*,
    Win32::{
        UI::Shell::PropertiesSystem::PROPERTYKEY,
        Devices::FunctionDiscovery::*,
        Media::{
            Audio::{eRender, IMMDeviceEnumerator, MMDeviceEnumerator, WAVEFORMATEX, *},
            KernelStreaming::*,
            Multimedia::*,
        },
        System::Com::*,
    },
};

/// The api represents time in 100 nanosecond units,lets call this unit `k` .\
/// time_this_constant_represents = REFTIMES_PER_SEC**k***(100ns/**k**) = 100_000_000_000ns  / 10^9(ns/sec) =  1 sec of time.
const REFTIMES_PER_SEC: i64 = 100_000_000;

/// ## Description
/// An "object" that writes sinwave data to a buffer
#[derive(Default)]
pub struct ImplicitWaveSource {
    format_ext: MYWAVEFORMATEXTENSIBLE,
    /// this basically represents time since `frames_written`/`sample_rate` = time in seconds since the start of the the signal
    /// you could also call this variable "samples_per_channel_written" if that makes it more clear. According to Microsoft docs
    /// a "frame" reffers to the group of samples  interleaved ,for example, stereo audio goes like this: \[L,R, L,R,...,L,R\] but
    /// frames **grouped** samples like this: \[ \[L,R\] , \[L,R\] , ... , \[L,R\] \]
    frames_written: u64,
}
impl ImplicitWaveSource {
    #[allow(non_snake_case)]
    pub fn SetFormat(&mut self, pwfx: *mut WAVEFORMATEX) {
        unsafe {
            self.format_ext = std::mem::transmute_copy(&*(pwfx as *mut MYWAVEFORMATEXTENSIBLE));
        }
    }

    #[allow(non_snake_case)]
    /// Writes Sine wave samples to the `data` slice
    pub fn LoadData(&mut self, numFramesAvailable: usize, buffer: &mut [u8], flags: &mut _AUDCLNT_BUFFERFLAGS) {
        let output_format = self.format_ext.Format;

        // I assume each sample is 32bits in size so i cast the slice to a slice of [f32]
        let samples_buffer = unsafe { std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut f32, buffer.len() / 4) };

        let num_channels = output_format.nChannels as usize;

        for frame_idx in 0..numFramesAvailable {
            let time_in_seconds = self.frames_written as f64 / output_format.nSamplesPerSec as f64;
            let sample = (time_in_seconds * 441.0).sin() * 0.1;
            let sample_out = sample as f32;
            //write to frame
            for k in 0..num_channels {
                samples_buffer[frame_idx * num_channels + k] = sample_out;
            }
            self.frames_written += 1;
        }
        *flags = _AUDCLNT_BUFFERFLAGS::default();
    }
}

#[repr(C, packed(1))]
#[derive(Copy, Clone, Debug, Default)]
#[allow(non_snake_case)]
/// a complete copy of the type WAVEFORMATEXTENSIBLE except I can now
/// derive Default and Debug traits which saves me some time typing
pub struct MYWAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}

#[repr(C, packed(1))]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
/// a complete copy of the type WAVEFORMATEXTENSIBLE_0
pub union MYWAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: u16,
    pub wSamplesPerBlock: u16,
    pub wReserved: u16,
}
impl Debug for MYWAVEFORMATEXTENSIBLE_0 {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl Default for MYWAVEFORMATEXTENSIBLE_0 {
    fn default() -> Self {
        Self { wValidBitsPerSample: 0 }
    }
}

#[repr(C, packed(1))]
#[derive(Copy, Clone, Debug, Default)]
#[allow(non_snake_case)]
/// a complete copy of the type WAVEFORMATEXTENSIBLE except I can now
/// derive Default and Debug traits which saves me some time typing when I want to print this info
pub struct MYWAVEFORMATEXTENSIBLE {
    pub Format: MYWAVEFORMATEX,
    pub Samples: MYWAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: ::windows::core::GUID,
}

unsafe fn get_devices_friendly_name(device: &IMMDevice) -> Result<String> {
    let property_store = device.OpenPropertyStore(STGM_READ)?;
    let friendly_name = property_store.GetValue(&PKEY_Device_FriendlyName as *const PROPERTYKEY)?;

    Ok(friendly_name.Anonymous.Anonymous.Anonymous.pwszVal.to_string().expect("string failed to convert"))
}

unsafe fn find_device_by<F>(device_collection: &IMMDeviceCollection, predicate: F) -> Result<IMMDevice>
where
    F: Fn(&WAVEFORMATEX) -> bool,
{
    let num_devices = device_collection.GetCount()?;
    let mut selected_dev = Err(Error::from_win32());
    println!("list_of_devices:");
    for device_idx in 0..num_devices {
        let dev = device_collection.Item(device_idx)?;
        println!("device[index={device_idx}]:{}", get_devices_friendly_name(&dev)?);

        let audio_client = dev.Activate::<IAudioClient>(CLSCTX_ALL, None).expect("audio client failed to activate");

        let pwfx = &*audio_client.GetMixFormat().expect("mix format failed to fetch");

        if selected_dev.is_err() && predicate(pwfx) {
            selected_dev = Ok(dev);
        }
    }

    selected_dev
}

#[allow(non_snake_case)]
fn main() -> Result<()> {
    let mut pMySource = ImplicitWaveSource::default();
    let bufferFrameCount;
    let mut pFlags = _AUDCLNT_BUFFERFLAGS::default();

    let args = env::args().collect::<Vec<_>>();

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).expect("co initalize failed");

        let clsid_enumerator = &MMDeviceEnumerator as *const GUID;
        let pEnumerator = CoCreateInstance::<_, IMMDeviceEnumerator>(clsid_enumerator, InParam::null(), CLSCTX_ALL).expect("CoCreateInstance(..) failed");

        let device_collection = pEnumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;

        let mut pDevice = find_device_by(&device_collection, |format| format.wFormatTag as u32 == WAVE_FORMAT_EXTENSIBLE).expect("could not find device with tag WAVE_FORMAT_EXTENSIBLE");

        if args.len() >= 2 {
            //you can manually select a device by index if you pass in an integer argument
            let device_id = args[1].parse().expect("device index must be a non-negative integer (0-n)");
            pDevice = device_collection.Item(device_id)?;
        }

        println!("We selected device = '{}'", get_devices_friendly_name(&pDevice)?);

        let pAudioClient = pDevice.Activate::<IAudioClient>(CLSCTX_ALL, None).expect("audio client failed to activate");

        // WASAPI gives you a pointer to specs of the device, WAVEFORMATEX is the header
        // and there could additional information beyond it
        let pwfx = pAudioClient.GetMixFormat().expect("mix format failed to fetch");

        let device_format_header = mem::transmute_copy::<_, MYWAVEFORMATEX>(&*(pwfx as *mut WAVEFORMATEX));

        if device_format_header.wFormatTag as u32 != WAVE_FORMAT_EXTENSIBLE {
            eprintln!("Device selected is not WAVE_FORMAT_EXTENSIBLE");
            return Err(Error::from_win32());
        }

        let device_format_header_with_body = mem::transmute_copy::<_, MYWAVEFORMATEXTENSIBLE>(&*(pwfx as *mut WAVEFORMATEXTENSIBLE));

        //print device specs here
        let sub_format = match device_format_header_with_body.SubFormat {
            KSDATAFORMAT_SUBTYPE_PCM => String::from("two's complement PCM "),
            KSDATAFORMAT_SUBTYPE_IEEE_FLOAT => String::from("IEEE-754 PCM"),
            _ => panic!("unknown subformat"),
        };
        println!("device subformat is = {sub_format}");
        println!("device full format details:\n{:?}", device_format_header_with_body);

        //this asks WASAPI to expose to us,the client, a buffer containing 1 second worth of audio
        pAudioClient.Initialize(AUDCLNT_SHAREMODE_SHARED, 0, REFTIMES_PER_SEC, 0, std::mem::transmute(pwfx), None).expect("client failed to initalize");

        //this just copies the format data to the ocilator
        pMySource.SetFormat(pwfx);

        // getBufferSize returns the size of the buffer in "frames"
        // "frames"  are the groups of samples in an interleaved format that play at the same time but go to different
        // speakers
        bufferFrameCount = pAudioClient.GetBufferSize()?;

        let bytes_per_frame = ((device_format_header.wBitsPerSample / 8) * device_format_header.nChannels) as u32;

        let pRenderClient: IAudioRenderClient = pAudioClient.GetService().expect("get service failed");

        let pData = std::slice::from_raw_parts_mut(
            //GetBuffer exposes  expects to recive data PCM INTERLEAVED
            pRenderClient.GetBuffer(bufferFrameCount)?,
            (bufferFrameCount * bytes_per_frame) as usize,
        );

        // write some sine-wave data to the buffer
        pMySource.LoadData(bufferFrameCount as usize, pData, &mut pFlags);

        //enqueue written data
        pRenderClient.ReleaseBuffer(bufferFrameCount, pFlags.0 as u32).expect("error releasing");

        //begin playing audio (WASAPI will start dequeing audio and playing it)
        pAudioClient.Start().expect("failed to start audio client");

        loop {
            //initial buffer plays for a second , so wait half a second before loading up
            //trying to enqueue more audio
            std::thread::sleep(std::time::Duration::from_millis(500));

            // GetCurrentPadding is used to figureout what part of the buffer,WASAPI exposes to use  is
            // actually available to be written to
            let numFramesPadding = pAudioClient.GetCurrentPadding().expect("padding failed");
            let numFramesAvailable = bufferFrameCount - numFramesPadding;

            // fetch a pointer to the buffer that WASAPI exposes to us
            let pData = std::slice::from_raw_parts_mut(pRenderClient.GetBuffer(numFramesAvailable)?, (numFramesAvailable * bytes_per_frame) as usize);

            // write more sine data into the buffer
            pMySource.LoadData(numFramesAvailable as usize, pData, &mut pFlags);

            //give the buffer back to WASAPI so it can play the audio
            pRenderClient.ReleaseBuffer(numFramesAvailable, pFlags.0 as u32).expect("failed to release");
        }
    }
}
