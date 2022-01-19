// TODO: implement ID3D12FunctionReflection - requires #453

// use windows::core::*;
// use test_return_com_interface::*;
// use Windows::Win32::Graphics::Direct3D12::*;

// #[implement(Windows::Win32::Graphics::Direct3D12::ID3D12FunctionReflection)]
// struct Test();

// impl Test {
//     fn GetDesc(&self) -> Result<D3D12_FUNCTION_DESC> {
//         Ok(D3D12_FUNCTION_DESC::default())
//     }

// 	fn GetConstantBufferByIndex(&self, _: u32) -> Option<ID3D12ShaderReflectionConstantBuffer> {
//         None
//     }

// 	fn GetConstantBufferByName(&self, _: PSTR) -> Option<ID3D12ShaderReflectionConstantBuffer> {
//         None
//     }

// 	fn GetResourceBindingDesc(&self, _: u32) -> Result<D3D12_SHADER_INPUT_BIND_DESC> {
//         Ok(D3D12_SHADER_INPUT_BIND_DESC::default())
//     }

// 	fn GetVariableByName(&self, _: PSTR) -> Option<ID3D12ShaderReflectionVariable> {
//         None
//     }

// 	fn GetResourceBindingDescByName(&self, _: PSTR) -> Result<D3D12_SHADER_INPUT_BIND_DESC> {
//         Ok(D3D12_SHADER_INPUT_BIND_DESC::default())
//     }

// 	fn GetFunctionParameter(_ : i32) -> Option<ID3D12FunctionParameterReflection> {
//         None
//     }
// }
