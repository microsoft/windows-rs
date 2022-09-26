use windows::{core::*, Win32::Foundation::*, Win32::Graphics::Direct3D10::*, Win32::Graphics::Direct3D12::*};

struct Reflection;

impl ID3D12FunctionParameterReflection_Impl for Reflection {
    fn GetDesc(&self) -> Result<D3D12_PARAMETER_DESC> {
        Ok(D3D12_PARAMETER_DESC { Name: s!("test"), ..Default::default() })
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
        todo!();
    }
    fn GetBackingStore(&self, _: u32, _: *mut D3D10_BLEND_DESC) -> Result<()> {
        todo!();
    }
}

impl ID3D10EffectVariable_Impl for Variable {
    fn IsValid(&self) -> BOOL {
        assert_eq!(self.0, 0x00000000_0000_0000_c000_000000000046);
        true.into()
    }
    fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        todo!();
    }
    fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        todo!();
    }
    fn GetAnnotationByIndex(&self, _: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        todo!();
    }
    fn GetAnnotationByName(&self, _: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
        todo!();
    }
    fn GetMemberByIndex(&self, _: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        todo!();
    }
    fn GetMemberByName(&self, _: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
        todo!();
    }
    fn GetMemberBySemantic(&self, _: &::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
        todo!();
    }
    fn GetElement(&self, _: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        todo!();
    }
    fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        todo!();
    }
    fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        todo!();
    }
    fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        todo!();
    }
    fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        todo!();
    }
    fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        todo!();
    }
    fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        todo!();
    }
    fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        todo!();
    }
    fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        todo!();
    }
    fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        todo!();
    }
    fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        todo!();
    }
    fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        todo!();
    }
    fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        todo!();
    }
    fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        todo!();
    }
    fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        todo!();
    }
    fn SetRawValue(&self, _: *const ::core::ffi::c_void, _: u32, _: u32) -> ::windows::core::Result<()> {
        todo!();
    }
    fn GetRawValue(&self, _: *mut ::core::ffi::c_void, _: u32, _: u32) -> ::windows::core::Result<()> {
        todo!();
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let reflection = ID3D12FunctionParameterReflection::new(&Reflection);
        let desc = reflection.GetDesc()?;
        assert_eq!("test", desc.Name.to_string().unwrap());

        let variable = Variable::default();
        let interface_variable = ID3D10EffectVariable::new(&variable);
        assert_eq!(interface_variable.IsValid(), true);

        let variable = Variable::default();
        let interface_variable = ID3D10EffectBlendVariable::new(&variable);
        assert_eq!(interface_variable.IsValid(), true);

        Ok(())
    }
}
