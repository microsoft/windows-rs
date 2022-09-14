use windows::{core::*, Win32::Graphics::Direct3D12::*};

struct Test;

impl ID3D12FunctionParameterReflection_Impl for Test {
    fn GetDesc(&self) -> Result<D3D12_PARAMETER_DESC> {
        Ok(D3D12_PARAMETER_DESC { Name: s!("test"), ..Default::default() })
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let test = ID3D12FunctionParameterReflection::new(&Test);
        let desc = test.GetDesc()?;
        assert_eq!("test", desc.Name.to_string().unwrap());
        Ok(())
    }
}
