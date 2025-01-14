use windows::{
    core::*, Win32::Graphics::Direct3D10::*, Win32::Graphics::Direct3D12::*,
    Win32::Media::Audio::XAudio2::*, Win32::System::Com::*, Win32::System::SystemInformation::*,
};

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
    fn GetType(&self) -> core::option::Option<ID3D10EffectType> {
        unimplemented!();
    }
    fn GetDesc(&self, _: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows::core::Result<()> {
        unimplemented!();
    }
    fn GetAnnotationByIndex(&self, _: u32) -> core::option::Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetAnnotationByName(
        &self,
        _: &::windows::core::PCSTR,
    ) -> core::option::Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetMemberByIndex(&self, _: u32) -> core::option::Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetMemberByName(
        &self,
        _: &::windows::core::PCSTR,
    ) -> core::option::Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetMemberBySemantic(
        &self,
        _: &::windows::core::PCSTR,
    ) -> core::option::Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetElement(&self, _: u32) -> core::option::Option<ID3D10EffectVariable> {
        unimplemented!();
    }
    fn GetParentConstantBuffer(&self) -> core::option::Option<ID3D10EffectConstantBuffer> {
        unimplemented!();
    }
    fn AsScalar(&self) -> core::option::Option<ID3D10EffectScalarVariable> {
        unimplemented!();
    }
    fn AsVector(&self) -> core::option::Option<ID3D10EffectVectorVariable> {
        unimplemented!();
    }
    fn AsMatrix(&self) -> core::option::Option<ID3D10EffectMatrixVariable> {
        unimplemented!();
    }
    fn AsString(&self) -> core::option::Option<ID3D10EffectStringVariable> {
        unimplemented!();
    }
    fn AsShaderResource(&self) -> core::option::Option<ID3D10EffectShaderResourceVariable> {
        unimplemented!();
    }
    fn AsRenderTargetView(&self) -> core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        unimplemented!();
    }
    fn AsDepthStencilView(&self) -> core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        unimplemented!();
    }
    fn AsConstantBuffer(&self) -> core::option::Option<ID3D10EffectConstantBuffer> {
        unimplemented!();
    }
    fn AsShader(&self) -> core::option::Option<ID3D10EffectShaderVariable> {
        unimplemented!();
    }
    fn AsBlend(&self) -> core::option::Option<ID3D10EffectBlendVariable> {
        unimplemented!();
    }
    fn AsDepthStencil(&self) -> core::option::Option<ID3D10EffectDepthStencilVariable> {
        unimplemented!();
    }
    fn AsRasterizer(&self) -> core::option::Option<ID3D10EffectRasterizerVariable> {
        unimplemented!();
    }
    fn AsSampler(&self) -> core::option::Option<ID3D10EffectSamplerVariable> {
        unimplemented!();
    }
    fn SetRawValue(
        &self,
        _: *const core::ffi::c_void,
        _: u32,
        _: u32,
    ) -> windows::core::Result<()> {
        unimplemented!();
    }
    fn GetRawValue(&self, _: *mut core::ffi::c_void, _: u32, _: u32) -> windows::core::Result<()> {
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
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        let reflection = ID3D12FunctionParameterReflection::new(&Reflection);
        let mut desc = Default::default();
        reflection.GetDesc(&mut desc)?;
        assert_eq!("test", desc.Name.to_string().unwrap());

        let variable = Variable::default();
        let interface_variable = ID3D10EffectVariable::new(&variable);
        assert_eq!(interface_variable.IsValid(), true);

        let variable = Variable::default();
        let interface_variable = ID3D10EffectBlendVariable::new(&variable);
        assert_eq!(interface_variable.IsValid(), true);

        let mut audio = None;
        XAudio2CreateWithVersionInfo(&mut audio, 0, XAUDIO2_DEFAULT_PROCESSOR, NTDDI_VERSION)?;
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
