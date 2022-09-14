use windows::{core::*, Win32::Graphics::Direct3D12::*};

#[implement(ID3D12FunctionParameterReflection)]
struct Test;

impl ID3D12FunctionParameterReflection_Impl for Test {
    fn GetDesc(&self) -> Result<D3D12_PARAMETER_DESC> {
        Ok(D3D12_PARAMETER_DESC { Name: s!("test"), ..Default::default() })
    }
}

#[test]
fn test_existing() -> Result<()> {
    unsafe {
        // TODO: the `into` will create a heap object but ID3D12FunctionParameterReflection
        // doesn't have a Drop impl so there's no way to free it nor should there be.

        // SOLUTION: don't use implement macro at all - ID3D12FunctionParameterReflection should provide a
        // "new" function that creates an implementation owned wrapper and derefs as &ID3D12FunctionParameterReflection
        // without any heap allocation. This "new" method is created by the interface macro when there's not parent.

        let test: ID3D12FunctionParameterReflection = Test.into();
        let desc = test.GetDesc()?;
        assert_eq!("test", desc.Name.to_string().unwrap());
        Ok(())
    }
}
