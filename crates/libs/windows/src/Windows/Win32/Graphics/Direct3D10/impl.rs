pub trait ID3D10AsynchronousImpl: Sized + ID3D10DeviceChildImpl {
    fn Begin();
    fn End();
    fn GetData();
    fn GetDataSize();
}
pub trait ID3D10BlendStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D10BlendState1Impl: Sized + ID3D10BlendStateImpl + ID3D10DeviceChildImpl {
    fn GetDesc1();
}
pub trait ID3D10BufferImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
pub trait ID3D10CounterImpl: Sized + ID3D10AsynchronousImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D10DebugImpl: Sized {
    fn SetFeatureMask();
    fn GetFeatureMask();
    fn SetPresentPerRenderOpDelay();
    fn GetPresentPerRenderOpDelay();
    fn SetSwapChain();
    fn GetSwapChain();
    fn Validate();
}
pub trait ID3D10DepthStencilStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D10DepthStencilViewImpl: Sized + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D10DeviceImpl: Sized {
    fn VSSetConstantBuffers();
    fn PSSetShaderResources();
    fn PSSetShader();
    fn PSSetSamplers();
    fn VSSetShader();
    fn DrawIndexed();
    fn Draw();
    fn PSSetConstantBuffers();
    fn IASetInputLayout();
    fn IASetVertexBuffers();
    fn IASetIndexBuffer();
    fn DrawIndexedInstanced();
    fn DrawInstanced();
    fn GSSetConstantBuffers();
    fn GSSetShader();
    fn IASetPrimitiveTopology();
    fn VSSetShaderResources();
    fn VSSetSamplers();
    fn SetPredication();
    fn GSSetShaderResources();
    fn GSSetSamplers();
    fn OMSetRenderTargets();
    fn OMSetBlendState();
    fn OMSetDepthStencilState();
    fn SOSetTargets();
    fn DrawAuto();
    fn RSSetState();
    fn RSSetViewports();
    fn RSSetScissorRects();
    fn CopySubresourceRegion();
    fn CopyResource();
    fn UpdateSubresource();
    fn ClearRenderTargetView();
    fn ClearDepthStencilView();
    fn GenerateMips();
    fn ResolveSubresource();
    fn VSGetConstantBuffers();
    fn PSGetShaderResources();
    fn PSGetShader();
    fn PSGetSamplers();
    fn VSGetShader();
    fn PSGetConstantBuffers();
    fn IAGetInputLayout();
    fn IAGetVertexBuffers();
    fn IAGetIndexBuffer();
    fn GSGetConstantBuffers();
    fn GSGetShader();
    fn IAGetPrimitiveTopology();
    fn VSGetShaderResources();
    fn VSGetSamplers();
    fn GetPredication();
    fn GSGetShaderResources();
    fn GSGetSamplers();
    fn OMGetRenderTargets();
    fn OMGetBlendState();
    fn OMGetDepthStencilState();
    fn SOGetTargets();
    fn RSGetState();
    fn RSGetViewports();
    fn RSGetScissorRects();
    fn GetDeviceRemovedReason();
    fn SetExceptionMode();
    fn GetExceptionMode();
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn ClearState();
    fn Flush();
    fn CreateBuffer();
    fn CreateTexture1D();
    fn CreateTexture2D();
    fn CreateTexture3D();
    fn CreateShaderResourceView();
    fn CreateRenderTargetView();
    fn CreateDepthStencilView();
    fn CreateInputLayout();
    fn CreateVertexShader();
    fn CreateGeometryShader();
    fn CreateGeometryShaderWithStreamOutput();
    fn CreatePixelShader();
    fn CreateBlendState();
    fn CreateDepthStencilState();
    fn CreateRasterizerState();
    fn CreateSamplerState();
    fn CreateQuery();
    fn CreatePredicate();
    fn CreateCounter();
    fn CheckFormatSupport();
    fn CheckMultisampleQualityLevels();
    fn CheckCounterInfo();
    fn CheckCounter();
    fn GetCreationFlags();
    fn OpenSharedResource();
    fn SetTextFilterSize();
    fn GetTextFilterSize();
}
pub trait ID3D10Device1Impl: Sized + ID3D10DeviceImpl {
    fn CreateShaderResourceView1();
    fn CreateBlendState1();
    fn GetFeatureLevel();
}
pub trait ID3D10DeviceChildImpl: Sized {
    fn GetDevice();
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
}
pub trait ID3D10EffectImpl: Sized {
    fn IsValid();
    fn IsPool();
    fn GetDevice();
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetVariableByIndex();
    fn GetVariableByName();
    fn GetVariableBySemantic();
    fn GetTechniqueByIndex();
    fn GetTechniqueByName();
    fn Optimize();
    fn IsOptimized();
}
pub trait ID3D10EffectBlendVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetBlendState();
    fn GetBackingStore();
}
pub trait ID3D10EffectConstantBufferImpl: Sized + ID3D10EffectVariableImpl {
    fn SetConstantBuffer();
    fn GetConstantBuffer();
    fn SetTextureBuffer();
    fn GetTextureBuffer();
}
pub trait ID3D10EffectDepthStencilVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetDepthStencilState();
    fn GetBackingStore();
}
pub trait ID3D10EffectDepthStencilViewVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetDepthStencil();
    fn GetDepthStencil();
    fn SetDepthStencilArray();
    fn GetDepthStencilArray();
}
pub trait ID3D10EffectMatrixVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetMatrix();
    fn GetMatrix();
    fn SetMatrixArray();
    fn GetMatrixArray();
    fn SetMatrixTranspose();
    fn GetMatrixTranspose();
    fn SetMatrixTransposeArray();
    fn GetMatrixTransposeArray();
}
pub trait ID3D10EffectPassImpl: Sized {
    fn IsValid();
    fn GetDesc();
    fn GetVertexShaderDesc();
    fn GetGeometryShaderDesc();
    fn GetPixelShaderDesc();
    fn GetAnnotationByIndex();
    fn GetAnnotationByName();
    fn Apply();
    fn ComputeStateBlockMask();
}
pub trait ID3D10EffectPoolImpl: Sized {
    fn AsEffect();
}
pub trait ID3D10EffectRasterizerVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetRasterizerState();
    fn GetBackingStore();
}
pub trait ID3D10EffectRenderTargetViewVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetRenderTarget();
    fn GetRenderTarget();
    fn SetRenderTargetArray();
    fn GetRenderTargetArray();
}
pub trait ID3D10EffectSamplerVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetSampler();
    fn GetBackingStore();
}
pub trait ID3D10EffectScalarVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetFloat();
    fn GetFloat();
    fn SetFloatArray();
    fn GetFloatArray();
    fn SetInt();
    fn GetInt();
    fn SetIntArray();
    fn GetIntArray();
    fn SetBool();
    fn GetBool();
    fn SetBoolArray();
    fn GetBoolArray();
}
pub trait ID3D10EffectShaderResourceVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetResource();
    fn GetResource();
    fn SetResourceArray();
    fn GetResourceArray();
}
pub trait ID3D10EffectShaderVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetShaderDesc();
    fn GetVertexShader();
    fn GetGeometryShader();
    fn GetPixelShader();
    fn GetInputSignatureElementDesc();
    fn GetOutputSignatureElementDesc();
}
pub trait ID3D10EffectStringVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn GetString();
    fn GetStringArray();
}
pub trait ID3D10EffectTechniqueImpl: Sized {
    fn IsValid();
    fn GetDesc();
    fn GetAnnotationByIndex();
    fn GetAnnotationByName();
    fn GetPassByIndex();
    fn GetPassByName();
    fn ComputeStateBlockMask();
}
pub trait ID3D10EffectTypeImpl: Sized {
    fn IsValid();
    fn GetDesc();
    fn GetMemberTypeByIndex();
    fn GetMemberTypeByName();
    fn GetMemberTypeBySemantic();
    fn GetMemberName();
    fn GetMemberSemantic();
}
pub trait ID3D10EffectVariableImpl: Sized {
    fn IsValid();
    fn GetType();
    fn GetDesc();
    fn GetAnnotationByIndex();
    fn GetAnnotationByName();
    fn GetMemberByIndex();
    fn GetMemberByName();
    fn GetMemberBySemantic();
    fn GetElement();
    fn GetParentConstantBuffer();
    fn AsScalar();
    fn AsVector();
    fn AsMatrix();
    fn AsString();
    fn AsShaderResource();
    fn AsRenderTargetView();
    fn AsDepthStencilView();
    fn AsConstantBuffer();
    fn AsShader();
    fn AsBlend();
    fn AsDepthStencil();
    fn AsRasterizer();
    fn AsSampler();
    fn SetRawValue();
    fn GetRawValue();
}
pub trait ID3D10EffectVectorVariableImpl: Sized + ID3D10EffectVariableImpl {
    fn SetBoolVector();
    fn SetIntVector();
    fn SetFloatVector();
    fn GetBoolVector();
    fn GetIntVector();
    fn GetFloatVector();
    fn SetBoolVectorArray();
    fn SetIntVectorArray();
    fn SetFloatVectorArray();
    fn GetBoolVectorArray();
    fn GetIntVectorArray();
    fn GetFloatVectorArray();
}
pub trait ID3D10GeometryShaderImpl: Sized + ID3D10DeviceChildImpl {}
pub trait ID3D10InfoQueueImpl: Sized {
    fn SetMessageCountLimit();
    fn ClearStoredMessages();
    fn GetMessage();
    fn GetNumMessagesAllowedByStorageFilter();
    fn GetNumMessagesDeniedByStorageFilter();
    fn GetNumStoredMessages();
    fn GetNumStoredMessagesAllowedByRetrievalFilter();
    fn GetNumMessagesDiscardedByMessageCountLimit();
    fn GetMessageCountLimit();
    fn AddStorageFilterEntries();
    fn GetStorageFilter();
    fn ClearStorageFilter();
    fn PushEmptyStorageFilter();
    fn PushCopyOfStorageFilter();
    fn PushStorageFilter();
    fn PopStorageFilter();
    fn GetStorageFilterStackSize();
    fn AddRetrievalFilterEntries();
    fn GetRetrievalFilter();
    fn ClearRetrievalFilter();
    fn PushEmptyRetrievalFilter();
    fn PushCopyOfRetrievalFilter();
    fn PushRetrievalFilter();
    fn PopRetrievalFilter();
    fn GetRetrievalFilterStackSize();
    fn AddMessage();
    fn AddApplicationMessage();
    fn SetBreakOnCategory();
    fn SetBreakOnSeverity();
    fn SetBreakOnID();
    fn GetBreakOnCategory();
    fn GetBreakOnSeverity();
    fn GetBreakOnID();
    fn SetMuteDebugOutput();
    fn GetMuteDebugOutput();
}
pub trait ID3D10InputLayoutImpl: Sized + ID3D10DeviceChildImpl {}
pub trait ID3D10MultithreadImpl: Sized {
    fn Enter();
    fn Leave();
    fn SetMultithreadProtected();
    fn GetMultithreadProtected();
}
pub trait ID3D10PixelShaderImpl: Sized + ID3D10DeviceChildImpl {}
pub trait ID3D10PredicateImpl: Sized + ID3D10QueryImpl + ID3D10AsynchronousImpl + ID3D10DeviceChildImpl {}
pub trait ID3D10QueryImpl: Sized + ID3D10AsynchronousImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D10RasterizerStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D10RenderTargetViewImpl: Sized + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D10ResourceImpl: Sized + ID3D10DeviceChildImpl {
    fn GetType();
    fn SetEvictionPriority();
    fn GetEvictionPriority();
}
pub trait ID3D10SamplerStateImpl: Sized + ID3D10DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D10ShaderReflectionImpl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetInputParameterDesc();
    fn GetOutputParameterDesc();
}
pub trait ID3D10ShaderReflection1Impl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetInputParameterDesc();
    fn GetOutputParameterDesc();
    fn GetVariableByName();
    fn GetResourceBindingDescByName();
    fn GetMovInstructionCount();
    fn GetMovcInstructionCount();
    fn GetConversionInstructionCount();
    fn GetBitwiseInstructionCount();
    fn GetGSInputPrimitive();
    fn IsLevel9Shader();
    fn IsSampleFrequencyShader();
}
pub trait ID3D10ShaderReflectionConstantBufferImpl: Sized {
    fn GetDesc();
    fn GetVariableByIndex();
    fn GetVariableByName();
}
pub trait ID3D10ShaderReflectionTypeImpl: Sized {
    fn GetDesc();
    fn GetMemberTypeByIndex();
    fn GetMemberTypeByName();
    fn GetMemberTypeName();
}
pub trait ID3D10ShaderReflectionVariableImpl: Sized {
    fn GetDesc();
    fn GetType();
}
pub trait ID3D10ShaderResourceViewImpl: Sized + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D10ShaderResourceView1Impl: Sized + ID3D10ShaderResourceViewImpl + ID3D10ViewImpl + ID3D10DeviceChildImpl {
    fn GetDesc1();
}
pub trait ID3D10StateBlockImpl: Sized {
    fn Capture();
    fn Apply();
    fn ReleaseAllDeviceObjects();
    fn GetDevice();
}
pub trait ID3D10SwitchToRefImpl: Sized {
    fn SetUseRef();
    fn GetUseRef();
}
pub trait ID3D10Texture1DImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
pub trait ID3D10Texture2DImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
pub trait ID3D10Texture3DImpl: Sized + ID3D10ResourceImpl + ID3D10DeviceChildImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
}
pub trait ID3D10VertexShaderImpl: Sized + ID3D10DeviceChildImpl {}
pub trait ID3D10ViewImpl: Sized + ID3D10DeviceChildImpl {
    fn GetResource();
}
