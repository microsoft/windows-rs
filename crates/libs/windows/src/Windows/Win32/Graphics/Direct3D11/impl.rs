pub trait ID3D11AsynchronousImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDataSize();
}
pub trait ID3D11AuthenticatedChannelImpl: Sized + ID3D11DeviceChildImpl {
    fn GetCertificateSize();
    fn GetCertificate();
    fn GetChannelHandle();
}
pub trait ID3D11BlendStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11BlendState1Impl: Sized + ID3D11BlendStateImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
pub trait ID3D11BufferImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11ClassInstanceImpl: Sized + ID3D11DeviceChildImpl {
    fn GetClassLinkage();
    fn GetDesc();
    fn GetInstanceName();
    fn GetTypeName();
}
pub trait ID3D11ClassLinkageImpl: Sized + ID3D11DeviceChildImpl {
    fn GetClassInstance();
    fn CreateClassInstance();
}
pub trait ID3D11CommandListImpl: Sized + ID3D11DeviceChildImpl {
    fn GetContextFlags();
}
pub trait ID3D11ComputeShaderImpl: Sized + ID3D11DeviceChildImpl {}
pub trait ID3D11CounterImpl: Sized + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11CryptoSessionImpl: Sized + ID3D11DeviceChildImpl {
    fn GetCryptoType();
    fn GetDecoderProfile();
    fn GetCertificateSize();
    fn GetCertificate();
    fn GetCryptoSessionHandle();
}
pub trait ID3D11DebugImpl: Sized {
    fn SetFeatureMask();
    fn GetFeatureMask();
    fn SetPresentPerRenderOpDelay();
    fn GetPresentPerRenderOpDelay();
    fn SetSwapChain();
    fn GetSwapChain();
    fn ValidateContext();
    fn ReportLiveDeviceObjects();
    fn ValidateContextForDispatch();
}
pub trait ID3D11DepthStencilStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11DepthStencilViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11DeviceImpl: Sized {
    fn CreateBuffer();
    fn CreateTexture1D();
    fn CreateTexture2D();
    fn CreateTexture3D();
    fn CreateShaderResourceView();
    fn CreateUnorderedAccessView();
    fn CreateRenderTargetView();
    fn CreateDepthStencilView();
    fn CreateInputLayout();
    fn CreateVertexShader();
    fn CreateGeometryShader();
    fn CreateGeometryShaderWithStreamOutput();
    fn CreatePixelShader();
    fn CreateHullShader();
    fn CreateDomainShader();
    fn CreateComputeShader();
    fn CreateClassLinkage();
    fn CreateBlendState();
    fn CreateDepthStencilState();
    fn CreateRasterizerState();
    fn CreateSamplerState();
    fn CreateQuery();
    fn CreatePredicate();
    fn CreateCounter();
    fn CreateDeferredContext();
    fn OpenSharedResource();
    fn CheckFormatSupport();
    fn CheckMultisampleQualityLevels();
    fn CheckCounterInfo();
    fn CheckCounter();
    fn CheckFeatureSupport();
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn GetFeatureLevel();
    fn GetCreationFlags();
    fn GetDeviceRemovedReason();
    fn GetImmediateContext();
    fn SetExceptionMode();
    fn GetExceptionMode();
}
pub trait ID3D11Device1Impl: Sized + ID3D11DeviceImpl {
    fn GetImmediateContext1();
    fn CreateDeferredContext1();
    fn CreateBlendState1();
    fn CreateRasterizerState1();
    fn CreateDeviceContextState();
    fn OpenSharedResource1();
    fn OpenSharedResourceByName();
}
pub trait ID3D11Device2Impl: Sized + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn GetImmediateContext2();
    fn CreateDeferredContext2();
    fn GetResourceTiling();
    fn CheckMultisampleQualityLevels1();
}
pub trait ID3D11Device3Impl: Sized + ID3D11Device2Impl + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn CreateTexture2D1();
    fn CreateTexture3D1();
    fn CreateRasterizerState2();
    fn CreateShaderResourceView1();
    fn CreateUnorderedAccessView1();
    fn CreateRenderTargetView1();
    fn CreateQuery1();
    fn GetImmediateContext3();
    fn CreateDeferredContext3();
    fn WriteToSubresource();
    fn ReadFromSubresource();
}
pub trait ID3D11Device4Impl: Sized + ID3D11Device3Impl + ID3D11Device2Impl + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn RegisterDeviceRemovedEvent();
    fn UnregisterDeviceRemoved();
}
pub trait ID3D11Device5Impl: Sized + ID3D11Device4Impl + ID3D11Device3Impl + ID3D11Device2Impl + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn OpenSharedFence();
    fn CreateFence();
}
pub trait ID3D11DeviceChildImpl: Sized {
    fn GetDevice();
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
}
pub trait ID3D11DeviceContextImpl: Sized + ID3D11DeviceChildImpl {
    fn VSSetConstantBuffers();
    fn PSSetShaderResources();
    fn PSSetShader();
    fn PSSetSamplers();
    fn VSSetShader();
    fn DrawIndexed();
    fn Draw();
    fn Map();
    fn Unmap();
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
    fn Begin();
    fn End();
    fn GetData();
    fn SetPredication();
    fn GSSetShaderResources();
    fn GSSetSamplers();
    fn OMSetRenderTargets();
    fn OMSetRenderTargetsAndUnorderedAccessViews();
    fn OMSetBlendState();
    fn OMSetDepthStencilState();
    fn SOSetTargets();
    fn DrawAuto();
    fn DrawIndexedInstancedIndirect();
    fn DrawInstancedIndirect();
    fn Dispatch();
    fn DispatchIndirect();
    fn RSSetState();
    fn RSSetViewports();
    fn RSSetScissorRects();
    fn CopySubresourceRegion();
    fn CopyResource();
    fn UpdateSubresource();
    fn CopyStructureCount();
    fn ClearRenderTargetView();
    fn ClearUnorderedAccessViewUint();
    fn ClearUnorderedAccessViewFloat();
    fn ClearDepthStencilView();
    fn GenerateMips();
    fn SetResourceMinLOD();
    fn GetResourceMinLOD();
    fn ResolveSubresource();
    fn ExecuteCommandList();
    fn HSSetShaderResources();
    fn HSSetShader();
    fn HSSetSamplers();
    fn HSSetConstantBuffers();
    fn DSSetShaderResources();
    fn DSSetShader();
    fn DSSetSamplers();
    fn DSSetConstantBuffers();
    fn CSSetShaderResources();
    fn CSSetUnorderedAccessViews();
    fn CSSetShader();
    fn CSSetSamplers();
    fn CSSetConstantBuffers();
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
    fn OMGetRenderTargetsAndUnorderedAccessViews();
    fn OMGetBlendState();
    fn OMGetDepthStencilState();
    fn SOGetTargets();
    fn RSGetState();
    fn RSGetViewports();
    fn RSGetScissorRects();
    fn HSGetShaderResources();
    fn HSGetShader();
    fn HSGetSamplers();
    fn HSGetConstantBuffers();
    fn DSGetShaderResources();
    fn DSGetShader();
    fn DSGetSamplers();
    fn DSGetConstantBuffers();
    fn CSGetShaderResources();
    fn CSGetUnorderedAccessViews();
    fn CSGetShader();
    fn CSGetSamplers();
    fn CSGetConstantBuffers();
    fn ClearState();
    fn Flush();
    fn GetType();
    fn GetContextFlags();
    fn FinishCommandList();
}
pub trait ID3D11DeviceContext1Impl: Sized + ID3D11DeviceContextImpl + ID3D11DeviceChildImpl {
    fn CopySubresourceRegion1();
    fn UpdateSubresource1();
    fn DiscardResource();
    fn DiscardView();
    fn VSSetConstantBuffers1();
    fn HSSetConstantBuffers1();
    fn DSSetConstantBuffers1();
    fn GSSetConstantBuffers1();
    fn PSSetConstantBuffers1();
    fn CSSetConstantBuffers1();
    fn VSGetConstantBuffers1();
    fn HSGetConstantBuffers1();
    fn DSGetConstantBuffers1();
    fn GSGetConstantBuffers1();
    fn PSGetConstantBuffers1();
    fn CSGetConstantBuffers1();
    fn SwapDeviceContextState();
    fn ClearView();
    fn DiscardView1();
}
pub trait ID3D11DeviceContext2Impl: Sized + ID3D11DeviceContext1Impl + ID3D11DeviceContextImpl + ID3D11DeviceChildImpl {
    fn UpdateTileMappings();
    fn CopyTileMappings();
    fn CopyTiles();
    fn UpdateTiles();
    fn ResizeTilePool();
    fn TiledResourceBarrier();
    fn IsAnnotationEnabled();
    fn SetMarkerInt();
    fn BeginEventInt();
    fn EndEvent();
}
pub trait ID3D11DeviceContext3Impl: Sized + ID3D11DeviceContext2Impl + ID3D11DeviceContext1Impl + ID3D11DeviceContextImpl + ID3D11DeviceChildImpl {
    fn Flush1();
    fn SetHardwareProtectionState();
    fn GetHardwareProtectionState();
}
pub trait ID3D11DeviceContext4Impl: Sized + ID3D11DeviceContext3Impl + ID3D11DeviceContext2Impl + ID3D11DeviceContext1Impl + ID3D11DeviceContextImpl + ID3D11DeviceChildImpl {
    fn Signal();
    fn Wait();
}
pub trait ID3D11DomainShaderImpl: Sized + ID3D11DeviceChildImpl {}
pub trait ID3D11FenceImpl: Sized + ID3D11DeviceChildImpl {
    fn CreateSharedHandle();
    fn GetCompletedValue();
    fn SetEventOnCompletion();
}
pub trait ID3D11FunctionLinkingGraphImpl: Sized {
    fn CreateModuleInstance();
    fn SetInputSignature();
    fn SetOutputSignature();
    fn CallFunction();
    fn PassValue();
    fn PassValueWithSwizzle();
    fn GetLastError();
    fn GenerateHlsl();
}
pub trait ID3D11FunctionParameterReflectionImpl: Sized {
    fn GetDesc();
}
pub trait ID3D11FunctionReflectionImpl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetVariableByName();
    fn GetResourceBindingDescByName();
    fn GetFunctionParameter();
}
pub trait ID3D11GeometryShaderImpl: Sized + ID3D11DeviceChildImpl {}
pub trait ID3D11HullShaderImpl: Sized + ID3D11DeviceChildImpl {}
pub trait ID3D11InfoQueueImpl: Sized {
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
pub trait ID3D11InputLayoutImpl: Sized + ID3D11DeviceChildImpl {}
pub trait ID3D11LibraryReflectionImpl: Sized {
    fn GetDesc();
    fn GetFunctionByIndex();
}
pub trait ID3D11LinkerImpl: Sized {
    fn Link();
    fn UseLibrary();
    fn AddClipPlaneFromCBuffer();
}
pub trait ID3D11LinkingNodeImpl: Sized {}
pub trait ID3D11ModuleImpl: Sized {
    fn CreateInstance();
}
pub trait ID3D11ModuleInstanceImpl: Sized {
    fn BindConstantBuffer();
    fn BindConstantBufferByName();
    fn BindResource();
    fn BindResourceByName();
    fn BindSampler();
    fn BindSamplerByName();
    fn BindUnorderedAccessView();
    fn BindUnorderedAccessViewByName();
    fn BindResourceAsUnorderedAccessView();
    fn BindResourceAsUnorderedAccessViewByName();
}
pub trait ID3D11MultithreadImpl: Sized {
    fn Enter();
    fn Leave();
    fn SetMultithreadProtected();
    fn GetMultithreadProtected();
}
pub trait ID3D11PixelShaderImpl: Sized + ID3D11DeviceChildImpl {}
pub trait ID3D11PredicateImpl: Sized + ID3D11QueryImpl + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {}
pub trait ID3D11QueryImpl: Sized + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11Query1Impl: Sized + ID3D11QueryImpl + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
pub trait ID3D11RasterizerStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11RasterizerState1Impl: Sized + ID3D11RasterizerStateImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
pub trait ID3D11RasterizerState2Impl: Sized + ID3D11RasterizerState1Impl + ID3D11RasterizerStateImpl + ID3D11DeviceChildImpl {
    fn GetDesc2();
}
pub trait ID3D11RefDefaultTrackingOptionsImpl: Sized {
    fn SetTrackingOptions();
}
pub trait ID3D11RefTrackingOptionsImpl: Sized {
    fn SetTrackingOptions();
}
pub trait ID3D11RenderTargetViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11RenderTargetView1Impl: Sized + ID3D11RenderTargetViewImpl + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
pub trait ID3D11ResourceImpl: Sized + ID3D11DeviceChildImpl {
    fn GetType();
    fn SetEvictionPriority();
    fn GetEvictionPriority();
}
pub trait ID3D11SamplerStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11ShaderReflectionImpl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetInputParameterDesc();
    fn GetOutputParameterDesc();
    fn GetPatchConstantParameterDesc();
    fn GetVariableByName();
    fn GetResourceBindingDescByName();
    fn GetMovInstructionCount();
    fn GetMovcInstructionCount();
    fn GetConversionInstructionCount();
    fn GetBitwiseInstructionCount();
    fn GetGSInputPrimitive();
    fn IsSampleFrequencyShader();
    fn GetNumInterfaceSlots();
    fn GetMinFeatureLevel();
    fn GetThreadGroupSize();
    fn GetRequiresFlags();
}
pub trait ID3D11ShaderReflectionConstantBufferImpl: Sized {
    fn GetDesc();
    fn GetVariableByIndex();
    fn GetVariableByName();
}
pub trait ID3D11ShaderReflectionTypeImpl: Sized {
    fn GetDesc();
    fn GetMemberTypeByIndex();
    fn GetMemberTypeByName();
    fn GetMemberTypeName();
    fn IsEqual();
    fn GetSubType();
    fn GetBaseClass();
    fn GetNumInterfaces();
    fn GetInterfaceByIndex();
    fn IsOfType();
    fn ImplementsInterface();
}
pub trait ID3D11ShaderReflectionVariableImpl: Sized {
    fn GetDesc();
    fn GetType();
    fn GetBuffer();
    fn GetInterfaceSlot();
}
pub trait ID3D11ShaderResourceViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11ShaderResourceView1Impl: Sized + ID3D11ShaderResourceViewImpl + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
pub trait ID3D11ShaderTraceImpl: Sized {
    fn TraceReady();
    fn ResetTrace();
    fn GetTraceStats();
    fn PSSelectStamp();
    fn GetInitialRegisterContents();
    fn GetStep();
    fn GetWrittenRegister();
    fn GetReadRegister();
}
pub trait ID3D11ShaderTraceFactoryImpl: Sized {
    fn CreateShaderTrace();
}
pub trait ID3D11SwitchToRefImpl: Sized {
    fn SetUseRef();
    fn GetUseRef();
}
pub trait ID3D11Texture1DImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11Texture2DImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11Texture2D1Impl: Sized + ID3D11Texture2DImpl + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
pub trait ID3D11Texture3DImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11Texture3D1Impl: Sized + ID3D11Texture3DImpl + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
pub trait ID3D11TracingDeviceImpl: Sized {
    fn SetShaderTrackingOptionsByType();
    fn SetShaderTrackingOptions();
}
pub trait ID3D11UnorderedAccessViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11UnorderedAccessView1Impl: Sized + ID3D11UnorderedAccessViewImpl + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
pub trait ID3D11VertexShaderImpl: Sized + ID3D11DeviceChildImpl {}
pub trait ID3D11VideoContextImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDecoderBuffer();
    fn ReleaseDecoderBuffer();
    fn DecoderBeginFrame();
    fn DecoderEndFrame();
    fn SubmitDecoderBuffers();
    fn DecoderExtension();
    fn VideoProcessorSetOutputTargetRect();
    fn VideoProcessorSetOutputBackgroundColor();
    fn VideoProcessorSetOutputColorSpace();
    fn VideoProcessorSetOutputAlphaFillMode();
    fn VideoProcessorSetOutputConstriction();
    fn VideoProcessorSetOutputStereoMode();
    fn VideoProcessorSetOutputExtension();
    fn VideoProcessorGetOutputTargetRect();
    fn VideoProcessorGetOutputBackgroundColor();
    fn VideoProcessorGetOutputColorSpace();
    fn VideoProcessorGetOutputAlphaFillMode();
    fn VideoProcessorGetOutputConstriction();
    fn VideoProcessorGetOutputStereoMode();
    fn VideoProcessorGetOutputExtension();
    fn VideoProcessorSetStreamFrameFormat();
    fn VideoProcessorSetStreamColorSpace();
    fn VideoProcessorSetStreamOutputRate();
    fn VideoProcessorSetStreamSourceRect();
    fn VideoProcessorSetStreamDestRect();
    fn VideoProcessorSetStreamAlpha();
    fn VideoProcessorSetStreamPalette();
    fn VideoProcessorSetStreamPixelAspectRatio();
    fn VideoProcessorSetStreamLumaKey();
    fn VideoProcessorSetStreamStereoFormat();
    fn VideoProcessorSetStreamAutoProcessingMode();
    fn VideoProcessorSetStreamFilter();
    fn VideoProcessorSetStreamExtension();
    fn VideoProcessorGetStreamFrameFormat();
    fn VideoProcessorGetStreamColorSpace();
    fn VideoProcessorGetStreamOutputRate();
    fn VideoProcessorGetStreamSourceRect();
    fn VideoProcessorGetStreamDestRect();
    fn VideoProcessorGetStreamAlpha();
    fn VideoProcessorGetStreamPalette();
    fn VideoProcessorGetStreamPixelAspectRatio();
    fn VideoProcessorGetStreamLumaKey();
    fn VideoProcessorGetStreamStereoFormat();
    fn VideoProcessorGetStreamAutoProcessingMode();
    fn VideoProcessorGetStreamFilter();
    fn VideoProcessorGetStreamExtension();
    fn VideoProcessorBlt();
    fn NegotiateCryptoSessionKeyExchange();
    fn EncryptionBlt();
    fn DecryptionBlt();
    fn StartSessionKeyRefresh();
    fn FinishSessionKeyRefresh();
    fn GetEncryptionBltKey();
    fn NegotiateAuthenticatedChannelKeyExchange();
    fn QueryAuthenticatedChannel();
    fn ConfigureAuthenticatedChannel();
    fn VideoProcessorSetStreamRotation();
    fn VideoProcessorGetStreamRotation();
}
pub trait ID3D11VideoContext1Impl: Sized + ID3D11VideoContextImpl + ID3D11DeviceChildImpl {
    fn SubmitDecoderBuffers1();
    fn GetDataForNewHardwareKey();
    fn CheckCryptoSessionStatus();
    fn DecoderEnableDownsampling();
    fn DecoderUpdateDownsampling();
    fn VideoProcessorSetOutputColorSpace1();
    fn VideoProcessorSetOutputShaderUsage();
    fn VideoProcessorGetOutputColorSpace1();
    fn VideoProcessorGetOutputShaderUsage();
    fn VideoProcessorSetStreamColorSpace1();
    fn VideoProcessorSetStreamMirror();
    fn VideoProcessorGetStreamColorSpace1();
    fn VideoProcessorGetStreamMirror();
    fn VideoProcessorGetBehaviorHints();
}
pub trait ID3D11VideoContext2Impl: Sized + ID3D11VideoContext1Impl + ID3D11VideoContextImpl + ID3D11DeviceChildImpl {
    fn VideoProcessorSetOutputHDRMetaData();
    fn VideoProcessorGetOutputHDRMetaData();
    fn VideoProcessorSetStreamHDRMetaData();
    fn VideoProcessorGetStreamHDRMetaData();
}
pub trait ID3D11VideoContext3Impl: Sized + ID3D11VideoContext2Impl + ID3D11VideoContext1Impl + ID3D11VideoContextImpl + ID3D11DeviceChildImpl {
    fn DecoderBeginFrame1();
    fn SubmitDecoderBuffers2();
}
pub trait ID3D11VideoDecoderImpl: Sized + ID3D11DeviceChildImpl {
    fn GetCreationParameters();
    fn GetDriverHandle();
}
pub trait ID3D11VideoDecoderOutputViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11VideoDeviceImpl: Sized {
    fn CreateVideoDecoder();
    fn CreateVideoProcessor();
    fn CreateAuthenticatedChannel();
    fn CreateCryptoSession();
    fn CreateVideoDecoderOutputView();
    fn CreateVideoProcessorInputView();
    fn CreateVideoProcessorOutputView();
    fn CreateVideoProcessorEnumerator();
    fn GetVideoDecoderProfileCount();
    fn GetVideoDecoderProfile();
    fn CheckVideoDecoderFormat();
    fn GetVideoDecoderConfigCount();
    fn GetVideoDecoderConfig();
    fn GetContentProtectionCaps();
    fn CheckCryptoKeyExchange();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
}
pub trait ID3D11VideoDevice1Impl: Sized + ID3D11VideoDeviceImpl {
    fn GetCryptoSessionPrivateDataSize();
    fn GetVideoDecoderCaps();
    fn CheckVideoDecoderDownsampling();
    fn RecommendVideoDecoderDownsampleParameters();
}
pub trait ID3D11VideoDevice2Impl: Sized + ID3D11VideoDevice1Impl + ID3D11VideoDeviceImpl {
    fn CheckFeatureSupport();
    fn NegotiateCryptoSessionKeyExchangeMT();
}
pub trait ID3D11VideoProcessorImpl: Sized + ID3D11DeviceChildImpl {
    fn GetContentDesc();
    fn GetRateConversionCaps();
}
pub trait ID3D11VideoProcessorEnumeratorImpl: Sized + ID3D11DeviceChildImpl {
    fn GetVideoProcessorContentDesc();
    fn CheckVideoProcessorFormat();
    fn GetVideoProcessorCaps();
    fn GetVideoProcessorRateConversionCaps();
    fn GetVideoProcessorCustomRate();
    fn GetVideoProcessorFilterRange();
}
pub trait ID3D11VideoProcessorEnumerator1Impl: Sized + ID3D11VideoProcessorEnumeratorImpl + ID3D11DeviceChildImpl {
    fn CheckVideoProcessorFormatConversion();
}
pub trait ID3D11VideoProcessorInputViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11VideoProcessorOutputViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
pub trait ID3D11ViewImpl: Sized + ID3D11DeviceChildImpl {
    fn GetResource();
}
pub trait ID3DDeviceContextStateImpl: Sized + ID3D11DeviceChildImpl {}
pub trait ID3DUserDefinedAnnotationImpl: Sized {
    fn BeginEvent();
    fn EndEvent();
    fn SetMarker();
    fn GetStatus();
}
pub trait ID3DX11FFTImpl: Sized {
    fn SetForwardScale();
    fn GetForwardScale();
    fn SetInverseScale();
    fn GetInverseScale();
    fn AttachBuffersAndPrecompute();
    fn ForwardTransform();
    fn InverseTransform();
}
pub trait ID3DX11ScanImpl: Sized {
    fn SetScanDirection();
    fn Scan();
    fn Multiscan();
}
pub trait ID3DX11SegmentedScanImpl: Sized {
    fn SetScanDirection();
    fn SegScan();
}
