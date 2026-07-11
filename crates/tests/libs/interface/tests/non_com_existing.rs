#![cfg(windows)]
use windows::{combaseapi::*, core::*, d3d10::*, d3d12::*, objbase::*, sdkddkver::*, xaudio2::*};

struct Reflection;

impl ID3D12FunctionParameterReflection_Impl for Reflection {
    fn GetDesc(&self, pdesc: *mut D3D12_PARAMETER_DESC) -> Result<()> {
        unsafe {
            *pdesc = D3D12_PARAMETER_DESC {
                Name: s!("test"),
                ..Default::default()
            };
            Ok(())
        }
    }
}

struct Variable(u128);

impl Default for Variable {
    fn default() -> Self {
        Self(0x00000000_0000_0000_c000_000000000046)
    }
}

impl ID3D10EffectBlendVariable_Impl for Variable {
    fn GetBlendState(&self, _: u32) -> Result<ID3D10BlendState> {
        unimplemented!();
    }
    fn GetBackingStore(&self, _: u32, _: *mut D3D10_BLEND_DESC) -> Result<()> {
        unimplemented!();
    }
}

impl ID3D10EffectVariable_Impl for Variable {
    fn IsValid(&self) -> BOOL {
        assert_eq!(self.0, 0x00000000_0000_0000_c000_000000000046);
        true.into()
    }
    fn GetType(&self) -> Option<ID3D10EffectType> {
        unimplemented!();
    }
    fn GetDesc(&self, _: *mut D3D10_EFFECT_VARIABLE_DESC) -> Result<()> {
        unimplemented!();
    }
    fn GetAnnotationByIndex(&self, _: u32) -> Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetAnnotationByName(&self, _: &PCSTR) -> Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetMemberByIndex(&self, _: u32) -> Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetMemberByName(&self, _: &PCSTR) -> Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetMemberBySemantic(&self, _: &PCSTR) -> Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetElement(&self, _: u32) -> Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unimplemented!();
    }
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unimplemented!();
    }
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unimplemented!();
    }
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unimplemented!();
    }
    fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unimplemented!();
    }
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unimplemented!();
    }
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unimplemented!();
    }
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unimplemented!();
    }
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unimplemented!();
    }
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unimplemented!();
    }
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unimplemented!();
    }
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unimplemented!();
    }
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unimplemented!();
    }
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unimplemented!();
    }
    fn SetRawValue(&self, _: *const core::ffi::c_void, _: u32, _: u32) -> Result<()> {
        unimplemented!();
    }
    fn GetRawValue(&self, _: *mut core::ffi::c_void, _: u32, _: u32) -> Result<()> {
        unimplemented!();
    }
}

struct Callback;

impl IXAudio2VoiceCallback_Impl for Callback {
    fn OnVoiceProcessingPassStart(&self, _: u32) {
        unimplemented!()
    }
    fn OnVoiceProcessingPassEnd(&self) {
        unimplemented!()
    }
    fn OnStreamEnd(&self) {}
    fn OnBufferStart(&self, _: *mut std::ffi::c_void) {
        unimplemented!()
    }
    fn OnBufferEnd(&self, _: *mut std::ffi::c_void) {
        unimplemented!()
    }
    fn OnLoopEnd(&self, _: *mut std::ffi::c_void) {
        unimplemented!()
    }
    fn OnVoiceError(&self, _: *mut std::ffi::c_void, _: HRESULT) {
        unimplemented!()
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()?;

        let reflection = ID3D12FunctionParameterReflection::new(&Reflection);
        let mut desc = Default::default();
        reflection.GetDesc(&mut desc).ok()?;
        assert_eq!("test", desc.Name.to_string().unwrap());

        let variable = Variable::default();
        let interface_variable = ID3D10EffectVariable::new(&variable);
        assert_eq!(interface_variable.IsValid(), true);

        let variable = Variable::default();
        let interface_variable = ID3D10EffectBlendVariable::new(&variable);
        assert_eq!(interface_variable.IsValid(), true);

        let mut audio = None;
        XAudio2CreateWithVersionInfo(
            &mut audio,
            0,
            XAUDIO2_PROCESSOR(XAUDIO2_DEFAULT_PROCESSOR),
            NTDDI_VERSION,
        )
        .ok()?;
        let audio = audio.unwrap();

        // Call the callback interface directly...
        let callback = IXAudio2VoiceCallback::new(&Callback);
        callback.OnStreamEnd();

        // Pass the callback to a function...
        call_callback(&callback);

        // Pass the callback to another API (ignore the result)...
        let mut source = None;
        _ = audio.CreateSourceVoice(
            &mut source,
            std::ptr::null(),
            0,
            0.0,
            &*callback,
            None,
            None,
        );

        // TODO: also test that the actual non-COM interface parameter binding works with null and non-null interface args.

        Ok(())
    }
}

fn call_callback(_: &IXAudio2VoiceCallback) {}
