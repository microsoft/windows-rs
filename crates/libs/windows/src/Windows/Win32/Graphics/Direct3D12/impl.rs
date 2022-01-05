pub trait ID3D12CommandAllocatorImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn Reset();
}
pub trait ID3D12CommandListImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetType();
}
pub trait ID3D12CommandQueueImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn UpdateTileMappings();
    fn CopyTileMappings();
    fn ExecuteCommandLists();
    fn SetMarker();
    fn BeginEvent();
    fn EndEvent();
    fn Signal();
    fn Wait();
    fn GetTimestampFrequency();
    fn GetClockCalibration();
    fn GetDesc();
}
pub trait ID3D12CommandSignatureImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {}
pub trait ID3D12DebugImpl: Sized {
    fn EnableDebugLayer();
}
pub trait ID3D12Debug1Impl: Sized {
    fn EnableDebugLayer();
    fn SetEnableGPUBasedValidation();
    fn SetEnableSynchronizedCommandQueueValidation();
}
pub trait ID3D12Debug2Impl: Sized {
    fn SetGPUBasedValidationFlags();
}
pub trait ID3D12Debug3Impl: Sized + ID3D12DebugImpl {
    fn SetEnableGPUBasedValidation();
    fn SetEnableSynchronizedCommandQueueValidation();
    fn SetGPUBasedValidationFlags();
}
pub trait ID3D12Debug4Impl: Sized + ID3D12Debug3Impl + ID3D12DebugImpl {
    fn DisableDebugLayer();
}
pub trait ID3D12Debug5Impl: Sized + ID3D12Debug4Impl + ID3D12Debug3Impl + ID3D12DebugImpl {
    fn SetEnableAutoName();
}
pub trait ID3D12DebugCommandListImpl: Sized {
    fn AssertResourceState();
    fn SetFeatureMask();
    fn GetFeatureMask();
}
pub trait ID3D12DebugCommandList1Impl: Sized {
    fn AssertResourceState();
    fn SetDebugParameter();
    fn GetDebugParameter();
}
pub trait ID3D12DebugCommandList2Impl: Sized + ID3D12DebugCommandListImpl {
    fn SetDebugParameter();
    fn GetDebugParameter();
}
pub trait ID3D12DebugCommandQueueImpl: Sized {
    fn AssertResourceState();
}
pub trait ID3D12DebugDeviceImpl: Sized {
    fn SetFeatureMask();
    fn GetFeatureMask();
    fn ReportLiveDeviceObjects();
}
pub trait ID3D12DebugDevice1Impl: Sized {
    fn SetDebugParameter();
    fn GetDebugParameter();
    fn ReportLiveDeviceObjects();
}
pub trait ID3D12DebugDevice2Impl: Sized + ID3D12DebugDeviceImpl {
    fn SetDebugParameter();
    fn GetDebugParameter();
}
pub trait ID3D12DescriptorHeapImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
    fn GetCPUDescriptorHandleForHeapStart();
    fn GetGPUDescriptorHandleForHeapStart();
}
pub trait ID3D12DeviceImpl: Sized + ID3D12ObjectImpl {
    fn GetNodeCount();
    fn CreateCommandQueue();
    fn CreateCommandAllocator();
    fn CreateGraphicsPipelineState();
    fn CreateComputePipelineState();
    fn CreateCommandList();
    fn CheckFeatureSupport();
    fn CreateDescriptorHeap();
    fn GetDescriptorHandleIncrementSize();
    fn CreateRootSignature();
    fn CreateConstantBufferView();
    fn CreateShaderResourceView();
    fn CreateUnorderedAccessView();
    fn CreateRenderTargetView();
    fn CreateDepthStencilView();
    fn CreateSampler();
    fn CopyDescriptors();
    fn CopyDescriptorsSimple();
    fn GetResourceAllocationInfo();
    fn GetCustomHeapProperties();
    fn CreateCommittedResource();
    fn CreateHeap();
    fn CreatePlacedResource();
    fn CreateReservedResource();
    fn CreateSharedHandle();
    fn OpenSharedHandle();
    fn OpenSharedHandleByName();
    fn MakeResident();
    fn Evict();
    fn CreateFence();
    fn GetDeviceRemovedReason();
    fn GetCopyableFootprints();
    fn CreateQueryHeap();
    fn SetStablePowerState();
    fn CreateCommandSignature();
    fn GetResourceTiling();
    fn GetAdapterLuid();
}
pub trait ID3D12Device1Impl: Sized + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn CreatePipelineLibrary();
    fn SetEventOnMultipleFenceCompletion();
    fn SetResidencyPriority();
}
pub trait ID3D12Device2Impl: Sized + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn CreatePipelineState();
}
pub trait ID3D12Device3Impl: Sized + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn OpenExistingHeapFromAddress();
    fn OpenExistingHeapFromFileMapping();
    fn EnqueueMakeResident();
}
pub trait ID3D12Device4Impl: Sized + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn CreateCommandList1();
    fn CreateProtectedResourceSession();
    fn CreateCommittedResource1();
    fn CreateHeap1();
    fn CreateReservedResource1();
    fn GetResourceAllocationInfo1();
}
pub trait ID3D12Device5Impl: Sized + ID3D12Device4Impl + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn CreateLifetimeTracker();
    fn RemoveDevice();
    fn EnumerateMetaCommands();
    fn EnumerateMetaCommandParameters();
    fn CreateMetaCommand();
    fn CreateStateObject();
    fn GetRaytracingAccelerationStructurePrebuildInfo();
    fn CheckDriverMatchingIdentifier();
}
pub trait ID3D12Device6Impl: Sized + ID3D12Device5Impl + ID3D12Device4Impl + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn SetBackgroundProcessingMode();
}
pub trait ID3D12Device7Impl: Sized + ID3D12Device6Impl + ID3D12Device5Impl + ID3D12Device4Impl + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn AddToStateObject();
    fn CreateProtectedResourceSession1();
}
pub trait ID3D12Device8Impl: Sized + ID3D12Device7Impl + ID3D12Device6Impl + ID3D12Device5Impl + ID3D12Device4Impl + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn GetResourceAllocationInfo2();
    fn CreateCommittedResource2();
    fn CreatePlacedResource1();
    fn CreateSamplerFeedbackUnorderedAccessView();
    fn GetCopyableFootprints1();
}
pub trait ID3D12Device9Impl: Sized + ID3D12Device8Impl + ID3D12Device7Impl + ID3D12Device6Impl + ID3D12Device5Impl + ID3D12Device4Impl + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn CreateShaderCacheSession();
    fn ShaderCacheControl();
    fn CreateCommandQueue1();
}
pub trait ID3D12DeviceChildImpl: Sized + ID3D12ObjectImpl {
    fn GetDevice();
}
pub trait ID3D12DeviceRemovedExtendedDataImpl: Sized {
    fn GetAutoBreadcrumbsOutput();
    fn GetPageFaultAllocationOutput();
}
pub trait ID3D12DeviceRemovedExtendedData1Impl: Sized + ID3D12DeviceRemovedExtendedDataImpl {
    fn GetAutoBreadcrumbsOutput1();
    fn GetPageFaultAllocationOutput1();
}
pub trait ID3D12DeviceRemovedExtendedData2Impl: Sized + ID3D12DeviceRemovedExtendedData1Impl + ID3D12DeviceRemovedExtendedDataImpl {
    fn GetPageFaultAllocationOutput2();
    fn GetDeviceState();
}
pub trait ID3D12DeviceRemovedExtendedDataSettingsImpl: Sized {
    fn SetAutoBreadcrumbsEnablement();
    fn SetPageFaultEnablement();
    fn SetWatsonDumpEnablement();
}
pub trait ID3D12DeviceRemovedExtendedDataSettings1Impl: Sized + ID3D12DeviceRemovedExtendedDataSettingsImpl {
    fn SetBreadcrumbContextEnablement();
}
pub trait ID3D12FenceImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetCompletedValue();
    fn SetEventOnCompletion();
    fn Signal();
}
pub trait ID3D12Fence1Impl: Sized + ID3D12FenceImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetCreationFlags();
}
pub trait ID3D12FunctionParameterReflectionImpl: Sized {
    fn GetDesc();
}
pub trait ID3D12FunctionReflectionImpl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetVariableByName();
    fn GetResourceBindingDescByName();
    fn GetFunctionParameter();
}
pub trait ID3D12GraphicsCommandListImpl: Sized + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn Close();
    fn Reset();
    fn ClearState();
    fn DrawInstanced();
    fn DrawIndexedInstanced();
    fn Dispatch();
    fn CopyBufferRegion();
    fn CopyTextureRegion();
    fn CopyResource();
    fn CopyTiles();
    fn ResolveSubresource();
    fn IASetPrimitiveTopology();
    fn RSSetViewports();
    fn RSSetScissorRects();
    fn OMSetBlendFactor();
    fn OMSetStencilRef();
    fn SetPipelineState();
    fn ResourceBarrier();
    fn ExecuteBundle();
    fn SetDescriptorHeaps();
    fn SetComputeRootSignature();
    fn SetGraphicsRootSignature();
    fn SetComputeRootDescriptorTable();
    fn SetGraphicsRootDescriptorTable();
    fn SetComputeRoot32BitConstant();
    fn SetGraphicsRoot32BitConstant();
    fn SetComputeRoot32BitConstants();
    fn SetGraphicsRoot32BitConstants();
    fn SetComputeRootConstantBufferView();
    fn SetGraphicsRootConstantBufferView();
    fn SetComputeRootShaderResourceView();
    fn SetGraphicsRootShaderResourceView();
    fn SetComputeRootUnorderedAccessView();
    fn SetGraphicsRootUnorderedAccessView();
    fn IASetIndexBuffer();
    fn IASetVertexBuffers();
    fn SOSetTargets();
    fn OMSetRenderTargets();
    fn ClearDepthStencilView();
    fn ClearRenderTargetView();
    fn ClearUnorderedAccessViewUint();
    fn ClearUnorderedAccessViewFloat();
    fn DiscardResource();
    fn BeginQuery();
    fn EndQuery();
    fn ResolveQueryData();
    fn SetPredication();
    fn SetMarker();
    fn BeginEvent();
    fn EndEvent();
    fn ExecuteIndirect();
}
pub trait ID3D12GraphicsCommandList1Impl: Sized + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn AtomicCopyBufferUINT();
    fn AtomicCopyBufferUINT64();
    fn OMSetDepthBounds();
    fn SetSamplePositions();
    fn ResolveSubresourceRegion();
    fn SetViewInstanceMask();
}
pub trait ID3D12GraphicsCommandList2Impl: Sized + ID3D12GraphicsCommandList1Impl + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn WriteBufferImmediate();
}
pub trait ID3D12GraphicsCommandList3Impl: Sized + ID3D12GraphicsCommandList2Impl + ID3D12GraphicsCommandList1Impl + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn SetProtectedResourceSession();
}
pub trait ID3D12GraphicsCommandList4Impl: Sized + ID3D12GraphicsCommandList3Impl + ID3D12GraphicsCommandList2Impl + ID3D12GraphicsCommandList1Impl + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn BeginRenderPass();
    fn EndRenderPass();
    fn InitializeMetaCommand();
    fn ExecuteMetaCommand();
    fn BuildRaytracingAccelerationStructure();
    fn EmitRaytracingAccelerationStructurePostbuildInfo();
    fn CopyRaytracingAccelerationStructure();
    fn SetPipelineState1();
    fn DispatchRays();
}
pub trait ID3D12GraphicsCommandList5Impl: Sized + ID3D12GraphicsCommandList4Impl + ID3D12GraphicsCommandList3Impl + ID3D12GraphicsCommandList2Impl + ID3D12GraphicsCommandList1Impl + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn RSSetShadingRate();
    fn RSSetShadingRateImage();
}
pub trait ID3D12GraphicsCommandList6Impl: Sized + ID3D12GraphicsCommandList5Impl + ID3D12GraphicsCommandList4Impl + ID3D12GraphicsCommandList3Impl + ID3D12GraphicsCommandList2Impl + ID3D12GraphicsCommandList1Impl + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn DispatchMesh();
}
pub trait ID3D12HeapImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
}
pub trait ID3D12Heap1Impl: Sized + ID3D12HeapImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetProtectedResourceSession();
}
pub trait ID3D12InfoQueueImpl: Sized {
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
pub trait ID3D12InfoQueue1Impl: Sized + ID3D12InfoQueueImpl {
    fn RegisterMessageCallback();
    fn UnregisterMessageCallback();
}
pub trait ID3D12LibraryReflectionImpl: Sized {
    fn GetDesc();
    fn GetFunctionByIndex();
}
pub trait ID3D12LifetimeOwnerImpl: Sized {
    fn LifetimeStateUpdated();
}
pub trait ID3D12LifetimeTrackerImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn DestroyOwnedObject();
}
pub trait ID3D12MetaCommandImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetRequiredParameterResourceSize();
}
pub trait ID3D12ObjectImpl: Sized {
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn SetName();
}
pub trait ID3D12PageableImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {}
pub trait ID3D12PipelineLibraryImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn StorePipeline();
    fn LoadGraphicsPipeline();
    fn LoadComputePipeline();
    fn GetSerializedSize();
    fn Serialize();
}
pub trait ID3D12PipelineLibrary1Impl: Sized + ID3D12PipelineLibraryImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn LoadPipeline();
}
pub trait ID3D12PipelineStateImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetCachedBlob();
}
pub trait ID3D12ProtectedResourceSessionImpl: Sized + ID3D12ProtectedSessionImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
}
pub trait ID3D12ProtectedResourceSession1Impl: Sized + ID3D12ProtectedResourceSessionImpl + ID3D12ProtectedSessionImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc1();
}
pub trait ID3D12ProtectedSessionImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetStatusFence();
    fn GetSessionStatus();
}
pub trait ID3D12QueryHeapImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {}
pub trait ID3D12ResourceImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
    fn GetGPUVirtualAddress();
    fn WriteToSubresource();
    fn ReadFromSubresource();
    fn GetHeapProperties();
}
pub trait ID3D12Resource1Impl: Sized + ID3D12ResourceImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetProtectedResourceSession();
}
pub trait ID3D12Resource2Impl: Sized + ID3D12Resource1Impl + ID3D12ResourceImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc1();
}
pub trait ID3D12RootSignatureImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {}
pub trait ID3D12RootSignatureDeserializerImpl: Sized {
    fn GetRootSignatureDesc();
}
pub trait ID3D12SDKConfigurationImpl: Sized {
    fn SetSDKVersion();
}
pub trait ID3D12ShaderCacheSessionImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn FindValue();
    fn StoreValue();
    fn SetDeleteOnDestroy();
    fn GetDesc();
}
pub trait ID3D12ShaderReflectionImpl: Sized {
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
pub trait ID3D12ShaderReflectionConstantBufferImpl: Sized {
    fn GetDesc();
    fn GetVariableByIndex();
    fn GetVariableByName();
}
pub trait ID3D12ShaderReflectionTypeImpl: Sized {
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
pub trait ID3D12ShaderReflectionVariableImpl: Sized {
    fn GetDesc();
    fn GetType();
    fn GetBuffer();
    fn GetInterfaceSlot();
}
pub trait ID3D12SharingContractImpl: Sized {
    fn Present();
    fn SharedFenceSignal();
    fn BeginCapturableWork();
    fn EndCapturableWork();
}
pub trait ID3D12StateObjectImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {}
pub trait ID3D12StateObjectPropertiesImpl: Sized {
    fn GetShaderIdentifier();
    fn GetShaderStackSize();
    fn GetPipelineStackSize();
    fn SetPipelineStackSize();
}
pub trait ID3D12SwapChainAssistantImpl: Sized {
    fn GetLUID();
    fn GetSwapChainObject();
    fn GetCurrentResourceAndCommandQueue();
    fn InsertImplicitSync();
}
pub trait ID3D12ToolsImpl: Sized {
    fn EnableShaderInstrumentation();
    fn ShaderInstrumentationEnabled();
}
pub trait ID3D12VersionedRootSignatureDeserializerImpl: Sized {
    fn GetRootSignatureDescAtVersion();
    fn GetUnconvertedRootSignatureDesc();
}
