pub trait IAdvancedMediaCaptureImpl: Sized {
    fn GetAdvancedMediaCaptureSettings();
}
impl IAdvancedMediaCaptureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedMediaCaptureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedMediaCaptureVtbl {
        unsafe extern "system" fn GetAdvancedMediaCaptureSettings<Impl: IAdvancedMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAdvancedMediaCaptureSettings: GetAdvancedMediaCaptureSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedMediaCapture as ::windows::core::Interface>::IID
    }
}
pub trait IAdvancedMediaCaptureInitializationSettingsImpl: Sized {
    fn SetDirectxDeviceManager();
}
impl IAdvancedMediaCaptureInitializationSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedMediaCaptureInitializationSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedMediaCaptureInitializationSettingsVtbl {
        unsafe extern "system" fn SetDirectxDeviceManager<Impl: IAdvancedMediaCaptureInitializationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetDirectxDeviceManager: SetDirectxDeviceManager::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedMediaCaptureInitializationSettings as ::windows::core::Interface>::IID
    }
}
pub trait IAdvancedMediaCaptureSettingsImpl: Sized {
    fn GetDirectxDeviceManager();
}
impl IAdvancedMediaCaptureSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedMediaCaptureSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedMediaCaptureSettingsVtbl {
        unsafe extern "system" fn GetDirectxDeviceManager<Impl: IAdvancedMediaCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDirectxDeviceManager: GetDirectxDeviceManager::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedMediaCaptureSettings as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSourceProviderImpl: Sized {
    fn ProvideInput();
}
impl IAudioSourceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSourceProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSourceProviderVtbl {
        unsafe extern "system" fn ProvideInput<Impl: IAudioSourceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsamplecount: u32, pdwchannelcount: *mut u32, pinterleavedaudiodata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ProvideInput: ProvideInput::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSourceProvider as ::windows::core::Interface>::IID
    }
}
pub trait IClusterDetectorImpl: Sized {
    fn Initialize();
    fn Detect();
}
impl IClusterDetectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClusterDetectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClusterDetectorVtbl {
        unsafe extern "system" fn Initialize<Impl: IClusterDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wbaseentrylevel: u16, wclusterentrylevel: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Detect<Impl: IClusterDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxnumclusters: u32, fminclusterduration: f32, fmaxclusterduration: f32, psrctoc: ::windows::core::RawPtr, ppdsttoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Detect: Detect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClusterDetector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICodecAPIImpl: Sized {
    fn IsSupported();
    fn IsModifiable();
    fn GetParameterRange();
    fn GetParameterValues();
    fn GetDefaultValue();
    fn GetValue();
    fn SetValue();
    fn RegisterForEvent();
    fn UnregisterForEvent();
    fn SetAllDefaults();
    fn SetValueWithNotify();
    fn SetAllDefaultsWithNotify();
    fn GetAllSettings();
    fn SetAllSettings();
    fn SetAllSettingsWithNotify();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICodecAPIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICodecAPIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICodecAPIVtbl {
        unsafe extern "system" fn IsSupported<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, api: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsModifiable<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, api: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParameterRange<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, api: *const ::windows::core::GUID, valuemin: *mut super::super::System::Com::VARIANT, valuemax: *mut super::super::System::Com::VARIANT, steppingdelta: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParameterValues<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, api: *const ::windows::core::GUID, values: *mut *mut super::super::System::Com::VARIANT, valuescount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultValue<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, api: *const ::windows::core::GUID, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, api: *const ::windows::core::GUID, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, api: *const ::windows::core::GUID, value: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterForEvent<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, api: *const ::windows::core::GUID, userdata: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterForEvent<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, api: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllDefaults<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValueWithNotify<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, api: *const ::windows::core::GUID, value: *mut super::super::System::Com::VARIANT, changedparam: *mut *mut ::windows::core::GUID, changedparamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllDefaultsWithNotify<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changedparam: *mut *mut ::windows::core::GUID, changedparamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllSettings<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__icodecapi0000: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllSettings<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__icodecapi0001: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllSettingsWithNotify<Impl: ICodecAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__icodecapi0002: ::windows::core::RawPtr, changedparam: *mut *mut ::windows::core::GUID, changedparamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            IsModifiable: IsModifiable::<Impl, IMPL_OFFSET>,
            GetParameterRange: GetParameterRange::<Impl, IMPL_OFFSET>,
            GetParameterValues: GetParameterValues::<Impl, IMPL_OFFSET>,
            GetDefaultValue: GetDefaultValue::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            RegisterForEvent: RegisterForEvent::<Impl, IMPL_OFFSET>,
            UnregisterForEvent: UnregisterForEvent::<Impl, IMPL_OFFSET>,
            SetAllDefaults: SetAllDefaults::<Impl, IMPL_OFFSET>,
            SetValueWithNotify: SetValueWithNotify::<Impl, IMPL_OFFSET>,
            SetAllDefaultsWithNotify: SetAllDefaultsWithNotify::<Impl, IMPL_OFFSET>,
            GetAllSettings: GetAllSettings::<Impl, IMPL_OFFSET>,
            SetAllSettings: SetAllSettings::<Impl, IMPL_OFFSET>,
            SetAllSettingsWithNotify: SetAllSettingsWithNotify::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICodecAPI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoDecodeCommandListImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12CommandListImpl {
    fn Close();
    fn Reset();
    fn ClearState();
    fn ResourceBarrier();
    fn DiscardResource();
    fn BeginQuery();
    fn EndQuery();
    fn ResolveQueryData();
    fn SetPredication();
    fn SetMarker();
    fn BeginEvent();
    fn EndEvent();
    fn DecodeFrame();
    fn WriteBufferImmediate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoDecodeCommandListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDecodeCommandListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDecodeCommandListVtbl {
        unsafe extern "system" fn Close<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearState<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResourceBarrier<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbarriers: u32, pbarriers: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_BARRIER) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscardResource<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pregion: *const super::super::Graphics::Direct3D12::D3D12_DISCARD_REGION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginQuery<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndQuery<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResolveQueryData<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: ::windows::core::RawPtr, aligneddestinationbufferoffset: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr, alignedbufferoffset: u64, operation: super::super::Graphics::Direct3D12::D3D12_PREDICATION_OP) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarker<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginEvent<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEvent<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecodeFrame<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteBufferImmediate<Impl: ID3D12VideoDecodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, pparams: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12CommandListVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Close: Close::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            ClearState: ClearState::<Impl, IMPL_OFFSET>,
            ResourceBarrier: ResourceBarrier::<Impl, IMPL_OFFSET>,
            DiscardResource: DiscardResource::<Impl, IMPL_OFFSET>,
            BeginQuery: BeginQuery::<Impl, IMPL_OFFSET>,
            EndQuery: EndQuery::<Impl, IMPL_OFFSET>,
            ResolveQueryData: ResolveQueryData::<Impl, IMPL_OFFSET>,
            SetPredication: SetPredication::<Impl, IMPL_OFFSET>,
            SetMarker: SetMarker::<Impl, IMPL_OFFSET>,
            BeginEvent: BeginEvent::<Impl, IMPL_OFFSET>,
            EndEvent: EndEvent::<Impl, IMPL_OFFSET>,
            DecodeFrame: DecodeFrame::<Impl, IMPL_OFFSET>,
            WriteBufferImmediate: WriteBufferImmediate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDecodeCommandList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoDecodeCommandList1Impl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12CommandListImpl + ID3D12VideoDecodeCommandListImpl {
    fn DecodeFrame1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoDecodeCommandList1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDecodeCommandList1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDecodeCommandList1Vtbl {
        unsafe extern "system" fn DecodeFrame1<Impl: ID3D12VideoDecodeCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ID3D12VideoDecodeCommandListVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), DecodeFrame1: DecodeFrame1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDecodeCommandList1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoDecodeCommandList2Impl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12CommandListImpl + ID3D12VideoDecodeCommandListImpl + ID3D12VideoDecodeCommandList1Impl {
    fn SetProtectedResourceSession();
    fn InitializeExtensionCommand();
    fn ExecuteExtensionCommand();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoDecodeCommandList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDecodeCommandList2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDecodeCommandList2Vtbl {
        unsafe extern "system" fn SetProtectedResourceSession<Impl: ID3D12VideoDecodeCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotectedresourcesession: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeExtensionCommand<Impl: ID3D12VideoDecodeCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextensioncommand: ::windows::core::RawPtr, pinitializationparameters: *const ::core::ffi::c_void, initializationparameterssizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteExtensionCommand<Impl: ID3D12VideoDecodeCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextensioncommand: ::windows::core::RawPtr, pexecutionparameters: *const ::core::ffi::c_void, executionparameterssizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12VideoDecodeCommandList1Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProtectedResourceSession: SetProtectedResourceSession::<Impl, IMPL_OFFSET>,
            InitializeExtensionCommand: InitializeExtensionCommand::<Impl, IMPL_OFFSET>,
            ExecuteExtensionCommand: ExecuteExtensionCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDecodeCommandList2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D12VideoDecoderImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl {
    fn GetDesc();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D12VideoDecoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDecoderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDecoderVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12VideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_VIDEO_DECODER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ID3D12PageableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDecoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D12VideoDecoder1Impl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl + ID3D12VideoDecoderImpl {
    fn GetProtectedResourceSession();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D12VideoDecoder1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDecoder1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDecoder1Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Impl: ID3D12VideoDecoder1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12VideoDecoderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetProtectedResourceSession: GetProtectedResourceSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDecoder1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoDecoderHeapImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl {
    fn GetDesc();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoDecoderHeapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDecoderHeapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDecoderHeapVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12VideoDecoderHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_VIDEO_DECODER_HEAP_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ID3D12PageableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDecoderHeap as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoDecoderHeap1Impl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl + ID3D12VideoDecoderHeapImpl {
    fn GetProtectedResourceSession();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoDecoderHeap1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDecoderHeap1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDecoderHeap1Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Impl: ID3D12VideoDecoderHeap1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12VideoDecoderHeapVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetProtectedResourceSession: GetProtectedResourceSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDecoderHeap1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoDeviceImpl: Sized {
    fn CheckFeatureSupport();
    fn CreateVideoDecoder();
    fn CreateVideoDecoderHeap();
    fn CreateVideoProcessor();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDeviceVtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID3D12VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, featurevideo: D3D12_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoDecoder<Impl: ID3D12VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_VIDEO_DECODER_DESC, riid: *const ::windows::core::GUID, ppvideodecoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoDecoderHeap<Impl: ID3D12VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC, riid: *const ::windows::core::GUID, ppvideodecoderheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoProcessor<Impl: ID3D12VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, numinputstreamdescs: u32, pinputstreamdescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC, riid: *const ::windows::core::GUID, ppvideoprocessor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CheckFeatureSupport: CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            CreateVideoDecoder: CreateVideoDecoder::<Impl, IMPL_OFFSET>,
            CreateVideoDecoderHeap: CreateVideoDecoderHeap::<Impl, IMPL_OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoDevice1Impl: Sized + ID3D12VideoDeviceImpl {
    fn CreateVideoMotionEstimator();
    fn CreateVideoMotionVectorHeap();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoDevice1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDevice1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDevice1Vtbl {
        unsafe extern "system" fn CreateVideoMotionEstimator<Impl: ID3D12VideoDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_VIDEO_MOTION_ESTIMATOR_DESC, pprotectedresourcesession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvideomotionestimator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoMotionVectorHeap<Impl: ID3D12VideoDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC, pprotectedresourcesession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvideomotionvectorheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12VideoDeviceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateVideoMotionEstimator: CreateVideoMotionEstimator::<Impl, IMPL_OFFSET>,
            CreateVideoMotionVectorHeap: CreateVideoMotionVectorHeap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDevice1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoDevice2Impl: Sized + ID3D12VideoDeviceImpl + ID3D12VideoDevice1Impl {
    fn CreateVideoDecoder1();
    fn CreateVideoDecoderHeap1();
    fn CreateVideoProcessor1();
    fn CreateVideoExtensionCommand();
    fn ExecuteExtensionCommand();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDevice2Vtbl {
        unsafe extern "system" fn CreateVideoDecoder1<Impl: ID3D12VideoDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_VIDEO_DECODER_DESC, pprotectedresourcesession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvideodecoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoDecoderHeap1<Impl: ID3D12VideoDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC, pprotectedresourcesession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvideodecoderheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoProcessor1<Impl: ID3D12VideoDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, numinputstreamdescs: u32, pinputstreamdescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC, pprotectedresourcesession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvideoprocessor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoExtensionCommand<Impl: ID3D12VideoDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_VIDEO_EXTENSION_COMMAND_DESC, pcreationparameters: *const ::core::ffi::c_void, creationparametersdatasizeinbytes: usize, pprotectedresourcesession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvideoextensioncommand: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteExtensionCommand<Impl: ID3D12VideoDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextensioncommand: ::windows::core::RawPtr, pexecutionparameters: *const ::core::ffi::c_void, executionparameterssizeinbytes: usize, poutputdata: *mut ::core::ffi::c_void, outputdatasizeinbytes: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12VideoDevice1Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateVideoDecoder1: CreateVideoDecoder1::<Impl, IMPL_OFFSET>,
            CreateVideoDecoderHeap1: CreateVideoDecoderHeap1::<Impl, IMPL_OFFSET>,
            CreateVideoProcessor1: CreateVideoProcessor1::<Impl, IMPL_OFFSET>,
            CreateVideoExtensionCommand: CreateVideoExtensionCommand::<Impl, IMPL_OFFSET>,
            ExecuteExtensionCommand: ExecuteExtensionCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoDevice3Impl: Sized + ID3D12VideoDeviceImpl + ID3D12VideoDevice1Impl + ID3D12VideoDevice2Impl {
    fn CreateVideoEncoder();
    fn CreateVideoEncoderHeap();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoDevice3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoDevice3Vtbl {
        unsafe extern "system" fn CreateVideoEncoder<Impl: ID3D12VideoDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_VIDEO_ENCODER_DESC, riid: *const ::windows::core::GUID, ppvideoencoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoEncoderHeap<Impl: ID3D12VideoDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_VIDEO_ENCODER_HEAP_DESC, riid: *const ::windows::core::GUID, ppvideoencoderheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12VideoDevice2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateVideoEncoder: CreateVideoEncoder::<Impl, IMPL_OFFSET>,
            CreateVideoEncoderHeap: CreateVideoEncoderHeap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoDevice3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D12VideoEncodeCommandListImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12CommandListImpl {
    fn Close();
    fn Reset();
    fn ClearState();
    fn ResourceBarrier();
    fn DiscardResource();
    fn BeginQuery();
    fn EndQuery();
    fn ResolveQueryData();
    fn SetPredication();
    fn SetMarker();
    fn BeginEvent();
    fn EndEvent();
    fn EstimateMotion();
    fn ResolveMotionVectorHeap();
    fn WriteBufferImmediate();
    fn SetProtectedResourceSession();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D12VideoEncodeCommandListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoEncodeCommandListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoEncodeCommandListVtbl {
        unsafe extern "system" fn Close<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearState<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResourceBarrier<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbarriers: u32, pbarriers: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_BARRIER) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscardResource<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pregion: *const super::super::Graphics::Direct3D12::D3D12_DISCARD_REGION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginQuery<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndQuery<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResolveQueryData<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: ::windows::core::RawPtr, aligneddestinationbufferoffset: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr, alignedbufferoffset: u64, operation: super::super::Graphics::Direct3D12::D3D12_PREDICATION_OP) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarker<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginEvent<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEvent<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EstimateMotion<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmotionestimator: ::windows::core::RawPtr, poutputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT, pinputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_INPUT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResolveMotionVectorHeap<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT, pinputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteBufferImmediate<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, pparams: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProtectedResourceSession<Impl: ID3D12VideoEncodeCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotectedresourcesession: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12CommandListVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Close: Close::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            ClearState: ClearState::<Impl, IMPL_OFFSET>,
            ResourceBarrier: ResourceBarrier::<Impl, IMPL_OFFSET>,
            DiscardResource: DiscardResource::<Impl, IMPL_OFFSET>,
            BeginQuery: BeginQuery::<Impl, IMPL_OFFSET>,
            EndQuery: EndQuery::<Impl, IMPL_OFFSET>,
            ResolveQueryData: ResolveQueryData::<Impl, IMPL_OFFSET>,
            SetPredication: SetPredication::<Impl, IMPL_OFFSET>,
            SetMarker: SetMarker::<Impl, IMPL_OFFSET>,
            BeginEvent: BeginEvent::<Impl, IMPL_OFFSET>,
            EndEvent: EndEvent::<Impl, IMPL_OFFSET>,
            EstimateMotion: EstimateMotion::<Impl, IMPL_OFFSET>,
            ResolveMotionVectorHeap: ResolveMotionVectorHeap::<Impl, IMPL_OFFSET>,
            WriteBufferImmediate: WriteBufferImmediate::<Impl, IMPL_OFFSET>,
            SetProtectedResourceSession: SetProtectedResourceSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoEncodeCommandList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D12VideoEncodeCommandList1Impl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12CommandListImpl + ID3D12VideoEncodeCommandListImpl {
    fn InitializeExtensionCommand();
    fn ExecuteExtensionCommand();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D12VideoEncodeCommandList1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoEncodeCommandList1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoEncodeCommandList1Vtbl {
        unsafe extern "system" fn InitializeExtensionCommand<Impl: ID3D12VideoEncodeCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextensioncommand: ::windows::core::RawPtr, pinitializationparameters: *const ::core::ffi::c_void, initializationparameterssizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteExtensionCommand<Impl: ID3D12VideoEncodeCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextensioncommand: ::windows::core::RawPtr, pexecutionparameters: *const ::core::ffi::c_void, executionparameterssizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12VideoEncodeCommandListVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeExtensionCommand: InitializeExtensionCommand::<Impl, IMPL_OFFSET>,
            ExecuteExtensionCommand: ExecuteExtensionCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoEncodeCommandList1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoEncodeCommandList2Impl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12CommandListImpl + ID3D12VideoEncodeCommandListImpl + ID3D12VideoEncodeCommandList1Impl {
    fn EncodeFrame();
    fn ResolveEncoderOutputMetadata();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoEncodeCommandList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoEncodeCommandList2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoEncodeCommandList2Vtbl {
        unsafe extern "system" fn EncodeFrame<Impl: ID3D12VideoEncodeCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pencoder: ::windows::core::RawPtr, pheap: ::windows::core::RawPtr, pinputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResolveEncoderOutputMetadata<Impl: ID3D12VideoEncodeCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12VideoEncodeCommandList1Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EncodeFrame: EncodeFrame::<Impl, IMPL_OFFSET>,
            ResolveEncoderOutputMetadata: ResolveEncoderOutputMetadata::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoEncodeCommandList2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoEncoderImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl {
    fn GetNodeMask();
    fn GetEncoderFlags();
    fn GetCodec();
    fn GetCodecProfile();
    fn GetCodecConfiguration();
    fn GetInputFormat();
    fn GetMaxMotionEstimationPrecision();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoEncoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoEncoderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoEncoderVtbl {
        unsafe extern "system" fn GetNodeMask<Impl: ID3D12VideoEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEncoderFlags<Impl: ID3D12VideoEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_VIDEO_ENCODER_FLAGS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodec<Impl: ID3D12VideoEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_VIDEO_ENCODER_CODEC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecProfile<Impl: ID3D12VideoEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dstprofile: D3D12_VIDEO_ENCODER_PROFILE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecConfiguration<Impl: ID3D12VideoEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dstcodecconfig: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputFormat<Impl: ID3D12VideoEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Graphics::Dxgi::Common::DXGI_FORMAT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxMotionEstimationPrecision<Impl: ID3D12VideoEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12PageableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetNodeMask: GetNodeMask::<Impl, IMPL_OFFSET>,
            GetEncoderFlags: GetEncoderFlags::<Impl, IMPL_OFFSET>,
            GetCodec: GetCodec::<Impl, IMPL_OFFSET>,
            GetCodecProfile: GetCodecProfile::<Impl, IMPL_OFFSET>,
            GetCodecConfiguration: GetCodecConfiguration::<Impl, IMPL_OFFSET>,
            GetInputFormat: GetInputFormat::<Impl, IMPL_OFFSET>,
            GetMaxMotionEstimationPrecision: GetMaxMotionEstimationPrecision::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D12VideoEncoderHeapImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl {
    fn GetNodeMask();
    fn GetEncoderHeapFlags();
    fn GetCodec();
    fn GetCodecProfile();
    fn GetCodecLevel();
    fn GetResolutionListCount();
    fn GetResolutionList();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D12VideoEncoderHeapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoEncoderHeapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoEncoderHeapVtbl {
        unsafe extern "system" fn GetNodeMask<Impl: ID3D12VideoEncoderHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEncoderHeapFlags<Impl: ID3D12VideoEncoderHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_VIDEO_ENCODER_HEAP_FLAGS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodec<Impl: ID3D12VideoEncoderHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_VIDEO_ENCODER_CODEC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecProfile<Impl: ID3D12VideoEncoderHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dstprofile: D3D12_VIDEO_ENCODER_PROFILE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecLevel<Impl: ID3D12VideoEncoderHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dstlevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResolutionListCount<Impl: ID3D12VideoEncoderHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResolutionList<Impl: ID3D12VideoEncoderHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolutionslistcount: u32, presolutionlist: *mut D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12PageableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetNodeMask: GetNodeMask::<Impl, IMPL_OFFSET>,
            GetEncoderHeapFlags: GetEncoderHeapFlags::<Impl, IMPL_OFFSET>,
            GetCodec: GetCodec::<Impl, IMPL_OFFSET>,
            GetCodecProfile: GetCodecProfile::<Impl, IMPL_OFFSET>,
            GetCodecLevel: GetCodecLevel::<Impl, IMPL_OFFSET>,
            GetResolutionListCount: GetResolutionListCount::<Impl, IMPL_OFFSET>,
            GetResolutionList: GetResolutionList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoEncoderHeap as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D12VideoExtensionCommandImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl {
    fn GetDesc();
    fn GetProtectedResourceSession();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D12VideoExtensionCommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoExtensionCommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoExtensionCommandVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12VideoExtensionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_VIDEO_EXTENSION_COMMAND_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtectedResourceSession<Impl: ID3D12VideoExtensionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12PageableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetProtectedResourceSession: GetProtectedResourceSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoExtensionCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoMotionEstimatorImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl {
    fn GetDesc();
    fn GetProtectedResourceSession();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoMotionEstimatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoMotionEstimatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoMotionEstimatorVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12VideoMotionEstimatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_VIDEO_MOTION_ESTIMATOR_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtectedResourceSession<Impl: ID3D12VideoMotionEstimatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12PageableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetProtectedResourceSession: GetProtectedResourceSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoMotionEstimator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoMotionVectorHeapImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl {
    fn GetDesc();
    fn GetProtectedResourceSession();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoMotionVectorHeapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoMotionVectorHeapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoMotionVectorHeapVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12VideoMotionVectorHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtectedResourceSession<Impl: ID3D12VideoMotionVectorHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12PageableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetProtectedResourceSession: GetProtectedResourceSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoMotionVectorHeap as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D12VideoProcessCommandListImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12CommandListImpl {
    fn Close();
    fn Reset();
    fn ClearState();
    fn ResourceBarrier();
    fn DiscardResource();
    fn BeginQuery();
    fn EndQuery();
    fn ResolveQueryData();
    fn SetPredication();
    fn SetMarker();
    fn BeginEvent();
    fn EndEvent();
    fn ProcessFrames();
    fn WriteBufferImmediate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D12VideoProcessCommandListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoProcessCommandListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoProcessCommandListVtbl {
        unsafe extern "system" fn Close<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearState<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResourceBarrier<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbarriers: u32, pbarriers: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_BARRIER) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscardResource<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pregion: *const super::super::Graphics::Direct3D12::D3D12_DISCARD_REGION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginQuery<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndQuery<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResolveQueryData<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: ::windows::core::RawPtr, aligneddestinationbufferoffset: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr, alignedbufferoffset: u64, operation: super::super::Graphics::Direct3D12::D3D12_PREDICATION_OP) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarker<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginEvent<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEvent<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessFrames<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, numinputstreams: u32, pinputarguments: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteBufferImmediate<Impl: ID3D12VideoProcessCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, pparams: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12CommandListVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Close: Close::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            ClearState: ClearState::<Impl, IMPL_OFFSET>,
            ResourceBarrier: ResourceBarrier::<Impl, IMPL_OFFSET>,
            DiscardResource: DiscardResource::<Impl, IMPL_OFFSET>,
            BeginQuery: BeginQuery::<Impl, IMPL_OFFSET>,
            EndQuery: EndQuery::<Impl, IMPL_OFFSET>,
            ResolveQueryData: ResolveQueryData::<Impl, IMPL_OFFSET>,
            SetPredication: SetPredication::<Impl, IMPL_OFFSET>,
            SetMarker: SetMarker::<Impl, IMPL_OFFSET>,
            BeginEvent: BeginEvent::<Impl, IMPL_OFFSET>,
            EndEvent: EndEvent::<Impl, IMPL_OFFSET>,
            ProcessFrames: ProcessFrames::<Impl, IMPL_OFFSET>,
            WriteBufferImmediate: WriteBufferImmediate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoProcessCommandList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D12VideoProcessCommandList1Impl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12CommandListImpl + ID3D12VideoProcessCommandListImpl {
    fn ProcessFrames1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D12VideoProcessCommandList1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoProcessCommandList1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoProcessCommandList1Vtbl {
        unsafe extern "system" fn ProcessFrames1<Impl: ID3D12VideoProcessCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, numinputstreams: u32, pinputarguments: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ID3D12VideoProcessCommandListVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ProcessFrames1: ProcessFrames1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoProcessCommandList1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D12VideoProcessCommandList2Impl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12CommandListImpl + ID3D12VideoProcessCommandListImpl + ID3D12VideoProcessCommandList1Impl {
    fn SetProtectedResourceSession();
    fn InitializeExtensionCommand();
    fn ExecuteExtensionCommand();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D12VideoProcessCommandList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoProcessCommandList2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoProcessCommandList2Vtbl {
        unsafe extern "system" fn SetProtectedResourceSession<Impl: ID3D12VideoProcessCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotectedresourcesession: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeExtensionCommand<Impl: ID3D12VideoProcessCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextensioncommand: ::windows::core::RawPtr, pinitializationparameters: *const ::core::ffi::c_void, initializationparameterssizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteExtensionCommand<Impl: ID3D12VideoProcessCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextensioncommand: ::windows::core::RawPtr, pexecutionparameters: *const ::core::ffi::c_void, executionparameterssizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12VideoProcessCommandList1Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProtectedResourceSession: SetProtectedResourceSession::<Impl, IMPL_OFFSET>,
            InitializeExtensionCommand: InitializeExtensionCommand::<Impl, IMPL_OFFSET>,
            ExecuteExtensionCommand: ExecuteExtensionCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoProcessCommandList2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoProcessorImpl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl {
    fn GetNodeMask();
    fn GetNumInputStreamDescs();
    fn GetInputStreamDescs();
    fn GetOutputStreamDesc();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoProcessorVtbl {
        unsafe extern "system" fn GetNodeMask<Impl: ID3D12VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumInputStreamDescs<Impl: ID3D12VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputStreamDescs<Impl: ID3D12VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numinputstreamdescs: u32, pinputstreamdescs: *mut D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputStreamDesc<Impl: ID3D12VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12PageableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetNodeMask: GetNodeMask::<Impl, IMPL_OFFSET>,
            GetNumInputStreamDescs: GetNumInputStreamDescs::<Impl, IMPL_OFFSET>,
            GetInputStreamDescs: GetInputStreamDescs::<Impl, IMPL_OFFSET>,
            GetOutputStreamDesc: GetOutputStreamDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoProcessor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12VideoProcessor1Impl: Sized + ID3D12ObjectImpl + ID3D12DeviceChildImpl + ID3D12PageableImpl + ID3D12VideoProcessorImpl {
    fn GetProtectedResourceSession();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12VideoProcessor1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VideoProcessor1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VideoProcessor1Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Impl: ID3D12VideoProcessor1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ID3D12VideoProcessorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetProtectedResourceSession: GetProtectedResourceSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VideoProcessor1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDXVAHD_DeviceImpl: Sized {
    fn CreateVideoSurface();
    fn GetVideoProcessorDeviceCaps();
    fn GetVideoProcessorOutputFormats();
    fn GetVideoProcessorInputFormats();
    fn GetVideoProcessorCaps();
    fn GetVideoProcessorCustomRates();
    fn GetVideoProcessorFilterRange();
    fn CreateVideoProcessor();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl IDXVAHD_DeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXVAHD_DeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXVAHD_DeviceVtbl {
        unsafe extern "system" fn CreateVideoSurface<Impl: IDXVAHD_DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: super::super::Graphics::Direct3D9::D3DFORMAT, pool: super::super::Graphics::Direct3D9::D3DPOOL, usage: u32, r#type: DXVAHD_SURFACE_TYPE, numsurfaces: u32, ppsurfaces: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorDeviceCaps<Impl: IDXVAHD_DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut DXVAHD_VPDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorOutputFormats<Impl: IDXVAHD_DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, pformats: *mut super::super::Graphics::Direct3D9::D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorInputFormats<Impl: IDXVAHD_DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, pformats: *mut super::super::Graphics::Direct3D9::D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Impl: IDXVAHD_DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, pcaps: *mut DXVAHD_VPCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorCustomRates<Impl: IDXVAHD_DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvpguid: *const ::windows::core::GUID, count: u32, prates: *mut DXVAHD_CUSTOM_RATE_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorFilterRange<Impl: IDXVAHD_DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: DXVAHD_FILTER, prange: *mut DXVAHD_FILTER_RANGE_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoProcessor<Impl: IDXVAHD_DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvpguid: *const ::windows::core::GUID, ppvideoprocessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateVideoSurface: CreateVideoSurface::<Impl, IMPL_OFFSET>,
            GetVideoProcessorDeviceCaps: GetVideoProcessorDeviceCaps::<Impl, IMPL_OFFSET>,
            GetVideoProcessorOutputFormats: GetVideoProcessorOutputFormats::<Impl, IMPL_OFFSET>,
            GetVideoProcessorInputFormats: GetVideoProcessorInputFormats::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCustomRates: GetVideoProcessorCustomRates::<Impl, IMPL_OFFSET>,
            GetVideoProcessorFilterRange: GetVideoProcessorFilterRange::<Impl, IMPL_OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXVAHD_Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDXVAHD_VideoProcessorImpl: Sized {
    fn SetVideoProcessBltState();
    fn GetVideoProcessBltState();
    fn SetVideoProcessStreamState();
    fn GetVideoProcessStreamState();
    fn VideoProcessBltHD();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl IDXVAHD_VideoProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXVAHD_VideoProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXVAHD_VideoProcessorVtbl {
        unsafe extern "system" fn SetVideoProcessBltState<Impl: IDXVAHD_VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: DXVAHD_BLT_STATE, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessBltState<Impl: IDXVAHD_VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: DXVAHD_BLT_STATE, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVideoProcessStreamState<Impl: IDXVAHD_VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, state: DXVAHD_STREAM_STATE, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessStreamState<Impl: IDXVAHD_VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, state: DXVAHD_STREAM_STATE, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessBltHD<Impl: IDXVAHD_VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputsurface: ::windows::core::RawPtr, outputframe: u32, streamcount: u32, pstreams: *const DXVAHD_STREAM_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetVideoProcessBltState: SetVideoProcessBltState::<Impl, IMPL_OFFSET>,
            GetVideoProcessBltState: GetVideoProcessBltState::<Impl, IMPL_OFFSET>,
            SetVideoProcessStreamState: SetVideoProcessStreamState::<Impl, IMPL_OFFSET>,
            GetVideoProcessStreamState: GetVideoProcessStreamState::<Impl, IMPL_OFFSET>,
            VideoProcessBltHD: VideoProcessBltHD::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXVAHD_VideoProcessor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub trait IDirect3D9ExOverlayExtensionImpl: Sized {
    fn CheckDeviceOverlayType();
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl IDirect3D9ExOverlayExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9ExOverlayExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D9ExOverlayExtensionVtbl {
        unsafe extern "system" fn CheckDeviceOverlayType<Impl: IDirect3D9ExOverlayExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devtype: super::super::Graphics::Direct3D9::D3DDEVTYPE, overlaywidth: u32, overlayheight: u32, overlayformat: super::super::Graphics::Direct3D9::D3DFORMAT, pdisplaymode: *mut super::super::Graphics::Direct3D9::D3DDISPLAYMODEEX, displayrotation: super::super::Graphics::Direct3D9::D3DDISPLAYROTATION, poverlaycaps: *mut D3DOVERLAYCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CheckDeviceOverlayType: CheckDeviceOverlayType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D9ExOverlayExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirect3DAuthenticatedChannel9Impl: Sized {
    fn GetCertificateSize();
    fn GetCertificate();
    fn NegotiateKeyExchange();
    fn Query();
    fn Configure();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl IDirect3DAuthenticatedChannel9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DAuthenticatedChannel9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DAuthenticatedChannel9Vtbl {
        unsafe extern "system" fn GetCertificateSize<Impl: IDirect3DAuthenticatedChannel9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificate<Impl: IDirect3DAuthenticatedChannel9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certifactesize: u32, ppcertificate: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NegotiateKeyExchange<Impl: IDirect3DAuthenticatedChannel9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Query<Impl: IDirect3DAuthenticatedChannel9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Configure<Impl: IDirect3DAuthenticatedChannel9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputsize: u32, pinput: *const ::core::ffi::c_void, poutput: *mut super::super::Graphics::Direct3D9::D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCertificateSize: GetCertificateSize::<Impl, IMPL_OFFSET>,
            GetCertificate: GetCertificate::<Impl, IMPL_OFFSET>,
            NegotiateKeyExchange: NegotiateKeyExchange::<Impl, IMPL_OFFSET>,
            Query: Query::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DAuthenticatedChannel9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub trait IDirect3DCryptoSession9Impl: Sized {
    fn GetCertificateSize();
    fn GetCertificate();
    fn NegotiateKeyExchange();
    fn EncryptionBlt();
    fn DecryptionBlt();
    fn GetSurfacePitch();
    fn StartSessionKeyRefresh();
    fn FinishSessionKeyRefresh();
    fn GetEncryptionBltKey();
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl IDirect3DCryptoSession9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DCryptoSession9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DCryptoSession9Vtbl {
        unsafe extern "system" fn GetCertificateSize<Impl: IDirect3DCryptoSession9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificate<Impl: IDirect3DCryptoSession9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certifactesize: u32, ppcertificate: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NegotiateKeyExchange<Impl: IDirect3DCryptoSession9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptionBlt<Impl: IDirect3DCryptoSession9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcsurface: ::windows::core::RawPtr, pdstsurface: ::windows::core::RawPtr, dstsurfacesize: u32, piv: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecryptionBlt<Impl: IDirect3DCryptoSession9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcsurface: ::windows::core::RawPtr, pdstsurface: ::windows::core::RawPtr, srcsurfacesize: u32, pencryptedblockinfo: *mut super::super::Graphics::Direct3D9::D3DENCRYPTED_BLOCK_INFO, pcontentkey: *mut ::core::ffi::c_void, piv: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSurfacePitch<Impl: IDirect3DCryptoSession9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcsurface: ::windows::core::RawPtr, psurfacepitch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartSessionKeyRefresh<Impl: IDirect3DCryptoSession9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomnumber: *mut ::core::ffi::c_void, randomnumbersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<Impl: IDirect3DCryptoSession9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEncryptionBltKey<Impl: IDirect3DCryptoSession9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadbackkey: *mut ::core::ffi::c_void, keysize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCertificateSize: GetCertificateSize::<Impl, IMPL_OFFSET>,
            GetCertificate: GetCertificate::<Impl, IMPL_OFFSET>,
            NegotiateKeyExchange: NegotiateKeyExchange::<Impl, IMPL_OFFSET>,
            EncryptionBlt: EncryptionBlt::<Impl, IMPL_OFFSET>,
            DecryptionBlt: DecryptionBlt::<Impl, IMPL_OFFSET>,
            GetSurfacePitch: GetSurfacePitch::<Impl, IMPL_OFFSET>,
            StartSessionKeyRefresh: StartSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            FinishSessionKeyRefresh: FinishSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            GetEncryptionBltKey: GetEncryptionBltKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DCryptoSession9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirect3DDevice9VideoImpl: Sized {
    fn GetContentProtectionCaps();
    fn CreateAuthenticatedChannel();
    fn CreateCryptoSession();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl IDirect3DDevice9VideoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9VideoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DDevice9VideoVtbl {
        unsafe extern "system" fn GetContentProtectionCaps<Impl: IDirect3DDevice9VideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecodeprofile: *const ::windows::core::GUID, pcaps: *mut D3DCONTENTPROTECTIONCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<Impl: IDirect3DDevice9VideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channeltype: super::super::Graphics::Direct3D9::D3DAUTHENTICATEDCHANNELTYPE, ppauthenticatedchannel: *mut ::windows::core::RawPtr, pchannelhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCryptoSession<Impl: IDirect3DDevice9VideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecodeprofile: *const ::windows::core::GUID, ppcryptosession: *mut ::windows::core::RawPtr, pcryptohandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetContentProtectionCaps: GetContentProtectionCaps::<Impl, IMPL_OFFSET>,
            CreateAuthenticatedChannel: CreateAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            CreateCryptoSession: CreateCryptoSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DDevice9Video as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirect3DDeviceManager9Impl: Sized {
    fn ResetDevice();
    fn OpenDeviceHandle();
    fn CloseDeviceHandle();
    fn TestDevice();
    fn LockDevice();
    fn UnlockDevice();
    fn GetVideoService();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl IDirect3DDeviceManager9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDeviceManager9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DDeviceManager9Vtbl {
        unsafe extern "system" fn ResetDevice<Impl: IDirect3DDeviceManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, resettoken: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenDeviceHandle<Impl: IDirect3DDeviceManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdevice: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseDeviceHandle<Impl: IDirect3DDeviceManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdevice: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TestDevice<Impl: IDirect3DDeviceManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdevice: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockDevice<Impl: IDirect3DDeviceManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdevice: super::super::Foundation::HANDLE, ppdevice: *mut ::windows::core::RawPtr, fblock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockDevice<Impl: IDirect3DDeviceManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdevice: super::super::Foundation::HANDLE, fsavestate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoService<Impl: IDirect3DDeviceManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdevice: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ResetDevice: ResetDevice::<Impl, IMPL_OFFSET>,
            OpenDeviceHandle: OpenDeviceHandle::<Impl, IMPL_OFFSET>,
            CloseDeviceHandle: CloseDeviceHandle::<Impl, IMPL_OFFSET>,
            TestDevice: TestDevice::<Impl, IMPL_OFFSET>,
            LockDevice: LockDevice::<Impl, IMPL_OFFSET>,
            UnlockDevice: UnlockDevice::<Impl, IMPL_OFFSET>,
            GetVideoService: GetVideoService::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DDeviceManager9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirectXVideoAccelerationServiceImpl: Sized {
    fn CreateSurface();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl IDirectXVideoAccelerationServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectXVideoAccelerationServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectXVideoAccelerationServiceVtbl {
        unsafe extern "system" fn CreateSurface<Impl: IDirectXVideoAccelerationServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, backbuffers: u32, format: super::super::Graphics::Direct3D9::D3DFORMAT, pool: super::super::Graphics::Direct3D9::D3DPOOL, usage: u32, dxvatype: DXVA2_VideoRenderTargetType, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateSurface: CreateSurface::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectXVideoAccelerationService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirectXVideoDecoderImpl: Sized {
    fn GetVideoDecoderService();
    fn GetCreationParameters();
    fn GetBuffer();
    fn ReleaseBuffer();
    fn BeginFrame();
    fn EndFrame();
    fn Execute();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl IDirectXVideoDecoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectXVideoDecoderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectXVideoDecoderVtbl {
        unsafe extern "system" fn GetVideoDecoderService<Impl: IDirectXVideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCreationParameters<Impl: IDirectXVideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceguid: *mut ::windows::core::GUID, pvideodesc: *mut DXVA2_VideoDesc, pconfig: *mut DXVA2_ConfigPictureDecode, pdecoderrendertargets: *mut *mut ::windows::core::RawPtr, pnumsurfaces: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBuffer<Impl: IDirectXVideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffertype: DXVA2_BufferfType, ppbuffer: *mut *mut ::core::ffi::c_void, pbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseBuffer<Impl: IDirectXVideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffertype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginFrame<Impl: IDirectXVideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, pvpvpdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndFrame<Impl: IDirectXVideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandlecomplete: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Execute<Impl: IDirectXVideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexecuteparams: *const DXVA2_DecodeExecuteParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetVideoDecoderService: GetVideoDecoderService::<Impl, IMPL_OFFSET>,
            GetCreationParameters: GetCreationParameters::<Impl, IMPL_OFFSET>,
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Impl, IMPL_OFFSET>,
            BeginFrame: BeginFrame::<Impl, IMPL_OFFSET>,
            EndFrame: EndFrame::<Impl, IMPL_OFFSET>,
            Execute: Execute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectXVideoDecoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirectXVideoDecoderServiceImpl: Sized + IDirectXVideoAccelerationServiceImpl {
    fn GetDecoderDeviceGuids();
    fn GetDecoderRenderTargets();
    fn GetDecoderConfigurations();
    fn CreateVideoDecoder();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl IDirectXVideoDecoderServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectXVideoDecoderServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectXVideoDecoderServiceVtbl {
        unsafe extern "system" fn GetDecoderDeviceGuids<Impl: IDirectXVideoDecoderServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32, pguids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDecoderRenderTargets<Impl: IDirectXVideoDecoderServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pcount: *mut u32, pformats: *mut *mut super::super::Graphics::Direct3D9::D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDecoderConfigurations<Impl: IDirectXVideoDecoderServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvideodesc: *const DXVA2_VideoDesc, preserved: *mut ::core::ffi::c_void, pcount: *mut u32, ppconfigs: *mut *mut DXVA2_ConfigPictureDecode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoDecoder<Impl: IDirectXVideoDecoderServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvideodesc: *const DXVA2_VideoDesc, pconfig: *const DXVA2_ConfigPictureDecode, ppdecoderrendertargets: *const ::windows::core::RawPtr, numrendertargets: u32, ppdecode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirectXVideoAccelerationServiceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDecoderDeviceGuids: GetDecoderDeviceGuids::<Impl, IMPL_OFFSET>,
            GetDecoderRenderTargets: GetDecoderRenderTargets::<Impl, IMPL_OFFSET>,
            GetDecoderConfigurations: GetDecoderConfigurations::<Impl, IMPL_OFFSET>,
            CreateVideoDecoder: CreateVideoDecoder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectXVideoDecoderService as ::windows::core::Interface>::IID
    }
}
pub trait IDirectXVideoMemoryConfigurationImpl: Sized {
    fn GetAvailableSurfaceTypeByIndex();
    fn SetSurfaceType();
}
impl IDirectXVideoMemoryConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectXVideoMemoryConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectXVideoMemoryConfigurationVtbl {
        unsafe extern "system" fn GetAvailableSurfaceTypeByIndex<Impl: IDirectXVideoMemoryConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtypeindex: u32, pdwtype: *mut DXVA2_SurfaceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSurfaceType<Impl: IDirectXVideoMemoryConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtype: DXVA2_SurfaceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAvailableSurfaceTypeByIndex: GetAvailableSurfaceTypeByIndex::<Impl, IMPL_OFFSET>,
            SetSurfaceType: SetSurfaceType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectXVideoMemoryConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirectXVideoProcessorImpl: Sized {
    fn GetVideoProcessorService();
    fn GetCreationParameters();
    fn GetVideoProcessorCaps();
    fn GetProcAmpRange();
    fn GetFilterPropertyRange();
    fn VideoProcessBlt();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl IDirectXVideoProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectXVideoProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectXVideoProcessorVtbl {
        unsafe extern "system" fn GetVideoProcessorService<Impl: IDirectXVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCreationParameters<Impl: IDirectXVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceguid: *mut ::windows::core::GUID, pvideodesc: *mut DXVA2_VideoDesc, prendertargetformat: *mut super::super::Graphics::Direct3D9::D3DFORMAT, pmaxnumsubstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Impl: IDirectXVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut DXVA2_VideoProcessorCaps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProcAmpRange<Impl: IDirectXVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, procampcap: u32, prange: *mut DXVA2_ValueRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilterPropertyRange<Impl: IDirectXVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filtersetting: u32, prange: *mut DXVA2_ValueRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessBlt<Impl: IDirectXVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, pbltparams: *const DXVA2_VideoProcessBltParams, psamples: *const DXVA2_VideoSample, numsamples: u32, phandlecomplete: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetVideoProcessorService: GetVideoProcessorService::<Impl, IMPL_OFFSET>,
            GetCreationParameters: GetCreationParameters::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Impl, IMPL_OFFSET>,
            GetProcAmpRange: GetProcAmpRange::<Impl, IMPL_OFFSET>,
            GetFilterPropertyRange: GetFilterPropertyRange::<Impl, IMPL_OFFSET>,
            VideoProcessBlt: VideoProcessBlt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectXVideoProcessor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirectXVideoProcessorServiceImpl: Sized + IDirectXVideoAccelerationServiceImpl {
    fn RegisterVideoProcessorSoftwareDevice();
    fn GetVideoProcessorDeviceGuids();
    fn GetVideoProcessorRenderTargets();
    fn GetVideoProcessorSubStreamFormats();
    fn GetVideoProcessorCaps();
    fn GetProcAmpRange();
    fn GetFilterPropertyRange();
    fn CreateVideoProcessor();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl IDirectXVideoProcessorServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectXVideoProcessorServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectXVideoProcessorServiceVtbl {
        unsafe extern "system" fn RegisterVideoProcessorSoftwareDevice<Impl: IDirectXVideoProcessorServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbacks: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorDeviceGuids<Impl: IDirectXVideoProcessorServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodesc: *const DXVA2_VideoDesc, pcount: *mut u32, pguids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorRenderTargets<Impl: IDirectXVideoProcessorServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videoprocdeviceguid: *const ::windows::core::GUID, pvideodesc: *const DXVA2_VideoDesc, pcount: *mut u32, pformats: *mut *mut super::super::Graphics::Direct3D9::D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorSubStreamFormats<Impl: IDirectXVideoProcessorServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videoprocdeviceguid: *const ::windows::core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, pcount: *mut u32, pformats: *mut *mut super::super::Graphics::Direct3D9::D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Impl: IDirectXVideoProcessorServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videoprocdeviceguid: *const ::windows::core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, pcaps: *mut DXVA2_VideoProcessorCaps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProcAmpRange<Impl: IDirectXVideoProcessorServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videoprocdeviceguid: *const ::windows::core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, procampcap: u32, prange: *mut DXVA2_ValueRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilterPropertyRange<Impl: IDirectXVideoProcessorServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videoprocdeviceguid: *const ::windows::core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, filtersetting: u32, prange: *mut DXVA2_ValueRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoProcessor<Impl: IDirectXVideoProcessorServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videoprocdeviceguid: *const ::windows::core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, maxnumsubstreams: u32, ppvidprocess: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirectXVideoAccelerationServiceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegisterVideoProcessorSoftwareDevice: RegisterVideoProcessorSoftwareDevice::<Impl, IMPL_OFFSET>,
            GetVideoProcessorDeviceGuids: GetVideoProcessorDeviceGuids::<Impl, IMPL_OFFSET>,
            GetVideoProcessorRenderTargets: GetVideoProcessorRenderTargets::<Impl, IMPL_OFFSET>,
            GetVideoProcessorSubStreamFormats: GetVideoProcessorSubStreamFormats::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Impl, IMPL_OFFSET>,
            GetProcAmpRange: GetProcAmpRange::<Impl, IMPL_OFFSET>,
            GetFilterPropertyRange: GetFilterPropertyRange::<Impl, IMPL_OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectXVideoProcessorService as ::windows::core::Interface>::IID
    }
}
pub trait IEVRFilterConfigImpl: Sized {
    fn SetNumberOfStreams();
    fn GetNumberOfStreams();
}
impl IEVRFilterConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEVRFilterConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEVRFilterConfigVtbl {
        unsafe extern "system" fn SetNumberOfStreams<Impl: IEVRFilterConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxstreams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumberOfStreams<Impl: IEVRFilterConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetNumberOfStreams: SetNumberOfStreams::<Impl, IMPL_OFFSET>,
            GetNumberOfStreams: GetNumberOfStreams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEVRFilterConfig as ::windows::core::Interface>::IID
    }
}
pub trait IEVRFilterConfigExImpl: Sized + IEVRFilterConfigImpl {
    fn SetConfigPrefs();
    fn GetConfigPrefs();
}
impl IEVRFilterConfigExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEVRFilterConfigExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEVRFilterConfigExVtbl {
        unsafe extern "system" fn SetConfigPrefs<Impl: IEVRFilterConfigExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconfigflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConfigPrefs<Impl: IEVRFilterConfigExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconfigflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IEVRFilterConfigVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetConfigPrefs: SetConfigPrefs::<Impl, IMPL_OFFSET>,
            GetConfigPrefs: GetConfigPrefs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEVRFilterConfigEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEVRTrustedVideoPluginImpl: Sized {
    fn IsInTrustedVideoMode();
    fn CanConstrict();
    fn SetConstriction();
    fn DisableImageExport();
}
#[cfg(feature = "Win32_Foundation")]
impl IEVRTrustedVideoPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEVRTrustedVideoPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEVRTrustedVideoPluginVtbl {
        unsafe extern "system" fn IsInTrustedVideoMode<Impl: IEVRTrustedVideoPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pyes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanConstrict<Impl: IEVRTrustedVideoPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pyes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConstriction<Impl: IEVRTrustedVideoPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwkpix: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableImageExport<Impl: IEVRTrustedVideoPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsInTrustedVideoMode: IsInTrustedVideoMode::<Impl, IMPL_OFFSET>,
            CanConstrict: CanConstrict::<Impl, IMPL_OFFSET>,
            SetConstriction: SetConstriction::<Impl, IMPL_OFFSET>,
            DisableImageExport: DisableImageExport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEVRTrustedVideoPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEVRVideoStreamControlImpl: Sized {
    fn SetStreamActiveState();
    fn GetStreamActiveState();
}
#[cfg(feature = "Win32_Foundation")]
impl IEVRVideoStreamControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEVRVideoStreamControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEVRVideoStreamControlVtbl {
        unsafe extern "system" fn SetStreamActiveState<Impl: IEVRVideoStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamActiveState<Impl: IEVRVideoStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpfactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetStreamActiveState: SetStreamActiveState::<Impl, IMPL_OFFSET>,
            GetStreamActiveState: GetStreamActiveState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEVRVideoStreamControl as ::windows::core::Interface>::IID
    }
}
pub trait IFileClientImpl: Sized {
    fn GetObjectDiskSize();
    fn Write();
    fn Read();
}
impl IFileClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileClientVtbl {
        unsafe extern "system" fn GetObjectDiskSize<Impl: IFileClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqwsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write<Impl: IFileClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfio: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Read<Impl: IFileClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfio: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetObjectDiskSize: GetObjectDiskSize::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFileIoImpl: Sized {
    fn Initialize();
    fn GetLength();
    fn SetLength();
    fn GetCurrentPosition();
    fn SetCurrentPosition();
    fn IsEndOfStream();
    fn Read();
    fn Write();
    fn Seek();
    fn Close();
}
#[cfg(feature = "Win32_Foundation")]
impl IFileIoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileIoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileIoVtbl {
        unsafe extern "system" fn Initialize<Impl: IFileIoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eaccessmode: FILE_ACCESSMODE, eopenmode: FILE_OPENMODE, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLength<Impl: IFileIoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqwlength: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLength<Impl: IFileIoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwlength: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPosition<Impl: IFileIoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqwposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentPosition<Impl: IFileIoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwposition: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEndOfStream<Impl: IFileIoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbendofstream: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Read<Impl: IFileIoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbt: *mut u8, ul: u32, pulread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write<Impl: IFileIoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbt: *mut u8, ul: u32, pulwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Seek<Impl: IFileIoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eseekorigin: SEEK_ORIGIN, qwseekoffset: u64, dwseekflags: u32, pqwcurrentposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IFileIoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetLength: GetLength::<Impl, IMPL_OFFSET>,
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Impl, IMPL_OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Impl, IMPL_OFFSET>,
            IsEndOfStream: IsEndOfStream::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileIo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMF2DBufferImpl: Sized {
    fn Lock2D();
    fn Unlock2D();
    fn GetScanline0AndPitch();
    fn IsContiguousFormat();
    fn GetContiguousLength();
    fn ContiguousCopyTo();
    fn ContiguousCopyFrom();
}
#[cfg(feature = "Win32_Foundation")]
impl IMF2DBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMF2DBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMF2DBufferVtbl {
        unsafe extern "system" fn Lock2D<Impl: IMF2DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbscanline0: *mut *mut u8, plpitch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock2D<Impl: IMF2DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScanline0AndPitch<Impl: IMF2DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbscanline0: *mut *mut u8, plpitch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsContiguousFormat<Impl: IMF2DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiscontiguous: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContiguousLength<Impl: IMF2DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContiguousCopyTo<Impl: IMF2DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdestbuffer: *mut u8, cbdestbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContiguousCopyFrom<Impl: IMF2DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsrcbuffer: *const u8, cbsrcbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Lock2D: Lock2D::<Impl, IMPL_OFFSET>,
            Unlock2D: Unlock2D::<Impl, IMPL_OFFSET>,
            GetScanline0AndPitch: GetScanline0AndPitch::<Impl, IMPL_OFFSET>,
            IsContiguousFormat: IsContiguousFormat::<Impl, IMPL_OFFSET>,
            GetContiguousLength: GetContiguousLength::<Impl, IMPL_OFFSET>,
            ContiguousCopyTo: ContiguousCopyTo::<Impl, IMPL_OFFSET>,
            ContiguousCopyFrom: ContiguousCopyFrom::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMF2DBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMF2DBuffer2Impl: Sized + IMF2DBufferImpl {
    fn Lock2DSize();
    fn Copy2DTo();
}
#[cfg(feature = "Win32_Foundation")]
impl IMF2DBuffer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMF2DBuffer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMF2DBuffer2Vtbl {
        unsafe extern "system" fn Lock2DSize<Impl: IMF2DBuffer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lockflags: MF2DBuffer_LockFlags, ppbscanline0: *mut *mut u8, plpitch: *mut i32, ppbbufferstart: *mut *mut u8, pcbbufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Copy2DTo<Impl: IMF2DBuffer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMF2DBufferVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Lock2DSize: Lock2DSize::<Impl, IMPL_OFFSET>,
            Copy2DTo: Copy2DTo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMF2DBuffer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IMFASFContentInfoImpl: Sized {
    fn GetHeaderSize();
    fn ParseHeader();
    fn GenerateHeader();
    fn GetProfile();
    fn SetProfile();
    fn GeneratePresentationDescriptor();
    fn GetEncodingConfigurationPropertyStore();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IMFASFContentInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFASFContentInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFASFContentInfoVtbl {
        unsafe extern "system" fn GetHeaderSize<Impl: IMFASFContentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistartofcontent: ::windows::core::RawPtr, cbheadersize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParseHeader<Impl: IMFASFContentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piheaderbuffer: ::windows::core::RawPtr, cboffsetwithinheader: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateHeader<Impl: IMFASFContentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piheader: ::windows::core::RawPtr, pcbheader: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProfile<Impl: IMFASFContentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProfile<Impl: IMFASFContentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GeneratePresentationDescriptor<Impl: IMFASFContentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipresentationdescriptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEncodingConfigurationPropertyStore<Impl: IMFASFContentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppistore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetHeaderSize: GetHeaderSize::<Impl, IMPL_OFFSET>,
            ParseHeader: ParseHeader::<Impl, IMPL_OFFSET>,
            GenerateHeader: GenerateHeader::<Impl, IMPL_OFFSET>,
            GetProfile: GetProfile::<Impl, IMPL_OFFSET>,
            SetProfile: SetProfile::<Impl, IMPL_OFFSET>,
            GeneratePresentationDescriptor: GeneratePresentationDescriptor::<Impl, IMPL_OFFSET>,
            GetEncodingConfigurationPropertyStore: GetEncodingConfigurationPropertyStore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFASFContentInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFASFIndexerImpl: Sized {
    fn SetFlags();
    fn GetFlags();
    fn Initialize();
    fn GetIndexPosition();
    fn SetIndexByteStreams();
    fn GetIndexByteStreamCount();
    fn GetIndexStatus();
    fn SetIndexStatus();
    fn GetSeekPositionForValue();
    fn GenerateIndexEntries();
    fn CommitIndex();
    fn GetIndexWriteSpace();
    fn GetCompletedIndex();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFASFIndexerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFASFIndexerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFASFIndexerVtbl {
        unsafe extern "system" fn SetFlags<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picontentinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIndexPosition<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picontentinfo: ::windows::core::RawPtr, pcbindexoffset: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIndexByteStreams<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibytestreams: *const ::windows::core::RawPtr, cbytestreams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIndexByteStreamCount<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbytestreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIndexStatus<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexidentifier: *const ASF_INDEX_IDENTIFIER, pfisindexed: *mut super::super::Foundation::BOOL, pbindexdescriptor: *mut u8, pcbindexdescriptor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIndexStatus<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindexdescriptor: *const u8, cbindexdescriptor: u32, fgenerateindex: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSeekPositionForValue<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pindexidentifier: *const ASF_INDEX_IDENTIFIER, pcboffsetwithindata: *mut u64, phnsapproxtime: *mut i64, pdwpayloadnumberofstreamwithinpacket: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateIndexEntries<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piasfpacketsample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitIndex<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picontentinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIndexWriteSpace<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbindexwritespace: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompletedIndex<Impl: IMFASFIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piindexbuffer: ::windows::core::RawPtr, cboffsetwithinindex: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetIndexPosition: GetIndexPosition::<Impl, IMPL_OFFSET>,
            SetIndexByteStreams: SetIndexByteStreams::<Impl, IMPL_OFFSET>,
            GetIndexByteStreamCount: GetIndexByteStreamCount::<Impl, IMPL_OFFSET>,
            GetIndexStatus: GetIndexStatus::<Impl, IMPL_OFFSET>,
            SetIndexStatus: SetIndexStatus::<Impl, IMPL_OFFSET>,
            GetSeekPositionForValue: GetSeekPositionForValue::<Impl, IMPL_OFFSET>,
            GenerateIndexEntries: GenerateIndexEntries::<Impl, IMPL_OFFSET>,
            CommitIndex: CommitIndex::<Impl, IMPL_OFFSET>,
            GetIndexWriteSpace: GetIndexWriteSpace::<Impl, IMPL_OFFSET>,
            GetCompletedIndex: GetCompletedIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFASFIndexer as ::windows::core::Interface>::IID
    }
}
pub trait IMFASFMultiplexerImpl: Sized {
    fn Initialize();
    fn SetFlags();
    fn GetFlags();
    fn ProcessSample();
    fn GetNextPacket();
    fn Flush();
    fn End();
    fn GetStatistics();
    fn SetSyncTolerance();
}
impl IMFASFMultiplexerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFASFMultiplexerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFASFMultiplexerVtbl {
        unsafe extern "system" fn Initialize<Impl: IMFASFMultiplexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picontentinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: IMFASFMultiplexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IMFASFMultiplexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessSample<Impl: IMFASFMultiplexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pisample: ::windows::core::RawPtr, hnstimestampadjust: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextPacket<Impl: IMFASFMultiplexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatusflags: *mut u32, ppipacket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IMFASFMultiplexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn End<Impl: IMFASFMultiplexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picontentinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatistics<Impl: IMFASFMultiplexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pmuxstats: *mut ASF_MUX_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSyncTolerance<Impl: IMFASFMultiplexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mssynctolerance: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            ProcessSample: ProcessSample::<Impl, IMPL_OFFSET>,
            GetNextPacket: GetNextPacket::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
            GetStatistics: GetStatistics::<Impl, IMPL_OFFSET>,
            SetSyncTolerance: SetSyncTolerance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFASFMultiplexer as ::windows::core::Interface>::IID
    }
}
pub trait IMFASFMutualExclusionImpl: Sized {
    fn GetType();
    fn SetType();
    fn GetRecordCount();
    fn GetStreamsForRecord();
    fn AddStreamForRecord();
    fn RemoveStreamFromRecord();
    fn RemoveRecord();
    fn AddRecord();
    fn Clone();
}
impl IMFASFMutualExclusionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFASFMutualExclusionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFASFMutualExclusionVtbl {
        unsafe extern "system" fn GetType<Impl: IMFASFMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetType<Impl: IMFASFMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecordCount<Impl: IMFASFMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrecordcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamsForRecord<Impl: IMFASFMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrecordnumber: u32, pwstreamnumarray: *mut u16, pcstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStreamForRecord<Impl: IMFASFMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrecordnumber: u32, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStreamFromRecord<Impl: IMFASFMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrecordnumber: u32, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveRecord<Impl: IMFASFMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrecordnumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRecord<Impl: IMFASFMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrecordnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IMFASFMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimutex: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            GetRecordCount: GetRecordCount::<Impl, IMPL_OFFSET>,
            GetStreamsForRecord: GetStreamsForRecord::<Impl, IMPL_OFFSET>,
            AddStreamForRecord: AddStreamForRecord::<Impl, IMPL_OFFSET>,
            RemoveStreamFromRecord: RemoveStreamFromRecord::<Impl, IMPL_OFFSET>,
            RemoveRecord: RemoveRecord::<Impl, IMPL_OFFSET>,
            AddRecord: AddRecord::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFASFMutualExclusion as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFASFProfileImpl: Sized + IMFAttributesImpl {
    fn GetStreamCount();
    fn GetStream();
    fn GetStreamByNumber();
    fn SetStream();
    fn RemoveStream();
    fn CreateStream();
    fn GetMutualExclusionCount();
    fn GetMutualExclusion();
    fn AddMutualExclusion();
    fn RemoveMutualExclusion();
    fn CreateMutualExclusion();
    fn GetStreamPrioritization();
    fn AddStreamPrioritization();
    fn RemoveStreamPrioritization();
    fn CreateStreamPrioritization();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFASFProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFASFProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFASFProfileVtbl {
        unsafe extern "system" fn GetStreamCount<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pwstreamnumber: *mut u16, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamByNumber<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStream<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStream<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStream<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimediatype: ::windows::core::RawPtr, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMutualExclusionCount<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmutexs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMutualExclusion<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmutexindex: u32, ppimutex: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddMutualExclusion<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimutex: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveMutualExclusion<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmutexindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMutualExclusion<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimutex: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamPrioritization<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppistreamprioritization: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStreamPrioritization<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistreamprioritization: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStreamPrioritization<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStreamPrioritization<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppistreamprioritization: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IMFASFProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStreamCount: GetStreamCount::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            GetStreamByNumber: GetStreamByNumber::<Impl, IMPL_OFFSET>,
            SetStream: SetStream::<Impl, IMPL_OFFSET>,
            RemoveStream: RemoveStream::<Impl, IMPL_OFFSET>,
            CreateStream: CreateStream::<Impl, IMPL_OFFSET>,
            GetMutualExclusionCount: GetMutualExclusionCount::<Impl, IMPL_OFFSET>,
            GetMutualExclusion: GetMutualExclusion::<Impl, IMPL_OFFSET>,
            AddMutualExclusion: AddMutualExclusion::<Impl, IMPL_OFFSET>,
            RemoveMutualExclusion: RemoveMutualExclusion::<Impl, IMPL_OFFSET>,
            CreateMutualExclusion: CreateMutualExclusion::<Impl, IMPL_OFFSET>,
            GetStreamPrioritization: GetStreamPrioritization::<Impl, IMPL_OFFSET>,
            AddStreamPrioritization: AddStreamPrioritization::<Impl, IMPL_OFFSET>,
            RemoveStreamPrioritization: RemoveStreamPrioritization::<Impl, IMPL_OFFSET>,
            CreateStreamPrioritization: CreateStreamPrioritization::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFASFProfile as ::windows::core::Interface>::IID
    }
}
pub trait IMFASFSplitterImpl: Sized {
    fn Initialize();
    fn SetFlags();
    fn GetFlags();
    fn SelectStreams();
    fn GetSelectedStreams();
    fn ParseData();
    fn GetNextSample();
    fn Flush();
    fn GetLastSendTime();
}
impl IMFASFSplitterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFASFSplitterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFASFSplitterVtbl {
        unsafe extern "system" fn Initialize<Impl: IMFASFSplitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picontentinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: IMFASFSplitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IMFASFSplitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectStreams<Impl: IMFASFSplitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnumbers: *const u16, wnumstreams: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSelectedStreams<Impl: IMFASFSplitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnumbers: *mut u16, pwnumstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParseData<Impl: IMFASFSplitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibuffer: ::windows::core::RawPtr, cbbufferoffset: u32, cblength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextSample<Impl: IMFASFSplitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatusflags: *mut ASF_STATUSFLAGS, pwstreamnumber: *mut u16, ppisample: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IMFASFSplitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastSendTime<Impl: IMFASFSplitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastsendtime: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            SelectStreams: SelectStreams::<Impl, IMPL_OFFSET>,
            GetSelectedStreams: GetSelectedStreams::<Impl, IMPL_OFFSET>,
            ParseData: ParseData::<Impl, IMPL_OFFSET>,
            GetNextSample: GetNextSample::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            GetLastSendTime: GetLastSendTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFASFSplitter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFASFStreamConfigImpl: Sized + IMFAttributesImpl {
    fn GetStreamType();
    fn GetStreamNumber();
    fn SetStreamNumber();
    fn GetMediaType();
    fn SetMediaType();
    fn GetPayloadExtensionCount();
    fn GetPayloadExtension();
    fn AddPayloadExtension();
    fn RemoveAllPayloadExtensions();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFASFStreamConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFASFStreamConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFASFStreamConfigVtbl {
        unsafe extern "system" fn GetStreamType<Impl: IMFASFStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamNumber<Impl: IMFASFStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u16 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamNumber<Impl: IMFASFStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMediaType<Impl: IMFASFStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMediaType<Impl: IMFASFStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimediatype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPayloadExtensionCount<Impl: IMFASFStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcpayloadextensions: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPayloadExtension<Impl: IMFASFStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wpayloadextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPayloadExtension<Impl: IMFASFStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllPayloadExtensions<Impl: IMFASFStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IMFASFStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppistreamconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStreamType: GetStreamType::<Impl, IMPL_OFFSET>,
            GetStreamNumber: GetStreamNumber::<Impl, IMPL_OFFSET>,
            SetStreamNumber: SetStreamNumber::<Impl, IMPL_OFFSET>,
            GetMediaType: GetMediaType::<Impl, IMPL_OFFSET>,
            SetMediaType: SetMediaType::<Impl, IMPL_OFFSET>,
            GetPayloadExtensionCount: GetPayloadExtensionCount::<Impl, IMPL_OFFSET>,
            GetPayloadExtension: GetPayloadExtension::<Impl, IMPL_OFFSET>,
            AddPayloadExtension: AddPayloadExtension::<Impl, IMPL_OFFSET>,
            RemoveAllPayloadExtensions: RemoveAllPayloadExtensions::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFASFStreamConfig as ::windows::core::Interface>::IID
    }
}
pub trait IMFASFStreamPrioritizationImpl: Sized {
    fn GetStreamCount();
    fn GetStream();
    fn AddStream();
    fn RemoveStream();
    fn Clone();
}
impl IMFASFStreamPrioritizationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFASFStreamPrioritizationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFASFStreamPrioritizationVtbl {
        unsafe extern "system" fn GetStreamCount<Impl: IMFASFStreamPrioritizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IMFASFStreamPrioritizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pwstreamnumber: *mut u16, pwstreamflags: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStream<Impl: IMFASFStreamPrioritizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, wstreamflags: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStream<Impl: IMFASFStreamPrioritizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IMFASFStreamPrioritizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppistreamprioritization: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreamCount: GetStreamCount::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            AddStream: AddStream::<Impl, IMPL_OFFSET>,
            RemoveStream: RemoveStream::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFASFStreamPrioritization as ::windows::core::Interface>::IID
    }
}
pub trait IMFASFStreamSelectorImpl: Sized {
    fn GetStreamCount();
    fn GetOutputCount();
    fn GetOutputStreamCount();
    fn GetOutputStreamNumbers();
    fn GetOutputFromStream();
    fn GetOutputOverride();
    fn SetOutputOverride();
    fn GetOutputMutexCount();
    fn GetOutputMutex();
    fn SetOutputMutexSelection();
    fn GetBandwidthStepCount();
    fn GetBandwidthStep();
    fn BitrateToStepNumber();
    fn SetStreamSelectorFlags();
}
impl IMFASFStreamSelectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFASFStreamSelectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFASFStreamSelectorVtbl {
        unsafe extern "system" fn GetStreamCount<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputCount<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputStreamCount<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputStreamNumbers<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, rgwstreamnumbers: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputFromStream<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutput: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputOverride<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pselection: *mut ASF_SELECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputOverride<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, selection: ASF_SELECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputMutexCount<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcmutexes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputMutex<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwmutexnum: u32, ppmutex: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputMutexSelection<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwmutexnum: u32, wselectedrecord: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBandwidthStepCount<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstepcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBandwidthStep<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstepnum: u32, pdwbitrate: *mut u32, rgwstreamnumbers: *mut u16, rgselections: *mut ASF_SELECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BitrateToStepNumber<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbitrate: u32, pdwstepnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamSelectorFlags<Impl: IMFASFStreamSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamselectorflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreamCount: GetStreamCount::<Impl, IMPL_OFFSET>,
            GetOutputCount: GetOutputCount::<Impl, IMPL_OFFSET>,
            GetOutputStreamCount: GetOutputStreamCount::<Impl, IMPL_OFFSET>,
            GetOutputStreamNumbers: GetOutputStreamNumbers::<Impl, IMPL_OFFSET>,
            GetOutputFromStream: GetOutputFromStream::<Impl, IMPL_OFFSET>,
            GetOutputOverride: GetOutputOverride::<Impl, IMPL_OFFSET>,
            SetOutputOverride: SetOutputOverride::<Impl, IMPL_OFFSET>,
            GetOutputMutexCount: GetOutputMutexCount::<Impl, IMPL_OFFSET>,
            GetOutputMutex: GetOutputMutex::<Impl, IMPL_OFFSET>,
            SetOutputMutexSelection: SetOutputMutexSelection::<Impl, IMPL_OFFSET>,
            GetBandwidthStepCount: GetBandwidthStepCount::<Impl, IMPL_OFFSET>,
            GetBandwidthStep: GetBandwidthStep::<Impl, IMPL_OFFSET>,
            BitrateToStepNumber: BitrateToStepNumber::<Impl, IMPL_OFFSET>,
            SetStreamSelectorFlags: SetStreamSelectorFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFASFStreamSelector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFActivateImpl: Sized + IMFAttributesImpl {
    fn ActivateObject();
    fn ShutdownObject();
    fn DetachObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFActivateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFActivateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFActivateVtbl {
        unsafe extern "system" fn ActivateObject<Impl: IMFActivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShutdownObject<Impl: IMFActivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DetachObject<Impl: IMFActivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActivateObject: ActivateObject::<Impl, IMPL_OFFSET>,
            ShutdownObject: ShutdownObject::<Impl, IMPL_OFFSET>,
            DetachObject: DetachObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFActivate as ::windows::core::Interface>::IID
    }
}
pub trait IMFAsyncCallbackImpl: Sized {
    fn GetParameters();
    fn Invoke();
}
impl IMFAsyncCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFAsyncCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFAsyncCallbackVtbl {
        unsafe extern "system" fn GetParameters<Impl: IMFAsyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, pdwqueue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Invoke<Impl: IMFAsyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetParameters: GetParameters::<Impl, IMPL_OFFSET>,
            Invoke: Invoke::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFAsyncCallback as ::windows::core::Interface>::IID
    }
}
pub trait IMFAsyncCallbackLoggingImpl: Sized + IMFAsyncCallbackImpl {
    fn GetObjectPointer();
    fn GetObjectTag();
}
impl IMFAsyncCallbackLoggingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFAsyncCallbackLoggingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFAsyncCallbackLoggingVtbl {
        unsafe extern "system" fn GetObjectPointer<Impl: IMFAsyncCallbackLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectTag<Impl: IMFAsyncCallbackLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAsyncCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetObjectPointer: GetObjectPointer::<Impl, IMPL_OFFSET>,
            GetObjectTag: GetObjectTag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFAsyncCallbackLogging as ::windows::core::Interface>::IID
    }
}
pub trait IMFAsyncResultImpl: Sized {
    fn GetState();
    fn GetStatus();
    fn SetStatus();
    fn GetObject();
    fn GetStateNoAddRef();
}
impl IMFAsyncResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFAsyncResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFAsyncResultVtbl {
        unsafe extern "system" fn GetState<Impl: IMFAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IMFAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatus<Impl: IMFAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IMFAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStateNoAddRef<Impl: IMFAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<::windows::core::IUnknown> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetState: GetState::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
            GetStateNoAddRef: GetStateNoAddRef::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFAsyncResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFAttributesImpl: Sized {
    fn GetItem();
    fn GetItemType();
    fn CompareItem();
    fn Compare();
    fn GetUINT32();
    fn GetUINT64();
    fn GetDouble();
    fn GetGUID();
    fn GetStringLength();
    fn GetString();
    fn GetAllocatedString();
    fn GetBlobSize();
    fn GetBlob();
    fn GetAllocatedBlob();
    fn GetUnknown();
    fn SetItem();
    fn DeleteItem();
    fn DeleteAllItems();
    fn SetUINT32();
    fn SetUINT64();
    fn SetDouble();
    fn SetGUID();
    fn SetString();
    fn SetBlob();
    fn SetUnknown();
    fn LockStore();
    fn UnlockStore();
    fn GetCount();
    fn GetItemByIndex();
    fn CopyAllItems();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFAttributesVtbl {
        unsafe extern "system" fn GetItem<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItemType<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, ptype: *mut MF_ATTRIBUTE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareItem<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pbresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Compare<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptheirs: ::windows::core::RawPtr, matchtype: MF_ATTRIBUTES_MATCH_TYPE, pbresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUINT32<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, punvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUINT64<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, punvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDouble<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pfvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGUID<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pguidvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringLength<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pcchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetString<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pwszvalue: super::super::Foundation::PWSTR, cchbufsize: u32, pcchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllocatedString<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut super::super::Foundation::PWSTR, pcchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBlobSize<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pcbblobsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBlob<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pbuf: *mut u8, cbbufsize: u32, pcbblobsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllocatedBlob<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnknown<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetItem<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteItem<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAllItems<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUINT32<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUINT64<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDouble<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGUID<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetString<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, wszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlob<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pbuf: *const u8, cbbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnknown<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockStore<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockStore<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcitems: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItemByIndex<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyAllItems<Impl: IMFAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
            GetItemType: GetItemType::<Impl, IMPL_OFFSET>,
            CompareItem: CompareItem::<Impl, IMPL_OFFSET>,
            Compare: Compare::<Impl, IMPL_OFFSET>,
            GetUINT32: GetUINT32::<Impl, IMPL_OFFSET>,
            GetUINT64: GetUINT64::<Impl, IMPL_OFFSET>,
            GetDouble: GetDouble::<Impl, IMPL_OFFSET>,
            GetGUID: GetGUID::<Impl, IMPL_OFFSET>,
            GetStringLength: GetStringLength::<Impl, IMPL_OFFSET>,
            GetString: GetString::<Impl, IMPL_OFFSET>,
            GetAllocatedString: GetAllocatedString::<Impl, IMPL_OFFSET>,
            GetBlobSize: GetBlobSize::<Impl, IMPL_OFFSET>,
            GetBlob: GetBlob::<Impl, IMPL_OFFSET>,
            GetAllocatedBlob: GetAllocatedBlob::<Impl, IMPL_OFFSET>,
            GetUnknown: GetUnknown::<Impl, IMPL_OFFSET>,
            SetItem: SetItem::<Impl, IMPL_OFFSET>,
            DeleteItem: DeleteItem::<Impl, IMPL_OFFSET>,
            DeleteAllItems: DeleteAllItems::<Impl, IMPL_OFFSET>,
            SetUINT32: SetUINT32::<Impl, IMPL_OFFSET>,
            SetUINT64: SetUINT64::<Impl, IMPL_OFFSET>,
            SetDouble: SetDouble::<Impl, IMPL_OFFSET>,
            SetGUID: SetGUID::<Impl, IMPL_OFFSET>,
            SetString: SetString::<Impl, IMPL_OFFSET>,
            SetBlob: SetBlob::<Impl, IMPL_OFFSET>,
            SetUnknown: SetUnknown::<Impl, IMPL_OFFSET>,
            LockStore: LockStore::<Impl, IMPL_OFFSET>,
            UnlockStore: UnlockStore::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetItemByIndex: GetItemByIndex::<Impl, IMPL_OFFSET>,
            CopyAllItems: CopyAllItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFAudioMediaTypeImpl: Sized + IMFAttributesImpl + IMFMediaTypeImpl {
    fn GetAudioFormat();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFAudioMediaTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFAudioMediaTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFAudioMediaTypeVtbl {
        unsafe extern "system" fn GetAudioFormat<Impl: IMFAudioMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::Audio::WAVEFORMATEX {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IMFMediaTypeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetAudioFormat: GetAudioFormat::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFAudioMediaType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFAudioPolicyImpl: Sized {
    fn SetGroupingParam();
    fn GetGroupingParam();
    fn SetDisplayName();
    fn GetDisplayName();
    fn SetIconPath();
    fn GetIconPath();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFAudioPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFAudioPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFAudioPolicyVtbl {
        unsafe extern "system" fn SetGroupingParam<Impl: IMFAudioPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidclass: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGroupingParam<Impl: IMFAudioPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: IMFAudioPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IMFAudioPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIconPath<Impl: IMFAudioPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIconPath<Impl: IMFAudioPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetGroupingParam: SetGroupingParam::<Impl, IMPL_OFFSET>,
            GetGroupingParam: GetGroupingParam::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET>,
            SetIconPath: SetIconPath::<Impl, IMPL_OFFSET>,
            GetIconPath: GetIconPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFAudioPolicy as ::windows::core::Interface>::IID
    }
}
pub trait IMFAudioStreamVolumeImpl: Sized {
    fn GetChannelCount();
    fn SetChannelVolume();
    fn GetChannelVolume();
    fn SetAllVolumes();
    fn GetAllVolumes();
}
impl IMFAudioStreamVolumeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFAudioStreamVolumeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFAudioStreamVolumeVtbl {
        unsafe extern "system" fn GetChannelCount<Impl: IMFAudioStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetChannelVolume<Impl: IMFAudioStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelVolume<Impl: IMFAudioStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllVolumes<Impl: IMFAudioStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllVolumes<Impl: IMFAudioStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetChannelCount: GetChannelCount::<Impl, IMPL_OFFSET>,
            SetChannelVolume: SetChannelVolume::<Impl, IMPL_OFFSET>,
            GetChannelVolume: GetChannelVolume::<Impl, IMPL_OFFSET>,
            SetAllVolumes: SetAllVolumes::<Impl, IMPL_OFFSET>,
            GetAllVolumes: GetAllVolumes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFAudioStreamVolume as ::windows::core::Interface>::IID
    }
}
pub trait IMFBufferListNotifyImpl: Sized {
    fn OnAddSourceBuffer();
    fn OnRemoveSourceBuffer();
}
impl IMFBufferListNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFBufferListNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFBufferListNotifyVtbl {
        unsafe extern "system" fn OnAddSourceBuffer<Impl: IMFBufferListNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnRemoveSourceBuffer<Impl: IMFBufferListNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnAddSourceBuffer: OnAddSourceBuffer::<Impl, IMPL_OFFSET>,
            OnRemoveSourceBuffer: OnRemoveSourceBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFBufferListNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFByteStreamImpl: Sized {
    fn GetCapabilities();
    fn GetLength();
    fn SetLength();
    fn GetCurrentPosition();
    fn SetCurrentPosition();
    fn IsEndOfStream();
    fn Read();
    fn BeginRead();
    fn EndRead();
    fn Write();
    fn BeginWrite();
    fn EndWrite();
    fn Seek();
    fn Flush();
    fn Close();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFByteStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFByteStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFByteStreamVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLength<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqwlength: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLength<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwlength: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPosition<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqwposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentPosition<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwposition: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEndOfStream<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfendofstream: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Read<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pb: *mut u8, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginRead<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pb: *mut u8, cb: u32, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndRead<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pb: *const u8, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginWrite<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pb: *const u8, cb: u32, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndWrite<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Seek<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seekorigin: MFBYTESTREAM_SEEK_ORIGIN, llseekoffset: i64, dwseekflags: u32, pqwcurrentposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMFByteStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            GetLength: GetLength::<Impl, IMPL_OFFSET>,
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Impl, IMPL_OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Impl, IMPL_OFFSET>,
            IsEndOfStream: IsEndOfStream::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            BeginRead: BeginRead::<Impl, IMPL_OFFSET>,
            EndRead: EndRead::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            BeginWrite: BeginWrite::<Impl, IMPL_OFFSET>,
            EndWrite: EndWrite::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFByteStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFByteStreamBufferingImpl: Sized {
    fn SetBufferingParams();
    fn EnableBuffering();
    fn StopBuffering();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFByteStreamBufferingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFByteStreamBufferingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFByteStreamBufferingVtbl {
        unsafe extern "system" fn SetBufferingParams<Impl: IMFByteStreamBufferingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *const MFBYTESTREAM_BUFFERING_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableBuffering<Impl: IMFByteStreamBufferingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopBuffering<Impl: IMFByteStreamBufferingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetBufferingParams: SetBufferingParams::<Impl, IMPL_OFFSET>,
            EnableBuffering: EnableBuffering::<Impl, IMPL_OFFSET>,
            StopBuffering: StopBuffering::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFByteStreamBuffering as ::windows::core::Interface>::IID
    }
}
pub trait IMFByteStreamCacheControlImpl: Sized {
    fn StopBackgroundTransfer();
}
impl IMFByteStreamCacheControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFByteStreamCacheControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFByteStreamCacheControlVtbl {
        unsafe extern "system" fn StopBackgroundTransfer<Impl: IMFByteStreamCacheControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), StopBackgroundTransfer: StopBackgroundTransfer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFByteStreamCacheControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFByteStreamCacheControl2Impl: Sized + IMFByteStreamCacheControlImpl {
    fn GetByteRanges();
    fn SetCacheLimit();
    fn IsBackgroundTransferActive();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFByteStreamCacheControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFByteStreamCacheControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFByteStreamCacheControl2Vtbl {
        unsafe extern "system" fn GetByteRanges<Impl: IMFByteStreamCacheControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcranges: *mut u32, ppranges: *mut *mut MF_BYTE_STREAM_CACHE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCacheLimit<Impl: IMFByteStreamCacheControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwbytes: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsBackgroundTransferActive<Impl: IMFByteStreamCacheControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFByteStreamCacheControlVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetByteRanges: GetByteRanges::<Impl, IMPL_OFFSET>,
            SetCacheLimit: SetCacheLimit::<Impl, IMPL_OFFSET>,
            IsBackgroundTransferActive: IsBackgroundTransferActive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFByteStreamCacheControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMFByteStreamHandlerImpl: Sized {
    fn BeginCreateObject();
    fn EndCreateObject();
    fn CancelObjectCreation();
    fn GetMaxNumberOfBytesRequiredForResolution();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMFByteStreamHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFByteStreamHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFByteStreamHandlerVtbl {
        unsafe extern "system" fn BeginCreateObject<Impl: IMFByteStreamHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytestream: ::windows::core::RawPtr, pwszurl: super::super::Foundation::PWSTR, dwflags: u32, pprops: ::windows::core::RawPtr, ppiunknowncancelcookie: *mut *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndCreateObject<Impl: IMFByteStreamHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelObjectCreation<Impl: IMFByteStreamHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piunknowncancelcookie: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxNumberOfBytesRequiredForResolution<Impl: IMFByteStreamHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqwbytes: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginCreateObject: BeginCreateObject::<Impl, IMPL_OFFSET>,
            EndCreateObject: EndCreateObject::<Impl, IMPL_OFFSET>,
            CancelObjectCreation: CancelObjectCreation::<Impl, IMPL_OFFSET>,
            GetMaxNumberOfBytesRequiredForResolution: GetMaxNumberOfBytesRequiredForResolution::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFByteStreamHandler as ::windows::core::Interface>::IID
    }
}
pub trait IMFByteStreamProxyClassFactoryImpl: Sized {
    fn CreateByteStreamProxy();
}
impl IMFByteStreamProxyClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFByteStreamProxyClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFByteStreamProxyClassFactoryVtbl {
        unsafe extern "system" fn CreateByteStreamProxy<Impl: IMFByteStreamProxyClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytestream: ::windows::core::RawPtr, pattributes: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateByteStreamProxy: CreateByteStreamProxy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFByteStreamProxyClassFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFByteStreamTimeSeekImpl: Sized {
    fn IsTimeSeekSupported();
    fn TimeSeek();
    fn GetTimeSeekResult();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFByteStreamTimeSeekVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFByteStreamTimeSeekImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFByteStreamTimeSeekVtbl {
        unsafe extern "system" fn IsTimeSeekSupported<Impl: IMFByteStreamTimeSeekImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftimeseekissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TimeSeek<Impl: IMFByteStreamTimeSeekImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwtimeposition: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimeSeekResult<Impl: IMFByteStreamTimeSeekImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqwstarttime: *mut u64, pqwstoptime: *mut u64, pqwduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsTimeSeekSupported: IsTimeSeekSupported::<Impl, IMPL_OFFSET>,
            TimeSeek: TimeSeek::<Impl, IMPL_OFFSET>,
            GetTimeSeekResult: GetTimeSeekResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFByteStreamTimeSeek as ::windows::core::Interface>::IID
    }
}
pub trait IMFCameraOcclusionStateMonitorImpl: Sized {
    fn Start();
    fn Stop();
    fn GetSupportedStates();
}
impl IMFCameraOcclusionStateMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCameraOcclusionStateMonitorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCameraOcclusionStateMonitorVtbl {
        unsafe extern "system" fn Start<Impl: IMFCameraOcclusionStateMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IMFCameraOcclusionStateMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedStates<Impl: IMFCameraOcclusionStateMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            GetSupportedStates: GetSupportedStates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCameraOcclusionStateMonitor as ::windows::core::Interface>::IID
    }
}
pub trait IMFCameraOcclusionStateReportImpl: Sized {
    fn GetOcclusionState();
}
impl IMFCameraOcclusionStateReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCameraOcclusionStateReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCameraOcclusionStateReportVtbl {
        unsafe extern "system" fn GetOcclusionState<Impl: IMFCameraOcclusionStateReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, occlusionstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetOcclusionState: GetOcclusionState::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCameraOcclusionStateReport as ::windows::core::Interface>::IID
    }
}
pub trait IMFCameraOcclusionStateReportCallbackImpl: Sized {
    fn OnOcclusionStateReport();
}
impl IMFCameraOcclusionStateReportCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCameraOcclusionStateReportCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCameraOcclusionStateReportCallbackVtbl {
        unsafe extern "system" fn OnOcclusionStateReport<Impl: IMFCameraOcclusionStateReportCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, occlusionstatereport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnOcclusionStateReport: OnOcclusionStateReport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCameraOcclusionStateReportCallback as ::windows::core::Interface>::IID
    }
}
pub trait IMFCameraSyncObjectImpl: Sized {
    fn WaitOnSignal();
    fn Shutdown();
}
impl IMFCameraSyncObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCameraSyncObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCameraSyncObjectVtbl {
        unsafe extern "system" fn WaitOnSignal<Impl: IMFCameraSyncObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinms: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFCameraSyncObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            WaitOnSignal: WaitOnSignal::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCameraSyncObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFCaptureEngineImpl: Sized {
    fn Initialize();
    fn StartPreview();
    fn StopPreview();
    fn StartRecord();
    fn StopRecord();
    fn TakePhoto();
    fn GetSink();
    fn GetSource();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFCaptureEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCaptureEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCaptureEngineVtbl {
        unsafe extern "system" fn Initialize<Impl: IMFCaptureEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventcallback: ::windows::core::RawPtr, pattributes: ::windows::core::RawPtr, paudiosource: *mut ::core::ffi::c_void, pvideosource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartPreview<Impl: IMFCaptureEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopPreview<Impl: IMFCaptureEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartRecord<Impl: IMFCaptureEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopRecord<Impl: IMFCaptureEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfinalize: super::super::Foundation::BOOL, bflushunprocessedsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TakePhoto<Impl: IMFCaptureEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSink<Impl: IMFCaptureEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mfcaptureenginesinktype: MF_CAPTURE_ENGINE_SINK_TYPE, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSource<Impl: IMFCaptureEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            StartPreview: StartPreview::<Impl, IMPL_OFFSET>,
            StopPreview: StopPreview::<Impl, IMPL_OFFSET>,
            StartRecord: StartRecord::<Impl, IMPL_OFFSET>,
            StopRecord: StopRecord::<Impl, IMPL_OFFSET>,
            TakePhoto: TakePhoto::<Impl, IMPL_OFFSET>,
            GetSink: GetSink::<Impl, IMPL_OFFSET>,
            GetSource: GetSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCaptureEngine as ::windows::core::Interface>::IID
    }
}
pub trait IMFCaptureEngineClassFactoryImpl: Sized {
    fn CreateInstance();
}
impl IMFCaptureEngineClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCaptureEngineClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCaptureEngineClassFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMFCaptureEngineClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCaptureEngineClassFactory as ::windows::core::Interface>::IID
    }
}
pub trait IMFCaptureEngineOnEventCallbackImpl: Sized {
    fn OnEvent();
}
impl IMFCaptureEngineOnEventCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCaptureEngineOnEventCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCaptureEngineOnEventCallbackVtbl {
        unsafe extern "system" fn OnEvent<Impl: IMFCaptureEngineOnEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnEvent: OnEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCaptureEngineOnEventCallback as ::windows::core::Interface>::IID
    }
}
pub trait IMFCaptureEngineOnSampleCallbackImpl: Sized {
    fn OnSample();
}
impl IMFCaptureEngineOnSampleCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCaptureEngineOnSampleCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCaptureEngineOnSampleCallbackVtbl {
        unsafe extern "system" fn OnSample<Impl: IMFCaptureEngineOnSampleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnSample: OnSample::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCaptureEngineOnSampleCallback as ::windows::core::Interface>::IID
    }
}
pub trait IMFCaptureEngineOnSampleCallback2Impl: Sized + IMFCaptureEngineOnSampleCallbackImpl {
    fn OnSynchronizedEvent();
}
impl IMFCaptureEngineOnSampleCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCaptureEngineOnSampleCallback2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCaptureEngineOnSampleCallback2Vtbl {
        unsafe extern "system" fn OnSynchronizedEvent<Impl: IMFCaptureEngineOnSampleCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFCaptureEngineOnSampleCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnSynchronizedEvent: OnSynchronizedEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCaptureEngineOnSampleCallback2 as ::windows::core::Interface>::IID
    }
}
pub trait IMFCapturePhotoConfirmationImpl: Sized {
    fn SetPhotoConfirmationCallback();
    fn SetPixelFormat();
    fn GetPixelFormat();
}
impl IMFCapturePhotoConfirmationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCapturePhotoConfirmationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCapturePhotoConfirmationVtbl {
        unsafe extern "system" fn SetPhotoConfirmationCallback<Impl: IMFCapturePhotoConfirmationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotificationcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPixelFormat<Impl: IMFCapturePhotoConfirmationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IMFCapturePhotoConfirmationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPhotoConfirmationCallback: SetPhotoConfirmationCallback::<Impl, IMPL_OFFSET>,
            SetPixelFormat: SetPixelFormat::<Impl, IMPL_OFFSET>,
            GetPixelFormat: GetPixelFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCapturePhotoConfirmation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFCapturePhotoSinkImpl: Sized + IMFCaptureSinkImpl {
    fn SetOutputFileName();
    fn SetSampleCallback();
    fn SetOutputByteStream();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFCapturePhotoSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCapturePhotoSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCapturePhotoSinkVtbl {
        unsafe extern "system" fn SetOutputFileName<Impl: IMFCapturePhotoSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSampleCallback<Impl: IMFCapturePhotoSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputByteStream<Impl: IMFCapturePhotoSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFCaptureSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOutputFileName: SetOutputFileName::<Impl, IMPL_OFFSET>,
            SetSampleCallback: SetSampleCallback::<Impl, IMPL_OFFSET>,
            SetOutputByteStream: SetOutputByteStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCapturePhotoSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFCapturePreviewSinkImpl: Sized + IMFCaptureSinkImpl {
    fn SetRenderHandle();
    fn SetRenderSurface();
    fn UpdateVideo();
    fn SetSampleCallback();
    fn GetMirrorState();
    fn SetMirrorState();
    fn GetRotation();
    fn SetRotation();
    fn SetCustomSink();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFCapturePreviewSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCapturePreviewSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCapturePreviewSinkVtbl {
        unsafe extern "system" fn SetRenderHandle<Impl: IMFCapturePreviewSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRenderSurface<Impl: IMFCapturePreviewSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psurface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateVideo<Impl: IMFCapturePreviewSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *const MFVideoNormalizedRect, pdst: *const super::super::Foundation::RECT, pborderclr: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSampleCallback<Impl: IMFCapturePreviewSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamsinkindex: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMirrorState<Impl: IMFCapturePreviewSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmirrorstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMirrorState<Impl: IMFCapturePreviewSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmirrorstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRotation<Impl: IMFCapturePreviewSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pdwrotationvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRotation<Impl: IMFCapturePreviewSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwrotationvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCustomSink<Impl: IMFCapturePreviewSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediasink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFCaptureSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRenderHandle: SetRenderHandle::<Impl, IMPL_OFFSET>,
            SetRenderSurface: SetRenderSurface::<Impl, IMPL_OFFSET>,
            UpdateVideo: UpdateVideo::<Impl, IMPL_OFFSET>,
            SetSampleCallback: SetSampleCallback::<Impl, IMPL_OFFSET>,
            GetMirrorState: GetMirrorState::<Impl, IMPL_OFFSET>,
            SetMirrorState: SetMirrorState::<Impl, IMPL_OFFSET>,
            GetRotation: GetRotation::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
            SetCustomSink: SetCustomSink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCapturePreviewSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFCaptureRecordSinkImpl: Sized + IMFCaptureSinkImpl {
    fn SetOutputByteStream();
    fn SetOutputFileName();
    fn SetSampleCallback();
    fn SetCustomSink();
    fn GetRotation();
    fn SetRotation();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFCaptureRecordSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCaptureRecordSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCaptureRecordSinkVtbl {
        unsafe extern "system" fn SetOutputByteStream<Impl: IMFCaptureRecordSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytestream: ::windows::core::RawPtr, guidcontainertype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputFileName<Impl: IMFCaptureRecordSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSampleCallback<Impl: IMFCaptureRecordSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamsinkindex: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCustomSink<Impl: IMFCaptureRecordSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediasink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRotation<Impl: IMFCaptureRecordSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pdwrotationvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRotation<Impl: IMFCaptureRecordSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwrotationvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFCaptureSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOutputByteStream: SetOutputByteStream::<Impl, IMPL_OFFSET>,
            SetOutputFileName: SetOutputFileName::<Impl, IMPL_OFFSET>,
            SetSampleCallback: SetSampleCallback::<Impl, IMPL_OFFSET>,
            SetCustomSink: SetCustomSink::<Impl, IMPL_OFFSET>,
            GetRotation: GetRotation::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCaptureRecordSink as ::windows::core::Interface>::IID
    }
}
pub trait IMFCaptureSinkImpl: Sized {
    fn GetOutputMediaType();
    fn GetService();
    fn AddStream();
    fn Prepare();
    fn RemoveAllStreams();
}
impl IMFCaptureSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCaptureSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCaptureSinkVtbl {
        unsafe extern "system" fn GetOutputMediaType<Impl: IMFCaptureSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsinkstreamindex: u32, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetService<Impl: IMFCaptureSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsinkstreamindex: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStream<Impl: IMFCaptureSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcestreamindex: u32, pmediatype: ::windows::core::RawPtr, pattributes: ::windows::core::RawPtr, pdwsinkstreamindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Prepare<Impl: IMFCaptureSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllStreams<Impl: IMFCaptureSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOutputMediaType: GetOutputMediaType::<Impl, IMPL_OFFSET>,
            GetService: GetService::<Impl, IMPL_OFFSET>,
            AddStream: AddStream::<Impl, IMPL_OFFSET>,
            Prepare: Prepare::<Impl, IMPL_OFFSET>,
            RemoveAllStreams: RemoveAllStreams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCaptureSink as ::windows::core::Interface>::IID
    }
}
pub trait IMFCaptureSink2Impl: Sized + IMFCaptureSinkImpl {
    fn SetOutputMediaType();
}
impl IMFCaptureSink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCaptureSink2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCaptureSink2Vtbl {
        unsafe extern "system" fn SetOutputMediaType<Impl: IMFCaptureSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pmediatype: ::windows::core::RawPtr, pencodingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IMFCaptureSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetOutputMediaType: SetOutputMediaType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCaptureSink2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFCaptureSourceImpl: Sized {
    fn GetCaptureDeviceSource();
    fn GetCaptureDeviceActivate();
    fn GetService();
    fn AddEffect();
    fn RemoveEffect();
    fn RemoveAllEffects();
    fn GetAvailableDeviceMediaType();
    fn SetCurrentDeviceMediaType();
    fn GetCurrentDeviceMediaType();
    fn GetDeviceStreamCount();
    fn GetDeviceStreamCategory();
    fn GetMirrorState();
    fn SetMirrorState();
    fn GetStreamIndexFromFriendlyName();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFCaptureSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCaptureSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCaptureSourceVtbl {
        unsafe extern "system" fn GetCaptureDeviceSource<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppmediasource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaptureDeviceActivate<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppactivate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetService<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEffect<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcestreamindex: u32, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveEffect<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcestreamindex: u32, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllEffects<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcestreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAvailableDeviceMediaType<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcestreamindex: u32, dwmediatypeindex: u32, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentDeviceMediaType<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcestreamindex: u32, pmediatype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentDeviceMediaType<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcestreamindex: u32, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceStreamCount<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceStreamCategory<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcestreamindex: u32, pstreamcategory: *mut MF_CAPTURE_ENGINE_STREAM_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMirrorState<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pfmirrorstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMirrorState<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, fmirrorstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamIndexFromFriendlyName<Impl: IMFCaptureSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uifriendlyname: u32, pdwactualstreamindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCaptureDeviceSource: GetCaptureDeviceSource::<Impl, IMPL_OFFSET>,
            GetCaptureDeviceActivate: GetCaptureDeviceActivate::<Impl, IMPL_OFFSET>,
            GetService: GetService::<Impl, IMPL_OFFSET>,
            AddEffect: AddEffect::<Impl, IMPL_OFFSET>,
            RemoveEffect: RemoveEffect::<Impl, IMPL_OFFSET>,
            RemoveAllEffects: RemoveAllEffects::<Impl, IMPL_OFFSET>,
            GetAvailableDeviceMediaType: GetAvailableDeviceMediaType::<Impl, IMPL_OFFSET>,
            SetCurrentDeviceMediaType: SetCurrentDeviceMediaType::<Impl, IMPL_OFFSET>,
            GetCurrentDeviceMediaType: GetCurrentDeviceMediaType::<Impl, IMPL_OFFSET>,
            GetDeviceStreamCount: GetDeviceStreamCount::<Impl, IMPL_OFFSET>,
            GetDeviceStreamCategory: GetDeviceStreamCategory::<Impl, IMPL_OFFSET>,
            GetMirrorState: GetMirrorState::<Impl, IMPL_OFFSET>,
            SetMirrorState: SetMirrorState::<Impl, IMPL_OFFSET>,
            GetStreamIndexFromFriendlyName: GetStreamIndexFromFriendlyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCaptureSource as ::windows::core::Interface>::IID
    }
}
pub trait IMFCdmSuspendNotifyImpl: Sized {
    fn Begin();
    fn End();
}
impl IMFCdmSuspendNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCdmSuspendNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCdmSuspendNotifyVtbl {
        unsafe extern "system" fn Begin<Impl: IMFCdmSuspendNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn End<Impl: IMFCdmSuspendNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Begin: Begin::<Impl, IMPL_OFFSET>, End: End::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCdmSuspendNotify as ::windows::core::Interface>::IID
    }
}
pub trait IMFClockImpl: Sized {
    fn GetClockCharacteristics();
    fn GetCorrelatedTime();
    fn GetContinuityKey();
    fn GetState();
    fn GetProperties();
}
impl IMFClockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFClockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFClockVtbl {
        unsafe extern "system" fn GetClockCharacteristics<Impl: IMFClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCorrelatedTime<Impl: IMFClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pllclocktime: *mut i64, phnssystemtime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContinuityKey<Impl: IMFClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcontinuitykey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetState<Impl: IMFClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, peclockstate: *mut MFCLOCK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperties<Impl: IMFClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclockproperties: *mut MFCLOCK_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClockCharacteristics: GetClockCharacteristics::<Impl, IMPL_OFFSET>,
            GetCorrelatedTime: GetCorrelatedTime::<Impl, IMPL_OFFSET>,
            GetContinuityKey: GetContinuityKey::<Impl, IMPL_OFFSET>,
            GetState: GetState::<Impl, IMPL_OFFSET>,
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFClock as ::windows::core::Interface>::IID
    }
}
pub trait IMFClockConsumerImpl: Sized {
    fn SetPresentationClock();
    fn GetPresentationClock();
}
impl IMFClockConsumerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFClockConsumerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFClockConsumerVtbl {
        unsafe extern "system" fn SetPresentationClock<Impl: IMFClockConsumerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentationClock<Impl: IMFClockConsumerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppresentationclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPresentationClock: SetPresentationClock::<Impl, IMPL_OFFSET>,
            GetPresentationClock: GetPresentationClock::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFClockConsumer as ::windows::core::Interface>::IID
    }
}
pub trait IMFClockStateSinkImpl: Sized {
    fn OnClockStart();
    fn OnClockStop();
    fn OnClockPause();
    fn OnClockRestart();
    fn OnClockSetRate();
}
impl IMFClockStateSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFClockStateSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFClockStateSinkVtbl {
        unsafe extern "system" fn OnClockStart<Impl: IMFClockStateSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnssystemtime: i64, llclockstartoffset: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnClockStop<Impl: IMFClockStateSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnssystemtime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnClockPause<Impl: IMFClockStateSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnssystemtime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnClockRestart<Impl: IMFClockStateSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnssystemtime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnClockSetRate<Impl: IMFClockStateSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnssystemtime: i64, flrate: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnClockStart: OnClockStart::<Impl, IMPL_OFFSET>,
            OnClockStop: OnClockStop::<Impl, IMPL_OFFSET>,
            OnClockPause: OnClockPause::<Impl, IMPL_OFFSET>,
            OnClockRestart: OnClockRestart::<Impl, IMPL_OFFSET>,
            OnClockSetRate: OnClockSetRate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFClockStateSink as ::windows::core::Interface>::IID
    }
}
pub trait IMFCollectionImpl: Sized {
    fn GetElementCount();
    fn GetElement();
    fn AddElement();
    fn RemoveElement();
    fn InsertElementAt();
    fn RemoveAllElements();
}
impl IMFCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFCollectionVtbl {
        unsafe extern "system" fn GetElementCount<Impl: IMFCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelements: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetElement<Impl: IMFCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwelementindex: u32, ppunkelement: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddElement<Impl: IMFCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkelement: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveElement<Impl: IMFCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwelementindex: u32, ppunkelement: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertElementAt<Impl: IMFCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllElements<Impl: IMFCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetElementCount: GetElementCount::<Impl, IMPL_OFFSET>,
            GetElement: GetElement::<Impl, IMPL_OFFSET>,
            AddElement: AddElement::<Impl, IMPL_OFFSET>,
            RemoveElement: RemoveElement::<Impl, IMPL_OFFSET>,
            InsertElementAt: InsertElementAt::<Impl, IMPL_OFFSET>,
            RemoveAllElements: RemoveAllElements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFCollection as ::windows::core::Interface>::IID
    }
}
pub trait IMFContentDecryptionModuleImpl: Sized {
    fn SetContentEnabler();
    fn GetSuspendNotify();
    fn SetPMPHostApp();
    fn CreateSession();
    fn SetServerCertificate();
    fn CreateTrustedInput();
    fn GetProtectionSystemIds();
}
impl IMFContentDecryptionModuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFContentDecryptionModuleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFContentDecryptionModuleVtbl {
        unsafe extern "system" fn SetContentEnabler<Impl: IMFContentDecryptionModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentenabler: ::windows::core::RawPtr, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSuspendNotify<Impl: IMFContentDecryptionModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPMPHostApp<Impl: IMFContentDecryptionModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmphostapp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSession<Impl: IMFContentDecryptionModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessiontype: MF_MEDIAKEYSESSION_TYPE, callbacks: ::windows::core::RawPtr, session: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServerCertificate<Impl: IMFContentDecryptionModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const u8, certificatesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTrustedInput<Impl: IMFContentDecryptionModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentinitdata: *const u8, contentinitdatasize: u32, trustedinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtectionSystemIds<Impl: IMFContentDecryptionModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemids: *mut *mut ::windows::core::GUID, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetContentEnabler: SetContentEnabler::<Impl, IMPL_OFFSET>,
            GetSuspendNotify: GetSuspendNotify::<Impl, IMPL_OFFSET>,
            SetPMPHostApp: SetPMPHostApp::<Impl, IMPL_OFFSET>,
            CreateSession: CreateSession::<Impl, IMPL_OFFSET>,
            SetServerCertificate: SetServerCertificate::<Impl, IMPL_OFFSET>,
            CreateTrustedInput: CreateTrustedInput::<Impl, IMPL_OFFSET>,
            GetProtectionSystemIds: GetProtectionSystemIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFContentDecryptionModule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMFContentDecryptionModuleAccessImpl: Sized {
    fn CreateContentDecryptionModule();
    fn GetConfiguration();
    fn GetKeySystem();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMFContentDecryptionModuleAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFContentDecryptionModuleAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFContentDecryptionModuleAccessVtbl {
        unsafe extern "system" fn CreateContentDecryptionModule<Impl: IMFContentDecryptionModuleAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentdecryptionmoduleproperties: ::windows::core::RawPtr, contentdecryptionmodule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConfiguration<Impl: IMFContentDecryptionModuleAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeySystem<Impl: IMFContentDecryptionModuleAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keysystem: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateContentDecryptionModule: CreateContentDecryptionModule::<Impl, IMPL_OFFSET>,
            GetConfiguration: GetConfiguration::<Impl, IMPL_OFFSET>,
            GetKeySystem: GetKeySystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFContentDecryptionModuleAccess as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMFContentDecryptionModuleFactoryImpl: Sized {
    fn IsTypeSupported();
    fn CreateContentDecryptionModuleAccess();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMFContentDecryptionModuleFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFContentDecryptionModuleFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFContentDecryptionModuleFactoryVtbl {
        unsafe extern "system" fn IsTypeSupported<Impl: IMFContentDecryptionModuleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keysystem: super::super::Foundation::PWSTR, contenttype: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateContentDecryptionModuleAccess<Impl: IMFContentDecryptionModuleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keysystem: super::super::Foundation::PWSTR, configurations: *const ::windows::core::RawPtr, numconfigurations: u32, contentdecryptionmoduleaccess: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsTypeSupported: IsTypeSupported::<Impl, IMPL_OFFSET>,
            CreateContentDecryptionModuleAccess: CreateContentDecryptionModuleAccess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFContentDecryptionModuleFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFContentDecryptionModuleSessionImpl: Sized {
    fn GetSessionId();
    fn GetExpiration();
    fn GetKeyStatuses();
    fn Load();
    fn GenerateRequest();
    fn Update();
    fn Close();
    fn Remove();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFContentDecryptionModuleSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFContentDecryptionModuleSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFContentDecryptionModuleSessionVtbl {
        unsafe extern "system" fn GetSessionId<Impl: IMFContentDecryptionModuleSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExpiration<Impl: IMFContentDecryptionModuleSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expiration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyStatuses<Impl: IMFContentDecryptionModuleSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keystatuses: *mut *mut MFMediaKeyStatus, numkeystatuses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IMFContentDecryptionModuleSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: super::super::Foundation::PWSTR, loaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateRequest<Impl: IMFContentDecryptionModuleSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initdatatype: super::super::Foundation::PWSTR, initdata: *const u8, initdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IMFContentDecryptionModuleSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: *const u8, responsesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMFContentDecryptionModuleSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IMFContentDecryptionModuleSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSessionId: GetSessionId::<Impl, IMPL_OFFSET>,
            GetExpiration: GetExpiration::<Impl, IMPL_OFFSET>,
            GetKeyStatuses: GetKeyStatuses::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            GenerateRequest: GenerateRequest::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFContentDecryptionModuleSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFContentDecryptionModuleSessionCallbacksImpl: Sized {
    fn KeyMessage();
    fn KeyStatusChanged();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFContentDecryptionModuleSessionCallbacksVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFContentDecryptionModuleSessionCallbacksImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFContentDecryptionModuleSessionCallbacksVtbl {
        unsafe extern "system" fn KeyMessage<Impl: IMFContentDecryptionModuleSessionCallbacksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: MF_MEDIAKEYSESSION_MESSAGETYPE, message: *const u8, messagesize: u32, destinationurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyStatusChanged<Impl: IMFContentDecryptionModuleSessionCallbacksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            KeyMessage: KeyMessage::<Impl, IMPL_OFFSET>,
            KeyStatusChanged: KeyStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFContentDecryptionModuleSessionCallbacks as ::windows::core::Interface>::IID
    }
}
pub trait IMFContentDecryptorContextImpl: Sized {
    fn InitializeHardwareKey();
}
impl IMFContentDecryptorContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFContentDecryptorContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFContentDecryptorContextVtbl {
        unsafe extern "system" fn InitializeHardwareKey<Impl: IMFContentDecryptorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputprivatedatabytecount: u32, inputprivatedata: *const ::core::ffi::c_void, outputprivatedata: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), InitializeHardwareKey: InitializeHardwareKey::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFContentDecryptorContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFContentEnablerImpl: Sized {
    fn GetEnableType();
    fn GetEnableURL();
    fn GetEnableData();
    fn IsAutomaticSupported();
    fn AutomaticEnable();
    fn MonitorEnable();
    fn Cancel();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFContentEnablerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFContentEnablerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFContentEnablerVtbl {
        unsafe extern "system" fn GetEnableType<Impl: IMFContentEnablerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnableURL<Impl: IMFContentEnablerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszurl: *mut super::super::Foundation::PWSTR, pcchurl: *mut u32, ptruststatus: *mut MF_URL_TRUST_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnableData<Impl: IMFContentEnablerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAutomaticSupported<Impl: IMFContentEnablerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfautomatic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutomaticEnable<Impl: IMFContentEnablerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MonitorEnable<Impl: IMFContentEnablerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IMFContentEnablerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEnableType: GetEnableType::<Impl, IMPL_OFFSET>,
            GetEnableURL: GetEnableURL::<Impl, IMPL_OFFSET>,
            GetEnableData: GetEnableData::<Impl, IMPL_OFFSET>,
            IsAutomaticSupported: IsAutomaticSupported::<Impl, IMPL_OFFSET>,
            AutomaticEnable: AutomaticEnable::<Impl, IMPL_OFFSET>,
            MonitorEnable: MonitorEnable::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFContentEnabler as ::windows::core::Interface>::IID
    }
}
pub trait IMFContentProtectionDeviceImpl: Sized {
    fn InvokeFunction();
    fn GetPrivateDataByteCount();
}
impl IMFContentProtectionDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFContentProtectionDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFContentProtectionDeviceVtbl {
        unsafe extern "system" fn InvokeFunction<Impl: IMFContentProtectionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: u32, inputbufferbytecount: u32, inputbuffer: *const u8, outputbufferbytecount: *mut u32, outputbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateDataByteCount<Impl: IMFContentProtectionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privateinputbytecount: *mut u32, privateoutputbytecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InvokeFunction: InvokeFunction::<Impl, IMPL_OFFSET>,
            GetPrivateDataByteCount: GetPrivateDataByteCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFContentProtectionDevice as ::windows::core::Interface>::IID
    }
}
pub trait IMFContentProtectionManagerImpl: Sized {
    fn BeginEnableContent();
    fn EndEnableContent();
}
impl IMFContentProtectionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFContentProtectionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFContentProtectionManagerVtbl {
        unsafe extern "system" fn BeginEnableContent<Impl: IMFContentProtectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penableractivate: ::windows::core::RawPtr, ptopo: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEnableContent<Impl: IMFContentProtectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginEnableContent: BeginEnableContent::<Impl, IMPL_OFFSET>,
            EndEnableContent: EndEnableContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFContentProtectionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFD3D12SynchronizationObjectImpl: Sized {
    fn SignalEventOnFinalResourceRelease();
    fn Reset();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFD3D12SynchronizationObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFD3D12SynchronizationObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFD3D12SynchronizationObjectVtbl {
        unsafe extern "system" fn SignalEventOnFinalResourceRelease<Impl: IMFD3D12SynchronizationObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IMFD3D12SynchronizationObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SignalEventOnFinalResourceRelease: SignalEventOnFinalResourceRelease::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFD3D12SynchronizationObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IMFD3D12SynchronizationObjectCommandsImpl: Sized {
    fn EnqueueResourceReady();
    fn EnqueueResourceReadyWait();
    fn SignalEventOnResourceReady();
    fn EnqueueResourceRelease();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IMFD3D12SynchronizationObjectCommandsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFD3D12SynchronizationObjectCommandsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFD3D12SynchronizationObjectCommandsVtbl {
        unsafe extern "system" fn EnqueueResourceReady<Impl: IMFD3D12SynchronizationObjectCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproducercommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnqueueResourceReadyWait<Impl: IMFD3D12SynchronizationObjectCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconsumercommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignalEventOnResourceReady<Impl: IMFD3D12SynchronizationObjectCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnqueueResourceRelease<Impl: IMFD3D12SynchronizationObjectCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconsumercommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnqueueResourceReady: EnqueueResourceReady::<Impl, IMPL_OFFSET>,
            EnqueueResourceReadyWait: EnqueueResourceReadyWait::<Impl, IMPL_OFFSET>,
            SignalEventOnResourceReady: SignalEventOnResourceReady::<Impl, IMPL_OFFSET>,
            EnqueueResourceRelease: EnqueueResourceRelease::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFD3D12SynchronizationObjectCommands as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFDLNASinkInitImpl: Sized {
    fn Initialize();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFDLNASinkInitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDLNASinkInitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFDLNASinkInitVtbl {
        unsafe extern "system" fn Initialize<Impl: IMFDLNASinkInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytestream: ::windows::core::RawPtr, fpal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDLNASinkInit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFDRMNetHelperImpl: Sized {
    fn ProcessLicenseRequest();
    fn GetChainedLicenseResponse();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFDRMNetHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDRMNetHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFDRMNetHelperVtbl {
        unsafe extern "system" fn ProcessLicenseRequest<Impl: IMFDRMNetHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plicenserequest: *const u8, cblicenserequest: u32, pplicenseresponse: *mut *mut u8, pcblicenseresponse: *mut u32, pbstrkid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChainedLicenseResponse<Impl: IMFDRMNetHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicenseresponse: *mut *mut u8, pcblicenseresponse: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProcessLicenseRequest: ProcessLicenseRequest::<Impl, IMPL_OFFSET>,
            GetChainedLicenseResponse: GetChainedLicenseResponse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDRMNetHelper as ::windows::core::Interface>::IID
    }
}
pub trait IMFDXGIBufferImpl: Sized {
    fn GetResource();
    fn GetSubresourceIndex();
    fn GetUnknown();
    fn SetUnknown();
}
impl IMFDXGIBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDXGIBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFDXGIBufferVtbl {
        unsafe extern "system" fn GetResource<Impl: IMFDXGIBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubresourceIndex<Impl: IMFDXGIBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusubresource: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnknown<Impl: IMFDXGIBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnknown<Impl: IMFDXGIBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, punkdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetResource: GetResource::<Impl, IMPL_OFFSET>,
            GetSubresourceIndex: GetSubresourceIndex::<Impl, IMPL_OFFSET>,
            GetUnknown: GetUnknown::<Impl, IMPL_OFFSET>,
            SetUnknown: SetUnknown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDXGIBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFDXGIDeviceManagerImpl: Sized {
    fn CloseDeviceHandle();
    fn GetVideoService();
    fn LockDevice();
    fn OpenDeviceHandle();
    fn ResetDevice();
    fn TestDevice();
    fn UnlockDevice();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFDXGIDeviceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDXGIDeviceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFDXGIDeviceManagerVtbl {
        unsafe extern "system" fn CloseDeviceHandle<Impl: IMFDXGIDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdevice: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoService<Impl: IMFDXGIDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdevice: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockDevice<Impl: IMFDXGIDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdevice: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppunkdevice: *mut *mut ::core::ffi::c_void, fblock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenDeviceHandle<Impl: IMFDXGIDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdevice: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetDevice<Impl: IMFDXGIDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdevice: *mut ::core::ffi::c_void, resettoken: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TestDevice<Impl: IMFDXGIDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdevice: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockDevice<Impl: IMFDXGIDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdevice: super::super::Foundation::HANDLE, fsavestate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CloseDeviceHandle: CloseDeviceHandle::<Impl, IMPL_OFFSET>,
            GetVideoService: GetVideoService::<Impl, IMPL_OFFSET>,
            LockDevice: LockDevice::<Impl, IMPL_OFFSET>,
            OpenDeviceHandle: OpenDeviceHandle::<Impl, IMPL_OFFSET>,
            ResetDevice: ResetDevice::<Impl, IMPL_OFFSET>,
            TestDevice: TestDevice::<Impl, IMPL_OFFSET>,
            UnlockDevice: UnlockDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDXGIDeviceManager as ::windows::core::Interface>::IID
    }
}
pub trait IMFDXGIDeviceManagerSourceImpl: Sized {
    fn GetManager();
}
impl IMFDXGIDeviceManagerSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDXGIDeviceManagerSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFDXGIDeviceManagerSourceVtbl {
        unsafe extern "system" fn GetManager<Impl: IMFDXGIDeviceManagerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetManager: GetManager::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDXGIDeviceManagerSource as ::windows::core::Interface>::IID
    }
}
pub trait IMFDesiredSampleImpl: Sized {
    fn GetDesiredSampleTimeAndDuration();
    fn SetDesiredSampleTimeAndDuration();
    fn Clear();
}
impl IMFDesiredSampleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDesiredSampleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFDesiredSampleVtbl {
        unsafe extern "system" fn GetDesiredSampleTimeAndDuration<Impl: IMFDesiredSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnssampletime: *mut i64, phnssampleduration: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredSampleTimeAndDuration<Impl: IMFDesiredSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnssampletime: i64, hnssampleduration: i64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IMFDesiredSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDesiredSampleTimeAndDuration: GetDesiredSampleTimeAndDuration::<Impl, IMPL_OFFSET>,
            SetDesiredSampleTimeAndDuration: SetDesiredSampleTimeAndDuration::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDesiredSample as ::windows::core::Interface>::IID
    }
}
pub trait IMFExtendedCameraControlImpl: Sized {
    fn GetCapabilities();
    fn SetFlags();
    fn GetFlags();
    fn LockPayload();
    fn UnlockPayload();
    fn CommitSettings();
}
impl IMFExtendedCameraControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFExtendedCameraControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFExtendedCameraControlVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IMFExtendedCameraControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: IMFExtendedCameraControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IMFExtendedCameraControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockPayload<Impl: IMFExtendedCameraControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppayload: *mut *mut u8, pulpayload: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockPayload<Impl: IMFExtendedCameraControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitSettings<Impl: IMFExtendedCameraControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            LockPayload: LockPayload::<Impl, IMPL_OFFSET>,
            UnlockPayload: UnlockPayload::<Impl, IMPL_OFFSET>,
            CommitSettings: CommitSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFExtendedCameraControl as ::windows::core::Interface>::IID
    }
}
pub trait IMFExtendedCameraControllerImpl: Sized {
    fn GetExtendedCameraControl();
}
impl IMFExtendedCameraControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFExtendedCameraControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFExtendedCameraControllerVtbl {
        unsafe extern "system" fn GetExtendedCameraControl<Impl: IMFExtendedCameraControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ulpropertyid: u32, ppcontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetExtendedCameraControl: GetExtendedCameraControl::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFExtendedCameraController as ::windows::core::Interface>::IID
    }
}
pub trait IMFExtendedCameraIntrinsicModelImpl: Sized {
    fn GetModel();
    fn SetModel();
    fn GetDistortionModelType();
}
impl IMFExtendedCameraIntrinsicModelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFExtendedCameraIntrinsicModelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFExtendedCameraIntrinsicModelVtbl {
        unsafe extern "system" fn GetModel<Impl: IMFExtendedCameraIntrinsicModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintrinsicmodel: *mut MFExtendedCameraIntrinsic_IntrinsicModel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModel<Impl: IMFExtendedCameraIntrinsicModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintrinsicmodel: *const MFExtendedCameraIntrinsic_IntrinsicModel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDistortionModelType<Impl: IMFExtendedCameraIntrinsicModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdistortionmodeltype: *mut MFCameraIntrinsic_DistortionModelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetModel: GetModel::<Impl, IMPL_OFFSET>,
            SetModel: SetModel::<Impl, IMPL_OFFSET>,
            GetDistortionModelType: GetDistortionModelType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFExtendedCameraIntrinsicModel as ::windows::core::Interface>::IID
    }
}
pub trait IMFExtendedCameraIntrinsicsImpl: Sized {
    fn InitializeFromBuffer();
    fn GetBufferSize();
    fn SerializeToBuffer();
    fn GetIntrinsicModelCount();
    fn GetIntrinsicModelByIndex();
    fn AddIntrinsicModel();
}
impl IMFExtendedCameraIntrinsicsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFExtendedCameraIntrinsicsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFExtendedCameraIntrinsicsVtbl {
        unsafe extern "system" fn InitializeFromBuffer<Impl: IMFExtendedCameraIntrinsicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, dwbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferSize<Impl: IMFExtendedCameraIntrinsicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SerializeToBuffer<Impl: IMFExtendedCameraIntrinsicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIntrinsicModelCount<Impl: IMFExtendedCameraIntrinsicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIntrinsicModelByIndex<Impl: IMFExtendedCameraIntrinsicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppintrinsicmodel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddIntrinsicModel<Impl: IMFExtendedCameraIntrinsicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintrinsicmodel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitializeFromBuffer: InitializeFromBuffer::<Impl, IMPL_OFFSET>,
            GetBufferSize: GetBufferSize::<Impl, IMPL_OFFSET>,
            SerializeToBuffer: SerializeToBuffer::<Impl, IMPL_OFFSET>,
            GetIntrinsicModelCount: GetIntrinsicModelCount::<Impl, IMPL_OFFSET>,
            GetIntrinsicModelByIndex: GetIntrinsicModelByIndex::<Impl, IMPL_OFFSET>,
            AddIntrinsicModel: AddIntrinsicModel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFExtendedCameraIntrinsics as ::windows::core::Interface>::IID
    }
}
pub trait IMFExtendedCameraIntrinsicsDistortionModel6KTImpl: Sized {
    fn GetDistortionModel();
    fn SetDistortionModel();
}
impl IMFExtendedCameraIntrinsicsDistortionModel6KTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFExtendedCameraIntrinsicsDistortionModel6KTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFExtendedCameraIntrinsicsDistortionModel6KTVtbl {
        unsafe extern "system" fn GetDistortionModel<Impl: IMFExtendedCameraIntrinsicsDistortionModel6KTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdistortionmodel: *mut MFCameraIntrinsic_DistortionModel6KT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDistortionModel<Impl: IMFExtendedCameraIntrinsicsDistortionModel6KTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdistortionmodel: *const MFCameraIntrinsic_DistortionModel6KT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDistortionModel: GetDistortionModel::<Impl, IMPL_OFFSET>,
            SetDistortionModel: SetDistortionModel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFExtendedCameraIntrinsicsDistortionModel6KT as ::windows::core::Interface>::IID
    }
}
pub trait IMFExtendedCameraIntrinsicsDistortionModelArcTanImpl: Sized {
    fn GetDistortionModel();
    fn SetDistortionModel();
}
impl IMFExtendedCameraIntrinsicsDistortionModelArcTanVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFExtendedCameraIntrinsicsDistortionModelArcTanImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFExtendedCameraIntrinsicsDistortionModelArcTanVtbl {
        unsafe extern "system" fn GetDistortionModel<Impl: IMFExtendedCameraIntrinsicsDistortionModelArcTanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdistortionmodel: *mut MFCameraIntrinsic_DistortionModelArcTan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDistortionModel<Impl: IMFExtendedCameraIntrinsicsDistortionModelArcTanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdistortionmodel: *const MFCameraIntrinsic_DistortionModelArcTan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDistortionModel: GetDistortionModel::<Impl, IMPL_OFFSET>,
            SetDistortionModel: SetDistortionModel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFExtendedCameraIntrinsicsDistortionModelArcTan as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFExtendedDRMTypeSupportImpl: Sized {
    fn IsTypeSupportedEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFExtendedDRMTypeSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFExtendedDRMTypeSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFExtendedDRMTypeSupportVtbl {
        unsafe extern "system" fn IsTypeSupportedEx<Impl: IMFExtendedDRMTypeSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keysystem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, panswer: *mut MF_MEDIA_ENGINE_CANPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IsTypeSupportedEx: IsTypeSupportedEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFExtendedDRMTypeSupport as ::windows::core::Interface>::IID
    }
}
pub trait IMFFieldOfUseMFTUnlockImpl: Sized {
    fn Unlock();
}
impl IMFFieldOfUseMFTUnlockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFFieldOfUseMFTUnlockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFFieldOfUseMFTUnlockVtbl {
        unsafe extern "system" fn Unlock<Impl: IMFFieldOfUseMFTUnlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkmft: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Unlock: Unlock::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFFieldOfUseMFTUnlock as ::windows::core::Interface>::IID
    }
}
pub trait IMFFinalizableMediaSinkImpl: Sized + IMFMediaSinkImpl {
    fn BeginFinalize();
    fn EndFinalize();
}
impl IMFFinalizableMediaSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFFinalizableMediaSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFFinalizableMediaSinkVtbl {
        unsafe extern "system" fn BeginFinalize<Impl: IMFFinalizableMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndFinalize<Impl: IMFFinalizableMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginFinalize: BeginFinalize::<Impl, IMPL_OFFSET>,
            EndFinalize: EndFinalize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFFinalizableMediaSink as ::windows::core::Interface>::IID
    }
}
pub trait IMFGetServiceImpl: Sized {
    fn GetService();
}
impl IMFGetServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFGetServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFGetServiceVtbl {
        unsafe extern "system" fn GetService<Impl: IMFGetServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetService: GetService::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFGetService as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFHDCPStatusImpl: Sized {
    fn Query();
    fn Set();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFHDCPStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFHDCPStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFHDCPStatusVtbl {
        unsafe extern "system" fn Query<Impl: IMFHDCPStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut MF_HDCP_STATUS, pfstatus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Set<Impl: IMFHDCPStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: MF_HDCP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Query: Query::<Impl, IMPL_OFFSET>, Set: Set::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFHDCPStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFHttpDownloadRequestImpl: Sized {
    fn AddHeader();
    fn BeginSendRequest();
    fn EndSendRequest();
    fn BeginReceiveResponse();
    fn EndReceiveResponse();
    fn BeginReadPayload();
    fn EndReadPayload();
    fn QueryHeader();
    fn GetURL();
    fn HasNullSourceOrigin();
    fn GetTimeSeekResult();
    fn GetHttpStatus();
    fn GetAtEndOfPayload();
    fn GetTotalLength();
    fn GetRangeEndOffset();
    fn Close();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFHttpDownloadRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFHttpDownloadRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFHttpDownloadRequestVtbl {
        unsafe extern "system" fn AddHeader<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szheader: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginSendRequest<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpayload: *const u8, cbpayload: u32, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndSendRequest<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginReceiveResponse<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndReceiveResponse<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginReadPayload<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pb: *mut u8, cb: u32, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndReadPayload<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, pqwoffset: *mut u64, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryHeader<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szheadername: super::super::Foundation::PWSTR, dwindex: u32, ppszheadervalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetURL<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasNullSourceOrigin<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnullsourceorigin: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimeSeekResult<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqwstarttime: *mut u64, pqwstoptime: *mut u64, pqwduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHttpStatus<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhttpstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAtEndOfPayload<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfatendofpayload: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTotalLength<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqwtotallength: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRangeEndOffset<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqwrangeend: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMFHttpDownloadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddHeader: AddHeader::<Impl, IMPL_OFFSET>,
            BeginSendRequest: BeginSendRequest::<Impl, IMPL_OFFSET>,
            EndSendRequest: EndSendRequest::<Impl, IMPL_OFFSET>,
            BeginReceiveResponse: BeginReceiveResponse::<Impl, IMPL_OFFSET>,
            EndReceiveResponse: EndReceiveResponse::<Impl, IMPL_OFFSET>,
            BeginReadPayload: BeginReadPayload::<Impl, IMPL_OFFSET>,
            EndReadPayload: EndReadPayload::<Impl, IMPL_OFFSET>,
            QueryHeader: QueryHeader::<Impl, IMPL_OFFSET>,
            GetURL: GetURL::<Impl, IMPL_OFFSET>,
            HasNullSourceOrigin: HasNullSourceOrigin::<Impl, IMPL_OFFSET>,
            GetTimeSeekResult: GetTimeSeekResult::<Impl, IMPL_OFFSET>,
            GetHttpStatus: GetHttpStatus::<Impl, IMPL_OFFSET>,
            GetAtEndOfPayload: GetAtEndOfPayload::<Impl, IMPL_OFFSET>,
            GetTotalLength: GetTotalLength::<Impl, IMPL_OFFSET>,
            GetRangeEndOffset: GetRangeEndOffset::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFHttpDownloadRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFHttpDownloadSessionImpl: Sized {
    fn SetServer();
    fn CreateRequest();
    fn Close();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFHttpDownloadSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFHttpDownloadSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFHttpDownloadSessionVtbl {
        unsafe extern "system" fn SetServer<Impl: IMFHttpDownloadSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szservername: super::super::Foundation::PWSTR, nport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRequest<Impl: IMFHttpDownloadSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szobjectname: super::super::Foundation::PWSTR, fbypassproxycache: super::super::Foundation::BOOL, fsecure: super::super::Foundation::BOOL, szverb: super::super::Foundation::PWSTR, szreferrer: super::super::Foundation::PWSTR, pprequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMFHttpDownloadSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetServer: SetServer::<Impl, IMPL_OFFSET>,
            CreateRequest: CreateRequest::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFHttpDownloadSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFHttpDownloadSessionProviderImpl: Sized {
    fn CreateHttpDownloadSession();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFHttpDownloadSessionProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFHttpDownloadSessionProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFHttpDownloadSessionProviderVtbl {
        unsafe extern "system" fn CreateHttpDownloadSession<Impl: IMFHttpDownloadSessionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszscheme: super::super::Foundation::PWSTR, ppdownloadsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateHttpDownloadSession: CreateHttpDownloadSession::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFHttpDownloadSessionProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFImageSharingEngineImpl: Sized {
    fn SetSource();
    fn GetDevice();
    fn Shutdown();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFImageSharingEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFImageSharingEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFImageSharingEngineVtbl {
        unsafe extern "system" fn SetSource<Impl: IMFImageSharingEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDevice<Impl: IMFImageSharingEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut DEVICE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFImageSharingEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFImageSharingEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFImageSharingEngineClassFactoryImpl: Sized {
    fn CreateInstanceFromUDN();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFImageSharingEngineClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFImageSharingEngineClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFImageSharingEngineClassFactoryVtbl {
        unsafe extern "system" fn CreateInstanceFromUDN<Impl: IMFImageSharingEngineClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puniquedevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppengine: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateInstanceFromUDN: CreateInstanceFromUDN::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFImageSharingEngineClassFactory as ::windows::core::Interface>::IID
    }
}
pub trait IMFInputTrustAuthorityImpl: Sized {
    fn GetDecrypter();
    fn RequestAccess();
    fn GetPolicy();
    fn BindAccess();
    fn UpdateAccess();
    fn Reset();
}
impl IMFInputTrustAuthorityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFInputTrustAuthorityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFInputTrustAuthorityVtbl {
        unsafe extern "system" fn GetDecrypter<Impl: IMFInputTrustAuthorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAccess<Impl: IMFInputTrustAuthorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: MFPOLICYMANAGER_ACTION, ppcontentenableractivate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPolicy<Impl: IMFInputTrustAuthorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: MFPOLICYMANAGER_ACTION, pppolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindAccess<Impl: IMFInputTrustAuthorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam: *const MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateAccess<Impl: IMFInputTrustAuthorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam: *const MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IMFInputTrustAuthorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDecrypter: GetDecrypter::<Impl, IMPL_OFFSET>,
            RequestAccess: RequestAccess::<Impl, IMPL_OFFSET>,
            GetPolicy: GetPolicy::<Impl, IMPL_OFFSET>,
            BindAccess: BindAccess::<Impl, IMPL_OFFSET>,
            UpdateAccess: UpdateAccess::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFInputTrustAuthority as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFLocalMFTRegistrationImpl: Sized {
    fn RegisterMFTs();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFLocalMFTRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFLocalMFTRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFLocalMFTRegistrationVtbl {
        unsafe extern "system" fn RegisterMFTs<Impl: IMFLocalMFTRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmfts: *const MFT_REGISTRATION_INFO, cmfts: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), RegisterMFTs: RegisterMFTs::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFLocalMFTRegistration as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaBufferImpl: Sized {
    fn Lock();
    fn Unlock();
    fn GetCurrentLength();
    fn SetCurrentLength();
    fn GetMaxLength();
}
impl IMFMediaBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaBufferVtbl {
        unsafe extern "system" fn Lock<Impl: IMFMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbbuffer: *mut *mut u8, pcbmaxlength: *mut u32, pcbcurrentlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IMFMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentLength<Impl: IMFMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbcurrentlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentLength<Impl: IMFMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbcurrentlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxLength<Impl: IMFMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbmaxlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Lock: Lock::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
            GetCurrentLength: GetCurrentLength::<Impl, IMPL_OFFSET>,
            SetCurrentLength: SetCurrentLength::<Impl, IMPL_OFFSET>,
            GetMaxLength: GetMaxLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineImpl: Sized {
    fn GetError();
    fn SetErrorCode();
    fn SetSourceElements();
    fn SetSource();
    fn GetCurrentSource();
    fn GetNetworkState();
    fn GetPreload();
    fn SetPreload();
    fn GetBuffered();
    fn Load();
    fn CanPlayType();
    fn GetReadyState();
    fn IsSeeking();
    fn GetCurrentTime();
    fn SetCurrentTime();
    fn GetStartTime();
    fn GetDuration();
    fn IsPaused();
    fn GetDefaultPlaybackRate();
    fn SetDefaultPlaybackRate();
    fn GetPlaybackRate();
    fn SetPlaybackRate();
    fn GetPlayed();
    fn GetSeekable();
    fn IsEnded();
    fn GetAutoPlay();
    fn SetAutoPlay();
    fn GetLoop();
    fn SetLoop();
    fn Play();
    fn Pause();
    fn GetMuted();
    fn SetMuted();
    fn GetVolume();
    fn SetVolume();
    fn HasVideo();
    fn HasAudio();
    fn GetNativeVideoSize();
    fn GetVideoAspectRatio();
    fn Shutdown();
    fn TransferVideoFrame();
    fn OnVideoStreamTick();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineVtbl {
        unsafe extern "system" fn GetError<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetErrorCode<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: MF_MEDIA_ENGINE_ERR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourceElements<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcelements: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSource<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentSource<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkState<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u16 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreload<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> MF_MEDIA_ENGINE_PRELOAD {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreload<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preload: MF_MEDIA_ENGINE_PRELOAD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBuffered<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuffered: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanPlayType<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, panswer: *mut MF_MEDIA_ENGINE_CANPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReadyState<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u16 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSeeking<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentTime<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentTime<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seektime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStartTime<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuration<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPaused<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultPlaybackRate<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultPlaybackRate<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPlaybackRate<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlaybackRate<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPlayed<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplayed: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSeekable<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppseekable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEnded<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutoPlay<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoPlay<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoplay: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLoop<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLoop<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#loop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Play<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMuted<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMuted<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, muted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVolume<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVolume<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasVideo<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasAudio<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNativeVideoSize<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: *mut u32, cy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoAspectRatio<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: *mut u32, cy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransferVideoFrame<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstsurf: *mut ::core::ffi::c_void, psrc: *const MFVideoNormalizedRect, pdst: *const super::super::Foundation::RECT, pborderclr: *const MFARGB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnVideoStreamTick<Impl: IMFMediaEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppts: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetError: GetError::<Impl, IMPL_OFFSET>,
            SetErrorCode: SetErrorCode::<Impl, IMPL_OFFSET>,
            SetSourceElements: SetSourceElements::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
            GetCurrentSource: GetCurrentSource::<Impl, IMPL_OFFSET>,
            GetNetworkState: GetNetworkState::<Impl, IMPL_OFFSET>,
            GetPreload: GetPreload::<Impl, IMPL_OFFSET>,
            SetPreload: SetPreload::<Impl, IMPL_OFFSET>,
            GetBuffered: GetBuffered::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            CanPlayType: CanPlayType::<Impl, IMPL_OFFSET>,
            GetReadyState: GetReadyState::<Impl, IMPL_OFFSET>,
            IsSeeking: IsSeeking::<Impl, IMPL_OFFSET>,
            GetCurrentTime: GetCurrentTime::<Impl, IMPL_OFFSET>,
            SetCurrentTime: SetCurrentTime::<Impl, IMPL_OFFSET>,
            GetStartTime: GetStartTime::<Impl, IMPL_OFFSET>,
            GetDuration: GetDuration::<Impl, IMPL_OFFSET>,
            IsPaused: IsPaused::<Impl, IMPL_OFFSET>,
            GetDefaultPlaybackRate: GetDefaultPlaybackRate::<Impl, IMPL_OFFSET>,
            SetDefaultPlaybackRate: SetDefaultPlaybackRate::<Impl, IMPL_OFFSET>,
            GetPlaybackRate: GetPlaybackRate::<Impl, IMPL_OFFSET>,
            SetPlaybackRate: SetPlaybackRate::<Impl, IMPL_OFFSET>,
            GetPlayed: GetPlayed::<Impl, IMPL_OFFSET>,
            GetSeekable: GetSeekable::<Impl, IMPL_OFFSET>,
            IsEnded: IsEnded::<Impl, IMPL_OFFSET>,
            GetAutoPlay: GetAutoPlay::<Impl, IMPL_OFFSET>,
            SetAutoPlay: SetAutoPlay::<Impl, IMPL_OFFSET>,
            GetLoop: GetLoop::<Impl, IMPL_OFFSET>,
            SetLoop: SetLoop::<Impl, IMPL_OFFSET>,
            Play: Play::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            GetMuted: GetMuted::<Impl, IMPL_OFFSET>,
            SetMuted: SetMuted::<Impl, IMPL_OFFSET>,
            GetVolume: GetVolume::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            HasVideo: HasVideo::<Impl, IMPL_OFFSET>,
            HasAudio: HasAudio::<Impl, IMPL_OFFSET>,
            GetNativeVideoSize: GetNativeVideoSize::<Impl, IMPL_OFFSET>,
            GetVideoAspectRatio: GetVideoAspectRatio::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
            TransferVideoFrame: TransferVideoFrame::<Impl, IMPL_OFFSET>,
            OnVideoStreamTick: OnVideoStreamTick::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineAudioEndpointIdImpl: Sized {
    fn SetAudioEndpointId();
    fn GetAudioEndpointId();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineAudioEndpointIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineAudioEndpointIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineAudioEndpointIdVtbl {
        unsafe extern "system" fn SetAudioEndpointId<Impl: IMFMediaEngineAudioEndpointIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszendpointid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAudioEndpointId<Impl: IMFMediaEngineAudioEndpointIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszendpointid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAudioEndpointId: SetAudioEndpointId::<Impl, IMPL_OFFSET>,
            GetAudioEndpointId: GetAudioEndpointId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineAudioEndpointId as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaEngineClassFactoryImpl: Sized {
    fn CreateInstance();
    fn CreateTimeRange();
    fn CreateError();
}
impl IMFMediaEngineClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineClassFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMFMediaEngineClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pattr: ::windows::core::RawPtr, ppplayer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTimeRange<Impl: IMFMediaEngineClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptimerange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateError<Impl: IMFMediaEngineClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateTimeRange: CreateTimeRange::<Impl, IMPL_OFFSET>,
            CreateError: CreateError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineClassFactory2Impl: Sized {
    fn CreateMediaKeys2();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineClassFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineClassFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineClassFactory2Vtbl {
        unsafe extern "system" fn CreateMediaKeys2<Impl: IMFMediaEngineClassFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keysystem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, defaultcdmstorepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, inprivatecdmstorepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateMediaKeys2: CreateMediaKeys2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMFMediaEngineClassFactory3Impl: Sized {
    fn CreateMediaKeySystemAccess();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMFMediaEngineClassFactory3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineClassFactory3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineClassFactory3Vtbl {
        unsafe extern "system" fn CreateMediaKeySystemAccess<Impl: IMFMediaEngineClassFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keysystem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsupportedconfigurationsarray: *const ::windows::core::RawPtr, usize: u32, ppkeyaccess: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateMediaKeySystemAccess: CreateMediaKeySystemAccess::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactory3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineClassFactory4Impl: Sized {
    fn CreateContentDecryptionModuleFactory();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineClassFactory4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineClassFactory4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineClassFactory4Vtbl {
        unsafe extern "system" fn CreateContentDecryptionModuleFactory<Impl: IMFMediaEngineClassFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keysystem: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateContentDecryptionModuleFactory: CreateContentDecryptionModuleFactory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactory4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineClassFactoryExImpl: Sized + IMFMediaEngineClassFactoryImpl {
    fn CreateMediaSourceExtension();
    fn CreateMediaKeys();
    fn IsTypeSupported();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineClassFactoryExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineClassFactoryExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineClassFactoryExVtbl {
        unsafe extern "system" fn CreateMediaSourceExtension<Impl: IMFMediaEngineClassFactoryExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pattr: ::windows::core::RawPtr, ppmse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMediaKeys<Impl: IMFMediaEngineClassFactoryExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keysystem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cdmstorepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTypeSupported<Impl: IMFMediaEngineClassFactoryExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keysystem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, issupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaEngineClassFactoryVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateMediaSourceExtension: CreateMediaSourceExtension::<Impl, IMPL_OFFSET>,
            CreateMediaKeys: CreateMediaKeys::<Impl, IMPL_OFFSET>,
            IsTypeSupported: IsTypeSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactoryEx as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaEngineEMEImpl: Sized {
    fn Keys();
    fn SetMediaKeys();
}
impl IMFMediaEngineEMEVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineEMEImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineEMEVtbl {
        unsafe extern "system" fn Keys<Impl: IMFMediaEngineEMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMediaKeys<Impl: IMFMediaEngineEMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keys: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Keys: Keys::<Impl, IMPL_OFFSET>,
            SetMediaKeys: SetMediaKeys::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineEME as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineEMENotifyImpl: Sized {
    fn Encrypted();
    fn WaitingForKey();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineEMENotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineEMENotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineEMENotifyVtbl {
        unsafe extern "system" fn Encrypted<Impl: IMFMediaEngineEMENotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbinitdata: *const u8, cb: u32, bstrinitdatatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitingForKey<Impl: IMFMediaEngineEMENotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Encrypted: Encrypted::<Impl, IMPL_OFFSET>,
            WaitingForKey: WaitingForKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineEMENotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaEngineExImpl: Sized + IMFMediaEngineImpl {
    fn SetSourceFromByteStream();
    fn GetStatistics();
    fn UpdateVideoStream();
    fn GetBalance();
    fn SetBalance();
    fn IsPlaybackRateSupported();
    fn FrameStep();
    fn GetResourceCharacteristics();
    fn GetPresentationAttribute();
    fn GetNumberOfStreams();
    fn GetStreamAttribute();
    fn GetStreamSelection();
    fn SetStreamSelection();
    fn ApplyStreamSelections();
    fn IsProtected();
    fn InsertVideoEffect();
    fn InsertAudioEffect();
    fn RemoveAllEffects();
    fn SetTimelineMarkerTimer();
    fn GetTimelineMarkerTimer();
    fn CancelTimelineMarkerTimer();
    fn IsStereo3D();
    fn GetStereo3DFramePackingMode();
    fn SetStereo3DFramePackingMode();
    fn GetStereo3DRenderMode();
    fn SetStereo3DRenderMode();
    fn EnableWindowlessSwapchainMode();
    fn GetVideoSwapchainHandle();
    fn EnableHorizontalMirrorMode();
    fn GetAudioStreamCategory();
    fn SetAudioStreamCategory();
    fn GetAudioEndpointRole();
    fn SetAudioEndpointRole();
    fn GetRealTimeMode();
    fn SetRealTimeMode();
    fn SetCurrentTimeEx();
    fn EnableTimeUpdateTimer();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaEngineExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineExVtbl {
        unsafe extern "system" fn SetSourceFromByteStream<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytestream: ::windows::core::RawPtr, purl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatistics<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statisticid: MF_MEDIA_ENGINE_STATISTIC, pstatistic: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateVideoStream<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *const MFVideoNormalizedRect, pdst: *const super::super::Foundation::RECT, pborderclr: *const MFARGB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBalance<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBalance<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, balance: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPlaybackRateSupported<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: f64) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FrameStep<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forward: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceCharacteristics<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentationAttribute<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmfattribute: *const ::windows::core::GUID, pvvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumberOfStreams<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamAttribute<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, guidmfattribute: *const ::windows::core::GUID, pvvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamSelection<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, penabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamSelection<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, enabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyStreamSelections<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsProtected<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertVideoEffect<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, foptional: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAudioEffect<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, foptional: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllEffects<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTimelineMarkerTimer<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timetofire: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimelineMarkerTimer<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimetofire: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelTimelineMarkerTimer<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsStereo3D<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStereo3DFramePackingMode<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packmode: *mut MF_MEDIA_ENGINE_S3D_PACKING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStereo3DFramePackingMode<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packmode: MF_MEDIA_ENGINE_S3D_PACKING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStereo3DRenderMode<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputtype: *mut MF3DVideoOutputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStereo3DRenderMode<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputtype: MF3DVideoOutputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableWindowlessSwapchainMode<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoSwapchainHandle<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phswapchain: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableHorizontalMirrorMode<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAudioStreamCategory<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategory: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAudioStreamCategory<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAudioEndpointRole<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAudioEndpointRole<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, role: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRealTimeMode<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRealTimeMode<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentTimeEx<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seektime: f64, seekmode: MF_MEDIA_ENGINE_SEEK_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableTimeUpdateTimer<Impl: IMFMediaEngineExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabletimer: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaEngineVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSourceFromByteStream: SetSourceFromByteStream::<Impl, IMPL_OFFSET>,
            GetStatistics: GetStatistics::<Impl, IMPL_OFFSET>,
            UpdateVideoStream: UpdateVideoStream::<Impl, IMPL_OFFSET>,
            GetBalance: GetBalance::<Impl, IMPL_OFFSET>,
            SetBalance: SetBalance::<Impl, IMPL_OFFSET>,
            IsPlaybackRateSupported: IsPlaybackRateSupported::<Impl, IMPL_OFFSET>,
            FrameStep: FrameStep::<Impl, IMPL_OFFSET>,
            GetResourceCharacteristics: GetResourceCharacteristics::<Impl, IMPL_OFFSET>,
            GetPresentationAttribute: GetPresentationAttribute::<Impl, IMPL_OFFSET>,
            GetNumberOfStreams: GetNumberOfStreams::<Impl, IMPL_OFFSET>,
            GetStreamAttribute: GetStreamAttribute::<Impl, IMPL_OFFSET>,
            GetStreamSelection: GetStreamSelection::<Impl, IMPL_OFFSET>,
            SetStreamSelection: SetStreamSelection::<Impl, IMPL_OFFSET>,
            ApplyStreamSelections: ApplyStreamSelections::<Impl, IMPL_OFFSET>,
            IsProtected: IsProtected::<Impl, IMPL_OFFSET>,
            InsertVideoEffect: InsertVideoEffect::<Impl, IMPL_OFFSET>,
            InsertAudioEffect: InsertAudioEffect::<Impl, IMPL_OFFSET>,
            RemoveAllEffects: RemoveAllEffects::<Impl, IMPL_OFFSET>,
            SetTimelineMarkerTimer: SetTimelineMarkerTimer::<Impl, IMPL_OFFSET>,
            GetTimelineMarkerTimer: GetTimelineMarkerTimer::<Impl, IMPL_OFFSET>,
            CancelTimelineMarkerTimer: CancelTimelineMarkerTimer::<Impl, IMPL_OFFSET>,
            IsStereo3D: IsStereo3D::<Impl, IMPL_OFFSET>,
            GetStereo3DFramePackingMode: GetStereo3DFramePackingMode::<Impl, IMPL_OFFSET>,
            SetStereo3DFramePackingMode: SetStereo3DFramePackingMode::<Impl, IMPL_OFFSET>,
            GetStereo3DRenderMode: GetStereo3DRenderMode::<Impl, IMPL_OFFSET>,
            SetStereo3DRenderMode: SetStereo3DRenderMode::<Impl, IMPL_OFFSET>,
            EnableWindowlessSwapchainMode: EnableWindowlessSwapchainMode::<Impl, IMPL_OFFSET>,
            GetVideoSwapchainHandle: GetVideoSwapchainHandle::<Impl, IMPL_OFFSET>,
            EnableHorizontalMirrorMode: EnableHorizontalMirrorMode::<Impl, IMPL_OFFSET>,
            GetAudioStreamCategory: GetAudioStreamCategory::<Impl, IMPL_OFFSET>,
            SetAudioStreamCategory: SetAudioStreamCategory::<Impl, IMPL_OFFSET>,
            GetAudioEndpointRole: GetAudioEndpointRole::<Impl, IMPL_OFFSET>,
            SetAudioEndpointRole: SetAudioEndpointRole::<Impl, IMPL_OFFSET>,
            GetRealTimeMode: GetRealTimeMode::<Impl, IMPL_OFFSET>,
            SetRealTimeMode: SetRealTimeMode::<Impl, IMPL_OFFSET>,
            SetCurrentTimeEx: SetCurrentTimeEx::<Impl, IMPL_OFFSET>,
            EnableTimeUpdateTimer: EnableTimeUpdateTimer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineExtensionImpl: Sized {
    fn CanPlayType();
    fn BeginCreateObject();
    fn CancelObjectCreation();
    fn EndCreateObject();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineExtensionVtbl {
        unsafe extern "system" fn CanPlayType<Impl: IMFMediaEngineExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioonly: super::super::Foundation::BOOL, mimetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, panswer: *mut MF_MEDIA_ENGINE_CANPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginCreateObject<Impl: IMFMediaEngineExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbytestream: ::windows::core::RawPtr, r#type: MF_OBJECT_TYPE, ppiunknowncancelcookie: *mut *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelObjectCreation<Impl: IMFMediaEngineExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piunknowncancelcookie: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndCreateObject<Impl: IMFMediaEngineExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CanPlayType: CanPlayType::<Impl, IMPL_OFFSET>,
            BeginCreateObject: BeginCreateObject::<Impl, IMPL_OFFSET>,
            CancelObjectCreation: CancelObjectCreation::<Impl, IMPL_OFFSET>,
            EndCreateObject: EndCreateObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineExtension as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaEngineNeedKeyNotifyImpl: Sized {
    fn NeedKey();
}
impl IMFMediaEngineNeedKeyNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineNeedKeyNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineNeedKeyNotifyVtbl {
        unsafe extern "system" fn NeedKey<Impl: IMFMediaEngineNeedKeyNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initdata: *const u8, cb: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), NeedKey: NeedKey::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineNeedKeyNotify as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaEngineNotifyImpl: Sized {
    fn EventNotify();
}
impl IMFMediaEngineNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineNotifyVtbl {
        unsafe extern "system" fn EventNotify<Impl: IMFMediaEngineNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: u32, param1: usize, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), EventNotify: EventNotify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineOPMInfoImpl: Sized {
    fn GetOPMInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineOPMInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineOPMInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineOPMInfoVtbl {
        unsafe extern "system" fn GetOPMInfo<Impl: IMFMediaEngineOPMInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut MF_MEDIA_ENGINE_OPM_STATUS, pconstricted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetOPMInfo: GetOPMInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineOPMInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineProtectedContentImpl: Sized {
    fn ShareResources();
    fn GetRequiredProtections();
    fn SetOPMWindow();
    fn TransferVideoFrame();
    fn SetContentProtectionManager();
    fn SetApplicationCertificate();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineProtectedContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineProtectedContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineProtectedContentVtbl {
        unsafe extern "system" fn ShareResources<Impl: IMFMediaEngineProtectedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdevicecontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequiredProtections<Impl: IMFMediaEngineProtectedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframeprotectionflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOPMWindow<Impl: IMFMediaEngineProtectedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransferVideoFrame<Impl: IMFMediaEngineProtectedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstsurf: *mut ::core::ffi::c_void, psrc: *const MFVideoNormalizedRect, pdst: *const super::super::Foundation::RECT, pborderclr: *const MFARGB, pframeprotectionflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContentProtectionManager<Impl: IMFMediaEngineProtectedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcpm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetApplicationCertificate<Impl: IMFMediaEngineProtectedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbblob: *const u8, cbblob: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ShareResources: ShareResources::<Impl, IMPL_OFFSET>,
            GetRequiredProtections: GetRequiredProtections::<Impl, IMPL_OFFSET>,
            SetOPMWindow: SetOPMWindow::<Impl, IMPL_OFFSET>,
            TransferVideoFrame: TransferVideoFrame::<Impl, IMPL_OFFSET>,
            SetContentProtectionManager: SetContentProtectionManager::<Impl, IMPL_OFFSET>,
            SetApplicationCertificate: SetApplicationCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineProtectedContent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineSrcElementsImpl: Sized {
    fn GetLength();
    fn GetURL();
    fn GetType();
    fn GetMedia();
    fn AddElement();
    fn RemoveAllElements();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineSrcElementsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineSrcElementsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineSrcElementsVtbl {
        unsafe extern "system" fn GetLength<Impl: IMFMediaEngineSrcElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetURL<Impl: IMFMediaEngineSrcElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, purl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IMFMediaEngineSrcElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ptype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMedia<Impl: IMFMediaEngineSrcElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pmedia: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddElement<Impl: IMFMediaEngineSrcElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ptype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pmedia: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllElements<Impl: IMFMediaEngineSrcElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLength: GetLength::<Impl, IMPL_OFFSET>,
            GetURL: GetURL::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetMedia: GetMedia::<Impl, IMPL_OFFSET>,
            AddElement: AddElement::<Impl, IMPL_OFFSET>,
            RemoveAllElements: RemoveAllElements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineSrcElements as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineSrcElementsExImpl: Sized + IMFMediaEngineSrcElementsImpl {
    fn AddElementEx();
    fn GetKeySystem();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineSrcElementsExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineSrcElementsExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineSrcElementsExVtbl {
        unsafe extern "system" fn AddElementEx<Impl: IMFMediaEngineSrcElementsExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ptype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pmedia: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keysystem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeySystem<Impl: IMFMediaEngineSrcElementsExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ptype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaEngineSrcElementsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddElementEx: AddElementEx::<Impl, IMPL_OFFSET>,
            GetKeySystem: GetKeySystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineSrcElementsEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineSupportsSourceTransferImpl: Sized {
    fn ShouldTransferSource();
    fn DetachMediaSource();
    fn AttachMediaSource();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineSupportsSourceTransferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineSupportsSourceTransferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineSupportsSourceTransferVtbl {
        unsafe extern "system" fn ShouldTransferSource<Impl: IMFMediaEngineSupportsSourceTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfshouldtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DetachMediaSource<Impl: IMFMediaEngineSupportsSourceTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbytestream: *mut ::windows::core::RawPtr, ppmediasource: *mut ::windows::core::RawPtr, ppmse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachMediaSource<Impl: IMFMediaEngineSupportsSourceTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytestream: ::windows::core::RawPtr, pmediasource: ::windows::core::RawPtr, pmse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ShouldTransferSource: ShouldTransferSource::<Impl, IMPL_OFFSET>,
            DetachMediaSource: DetachMediaSource::<Impl, IMPL_OFFSET>,
            AttachMediaSource: AttachMediaSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineSupportsSourceTransfer as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaEngineTransferSourceImpl: Sized {
    fn TransferSourceToMediaEngine();
}
impl IMFMediaEngineTransferSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineTransferSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineTransferSourceVtbl {
        unsafe extern "system" fn TransferSourceToMediaEngine<Impl: IMFMediaEngineTransferSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TransferSourceToMediaEngine: TransferSourceToMediaEngine::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineTransferSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaEngineWebSupportImpl: Sized {
    fn ShouldDelayTheLoadEvent();
    fn ConnectWebAudio();
    fn DisconnectWebAudio();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaEngineWebSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEngineWebSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEngineWebSupportVtbl {
        unsafe extern "system" fn ShouldDelayTheLoadEvent<Impl: IMFMediaEngineWebSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectWebAudio<Impl: IMFMediaEngineWebSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsamplerate: u32, ppsourceprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectWebAudio<Impl: IMFMediaEngineWebSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ShouldDelayTheLoadEvent: ShouldDelayTheLoadEvent::<Impl, IMPL_OFFSET>,
            ConnectWebAudio: ConnectWebAudio::<Impl, IMPL_OFFSET>,
            DisconnectWebAudio: DisconnectWebAudio::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEngineWebSupport as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaErrorImpl: Sized {
    fn GetErrorCode();
    fn GetExtendedErrorCode();
    fn SetErrorCode();
    fn SetExtendedErrorCode();
}
impl IMFMediaErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaErrorVtbl {
        unsafe extern "system" fn GetErrorCode<Impl: IMFMediaErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u16 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendedErrorCode<Impl: IMFMediaErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetErrorCode<Impl: IMFMediaErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: MF_MEDIA_ENGINE_ERR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExtendedErrorCode<Impl: IMFMediaErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetErrorCode: GetErrorCode::<Impl, IMPL_OFFSET>,
            GetExtendedErrorCode: GetExtendedErrorCode::<Impl, IMPL_OFFSET>,
            SetErrorCode: SetErrorCode::<Impl, IMPL_OFFSET>,
            SetExtendedErrorCode: SetExtendedErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaEventImpl: Sized + IMFAttributesImpl {
    fn GetType();
    fn GetExtendedType();
    fn GetStatus();
    fn GetValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEventVtbl {
        unsafe extern "system" fn GetType<Impl: IMFMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmet: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendedType<Impl: IMFMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidextendedtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IMFMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IMFMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetExtendedType: GetExtendedType::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaEventGeneratorImpl: Sized {
    fn GetEvent();
    fn BeginGetEvent();
    fn EndGetEvent();
    fn QueueEvent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaEventGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEventGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEventGeneratorVtbl {
        unsafe extern "system" fn GetEvent<Impl: IMFMediaEventGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS, ppevent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginGetEvent<Impl: IMFMediaEventGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndGetEvent<Impl: IMFMediaEventGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, ppevent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueEvent<Impl: IMFMediaEventGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, met: u32, guidextendedtype: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT, pvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEvent: GetEvent::<Impl, IMPL_OFFSET>,
            BeginGetEvent: BeginGetEvent::<Impl, IMPL_OFFSET>,
            EndGetEvent: EndGetEvent::<Impl, IMPL_OFFSET>,
            QueueEvent: QueueEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEventGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaEventQueueImpl: Sized {
    fn GetEvent();
    fn BeginGetEvent();
    fn EndGetEvent();
    fn QueueEvent();
    fn QueueEventParamVar();
    fn QueueEventParamUnk();
    fn Shutdown();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaEventQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaEventQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaEventQueueVtbl {
        unsafe extern "system" fn GetEvent<Impl: IMFMediaEventQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppevent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginGetEvent<Impl: IMFMediaEventQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndGetEvent<Impl: IMFMediaEventQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, ppevent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueEvent<Impl: IMFMediaEventQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueEventParamVar<Impl: IMFMediaEventQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, met: u32, guidextendedtype: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT, pvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueEventParamUnk<Impl: IMFMediaEventQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, met: u32, guidextendedtype: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFMediaEventQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEvent: GetEvent::<Impl, IMPL_OFFSET>,
            BeginGetEvent: BeginGetEvent::<Impl, IMPL_OFFSET>,
            EndGetEvent: EndGetEvent::<Impl, IMPL_OFFSET>,
            QueueEvent: QueueEvent::<Impl, IMPL_OFFSET>,
            QueueEventParamVar: QueueEventParamVar::<Impl, IMPL_OFFSET>,
            QueueEventParamUnk: QueueEventParamUnk::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaEventQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaKeySessionImpl: Sized {
    fn GetError();
    fn KeySystem();
    fn SessionId();
    fn Update();
    fn Close();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaKeySessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaKeySessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaKeySessionVtbl {
        unsafe extern "system" fn GetError<Impl: IMFMediaKeySessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: *mut u16, systemcode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeySystem<Impl: IMFMediaKeySessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keysystem: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionId<Impl: IMFMediaKeySessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IMFMediaKeySessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const u8, cb: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMFMediaKeySessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetError: GetError::<Impl, IMPL_OFFSET>,
            KeySystem: KeySystem::<Impl, IMPL_OFFSET>,
            SessionId: SessionId::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaKeySession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaKeySession2Impl: Sized + IMFMediaKeySessionImpl {
    fn KeyStatuses();
    fn Load();
    fn GenerateRequest();
    fn Expiration();
    fn Remove();
    fn Shutdown();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaKeySession2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaKeySession2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaKeySession2Vtbl {
        unsafe extern "system" fn KeyStatuses<Impl: IMFMediaKeySession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeystatusesarray: *mut *mut MFMediaKeyStatus, pusize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IMFMediaKeySession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsessionid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateRequest<Impl: IMFMediaKeySession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initdatatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbinitdata: *const u8, cb: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Expiration<Impl: IMFMediaKeySession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dblexpiration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IMFMediaKeySession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFMediaKeySession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaKeySessionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            KeyStatuses: KeyStatuses::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            GenerateRequest: GenerateRequest::<Impl, IMPL_OFFSET>,
            Expiration: Expiration::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaKeySession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaKeySessionNotifyImpl: Sized {
    fn KeyMessage();
    fn KeyAdded();
    fn KeyError();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaKeySessionNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaKeySessionNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaKeySessionNotifyVtbl {
        unsafe extern "system" fn KeyMessage<Impl: IMFMediaKeySessionNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, message: *const u8, cb: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyAdded<Impl: IMFMediaKeySessionNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyError<Impl: IMFMediaKeySessionNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: u16, systemcode: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            KeyMessage: KeyMessage::<Impl, IMPL_OFFSET>,
            KeyAdded: KeyAdded::<Impl, IMPL_OFFSET>,
            KeyError: KeyError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaKeySessionNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaKeySessionNotify2Impl: Sized + IMFMediaKeySessionNotifyImpl {
    fn KeyMessage2();
    fn KeyStatusChange();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaKeySessionNotify2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaKeySessionNotify2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaKeySessionNotify2Vtbl {
        unsafe extern "system" fn KeyMessage2<Impl: IMFMediaKeySessionNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emessagetype: MF_MEDIAKEYSESSION_MESSAGETYPE, destinationurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbmessage: *const u8, cbmessage: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyStatusChange<Impl: IMFMediaKeySessionNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaKeySessionNotifyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            KeyMessage2: KeyMessage2::<Impl, IMPL_OFFSET>,
            KeyStatusChange: KeyStatusChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaKeySessionNotify2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMFMediaKeySystemAccessImpl: Sized {
    fn CreateMediaKeys();
    fn SupportedConfiguration();
    fn KeySystem();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMFMediaKeySystemAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaKeySystemAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaKeySystemAccessVtbl {
        unsafe extern "system" fn CreateMediaKeys<Impl: IMFMediaKeySystemAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdmcustomconfig: ::windows::core::RawPtr, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedConfiguration<Impl: IMFMediaKeySystemAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsupportedconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeySystem<Impl: IMFMediaKeySystemAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeysystem: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateMediaKeys: CreateMediaKeys::<Impl, IMPL_OFFSET>,
            SupportedConfiguration: SupportedConfiguration::<Impl, IMPL_OFFSET>,
            KeySystem: KeySystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaKeySystemAccess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaKeysImpl: Sized {
    fn CreateSession();
    fn KeySystem();
    fn Shutdown();
    fn GetSuspendNotify();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaKeysVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaKeysImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaKeysVtbl {
        unsafe extern "system" fn CreateSession<Impl: IMFMediaKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initdata: *const u8, cb: u32, customdata: *const u8, cbcustomdata: u32, notify: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeySystem<Impl: IMFMediaKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keysystem: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFMediaKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSuspendNotify<Impl: IMFMediaKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateSession: CreateSession::<Impl, IMPL_OFFSET>,
            KeySystem: KeySystem::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
            GetSuspendNotify: GetSuspendNotify::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaKeys as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaKeys2Impl: Sized + IMFMediaKeysImpl {
    fn CreateSession2();
    fn SetServerCertificate();
    fn GetDOMException();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaKeys2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaKeys2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaKeys2Vtbl {
        unsafe extern "system" fn CreateSession2<Impl: IMFMediaKeys2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, esessiontype: MF_MEDIAKEYSESSION_TYPE, pmfmediakeysessionnotify2: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServerCertificate<Impl: IMFMediaKeys2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbservercertificate: *const u8, cb: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDOMException<Impl: IMFMediaKeys2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemcode: ::windows::core::HRESULT, code: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaKeysVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateSession2: CreateSession2::<Impl, IMPL_OFFSET>,
            SetServerCertificate: SetServerCertificate::<Impl, IMPL_OFFSET>,
            GetDOMException: GetDOMException::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaKeys2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaSessionImpl: Sized + IMFMediaEventGeneratorImpl {
    fn SetTopology();
    fn ClearTopologies();
    fn Start();
    fn Pause();
    fn Stop();
    fn Close();
    fn Shutdown();
    fn GetClock();
    fn GetSessionCapabilities();
    fn GetFullTopology();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSessionVtbl {
        unsafe extern "system" fn SetTopology<Impl: IMFMediaSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsettopologyflags: u32, ptopology: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearTopologies<Impl: IMFMediaSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Start<Impl: IMFMediaSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtimeformat: *const ::windows::core::GUID, pvarstartposition: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IMFMediaSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IMFMediaSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMFMediaSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFMediaSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClock<Impl: IMFMediaSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSessionCapabilities<Impl: IMFMediaSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcaps: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFullTopology<Impl: IMFMediaSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwgetfulltopologyflags: u32, topoid: u64, ppfulltopology: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaEventGeneratorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetTopology: SetTopology::<Impl, IMPL_OFFSET>,
            ClearTopologies: ClearTopologies::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
            GetClock: GetClock::<Impl, IMPL_OFFSET>,
            GetSessionCapabilities: GetSessionCapabilities::<Impl, IMPL_OFFSET>,
            GetFullTopology: GetFullTopology::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaSharingEngineImpl: Sized + IMFMediaEngineImpl {
    fn GetDevice();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaSharingEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSharingEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSharingEngineVtbl {
        unsafe extern "system" fn GetDevice<Impl: IMFMediaSharingEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut DEVICE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IMFMediaEngineVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDevice: GetDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSharingEngine as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaSharingEngineClassFactoryImpl: Sized {
    fn CreateInstance();
}
impl IMFMediaSharingEngineClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSharingEngineClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSharingEngineClassFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMFMediaSharingEngineClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pattr: ::windows::core::RawPtr, ppengine: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSharingEngineClassFactory as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaSinkImpl: Sized {
    fn GetCharacteristics();
    fn AddStreamSink();
    fn RemoveStreamSink();
    fn GetStreamSinkCount();
    fn GetStreamSinkByIndex();
    fn GetStreamSinkById();
    fn SetPresentationClock();
    fn GetPresentationClock();
    fn Shutdown();
}
impl IMFMediaSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSinkVtbl {
        unsafe extern "system" fn GetCharacteristics<Impl: IMFMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStreamSink<Impl: IMFMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamsinkidentifier: u32, pmediatype: ::windows::core::RawPtr, ppstreamsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStreamSink<Impl: IMFMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamsinkidentifier: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamSinkCount<Impl: IMFMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstreamsinkcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamSinkByIndex<Impl: IMFMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppstreamsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamSinkById<Impl: IMFMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamsinkidentifier: u32, ppstreamsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPresentationClock<Impl: IMFMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentationClock<Impl: IMFMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppresentationclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFMediaSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCharacteristics: GetCharacteristics::<Impl, IMPL_OFFSET>,
            AddStreamSink: AddStreamSink::<Impl, IMPL_OFFSET>,
            RemoveStreamSink: RemoveStreamSink::<Impl, IMPL_OFFSET>,
            GetStreamSinkCount: GetStreamSinkCount::<Impl, IMPL_OFFSET>,
            GetStreamSinkByIndex: GetStreamSinkByIndex::<Impl, IMPL_OFFSET>,
            GetStreamSinkById: GetStreamSinkById::<Impl, IMPL_OFFSET>,
            SetPresentationClock: SetPresentationClock::<Impl, IMPL_OFFSET>,
            GetPresentationClock: GetPresentationClock::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSink as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaSinkPrerollImpl: Sized {
    fn NotifyPreroll();
}
impl IMFMediaSinkPrerollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSinkPrerollImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSinkPrerollVtbl {
        unsafe extern "system" fn NotifyPreroll<Impl: IMFMediaSinkPrerollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnsupcomingstarttime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), NotifyPreroll: NotifyPreroll::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSinkPreroll as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaSourceImpl: Sized + IMFMediaEventGeneratorImpl {
    fn GetCharacteristics();
    fn CreatePresentationDescriptor();
    fn Start();
    fn Stop();
    fn Pause();
    fn Shutdown();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSourceVtbl {
        unsafe extern "system" fn GetCharacteristics<Impl: IMFMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePresentationDescriptor<Impl: IMFMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppresentationdescriptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Start<Impl: IMFMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationdescriptor: ::windows::core::RawPtr, pguidtimeformat: *const ::windows::core::GUID, pvarstartposition: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IMFMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IMFMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaEventGeneratorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCharacteristics: GetCharacteristics::<Impl, IMPL_OFFSET>,
            CreatePresentationDescriptor: CreatePresentationDescriptor::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaSource2Impl: Sized + IMFMediaEventGeneratorImpl + IMFMediaSourceImpl + IMFMediaSourceExImpl {
    fn SetMediaType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSource2Vtbl {
        unsafe extern "system" fn SetMediaType<Impl: IMFMediaSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IMFMediaSourceExVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetMediaType: SetMediaType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaSourceExImpl: Sized + IMFMediaEventGeneratorImpl + IMFMediaSourceImpl {
    fn GetSourceAttributes();
    fn GetStreamAttributes();
    fn SetD3DManager();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaSourceExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSourceExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSourceExVtbl {
        unsafe extern "system" fn GetSourceAttributes<Impl: IMFMediaSourceExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamAttributes<Impl: IMFMediaSourceExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamidentifier: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetD3DManager<Impl: IMFMediaSourceExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaSourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSourceAttributes: GetSourceAttributes::<Impl, IMPL_OFFSET>,
            GetStreamAttributes: GetStreamAttributes::<Impl, IMPL_OFFSET>,
            SetD3DManager: SetD3DManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSourceEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaSourceExtensionImpl: Sized {
    fn GetSourceBuffers();
    fn GetActiveSourceBuffers();
    fn GetReadyState();
    fn GetDuration();
    fn SetDuration();
    fn AddSourceBuffer();
    fn RemoveSourceBuffer();
    fn SetEndOfStream();
    fn IsTypeSupported();
    fn GetSourceBuffer();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaSourceExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSourceExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSourceExtensionVtbl {
        unsafe extern "system" fn GetSourceBuffers<Impl: IMFMediaSourceExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<IMFSourceBufferList> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActiveSourceBuffers<Impl: IMFMediaSourceExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<IMFSourceBufferList> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReadyState<Impl: IMFMediaSourceExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> MF_MSE_READY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuration<Impl: IMFMediaSourceExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDuration<Impl: IMFMediaSourceExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSourceBuffer<Impl: IMFMediaSourceExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pnotify: ::windows::core::RawPtr, ppsourcebuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveSourceBuffer<Impl: IMFMediaSourceExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcebuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEndOfStream<Impl: IMFMediaSourceExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: MF_MSE_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTypeSupported<Impl: IMFMediaSourceExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceBuffer<Impl: IMFMediaSourceExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32) -> ::core::option::Option<IMFSourceBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSourceBuffers: GetSourceBuffers::<Impl, IMPL_OFFSET>,
            GetActiveSourceBuffers: GetActiveSourceBuffers::<Impl, IMPL_OFFSET>,
            GetReadyState: GetReadyState::<Impl, IMPL_OFFSET>,
            GetDuration: GetDuration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            AddSourceBuffer: AddSourceBuffer::<Impl, IMPL_OFFSET>,
            RemoveSourceBuffer: RemoveSourceBuffer::<Impl, IMPL_OFFSET>,
            SetEndOfStream: SetEndOfStream::<Impl, IMPL_OFFSET>,
            IsTypeSupported: IsTypeSupported::<Impl, IMPL_OFFSET>,
            GetSourceBuffer: GetSourceBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSourceExtension as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaSourceExtensionLiveSeekableRangeImpl: Sized {
    fn SetLiveSeekableRange();
    fn ClearLiveSeekableRange();
}
impl IMFMediaSourceExtensionLiveSeekableRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSourceExtensionLiveSeekableRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSourceExtensionLiveSeekableRangeVtbl {
        unsafe extern "system" fn SetLiveSeekableRange<Impl: IMFMediaSourceExtensionLiveSeekableRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: f64, end: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearLiveSeekableRange<Impl: IMFMediaSourceExtensionLiveSeekableRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetLiveSeekableRange: SetLiveSeekableRange::<Impl, IMPL_OFFSET>,
            ClearLiveSeekableRange: ClearLiveSeekableRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSourceExtensionLiveSeekableRange as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaSourceExtensionNotifyImpl: Sized {
    fn OnSourceOpen();
    fn OnSourceEnded();
    fn OnSourceClose();
}
impl IMFMediaSourceExtensionNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSourceExtensionNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSourceExtensionNotifyVtbl {
        unsafe extern "system" fn OnSourceOpen<Impl: IMFMediaSourceExtensionNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSourceEnded<Impl: IMFMediaSourceExtensionNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSourceClose<Impl: IMFMediaSourceExtensionNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnSourceOpen: OnSourceOpen::<Impl, IMPL_OFFSET>,
            OnSourceEnded: OnSourceEnded::<Impl, IMPL_OFFSET>,
            OnSourceClose: OnSourceClose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSourceExtensionNotify as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaSourcePresentationProviderImpl: Sized {
    fn ForceEndOfPresentation();
}
impl IMFMediaSourcePresentationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSourcePresentationProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSourcePresentationProviderVtbl {
        unsafe extern "system" fn ForceEndOfPresentation<Impl: IMFMediaSourcePresentationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationdescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ForceEndOfPresentation: ForceEndOfPresentation::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSourcePresentationProvider as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaSourceTopologyProviderImpl: Sized {
    fn GetMediaSourceTopology();
}
impl IMFMediaSourceTopologyProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaSourceTopologyProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaSourceTopologyProviderVtbl {
        unsafe extern "system" fn GetMediaSourceTopology<Impl: IMFMediaSourceTopologyProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationdescriptor: ::windows::core::RawPtr, pptopology: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetMediaSourceTopology: GetMediaSourceTopology::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaSourceTopologyProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaStreamImpl: Sized + IMFMediaEventGeneratorImpl {
    fn GetMediaSource();
    fn GetStreamDescriptor();
    fn RequestSample();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaStreamVtbl {
        unsafe extern "system" fn GetMediaSource<Impl: IMFMediaStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediasource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamDescriptor<Impl: IMFMediaStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstreamdescriptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestSample<Impl: IMFMediaStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaEventGeneratorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMediaSource: GetMediaSource::<Impl, IMPL_OFFSET>,
            GetStreamDescriptor: GetStreamDescriptor::<Impl, IMPL_OFFSET>,
            RequestSample: RequestSample::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaStream2Impl: Sized + IMFMediaEventGeneratorImpl + IMFMediaStreamImpl {
    fn SetStreamState();
    fn GetStreamState();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaStream2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaStream2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaStream2Vtbl {
        unsafe extern "system" fn SetStreamState<Impl: IMFMediaStream2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MF_STREAM_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamState<Impl: IMFMediaStream2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut MF_STREAM_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaStreamVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetStreamState: SetStreamState::<Impl, IMPL_OFFSET>,
            GetStreamState: GetStreamState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaStream2 as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaStreamSourceSampleRequestImpl: Sized {
    fn SetSample();
}
impl IMFMediaStreamSourceSampleRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaStreamSourceSampleRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaStreamSourceSampleRequestVtbl {
        unsafe extern "system" fn SetSample<Impl: IMFMediaStreamSourceSampleRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetSample: SetSample::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaStreamSourceSampleRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFMediaTimeRangeImpl: Sized {
    fn GetLength();
    fn GetStart();
    fn GetEnd();
    fn ContainsTime();
    fn AddRange();
    fn Clear();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFMediaTimeRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaTimeRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaTimeRangeVtbl {
        unsafe extern "system" fn GetLength<Impl: IMFMediaTimeRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStart<Impl: IMFMediaTimeRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnd<Impl: IMFMediaTimeRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainsTime<Impl: IMFMediaTimeRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRange<Impl: IMFMediaTimeRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64, endtime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IMFMediaTimeRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLength: GetLength::<Impl, IMPL_OFFSET>,
            GetStart: GetStart::<Impl, IMPL_OFFSET>,
            GetEnd: GetEnd::<Impl, IMPL_OFFSET>,
            ContainsTime: ContainsTime::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaTimeRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMediaTypeImpl: Sized + IMFAttributesImpl {
    fn GetMajorType();
    fn IsCompressedFormat();
    fn IsEqual();
    fn GetRepresentation();
    fn FreeRepresentation();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMediaTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaTypeVtbl {
        unsafe extern "system" fn GetMajorType<Impl: IMFMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmajortype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCompressedFormat<Impl: IMFMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual<Impl: IMFMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimediatype: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRepresentation<Impl: IMFMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidrepresentation: ::windows::core::GUID, ppvrepresentation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeRepresentation<Impl: IMFMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidrepresentation: ::windows::core::GUID, pvrepresentation: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMajorType: GetMajorType::<Impl, IMPL_OFFSET>,
            IsCompressedFormat: IsCompressedFormat::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            GetRepresentation: GetRepresentation::<Impl, IMPL_OFFSET>,
            FreeRepresentation: FreeRepresentation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaType as ::windows::core::Interface>::IID
    }
}
pub trait IMFMediaTypeHandlerImpl: Sized {
    fn IsMediaTypeSupported();
    fn GetMediaTypeCount();
    fn GetMediaTypeByIndex();
    fn SetCurrentMediaType();
    fn GetCurrentMediaType();
    fn GetMajorType();
}
impl IMFMediaTypeHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMediaTypeHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMediaTypeHandlerVtbl {
        unsafe extern "system" fn IsMediaTypeSupported<Impl: IMFMediaTypeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: ::windows::core::RawPtr, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMediaTypeCount<Impl: IMFMediaTypeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMediaTypeByIndex<Impl: IMFMediaTypeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pptype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentMediaType<Impl: IMFMediaTypeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentMediaType<Impl: IMFMediaTypeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMajorType<Impl: IMFMediaTypeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmajortype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsMediaTypeSupported: IsMediaTypeSupported::<Impl, IMPL_OFFSET>,
            GetMediaTypeCount: GetMediaTypeCount::<Impl, IMPL_OFFSET>,
            GetMediaTypeByIndex: GetMediaTypeByIndex::<Impl, IMPL_OFFSET>,
            SetCurrentMediaType: SetCurrentMediaType::<Impl, IMPL_OFFSET>,
            GetCurrentMediaType: GetCurrentMediaType::<Impl, IMPL_OFFSET>,
            GetMajorType: GetMajorType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMediaTypeHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFMetadataImpl: Sized {
    fn SetLanguage();
    fn GetLanguage();
    fn GetAllLanguages();
    fn SetProperty();
    fn GetProperty();
    fn DeleteProperty();
    fn GetAllPropertyNames();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMetadataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMetadataVtbl {
        unsafe extern "system" fn SetLanguage<Impl: IMFMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrfc1766: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLanguage<Impl: IMFMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszrfc1766: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllLanguages<Impl: IMFMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvlanguages: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IMFMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, ppvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IMFMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, ppvvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteProperty<Impl: IMFMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllPropertyNames<Impl: IMFMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvnames: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
            GetLanguage: GetLanguage::<Impl, IMPL_OFFSET>,
            GetAllLanguages: GetAllLanguages::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            DeleteProperty: DeleteProperty::<Impl, IMPL_OFFSET>,
            GetAllPropertyNames: GetAllPropertyNames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMetadata as ::windows::core::Interface>::IID
    }
}
pub trait IMFMetadataProviderImpl: Sized {
    fn GetMFMetadata();
}
impl IMFMetadataProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMetadataProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMetadataProviderVtbl {
        unsafe extern "system" fn GetMFMetadata<Impl: IMFMetadataProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationdescriptor: ::windows::core::RawPtr, dwstreamidentifier: u32, dwflags: u32, ppmfmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetMFMetadata: GetMFMetadata::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMetadataProvider as ::windows::core::Interface>::IID
    }
}
pub trait IMFMuxStreamAttributesManagerImpl: Sized {
    fn GetStreamCount();
    fn GetAttributes();
}
impl IMFMuxStreamAttributesManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMuxStreamAttributesManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMuxStreamAttributesManagerVtbl {
        unsafe extern "system" fn GetStreamCount<Impl: IMFMuxStreamAttributesManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmuxstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributes<Impl: IMFMuxStreamAttributesManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmuxstreamindex: u32, ppstreamattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreamCount: GetStreamCount::<Impl, IMPL_OFFSET>,
            GetAttributes: GetAttributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMuxStreamAttributesManager as ::windows::core::Interface>::IID
    }
}
pub trait IMFMuxStreamMediaTypeManagerImpl: Sized {
    fn GetStreamCount();
    fn GetMediaType();
    fn GetStreamConfigurationCount();
    fn AddStreamConfiguration();
    fn RemoveStreamConfiguration();
    fn GetStreamConfiguration();
}
impl IMFMuxStreamMediaTypeManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMuxStreamMediaTypeManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMuxStreamMediaTypeManagerVtbl {
        unsafe extern "system" fn GetStreamCount<Impl: IMFMuxStreamMediaTypeManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmuxstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMediaType<Impl: IMFMuxStreamMediaTypeManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmuxstreamindex: u32, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamConfigurationCount<Impl: IMFMuxStreamMediaTypeManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStreamConfiguration<Impl: IMFMuxStreamMediaTypeManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullstreammask: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStreamConfiguration<Impl: IMFMuxStreamMediaTypeManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullstreammask: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamConfiguration<Impl: IMFMuxStreamMediaTypeManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, pullstreammask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreamCount: GetStreamCount::<Impl, IMPL_OFFSET>,
            GetMediaType: GetMediaType::<Impl, IMPL_OFFSET>,
            GetStreamConfigurationCount: GetStreamConfigurationCount::<Impl, IMPL_OFFSET>,
            AddStreamConfiguration: AddStreamConfiguration::<Impl, IMPL_OFFSET>,
            RemoveStreamConfiguration: RemoveStreamConfiguration::<Impl, IMPL_OFFSET>,
            GetStreamConfiguration: GetStreamConfiguration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMuxStreamMediaTypeManager as ::windows::core::Interface>::IID
    }
}
pub trait IMFMuxStreamSampleManagerImpl: Sized {
    fn GetStreamCount();
    fn GetSample();
    fn GetStreamConfiguration();
}
impl IMFMuxStreamSampleManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFMuxStreamSampleManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFMuxStreamSampleManagerVtbl {
        unsafe extern "system" fn GetStreamCount<Impl: IMFMuxStreamSampleManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmuxstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSample<Impl: IMFMuxStreamSampleManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmuxstreamindex: u32, ppsample: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamConfiguration<Impl: IMFMuxStreamSampleManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreamCount: GetStreamCount::<Impl, IMPL_OFFSET>,
            GetSample: GetSample::<Impl, IMPL_OFFSET>,
            GetStreamConfiguration: GetStreamConfiguration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFMuxStreamSampleManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFNetCredentialImpl: Sized {
    fn SetUser();
    fn SetPassword();
    fn GetUser();
    fn GetPassword();
    fn LoggedOnUser();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFNetCredentialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFNetCredentialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFNetCredentialVtbl {
        unsafe extern "system" fn SetUser<Impl: IMFNetCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32, fdataisencrypted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPassword<Impl: IMFNetCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32, fdataisencrypted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUser<Impl: IMFNetCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *mut u8, pcbdata: *mut u32, fencryptdata: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPassword<Impl: IMFNetCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *mut u8, pcbdata: *mut u32, fencryptdata: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoggedOnUser<Impl: IMFNetCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfloggedonuser: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetUser: SetUser::<Impl, IMPL_OFFSET>,
            SetPassword: SetPassword::<Impl, IMPL_OFFSET>,
            GetUser: GetUser::<Impl, IMPL_OFFSET>,
            GetPassword: GetPassword::<Impl, IMPL_OFFSET>,
            LoggedOnUser: LoggedOnUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFNetCredential as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFNetCredentialCacheImpl: Sized {
    fn GetCredential();
    fn SetGood();
    fn SetUserOptions();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFNetCredentialCacheVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFNetCredentialCacheImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFNetCredentialCacheVtbl {
        unsafe extern "system" fn GetCredential<Impl: IMFNetCredentialCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pszrealm: super::super::Foundation::PWSTR, dwauthenticationflags: u32, ppcred: *mut ::windows::core::RawPtr, pdwrequirementsflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGood<Impl: IMFNetCredentialCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcred: ::windows::core::RawPtr, fgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserOptions<Impl: IMFNetCredentialCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcred: ::windows::core::RawPtr, dwoptionsflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCredential: GetCredential::<Impl, IMPL_OFFSET>,
            SetGood: SetGood::<Impl, IMPL_OFFSET>,
            SetUserOptions: SetUserOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFNetCredentialCache as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFNetCredentialManagerImpl: Sized {
    fn BeginGetCredentials();
    fn EndGetCredentials();
    fn SetGood();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFNetCredentialManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFNetCredentialManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFNetCredentialManagerVtbl {
        unsafe extern "system" fn BeginGetCredentials<Impl: IMFNetCredentialManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam: *const MFNetCredentialManagerGetParam, pcallback: ::windows::core::RawPtr, pstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndGetCredentials<Impl: IMFNetCredentialManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, ppcred: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGood<Impl: IMFNetCredentialManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcred: ::windows::core::RawPtr, fgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginGetCredentials: BeginGetCredentials::<Impl, IMPL_OFFSET>,
            EndGetCredentials: EndGetCredentials::<Impl, IMPL_OFFSET>,
            SetGood: SetGood::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFNetCredentialManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFNetCrossOriginSupportImpl: Sized {
    fn GetCrossOriginPolicy();
    fn GetSourceOrigin();
    fn IsSameOrigin();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFNetCrossOriginSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFNetCrossOriginSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFNetCrossOriginSupportVtbl {
        unsafe extern "system" fn GetCrossOriginPolicy<Impl: IMFNetCrossOriginSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut MF_CROSS_ORIGIN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceOrigin<Impl: IMFNetCrossOriginSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsourceorigin: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSameOrigin<Impl: IMFNetCrossOriginSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszurl: super::super::Foundation::PWSTR, pfissameorigin: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCrossOriginPolicy: GetCrossOriginPolicy::<Impl, IMPL_OFFSET>,
            GetSourceOrigin: GetSourceOrigin::<Impl, IMPL_OFFSET>,
            IsSameOrigin: IsSameOrigin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFNetCrossOriginSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFNetProxyLocatorImpl: Sized {
    fn FindFirstProxy();
    fn FindNextProxy();
    fn RegisterProxyResult();
    fn GetCurrentProxy();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFNetProxyLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFNetProxyLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFNetProxyLocatorVtbl {
        unsafe extern "system" fn FindFirstProxy<Impl: IMFNetProxyLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhost: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindNextProxy<Impl: IMFNetProxyLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterProxyResult<Impl: IMFNetProxyLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrop: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentProxy<Impl: IMFNetProxyLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstr: super::super::Foundation::PWSTR, pcchstr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IMFNetProxyLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproxylocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FindFirstProxy: FindFirstProxy::<Impl, IMPL_OFFSET>,
            FindNextProxy: FindNextProxy::<Impl, IMPL_OFFSET>,
            RegisterProxyResult: RegisterProxyResult::<Impl, IMPL_OFFSET>,
            GetCurrentProxy: GetCurrentProxy::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFNetProxyLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFNetProxyLocatorFactoryImpl: Sized {
    fn CreateProxyLocator();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFNetProxyLocatorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFNetProxyLocatorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFNetProxyLocatorFactoryVtbl {
        unsafe extern "system" fn CreateProxyLocator<Impl: IMFNetProxyLocatorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprotocol: super::super::Foundation::PWSTR, ppproxylocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateProxyLocator: CreateProxyLocator::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFNetProxyLocatorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFNetResourceFilterImpl: Sized {
    fn OnRedirect();
    fn OnSendingRequest();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFNetResourceFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFNetResourceFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFNetResourceFilterVtbl {
        unsafe extern "system" fn OnRedirect<Impl: IMFNetResourceFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pvbcancel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSendingRequest<Impl: IMFNetResourceFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnRedirect: OnRedirect::<Impl, IMPL_OFFSET>,
            OnSendingRequest: OnSendingRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFNetResourceFilter as ::windows::core::Interface>::IID
    }
}
pub trait IMFNetSchemeHandlerConfigImpl: Sized {
    fn GetNumberOfSupportedProtocols();
    fn GetSupportedProtocolType();
    fn ResetProtocolRolloverSettings();
}
impl IMFNetSchemeHandlerConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFNetSchemeHandlerConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFNetSchemeHandlerConfigVtbl {
        unsafe extern "system" fn GetNumberOfSupportedProtocols<Impl: IMFNetSchemeHandlerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedProtocolType<Impl: IMFNetSchemeHandlerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nprotocolindex: u32, pnprotocoltype: *mut MFNETSOURCE_PROTOCOL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetProtocolRolloverSettings<Impl: IMFNetSchemeHandlerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNumberOfSupportedProtocols: GetNumberOfSupportedProtocols::<Impl, IMPL_OFFSET>,
            GetSupportedProtocolType: GetSupportedProtocolType::<Impl, IMPL_OFFSET>,
            ResetProtocolRolloverSettings: ResetProtocolRolloverSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFNetSchemeHandlerConfig as ::windows::core::Interface>::IID
    }
}
pub trait IMFObjectReferenceStreamImpl: Sized {
    fn SaveReference();
    fn LoadReference();
}
impl IMFObjectReferenceStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFObjectReferenceStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFObjectReferenceStreamVtbl {
        unsafe extern "system" fn SaveReference<Impl: IMFObjectReferenceStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadReference<Impl: IMFObjectReferenceStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SaveReference: SaveReference::<Impl, IMPL_OFFSET>,
            LoadReference: LoadReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFObjectReferenceStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFOutputPolicyImpl: Sized + IMFAttributesImpl {
    fn GenerateRequiredSchemas();
    fn GetOriginatorID();
    fn GetMinimumGRLVersion();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFOutputPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFOutputPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFOutputPolicyVtbl {
        unsafe extern "system" fn GenerateRequiredSchemas<Impl: IMFOutputPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, guidoutputsubtype: ::windows::core::GUID, rgguidprotectionschemassupported: *const ::windows::core::GUID, cprotectionschemassupported: u32, pprequiredprotectionschemas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOriginatorID<Impl: IMFOutputPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidoriginatorid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinimumGRLVersion<Impl: IMFOutputPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminimumgrlversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GenerateRequiredSchemas: GenerateRequiredSchemas::<Impl, IMPL_OFFSET>,
            GetOriginatorID: GetOriginatorID::<Impl, IMPL_OFFSET>,
            GetMinimumGRLVersion: GetMinimumGRLVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFOutputPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFOutputSchemaImpl: Sized + IMFAttributesImpl {
    fn GetSchemaType();
    fn GetConfigurationData();
    fn GetOriginatorID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFOutputSchemaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFOutputSchemaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFOutputSchemaVtbl {
        unsafe extern "system" fn GetSchemaType<Impl: IMFOutputSchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidschematype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConfigurationData<Impl: IMFOutputSchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOriginatorID<Impl: IMFOutputSchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidoriginatorid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSchemaType: GetSchemaType::<Impl, IMPL_OFFSET>,
            GetConfigurationData: GetConfigurationData::<Impl, IMPL_OFFSET>,
            GetOriginatorID: GetOriginatorID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFOutputSchema as ::windows::core::Interface>::IID
    }
}
pub trait IMFOutputTrustAuthorityImpl: Sized {
    fn GetAction();
    fn SetPolicy();
}
impl IMFOutputTrustAuthorityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFOutputTrustAuthorityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFOutputTrustAuthorityVtbl {
        unsafe extern "system" fn GetAction<Impl: IMFOutputTrustAuthorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paction: *mut MFPOLICYMANAGER_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPolicy<Impl: IMFOutputTrustAuthorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicy: *const ::windows::core::RawPtr, npolicy: u32, ppbticket: *mut *mut u8, pcbticket: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAction: GetAction::<Impl, IMPL_OFFSET>,
            SetPolicy: SetPolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFOutputTrustAuthority as ::windows::core::Interface>::IID
    }
}
pub trait IMFPMPClientImpl: Sized {
    fn SetPMPHost();
}
impl IMFPMPClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPMPClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPMPClientVtbl {
        unsafe extern "system" fn SetPMPHost<Impl: IMFPMPClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmphost: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetPMPHost: SetPMPHost::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPMPClient as ::windows::core::Interface>::IID
    }
}
pub trait IMFPMPClientAppImpl: Sized {
    fn SetPMPHost();
}
impl IMFPMPClientAppVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPMPClientAppImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPMPClientAppVtbl {
        unsafe extern "system" fn SetPMPHost<Impl: IMFPMPClientAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmphost: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetPMPHost: SetPMPHost::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPMPClientApp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMFPMPHostImpl: Sized {
    fn LockProcess();
    fn UnlockProcess();
    fn CreateObjectByCLSID();
}
#[cfg(feature = "Win32_System_Com")]
impl IMFPMPHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPMPHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPMPHostVtbl {
        unsafe extern "system" fn LockProcess<Impl: IMFPMPHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockProcess<Impl: IMFPMPHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateObjectByCLSID<Impl: IMFPMPHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, pstream: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LockProcess: LockProcess::<Impl, IMPL_OFFSET>,
            UnlockProcess: UnlockProcess::<Impl, IMPL_OFFSET>,
            CreateObjectByCLSID: CreateObjectByCLSID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPMPHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMFPMPHostAppImpl: Sized {
    fn LockProcess();
    fn UnlockProcess();
    fn ActivateClassById();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMFPMPHostAppVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPMPHostAppImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPMPHostAppVtbl {
        unsafe extern "system" fn LockProcess<Impl: IMFPMPHostAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockProcess<Impl: IMFPMPHostAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActivateClassById<Impl: IMFPMPHostAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: super::super::Foundation::PWSTR, pstream: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LockProcess: LockProcess::<Impl, IMPL_OFFSET>,
            UnlockProcess: UnlockProcess::<Impl, IMPL_OFFSET>,
            ActivateClassById: ActivateClassById::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPMPHostApp as ::windows::core::Interface>::IID
    }
}
pub trait IMFPMPServerImpl: Sized {
    fn LockProcess();
    fn UnlockProcess();
    fn CreateObjectByCLSID();
}
impl IMFPMPServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPMPServerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPMPServerVtbl {
        unsafe extern "system" fn LockProcess<Impl: IMFPMPServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockProcess<Impl: IMFPMPServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateObjectByCLSID<Impl: IMFPMPServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LockProcess: LockProcess::<Impl, IMPL_OFFSET>,
            UnlockProcess: UnlockProcess::<Impl, IMPL_OFFSET>,
            CreateObjectByCLSID: CreateObjectByCLSID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPMPServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMFPMediaItemImpl: Sized {
    fn GetMediaPlayer();
    fn GetURL();
    fn GetObject();
    fn GetUserData();
    fn SetUserData();
    fn GetStartStopPosition();
    fn SetStartStopPosition();
    fn HasVideo();
    fn HasAudio();
    fn IsProtected();
    fn GetDuration();
    fn GetNumberOfStreams();
    fn GetStreamSelection();
    fn SetStreamSelection();
    fn GetStreamAttribute();
    fn GetPresentationAttribute();
    fn GetCharacteristics();
    fn SetStreamSink();
    fn GetMetadata();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMFPMediaItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPMediaItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPMediaItemVtbl {
        unsafe extern "system" fn GetMediaPlayer<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediaplayer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetURL<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserData<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwuserdata: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserData<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuserdata: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStartStopPosition<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidstartpositiontype: *mut ::windows::core::GUID, pvstartvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pguidstoppositiontype: *mut ::windows::core::GUID, pvstopvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartStopPosition<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidstartpositiontype: *const ::windows::core::GUID, pvstartvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pguidstoppositiontype: *const ::windows::core::GUID, pvstopvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasVideo<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasvideo: *mut super::super::Foundation::BOOL, pfselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasAudio<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasaudio: *mut super::super::Foundation::BOOL, pfselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsProtected<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuration<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidpositiontype: *const ::windows::core::GUID, pvdurationvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumberOfStreams<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamSelection<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamSelection<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, fenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamAttribute<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, guidmfattribute: *const ::windows::core::GUID, pvvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentationAttribute<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmfattribute: *const ::windows::core::GUID, pvvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCharacteristics<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamSink<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pmediasink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetadata<Impl: IMFPMediaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadatastore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetMediaPlayer: GetMediaPlayer::<Impl, IMPL_OFFSET>,
            GetURL: GetURL::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
            GetUserData: GetUserData::<Impl, IMPL_OFFSET>,
            SetUserData: SetUserData::<Impl, IMPL_OFFSET>,
            GetStartStopPosition: GetStartStopPosition::<Impl, IMPL_OFFSET>,
            SetStartStopPosition: SetStartStopPosition::<Impl, IMPL_OFFSET>,
            HasVideo: HasVideo::<Impl, IMPL_OFFSET>,
            HasAudio: HasAudio::<Impl, IMPL_OFFSET>,
            IsProtected: IsProtected::<Impl, IMPL_OFFSET>,
            GetDuration: GetDuration::<Impl, IMPL_OFFSET>,
            GetNumberOfStreams: GetNumberOfStreams::<Impl, IMPL_OFFSET>,
            GetStreamSelection: GetStreamSelection::<Impl, IMPL_OFFSET>,
            SetStreamSelection: SetStreamSelection::<Impl, IMPL_OFFSET>,
            GetStreamAttribute: GetStreamAttribute::<Impl, IMPL_OFFSET>,
            GetPresentationAttribute: GetPresentationAttribute::<Impl, IMPL_OFFSET>,
            GetCharacteristics: GetCharacteristics::<Impl, IMPL_OFFSET>,
            SetStreamSink: SetStreamSink::<Impl, IMPL_OFFSET>,
            GetMetadata: GetMetadata::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPMediaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFPMediaPlayerImpl: Sized {
    fn Play();
    fn Pause();
    fn Stop();
    fn FrameStep();
    fn SetPosition();
    fn GetPosition();
    fn GetDuration();
    fn SetRate();
    fn GetRate();
    fn GetSupportedRates();
    fn GetState();
    fn CreateMediaItemFromURL();
    fn CreateMediaItemFromObject();
    fn SetMediaItem();
    fn ClearMediaItem();
    fn GetMediaItem();
    fn GetVolume();
    fn SetVolume();
    fn GetBalance();
    fn SetBalance();
    fn GetMute();
    fn SetMute();
    fn GetNativeVideoSize();
    fn GetIdealVideoSize();
    fn SetVideoSourceRect();
    fn GetVideoSourceRect();
    fn SetAspectRatioMode();
    fn GetAspectRatioMode();
    fn GetVideoWindow();
    fn UpdateVideo();
    fn SetBorderColor();
    fn GetBorderColor();
    fn InsertEffect();
    fn RemoveEffect();
    fn RemoveAllEffects();
    fn Shutdown();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFPMediaPlayerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPMediaPlayerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPMediaPlayerVtbl {
        unsafe extern "system" fn Play<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FrameStep<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPosition<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidpositiontype: *const ::windows::core::GUID, pvpositionvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPosition<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidpositiontype: *const ::windows::core::GUID, pvpositionvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuration<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidpositiontype: *const ::windows::core::GUID, pvdurationvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRate<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flrate: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRate<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflrate: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedRates<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforwarddirection: super::super::Foundation::BOOL, pflslowestrate: *mut f32, pflfastestrate: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetState<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestate: *mut MFP_MEDIAPLAYER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMediaItemFromURL<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, fsync: super::super::Foundation::BOOL, dwuserdata: usize, ppmediaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMediaItemFromObject<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piunknownobj: *mut ::core::ffi::c_void, fsync: super::super::Foundation::BOOL, dwuserdata: usize, ppmediaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMediaItem<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimfpmediaitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearMediaItem<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMediaItem<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimfpmediaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVolume<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflvolume: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVolume<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flvolume: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBalance<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflbalance: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBalance<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flbalance: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMute<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmute: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMute<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmute: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNativeVideoSize<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvideo: *mut super::super::Foundation::SIZE, pszarvideo: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIdealVideoSize<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmin: *mut super::super::Foundation::SIZE, pszmax: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVideoSourceRect<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnrcsource: *const MFVideoNormalizedRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoSourceRect<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnrcsource: *mut MFVideoNormalizedRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAspectRatioMode<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspectratiomode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAspectRatioMode<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwaspectratiomode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoWindow<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndvideo: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateVideo<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBorderColor<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clr: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBorderColor<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertEffect<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, foptional: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveEffect<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllEffects<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFPMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Play: Play::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            FrameStep: FrameStep::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
            GetDuration: GetDuration::<Impl, IMPL_OFFSET>,
            SetRate: SetRate::<Impl, IMPL_OFFSET>,
            GetRate: GetRate::<Impl, IMPL_OFFSET>,
            GetSupportedRates: GetSupportedRates::<Impl, IMPL_OFFSET>,
            GetState: GetState::<Impl, IMPL_OFFSET>,
            CreateMediaItemFromURL: CreateMediaItemFromURL::<Impl, IMPL_OFFSET>,
            CreateMediaItemFromObject: CreateMediaItemFromObject::<Impl, IMPL_OFFSET>,
            SetMediaItem: SetMediaItem::<Impl, IMPL_OFFSET>,
            ClearMediaItem: ClearMediaItem::<Impl, IMPL_OFFSET>,
            GetMediaItem: GetMediaItem::<Impl, IMPL_OFFSET>,
            GetVolume: GetVolume::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            GetBalance: GetBalance::<Impl, IMPL_OFFSET>,
            SetBalance: SetBalance::<Impl, IMPL_OFFSET>,
            GetMute: GetMute::<Impl, IMPL_OFFSET>,
            SetMute: SetMute::<Impl, IMPL_OFFSET>,
            GetNativeVideoSize: GetNativeVideoSize::<Impl, IMPL_OFFSET>,
            GetIdealVideoSize: GetIdealVideoSize::<Impl, IMPL_OFFSET>,
            SetVideoSourceRect: SetVideoSourceRect::<Impl, IMPL_OFFSET>,
            GetVideoSourceRect: GetVideoSourceRect::<Impl, IMPL_OFFSET>,
            SetAspectRatioMode: SetAspectRatioMode::<Impl, IMPL_OFFSET>,
            GetAspectRatioMode: GetAspectRatioMode::<Impl, IMPL_OFFSET>,
            GetVideoWindow: GetVideoWindow::<Impl, IMPL_OFFSET>,
            UpdateVideo: UpdateVideo::<Impl, IMPL_OFFSET>,
            SetBorderColor: SetBorderColor::<Impl, IMPL_OFFSET>,
            GetBorderColor: GetBorderColor::<Impl, IMPL_OFFSET>,
            InsertEffect: InsertEffect::<Impl, IMPL_OFFSET>,
            RemoveEffect: RemoveEffect::<Impl, IMPL_OFFSET>,
            RemoveAllEffects: RemoveAllEffects::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPMediaPlayer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IMFPMediaPlayerCallbackImpl: Sized {
    fn OnMediaPlayerEvent();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IMFPMediaPlayerCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPMediaPlayerCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPMediaPlayerCallbackVtbl {
        unsafe extern "system" fn OnMediaPlayerEvent<Impl: IMFPMediaPlayerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventheader: *const MFP_EVENT_HEADER) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnMediaPlayerEvent: OnMediaPlayerEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPMediaPlayerCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFPluginControlImpl: Sized {
    fn GetPreferredClsid();
    fn GetPreferredClsidByIndex();
    fn SetPreferredClsid();
    fn IsDisabled();
    fn GetDisabledByIndex();
    fn SetDisabled();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFPluginControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPluginControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPluginControlVtbl {
        unsafe extern "system" fn GetPreferredClsid<Impl: IMFPluginControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugintype: u32, selector: super::super::Foundation::PWSTR, clsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreferredClsidByIndex<Impl: IMFPluginControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugintype: u32, index: u32, selector: *mut super::super::Foundation::PWSTR, clsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredClsid<Impl: IMFPluginControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugintype: u32, selector: super::super::Foundation::PWSTR, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDisabled<Impl: IMFPluginControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugintype: u32, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisabledByIndex<Impl: IMFPluginControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugintype: u32, index: u32, clsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisabled<Impl: IMFPluginControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugintype: u32, clsid: *const ::windows::core::GUID, disabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPreferredClsid: GetPreferredClsid::<Impl, IMPL_OFFSET>,
            GetPreferredClsidByIndex: GetPreferredClsidByIndex::<Impl, IMPL_OFFSET>,
            SetPreferredClsid: SetPreferredClsid::<Impl, IMPL_OFFSET>,
            IsDisabled: IsDisabled::<Impl, IMPL_OFFSET>,
            GetDisabledByIndex: GetDisabledByIndex::<Impl, IMPL_OFFSET>,
            SetDisabled: SetDisabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPluginControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFPluginControl2Impl: Sized + IMFPluginControlImpl {
    fn SetPolicy();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFPluginControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPluginControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPluginControl2Vtbl {
        unsafe extern "system" fn SetPolicy<Impl: IMFPluginControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: MF_PLUGIN_CONTROL_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IMFPluginControlVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetPolicy: SetPolicy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPluginControl2 as ::windows::core::Interface>::IID
    }
}
pub trait IMFPresentationClockImpl: Sized + IMFClockImpl {
    fn SetTimeSource();
    fn GetTimeSource();
    fn GetTime();
    fn AddClockStateSink();
    fn RemoveClockStateSink();
    fn Start();
    fn Stop();
    fn Pause();
}
impl IMFPresentationClockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPresentationClockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPresentationClockVtbl {
        unsafe extern "system" fn SetTimeSource<Impl: IMFPresentationClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimesource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimeSource<Impl: IMFPresentationClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptimesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTime<Impl: IMFPresentationClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnsclocktime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddClockStateSink<Impl: IMFPresentationClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveClockStateSink<Impl: IMFPresentationClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Start<Impl: IMFPresentationClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llclockstartoffset: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IMFPresentationClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IMFPresentationClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFClockVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetTimeSource: SetTimeSource::<Impl, IMPL_OFFSET>,
            GetTimeSource: GetTimeSource::<Impl, IMPL_OFFSET>,
            GetTime: GetTime::<Impl, IMPL_OFFSET>,
            AddClockStateSink: AddClockStateSink::<Impl, IMPL_OFFSET>,
            RemoveClockStateSink: RemoveClockStateSink::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPresentationClock as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFPresentationDescriptorImpl: Sized + IMFAttributesImpl {
    fn GetStreamDescriptorCount();
    fn GetStreamDescriptorByIndex();
    fn SelectStream();
    fn DeselectStream();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFPresentationDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPresentationDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPresentationDescriptorVtbl {
        unsafe extern "system" fn GetStreamDescriptorCount<Impl: IMFPresentationDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdescriptorcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamDescriptorByIndex<Impl: IMFPresentationDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pfselected: *mut super::super::Foundation::BOOL, ppdescriptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectStream<Impl: IMFPresentationDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdescriptorindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeselectStream<Impl: IMFPresentationDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdescriptorindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IMFPresentationDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppresentationdescriptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStreamDescriptorCount: GetStreamDescriptorCount::<Impl, IMPL_OFFSET>,
            GetStreamDescriptorByIndex: GetStreamDescriptorByIndex::<Impl, IMPL_OFFSET>,
            SelectStream: SelectStream::<Impl, IMPL_OFFSET>,
            DeselectStream: DeselectStream::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPresentationDescriptor as ::windows::core::Interface>::IID
    }
}
pub trait IMFPresentationTimeSourceImpl: Sized + IMFClockImpl {
    fn GetUnderlyingClock();
}
impl IMFPresentationTimeSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFPresentationTimeSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFPresentationTimeSourceVtbl {
        unsafe extern "system" fn GetUnderlyingClock<Impl: IMFPresentationTimeSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IMFClockVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetUnderlyingClock: GetUnderlyingClock::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFPresentationTimeSource as ::windows::core::Interface>::IID
    }
}
pub trait IMFProtectedEnvironmentAccessImpl: Sized {
    fn Call();
    fn ReadGRL();
}
impl IMFProtectedEnvironmentAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFProtectedEnvironmentAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFProtectedEnvironmentAccessVtbl {
        unsafe extern "system" fn Call<Impl: IMFProtectedEnvironmentAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputlength: u32, input: *const u8, outputlength: u32, output: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadGRL<Impl: IMFProtectedEnvironmentAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputlength: *mut u32, output: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Call: Call::<Impl, IMPL_OFFSET>, ReadGRL: ReadGRL::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFProtectedEnvironmentAccess as ::windows::core::Interface>::IID
    }
}
pub trait IMFQualityAdviseImpl: Sized {
    fn SetDropMode();
    fn SetQualityLevel();
    fn GetDropMode();
    fn GetQualityLevel();
    fn DropTime();
}
impl IMFQualityAdviseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFQualityAdviseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFQualityAdviseVtbl {
        unsafe extern "system" fn SetDropMode<Impl: IMFQualityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edropmode: MF_QUALITY_DROP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQualityLevel<Impl: IMFQualityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, equalitylevel: MF_QUALITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDropMode<Impl: IMFQualityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedropmode: *mut MF_QUALITY_DROP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQualityLevel<Impl: IMFQualityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pequalitylevel: *mut MF_QUALITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DropTime<Impl: IMFQualityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnsamounttodrop: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDropMode: SetDropMode::<Impl, IMPL_OFFSET>,
            SetQualityLevel: SetQualityLevel::<Impl, IMPL_OFFSET>,
            GetDropMode: GetDropMode::<Impl, IMPL_OFFSET>,
            GetQualityLevel: GetQualityLevel::<Impl, IMPL_OFFSET>,
            DropTime: DropTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFQualityAdvise as ::windows::core::Interface>::IID
    }
}
pub trait IMFQualityAdvise2Impl: Sized + IMFQualityAdviseImpl {
    fn NotifyQualityEvent();
}
impl IMFQualityAdvise2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFQualityAdvise2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFQualityAdvise2Vtbl {
        unsafe extern "system" fn NotifyQualityEvent<Impl: IMFQualityAdvise2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IMFQualityAdviseVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), NotifyQualityEvent: NotifyQualityEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFQualityAdvise2 as ::windows::core::Interface>::IID
    }
}
pub trait IMFQualityAdviseLimitsImpl: Sized {
    fn GetMaximumDropMode();
    fn GetMinimumQualityLevel();
}
impl IMFQualityAdviseLimitsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFQualityAdviseLimitsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFQualityAdviseLimitsVtbl {
        unsafe extern "system" fn GetMaximumDropMode<Impl: IMFQualityAdviseLimitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedropmode: *mut MF_QUALITY_DROP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinimumQualityLevel<Impl: IMFQualityAdviseLimitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pequalitylevel: *mut MF_QUALITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetMaximumDropMode: GetMaximumDropMode::<Impl, IMPL_OFFSET>,
            GetMinimumQualityLevel: GetMinimumQualityLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFQualityAdviseLimits as ::windows::core::Interface>::IID
    }
}
pub trait IMFQualityManagerImpl: Sized {
    fn NotifyTopology();
    fn NotifyPresentationClock();
    fn NotifyProcessInput();
    fn NotifyProcessOutput();
    fn NotifyQualityEvent();
    fn Shutdown();
}
impl IMFQualityManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFQualityManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFQualityManagerVtbl {
        unsafe extern "system" fn NotifyTopology<Impl: IMFQualityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyPresentationClock<Impl: IMFQualityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyProcessInput<Impl: IMFQualityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, linputindex: i32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyProcessOutput<Impl: IMFQualityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, loutputindex: i32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyQualityEvent<Impl: IMFQualityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFQualityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NotifyTopology: NotifyTopology::<Impl, IMPL_OFFSET>,
            NotifyPresentationClock: NotifyPresentationClock::<Impl, IMPL_OFFSET>,
            NotifyProcessInput: NotifyProcessInput::<Impl, IMPL_OFFSET>,
            NotifyProcessOutput: NotifyProcessOutput::<Impl, IMPL_OFFSET>,
            NotifyQualityEvent: NotifyQualityEvent::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFQualityManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFRateControlImpl: Sized {
    fn SetRate();
    fn GetRate();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFRateControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFRateControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFRateControlVtbl {
        unsafe extern "system" fn SetRate<Impl: IMFRateControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fthin: super::super::Foundation::BOOL, flrate: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRate<Impl: IMFRateControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfthin: *mut super::super::Foundation::BOOL, pflrate: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetRate: SetRate::<Impl, IMPL_OFFSET>,
            GetRate: GetRate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFRateControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFRateSupportImpl: Sized {
    fn GetSlowestRate();
    fn GetFastestRate();
    fn IsRateSupported();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFRateSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFRateSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFRateSupportVtbl {
        unsafe extern "system" fn GetSlowestRate<Impl: IMFRateSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edirection: MFRATE_DIRECTION, fthin: super::super::Foundation::BOOL, pflrate: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFastestRate<Impl: IMFRateSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edirection: MFRATE_DIRECTION, fthin: super::super::Foundation::BOOL, pflrate: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRateSupported<Impl: IMFRateSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fthin: super::super::Foundation::BOOL, flrate: f32, pflnearestsupportedrate: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSlowestRate: GetSlowestRate::<Impl, IMPL_OFFSET>,
            GetFastestRate: GetFastestRate::<Impl, IMPL_OFFSET>,
            IsRateSupported: IsRateSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFRateSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFReadWriteClassFactoryImpl: Sized {
    fn CreateInstanceFromURL();
    fn CreateInstanceFromObject();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFReadWriteClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFReadWriteClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFReadWriteClassFactoryVtbl {
        unsafe extern "system" fn CreateInstanceFromURL<Impl: IMFReadWriteClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, pwszurl: super::super::Foundation::PWSTR, pattributes: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstanceFromObject<Impl: IMFReadWriteClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, punkobject: *mut ::core::ffi::c_void, pattributes: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateInstanceFromURL: CreateInstanceFromURL::<Impl, IMPL_OFFSET>,
            CreateInstanceFromObject: CreateInstanceFromObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFReadWriteClassFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFRealTimeClientImpl: Sized {
    fn RegisterThreads();
    fn UnregisterThreads();
    fn SetWorkQueue();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFRealTimeClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFRealTimeClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFRealTimeClientVtbl {
        unsafe extern "system" fn RegisterThreads<Impl: IMFRealTimeClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtaskindex: u32, wszclass: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterThreads<Impl: IMFRealTimeClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWorkQueue<Impl: IMFRealTimeClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwworkqueueid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterThreads: RegisterThreads::<Impl, IMPL_OFFSET>,
            UnregisterThreads: UnregisterThreads::<Impl, IMPL_OFFSET>,
            SetWorkQueue: SetWorkQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFRealTimeClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFRealTimeClientExImpl: Sized {
    fn RegisterThreadsEx();
    fn UnregisterThreads();
    fn SetWorkQueueEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFRealTimeClientExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFRealTimeClientExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFRealTimeClientExVtbl {
        unsafe extern "system" fn RegisterThreadsEx<Impl: IMFRealTimeClientExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtaskindex: *mut u32, wszclassname: super::super::Foundation::PWSTR, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterThreads<Impl: IMFRealTimeClientExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWorkQueueEx<Impl: IMFRealTimeClientExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmultithreadedworkqueueid: u32, lworkitembasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterThreadsEx: RegisterThreadsEx::<Impl, IMPL_OFFSET>,
            UnregisterThreads: UnregisterThreads::<Impl, IMPL_OFFSET>,
            SetWorkQueueEx: SetWorkQueueEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFRealTimeClientEx as ::windows::core::Interface>::IID
    }
}
pub trait IMFRelativePanelReportImpl: Sized {
    fn GetRelativePanel();
}
impl IMFRelativePanelReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFRelativePanelReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFRelativePanelReportVtbl {
        unsafe extern "system" fn GetRelativePanel<Impl: IMFRelativePanelReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetRelativePanel: GetRelativePanel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFRelativePanelReport as ::windows::core::Interface>::IID
    }
}
pub trait IMFRelativePanelWatcherImpl: Sized + IMFShutdownImpl {
    fn BeginGetReport();
    fn EndGetReport();
    fn GetReport();
}
impl IMFRelativePanelWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFRelativePanelWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFRelativePanelWatcherVtbl {
        unsafe extern "system" fn BeginGetReport<Impl: IMFRelativePanelWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndGetReport<Impl: IMFRelativePanelWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, pprelativepanelreport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReport<Impl: IMFRelativePanelWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprelativepanelreport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFShutdownVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginGetReport: BeginGetReport::<Impl, IMPL_OFFSET>,
            EndGetReport: EndGetReport::<Impl, IMPL_OFFSET>,
            GetReport: GetReport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFRelativePanelWatcher as ::windows::core::Interface>::IID
    }
}
pub trait IMFRemoteAsyncCallbackImpl: Sized {
    fn Invoke();
}
impl IMFRemoteAsyncCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFRemoteAsyncCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFRemoteAsyncCallbackVtbl {
        unsafe extern "system" fn Invoke<Impl: IMFRemoteAsyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, premoteresult: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFRemoteAsyncCallback as ::windows::core::Interface>::IID
    }
}
pub trait IMFRemoteDesktopPluginImpl: Sized {
    fn UpdateTopology();
}
impl IMFRemoteDesktopPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFRemoteDesktopPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFRemoteDesktopPluginVtbl {
        unsafe extern "system" fn UpdateTopology<Impl: IMFRemoteDesktopPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), UpdateTopology: UpdateTopology::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFRemoteDesktopPlugin as ::windows::core::Interface>::IID
    }
}
pub trait IMFRemoteProxyImpl: Sized {
    fn GetRemoteObject();
    fn GetRemoteHost();
}
impl IMFRemoteProxyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFRemoteProxyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFRemoteProxyVtbl {
        unsafe extern "system" fn GetRemoteObject<Impl: IMFRemoteProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRemoteHost<Impl: IMFRemoteProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRemoteObject: GetRemoteObject::<Impl, IMPL_OFFSET>,
            GetRemoteHost: GetRemoteHost::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFRemoteProxy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFSAMIStyleImpl: Sized {
    fn GetStyleCount();
    fn GetStyles();
    fn SetSelectedStyle();
    fn GetSelectedStyle();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFSAMIStyleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSAMIStyleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSAMIStyleVtbl {
        unsafe extern "system" fn GetStyleCount<Impl: IMFSAMIStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStyles<Impl: IMFSAMIStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvarstylearray: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelectedStyle<Impl: IMFSAMIStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszstyle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSelectedStyle<Impl: IMFSAMIStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszstyle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStyleCount: GetStyleCount::<Impl, IMPL_OFFSET>,
            GetStyles: GetStyles::<Impl, IMPL_OFFSET>,
            SetSelectedStyle: SetSelectedStyle::<Impl, IMPL_OFFSET>,
            GetSelectedStyle: GetSelectedStyle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSAMIStyle as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFSSLCertificateManagerImpl: Sized {
    fn GetClientCertificate();
    fn BeginGetClientCertificate();
    fn EndGetClientCertificate();
    fn GetCertificatePolicy();
    fn OnServerCertificate();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFSSLCertificateManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSSLCertificateManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSSLCertificateManagerVtbl {
        unsafe extern "system" fn GetClientCertificate<Impl: IMFSSLCertificateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginGetClientCertificate<Impl: IMFSSLCertificateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndGetClientCertificate<Impl: IMFSSLCertificateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificatePolicy<Impl: IMFSSLCertificateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pfoverrideautomaticcheck: *mut super::super::Foundation::BOOL, pfclientcertificateavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnServerCertificate<Impl: IMFSSLCertificateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pbdata: *const u8, cbdata: u32, pfisgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClientCertificate: GetClientCertificate::<Impl, IMPL_OFFSET>,
            BeginGetClientCertificate: BeginGetClientCertificate::<Impl, IMPL_OFFSET>,
            EndGetClientCertificate: EndGetClientCertificate::<Impl, IMPL_OFFSET>,
            GetCertificatePolicy: GetCertificatePolicy::<Impl, IMPL_OFFSET>,
            OnServerCertificate: OnServerCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSSLCertificateManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFSampleImpl: Sized + IMFAttributesImpl {
    fn GetSampleFlags();
    fn SetSampleFlags();
    fn GetSampleTime();
    fn SetSampleTime();
    fn GetSampleDuration();
    fn SetSampleDuration();
    fn GetBufferCount();
    fn GetBufferByIndex();
    fn ConvertToContiguousBuffer();
    fn AddBuffer();
    fn RemoveBufferByIndex();
    fn RemoveAllBuffers();
    fn GetTotalLength();
    fn CopyToBuffer();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFSampleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSampleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSampleVtbl {
        unsafe extern "system" fn GetSampleFlags<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsampleflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSampleFlags<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsampleflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSampleTime<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnssampletime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSampleTime<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnssampletime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSampleDuration<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnssampleduration: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSampleDuration<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnssampleduration: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferCount<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbuffercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferByIndex<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertToContiguousBuffer<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddBuffer<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveBufferByIndex<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllBuffers<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTotalLength<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbtotallength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyToBuffer<Impl: IMFSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSampleFlags: GetSampleFlags::<Impl, IMPL_OFFSET>,
            SetSampleFlags: SetSampleFlags::<Impl, IMPL_OFFSET>,
            GetSampleTime: GetSampleTime::<Impl, IMPL_OFFSET>,
            SetSampleTime: SetSampleTime::<Impl, IMPL_OFFSET>,
            GetSampleDuration: GetSampleDuration::<Impl, IMPL_OFFSET>,
            SetSampleDuration: SetSampleDuration::<Impl, IMPL_OFFSET>,
            GetBufferCount: GetBufferCount::<Impl, IMPL_OFFSET>,
            GetBufferByIndex: GetBufferByIndex::<Impl, IMPL_OFFSET>,
            ConvertToContiguousBuffer: ConvertToContiguousBuffer::<Impl, IMPL_OFFSET>,
            AddBuffer: AddBuffer::<Impl, IMPL_OFFSET>,
            RemoveBufferByIndex: RemoveBufferByIndex::<Impl, IMPL_OFFSET>,
            RemoveAllBuffers: RemoveAllBuffers::<Impl, IMPL_OFFSET>,
            GetTotalLength: GetTotalLength::<Impl, IMPL_OFFSET>,
            CopyToBuffer: CopyToBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSample as ::windows::core::Interface>::IID
    }
}
pub trait IMFSampleAllocatorControlImpl: Sized {
    fn SetDefaultAllocator();
    fn GetAllocatorUsage();
}
impl IMFSampleAllocatorControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSampleAllocatorControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSampleAllocatorControlVtbl {
        unsafe extern "system" fn SetDefaultAllocator<Impl: IMFSampleAllocatorControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, pallocator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllocatorUsage<Impl: IMFSampleAllocatorControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, pdwinputstreamid: *mut u32, peusage: *mut MFSampleAllocatorUsage) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDefaultAllocator: SetDefaultAllocator::<Impl, IMPL_OFFSET>,
            GetAllocatorUsage: GetAllocatorUsage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSampleAllocatorControl as ::windows::core::Interface>::IID
    }
}
pub trait IMFSampleGrabberSinkCallbackImpl: Sized + IMFClockStateSinkImpl {
    fn OnSetPresentationClock();
    fn OnProcessSample();
    fn OnShutdown();
}
impl IMFSampleGrabberSinkCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSampleGrabberSinkCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSampleGrabberSinkCallbackVtbl {
        unsafe extern "system" fn OnSetPresentationClock<Impl: IMFSampleGrabberSinkCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnProcessSample<Impl: IMFSampleGrabberSinkCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmajormediatype: *const ::windows::core::GUID, dwsampleflags: u32, llsampletime: i64, llsampleduration: i64, psamplebuffer: *const u8, dwsamplesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnShutdown<Impl: IMFSampleGrabberSinkCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFClockStateSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnSetPresentationClock: OnSetPresentationClock::<Impl, IMPL_OFFSET>,
            OnProcessSample: OnProcessSample::<Impl, IMPL_OFFSET>,
            OnShutdown: OnShutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSampleGrabberSinkCallback as ::windows::core::Interface>::IID
    }
}
pub trait IMFSampleGrabberSinkCallback2Impl: Sized + IMFClockStateSinkImpl + IMFSampleGrabberSinkCallbackImpl {
    fn OnProcessSampleEx();
}
impl IMFSampleGrabberSinkCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSampleGrabberSinkCallback2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSampleGrabberSinkCallback2Vtbl {
        unsafe extern "system" fn OnProcessSampleEx<Impl: IMFSampleGrabberSinkCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmajormediatype: *const ::windows::core::GUID, dwsampleflags: u32, llsampletime: i64, llsampleduration: i64, psamplebuffer: *const u8, dwsamplesize: u32, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFSampleGrabberSinkCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnProcessSampleEx: OnProcessSampleEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSampleGrabberSinkCallback2 as ::windows::core::Interface>::IID
    }
}
pub trait IMFSampleOutputStreamImpl: Sized {
    fn BeginWriteSample();
    fn EndWriteSample();
    fn Close();
}
impl IMFSampleOutputStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSampleOutputStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSampleOutputStreamVtbl {
        unsafe extern "system" fn BeginWriteSample<Impl: IMFSampleOutputStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psample: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndWriteSample<Impl: IMFSampleOutputStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMFSampleOutputStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginWriteSample: BeginWriteSample::<Impl, IMPL_OFFSET>,
            EndWriteSample: EndWriteSample::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSampleOutputStream as ::windows::core::Interface>::IID
    }
}
pub trait IMFSampleProtectionImpl: Sized {
    fn GetInputProtectionVersion();
    fn GetOutputProtectionVersion();
    fn GetProtectionCertificate();
    fn InitOutputProtection();
    fn InitInputProtection();
}
impl IMFSampleProtectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSampleProtectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSampleProtectionVtbl {
        unsafe extern "system" fn GetInputProtectionVersion<Impl: IMFSampleProtectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputProtectionVersion<Impl: IMFSampleProtectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtectionCertificate<Impl: IMFSampleProtectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: u32, ppcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitOutputProtection<Impl: IMFSampleProtectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: u32, dwoutputid: u32, pbcert: *const u8, cbcert: u32, ppbseed: *mut *mut u8, pcbseed: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitInputProtection<Impl: IMFSampleProtectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: u32, dwinputid: u32, pbseed: *const u8, cbseed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInputProtectionVersion: GetInputProtectionVersion::<Impl, IMPL_OFFSET>,
            GetOutputProtectionVersion: GetOutputProtectionVersion::<Impl, IMPL_OFFSET>,
            GetProtectionCertificate: GetProtectionCertificate::<Impl, IMPL_OFFSET>,
            InitOutputProtection: InitOutputProtection::<Impl, IMPL_OFFSET>,
            InitInputProtection: InitInputProtection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSampleProtection as ::windows::core::Interface>::IID
    }
}
pub trait IMFSaveJobImpl: Sized {
    fn BeginSave();
    fn EndSave();
    fn CancelSave();
    fn GetProgress();
}
impl IMFSaveJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSaveJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSaveJobVtbl {
        unsafe extern "system" fn BeginSave<Impl: IMFSaveJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndSave<Impl: IMFSaveJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelSave<Impl: IMFSaveJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProgress<Impl: IMFSaveJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercentcomplete: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginSave: BeginSave::<Impl, IMPL_OFFSET>,
            EndSave: EndSave::<Impl, IMPL_OFFSET>,
            CancelSave: CancelSave::<Impl, IMPL_OFFSET>,
            GetProgress: GetProgress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSaveJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMFSchemeHandlerImpl: Sized {
    fn BeginCreateObject();
    fn EndCreateObject();
    fn CancelObjectCreation();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMFSchemeHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSchemeHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSchemeHandlerVtbl {
        unsafe extern "system" fn BeginCreateObject<Impl: IMFSchemeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwflags: u32, pprops: ::windows::core::RawPtr, ppiunknowncancelcookie: *mut *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndCreateObject<Impl: IMFSchemeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelObjectCreation<Impl: IMFSchemeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piunknowncancelcookie: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginCreateObject: BeginCreateObject::<Impl, IMPL_OFFSET>,
            EndCreateObject: EndCreateObject::<Impl, IMPL_OFFSET>,
            CancelObjectCreation: CancelObjectCreation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSchemeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IMFSecureBufferImpl: Sized {
    fn GetIdentifier();
}
impl IMFSecureBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSecureBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSecureBufferVtbl {
        unsafe extern "system" fn GetIdentifier<Impl: IMFSecureBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguididentifier: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIdentifier: GetIdentifier::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSecureBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IMFSecureChannelImpl: Sized {
    fn GetCertificate();
    fn SetupSession();
}
impl IMFSecureChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSecureChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSecureChannelVtbl {
        unsafe extern "system" fn GetCertificate<Impl: IMFSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetupSession<Impl: IMFSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbencryptedsessionkey: *const u8, cbsessionkey: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCertificate: GetCertificate::<Impl, IMPL_OFFSET>,
            SetupSession: SetupSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSecureChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFSeekInfoImpl: Sized {
    fn GetNearestKeyFrames();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFSeekInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSeekInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSeekInfoVtbl {
        unsafe extern "system" fn GetNearestKeyFrames<Impl: IMFSeekInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtimeformat: *const ::windows::core::GUID, pvarstartposition: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarpreviouskeyframe: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarnextkeyframe: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetNearestKeyFrames: GetNearestKeyFrames::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSeekInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFSensorActivitiesReportImpl: Sized {
    fn GetCount();
    fn GetActivityReport();
    fn GetActivityReportByDeviceName();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFSensorActivitiesReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorActivitiesReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorActivitiesReportVtbl {
        unsafe extern "system" fn GetCount<Impl: IMFSensorActivitiesReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActivityReport<Impl: IMFSensorActivitiesReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, sensoractivityreport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActivityReportByDeviceName<Impl: IMFSensorActivitiesReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symbolicname: super::super::Foundation::PWSTR, sensoractivityreport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetActivityReport: GetActivityReport::<Impl, IMPL_OFFSET>,
            GetActivityReportByDeviceName: GetActivityReportByDeviceName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorActivitiesReport as ::windows::core::Interface>::IID
    }
}
pub trait IMFSensorActivitiesReportCallbackImpl: Sized {
    fn OnActivitiesReport();
}
impl IMFSensorActivitiesReportCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorActivitiesReportCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorActivitiesReportCallbackVtbl {
        unsafe extern "system" fn OnActivitiesReport<Impl: IMFSensorActivitiesReportCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensoractivitiesreport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnActivitiesReport: OnActivitiesReport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorActivitiesReportCallback as ::windows::core::Interface>::IID
    }
}
pub trait IMFSensorActivityMonitorImpl: Sized {
    fn Start();
    fn Stop();
}
impl IMFSensorActivityMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorActivityMonitorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorActivityMonitorVtbl {
        unsafe extern "system" fn Start<Impl: IMFSensorActivityMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IMFSensorActivityMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Start: Start::<Impl, IMPL_OFFSET>, Stop: Stop::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorActivityMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFSensorActivityReportImpl: Sized {
    fn GetFriendlyName();
    fn GetSymbolicLink();
    fn GetProcessCount();
    fn GetProcessActivity();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFSensorActivityReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorActivityReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorActivityReportVtbl {
        unsafe extern "system" fn GetFriendlyName<Impl: IMFSensorActivityReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: super::super::Foundation::PWSTR, cchfriendlyname: u32, pcchwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSymbolicLink<Impl: IMFSensorActivityReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symboliclink: super::super::Foundation::PWSTR, cchsymboliclink: u32, pcchwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProcessCount<Impl: IMFSensorActivityReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProcessActivity<Impl: IMFSensorActivityReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppprocessactivity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFriendlyName: GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetSymbolicLink: GetSymbolicLink::<Impl, IMPL_OFFSET>,
            GetProcessCount: GetProcessCount::<Impl, IMPL_OFFSET>,
            GetProcessActivity: GetProcessActivity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorActivityReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFSensorDeviceImpl: Sized {
    fn GetDeviceId();
    fn GetDeviceType();
    fn GetFlags();
    fn GetSymbolicLink();
    fn GetDeviceAttributes();
    fn GetStreamAttributesCount();
    fn GetStreamAttributes();
    fn SetSensorDeviceMode();
    fn GetSensorDeviceMode();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFSensorDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorDeviceVtbl {
        unsafe extern "system" fn GetDeviceId<Impl: IMFSensorDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceType<Impl: IMFSensorDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut MFSensorDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IMFSensorDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSymbolicLink<Impl: IMFSensorDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symboliclink: super::super::Foundation::PWSTR, cchsymboliclink: i32, pcchwritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceAttributes<Impl: IMFSensorDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamAttributesCount<Impl: IMFSensorDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, etype: MFSensorStreamType, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamAttributes<Impl: IMFSensorDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, etype: MFSensorStreamType, dwindex: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSensorDeviceMode<Impl: IMFSensorDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emode: MFSensorDeviceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSensorDeviceMode<Impl: IMFSensorDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pemode: *mut MFSensorDeviceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDeviceId: GetDeviceId::<Impl, IMPL_OFFSET>,
            GetDeviceType: GetDeviceType::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            GetSymbolicLink: GetSymbolicLink::<Impl, IMPL_OFFSET>,
            GetDeviceAttributes: GetDeviceAttributes::<Impl, IMPL_OFFSET>,
            GetStreamAttributesCount: GetStreamAttributesCount::<Impl, IMPL_OFFSET>,
            GetStreamAttributes: GetStreamAttributes::<Impl, IMPL_OFFSET>,
            SetSensorDeviceMode: SetSensorDeviceMode::<Impl, IMPL_OFFSET>,
            GetSensorDeviceMode: GetSensorDeviceMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFSensorGroupImpl: Sized {
    fn GetSymbolicLink();
    fn GetFlags();
    fn GetSensorGroupAttributes();
    fn GetSensorDeviceCount();
    fn GetSensorDevice();
    fn SetDefaultSensorDeviceIndex();
    fn GetDefaultSensorDeviceIndex();
    fn CreateMediaSource();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFSensorGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorGroupVtbl {
        unsafe extern "system" fn GetSymbolicLink<Impl: IMFSensorGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symboliclink: super::super::Foundation::PWSTR, cchsymboliclink: i32, pcchwritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IMFSensorGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSensorGroupAttributes<Impl: IMFSensorGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSensorDeviceCount<Impl: IMFSensorGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSensorDevice<Impl: IMFSensorGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultSensorDeviceIndex<Impl: IMFSensorGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultSensorDeviceIndex<Impl: IMFSensorGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMediaSource<Impl: IMFSensorGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSymbolicLink: GetSymbolicLink::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            GetSensorGroupAttributes: GetSensorGroupAttributes::<Impl, IMPL_OFFSET>,
            GetSensorDeviceCount: GetSensorDeviceCount::<Impl, IMPL_OFFSET>,
            GetSensorDevice: GetSensorDevice::<Impl, IMPL_OFFSET>,
            SetDefaultSensorDeviceIndex: SetDefaultSensorDeviceIndex::<Impl, IMPL_OFFSET>,
            GetDefaultSensorDeviceIndex: GetDefaultSensorDeviceIndex::<Impl, IMPL_OFFSET>,
            CreateMediaSource: CreateMediaSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFSensorProcessActivityImpl: Sized {
    fn GetProcessId();
    fn GetStreamingState();
    fn GetStreamingMode();
    fn GetReportTime();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFSensorProcessActivityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorProcessActivityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorProcessActivityVtbl {
        unsafe extern "system" fn GetProcessId<Impl: IMFSensorProcessActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamingState<Impl: IMFSensorProcessActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfstreaming: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamingMode<Impl: IMFSensorProcessActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut MFSensorDeviceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReportTime<Impl: IMFSensorProcessActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pft: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProcessId: GetProcessId::<Impl, IMPL_OFFSET>,
            GetStreamingState: GetStreamingState::<Impl, IMPL_OFFSET>,
            GetStreamingMode: GetStreamingMode::<Impl, IMPL_OFFSET>,
            GetReportTime: GetReportTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorProcessActivity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFSensorProfileImpl: Sized {
    fn GetProfileId();
    fn AddProfileFilter();
    fn IsMediaTypeSupported();
    fn AddBlockedControl();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFSensorProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorProfileVtbl {
        unsafe extern "system" fn GetProfileId<Impl: IMFSensorProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut SENSORPROFILEID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddProfileFilter<Impl: IMFSensorProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamid: u32, wzfiltersetstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsMediaTypeSupported<Impl: IMFSensorProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamid: u32, pmediatype: ::windows::core::RawPtr, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddBlockedControl<Impl: IMFSensorProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzblockedcontrol: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProfileId: GetProfileId::<Impl, IMPL_OFFSET>,
            AddProfileFilter: AddProfileFilter::<Impl, IMPL_OFFSET>,
            IsMediaTypeSupported: IsMediaTypeSupported::<Impl, IMPL_OFFSET>,
            AddBlockedControl: AddBlockedControl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorProfile as ::windows::core::Interface>::IID
    }
}
pub trait IMFSensorProfileCollectionImpl: Sized {
    fn GetProfileCount();
    fn GetProfile();
    fn AddProfile();
    fn FindProfile();
    fn RemoveProfileByIndex();
    fn RemoveProfile();
}
impl IMFSensorProfileCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorProfileCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorProfileCollectionVtbl {
        unsafe extern "system" fn GetProfileCount<Impl: IMFSensorProfileCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProfile<Impl: IMFSensorProfileCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddProfile<Impl: IMFSensorProfileCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindProfile<Impl: IMFSensorProfileCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profileid: *const SENSORPROFILEID, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveProfileByIndex<Impl: IMFSensorProfileCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveProfile<Impl: IMFSensorProfileCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profileid: *const SENSORPROFILEID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProfileCount: GetProfileCount::<Impl, IMPL_OFFSET>,
            GetProfile: GetProfile::<Impl, IMPL_OFFSET>,
            AddProfile: AddProfile::<Impl, IMPL_OFFSET>,
            FindProfile: FindProfile::<Impl, IMPL_OFFSET>,
            RemoveProfileByIndex: RemoveProfileByIndex::<Impl, IMPL_OFFSET>,
            RemoveProfile: RemoveProfile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorProfileCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFSensorStreamImpl: Sized + IMFAttributesImpl {
    fn GetMediaTypeCount();
    fn GetMediaType();
    fn CloneSensorStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFSensorStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorStreamVtbl {
        unsafe extern "system" fn GetMediaTypeCount<Impl: IMFSensorStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMediaType<Impl: IMFSensorStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloneSensorStream<Impl: IMFSensorStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMediaTypeCount: GetMediaTypeCount::<Impl, IMPL_OFFSET>,
            GetMediaType: GetMediaType::<Impl, IMPL_OFFSET>,
            CloneSensorStream: CloneSensorStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Streaming")]
pub trait IMFSensorTransformFactoryImpl: Sized {
    fn GetFactoryAttributes();
    fn InitializeFactory();
    fn GetTransformCount();
    fn GetTransformInformation();
    fn CreateTransform();
}
#[cfg(feature = "Win32_Media_Streaming")]
impl IMFSensorTransformFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSensorTransformFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSensorTransformFactoryVtbl {
        unsafe extern "system" fn GetFactoryAttributes<Impl: IMFSensorTransformFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFactory<Impl: IMFSensorTransformFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxtransformcount: u32, psensordevices: ::windows::core::RawPtr, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformCount<Impl: IMFSensorTransformFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformInformation<Impl: IMFSensorTransformFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformindex: u32, pguidtransformid: *mut ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr, ppstreaminformation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTransform<Impl: IMFSensorTransformFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidsensortransformid: *const ::windows::core::GUID, pattributes: ::windows::core::RawPtr, ppdevicemft: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFactoryAttributes: GetFactoryAttributes::<Impl, IMPL_OFFSET>,
            InitializeFactory: InitializeFactory::<Impl, IMPL_OFFSET>,
            GetTransformCount: GetTransformCount::<Impl, IMPL_OFFSET>,
            GetTransformInformation: GetTransformInformation::<Impl, IMPL_OFFSET>,
            CreateTransform: CreateTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSensorTransformFactory as ::windows::core::Interface>::IID
    }
}
pub trait IMFSequencerSourceImpl: Sized {
    fn AppendTopology();
    fn DeleteTopology();
    fn GetPresentationContext();
    fn UpdateTopology();
    fn UpdateTopologyFlags();
}
impl IMFSequencerSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSequencerSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSequencerSourceVtbl {
        unsafe extern "system" fn AppendTopology<Impl: IMFSequencerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: ::windows::core::RawPtr, dwflags: u32, pdwid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteTopology<Impl: IMFSequencerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentationContext<Impl: IMFSequencerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppd: ::windows::core::RawPtr, pid: *mut u32, pptopology: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateTopology<Impl: IMFSequencerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwid: u32, ptopology: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateTopologyFlags<Impl: IMFSequencerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwid: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AppendTopology: AppendTopology::<Impl, IMPL_OFFSET>,
            DeleteTopology: DeleteTopology::<Impl, IMPL_OFFSET>,
            GetPresentationContext: GetPresentationContext::<Impl, IMPL_OFFSET>,
            UpdateTopology: UpdateTopology::<Impl, IMPL_OFFSET>,
            UpdateTopologyFlags: UpdateTopologyFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSequencerSource as ::windows::core::Interface>::IID
    }
}
pub trait IMFSharingEngineClassFactoryImpl: Sized {
    fn CreateInstance();
}
impl IMFSharingEngineClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSharingEngineClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSharingEngineClassFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMFSharingEngineClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pattr: ::windows::core::RawPtr, ppengine: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSharingEngineClassFactory as ::windows::core::Interface>::IID
    }
}
pub trait IMFShutdownImpl: Sized {
    fn Shutdown();
    fn GetShutdownStatus();
}
impl IMFShutdownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFShutdownImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFShutdownVtbl {
        unsafe extern "system" fn Shutdown<Impl: IMFShutdownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetShutdownStatus<Impl: IMFShutdownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut MFSHUTDOWN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
            GetShutdownStatus: GetShutdownStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFShutdown as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFSignedLibraryImpl: Sized {
    fn GetProcedureAddress();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFSignedLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSignedLibraryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSignedLibraryVtbl {
        unsafe extern "system" fn GetProcedureAddress<Impl: IMFSignedLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, address: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetProcedureAddress: GetProcedureAddress::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSignedLibrary as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFSimpleAudioVolumeImpl: Sized {
    fn SetMasterVolume();
    fn GetMasterVolume();
    fn SetMute();
    fn GetMute();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFSimpleAudioVolumeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSimpleAudioVolumeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSimpleAudioVolumeVtbl {
        unsafe extern "system" fn SetMasterVolume<Impl: IMFSimpleAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flevel: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMasterVolume<Impl: IMFSimpleAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMute<Impl: IMFSimpleAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMute<Impl: IMFSimpleAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMasterVolume: SetMasterVolume::<Impl, IMPL_OFFSET>,
            GetMasterVolume: GetMasterVolume::<Impl, IMPL_OFFSET>,
            SetMute: SetMute::<Impl, IMPL_OFFSET>,
            GetMute: GetMute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSimpleAudioVolume as ::windows::core::Interface>::IID
    }
}
pub trait IMFSinkWriterImpl: Sized {
    fn AddStream();
    fn SetInputMediaType();
    fn BeginWriting();
    fn WriteSample();
    fn SendStreamTick();
    fn PlaceMarker();
    fn NotifyEndOfSegment();
    fn Flush();
    fn Finalize();
    fn GetServiceForStream();
    fn GetStatistics();
}
impl IMFSinkWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSinkWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSinkWriterVtbl {
        unsafe extern "system" fn AddStream<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetmediatype: ::windows::core::RawPtr, pdwstreamindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputMediaType<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pinputmediatype: ::windows::core::RawPtr, pencodingparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginWriting<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteSample<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendStreamTick<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, lltimestamp: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlaceMarker<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyEndOfSegment<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finalize<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetServiceForStream<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatistics<Impl: IMFSinkWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pstats: *mut MF_SINK_WRITER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddStream: AddStream::<Impl, IMPL_OFFSET>,
            SetInputMediaType: SetInputMediaType::<Impl, IMPL_OFFSET>,
            BeginWriting: BeginWriting::<Impl, IMPL_OFFSET>,
            WriteSample: WriteSample::<Impl, IMPL_OFFSET>,
            SendStreamTick: SendStreamTick::<Impl, IMPL_OFFSET>,
            PlaceMarker: PlaceMarker::<Impl, IMPL_OFFSET>,
            NotifyEndOfSegment: NotifyEndOfSegment::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            Finalize: Finalize::<Impl, IMPL_OFFSET>,
            GetServiceForStream: GetServiceForStream::<Impl, IMPL_OFFSET>,
            GetStatistics: GetStatistics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSinkWriter as ::windows::core::Interface>::IID
    }
}
pub trait IMFSinkWriterCallbackImpl: Sized {
    fn OnFinalize();
    fn OnMarker();
}
impl IMFSinkWriterCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSinkWriterCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSinkWriterCallbackVtbl {
        unsafe extern "system" fn OnFinalize<Impl: IMFSinkWriterCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnMarker<Impl: IMFSinkWriterCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnFinalize: OnFinalize::<Impl, IMPL_OFFSET>,
            OnMarker: OnMarker::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSinkWriterCallback as ::windows::core::Interface>::IID
    }
}
pub trait IMFSinkWriterCallback2Impl: Sized + IMFSinkWriterCallbackImpl {
    fn OnTransformChange();
    fn OnStreamError();
}
impl IMFSinkWriterCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSinkWriterCallback2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSinkWriterCallback2Vtbl {
        unsafe extern "system" fn OnTransformChange<Impl: IMFSinkWriterCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnStreamError<Impl: IMFSinkWriterCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFSinkWriterCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnTransformChange: OnTransformChange::<Impl, IMPL_OFFSET>,
            OnStreamError: OnStreamError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSinkWriterCallback2 as ::windows::core::Interface>::IID
    }
}
pub trait IMFSinkWriterEncoderConfigImpl: Sized {
    fn SetTargetMediaType();
    fn PlaceEncodingParameters();
}
impl IMFSinkWriterEncoderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSinkWriterEncoderConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSinkWriterEncoderConfigVtbl {
        unsafe extern "system" fn SetTargetMediaType<Impl: IMFSinkWriterEncoderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ptargetmediatype: ::windows::core::RawPtr, pencodingparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlaceEncodingParameters<Impl: IMFSinkWriterEncoderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pencodingparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetTargetMediaType: SetTargetMediaType::<Impl, IMPL_OFFSET>,
            PlaceEncodingParameters: PlaceEncodingParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSinkWriterEncoderConfig as ::windows::core::Interface>::IID
    }
}
pub trait IMFSinkWriterExImpl: Sized + IMFSinkWriterImpl {
    fn GetTransformForStream();
}
impl IMFSinkWriterExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSinkWriterExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSinkWriterExVtbl {
        unsafe extern "system" fn GetTransformForStream<Impl: IMFSinkWriterExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwtransformindex: u32, pguidcategory: *mut ::windows::core::GUID, pptransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IMFSinkWriterVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetTransformForStream: GetTransformForStream::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSinkWriterEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFSourceBufferImpl: Sized {
    fn GetUpdating();
    fn GetBuffered();
    fn GetTimeStampOffset();
    fn SetTimeStampOffset();
    fn GetAppendWindowStart();
    fn SetAppendWindowStart();
    fn GetAppendWindowEnd();
    fn SetAppendWindowEnd();
    fn Append();
    fn AppendByteStream();
    fn Abort();
    fn Remove();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFSourceBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSourceBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSourceBufferVtbl {
        unsafe extern "system" fn GetUpdating<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBuffered<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuffered: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimeStampOffset<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTimeStampOffset<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAppendWindowStart<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAppendWindowStart<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAppendWindowEnd<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAppendWindowEnd<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, len: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppendByteStream<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pmaxlen: *const u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abort<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IMFSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: f64, end: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetUpdating: GetUpdating::<Impl, IMPL_OFFSET>,
            GetBuffered: GetBuffered::<Impl, IMPL_OFFSET>,
            GetTimeStampOffset: GetTimeStampOffset::<Impl, IMPL_OFFSET>,
            SetTimeStampOffset: SetTimeStampOffset::<Impl, IMPL_OFFSET>,
            GetAppendWindowStart: GetAppendWindowStart::<Impl, IMPL_OFFSET>,
            SetAppendWindowStart: SetAppendWindowStart::<Impl, IMPL_OFFSET>,
            GetAppendWindowEnd: GetAppendWindowEnd::<Impl, IMPL_OFFSET>,
            SetAppendWindowEnd: SetAppendWindowEnd::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            AppendByteStream: AppendByteStream::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSourceBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IMFSourceBufferAppendModeImpl: Sized {
    fn GetAppendMode();
    fn SetAppendMode();
}
impl IMFSourceBufferAppendModeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSourceBufferAppendModeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSourceBufferAppendModeVtbl {
        unsafe extern "system" fn GetAppendMode<Impl: IMFSourceBufferAppendModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> MF_MSE_APPEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAppendMode<Impl: IMFSourceBufferAppendModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: MF_MSE_APPEND_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAppendMode: GetAppendMode::<Impl, IMPL_OFFSET>,
            SetAppendMode: SetAppendMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSourceBufferAppendMode as ::windows::core::Interface>::IID
    }
}
pub trait IMFSourceBufferListImpl: Sized {
    fn GetLength();
    fn GetSourceBuffer();
}
impl IMFSourceBufferListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSourceBufferListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSourceBufferListVtbl {
        unsafe extern "system" fn GetLength<Impl: IMFSourceBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceBuffer<Impl: IMFSourceBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<IMFSourceBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLength: GetLength::<Impl, IMPL_OFFSET>,
            GetSourceBuffer: GetSourceBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSourceBufferList as ::windows::core::Interface>::IID
    }
}
pub trait IMFSourceBufferNotifyImpl: Sized {
    fn OnUpdateStart();
    fn OnAbort();
    fn OnError();
    fn OnUpdate();
    fn OnUpdateEnd();
}
impl IMFSourceBufferNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSourceBufferNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSourceBufferNotifyVtbl {
        unsafe extern "system" fn OnUpdateStart<Impl: IMFSourceBufferNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnAbort<Impl: IMFSourceBufferNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnError<Impl: IMFSourceBufferNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnUpdate<Impl: IMFSourceBufferNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnUpdateEnd<Impl: IMFSourceBufferNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnUpdateStart: OnUpdateStart::<Impl, IMPL_OFFSET>,
            OnAbort: OnAbort::<Impl, IMPL_OFFSET>,
            OnError: OnError::<Impl, IMPL_OFFSET>,
            OnUpdate: OnUpdate::<Impl, IMPL_OFFSET>,
            OnUpdateEnd: OnUpdateEnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSourceBufferNotify as ::windows::core::Interface>::IID
    }
}
pub trait IMFSourceOpenMonitorImpl: Sized {
    fn OnSourceEvent();
}
impl IMFSourceOpenMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSourceOpenMonitorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSourceOpenMonitorVtbl {
        unsafe extern "system" fn OnSourceEvent<Impl: IMFSourceOpenMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnSourceEvent: OnSourceEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSourceOpenMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFSourceReaderImpl: Sized {
    fn GetStreamSelection();
    fn SetStreamSelection();
    fn GetNativeMediaType();
    fn GetCurrentMediaType();
    fn SetCurrentMediaType();
    fn SetCurrentPosition();
    fn ReadSample();
    fn Flush();
    fn GetServiceForStream();
    fn GetPresentationAttribute();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFSourceReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSourceReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSourceReaderVtbl {
        unsafe extern "system" fn GetStreamSelection<Impl: IMFSourceReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pfselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamSelection<Impl: IMFSourceReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, fselected: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNativeMediaType<Impl: IMFSourceReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwmediatypeindex: u32, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentMediaType<Impl: IMFSourceReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentMediaType<Impl: IMFSourceReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pdwreserved: *mut u32, pmediatype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentPosition<Impl: IMFSourceReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtimeformat: *const ::windows::core::GUID, varposition: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadSample<Impl: IMFSourceReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwcontrolflags: u32, pdwactualstreamindex: *mut u32, pdwstreamflags: *mut u32, plltimestamp: *mut i64, ppsample: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IMFSourceReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetServiceForStream<Impl: IMFSourceReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentationAttribute<Impl: IMFSourceReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, guidattribute: *const ::windows::core::GUID, pvarattribute: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreamSelection: GetStreamSelection::<Impl, IMPL_OFFSET>,
            SetStreamSelection: SetStreamSelection::<Impl, IMPL_OFFSET>,
            GetNativeMediaType: GetNativeMediaType::<Impl, IMPL_OFFSET>,
            GetCurrentMediaType: GetCurrentMediaType::<Impl, IMPL_OFFSET>,
            SetCurrentMediaType: SetCurrentMediaType::<Impl, IMPL_OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Impl, IMPL_OFFSET>,
            ReadSample: ReadSample::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            GetServiceForStream: GetServiceForStream::<Impl, IMPL_OFFSET>,
            GetPresentationAttribute: GetPresentationAttribute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSourceReader as ::windows::core::Interface>::IID
    }
}
pub trait IMFSourceReaderCallbackImpl: Sized {
    fn OnReadSample();
    fn OnFlush();
    fn OnEvent();
}
impl IMFSourceReaderCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSourceReaderCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSourceReaderCallbackVtbl {
        unsafe extern "system" fn OnReadSample<Impl: IMFSourceReaderCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, dwstreamindex: u32, dwstreamflags: u32, lltimestamp: i64, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnFlush<Impl: IMFSourceReaderCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEvent<Impl: IMFSourceReaderCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnReadSample: OnReadSample::<Impl, IMPL_OFFSET>,
            OnFlush: OnFlush::<Impl, IMPL_OFFSET>,
            OnEvent: OnEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSourceReaderCallback as ::windows::core::Interface>::IID
    }
}
pub trait IMFSourceReaderCallback2Impl: Sized + IMFSourceReaderCallbackImpl {
    fn OnTransformChange();
    fn OnStreamError();
}
impl IMFSourceReaderCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSourceReaderCallback2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSourceReaderCallback2Vtbl {
        unsafe extern "system" fn OnTransformChange<Impl: IMFSourceReaderCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnStreamError<Impl: IMFSourceReaderCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFSourceReaderCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnTransformChange: OnTransformChange::<Impl, IMPL_OFFSET>,
            OnStreamError: OnStreamError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSourceReaderCallback2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFSourceReaderExImpl: Sized + IMFSourceReaderImpl {
    fn SetNativeMediaType();
    fn AddTransformForStream();
    fn RemoveAllTransformsForStream();
    fn GetTransformForStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFSourceReaderExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSourceReaderExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSourceReaderExVtbl {
        unsafe extern "system" fn SetNativeMediaType<Impl: IMFSourceReaderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, pmediatype: ::windows::core::RawPtr, pdwstreamflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTransformForStream<Impl: IMFSourceReaderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ptransformoractivate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllTransformsForStream<Impl: IMFSourceReaderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformForStream<Impl: IMFSourceReaderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwtransformindex: u32, pguidcategory: *mut ::windows::core::GUID, pptransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFSourceReaderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetNativeMediaType: SetNativeMediaType::<Impl, IMPL_OFFSET>,
            AddTransformForStream: AddTransformForStream::<Impl, IMPL_OFFSET>,
            RemoveAllTransformsForStream: RemoveAllTransformsForStream::<Impl, IMPL_OFFSET>,
            GetTransformForStream: GetTransformForStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSourceReaderEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMFSourceResolverImpl: Sized {
    fn CreateObjectFromURL();
    fn CreateObjectFromByteStream();
    fn BeginCreateObjectFromURL();
    fn EndCreateObjectFromURL();
    fn BeginCreateObjectFromByteStream();
    fn EndCreateObjectFromByteStream();
    fn CancelObjectCreation();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMFSourceResolverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSourceResolverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSourceResolverVtbl {
        unsafe extern "system" fn CreateObjectFromURL<Impl: IMFSourceResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwflags: u32, pprops: ::windows::core::RawPtr, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateObjectFromByteStream<Impl: IMFSourceResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytestream: ::windows::core::RawPtr, pwszurl: super::super::Foundation::PWSTR, dwflags: u32, pprops: ::windows::core::RawPtr, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginCreateObjectFromURL<Impl: IMFSourceResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwflags: u32, pprops: ::windows::core::RawPtr, ppiunknowncancelcookie: *mut *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndCreateObjectFromURL<Impl: IMFSourceResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginCreateObjectFromByteStream<Impl: IMFSourceResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytestream: ::windows::core::RawPtr, pwszurl: super::super::Foundation::PWSTR, dwflags: u32, pprops: ::windows::core::RawPtr, ppiunknowncancelcookie: *mut *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndCreateObjectFromByteStream<Impl: IMFSourceResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelObjectCreation<Impl: IMFSourceResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piunknowncancelcookie: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateObjectFromURL: CreateObjectFromURL::<Impl, IMPL_OFFSET>,
            CreateObjectFromByteStream: CreateObjectFromByteStream::<Impl, IMPL_OFFSET>,
            BeginCreateObjectFromURL: BeginCreateObjectFromURL::<Impl, IMPL_OFFSET>,
            EndCreateObjectFromURL: EndCreateObjectFromURL::<Impl, IMPL_OFFSET>,
            BeginCreateObjectFromByteStream: BeginCreateObjectFromByteStream::<Impl, IMPL_OFFSET>,
            EndCreateObjectFromByteStream: EndCreateObjectFromByteStream::<Impl, IMPL_OFFSET>,
            CancelObjectCreation: CancelObjectCreation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSourceResolver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMFSpatialAudioObjectBufferImpl: Sized + IMFMediaBufferImpl {
    fn SetID();
    fn GetID();
    fn SetType();
    fn GetType();
    fn GetMetadataItems();
}
#[cfg(feature = "Win32_Media_Audio")]
impl IMFSpatialAudioObjectBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSpatialAudioObjectBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSpatialAudioObjectBufferVtbl {
        unsafe extern "system" fn SetID<Impl: IMFSpatialAudioObjectBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetID<Impl: IMFSpatialAudioObjectBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetType<Impl: IMFSpatialAudioObjectBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: super::Audio::AudioObjectType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IMFSpatialAudioObjectBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut super::Audio::AudioObjectType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetadataItems<Impl: IMFSpatialAudioObjectBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadataitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaBufferVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetID: SetID::<Impl, IMPL_OFFSET>,
            GetID: GetID::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetMetadataItems: GetMetadataItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSpatialAudioObjectBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFSpatialAudioSampleImpl: Sized + IMFAttributesImpl + IMFSampleImpl {
    fn GetObjectCount();
    fn AddSpatialAudioObject();
    fn GetSpatialAudioObjectByIndex();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFSpatialAudioSampleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSpatialAudioSampleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSpatialAudioSampleVtbl {
        unsafe extern "system" fn GetObjectCount<Impl: IMFSpatialAudioSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwobjectcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSpatialAudioObject<Impl: IMFSpatialAudioSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudioobjbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpatialAudioObjectByIndex<Impl: IMFSpatialAudioSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppaudioobjbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFSampleVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetObjectCount: GetObjectCount::<Impl, IMPL_OFFSET>,
            AddSpatialAudioObject: AddSpatialAudioObject::<Impl, IMPL_OFFSET>,
            GetSpatialAudioObjectByIndex: GetSpatialAudioObjectByIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSpatialAudioSample as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFStreamDescriptorImpl: Sized + IMFAttributesImpl {
    fn GetStreamIdentifier();
    fn GetMediaTypeHandler();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFStreamDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFStreamDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFStreamDescriptorVtbl {
        unsafe extern "system" fn GetStreamIdentifier<Impl: IMFStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstreamidentifier: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMediaTypeHandler<Impl: IMFStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediatypehandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStreamIdentifier: GetStreamIdentifier::<Impl, IMPL_OFFSET>,
            GetMediaTypeHandler: GetMediaTypeHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFStreamDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFStreamSinkImpl: Sized + IMFMediaEventGeneratorImpl {
    fn GetMediaSink();
    fn GetIdentifier();
    fn GetMediaTypeHandler();
    fn ProcessSample();
    fn PlaceMarker();
    fn Flush();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFStreamSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFStreamSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFStreamSinkVtbl {
        unsafe extern "system" fn GetMediaSink<Impl: IMFStreamSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediasink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIdentifier<Impl: IMFStreamSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwidentifier: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMediaTypeHandler<Impl: IMFStreamSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessSample<Impl: IMFStreamSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlaceMarker<Impl: IMFStreamSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emarkertype: MFSTREAMSINK_MARKER_TYPE, pvarmarkervalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarcontextvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IMFStreamSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaEventGeneratorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMediaSink: GetMediaSink::<Impl, IMPL_OFFSET>,
            GetIdentifier: GetIdentifier::<Impl, IMPL_OFFSET>,
            GetMediaTypeHandler: GetMediaTypeHandler::<Impl, IMPL_OFFSET>,
            ProcessSample: ProcessSample::<Impl, IMPL_OFFSET>,
            PlaceMarker: PlaceMarker::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFStreamSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFStreamingSinkConfigImpl: Sized {
    fn StartStreaming();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFStreamingSinkConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFStreamingSinkConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFStreamingSinkConfigVtbl {
        unsafe extern "system" fn StartStreaming<Impl: IMFStreamingSinkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fseekoffsetisbyteoffset: super::super::Foundation::BOOL, qwseekoffset: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), StartStreaming: StartStreaming::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFStreamingSinkConfig as ::windows::core::Interface>::IID
    }
}
pub trait IMFSystemIdImpl: Sized {
    fn GetData();
    fn Setup();
}
impl IMFSystemIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFSystemIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFSystemIdVtbl {
        unsafe extern "system" fn GetData<Impl: IMFSystemIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32, data: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setup<Impl: IMFSystemIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, cbin: u32, pbin: *const u8, pcbout: *mut u32, ppbout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetData: GetData::<Impl, IMPL_OFFSET>, Setup: Setup::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFSystemId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFTimecodeTranslateImpl: Sized {
    fn BeginConvertTimecodeToHNS();
    fn EndConvertTimecodeToHNS();
    fn BeginConvertHNSToTimecode();
    fn EndConvertHNSToTimecode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFTimecodeTranslateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimecodeTranslateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimecodeTranslateVtbl {
        unsafe extern "system" fn BeginConvertTimecodeToHNS<Impl: IMFTimecodeTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvartimecode: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndConvertTimecodeToHNS<Impl: IMFTimecodeTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, phnstime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginConvertHNSToTimecode<Impl: IMFTimecodeTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnstime: i64, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndConvertHNSToTimecode<Impl: IMFTimecodeTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, ppropvartimecode: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginConvertTimecodeToHNS: BeginConvertTimecodeToHNS::<Impl, IMPL_OFFSET>,
            EndConvertTimecodeToHNS: EndConvertTimecodeToHNS::<Impl, IMPL_OFFSET>,
            BeginConvertHNSToTimecode: BeginConvertHNSToTimecode::<Impl, IMPL_OFFSET>,
            EndConvertHNSToTimecode: EndConvertHNSToTimecode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimecodeTranslate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTimedTextImpl: Sized {
    fn RegisterNotifications();
    fn SelectTrack();
    fn AddDataSource();
    fn AddDataSourceFromUrl();
    fn AddTrack();
    fn RemoveTrack();
    fn GetCueTimeOffset();
    fn SetCueTimeOffset();
    fn GetTracks();
    fn GetActiveTracks();
    fn GetTextTracks();
    fn GetMetadataTracks();
    fn SetInBandEnabled();
    fn IsInBandEnabled();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTimedTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextVtbl {
        unsafe extern "system" fn RegisterNotifications<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectTrack<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackid: u32, selected: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDataSource<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytestream: ::windows::core::RawPtr, label: super::super::Foundation::PWSTR, language: super::super::Foundation::PWSTR, kind: MF_TIMED_TEXT_TRACK_KIND, isdefault: super::super::Foundation::BOOL, trackid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDataSourceFromUrl<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, url: super::super::Foundation::PWSTR, label: super::super::Foundation::PWSTR, language: super::super::Foundation::PWSTR, kind: MF_TIMED_TEXT_TRACK_KIND, isdefault: super::super::Foundation::BOOL, trackid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTrack<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, label: super::super::Foundation::PWSTR, language: super::super::Foundation::PWSTR, kind: MF_TIMED_TEXT_TRACK_KIND, track: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveTrack<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, track: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCueTimeOffset<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCueTimeOffset<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTracks<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActiveTracks<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activetracks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextTracks<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texttracks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetadataTracks<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadatatracks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInBandEnabled<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsInBandEnabled<Impl: IMFTimedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterNotifications: RegisterNotifications::<Impl, IMPL_OFFSET>,
            SelectTrack: SelectTrack::<Impl, IMPL_OFFSET>,
            AddDataSource: AddDataSource::<Impl, IMPL_OFFSET>,
            AddDataSourceFromUrl: AddDataSourceFromUrl::<Impl, IMPL_OFFSET>,
            AddTrack: AddTrack::<Impl, IMPL_OFFSET>,
            RemoveTrack: RemoveTrack::<Impl, IMPL_OFFSET>,
            GetCueTimeOffset: GetCueTimeOffset::<Impl, IMPL_OFFSET>,
            SetCueTimeOffset: SetCueTimeOffset::<Impl, IMPL_OFFSET>,
            GetTracks: GetTracks::<Impl, IMPL_OFFSET>,
            GetActiveTracks: GetActiveTracks::<Impl, IMPL_OFFSET>,
            GetTextTracks: GetTextTracks::<Impl, IMPL_OFFSET>,
            GetMetadataTracks: GetMetadataTracks::<Impl, IMPL_OFFSET>,
            SetInBandEnabled: SetInBandEnabled::<Impl, IMPL_OFFSET>,
            IsInBandEnabled: IsInBandEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedText as ::windows::core::Interface>::IID
    }
}
pub trait IMFTimedTextBinaryImpl: Sized {
    fn GetData();
}
impl IMFTimedTextBinaryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextBinaryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextBinaryVtbl {
        unsafe extern "system" fn GetData<Impl: IMFTimedTextBinaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetData: GetData::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextBinary as ::windows::core::Interface>::IID
    }
}
pub trait IMFTimedTextBoutenImpl: Sized {
    fn GetBoutenType();
    fn GetBoutenColor();
    fn GetBoutenPosition();
}
impl IMFTimedTextBoutenVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextBoutenImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextBoutenVtbl {
        unsafe extern "system" fn GetBoutenType<Impl: IMFTimedTextBoutenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut MF_TIMED_TEXT_BOUTEN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoutenColor<Impl: IMFTimedTextBoutenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut MFARGB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoutenPosition<Impl: IMFTimedTextBoutenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut MF_TIMED_TEXT_BOUTEN_POSITION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBoutenType: GetBoutenType::<Impl, IMPL_OFFSET>,
            GetBoutenColor: GetBoutenColor::<Impl, IMPL_OFFSET>,
            GetBoutenPosition: GetBoutenPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextBouten as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTimedTextCueImpl: Sized {
    fn GetId();
    fn GetOriginalId();
    fn GetCueKind();
    fn GetStartTime();
    fn GetDuration();
    fn GetTrackId();
    fn GetData();
    fn GetRegion();
    fn GetStyle();
    fn GetLineCount();
    fn GetLine();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTimedTextCueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextCueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextCueVtbl {
        unsafe extern "system" fn GetId<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOriginalId<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, originalid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCueKind<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> MF_TIMED_TEXT_TRACK_KIND {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStartTime<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuration<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrackId<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetData<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegion<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, region: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStyle<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, style: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineCount<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLine<Impl: IMFTimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, line: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetId: GetId::<Impl, IMPL_OFFSET>,
            GetOriginalId: GetOriginalId::<Impl, IMPL_OFFSET>,
            GetCueKind: GetCueKind::<Impl, IMPL_OFFSET>,
            GetStartTime: GetStartTime::<Impl, IMPL_OFFSET>,
            GetDuration: GetDuration::<Impl, IMPL_OFFSET>,
            GetTrackId: GetTrackId::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
            GetRegion: GetRegion::<Impl, IMPL_OFFSET>,
            GetStyle: GetStyle::<Impl, IMPL_OFFSET>,
            GetLineCount: GetLineCount::<Impl, IMPL_OFFSET>,
            GetLine: GetLine::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextCue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTimedTextCueListImpl: Sized {
    fn GetLength();
    fn GetCueByIndex();
    fn GetCueById();
    fn GetCueByOriginalId();
    fn AddTextCue();
    fn AddDataCue();
    fn RemoveCue();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTimedTextCueListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextCueListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextCueListVtbl {
        unsafe extern "system" fn GetLength<Impl: IMFTimedTextCueListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCueByIndex<Impl: IMFTimedTextCueListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, cue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCueById<Impl: IMFTimedTextCueListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, cue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCueByOriginalId<Impl: IMFTimedTextCueListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, originalid: super::super::Foundation::PWSTR, cue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTextCue<Impl: IMFTimedTextCueListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: f64, duration: f64, text: super::super::Foundation::PWSTR, cue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDataCue<Impl: IMFTimedTextCueListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: f64, duration: f64, data: *const u8, datasize: u32, cue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveCue<Impl: IMFTimedTextCueListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLength: GetLength::<Impl, IMPL_OFFSET>,
            GetCueByIndex: GetCueByIndex::<Impl, IMPL_OFFSET>,
            GetCueById: GetCueById::<Impl, IMPL_OFFSET>,
            GetCueByOriginalId: GetCueByOriginalId::<Impl, IMPL_OFFSET>,
            AddTextCue: AddTextCue::<Impl, IMPL_OFFSET>,
            AddDataCue: AddDataCue::<Impl, IMPL_OFFSET>,
            RemoveCue: RemoveCue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextCueList as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTimedTextFormattedTextImpl: Sized {
    fn GetText();
    fn GetSubformattingCount();
    fn GetSubformatting();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTimedTextFormattedTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextFormattedTextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextFormattedTextVtbl {
        unsafe extern "system" fn GetText<Impl: IMFTimedTextFormattedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubformattingCount<Impl: IMFTimedTextFormattedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubformatting<Impl: IMFTimedTextFormattedTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, firstchar: *mut u32, charlength: *mut u32, style: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetText: GetText::<Impl, IMPL_OFFSET>,
            GetSubformattingCount: GetSubformattingCount::<Impl, IMPL_OFFSET>,
            GetSubformatting: GetSubformatting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextFormattedText as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTimedTextNotifyImpl: Sized {
    fn TrackAdded();
    fn TrackRemoved();
    fn TrackSelected();
    fn TrackReadyStateChanged();
    fn Error();
    fn Cue();
    fn Reset();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTimedTextNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextNotifyVtbl {
        unsafe extern "system" fn TrackAdded<Impl: IMFTimedTextNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackid: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrackRemoved<Impl: IMFTimedTextNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackid: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrackSelected<Impl: IMFTimedTextNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackid: u32, selected: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrackReadyStateChanged<Impl: IMFTimedTextNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackid: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: IMFTimedTextNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: MF_TIMED_TEXT_ERROR_CODE, extendederrorcode: ::windows::core::HRESULT, sourcetrackid: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cue<Impl: IMFTimedTextNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cueevent: MF_TIMED_TEXT_CUE_EVENT, currenttime: f64, cue: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IMFTimedTextNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TrackAdded: TrackAdded::<Impl, IMPL_OFFSET>,
            TrackRemoved: TrackRemoved::<Impl, IMPL_OFFSET>,
            TrackSelected: TrackSelected::<Impl, IMPL_OFFSET>,
            TrackReadyStateChanged: TrackReadyStateChanged::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
            Cue: Cue::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTimedTextRegionImpl: Sized {
    fn GetName();
    fn GetPosition();
    fn GetExtent();
    fn GetBackgroundColor();
    fn GetWritingMode();
    fn GetDisplayAlignment();
    fn GetLineHeight();
    fn GetClipOverflow();
    fn GetPadding();
    fn GetWrap();
    fn GetZIndex();
    fn GetScrollMode();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTimedTextRegionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextRegionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextRegionVtbl {
        unsafe extern "system" fn GetName<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPosition<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, px: *mut f64, py: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtent<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut f64, pheight: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackgroundColor<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bgcolor: *mut MFARGB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWritingMode<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writingmode: *mut MF_TIMED_TEXT_WRITING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayAlignment<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayalign: *mut MF_TIMED_TEXT_DISPLAY_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineHeight<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plineheight: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipOverflow<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipoverflow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPadding<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: *mut f64, start: *mut f64, after: *mut f64, end: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWrap<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrap: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetZIndex<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScrollMode<Impl: IMFTimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollmode: *mut MF_TIMED_TEXT_SCROLL_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
            GetExtent: GetExtent::<Impl, IMPL_OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Impl, IMPL_OFFSET>,
            GetWritingMode: GetWritingMode::<Impl, IMPL_OFFSET>,
            GetDisplayAlignment: GetDisplayAlignment::<Impl, IMPL_OFFSET>,
            GetLineHeight: GetLineHeight::<Impl, IMPL_OFFSET>,
            GetClipOverflow: GetClipOverflow::<Impl, IMPL_OFFSET>,
            GetPadding: GetPadding::<Impl, IMPL_OFFSET>,
            GetWrap: GetWrap::<Impl, IMPL_OFFSET>,
            GetZIndex: GetZIndex::<Impl, IMPL_OFFSET>,
            GetScrollMode: GetScrollMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextRegion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTimedTextRubyImpl: Sized {
    fn GetRubyText();
    fn GetRubyPosition();
    fn GetRubyAlign();
    fn GetRubyReserve();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTimedTextRubyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextRubyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextRubyVtbl {
        unsafe extern "system" fn GetRubyText<Impl: IMFTimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rubytext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRubyPosition<Impl: IMFTimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut MF_TIMED_TEXT_RUBY_POSITION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRubyAlign<Impl: IMFTimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut MF_TIMED_TEXT_RUBY_ALIGN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRubyReserve<Impl: IMFTimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut MF_TIMED_TEXT_RUBY_RESERVE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRubyText: GetRubyText::<Impl, IMPL_OFFSET>,
            GetRubyPosition: GetRubyPosition::<Impl, IMPL_OFFSET>,
            GetRubyAlign: GetRubyAlign::<Impl, IMPL_OFFSET>,
            GetRubyReserve: GetRubyReserve::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextRuby as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTimedTextStyleImpl: Sized {
    fn GetName();
    fn IsExternal();
    fn GetFontFamily();
    fn GetFontSize();
    fn GetColor();
    fn GetBackgroundColor();
    fn GetShowBackgroundAlways();
    fn GetFontStyle();
    fn GetBold();
    fn GetRightToLeft();
    fn GetTextAlignment();
    fn GetTextDecoration();
    fn GetTextOutline();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTimedTextStyleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextStyleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextStyleVtbl {
        unsafe extern "system" fn GetName<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsExternal<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFamily<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamily: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontSize<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsize: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColor<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut MFARGB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackgroundColor<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bgcolor: *mut MFARGB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetShowBackgroundAlways<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, showbackgroundalways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontStyle<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontstyle: *mut MF_TIMED_TEXT_FONT_STYLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBold<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bold: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRightToLeft<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, righttoleft: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextAlignment<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textalign: *mut MF_TIMED_TEXT_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextDecoration<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textdecoration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextOutline<Impl: IMFTimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut MFARGB, thickness: *mut f64, blurradius: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            IsExternal: IsExternal::<Impl, IMPL_OFFSET>,
            GetFontFamily: GetFontFamily::<Impl, IMPL_OFFSET>,
            GetFontSize: GetFontSize::<Impl, IMPL_OFFSET>,
            GetColor: GetColor::<Impl, IMPL_OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Impl, IMPL_OFFSET>,
            GetShowBackgroundAlways: GetShowBackgroundAlways::<Impl, IMPL_OFFSET>,
            GetFontStyle: GetFontStyle::<Impl, IMPL_OFFSET>,
            GetBold: GetBold::<Impl, IMPL_OFFSET>,
            GetRightToLeft: GetRightToLeft::<Impl, IMPL_OFFSET>,
            GetTextAlignment: GetTextAlignment::<Impl, IMPL_OFFSET>,
            GetTextDecoration: GetTextDecoration::<Impl, IMPL_OFFSET>,
            GetTextOutline: GetTextOutline::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextStyle as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTimedTextStyle2Impl: Sized {
    fn GetRuby();
    fn GetBouten();
    fn IsTextCombined();
    fn GetFontAngleInDegrees();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTimedTextStyle2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextStyle2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextStyle2Vtbl {
        unsafe extern "system" fn GetRuby<Impl: IMFTimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruby: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBouten<Impl: IMFTimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bouten: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTextCombined<Impl: IMFTimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAngleInDegrees<Impl: IMFTimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRuby: GetRuby::<Impl, IMPL_OFFSET>,
            GetBouten: GetBouten::<Impl, IMPL_OFFSET>,
            IsTextCombined: IsTextCombined::<Impl, IMPL_OFFSET>,
            GetFontAngleInDegrees: GetFontAngleInDegrees::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextStyle2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTimedTextTrackImpl: Sized {
    fn GetId();
    fn GetLabel();
    fn SetLabel();
    fn GetLanguage();
    fn GetTrackKind();
    fn IsInBand();
    fn GetInBandMetadataTrackDispatchType();
    fn IsActive();
    fn GetErrorCode();
    fn GetExtendedErrorCode();
    fn GetDataFormat();
    fn GetReadyState();
    fn GetCueList();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTimedTextTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextTrackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextTrackVtbl {
        unsafe extern "system" fn GetId<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLabel<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, label: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLabel<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, label: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLanguage<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrackKind<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> MF_TIMED_TEXT_TRACK_KIND {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsInBand<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInBandMetadataTrackDispatchType<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispatchtype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsActive<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorCode<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> MF_TIMED_TEXT_ERROR_CODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendedErrorCode<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataFormat<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReadyState<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> MF_TIMED_TEXT_TRACK_READY_STATE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCueList<Impl: IMFTimedTextTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetId: GetId::<Impl, IMPL_OFFSET>,
            GetLabel: GetLabel::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            GetLanguage: GetLanguage::<Impl, IMPL_OFFSET>,
            GetTrackKind: GetTrackKind::<Impl, IMPL_OFFSET>,
            IsInBand: IsInBand::<Impl, IMPL_OFFSET>,
            GetInBandMetadataTrackDispatchType: GetInBandMetadataTrackDispatchType::<Impl, IMPL_OFFSET>,
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            GetErrorCode: GetErrorCode::<Impl, IMPL_OFFSET>,
            GetExtendedErrorCode: GetExtendedErrorCode::<Impl, IMPL_OFFSET>,
            GetDataFormat: GetDataFormat::<Impl, IMPL_OFFSET>,
            GetReadyState: GetReadyState::<Impl, IMPL_OFFSET>,
            GetCueList: GetCueList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextTrack as ::windows::core::Interface>::IID
    }
}
pub trait IMFTimedTextTrackListImpl: Sized {
    fn GetLength();
    fn GetTrack();
    fn GetTrackById();
}
impl IMFTimedTextTrackListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimedTextTrackListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimedTextTrackListVtbl {
        unsafe extern "system" fn GetLength<Impl: IMFTimedTextTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrack<Impl: IMFTimedTextTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, track: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrackById<Impl: IMFTimedTextTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackid: u32, track: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLength: GetLength::<Impl, IMPL_OFFSET>,
            GetTrack: GetTrack::<Impl, IMPL_OFFSET>,
            GetTrackById: GetTrackById::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimedTextTrackList as ::windows::core::Interface>::IID
    }
}
pub trait IMFTimerImpl: Sized {
    fn SetTimer();
    fn CancelTimer();
}
impl IMFTimerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTimerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTimerVtbl {
        unsafe extern "system" fn SetTimer<Impl: IMFTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, llclocktime: i64, pcallback: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void, ppunkkey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelTimer<Impl: IMFTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkkey: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetTimer: SetTimer::<Impl, IMPL_OFFSET>,
            CancelTimer: CancelTimer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTimer as ::windows::core::Interface>::IID
    }
}
pub trait IMFTopoLoaderImpl: Sized {
    fn Load();
}
impl IMFTopoLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTopoLoaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTopoLoaderVtbl {
        unsafe extern "system" fn Load<Impl: IMFTopoLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputtopo: ::windows::core::RawPtr, ppoutputtopo: *mut ::windows::core::RawPtr, pcurrenttopo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Load: Load::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTopoLoader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFTopologyImpl: Sized + IMFAttributesImpl {
    fn GetTopologyID();
    fn AddNode();
    fn RemoveNode();
    fn GetNodeCount();
    fn GetNode();
    fn Clear();
    fn CloneFrom();
    fn GetNodeByID();
    fn GetSourceNodeCollection();
    fn GetOutputNodeCollection();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFTopologyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTopologyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTopologyVtbl {
        unsafe extern "system" fn GetTopologyID<Impl: IMFTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddNode<Impl: IMFTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveNode<Impl: IMFTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNodeCount<Impl: IMFTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwnodes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNode<Impl: IMFTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IMFTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloneFrom<Impl: IMFTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNodeByID<Impl: IMFTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwtoponodeid: u64, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceNodeCollection<Impl: IMFTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputNodeCollection<Impl: IMFTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetTopologyID: GetTopologyID::<Impl, IMPL_OFFSET>,
            AddNode: AddNode::<Impl, IMPL_OFFSET>,
            RemoveNode: RemoveNode::<Impl, IMPL_OFFSET>,
            GetNodeCount: GetNodeCount::<Impl, IMPL_OFFSET>,
            GetNode: GetNode::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            CloneFrom: CloneFrom::<Impl, IMPL_OFFSET>,
            GetNodeByID: GetNodeByID::<Impl, IMPL_OFFSET>,
            GetSourceNodeCollection: GetSourceNodeCollection::<Impl, IMPL_OFFSET>,
            GetOutputNodeCollection: GetOutputNodeCollection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTopology as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFTopologyNodeImpl: Sized + IMFAttributesImpl {
    fn SetObject();
    fn GetObject();
    fn GetNodeType();
    fn GetTopoNodeID();
    fn SetTopoNodeID();
    fn GetInputCount();
    fn GetOutputCount();
    fn ConnectOutput();
    fn DisconnectOutput();
    fn GetInput();
    fn GetOutput();
    fn SetOutputPrefType();
    fn GetOutputPrefType();
    fn SetInputPrefType();
    fn GetInputPrefType();
    fn CloneFrom();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFTopologyNodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTopologyNodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTopologyNodeVtbl {
        unsafe extern "system" fn SetObject<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNodeType<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut MF_TOPOLOGY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTopoNodeID<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTopoNodeID<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulltopoid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputCount<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputCount<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectOutput<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputindex: u32, pdownstreamnode: ::windows::core::RawPtr, dwinputindexondownstreamnode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectOutput<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInput<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputindex: u32, ppupstreamnode: *mut ::windows::core::RawPtr, pdwoutputindexonupstreamnode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutput<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputindex: u32, ppdownstreamnode: *mut ::windows::core::RawPtr, pdwinputindexondownstreamnode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputPrefType<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputindex: u32, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputPrefType<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputindex: u32, pptype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputPrefType<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputindex: u32, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputPrefType<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputindex: u32, pptype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloneFrom<Impl: IMFTopologyNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetObject: SetObject::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
            GetNodeType: GetNodeType::<Impl, IMPL_OFFSET>,
            GetTopoNodeID: GetTopoNodeID::<Impl, IMPL_OFFSET>,
            SetTopoNodeID: SetTopoNodeID::<Impl, IMPL_OFFSET>,
            GetInputCount: GetInputCount::<Impl, IMPL_OFFSET>,
            GetOutputCount: GetOutputCount::<Impl, IMPL_OFFSET>,
            ConnectOutput: ConnectOutput::<Impl, IMPL_OFFSET>,
            DisconnectOutput: DisconnectOutput::<Impl, IMPL_OFFSET>,
            GetInput: GetInput::<Impl, IMPL_OFFSET>,
            GetOutput: GetOutput::<Impl, IMPL_OFFSET>,
            SetOutputPrefType: SetOutputPrefType::<Impl, IMPL_OFFSET>,
            GetOutputPrefType: GetOutputPrefType::<Impl, IMPL_OFFSET>,
            SetInputPrefType: SetInputPrefType::<Impl, IMPL_OFFSET>,
            GetInputPrefType: GetInputPrefType::<Impl, IMPL_OFFSET>,
            CloneFrom: CloneFrom::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTopologyNode as ::windows::core::Interface>::IID
    }
}
pub trait IMFTopologyNodeAttributeEditorImpl: Sized {
    fn UpdateNodeAttributes();
}
impl IMFTopologyNodeAttributeEditorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTopologyNodeAttributeEditorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTopologyNodeAttributeEditorVtbl {
        unsafe extern "system" fn UpdateNodeAttributes<Impl: IMFTopologyNodeAttributeEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topoid: u64, cupdates: u32, pupdates: *const MFTOPONODE_ATTRIBUTE_UPDATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), UpdateNodeAttributes: UpdateNodeAttributes::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTopologyNodeAttributeEditor as ::windows::core::Interface>::IID
    }
}
pub trait IMFTopologyServiceLookupImpl: Sized {
    fn LookupService();
}
impl IMFTopologyServiceLookupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTopologyServiceLookupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTopologyServiceLookupVtbl {
        unsafe extern "system" fn LookupService<Impl: IMFTopologyServiceLookupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: MF_SERVICE_LOOKUP_TYPE, dwindex: u32, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobjects: *mut *mut ::core::ffi::c_void, pnobjects: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), LookupService: LookupService::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTopologyServiceLookup as ::windows::core::Interface>::IID
    }
}
pub trait IMFTopologyServiceLookupClientImpl: Sized {
    fn InitServicePointers();
    fn ReleaseServicePointers();
}
impl IMFTopologyServiceLookupClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTopologyServiceLookupClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTopologyServiceLookupClientVtbl {
        unsafe extern "system" fn InitServicePointers<Impl: IMFTopologyServiceLookupClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plookup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseServicePointers<Impl: IMFTopologyServiceLookupClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitServicePointers: InitServicePointers::<Impl, IMPL_OFFSET>,
            ReleaseServicePointers: ReleaseServicePointers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTopologyServiceLookupClient as ::windows::core::Interface>::IID
    }
}
pub trait IMFTrackedSampleImpl: Sized {
    fn SetAllocator();
}
impl IMFTrackedSampleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTrackedSampleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTrackedSampleVtbl {
        unsafe extern "system" fn SetAllocator<Impl: IMFTrackedSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psampleallocator: ::windows::core::RawPtr, punkstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetAllocator: SetAllocator::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTrackedSample as ::windows::core::Interface>::IID
    }
}
pub trait IMFTranscodeProfileImpl: Sized {
    fn SetAudioAttributes();
    fn GetAudioAttributes();
    fn SetVideoAttributes();
    fn GetVideoAttributes();
    fn SetContainerAttributes();
    fn GetContainerAttributes();
}
impl IMFTranscodeProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTranscodeProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTranscodeProfileVtbl {
        unsafe extern "system" fn SetAudioAttributes<Impl: IMFTranscodeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattrs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAudioAttributes<Impl: IMFTranscodeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppattrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVideoAttributes<Impl: IMFTranscodeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattrs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoAttributes<Impl: IMFTranscodeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppattrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContainerAttributes<Impl: IMFTranscodeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattrs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContainerAttributes<Impl: IMFTranscodeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppattrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAudioAttributes: SetAudioAttributes::<Impl, IMPL_OFFSET>,
            GetAudioAttributes: GetAudioAttributes::<Impl, IMPL_OFFSET>,
            SetVideoAttributes: SetVideoAttributes::<Impl, IMPL_OFFSET>,
            GetVideoAttributes: GetVideoAttributes::<Impl, IMPL_OFFSET>,
            SetContainerAttributes: SetContainerAttributes::<Impl, IMPL_OFFSET>,
            GetContainerAttributes: GetContainerAttributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTranscodeProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTranscodeSinkInfoProviderImpl: Sized {
    fn SetOutputFile();
    fn SetOutputByteStream();
    fn SetProfile();
    fn GetSinkInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTranscodeSinkInfoProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTranscodeSinkInfoProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTranscodeSinkInfoProviderVtbl {
        unsafe extern "system" fn SetOutputFile<Impl: IMFTranscodeSinkInfoProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputByteStream<Impl: IMFTranscodeSinkInfoProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytestreamactivate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProfile<Impl: IMFTranscodeSinkInfoProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSinkInfo<Impl: IMFTranscodeSinkInfoProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psinkinfo: *mut MF_TRANSCODE_SINK_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetOutputFile: SetOutputFile::<Impl, IMPL_OFFSET>,
            SetOutputByteStream: SetOutputByteStream::<Impl, IMPL_OFFSET>,
            SetProfile: SetProfile::<Impl, IMPL_OFFSET>,
            GetSinkInfo: GetSinkInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTranscodeSinkInfoProvider as ::windows::core::Interface>::IID
    }
}
pub trait IMFTransformImpl: Sized {
    fn GetStreamLimits();
    fn GetStreamCount();
    fn GetStreamIDs();
    fn GetInputStreamInfo();
    fn GetOutputStreamInfo();
    fn GetAttributes();
    fn GetInputStreamAttributes();
    fn GetOutputStreamAttributes();
    fn DeleteInputStream();
    fn AddInputStreams();
    fn GetInputAvailableType();
    fn GetOutputAvailableType();
    fn SetInputType();
    fn SetOutputType();
    fn GetInputCurrentType();
    fn GetOutputCurrentType();
    fn GetInputStatus();
    fn GetOutputStatus();
    fn SetOutputBounds();
    fn ProcessEvent();
    fn ProcessMessage();
    fn ProcessInput();
    fn ProcessOutput();
}
impl IMFTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTransformVtbl {
        unsafe extern "system" fn GetStreamLimits<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwinputminimum: *mut u32, pdwinputmaximum: *mut u32, pdwoutputminimum: *mut u32, pdwoutputmaximum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamCount<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamIDs<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputidarraysize: u32, pdwinputids: *mut u32, dwoutputidarraysize: u32, pdwoutputids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputStreamInfo<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pstreaminfo: *mut MFT_INPUT_STREAM_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputStreamInfo<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, pstreaminfo: *mut MFT_OUTPUT_STREAM_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributes<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputStreamAttributes<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputStreamAttributes<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, pattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteInputStream<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddInputStreams<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstreams: u32, adwstreamids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputAvailableType<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, dwtypeindex: u32, pptype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputAvailableType<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, dwtypeindex: u32, pptype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputType<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, ptype: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputType<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, ptype: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputCurrentType<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pptype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputCurrentType<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, pptype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputStatus<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputStatus<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputBounds<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnslowerbound: i64, hnsupperbound: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessEvent<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessMessage<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emessage: MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessInput<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, psample: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessOutput<Impl: IMFTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputsamples: *mut MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreamLimits: GetStreamLimits::<Impl, IMPL_OFFSET>,
            GetStreamCount: GetStreamCount::<Impl, IMPL_OFFSET>,
            GetStreamIDs: GetStreamIDs::<Impl, IMPL_OFFSET>,
            GetInputStreamInfo: GetInputStreamInfo::<Impl, IMPL_OFFSET>,
            GetOutputStreamInfo: GetOutputStreamInfo::<Impl, IMPL_OFFSET>,
            GetAttributes: GetAttributes::<Impl, IMPL_OFFSET>,
            GetInputStreamAttributes: GetInputStreamAttributes::<Impl, IMPL_OFFSET>,
            GetOutputStreamAttributes: GetOutputStreamAttributes::<Impl, IMPL_OFFSET>,
            DeleteInputStream: DeleteInputStream::<Impl, IMPL_OFFSET>,
            AddInputStreams: AddInputStreams::<Impl, IMPL_OFFSET>,
            GetInputAvailableType: GetInputAvailableType::<Impl, IMPL_OFFSET>,
            GetOutputAvailableType: GetOutputAvailableType::<Impl, IMPL_OFFSET>,
            SetInputType: SetInputType::<Impl, IMPL_OFFSET>,
            SetOutputType: SetOutputType::<Impl, IMPL_OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Impl, IMPL_OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Impl, IMPL_OFFSET>,
            GetInputStatus: GetInputStatus::<Impl, IMPL_OFFSET>,
            GetOutputStatus: GetOutputStatus::<Impl, IMPL_OFFSET>,
            SetOutputBounds: SetOutputBounds::<Impl, IMPL_OFFSET>,
            ProcessEvent: ProcessEvent::<Impl, IMPL_OFFSET>,
            ProcessMessage: ProcessMessage::<Impl, IMPL_OFFSET>,
            ProcessInput: ProcessInput::<Impl, IMPL_OFFSET>,
            ProcessOutput: ProcessOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTransform as ::windows::core::Interface>::IID
    }
}
pub trait IMFTrustedInputImpl: Sized {
    fn GetInputTrustAuthority();
}
impl IMFTrustedInputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTrustedInputImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTrustedInputVtbl {
        unsafe extern "system" fn GetInputTrustAuthority<Impl: IMFTrustedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, riid: *const ::windows::core::GUID, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetInputTrustAuthority: GetInputTrustAuthority::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTrustedInput as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFTrustedOutputImpl: Sized {
    fn GetOutputTrustAuthorityCount();
    fn GetOutputTrustAuthorityByIndex();
    fn IsFinal();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFTrustedOutputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFTrustedOutputImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFTrustedOutputVtbl {
        unsafe extern "system" fn GetOutputTrustAuthorityCount<Impl: IMFTrustedOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputtrustauthorities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputTrustAuthorityByIndex<Impl: IMFTrustedOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppauthority: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsFinal<Impl: IMFTrustedOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisfinal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOutputTrustAuthorityCount: GetOutputTrustAuthorityCount::<Impl, IMPL_OFFSET>,
            GetOutputTrustAuthorityByIndex: GetOutputTrustAuthorityByIndex::<Impl, IMPL_OFFSET>,
            IsFinal: IsFinal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFTrustedOutput as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoCaptureSampleAllocatorImpl: Sized + IMFVideoSampleAllocatorImpl {
    fn InitializeCaptureSampleAllocator();
}
impl IMFVideoCaptureSampleAllocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoCaptureSampleAllocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoCaptureSampleAllocatorVtbl {
        unsafe extern "system" fn InitializeCaptureSampleAllocator<Impl: IMFVideoCaptureSampleAllocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsamplesize: u32, cbcapturemetadatasize: u32, cbalignment: u32, cminimumsamples: u32, pattributes: ::windows::core::RawPtr, pmediatype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFVideoSampleAllocatorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeCaptureSampleAllocator: InitializeCaptureSampleAllocator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoCaptureSampleAllocator as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoDeviceIDImpl: Sized {
    fn GetDeviceID();
}
impl IMFVideoDeviceIDVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoDeviceIDImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoDeviceIDVtbl {
        unsafe extern "system" fn GetDeviceID<Impl: IMFVideoDeviceIDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDeviceID: GetDeviceID::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoDeviceID as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IMFVideoDisplayControlImpl: Sized {
    fn GetNativeVideoSize();
    fn GetIdealVideoSize();
    fn SetVideoPosition();
    fn GetVideoPosition();
    fn SetAspectRatioMode();
    fn GetAspectRatioMode();
    fn SetVideoWindow();
    fn GetVideoWindow();
    fn RepaintVideo();
    fn GetCurrentImage();
    fn SetBorderColor();
    fn GetBorderColor();
    fn SetRenderingPrefs();
    fn GetRenderingPrefs();
    fn SetFullscreen();
    fn GetFullscreen();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IMFVideoDisplayControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoDisplayControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoDisplayControlVtbl {
        unsafe extern "system" fn GetNativeVideoSize<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvideo: *mut super::super::Foundation::SIZE, pszarvideo: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIdealVideoSize<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmin: *mut super::super::Foundation::SIZE, pszmax: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVideoPosition<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnrcsource: *const MFVideoNormalizedRect, prcdest: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoPosition<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnrcsource: *mut MFVideoNormalizedRect, prcdest: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAspectRatioMode<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspectratiomode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAspectRatioMode<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwaspectratiomode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVideoWindow<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndvideo: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoWindow<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndvideo: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RepaintVideo<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentImage<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbih: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER, pdib: *mut *mut u8, pcbdib: *mut u32, ptimestamp: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBorderColor<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clr: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBorderColor<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRenderingPrefs<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrenderflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRenderingPrefs<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrenderflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFullscreen<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFullscreen<Impl: IMFVideoDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNativeVideoSize: GetNativeVideoSize::<Impl, IMPL_OFFSET>,
            GetIdealVideoSize: GetIdealVideoSize::<Impl, IMPL_OFFSET>,
            SetVideoPosition: SetVideoPosition::<Impl, IMPL_OFFSET>,
            GetVideoPosition: GetVideoPosition::<Impl, IMPL_OFFSET>,
            SetAspectRatioMode: SetAspectRatioMode::<Impl, IMPL_OFFSET>,
            GetAspectRatioMode: GetAspectRatioMode::<Impl, IMPL_OFFSET>,
            SetVideoWindow: SetVideoWindow::<Impl, IMPL_OFFSET>,
            GetVideoWindow: GetVideoWindow::<Impl, IMPL_OFFSET>,
            RepaintVideo: RepaintVideo::<Impl, IMPL_OFFSET>,
            GetCurrentImage: GetCurrentImage::<Impl, IMPL_OFFSET>,
            SetBorderColor: SetBorderColor::<Impl, IMPL_OFFSET>,
            GetBorderColor: GetBorderColor::<Impl, IMPL_OFFSET>,
            SetRenderingPrefs: SetRenderingPrefs::<Impl, IMPL_OFFSET>,
            GetRenderingPrefs: GetRenderingPrefs::<Impl, IMPL_OFFSET>,
            SetFullscreen: SetFullscreen::<Impl, IMPL_OFFSET>,
            GetFullscreen: GetFullscreen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoDisplayControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFVideoMediaTypeImpl: Sized + IMFAttributesImpl + IMFMediaTypeImpl {
    fn GetVideoFormat();
    fn GetVideoRepresentation();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFVideoMediaTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoMediaTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoMediaTypeVtbl {
        unsafe extern "system" fn GetVideoFormat<Impl: IMFVideoMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut MFVIDEOFORMAT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoRepresentation<Impl: IMFVideoMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidrepresentation: ::windows::core::GUID, ppvrepresentation: *mut *mut ::core::ffi::c_void, lstride: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFMediaTypeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetVideoFormat: GetVideoFormat::<Impl, IMPL_OFFSET>,
            GetVideoRepresentation: GetVideoRepresentation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoMediaType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
pub trait IMFVideoMixerBitmapImpl: Sized {
    fn SetAlphaBitmap();
    fn ClearAlphaBitmap();
    fn UpdateAlphaBitmapParameters();
    fn GetAlphaBitmapParameters();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
impl IMFVideoMixerBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoMixerBitmapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoMixerBitmapVtbl {
        unsafe extern "system" fn SetAlphaBitmap<Impl: IMFVideoMixerBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmpparms: *const MFVideoAlphaBitmap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearAlphaBitmap<Impl: IMFVideoMixerBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateAlphaBitmapParameters<Impl: IMFVideoMixerBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmpparms: *const MFVideoAlphaBitmapParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAlphaBitmapParameters<Impl: IMFVideoMixerBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmpparms: *mut MFVideoAlphaBitmapParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAlphaBitmap: SetAlphaBitmap::<Impl, IMPL_OFFSET>,
            ClearAlphaBitmap: ClearAlphaBitmap::<Impl, IMPL_OFFSET>,
            UpdateAlphaBitmapParameters: UpdateAlphaBitmapParameters::<Impl, IMPL_OFFSET>,
            GetAlphaBitmapParameters: GetAlphaBitmapParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoMixerBitmap as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoMixerControlImpl: Sized {
    fn SetStreamZOrder();
    fn GetStreamZOrder();
    fn SetStreamOutputRect();
    fn GetStreamOutputRect();
}
impl IMFVideoMixerControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoMixerControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoMixerControlVtbl {
        unsafe extern "system" fn SetStreamZOrder<Impl: IMFVideoMixerControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, dwz: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamZOrder<Impl: IMFVideoMixerControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pdwz: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamOutputRect<Impl: IMFVideoMixerControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pnrcoutput: *const MFVideoNormalizedRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamOutputRect<Impl: IMFVideoMixerControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pnrcoutput: *mut MFVideoNormalizedRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetStreamZOrder: SetStreamZOrder::<Impl, IMPL_OFFSET>,
            GetStreamZOrder: GetStreamZOrder::<Impl, IMPL_OFFSET>,
            SetStreamOutputRect: SetStreamOutputRect::<Impl, IMPL_OFFSET>,
            GetStreamOutputRect: GetStreamOutputRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoMixerControl as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoMixerControl2Impl: Sized + IMFVideoMixerControlImpl {
    fn SetMixingPrefs();
    fn GetMixingPrefs();
}
impl IMFVideoMixerControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoMixerControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoMixerControl2Vtbl {
        unsafe extern "system" fn SetMixingPrefs<Impl: IMFVideoMixerControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmixflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMixingPrefs<Impl: IMFVideoMixerControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmixflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFVideoMixerControlVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMixingPrefs: SetMixingPrefs::<Impl, IMPL_OFFSET>,
            GetMixingPrefs: GetMixingPrefs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoMixerControl2 as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoPositionMapperImpl: Sized {
    fn MapOutputCoordinateToInputStream();
}
impl IMFVideoPositionMapperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoPositionMapperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoPositionMapperVtbl {
        unsafe extern "system" fn MapOutputCoordinateToInputStream<Impl: IMFVideoPositionMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xout: f32, yout: f32, dwoutputstreamindex: u32, dwinputstreamindex: u32, pxin: *mut f32, pyin: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MapOutputCoordinateToInputStream: MapOutputCoordinateToInputStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoPositionMapper as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoPresenterImpl: Sized + IMFClockStateSinkImpl {
    fn ProcessMessage();
    fn GetCurrentMediaType();
}
impl IMFVideoPresenterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoPresenterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoPresenterVtbl {
        unsafe extern "system" fn ProcessMessage<Impl: IMFVideoPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emessage: MFVP_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentMediaType<Impl: IMFVideoPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFClockStateSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ProcessMessage: ProcessMessage::<Impl, IMPL_OFFSET>,
            GetCurrentMediaType: GetCurrentMediaType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoPresenter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub trait IMFVideoProcessorImpl: Sized {
    fn GetAvailableVideoProcessorModes();
    fn GetVideoProcessorCaps();
    fn GetVideoProcessorMode();
    fn SetVideoProcessorMode();
    fn GetProcAmpRange();
    fn GetProcAmpValues();
    fn SetProcAmpValues();
    fn GetFilteringRange();
    fn GetFilteringValue();
    fn SetFilteringValue();
    fn GetBackgroundColor();
    fn SetBackgroundColor();
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl IMFVideoProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoProcessorVtbl {
        unsafe extern "system" fn GetAvailableVideoProcessorModes<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwnumprocessingmodes: *mut u32, ppvideoprocessingmodes: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvideoprocessormode: *const ::windows::core::GUID, lpvideoprocessorcaps: *mut DXVA2_VideoProcessorCaps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorMode<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmode: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVideoProcessorMode<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmode: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProcAmpRange<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: u32, pproprange: *mut DXVA2_ValueRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProcAmpValues<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, values: *mut DXVA2_ProcAmpValues) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProcAmpValues<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pvalues: *const DXVA2_ProcAmpValues) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteringRange<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: u32, pproprange: *mut DXVA2_ValueRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteringValue<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: u32, pvalue: *mut DXVA2_Fixed32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFilteringValue<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: u32, pvalue: *const DXVA2_Fixed32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackgroundColor<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpclrbkg: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IMFVideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clrbkg: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAvailableVideoProcessorModes: GetAvailableVideoProcessorModes::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Impl, IMPL_OFFSET>,
            GetVideoProcessorMode: GetVideoProcessorMode::<Impl, IMPL_OFFSET>,
            SetVideoProcessorMode: SetVideoProcessorMode::<Impl, IMPL_OFFSET>,
            GetProcAmpRange: GetProcAmpRange::<Impl, IMPL_OFFSET>,
            GetProcAmpValues: GetProcAmpValues::<Impl, IMPL_OFFSET>,
            SetProcAmpValues: SetProcAmpValues::<Impl, IMPL_OFFSET>,
            GetFilteringRange: GetFilteringRange::<Impl, IMPL_OFFSET>,
            GetFilteringValue: GetFilteringValue::<Impl, IMPL_OFFSET>,
            SetFilteringValue: SetFilteringValue::<Impl, IMPL_OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoProcessor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFVideoProcessorControlImpl: Sized {
    fn SetBorderColor();
    fn SetSourceRectangle();
    fn SetDestinationRectangle();
    fn SetMirror();
    fn SetRotation();
    fn SetConstrictionSize();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFVideoProcessorControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoProcessorControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoProcessorControlVtbl {
        unsafe extern "system" fn SetBorderColor<Impl: IMFVideoProcessorControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbordercolor: *const MFARGB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourceRectangle<Impl: IMFVideoProcessorControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDestinationRectangle<Impl: IMFVideoProcessorControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMirror<Impl: IMFVideoProcessorControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emirror: MF_VIDEO_PROCESSOR_MIRROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRotation<Impl: IMFVideoProcessorControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erotation: MF_VIDEO_PROCESSOR_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConstrictionSize<Impl: IMFVideoProcessorControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstrictionsize: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetBorderColor: SetBorderColor::<Impl, IMPL_OFFSET>,
            SetSourceRectangle: SetSourceRectangle::<Impl, IMPL_OFFSET>,
            SetDestinationRectangle: SetDestinationRectangle::<Impl, IMPL_OFFSET>,
            SetMirror: SetMirror::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
            SetConstrictionSize: SetConstrictionSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoProcessorControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFVideoProcessorControl2Impl: Sized + IMFVideoProcessorControlImpl {
    fn SetRotationOverride();
    fn EnableHardwareEffects();
    fn GetSupportedHardwareEffects();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFVideoProcessorControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoProcessorControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoProcessorControl2Vtbl {
        unsafe extern "system" fn SetRotationOverride<Impl: IMFVideoProcessorControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uirotation: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableHardwareEffects<Impl: IMFVideoProcessorControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedHardwareEffects<Impl: IMFVideoProcessorControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puisupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFVideoProcessorControlVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRotationOverride: SetRotationOverride::<Impl, IMPL_OFFSET>,
            EnableHardwareEffects: EnableHardwareEffects::<Impl, IMPL_OFFSET>,
            GetSupportedHardwareEffects: GetSupportedHardwareEffects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoProcessorControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFVideoProcessorControl3Impl: Sized + IMFVideoProcessorControlImpl + IMFVideoProcessorControl2Impl {
    fn GetNaturalOutputType();
    fn EnableSphericalVideoProcessing();
    fn SetSphericalVideoProperties();
    fn SetOutputDevice();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFVideoProcessorControl3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoProcessorControl3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoProcessorControl3Vtbl {
        unsafe extern "system" fn GetNaturalOutputType<Impl: IMFVideoProcessorControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableSphericalVideoProcessing<Impl: IMFVideoProcessorControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL, eformat: MFVideoSphericalFormat, eprojectionmode: MFVideoSphericalProjectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSphericalVideoProperties<Impl: IMFVideoProcessorControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, w: f32, fieldofview: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputDevice<Impl: IMFVideoProcessorControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputdevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFVideoProcessorControl2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetNaturalOutputType: GetNaturalOutputType::<Impl, IMPL_OFFSET>,
            EnableSphericalVideoProcessing: EnableSphericalVideoProcessing::<Impl, IMPL_OFFSET>,
            SetSphericalVideoProperties: SetSphericalVideoProperties::<Impl, IMPL_OFFSET>,
            SetOutputDevice: SetOutputDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoProcessorControl3 as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoRendererImpl: Sized {
    fn InitializeRenderer();
}
impl IMFVideoRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoRendererImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoRendererVtbl {
        unsafe extern "system" fn InitializeRenderer<Impl: IMFVideoRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideomixer: ::windows::core::RawPtr, pvideopresenter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), InitializeRenderer: InitializeRenderer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoRenderer as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoRendererEffectControlImpl: Sized {
    fn OnAppServiceConnectionEstablished();
}
impl IMFVideoRendererEffectControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoRendererEffectControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoRendererEffectControlVtbl {
        unsafe extern "system" fn OnAppServiceConnectionEstablished<Impl: IMFVideoRendererEffectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappserviceconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnAppServiceConnectionEstablished: OnAppServiceConnectionEstablished::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoRendererEffectControl as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoSampleAllocatorImpl: Sized {
    fn SetDirectXManager();
    fn UninitializeSampleAllocator();
    fn InitializeSampleAllocator();
    fn AllocateSample();
}
impl IMFVideoSampleAllocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoSampleAllocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoSampleAllocatorVtbl {
        unsafe extern "system" fn SetDirectXManager<Impl: IMFVideoSampleAllocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UninitializeSampleAllocator<Impl: IMFVideoSampleAllocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeSampleAllocator<Impl: IMFVideoSampleAllocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequestedframes: u32, pmediatype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllocateSample<Impl: IMFVideoSampleAllocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsample: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDirectXManager: SetDirectXManager::<Impl, IMPL_OFFSET>,
            UninitializeSampleAllocator: UninitializeSampleAllocator::<Impl, IMPL_OFFSET>,
            InitializeSampleAllocator: InitializeSampleAllocator::<Impl, IMPL_OFFSET>,
            AllocateSample: AllocateSample::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoSampleAllocator as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoSampleAllocatorCallbackImpl: Sized {
    fn SetCallback();
    fn GetFreeSampleCount();
}
impl IMFVideoSampleAllocatorCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoSampleAllocatorCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoSampleAllocatorCallbackVtbl {
        unsafe extern "system" fn SetCallback<Impl: IMFVideoSampleAllocatorCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFreeSampleCount<Impl: IMFVideoSampleAllocatorCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsamples: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetCallback: SetCallback::<Impl, IMPL_OFFSET>,
            GetFreeSampleCount: GetFreeSampleCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoSampleAllocatorCallback as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoSampleAllocatorExImpl: Sized + IMFVideoSampleAllocatorImpl {
    fn InitializeSampleAllocatorEx();
}
impl IMFVideoSampleAllocatorExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoSampleAllocatorExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoSampleAllocatorExVtbl {
        unsafe extern "system" fn InitializeSampleAllocatorEx<Impl: IMFVideoSampleAllocatorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinitialsamples: u32, cmaximumsamples: u32, pattributes: ::windows::core::RawPtr, pmediatype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFVideoSampleAllocatorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeSampleAllocatorEx: InitializeSampleAllocatorEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoSampleAllocatorEx as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoSampleAllocatorNotifyImpl: Sized {
    fn NotifyRelease();
}
impl IMFVideoSampleAllocatorNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoSampleAllocatorNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoSampleAllocatorNotifyVtbl {
        unsafe extern "system" fn NotifyRelease<Impl: IMFVideoSampleAllocatorNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), NotifyRelease: NotifyRelease::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoSampleAllocatorNotify as ::windows::core::Interface>::IID
    }
}
pub trait IMFVideoSampleAllocatorNotifyExImpl: Sized + IMFVideoSampleAllocatorNotifyImpl {
    fn NotifyPrune();
}
impl IMFVideoSampleAllocatorNotifyExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVideoSampleAllocatorNotifyExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVideoSampleAllocatorNotifyExVtbl {
        unsafe extern "system" fn NotifyPrune<Impl: IMFVideoSampleAllocatorNotifyExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__imfvideosampleallocatornotifyex0000: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IMFVideoSampleAllocatorNotifyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), NotifyPrune: NotifyPrune::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVideoSampleAllocatorNotifyEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMFVirtualCameraImpl: Sized + IMFAttributesImpl {
    fn AddDeviceSourceInfo();
    fn AddProperty();
    fn AddRegistryEntry();
    fn Start();
    fn Stop();
    fn Remove();
    fn GetMediaSource();
    fn SendCameraProperty();
    fn CreateSyncEvent();
    fn CreateSyncSemaphore();
    fn Shutdown();
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMFVirtualCameraVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFVirtualCameraImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFVirtualCameraVtbl {
        unsafe extern "system" fn AddDeviceSourceInfo<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicesourceinfo: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddProperty<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const super::super::Devices::Properties::DEVPROPKEY, r#type: u32, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRegistryEntry<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entryname: super::super::Foundation::PWSTR, subkeypath: super::super::Foundation::PWSTR, dwregtype: u32, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Start<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMediaSource<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediasource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendCameraProperty<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyset: *const ::windows::core::GUID, propertyid: u32, propertyflags: u32, propertypayload: *mut ::core::ffi::c_void, propertypayloadlength: u32, data: *mut ::core::ffi::c_void, datalength: u32, datawritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSyncEvent<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kseventset: *const ::windows::core::GUID, kseventid: u32, kseventflags: u32, eventhandle: super::super::Foundation::HANDLE, camerasyncobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSyncSemaphore<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kseventset: *const ::windows::core::GUID, kseventid: u32, kseventflags: u32, semaphorehandle: super::super::Foundation::HANDLE, semaphoreadjustment: i32, camerasyncobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IMFVirtualCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddDeviceSourceInfo: AddDeviceSourceInfo::<Impl, IMPL_OFFSET>,
            AddProperty: AddProperty::<Impl, IMPL_OFFSET>,
            AddRegistryEntry: AddRegistryEntry::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            GetMediaSource: GetMediaSource::<Impl, IMPL_OFFSET>,
            SendCameraProperty: SendCameraProperty::<Impl, IMPL_OFFSET>,
            CreateSyncEvent: CreateSyncEvent::<Impl, IMPL_OFFSET>,
            CreateSyncSemaphore: CreateSyncSemaphore::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFVirtualCamera as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFWorkQueueServicesImpl: Sized {
    fn BeginRegisterTopologyWorkQueuesWithMMCSS();
    fn EndRegisterTopologyWorkQueuesWithMMCSS();
    fn BeginUnregisterTopologyWorkQueuesWithMMCSS();
    fn EndUnregisterTopologyWorkQueuesWithMMCSS();
    fn GetTopologyWorkQueueMMCSSClass();
    fn GetTopologyWorkQueueMMCSSTaskId();
    fn BeginRegisterPlatformWorkQueueWithMMCSS();
    fn EndRegisterPlatformWorkQueueWithMMCSS();
    fn BeginUnregisterPlatformWorkQueueWithMMCSS();
    fn EndUnregisterPlatformWorkQueueWithMMCSS();
    fn GetPlaftormWorkQueueMMCSSClass();
    fn GetPlatformWorkQueueMMCSSTaskId();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFWorkQueueServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFWorkQueueServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFWorkQueueServicesVtbl {
        unsafe extern "system" fn BeginRegisterTopologyWorkQueuesWithMMCSS<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndRegisterTopologyWorkQueuesWithMMCSS<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginUnregisterTopologyWorkQueuesWithMMCSS<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndUnregisterTopologyWorkQueuesWithMMCSS<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTopologyWorkQueueMMCSSClass<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtopologyworkqueueid: u32, pwszclass: super::super::Foundation::PWSTR, pcchclass: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTopologyWorkQueueMMCSSTaskId<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtopologyworkqueueid: u32, pdwtaskid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginRegisterPlatformWorkQueueWithMMCSS<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplatformworkqueue: u32, wszclass: super::super::Foundation::PWSTR, dwtaskid: u32, pcallback: ::windows::core::RawPtr, pstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndRegisterPlatformWorkQueueWithMMCSS<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, pdwtaskid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginUnregisterPlatformWorkQueueWithMMCSS<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplatformworkqueue: u32, pcallback: ::windows::core::RawPtr, pstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndUnregisterPlatformWorkQueueWithMMCSS<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPlaftormWorkQueueMMCSSClass<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplatformworkqueueid: u32, pwszclass: super::super::Foundation::PWSTR, pcchclass: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPlatformWorkQueueMMCSSTaskId<Impl: IMFWorkQueueServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplatformworkqueueid: u32, pdwtaskid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginRegisterTopologyWorkQueuesWithMMCSS: BeginRegisterTopologyWorkQueuesWithMMCSS::<Impl, IMPL_OFFSET>,
            EndRegisterTopologyWorkQueuesWithMMCSS: EndRegisterTopologyWorkQueuesWithMMCSS::<Impl, IMPL_OFFSET>,
            BeginUnregisterTopologyWorkQueuesWithMMCSS: BeginUnregisterTopologyWorkQueuesWithMMCSS::<Impl, IMPL_OFFSET>,
            EndUnregisterTopologyWorkQueuesWithMMCSS: EndUnregisterTopologyWorkQueuesWithMMCSS::<Impl, IMPL_OFFSET>,
            GetTopologyWorkQueueMMCSSClass: GetTopologyWorkQueueMMCSSClass::<Impl, IMPL_OFFSET>,
            GetTopologyWorkQueueMMCSSTaskId: GetTopologyWorkQueueMMCSSTaskId::<Impl, IMPL_OFFSET>,
            BeginRegisterPlatformWorkQueueWithMMCSS: BeginRegisterPlatformWorkQueueWithMMCSS::<Impl, IMPL_OFFSET>,
            EndRegisterPlatformWorkQueueWithMMCSS: EndRegisterPlatformWorkQueueWithMMCSS::<Impl, IMPL_OFFSET>,
            BeginUnregisterPlatformWorkQueueWithMMCSS: BeginUnregisterPlatformWorkQueueWithMMCSS::<Impl, IMPL_OFFSET>,
            EndUnregisterPlatformWorkQueueWithMMCSS: EndUnregisterPlatformWorkQueueWithMMCSS::<Impl, IMPL_OFFSET>,
            GetPlaftormWorkQueueMMCSSClass: GetPlaftormWorkQueueMMCSSClass::<Impl, IMPL_OFFSET>,
            GetPlatformWorkQueueMMCSSTaskId: GetPlatformWorkQueueMMCSSTaskId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFWorkQueueServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMFWorkQueueServicesExImpl: Sized + IMFWorkQueueServicesImpl {
    fn GetTopologyWorkQueueMMCSSPriority();
    fn BeginRegisterPlatformWorkQueueWithMMCSSEx();
    fn GetPlatformWorkQueueMMCSSPriority();
}
#[cfg(feature = "Win32_Foundation")]
impl IMFWorkQueueServicesExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFWorkQueueServicesExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFWorkQueueServicesExVtbl {
        unsafe extern "system" fn GetTopologyWorkQueueMMCSSPriority<Impl: IMFWorkQueueServicesExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtopologyworkqueueid: u32, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginRegisterPlatformWorkQueueWithMMCSSEx<Impl: IMFWorkQueueServicesExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplatformworkqueue: u32, wszclass: super::super::Foundation::PWSTR, dwtaskid: u32, lpriority: i32, pcallback: ::windows::core::RawPtr, pstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPlatformWorkQueueMMCSSPriority<Impl: IMFWorkQueueServicesExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplatformworkqueueid: u32, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMFWorkQueueServicesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetTopologyWorkQueueMMCSSPriority: GetTopologyWorkQueueMMCSSPriority::<Impl, IMPL_OFFSET>,
            BeginRegisterPlatformWorkQueueWithMMCSSEx: BeginRegisterPlatformWorkQueueWithMMCSSEx::<Impl, IMPL_OFFSET>,
            GetPlatformWorkQueueMMCSSPriority: GetPlatformWorkQueueMMCSSPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFWorkQueueServicesEx as ::windows::core::Interface>::IID
    }
}
pub trait IOPMVideoOutputImpl: Sized {
    fn StartInitialization();
    fn FinishInitialization();
    fn GetInformation();
    fn COPPCompatibleGetInformation();
    fn Configure();
}
impl IOPMVideoOutputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOPMVideoOutputImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOPMVideoOutputVtbl {
        unsafe extern "system" fn StartInitialization<Impl: IOPMVideoOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prnrandomnumber: *mut OPM_RANDOM_NUMBER, ppbcertificate: *mut *mut u8, pulcertificatelength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FinishInitialization<Impl: IOPMVideoOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const OPM_ENCRYPTED_INITIALIZATION_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInformation<Impl: IOPMVideoOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const OPM_GET_INFO_PARAMETERS, prequestedinformation: *mut OPM_REQUESTED_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn COPPCompatibleGetInformation<Impl: IOPMVideoOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS, prequestedinformation: *mut OPM_REQUESTED_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Configure<Impl: IOPMVideoOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const OPM_CONFIGURE_PARAMETERS, uladditionalparameterssize: u32, pbadditionalparameters: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartInitialization: StartInitialization::<Impl, IMPL_OFFSET>,
            FinishInitialization: FinishInitialization::<Impl, IMPL_OFFSET>,
            GetInformation: GetInformation::<Impl, IMPL_OFFSET>,
            COPPCompatibleGetInformation: COPPCompatibleGetInformation::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOPMVideoOutput as ::windows::core::Interface>::IID
    }
}
pub trait IPlayToControlImpl: Sized {
    fn Connect();
    fn Disconnect();
}
impl IPlayToControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToControlVtbl {
        unsafe extern "system" fn Connect<Impl: IPlayToControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IPlayToControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToControl as ::windows::core::Interface>::IID
    }
}
pub trait IPlayToControlWithCapabilitiesImpl: Sized + IPlayToControlImpl {
    fn GetCapabilities();
}
impl IPlayToControlWithCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToControlWithCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToControlWithCapabilitiesVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IPlayToControlWithCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcapabilities: *mut PLAYTO_SOURCE_CREATEFLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IPlayToControlVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToControlWithCapabilities as ::windows::core::Interface>::IID
    }
}
pub trait IPlayToSourceClassFactoryImpl: Sized {
    fn CreateInstance();
}
impl IPlayToSourceClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToSourceClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToSourceClassFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPlayToSourceClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcontrol: ::windows::core::RawPtr, ppsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToSourceClassFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITocImpl: Sized {
    fn SetDescriptor();
    fn GetDescriptor();
    fn SetDescription();
    fn GetDescription();
    fn SetContext();
    fn GetContext();
    fn GetEntryListCount();
    fn GetEntryListByIndex();
    fn AddEntryList();
    fn AddEntryListByIndex();
    fn RemoveEntryListByIndex();
}
#[cfg(feature = "Win32_Foundation")]
impl ITocVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITocImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITocVtbl {
        unsafe extern "system" fn SetDescriptor<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescriptor: *mut TOC_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescriptor<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescriptor: *mut TOC_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwdescriptionsize: *mut u16, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContext<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcontextsize: u32, pbtcontext: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContext<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcontextsize: *mut u32, pbtcontext: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEntryListCount<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEntryListByIndex<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wentrylistindex: u16, ppentrylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEntryList<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentrylist: ::windows::core::RawPtr, pwentrylistindex: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEntryListByIndex<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wentrylistindex: u16, pentrylist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveEntryListByIndex<Impl: ITocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wentrylistindex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDescriptor: SetDescriptor::<Impl, IMPL_OFFSET>,
            GetDescriptor: GetDescriptor::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            SetContext: SetContext::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
            GetEntryListCount: GetEntryListCount::<Impl, IMPL_OFFSET>,
            GetEntryListByIndex: GetEntryListByIndex::<Impl, IMPL_OFFSET>,
            AddEntryList: AddEntryList::<Impl, IMPL_OFFSET>,
            AddEntryListByIndex: AddEntryListByIndex::<Impl, IMPL_OFFSET>,
            RemoveEntryListByIndex: RemoveEntryListByIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToc as ::windows::core::Interface>::IID
    }
}
pub trait ITocCollectionImpl: Sized {
    fn GetEntryCount();
    fn GetEntryByIndex();
    fn AddEntry();
    fn AddEntryByIndex();
    fn RemoveEntryByIndex();
}
impl ITocCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITocCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITocCollectionVtbl {
        unsafe extern "system" fn GetEntryCount<Impl: ITocCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwentrycount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEntryByIndex<Impl: ITocCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwentryindex: u32, pptoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEntry<Impl: ITocCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoc: ::windows::core::RawPtr, pdwentryindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEntryByIndex<Impl: ITocCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwentryindex: u32, ptoc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveEntryByIndex<Impl: ITocCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwentryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEntryCount: GetEntryCount::<Impl, IMPL_OFFSET>,
            GetEntryByIndex: GetEntryByIndex::<Impl, IMPL_OFFSET>,
            AddEntry: AddEntry::<Impl, IMPL_OFFSET>,
            AddEntryByIndex: AddEntryByIndex::<Impl, IMPL_OFFSET>,
            RemoveEntryByIndex: RemoveEntryByIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITocCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITocEntryImpl: Sized {
    fn SetTitle();
    fn GetTitle();
    fn SetDescriptor();
    fn GetDescriptor();
    fn SetSubEntries();
    fn GetSubEntries();
    fn SetDescriptionData();
    fn GetDescriptionData();
}
#[cfg(feature = "Win32_Foundation")]
impl ITocEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITocEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITocEntryVtbl {
        unsafe extern "system" fn SetTitle<Impl: ITocEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTitle<Impl: ITocEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwtitlesize: *mut u16, pwsztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescriptor<Impl: ITocEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescriptor: *mut TOC_ENTRY_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescriptor<Impl: ITocEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescriptor: *mut TOC_ENTRY_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubEntries<Impl: ITocEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnumsubentries: u32, pwsubentryindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubEntries<Impl: ITocEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwnumsubentries: *mut u32, pwsubentryindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescriptionData<Impl: ITocEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdescriptiondatasize: u32, pbtdescriptiondata: *mut u8, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescriptionData<Impl: ITocEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdescriptiondatasize: *mut u32, pbtdescriptiondata: *mut u8, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            GetTitle: GetTitle::<Impl, IMPL_OFFSET>,
            SetDescriptor: SetDescriptor::<Impl, IMPL_OFFSET>,
            GetDescriptor: GetDescriptor::<Impl, IMPL_OFFSET>,
            SetSubEntries: SetSubEntries::<Impl, IMPL_OFFSET>,
            GetSubEntries: GetSubEntries::<Impl, IMPL_OFFSET>,
            SetDescriptionData: SetDescriptionData::<Impl, IMPL_OFFSET>,
            GetDescriptionData: GetDescriptionData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITocEntry as ::windows::core::Interface>::IID
    }
}
pub trait ITocEntryListImpl: Sized {
    fn GetEntryCount();
    fn GetEntryByIndex();
    fn AddEntry();
    fn AddEntryByIndex();
    fn RemoveEntryByIndex();
}
impl ITocEntryListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITocEntryListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITocEntryListVtbl {
        unsafe extern "system" fn GetEntryCount<Impl: ITocEntryListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwentrycount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEntryByIndex<Impl: ITocEntryListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwentryindex: u32, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEntry<Impl: ITocEntryListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentry: ::windows::core::RawPtr, pdwentryindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEntryByIndex<Impl: ITocEntryListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwentryindex: u32, pentry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveEntryByIndex<Impl: ITocEntryListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwentryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEntryCount: GetEntryCount::<Impl, IMPL_OFFSET>,
            GetEntryByIndex: GetEntryByIndex::<Impl, IMPL_OFFSET>,
            AddEntry: AddEntry::<Impl, IMPL_OFFSET>,
            AddEntryByIndex: AddEntryByIndex::<Impl, IMPL_OFFSET>,
            RemoveEntryByIndex: RemoveEntryByIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITocEntryList as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITocParserImpl: Sized {
    fn Init();
    fn GetTocCount();
    fn GetTocByIndex();
    fn GetTocByType();
    fn AddToc();
    fn RemoveTocByIndex();
    fn RemoveTocByType();
    fn Commit();
}
#[cfg(feature = "Win32_Foundation")]
impl ITocParserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITocParserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITocParserVtbl {
        unsafe extern "system" fn Init<Impl: ITocParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTocCount<Impl: ITocParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, pdwtoccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTocByIndex<Impl: ITocParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, dwtocindex: u32, pptoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTocByType<Impl: ITocParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, guidtoctype: ::windows::core::GUID, pptocs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddToc<Impl: ITocParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, ptoc: ::windows::core::RawPtr, pdwtocindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveTocByIndex<Impl: ITocParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, dwtocindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveTocByType<Impl: ITocParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, guidtoctype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: ITocParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            GetTocCount: GetTocCount::<Impl, IMPL_OFFSET>,
            GetTocByIndex: GetTocByIndex::<Impl, IMPL_OFFSET>,
            GetTocByType: GetTocByType::<Impl, IMPL_OFFSET>,
            AddToc: AddToc::<Impl, IMPL_OFFSET>,
            RemoveTocByIndex: RemoveTocByIndex::<Impl, IMPL_OFFSET>,
            RemoveTocByType: RemoveTocByType::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITocParser as ::windows::core::Interface>::IID
    }
}
pub trait IValidateBindingImpl: Sized {
    fn GetIdentifier();
}
impl IValidateBindingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValidateBindingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValidateBindingVtbl {
        unsafe extern "system" fn GetIdentifier<Impl: IValidateBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidlicensorid: ::windows::core::GUID, pbephemeron: *const u8, cbephemeron: u32, ppbblobvalidationid: *mut *mut u8, pcbblobsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIdentifier: GetIdentifier::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValidateBinding as ::windows::core::Interface>::IID
    }
}
pub trait IWMCodecLeakyBucketImpl: Sized {
    fn SetBufferSizeBits();
    fn GetBufferSizeBits();
    fn SetBufferFullnessBits();
    fn GetBufferFullnessBits();
}
impl IWMCodecLeakyBucketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecLeakyBucketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecLeakyBucketVtbl {
        unsafe extern "system" fn SetBufferSizeBits<Impl: IWMCodecLeakyBucketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferSizeBits<Impl: IWMCodecLeakyBucketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBufferFullnessBits<Impl: IWMCodecLeakyBucketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulbufferfullness: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferFullnessBits<Impl: IWMCodecLeakyBucketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulbufferfullness: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetBufferSizeBits: SetBufferSizeBits::<Impl, IMPL_OFFSET>,
            GetBufferSizeBits: GetBufferSizeBits::<Impl, IMPL_OFFSET>,
            SetBufferFullnessBits: SetBufferFullnessBits::<Impl, IMPL_OFFSET>,
            GetBufferFullnessBits: GetBufferFullnessBits::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecLeakyBucket as ::windows::core::Interface>::IID
    }
}
pub trait IWMCodecOutputTimestampImpl: Sized {
    fn GetNextOutputTime();
}
impl IWMCodecOutputTimestampVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecOutputTimestampImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecOutputTimestampVtbl {
        unsafe extern "system" fn GetNextOutputTime<Impl: IWMCodecOutputTimestampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prttime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetNextOutputTime: GetNextOutputTime::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecOutputTimestamp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DxMediaObjects"))]
pub trait IWMCodecPrivateDataImpl: Sized {
    fn SetPartialOutputType();
    fn GetPrivateData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DxMediaObjects"))]
impl IWMCodecPrivateDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecPrivateDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecPrivateDataVtbl {
        unsafe extern "system" fn SetPartialOutputType<Impl: IWMCodecPrivateDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *mut super::DxMediaObjects::DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateData<Impl: IWMCodecPrivateDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *mut u8, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPartialOutputType: SetPartialOutputType::<Impl, IMPL_OFFSET>,
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecPrivateData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DxMediaObjects"))]
pub trait IWMCodecPropsImpl: Sized {
    fn GetFormatProp();
    fn GetCodecProp();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DxMediaObjects"))]
impl IWMCodecPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecPropsVtbl {
        unsafe extern "system" fn GetFormatProp<Impl: IWMCodecPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *mut super::DxMediaObjects::DMO_MEDIA_TYPE, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_PROP_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecProp<Impl: IWMCodecPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwformat: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_PROP_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFormatProp: GetFormatProp::<Impl, IMPL_OFFSET>,
            GetCodecProp: GetCodecProp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecProps as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DxMediaObjects"))]
pub trait IWMCodecStringsImpl: Sized {
    fn GetName();
    fn GetDescription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DxMediaObjects"))]
impl IWMCodecStringsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecStringsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecStringsVtbl {
        unsafe extern "system" fn GetName<Impl: IWMCodecStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *mut super::DxMediaObjects::DMO_MEDIA_TYPE, cchlength: u32, szname: super::super::Foundation::PWSTR, pcchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: IWMCodecStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *mut super::DxMediaObjects::DMO_MEDIA_TYPE, cchlength: u32, szdescription: super::super::Foundation::PWSTR, pcchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecStrings as ::windows::core::Interface>::IID
    }
}
pub trait IWMColorConvPropsImpl: Sized {
    fn SetMode();
    fn SetFullCroppingParam();
}
impl IWMColorConvPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMColorConvPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMColorConvPropsVtbl {
        unsafe extern "system" fn SetMode<Impl: IWMColorConvPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFullCroppingParam<Impl: IWMColorConvPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrccropleft: i32, lsrccroptop: i32, ldstcropleft: i32, ldstcroptop: i32, lcropwidth: i32, lcropheight: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            SetFullCroppingParam: SetFullCroppingParam::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMColorConvProps as ::windows::core::Interface>::IID
    }
}
pub trait IWMColorLegalizerPropsImpl: Sized {
    fn SetColorLegalizerQuality();
}
impl IWMColorLegalizerPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMColorLegalizerPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMColorLegalizerPropsVtbl {
        unsafe extern "system" fn SetColorLegalizerQuality<Impl: IWMColorLegalizerPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquality: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetColorLegalizerQuality: SetColorLegalizerQuality::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMColorLegalizerProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMFrameInterpPropsImpl: Sized {
    fn SetFrameRateIn();
    fn SetFrameRateOut();
    fn SetFrameInterpEnabled();
    fn SetComplexityLevel();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMFrameInterpPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMFrameInterpPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMFrameInterpPropsVtbl {
        unsafe extern "system" fn SetFrameRateIn<Impl: IWMFrameInterpPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lframerate: i32, lscale: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFrameRateOut<Impl: IWMFrameInterpPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lframerate: i32, lscale: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFrameInterpEnabled<Impl: IWMFrameInterpPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfienabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetComplexityLevel<Impl: IWMFrameInterpPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icomplexity: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetFrameRateIn: SetFrameRateIn::<Impl, IMPL_OFFSET>,
            SetFrameRateOut: SetFrameRateOut::<Impl, IMPL_OFFSET>,
            SetFrameInterpEnabled: SetFrameInterpEnabled::<Impl, IMPL_OFFSET>,
            SetComplexityLevel: SetComplexityLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMFrameInterpProps as ::windows::core::Interface>::IID
    }
}
pub trait IWMInterlacePropsImpl: Sized {
    fn SetProcessType();
    fn SetInitInverseTeleCinePattern();
    fn SetLastFrame();
}
impl IWMInterlacePropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMInterlacePropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMInterlacePropsVtbl {
        unsafe extern "system" fn SetProcessType<Impl: IWMInterlacePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprocesstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitInverseTeleCinePattern<Impl: IWMInterlacePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iinitpattern: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLastFrame<Impl: IWMInterlacePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetProcessType: SetProcessType::<Impl, IMPL_OFFSET>,
            SetInitInverseTeleCinePattern: SetInitInverseTeleCinePattern::<Impl, IMPL_OFFSET>,
            SetLastFrame: SetLastFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMInterlaceProps as ::windows::core::Interface>::IID
    }
}
pub trait IWMResamplerPropsImpl: Sized {
    fn SetHalfFilterLength();
    fn SetUserChannelMtx();
}
impl IWMResamplerPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMResamplerPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMResamplerPropsVtbl {
        unsafe extern "system" fn SetHalfFilterLength<Impl: IWMResamplerPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhalffilterlen: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserChannelMtx<Impl: IWMResamplerPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userchannelmtx: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetHalfFilterLength: SetHalfFilterLength::<Impl, IMPL_OFFSET>,
            SetUserChannelMtx: SetUserChannelMtx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMResamplerProps as ::windows::core::Interface>::IID
    }
}
pub trait IWMResizerPropsImpl: Sized {
    fn SetResizerQuality();
    fn SetInterlaceMode();
    fn SetClipRegion();
    fn SetFullCropRegion();
    fn GetFullCropRegion();
}
impl IWMResizerPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMResizerPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMResizerPropsVtbl {
        unsafe extern "system" fn SetResizerQuality<Impl: IWMResizerPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquality: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInterlaceMode<Impl: IWMResizerPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipRegion<Impl: IWMResizerPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcliporixsrc: i32, lcliporiysrc: i32, lclipwidthsrc: i32, lclipheightsrc: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFullCropRegion<Impl: IWMResizerPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcliporixsrc: i32, lcliporiysrc: i32, lclipwidthsrc: i32, lclipheightsrc: i32, lcliporixdst: i32, lcliporiydst: i32, lclipwidthdst: i32, lclipheightdst: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFullCropRegion<Impl: IWMResizerPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcliporixsrc: *mut i32, lcliporiysrc: *mut i32, lclipwidthsrc: *mut i32, lclipheightsrc: *mut i32, lcliporixdst: *mut i32, lcliporiydst: *mut i32, lclipwidthdst: *mut i32, lclipheightdst: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetResizerQuality: SetResizerQuality::<Impl, IMPL_OFFSET>,
            SetInterlaceMode: SetInterlaceMode::<Impl, IMPL_OFFSET>,
            SetClipRegion: SetClipRegion::<Impl, IMPL_OFFSET>,
            SetFullCropRegion: SetFullCropRegion::<Impl, IMPL_OFFSET>,
            GetFullCropRegion: GetFullCropRegion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMResizerProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSampleExtensionSupportImpl: Sized {
    fn SetUseSampleExtensions();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSampleExtensionSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSampleExtensionSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSampleExtensionSupportVtbl {
        unsafe extern "system" fn SetUseSampleExtensions<Impl: IWMSampleExtensionSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuseextensions: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetUseSampleExtensions: SetUseSampleExtensions::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSampleExtensionSupport as ::windows::core::Interface>::IID
    }
}
pub trait IWMValidateImpl: Sized {
    fn SetIdentifier();
}
impl IWMValidateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMValidateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMValidateVtbl {
        unsafe extern "system" fn SetIdentifier<Impl: IWMValidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidvalidationid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetIdentifier: SetIdentifier::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMValidate as ::windows::core::Interface>::IID
    }
}
pub trait IWMVideoDecoderHurryupImpl: Sized {
    fn SetHurryup();
    fn GetHurryup();
}
impl IWMVideoDecoderHurryupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMVideoDecoderHurryupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMVideoDecoderHurryupVtbl {
        unsafe extern "system" fn SetHurryup<Impl: IWMVideoDecoderHurryupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhurryup: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHurryup<Impl: IWMVideoDecoderHurryupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhurryup: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetHurryup: SetHurryup::<Impl, IMPL_OFFSET>,
            GetHurryup: GetHurryup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMVideoDecoderHurryup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DxMediaObjects")]
pub trait IWMVideoDecoderReconBufferImpl: Sized {
    fn GetReconstructedVideoFrameSize();
    fn GetReconstructedVideoFrame();
    fn SetReconstructedVideoFrame();
}
#[cfg(feature = "Win32_Media_DxMediaObjects")]
impl IWMVideoDecoderReconBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMVideoDecoderReconBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMVideoDecoderReconBufferVtbl {
        unsafe extern "system" fn GetReconstructedVideoFrameSize<Impl: IWMVideoDecoderReconBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReconstructedVideoFrame<Impl: IWMVideoDecoderReconBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuf: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReconstructedVideoFrame<Impl: IWMVideoDecoderReconBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuf: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetReconstructedVideoFrameSize: GetReconstructedVideoFrameSize::<Impl, IMPL_OFFSET>,
            GetReconstructedVideoFrame: GetReconstructedVideoFrame::<Impl, IMPL_OFFSET>,
            SetReconstructedVideoFrame: SetReconstructedVideoFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMVideoDecoderReconBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IWMVideoForceKeyFrameImpl: Sized {
    fn SetKeyFrame();
}
impl IWMVideoForceKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMVideoForceKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMVideoForceKeyFrameVtbl {
        unsafe extern "system" fn SetKeyFrame<Impl: IWMVideoForceKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetKeyFrame: SetKeyFrame::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMVideoForceKeyFrame as ::windows::core::Interface>::IID
    }
}
pub trait MFASYNCRESULTImpl: Sized + IMFAsyncResultImpl {}
impl MFASYNCRESULTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: MFASYNCRESULTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> MFASYNCRESULTVtbl {
        Self { base: IMFAsyncResultVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<MFASYNCRESULT as ::windows::core::Interface>::IID
    }
}
