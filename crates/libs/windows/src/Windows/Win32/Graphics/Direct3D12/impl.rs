pub trait ID3D12CommandAllocatorImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn Reset();
}
impl ::windows::core::RuntimeName for ID3D12CommandAllocator {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12CommandAllocator";
}
impl ID3D12CommandAllocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandAllocatorImpl, const OFFSET: isize>() -> ID3D12CommandAllocatorVtbl {
        unsafe extern "system" fn Reset<Impl: ID3D12CommandAllocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12CommandAllocator>, ::windows::core::GetTrustLevel, Reset::<Impl, OFFSET>)
    }
}
pub trait ID3D12CommandListImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetType();
}
impl ::windows::core::RuntimeName for ID3D12CommandList {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12CommandList";
}
impl ID3D12CommandListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandListImpl, const OFFSET: isize>() -> ID3D12CommandListVtbl {
        unsafe extern "system" fn GetType<Impl: ID3D12CommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_COMMAND_LIST_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12CommandList>, ::windows::core::GetTrustLevel, GetType::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for ID3D12CommandQueue {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12CommandQueue";
}
impl ID3D12CommandQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueueImpl, const OFFSET: isize>() -> ID3D12CommandQueueVtbl {
        unsafe extern "system" fn UpdateTileMappings<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, numresourceregions: u32, presourceregionstartcoordinates: *const D3D12_TILED_RESOURCE_COORDINATE, presourceregionsizes: *const D3D12_TILE_REGION_SIZE, pheap: ::windows::core::RawPtr, numranges: u32, prangeflags: *const D3D12_TILE_RANGE_FLAGS, pheaprangestartoffsets: *const u32, prangetilecounts: *const u32, flags: D3D12_TILE_MAPPING_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .UpdateTileMappings(
                    &*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    numresourceregions,
                    &*(&presourceregionstartcoordinates as *const <D3D12_TILED_RESOURCE_COORDINATE as ::windows::core::Abi>::Abi as *const <D3D12_TILED_RESOURCE_COORDINATE as ::windows::core::DefaultType>::DefaultType),
                    &*(&presourceregionsizes as *const <D3D12_TILE_REGION_SIZE as ::windows::core::Abi>::Abi as *const <D3D12_TILE_REGION_SIZE as ::windows::core::DefaultType>::DefaultType),
                    &*(&pheap as *const <ID3D12Heap as ::windows::core::Abi>::Abi as *const <ID3D12Heap as ::windows::core::DefaultType>::DefaultType),
                    numranges,
                    prangeflags,
                    pheaprangestartoffsets,
                    prangetilecounts,
                    flags,
                )
                .into()
        }
        unsafe extern "system" fn CopyTileMappings<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: ::windows::core::RawPtr, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CopyTileMappings(
                    &*(&pdstresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdstregionstartcoordinate as *const <D3D12_TILED_RESOURCE_COORDINATE as ::windows::core::Abi>::Abi as *const <D3D12_TILED_RESOURCE_COORDINATE as ::windows::core::DefaultType>::DefaultType),
                    &*(&psrcresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&psrcregionstartcoordinate as *const <D3D12_TILED_RESOURCE_COORDINATE as ::windows::core::Abi>::Abi as *const <D3D12_TILED_RESOURCE_COORDINATE as ::windows::core::DefaultType>::DefaultType),
                    &*(&pregionsize as *const <D3D12_TILE_REGION_SIZE as ::windows::core::Abi>::Abi as *const <D3D12_TILE_REGION_SIZE as ::windows::core::DefaultType>::DefaultType),
                    flags,
                )
                .into()
        }
        unsafe extern "system" fn ExecuteCommandLists<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numcommandlists: u32, ppcommandlists: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteCommandLists(numcommandlists, &*(&ppcommandlists as *const <ID3D12CommandList as ::windows::core::Abi>::Abi as *const <ID3D12CommandList as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMarker<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMarker(metadata, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), size).into()
        }
        unsafe extern "system" fn BeginEvent<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEvent(metadata, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), size).into()
        }
        unsafe extern "system" fn EndEvent<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEvent().into()
        }
        unsafe extern "system" fn Signal<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signal(&*(&pfence as *const <ID3D12Fence as ::windows::core::Abi>::Abi as *const <ID3D12Fence as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wait<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wait(&*(&pfence as *const <ID3D12Fence as ::windows::core::Abi>::Abi as *const <ID3D12Fence as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimestampFrequency<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequency: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimestampFrequency(::core::mem::transmute_copy(&pfrequency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClockCalibration<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClockCalibration(::core::mem::transmute_copy(&pgputimestamp), ::core::mem::transmute_copy(&pcputimestamp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D12CommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_COMMAND_QUEUE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12CommandQueue>,
            ::windows::core::GetTrustLevel,
            UpdateTileMappings::<Impl, OFFSET>,
            CopyTileMappings::<Impl, OFFSET>,
            ExecuteCommandLists::<Impl, OFFSET>,
            SetMarker::<Impl, OFFSET>,
            BeginEvent::<Impl, OFFSET>,
            EndEvent::<Impl, OFFSET>,
            Signal::<Impl, OFFSET>,
            Wait::<Impl, OFFSET>,
            GetTimestampFrequency::<Impl, OFFSET>,
            GetClockCalibration::<Impl, OFFSET>,
            GetDesc::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D12CommandSignatureImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {}
impl ::windows::core::RuntimeName for ID3D12CommandSignature {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12CommandSignature";
}
impl ID3D12CommandSignatureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandSignatureImpl, const OFFSET: isize>() -> ID3D12CommandSignatureVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12CommandSignature>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D12DebugImpl: Sized {
    fn EnableDebugLayer();
}
impl ::windows::core::RuntimeName for ID3D12Debug {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Debug";
}
impl ID3D12DebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugImpl, const OFFSET: isize>() -> ID3D12DebugVtbl {
        unsafe extern "system" fn EnableDebugLayer<Impl: ID3D12DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableDebugLayer().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Debug>, ::windows::core::GetTrustLevel, EnableDebugLayer::<Impl, OFFSET>)
    }
}
pub trait ID3D12Debug1Impl: Sized {
    fn EnableDebugLayer();
    fn SetEnableGPUBasedValidation();
    fn SetEnableSynchronizedCommandQueueValidation();
}
impl ::windows::core::RuntimeName for ID3D12Debug1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Debug1";
}
impl ID3D12Debug1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug1Impl, const OFFSET: isize>() -> ID3D12Debug1Vtbl {
        unsafe extern "system" fn EnableDebugLayer<Impl: ID3D12Debug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableDebugLayer().into()
        }
        unsafe extern "system" fn SetEnableGPUBasedValidation<Impl: ID3D12Debug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableGPUBasedValidation(&*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetEnableSynchronizedCommandQueueValidation<Impl: ID3D12Debug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableSynchronizedCommandQueueValidation(&*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Debug1>, ::windows::core::GetTrustLevel, EnableDebugLayer::<Impl, OFFSET>, SetEnableGPUBasedValidation::<Impl, OFFSET>, SetEnableSynchronizedCommandQueueValidation::<Impl, OFFSET>)
    }
}
pub trait ID3D12Debug2Impl: Sized {
    fn SetGPUBasedValidationFlags();
}
impl ::windows::core::RuntimeName for ID3D12Debug2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Debug2";
}
impl ID3D12Debug2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug2Impl, const OFFSET: isize>() -> ID3D12Debug2Vtbl {
        unsafe extern "system" fn SetGPUBasedValidationFlags<Impl: ID3D12Debug2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGPUBasedValidationFlags(flags).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Debug2>, ::windows::core::GetTrustLevel, SetGPUBasedValidationFlags::<Impl, OFFSET>)
    }
}
pub trait ID3D12Debug3Impl: Sized + ID3D12DebugImpl {
    fn SetEnableGPUBasedValidation();
    fn SetEnableSynchronizedCommandQueueValidation();
    fn SetGPUBasedValidationFlags();
}
impl ::windows::core::RuntimeName for ID3D12Debug3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Debug3";
}
impl ID3D12Debug3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug3Impl, const OFFSET: isize>() -> ID3D12Debug3Vtbl {
        unsafe extern "system" fn SetEnableGPUBasedValidation<Impl: ID3D12Debug3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableGPUBasedValidation(&*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetEnableSynchronizedCommandQueueValidation<Impl: ID3D12Debug3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableSynchronizedCommandQueueValidation(&*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetGPUBasedValidationFlags<Impl: ID3D12Debug3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGPUBasedValidationFlags(flags).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Debug3>, ::windows::core::GetTrustLevel, SetEnableGPUBasedValidation::<Impl, OFFSET>, SetEnableSynchronizedCommandQueueValidation::<Impl, OFFSET>, SetGPUBasedValidationFlags::<Impl, OFFSET>)
    }
}
pub trait ID3D12Debug4Impl: Sized + ID3D12Debug3Impl + ID3D12DebugImpl {
    fn DisableDebugLayer();
}
impl ::windows::core::RuntimeName for ID3D12Debug4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Debug4";
}
impl ID3D12Debug4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug4Impl, const OFFSET: isize>() -> ID3D12Debug4Vtbl {
        unsafe extern "system" fn DisableDebugLayer<Impl: ID3D12Debug4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableDebugLayer().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Debug4>, ::windows::core::GetTrustLevel, DisableDebugLayer::<Impl, OFFSET>)
    }
}
pub trait ID3D12Debug5Impl: Sized + ID3D12Debug4Impl + ID3D12Debug3Impl + ID3D12DebugImpl {
    fn SetEnableAutoName();
}
impl ::windows::core::RuntimeName for ID3D12Debug5 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Debug5";
}
impl ID3D12Debug5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug5Impl, const OFFSET: isize>() -> ID3D12Debug5Vtbl {
        unsafe extern "system" fn SetEnableAutoName<Impl: ID3D12Debug5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableAutoName(&*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Debug5>, ::windows::core::GetTrustLevel, SetEnableAutoName::<Impl, OFFSET>)
    }
}
pub trait ID3D12DebugCommandListImpl: Sized {
    fn AssertResourceState();
    fn SetFeatureMask();
    fn GetFeatureMask();
}
impl ::windows::core::RuntimeName for ID3D12DebugCommandList {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DebugCommandList";
}
impl ID3D12DebugCommandListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandListImpl, const OFFSET: isize>() -> ID3D12DebugCommandListVtbl {
        unsafe extern "system" fn AssertResourceState<Impl: ID3D12DebugCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssertResourceState(&*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), subresource, state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFeatureMask<Impl: ID3D12DebugCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFeatureMask(mask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureMask<Impl: ID3D12DebugCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_DEBUG_FEATURE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeatureMask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DebugCommandList>, ::windows::core::GetTrustLevel, AssertResourceState::<Impl, OFFSET>, SetFeatureMask::<Impl, OFFSET>, GetFeatureMask::<Impl, OFFSET>)
    }
}
pub trait ID3D12DebugCommandList1Impl: Sized {
    fn AssertResourceState();
    fn SetDebugParameter();
    fn GetDebugParameter();
}
impl ::windows::core::RuntimeName for ID3D12DebugCommandList1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DebugCommandList1";
}
impl ID3D12DebugCommandList1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList1Impl, const OFFSET: isize>() -> ID3D12DebugCommandList1Vtbl {
        unsafe extern "system" fn AssertResourceState<Impl: ID3D12DebugCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssertResourceState(&*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), subresource, state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDebugParameter<Impl: ID3D12DebugCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDebugParameter(r#type, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDebugParameter<Impl: ID3D12DebugCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDebugParameter(r#type, ::core::mem::transmute_copy(&pdata), datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DebugCommandList1>, ::windows::core::GetTrustLevel, AssertResourceState::<Impl, OFFSET>, SetDebugParameter::<Impl, OFFSET>, GetDebugParameter::<Impl, OFFSET>)
    }
}
pub trait ID3D12DebugCommandList2Impl: Sized + ID3D12DebugCommandListImpl {
    fn SetDebugParameter();
    fn GetDebugParameter();
}
impl ::windows::core::RuntimeName for ID3D12DebugCommandList2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DebugCommandList2";
}
impl ID3D12DebugCommandList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList2Impl, const OFFSET: isize>() -> ID3D12DebugCommandList2Vtbl {
        unsafe extern "system" fn SetDebugParameter<Impl: ID3D12DebugCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDebugParameter(r#type, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDebugParameter<Impl: ID3D12DebugCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDebugParameter(r#type, ::core::mem::transmute_copy(&pdata), datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DebugCommandList2>, ::windows::core::GetTrustLevel, SetDebugParameter::<Impl, OFFSET>, GetDebugParameter::<Impl, OFFSET>)
    }
}
pub trait ID3D12DebugCommandQueueImpl: Sized {
    fn AssertResourceState();
}
impl ::windows::core::RuntimeName for ID3D12DebugCommandQueue {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DebugCommandQueue";
}
impl ID3D12DebugCommandQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandQueueImpl, const OFFSET: isize>() -> ID3D12DebugCommandQueueVtbl {
        unsafe extern "system" fn AssertResourceState<Impl: ID3D12DebugCommandQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssertResourceState(&*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), subresource, state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DebugCommandQueue>, ::windows::core::GetTrustLevel, AssertResourceState::<Impl, OFFSET>)
    }
}
pub trait ID3D12DebugDeviceImpl: Sized {
    fn SetFeatureMask();
    fn GetFeatureMask();
    fn ReportLiveDeviceObjects();
}
impl ::windows::core::RuntimeName for ID3D12DebugDevice {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DebugDevice";
}
impl ID3D12DebugDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDeviceImpl, const OFFSET: isize>() -> ID3D12DebugDeviceVtbl {
        unsafe extern "system" fn SetFeatureMask<Impl: ID3D12DebugDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFeatureMask(mask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureMask<Impl: ID3D12DebugDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_DEBUG_FEATURE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeatureMask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Impl: ID3D12DebugDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLiveDeviceObjects(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DebugDevice>, ::windows::core::GetTrustLevel, SetFeatureMask::<Impl, OFFSET>, GetFeatureMask::<Impl, OFFSET>, ReportLiveDeviceObjects::<Impl, OFFSET>)
    }
}
pub trait ID3D12DebugDevice1Impl: Sized {
    fn SetDebugParameter();
    fn GetDebugParameter();
    fn ReportLiveDeviceObjects();
}
impl ::windows::core::RuntimeName for ID3D12DebugDevice1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DebugDevice1";
}
impl ID3D12DebugDevice1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice1Impl, const OFFSET: isize>() -> ID3D12DebugDevice1Vtbl {
        unsafe extern "system" fn SetDebugParameter<Impl: ID3D12DebugDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDebugParameter(r#type, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDebugParameter<Impl: ID3D12DebugDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDebugParameter(r#type, ::core::mem::transmute_copy(&pdata), datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Impl: ID3D12DebugDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLiveDeviceObjects(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DebugDevice1>, ::windows::core::GetTrustLevel, SetDebugParameter::<Impl, OFFSET>, GetDebugParameter::<Impl, OFFSET>, ReportLiveDeviceObjects::<Impl, OFFSET>)
    }
}
pub trait ID3D12DebugDevice2Impl: Sized + ID3D12DebugDeviceImpl {
    fn SetDebugParameter();
    fn GetDebugParameter();
}
impl ::windows::core::RuntimeName for ID3D12DebugDevice2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DebugDevice2";
}
impl ID3D12DebugDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice2Impl, const OFFSET: isize>() -> ID3D12DebugDevice2Vtbl {
        unsafe extern "system" fn SetDebugParameter<Impl: ID3D12DebugDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDebugParameter(r#type, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDebugParameter<Impl: ID3D12DebugDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDebugParameter(r#type, ::core::mem::transmute_copy(&pdata), datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DebugDevice2>, ::windows::core::GetTrustLevel, SetDebugParameter::<Impl, OFFSET>, GetDebugParameter::<Impl, OFFSET>)
    }
}
pub trait ID3D12DescriptorHeapImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
    fn GetCPUDescriptorHandleForHeapStart();
    fn GetGPUDescriptorHandleForHeapStart();
}
impl ::windows::core::RuntimeName for ID3D12DescriptorHeap {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DescriptorHeap";
}
impl ID3D12DescriptorHeapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DescriptorHeapImpl, const OFFSET: isize>() -> ID3D12DescriptorHeapVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12DescriptorHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_DESCRIPTOR_HEAP_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCPUDescriptorHandleForHeapStart<Impl: ID3D12DescriptorHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCPUDescriptorHandleForHeapStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPUDescriptorHandleForHeapStart<Impl: ID3D12DescriptorHeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGPUDescriptorHandleForHeapStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DescriptorHeap>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, GetCPUDescriptorHandleForHeapStart::<Impl, OFFSET>, GetGPUDescriptorHandleForHeapStart::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for ID3D12Device {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Device";
}
impl ID3D12DeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceImpl, const OFFSET: isize>() -> ID3D12DeviceVtbl {
        unsafe extern "system" fn GetNodeCount<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNodeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommandQueue<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, riid: *const ::windows::core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommandQueue(&*(&pdesc as *const <D3D12_COMMAND_QUEUE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_COMMAND_QUEUE_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcommandqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommandAllocator<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_COMMAND_LIST_TYPE, riid: *const ::windows::core::GUID, ppcommandallocator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommandAllocator(r#type, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcommandallocator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGraphicsPipelineState<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGraphicsPipelineState(&*(&pdesc as *const <D3D12_GRAPHICS_PIPELINE_STATE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_GRAPHICS_PIPELINE_STATE_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppipelinestate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComputePipelineState<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComputePipelineState(&*(&pdesc as *const <D3D12_COMPUTE_PIPELINE_STATE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_COMPUTE_PIPELINE_STATE_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppipelinestate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommandList<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: ::windows::core::RawPtr, pinitialstate: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommandList(
                nodemask,
                r#type,
                &*(&pcommandallocator as *const <ID3D12CommandAllocator as ::windows::core::Abi>::Abi as *const <ID3D12CommandAllocator as ::windows::core::DefaultType>::DefaultType),
                &*(&pinitialstate as *const <ID3D12PipelineState as ::windows::core::Abi>::Abi as *const <ID3D12PipelineState as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppcommandlist),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckFeatureSupport(feature, &*(&pfeaturesupportdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), featuresupportdatasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDescriptorHeap<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDescriptorHeap(&*(&pdescriptorheapdesc as *const <D3D12_DESCRIPTOR_HEAP_DESC as ::windows::core::Abi>::Abi as *const <D3D12_DESCRIPTOR_HEAP_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvheap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptorHandleIncrementSize<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptorHandleIncrementSize(descriptorheaptype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRootSignature<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, pblobwithrootsignature: *const ::core::ffi::c_void, bloblengthinbytes: usize, riid: *const ::windows::core::GUID, ppvrootsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRootSignature(nodemask, &*(&pblobwithrootsignature as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bloblengthinbytes, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvrootsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantBufferView<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_CONSTANT_BUFFER_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateConstantBufferView(&*(&pdesc as *const <D3D12_CONSTANT_BUFFER_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D12_CONSTANT_BUFFER_VIEW_DESC as ::windows::core::DefaultType>::DefaultType), &*(&destdescriptor as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateShaderResourceView<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D12_SHADER_RESOURCE_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CreateShaderResourceView(
                    &*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdesc as *const <D3D12_SHADER_RESOURCE_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D12_SHADER_RESOURCE_VIEW_DESC as ::windows::core::DefaultType>::DefaultType),
                    &*(&destdescriptor as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pcounterresource: ::windows::core::RawPtr, pdesc: *const D3D12_UNORDERED_ACCESS_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CreateUnorderedAccessView(
                    &*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&pcounterresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdesc as *const <D3D12_UNORDERED_ACCESS_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D12_UNORDERED_ACCESS_VIEW_DESC as ::windows::core::DefaultType>::DefaultType),
                    &*(&destdescriptor as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn CreateRenderTargetView<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D12_RENDER_TARGET_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CreateRenderTargetView(
                    &*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdesc as *const <D3D12_RENDER_TARGET_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RENDER_TARGET_VIEW_DESC as ::windows::core::DefaultType>::DefaultType),
                    &*(&destdescriptor as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn CreateDepthStencilView<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D12_DEPTH_STENCIL_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CreateDepthStencilView(
                    &*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdesc as *const <D3D12_DEPTH_STENCIL_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D12_DEPTH_STENCIL_VIEW_DESC as ::windows::core::DefaultType>::DefaultType),
                    &*(&destdescriptor as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn CreateSampler<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSampler(&*(&pdesc as *const <D3D12_SAMPLER_DESC as ::windows::core::Abi>::Abi as *const <D3D12_SAMPLER_DESC as ::windows::core::DefaultType>::DefaultType), &*(&destdescriptor as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyDescriptors<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: *const u32, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: *const u32, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CopyDescriptors(
                    numdestdescriptorranges,
                    &*(&pdestdescriptorrangestarts as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                    pdestdescriptorrangesizes,
                    numsrcdescriptorranges,
                    &*(&psrcdescriptorrangestarts as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                    psrcdescriptorrangesizes,
                    descriptorheapstype,
                )
                .into()
        }
        unsafe extern "system" fn CopyDescriptorsSimple<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyDescriptorsSimple(numdescriptors, &*(&destdescriptorrangestart as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&srcdescriptorrangestart as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType), descriptorheapstype).into()
        }
        unsafe extern "system" fn GetResourceAllocationInfo<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceAllocationInfo(visiblemask, numresourcedescs, &*(&presourcedescs as *const <D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomHeapProperties<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_HEAP_PROPERTIES, nodemask: u32, heaptype: D3D12_HEAP_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomHeapProperties(nodemask, heaptype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommittedResource<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommittedResource(
                &*(&pheapproperties as *const <D3D12_HEAP_PROPERTIES as ::windows::core::Abi>::Abi as *const <D3D12_HEAP_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                heapflags,
                &*(&pdesc as *const <D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType),
                initialresourcestate,
                &*(&poptimizedclearvalue as *const <D3D12_CLEAR_VALUE as ::windows::core::Abi>::Abi as *const <D3D12_CLEAR_VALUE as ::windows::core::DefaultType>::DefaultType),
                &*(&riidresource as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHeap<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHeap(&*(&pdesc as *const <D3D12_HEAP_DESC as ::windows::core::Abi>::Abi as *const <D3D12_HEAP_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvheap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePlacedResource<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheap: ::windows::core::RawPtr, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePlacedResource(
                &*(&pheap as *const <ID3D12Heap as ::windows::core::Abi>::Abi as *const <ID3D12Heap as ::windows::core::DefaultType>::DefaultType),
                heapoffset,
                &*(&pdesc as *const <D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType),
                initialstate,
                &*(&poptimizedclearvalue as *const <D3D12_CLEAR_VALUE as ::windows::core::Abi>::Abi as *const <D3D12_CLEAR_VALUE as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReservedResource<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateReservedResource(
                &*(&pdesc as *const <D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType),
                initialstate,
                &*(&poptimizedclearvalue as *const <D3D12_CLEAR_VALUE as ::windows::core::Abi>::Abi as *const <D3D12_CLEAR_VALUE as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSharedHandle<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: super::super::Foundation::PWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSharedHandle(
                &*(&pobject as *const <ID3D12DeviceChild as ::windows::core::Abi>::Abi as *const <ID3D12DeviceChild as ::windows::core::DefaultType>::DefaultType),
                &*(&pattributes as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType),
                access,
                &*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&phandle),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedHandle<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nthandle: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenSharedHandle(&*(&nthandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedHandleByName<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, access: u32, pnthandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenSharedHandleByName(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), access, ::core::mem::transmute_copy(&pnthandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeResident<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MakeResident(numobjects, &*(&ppobjects as *const <ID3D12Pageable as ::windows::core::Abi>::Abi as *const <ID3D12Pageable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Evict<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Evict(numobjects, &*(&ppobjects as *const <ID3D12Pageable as ::windows::core::Abi>::Abi as *const <ID3D12Pageable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFence<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: u64, flags: D3D12_FENCE_FLAGS, riid: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFence(initialvalue, flags, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceRemovedReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCopyableFootprints<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCopyableFootprints(&*(&presourcedesc as *const <D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType), firstsubresource, numsubresources, baseoffset, ::core::mem::transmute_copy(&playouts), ::core::mem::transmute_copy(&pnumrows), ::core::mem::transmute_copy(&prowsizeinbytes), ::core::mem::transmute_copy(&ptotalbytes)).into()
        }
        unsafe extern "system" fn CreateQueryHeap<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_QUERY_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryHeap(&*(&pdesc as *const <D3D12_QUERY_HEAP_DESC as ::windows::core::Abi>::Abi as *const <D3D12_QUERY_HEAP_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvheap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStablePowerState<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStablePowerState(&*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommandSignature<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvcommandsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommandSignature(
                &*(&pdesc as *const <D3D12_COMMAND_SIGNATURE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_COMMAND_SIGNATURE_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&prootsignature as *const <ID3D12RootSignature as ::windows::core::Abi>::Abi as *const <ID3D12RootSignature as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvcommandsignature),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceTiling<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D12_PACKED_MIP_INFO, pstandardtileshapefornonpackedmips: *mut D3D12_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResourceTiling(&*(&ptiledresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnumtilesforentireresource), ::core::mem::transmute_copy(&ppackedmipdesc), ::core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), pnumsubresourcetilings, firstsubresourcetilingtoget, ::core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips)).into()
        }
        unsafe extern "system" fn GetAdapterLuid<Impl: ID3D12DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdapterLuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12Device>,
            ::windows::core::GetTrustLevel,
            GetNodeCount::<Impl, OFFSET>,
            CreateCommandQueue::<Impl, OFFSET>,
            CreateCommandAllocator::<Impl, OFFSET>,
            CreateGraphicsPipelineState::<Impl, OFFSET>,
            CreateComputePipelineState::<Impl, OFFSET>,
            CreateCommandList::<Impl, OFFSET>,
            CheckFeatureSupport::<Impl, OFFSET>,
            CreateDescriptorHeap::<Impl, OFFSET>,
            GetDescriptorHandleIncrementSize::<Impl, OFFSET>,
            CreateRootSignature::<Impl, OFFSET>,
            CreateConstantBufferView::<Impl, OFFSET>,
            CreateShaderResourceView::<Impl, OFFSET>,
            CreateUnorderedAccessView::<Impl, OFFSET>,
            CreateRenderTargetView::<Impl, OFFSET>,
            CreateDepthStencilView::<Impl, OFFSET>,
            CreateSampler::<Impl, OFFSET>,
            CopyDescriptors::<Impl, OFFSET>,
            CopyDescriptorsSimple::<Impl, OFFSET>,
            GetResourceAllocationInfo::<Impl, OFFSET>,
            GetCustomHeapProperties::<Impl, OFFSET>,
            CreateCommittedResource::<Impl, OFFSET>,
            CreateHeap::<Impl, OFFSET>,
            CreatePlacedResource::<Impl, OFFSET>,
            CreateReservedResource::<Impl, OFFSET>,
            CreateSharedHandle::<Impl, OFFSET>,
            OpenSharedHandle::<Impl, OFFSET>,
            OpenSharedHandleByName::<Impl, OFFSET>,
            MakeResident::<Impl, OFFSET>,
            Evict::<Impl, OFFSET>,
            CreateFence::<Impl, OFFSET>,
            GetDeviceRemovedReason::<Impl, OFFSET>,
            GetCopyableFootprints::<Impl, OFFSET>,
            CreateQueryHeap::<Impl, OFFSET>,
            SetStablePowerState::<Impl, OFFSET>,
            CreateCommandSignature::<Impl, OFFSET>,
            GetResourceTiling::<Impl, OFFSET>,
            GetAdapterLuid::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D12Device1Impl: Sized + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn CreatePipelineLibrary();
    fn SetEventOnMultipleFenceCompletion();
    fn SetResidencyPriority();
}
impl ::windows::core::RuntimeName for ID3D12Device1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Device1";
}
impl ID3D12Device1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device1Impl, const OFFSET: isize>() -> ID3D12Device1Vtbl {
        unsafe extern "system" fn CreatePipelineLibrary<Impl: ID3D12Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibraryblob: *const ::core::ffi::c_void, bloblength: usize, riid: *const ::windows::core::GUID, pppipelinelibrary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePipelineLibrary(&*(&plibraryblob as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bloblength, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppipelinelibrary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventOnMultipleFenceCompletion<Impl: ID3D12Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfences: *const ::windows::core::RawPtr, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventOnMultipleFenceCompletion(&*(&ppfences as *const <ID3D12Fence as ::windows::core::Abi>::Abi as *const <ID3D12Fence as ::windows::core::DefaultType>::DefaultType), pfencevalues, numfences, flags, &*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResidencyPriority<Impl: ID3D12Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const ::windows::core::RawPtr, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetResidencyPriority(numobjects, &*(&ppobjects as *const <ID3D12Pageable as ::windows::core::Abi>::Abi as *const <ID3D12Pageable as ::windows::core::DefaultType>::DefaultType), ppriorities) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Device1>, ::windows::core::GetTrustLevel, CreatePipelineLibrary::<Impl, OFFSET>, SetEventOnMultipleFenceCompletion::<Impl, OFFSET>, SetResidencyPriority::<Impl, OFFSET>)
    }
}
pub trait ID3D12Device2Impl: Sized + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn CreatePipelineState();
}
impl ::windows::core::RuntimeName for ID3D12Device2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Device2";
}
impl ID3D12Device2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device2Impl, const OFFSET: isize>() -> ID3D12Device2Vtbl {
        unsafe extern "system" fn CreatePipelineState<Impl: ID3D12Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePipelineState(&*(&pdesc as *const <D3D12_PIPELINE_STATE_STREAM_DESC as ::windows::core::Abi>::Abi as *const <D3D12_PIPELINE_STATE_STREAM_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppipelinestate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Device2>, ::windows::core::GetTrustLevel, CreatePipelineState::<Impl, OFFSET>)
    }
}
pub trait ID3D12Device3Impl: Sized + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn OpenExistingHeapFromAddress();
    fn OpenExistingHeapFromFileMapping();
    fn EnqueueMakeResident();
}
impl ::windows::core::RuntimeName for ID3D12Device3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Device3";
}
impl ID3D12Device3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device3Impl, const OFFSET: isize>() -> ID3D12Device3Vtbl {
        unsafe extern "system" fn OpenExistingHeapFromAddress<Impl: ID3D12Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenExistingHeapFromAddress(&*(&paddress as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvheap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenExistingHeapFromFileMapping<Impl: ID3D12Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfilemapping: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenExistingHeapFromFileMapping(&*(&hfilemapping as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvheap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnqueueMakeResident<Impl: ID3D12Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_RESIDENCY_FLAGS, numobjects: u32, ppobjects: *const ::windows::core::RawPtr, pfencetosignal: ::windows::core::RawPtr, fencevaluetosignal: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnqueueMakeResident(flags, numobjects, &*(&ppobjects as *const <ID3D12Pageable as ::windows::core::Abi>::Abi as *const <ID3D12Pageable as ::windows::core::DefaultType>::DefaultType), &*(&pfencetosignal as *const <ID3D12Fence as ::windows::core::Abi>::Abi as *const <ID3D12Fence as ::windows::core::DefaultType>::DefaultType), fencevaluetosignal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Device3>, ::windows::core::GetTrustLevel, OpenExistingHeapFromAddress::<Impl, OFFSET>, OpenExistingHeapFromFileMapping::<Impl, OFFSET>, EnqueueMakeResident::<Impl, OFFSET>)
    }
}
pub trait ID3D12Device4Impl: Sized + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn CreateCommandList1();
    fn CreateProtectedResourceSession();
    fn CreateCommittedResource1();
    fn CreateHeap1();
    fn CreateReservedResource1();
    fn GetResourceAllocationInfo1();
}
impl ::windows::core::RuntimeName for ID3D12Device4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Device4";
}
impl ID3D12Device4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device4Impl, const OFFSET: isize>() -> ID3D12Device4Vtbl {
        unsafe extern "system" fn CreateCommandList1<Impl: ID3D12Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS, riid: *const ::windows::core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommandList1(nodemask, r#type, flags, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcommandlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProtectedResourceSession<Impl: ID3D12Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProtectedResourceSession(&*(&pdesc as *const <D3D12_PROTECTED_RESOURCE_SESSION_DESC as ::windows::core::Abi>::Abi as *const <D3D12_PROTECTED_RESOURCE_SESSION_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommittedResource1<Impl: ID3D12Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::windows::core::RawPtr, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommittedResource1(
                &*(&pheapproperties as *const <D3D12_HEAP_PROPERTIES as ::windows::core::Abi>::Abi as *const <D3D12_HEAP_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                heapflags,
                &*(&pdesc as *const <D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType),
                initialresourcestate,
                &*(&poptimizedclearvalue as *const <D3D12_CLEAR_VALUE as ::windows::core::Abi>::Abi as *const <D3D12_CLEAR_VALUE as ::windows::core::DefaultType>::DefaultType),
                &*(&pprotectedsession as *const <ID3D12ProtectedResourceSession as ::windows::core::Abi>::Abi as *const <ID3D12ProtectedResourceSession as ::windows::core::DefaultType>::DefaultType),
                &*(&riidresource as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHeap1<Impl: ID3D12Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHeap1(
                &*(&pdesc as *const <D3D12_HEAP_DESC as ::windows::core::Abi>::Abi as *const <D3D12_HEAP_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&pprotectedsession as *const <ID3D12ProtectedResourceSession as ::windows::core::Abi>::Abi as *const <ID3D12ProtectedResourceSession as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvheap),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReservedResource1<Impl: ID3D12Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateReservedResource1(
                &*(&pdesc as *const <D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType),
                initialstate,
                &*(&poptimizedclearvalue as *const <D3D12_CLEAR_VALUE as ::windows::core::Abi>::Abi as *const <D3D12_CLEAR_VALUE as ::windows::core::DefaultType>::DefaultType),
                &*(&pprotectedsession as *const <ID3D12ProtectedResourceSession as ::windows::core::Abi>::Abi as *const <ID3D12ProtectedResourceSession as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceAllocationInfo1<Impl: ID3D12Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceAllocationInfo1(visiblemask, numresourcedescs, &*(&presourcedescs as *const <D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presourceallocationinfo1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12Device4>,
            ::windows::core::GetTrustLevel,
            CreateCommandList1::<Impl, OFFSET>,
            CreateProtectedResourceSession::<Impl, OFFSET>,
            CreateCommittedResource1::<Impl, OFFSET>,
            CreateHeap1::<Impl, OFFSET>,
            CreateReservedResource1::<Impl, OFFSET>,
            GetResourceAllocationInfo1::<Impl, OFFSET>,
        )
    }
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
impl ::windows::core::RuntimeName for ID3D12Device5 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Device5";
}
impl ID3D12Device5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5Impl, const OFFSET: isize>() -> ID3D12Device5Vtbl {
        unsafe extern "system" fn CreateLifetimeTracker<Impl: ID3D12Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, powner: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvtracker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLifetimeTracker(&*(&powner as *const <ID3D12LifetimeOwner as ::windows::core::Abi>::Abi as *const <ID3D12LifetimeOwner as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvtracker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDevice<Impl: ID3D12Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDevice().into()
        }
        unsafe extern "system" fn EnumerateMetaCommands<Impl: ID3D12Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnummetacommands: *mut u32, pdescs: *mut D3D12_META_COMMAND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateMetaCommands(pnummetacommands, ::core::mem::transmute_copy(&pdescs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetaCommandParameters<Impl: ID3D12Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: *const ::windows::core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: *mut u32, pparametercount: *mut u32, pparameterdescs: *mut D3D12_META_COMMAND_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateMetaCommandParameters(&*(&commandid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), stage, ::core::mem::transmute_copy(&ptotalstructuresizeinbytes), pparametercount, ::core::mem::transmute_copy(&pparameterdescs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetaCommand<Impl: ID3D12Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: *const ::windows::core::GUID, nodemask: u32, pcreationparametersdata: *const ::core::ffi::c_void, creationparametersdatasizeinbytes: usize, riid: *const ::windows::core::GUID, ppmetacommand: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetaCommand(
                &*(&commandid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                nodemask,
                &*(&pcreationparametersdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                creationparametersdatasizeinbytes,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmetacommand),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStateObject<Impl: ID3D12Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_STATE_OBJECT_DESC, riid: *const ::windows::core::GUID, ppstateobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStateObject(&*(&pdesc as *const <D3D12_STATE_OBJECT_DESC as ::windows::core::Abi>::Abi as *const <D3D12_STATE_OBJECT_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstateobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRaytracingAccelerationStructurePrebuildInfo<Impl: ID3D12Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRaytracingAccelerationStructurePrebuildInfo(&*(&pdesc as *const <D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS as ::windows::core::Abi>::Abi as *const <D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn CheckDriverMatchingIdentifier<Impl: ID3D12Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckDriverMatchingIdentifier(serializeddatatype, &*(&pidentifiertocheck as *const <D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER as ::windows::core::Abi>::Abi as *const <D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12Device5>,
            ::windows::core::GetTrustLevel,
            CreateLifetimeTracker::<Impl, OFFSET>,
            RemoveDevice::<Impl, OFFSET>,
            EnumerateMetaCommands::<Impl, OFFSET>,
            EnumerateMetaCommandParameters::<Impl, OFFSET>,
            CreateMetaCommand::<Impl, OFFSET>,
            CreateStateObject::<Impl, OFFSET>,
            GetRaytracingAccelerationStructurePrebuildInfo::<Impl, OFFSET>,
            CheckDriverMatchingIdentifier::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D12Device6Impl: Sized + ID3D12Device5Impl + ID3D12Device4Impl + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn SetBackgroundProcessingMode();
}
impl ::windows::core::RuntimeName for ID3D12Device6 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Device6";
}
impl ID3D12Device6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device6Impl, const OFFSET: isize>() -> ID3D12Device6Vtbl {
        unsafe extern "system" fn SetBackgroundProcessingMode<Impl: ID3D12Device6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: super::super::Foundation::HANDLE, pbfurthermeasurementsdesired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackgroundProcessingMode(mode, measurementsaction, &*(&heventtosignaluponcompletion as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbfurthermeasurementsdesired)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Device6>, ::windows::core::GetTrustLevel, SetBackgroundProcessingMode::<Impl, OFFSET>)
    }
}
pub trait ID3D12Device7Impl: Sized + ID3D12Device6Impl + ID3D12Device5Impl + ID3D12Device4Impl + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn AddToStateObject();
    fn CreateProtectedResourceSession1();
}
impl ::windows::core::RuntimeName for ID3D12Device7 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Device7";
}
impl ID3D12Device7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device7Impl, const OFFSET: isize>() -> ID3D12Device7Vtbl {
        unsafe extern "system" fn AddToStateObject<Impl: ID3D12Device7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppnewstateobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddToStateObject(
                &*(&paddition as *const <D3D12_STATE_OBJECT_DESC as ::windows::core::Abi>::Abi as *const <D3D12_STATE_OBJECT_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&pstateobjecttogrowfrom as *const <ID3D12StateObject as ::windows::core::Abi>::Abi as *const <ID3D12StateObject as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppnewstateobject),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProtectedResourceSession1<Impl: ID3D12Device7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProtectedResourceSession1(&*(&pdesc as *const <D3D12_PROTECTED_RESOURCE_SESSION_DESC1 as ::windows::core::Abi>::Abi as *const <D3D12_PROTECTED_RESOURCE_SESSION_DESC1 as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Device7>, ::windows::core::GetTrustLevel, AddToStateObject::<Impl, OFFSET>, CreateProtectedResourceSession1::<Impl, OFFSET>)
    }
}
pub trait ID3D12Device8Impl: Sized + ID3D12Device7Impl + ID3D12Device6Impl + ID3D12Device5Impl + ID3D12Device4Impl + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn GetResourceAllocationInfo2();
    fn CreateCommittedResource2();
    fn CreatePlacedResource1();
    fn CreateSamplerFeedbackUnorderedAccessView();
    fn GetCopyableFootprints1();
}
impl ::windows::core::RuntimeName for ID3D12Device8 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Device8";
}
impl ID3D12Device8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device8Impl, const OFFSET: isize>() -> ID3D12Device8Vtbl {
        unsafe extern "system" fn GetResourceAllocationInfo2<Impl: ID3D12Device8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceAllocationInfo2(visiblemask, numresourcedescs, &*(&presourcedescs as *const <D3D12_RESOURCE_DESC1 as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presourceallocationinfo1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommittedResource2<Impl: ID3D12Device8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::windows::core::RawPtr, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommittedResource2(
                &*(&pheapproperties as *const <D3D12_HEAP_PROPERTIES as ::windows::core::Abi>::Abi as *const <D3D12_HEAP_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                heapflags,
                &*(&pdesc as *const <D3D12_RESOURCE_DESC1 as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC1 as ::windows::core::DefaultType>::DefaultType),
                initialresourcestate,
                &*(&poptimizedclearvalue as *const <D3D12_CLEAR_VALUE as ::windows::core::Abi>::Abi as *const <D3D12_CLEAR_VALUE as ::windows::core::DefaultType>::DefaultType),
                &*(&pprotectedsession as *const <ID3D12ProtectedResourceSession as ::windows::core::Abi>::Abi as *const <ID3D12ProtectedResourceSession as ::windows::core::DefaultType>::DefaultType),
                &*(&riidresource as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePlacedResource1<Impl: ID3D12Device8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheap: ::windows::core::RawPtr, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePlacedResource1(
                &*(&pheap as *const <ID3D12Heap as ::windows::core::Abi>::Abi as *const <ID3D12Heap as ::windows::core::DefaultType>::DefaultType),
                heapoffset,
                &*(&pdesc as *const <D3D12_RESOURCE_DESC1 as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC1 as ::windows::core::DefaultType>::DefaultType),
                initialstate,
                &*(&poptimizedclearvalue as *const <D3D12_CLEAR_VALUE as ::windows::core::Abi>::Abi as *const <D3D12_CLEAR_VALUE as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSamplerFeedbackUnorderedAccessView<Impl: ID3D12Device8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetedresource: ::windows::core::RawPtr, pfeedbackresource: ::windows::core::RawPtr, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CreateSamplerFeedbackUnorderedAccessView(&*(&ptargetedresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), &*(&pfeedbackresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), &*(&destdescriptor as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn GetCopyableFootprints1<Impl: ID3D12Device8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCopyableFootprints1(&*(&presourcedesc as *const <D3D12_RESOURCE_DESC1 as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_DESC1 as ::windows::core::DefaultType>::DefaultType), firstsubresource, numsubresources, baseoffset, ::core::mem::transmute_copy(&playouts), ::core::mem::transmute_copy(&pnumrows), ::core::mem::transmute_copy(&prowsizeinbytes), ::core::mem::transmute_copy(&ptotalbytes)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Device8>, ::windows::core::GetTrustLevel, GetResourceAllocationInfo2::<Impl, OFFSET>, CreateCommittedResource2::<Impl, OFFSET>, CreatePlacedResource1::<Impl, OFFSET>, CreateSamplerFeedbackUnorderedAccessView::<Impl, OFFSET>, GetCopyableFootprints1::<Impl, OFFSET>)
    }
}
pub trait ID3D12Device9Impl: Sized + ID3D12Device8Impl + ID3D12Device7Impl + ID3D12Device6Impl + ID3D12Device5Impl + ID3D12Device4Impl + ID3D12Device3Impl + ID3D12Device2Impl + ID3D12Device1Impl + ID3D12DeviceImpl + ID3D12ObjectImpl {
    fn CreateShaderCacheSession();
    fn ShaderCacheControl();
    fn CreateCommandQueue1();
}
impl ::windows::core::RuntimeName for ID3D12Device9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Device9";
}
impl ID3D12Device9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device9Impl, const OFFSET: isize>() -> ID3D12Device9Vtbl {
        unsafe extern "system" fn CreateShaderCacheSession<Impl: ID3D12Device9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, riid: *const ::windows::core::GUID, ppvsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderCacheSession(&*(&pdesc as *const <D3D12_SHADER_CACHE_SESSION_DESC as ::windows::core::Abi>::Abi as *const <D3D12_SHADER_CACHE_SESSION_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShaderCacheControl<Impl: ID3D12Device9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShaderCacheControl(kinds, control) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommandQueue1<Impl: ID3D12Device9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommandQueue1(
                &*(&pdesc as *const <D3D12_COMMAND_QUEUE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_COMMAND_QUEUE_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&creatorid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppcommandqueue),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Device9>, ::windows::core::GetTrustLevel, CreateShaderCacheSession::<Impl, OFFSET>, ShaderCacheControl::<Impl, OFFSET>, CreateCommandQueue1::<Impl, OFFSET>)
    }
}
pub trait ID3D12DeviceChildImpl: Sized + ID3D12ObjectImpl {
    fn GetDevice();
}
impl ::windows::core::RuntimeName for ID3D12DeviceChild {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DeviceChild";
}
impl ID3D12DeviceChildVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceChildImpl, const OFFSET: isize>() -> ID3D12DeviceChildVtbl {
        unsafe extern "system" fn GetDevice<Impl: ID3D12DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DeviceChild>, ::windows::core::GetTrustLevel, GetDevice::<Impl, OFFSET>)
    }
}
pub trait ID3D12DeviceRemovedExtendedDataImpl: Sized {
    fn GetAutoBreadcrumbsOutput();
    fn GetPageFaultAllocationOutput();
}
impl ::windows::core::RuntimeName for ID3D12DeviceRemovedExtendedData {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DeviceRemovedExtendedData";
}
impl ID3D12DeviceRemovedExtendedDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataImpl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedDataVtbl {
        unsafe extern "system" fn GetAutoBreadcrumbsOutput<Impl: ID3D12DeviceRemovedExtendedDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutoBreadcrumbsOutput(::core::mem::transmute_copy(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageFaultAllocationOutput<Impl: ID3D12DeviceRemovedExtendedDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageFaultAllocationOutput(::core::mem::transmute_copy(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DeviceRemovedExtendedData>, ::windows::core::GetTrustLevel, GetAutoBreadcrumbsOutput::<Impl, OFFSET>, GetPageFaultAllocationOutput::<Impl, OFFSET>)
    }
}
pub trait ID3D12DeviceRemovedExtendedData1Impl: Sized + ID3D12DeviceRemovedExtendedDataImpl {
    fn GetAutoBreadcrumbsOutput1();
    fn GetPageFaultAllocationOutput1();
}
impl ::windows::core::RuntimeName for ID3D12DeviceRemovedExtendedData1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DeviceRemovedExtendedData1";
}
impl ID3D12DeviceRemovedExtendedData1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData1Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData1Vtbl {
        unsafe extern "system" fn GetAutoBreadcrumbsOutput1<Impl: ID3D12DeviceRemovedExtendedData1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutoBreadcrumbsOutput1(::core::mem::transmute_copy(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageFaultAllocationOutput1<Impl: ID3D12DeviceRemovedExtendedData1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageFaultAllocationOutput1(::core::mem::transmute_copy(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DeviceRemovedExtendedData1>, ::windows::core::GetTrustLevel, GetAutoBreadcrumbsOutput1::<Impl, OFFSET>, GetPageFaultAllocationOutput1::<Impl, OFFSET>)
    }
}
pub trait ID3D12DeviceRemovedExtendedData2Impl: Sized + ID3D12DeviceRemovedExtendedData1Impl + ID3D12DeviceRemovedExtendedDataImpl {
    fn GetPageFaultAllocationOutput2();
    fn GetDeviceState();
}
impl ::windows::core::RuntimeName for ID3D12DeviceRemovedExtendedData2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DeviceRemovedExtendedData2";
}
impl ID3D12DeviceRemovedExtendedData2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData2Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData2Vtbl {
        unsafe extern "system" fn GetPageFaultAllocationOutput2<Impl: ID3D12DeviceRemovedExtendedData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageFaultAllocationOutput2(::core::mem::transmute_copy(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceState<Impl: ID3D12DeviceRemovedExtendedData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_DRED_DEVICE_STATE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DeviceRemovedExtendedData2>, ::windows::core::GetTrustLevel, GetPageFaultAllocationOutput2::<Impl, OFFSET>, GetDeviceState::<Impl, OFFSET>)
    }
}
pub trait ID3D12DeviceRemovedExtendedDataSettingsImpl: Sized {
    fn SetAutoBreadcrumbsEnablement();
    fn SetPageFaultEnablement();
    fn SetWatsonDumpEnablement();
}
impl ::windows::core::RuntimeName for ID3D12DeviceRemovedExtendedDataSettings {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DeviceRemovedExtendedDataSettings";
}
impl ID3D12DeviceRemovedExtendedDataSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataSettingsImpl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedDataSettingsVtbl {
        unsafe extern "system" fn SetAutoBreadcrumbsEnablement<Impl: ID3D12DeviceRemovedExtendedDataSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoBreadcrumbsEnablement(enablement).into()
        }
        unsafe extern "system" fn SetPageFaultEnablement<Impl: ID3D12DeviceRemovedExtendedDataSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageFaultEnablement(enablement).into()
        }
        unsafe extern "system" fn SetWatsonDumpEnablement<Impl: ID3D12DeviceRemovedExtendedDataSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWatsonDumpEnablement(enablement).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DeviceRemovedExtendedDataSettings>, ::windows::core::GetTrustLevel, SetAutoBreadcrumbsEnablement::<Impl, OFFSET>, SetPageFaultEnablement::<Impl, OFFSET>, SetWatsonDumpEnablement::<Impl, OFFSET>)
    }
}
pub trait ID3D12DeviceRemovedExtendedDataSettings1Impl: Sized + ID3D12DeviceRemovedExtendedDataSettingsImpl {
    fn SetBreadcrumbContextEnablement();
}
impl ::windows::core::RuntimeName for ID3D12DeviceRemovedExtendedDataSettings1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12DeviceRemovedExtendedDataSettings1";
}
impl ID3D12DeviceRemovedExtendedDataSettings1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataSettings1Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedDataSettings1Vtbl {
        unsafe extern "system" fn SetBreadcrumbContextEnablement<Impl: ID3D12DeviceRemovedExtendedDataSettings1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreadcrumbContextEnablement(enablement).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12DeviceRemovedExtendedDataSettings1>, ::windows::core::GetTrustLevel, SetBreadcrumbContextEnablement::<Impl, OFFSET>)
    }
}
pub trait ID3D12FenceImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetCompletedValue();
    fn SetEventOnCompletion();
    fn Signal();
}
impl ::windows::core::RuntimeName for ID3D12Fence {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Fence";
}
impl ID3D12FenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FenceImpl, const OFFSET: isize>() -> ID3D12FenceVtbl {
        unsafe extern "system" fn GetCompletedValue<Impl: ID3D12FenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompletedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventOnCompletion<Impl: ID3D12FenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventOnCompletion(value, &*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signal<Impl: ID3D12FenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signal(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Fence>, ::windows::core::GetTrustLevel, GetCompletedValue::<Impl, OFFSET>, SetEventOnCompletion::<Impl, OFFSET>, Signal::<Impl, OFFSET>)
    }
}
pub trait ID3D12Fence1Impl: Sized + ID3D12FenceImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetCreationFlags();
}
impl ::windows::core::RuntimeName for ID3D12Fence1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Fence1";
}
impl ID3D12Fence1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Fence1Impl, const OFFSET: isize>() -> ID3D12Fence1Vtbl {
        unsafe extern "system" fn GetCreationFlags<Impl: ID3D12Fence1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_FENCE_FLAGS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreationFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Fence1>, ::windows::core::GetTrustLevel, GetCreationFlags::<Impl, OFFSET>)
    }
}
pub trait ID3D12FunctionParameterReflectionImpl: Sized {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D12FunctionParameterReflection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12FunctionParameterReflection";
}
impl ID3D12FunctionParameterReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionParameterReflectionImpl, const OFFSET: isize>() -> ID3D12FunctionParameterReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12FunctionParameterReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12FunctionParameterReflection>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for ID3D12FunctionReflection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12FunctionReflection";
}
impl ID3D12FunctionReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionReflectionImpl, const OFFSET: isize>() -> ID3D12FunctionReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_FUNCTION_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D12FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByIndex(bufferindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D12FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D12FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(resourceindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D12FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D12FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDescByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionParameter<Impl: ID3D12FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: i32) -> ::core::option::Option<ID3D12FunctionParameterReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionParameter(parameterindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12FunctionReflection>,
            ::windows::core::GetTrustLevel,
            GetDesc::<Impl, OFFSET>,
            GetConstantBufferByIndex::<Impl, OFFSET>,
            GetConstantBufferByName::<Impl, OFFSET>,
            GetResourceBindingDesc::<Impl, OFFSET>,
            GetVariableByName::<Impl, OFFSET>,
            GetResourceBindingDescByName::<Impl, OFFSET>,
            GetFunctionParameter::<Impl, OFFSET>,
        )
    }
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
impl ::windows::core::RuntimeName for ID3D12GraphicsCommandList {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12GraphicsCommandList";
}
impl ID3D12GraphicsCommandListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>() -> ID3D12GraphicsCommandListVtbl {
        unsafe extern "system" fn Close<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocator: ::windows::core::RawPtr, pinitialstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset(&*(&pallocator as *const <ID3D12CommandAllocator as ::windows::core::Abi>::Abi as *const <ID3D12CommandAllocator as ::windows::core::DefaultType>::DefaultType), &*(&pinitialstate as *const <ID3D12PipelineState as ::windows::core::Abi>::Abi as *const <ID3D12PipelineState as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearState<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipelinestate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearState(&*(&ppipelinestate as *const <ID3D12PipelineState as ::windows::core::Abi>::Abi as *const <ID3D12PipelineState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawInstanced<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawInstanced(vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation).into()
        }
        unsafe extern "system" fn DrawIndexedInstanced<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexedInstanced(indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation).into()
        }
        unsafe extern "system" fn Dispatch<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Dispatch(threadgroupcountx, threadgroupcounty, threadgroupcountz).into()
        }
        unsafe extern "system" fn CopyBufferRegion<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstoffset: u64, psrcbuffer: ::windows::core::RawPtr, srcoffset: u64, numbytes: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyBufferRegion(&*(&pdstbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), dstoffset, &*(&psrcbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), srcoffset, numbytes).into()
        }
        unsafe extern "system" fn CopyTextureRegion<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: *const D3D12_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CopyTextureRegion(&*(&pdst as *const <D3D12_TEXTURE_COPY_LOCATION as ::windows::core::Abi>::Abi as *const <D3D12_TEXTURE_COPY_LOCATION as ::windows::core::DefaultType>::DefaultType), dstx, dsty, dstz, &*(&psrc as *const <D3D12_TEXTURE_COPY_LOCATION as ::windows::core::Abi>::Abi as *const <D3D12_TEXTURE_COPY_LOCATION as ::windows::core::DefaultType>::DefaultType), &*(&psrcbox as *const <D3D12_BOX as ::windows::core::Abi>::Abi as *const <D3D12_BOX as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn CopyResource<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyResource(&*(&pdstresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), &*(&psrcresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyTiles<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: ::windows::core::RawPtr, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CopyTiles(
                    &*(&ptiledresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&ptileregionstartcoordinate as *const <D3D12_TILED_RESOURCE_COORDINATE as ::windows::core::Abi>::Abi as *const <D3D12_TILED_RESOURCE_COORDINATE as ::windows::core::DefaultType>::DefaultType),
                    &*(&ptileregionsize as *const <D3D12_TILE_REGION_SIZE as ::windows::core::Abi>::Abi as *const <D3D12_TILE_REGION_SIZE as ::windows::core::DefaultType>::DefaultType),
                    &*(&pbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    bufferstartoffsetinbytes,
                    flags,
                )
                .into()
        }
        unsafe extern "system" fn ResolveSubresource<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveSubresource(&*(&pdstresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), dstsubresource, &*(&psrcresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), srcsubresource, format).into()
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetPrimitiveTopology(primitivetopology).into()
        }
        unsafe extern "system" fn RSSetViewports<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D12_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetViewports(numviewports, &*(&pviewports as *const <D3D12_VIEWPORT as ::windows::core::Abi>::Abi as *const <D3D12_VIEWPORT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RSSetScissorRects<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetScissorRects(numrects, &*(&prects as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OMSetBlendFactor<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blendfactor: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetBlendFactor(blendfactor).into()
        }
        unsafe extern "system" fn OMSetStencilRef<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stencilref: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetStencilRef(stencilref).into()
        }
        unsafe extern "system" fn SetPipelineState<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipelinestate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPipelineState(&*(&ppipelinestate as *const <ID3D12PipelineState as ::windows::core::Abi>::Abi as *const <ID3D12PipelineState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResourceBarrier<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbarriers: u32, pbarriers: *const D3D12_RESOURCE_BARRIER) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResourceBarrier(numbarriers, &*(&pbarriers as *const <D3D12_RESOURCE_BARRIER as ::windows::core::Abi>::Abi as *const <D3D12_RESOURCE_BARRIER as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExecuteBundle<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandlist: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteBundle(&*(&pcommandlist as *const <ID3D12GraphicsCommandList as ::windows::core::Abi>::Abi as *const <ID3D12GraphicsCommandList as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDescriptorHeaps<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numdescriptorheaps: u32, ppdescriptorheaps: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescriptorHeaps(numdescriptorheaps, &*(&ppdescriptorheaps as *const <ID3D12DescriptorHeap as ::windows::core::Abi>::Abi as *const <ID3D12DescriptorHeap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetComputeRootSignature<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootsignature: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRootSignature(&*(&prootsignature as *const <ID3D12RootSignature as ::windows::core::Abi>::Abi as *const <ID3D12RootSignature as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetGraphicsRootSignature<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootsignature: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRootSignature(&*(&prootsignature as *const <ID3D12RootSignature as ::windows::core::Abi>::Abi as *const <ID3D12RootSignature as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetComputeRootDescriptorTable<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRootDescriptorTable(rootparameterindex, &*(&basedescriptor as *const <D3D12_GPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_GPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetGraphicsRootDescriptorTable<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRootDescriptorTable(rootparameterindex, &*(&basedescriptor as *const <D3D12_GPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_GPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetComputeRoot32BitConstant<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRoot32BitConstant(rootparameterindex, srcdata, destoffsetin32bitvalues).into()
        }
        unsafe extern "system" fn SetGraphicsRoot32BitConstant<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRoot32BitConstant(rootparameterindex, srcdata, destoffsetin32bitvalues).into()
        }
        unsafe extern "system" fn SetComputeRoot32BitConstants<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRoot32BitConstants(rootparameterindex, num32bitvaluestoset, &*(&psrcdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), destoffsetin32bitvalues).into()
        }
        unsafe extern "system" fn SetGraphicsRoot32BitConstants<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRoot32BitConstants(rootparameterindex, num32bitvaluestoset, &*(&psrcdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), destoffsetin32bitvalues).into()
        }
        unsafe extern "system" fn SetComputeRootConstantBufferView<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRootConstantBufferView(rootparameterindex, bufferlocation).into()
        }
        unsafe extern "system" fn SetGraphicsRootConstantBufferView<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRootConstantBufferView(rootparameterindex, bufferlocation).into()
        }
        unsafe extern "system" fn SetComputeRootShaderResourceView<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRootShaderResourceView(rootparameterindex, bufferlocation).into()
        }
        unsafe extern "system" fn SetGraphicsRootShaderResourceView<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRootShaderResourceView(rootparameterindex, bufferlocation).into()
        }
        unsafe extern "system" fn SetComputeRootUnorderedAccessView<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRootUnorderedAccessView(rootparameterindex, bufferlocation).into()
        }
        unsafe extern "system" fn SetGraphicsRootUnorderedAccessView<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRootUnorderedAccessView(rootparameterindex, bufferlocation).into()
        }
        unsafe extern "system" fn IASetIndexBuffer<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pview: *const D3D12_INDEX_BUFFER_VIEW) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetIndexBuffer(&*(&pview as *const <D3D12_INDEX_BUFFER_VIEW as ::windows::core::Abi>::Abi as *const <D3D12_INDEX_BUFFER_VIEW as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IASetVertexBuffers<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_VERTEX_BUFFER_VIEW) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetVertexBuffers(startslot, numviews, &*(&pviews as *const <D3D12_VERTEX_BUFFER_VIEW as ::windows::core::Abi>::Abi as *const <D3D12_VERTEX_BUFFER_VIEW as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SOSetTargets<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_STREAM_OUTPUT_BUFFER_VIEW) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SOSetTargets(startslot, numviews, &*(&pviews as *const <D3D12_STREAM_OUTPUT_BUFFER_VIEW as ::windows::core::Abi>::Abi as *const <D3D12_STREAM_OUTPUT_BUFFER_VIEW as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OMSetRenderTargets<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrendertargetdescriptors: u32, prendertargetdescriptors: *const D3D12_CPU_DESCRIPTOR_HANDLE, rtssinglehandletodescriptorrange: super::super::Foundation::BOOL, pdepthstencildescriptor: *const D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .OMSetRenderTargets(
                    numrendertargetdescriptors,
                    &*(&prendertargetdescriptors as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                    &*(&rtssinglehandletodescriptorrange as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdepthstencildescriptor as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn ClearDepthStencilView<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearDepthStencilView(&*(&depthstencilview as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType), clearflags, depth, stencil, numrects, &*(&prects as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearRenderTargetView<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRenderTargetView(&*(&rendertargetview as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType), colorrgba, numrects, &*(&prects as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: ::windows::core::RawPtr, values: *const u32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ClearUnorderedAccessViewUint(
                    &*(&viewgpuhandleincurrentheap as *const <D3D12_GPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_GPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                    &*(&viewcpuhandle as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                    &*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    values,
                    numrects,
                    &*(&prects as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: ::windows::core::RawPtr, values: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ClearUnorderedAccessViewFloat(
                    &*(&viewgpuhandleincurrentheap as *const <D3D12_GPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_GPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                    &*(&viewcpuhandle as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::Abi>::Abi as *const <D3D12_CPU_DESCRIPTOR_HANDLE as ::windows::core::DefaultType>::DefaultType),
                    &*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    values,
                    numrects,
                    &*(&prects as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn DiscardResource<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pregion: *const D3D12_DISCARD_REGION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardResource(&*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), &*(&pregion as *const <D3D12_DISCARD_REGION as ::windows::core::Abi>::Abi as *const <D3D12_DISCARD_REGION as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeginQuery<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginQuery(&*(&pqueryheap as *const <ID3D12QueryHeap as ::windows::core::Abi>::Abi as *const <ID3D12QueryHeap as ::windows::core::DefaultType>::DefaultType), r#type, index).into()
        }
        unsafe extern "system" fn EndQuery<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndQuery(&*(&pqueryheap as *const <ID3D12QueryHeap as ::windows::core::Abi>::Abi as *const <ID3D12QueryHeap as ::windows::core::DefaultType>::DefaultType), r#type, index).into()
        }
        unsafe extern "system" fn ResolveQueryData<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: ::windows::core::RawPtr, aligneddestinationbufferoffset: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveQueryData(&*(&pqueryheap as *const <ID3D12QueryHeap as ::windows::core::Abi>::Abi as *const <ID3D12QueryHeap as ::windows::core::DefaultType>::DefaultType), r#type, startindex, numqueries, &*(&pdestinationbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), aligneddestinationbufferoffset).into()
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPredication(&*(&pbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), alignedbufferoffset, operation).into()
        }
        unsafe extern "system" fn SetMarker<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMarker(metadata, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), size).into()
        }
        unsafe extern "system" fn BeginEvent<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEvent(metadata, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), size).into()
        }
        unsafe extern "system" fn EndEvent<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEvent().into()
        }
        unsafe extern "system" fn ExecuteIndirect<Impl: ID3D12GraphicsCommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandsignature: ::windows::core::RawPtr, maxcommandcount: u32, pargumentbuffer: ::windows::core::RawPtr, argumentbufferoffset: u64, pcountbuffer: ::windows::core::RawPtr, countbufferoffset: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ExecuteIndirect(
                    &*(&pcommandsignature as *const <ID3D12CommandSignature as ::windows::core::Abi>::Abi as *const <ID3D12CommandSignature as ::windows::core::DefaultType>::DefaultType),
                    maxcommandcount,
                    &*(&pargumentbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    argumentbufferoffset,
                    &*(&pcountbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    countbufferoffset,
                )
                .into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12GraphicsCommandList>,
            ::windows::core::GetTrustLevel,
            Close::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            ClearState::<Impl, OFFSET>,
            DrawInstanced::<Impl, OFFSET>,
            DrawIndexedInstanced::<Impl, OFFSET>,
            Dispatch::<Impl, OFFSET>,
            CopyBufferRegion::<Impl, OFFSET>,
            CopyTextureRegion::<Impl, OFFSET>,
            CopyResource::<Impl, OFFSET>,
            CopyTiles::<Impl, OFFSET>,
            ResolveSubresource::<Impl, OFFSET>,
            IASetPrimitiveTopology::<Impl, OFFSET>,
            RSSetViewports::<Impl, OFFSET>,
            RSSetScissorRects::<Impl, OFFSET>,
            OMSetBlendFactor::<Impl, OFFSET>,
            OMSetStencilRef::<Impl, OFFSET>,
            SetPipelineState::<Impl, OFFSET>,
            ResourceBarrier::<Impl, OFFSET>,
            ExecuteBundle::<Impl, OFFSET>,
            SetDescriptorHeaps::<Impl, OFFSET>,
            SetComputeRootSignature::<Impl, OFFSET>,
            SetGraphicsRootSignature::<Impl, OFFSET>,
            SetComputeRootDescriptorTable::<Impl, OFFSET>,
            SetGraphicsRootDescriptorTable::<Impl, OFFSET>,
            SetComputeRoot32BitConstant::<Impl, OFFSET>,
            SetGraphicsRoot32BitConstant::<Impl, OFFSET>,
            SetComputeRoot32BitConstants::<Impl, OFFSET>,
            SetGraphicsRoot32BitConstants::<Impl, OFFSET>,
            SetComputeRootConstantBufferView::<Impl, OFFSET>,
            SetGraphicsRootConstantBufferView::<Impl, OFFSET>,
            SetComputeRootShaderResourceView::<Impl, OFFSET>,
            SetGraphicsRootShaderResourceView::<Impl, OFFSET>,
            SetComputeRootUnorderedAccessView::<Impl, OFFSET>,
            SetGraphicsRootUnorderedAccessView::<Impl, OFFSET>,
            IASetIndexBuffer::<Impl, OFFSET>,
            IASetVertexBuffers::<Impl, OFFSET>,
            SOSetTargets::<Impl, OFFSET>,
            OMSetRenderTargets::<Impl, OFFSET>,
            ClearDepthStencilView::<Impl, OFFSET>,
            ClearRenderTargetView::<Impl, OFFSET>,
            ClearUnorderedAccessViewUint::<Impl, OFFSET>,
            ClearUnorderedAccessViewFloat::<Impl, OFFSET>,
            DiscardResource::<Impl, OFFSET>,
            BeginQuery::<Impl, OFFSET>,
            EndQuery::<Impl, OFFSET>,
            ResolveQueryData::<Impl, OFFSET>,
            SetPredication::<Impl, OFFSET>,
            SetMarker::<Impl, OFFSET>,
            BeginEvent::<Impl, OFFSET>,
            EndEvent::<Impl, OFFSET>,
            ExecuteIndirect::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D12GraphicsCommandList1Impl: Sized + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn AtomicCopyBufferUINT();
    fn AtomicCopyBufferUINT64();
    fn OMSetDepthBounds();
    fn SetSamplePositions();
    fn ResolveSubresourceRegion();
    fn SetViewInstanceMask();
}
impl ::windows::core::RuntimeName for ID3D12GraphicsCommandList1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12GraphicsCommandList1";
}
impl ID3D12GraphicsCommandList1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList1Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList1Vtbl {
        unsafe extern "system" fn AtomicCopyBufferUINT<Impl: ID3D12GraphicsCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstoffset: u64, psrcbuffer: ::windows::core::RawPtr, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::windows::core::RawPtr, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AtomicCopyBufferUINT(
                    &*(&pdstbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    dstoffset,
                    &*(&psrcbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    srcoffset,
                    dependencies,
                    &*(&ppdependentresources as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdependentsubresourceranges as *const <D3D12_SUBRESOURCE_RANGE_UINT64 as ::windows::core::Abi>::Abi as *const <D3D12_SUBRESOURCE_RANGE_UINT64 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn AtomicCopyBufferUINT64<Impl: ID3D12GraphicsCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstoffset: u64, psrcbuffer: ::windows::core::RawPtr, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::windows::core::RawPtr, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AtomicCopyBufferUINT64(
                    &*(&pdstbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    dstoffset,
                    &*(&psrcbuffer as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    srcoffset,
                    dependencies,
                    &*(&ppdependentresources as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdependentsubresourceranges as *const <D3D12_SUBRESOURCE_RANGE_UINT64 as ::windows::core::Abi>::Abi as *const <D3D12_SUBRESOURCE_RANGE_UINT64 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn OMSetDepthBounds<Impl: ID3D12GraphicsCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, min: f32, max: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetDepthBounds(min, max).into()
        }
        unsafe extern "system" fn SetSamplePositions<Impl: ID3D12GraphicsCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSamplePositions(numsamplesperpixel, numpixels, &*(&psamplepositions as *const <D3D12_SAMPLE_POSITION as ::windows::core::Abi>::Abi as *const <D3D12_SAMPLE_POSITION as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResolveSubresourceRegion<Impl: ID3D12GraphicsCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcrect: *const super::super::Foundation::RECT, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ResolveSubresourceRegion(
                    &*(&pdstresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    dstsubresource,
                    dstx,
                    dsty,
                    &*(&psrcresource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                    srcsubresource,
                    &*(&psrcrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                    format,
                    resolvemode,
                )
                .into()
        }
        unsafe extern "system" fn SetViewInstanceMask<Impl: ID3D12GraphicsCommandList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewInstanceMask(mask).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12GraphicsCommandList1>,
            ::windows::core::GetTrustLevel,
            AtomicCopyBufferUINT::<Impl, OFFSET>,
            AtomicCopyBufferUINT64::<Impl, OFFSET>,
            OMSetDepthBounds::<Impl, OFFSET>,
            SetSamplePositions::<Impl, OFFSET>,
            ResolveSubresourceRegion::<Impl, OFFSET>,
            SetViewInstanceMask::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D12GraphicsCommandList2Impl: Sized + ID3D12GraphicsCommandList1Impl + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn WriteBufferImmediate();
}
impl ::windows::core::RuntimeName for ID3D12GraphicsCommandList2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12GraphicsCommandList2";
}
impl ID3D12GraphicsCommandList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList2Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList2Vtbl {
        unsafe extern "system" fn WriteBufferImmediate<Impl: ID3D12GraphicsCommandList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteBufferImmediate(count, &*(&pparams as *const <D3D12_WRITEBUFFERIMMEDIATE_PARAMETER as ::windows::core::Abi>::Abi as *const <D3D12_WRITEBUFFERIMMEDIATE_PARAMETER as ::windows::core::DefaultType>::DefaultType), pmodes).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12GraphicsCommandList2>, ::windows::core::GetTrustLevel, WriteBufferImmediate::<Impl, OFFSET>)
    }
}
pub trait ID3D12GraphicsCommandList3Impl: Sized + ID3D12GraphicsCommandList2Impl + ID3D12GraphicsCommandList1Impl + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn SetProtectedResourceSession();
}
impl ::windows::core::RuntimeName for ID3D12GraphicsCommandList3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12GraphicsCommandList3";
}
impl ID3D12GraphicsCommandList3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList3Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList3Vtbl {
        unsafe extern "system" fn SetProtectedResourceSession<Impl: ID3D12GraphicsCommandList3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotectedresourcesession: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectedResourceSession(&*(&pprotectedresourcesession as *const <ID3D12ProtectedResourceSession as ::windows::core::Abi>::Abi as *const <ID3D12ProtectedResourceSession as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12GraphicsCommandList3>, ::windows::core::GetTrustLevel, SetProtectedResourceSession::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for ID3D12GraphicsCommandList4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12GraphicsCommandList4";
}
impl ID3D12GraphicsCommandList4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList4Vtbl {
        unsafe extern "system" fn BeginRenderPass<Impl: ID3D12GraphicsCommandList4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrendertargets: u32, prendertargets: *const D3D12_RENDER_PASS_RENDER_TARGET_DESC, pdepthstencil: *const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, flags: D3D12_RENDER_PASS_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginRenderPass(numrendertargets, &*(&prendertargets as *const <D3D12_RENDER_PASS_RENDER_TARGET_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RENDER_PASS_RENDER_TARGET_DESC as ::windows::core::DefaultType>::DefaultType), &*(&pdepthstencil as *const <D3D12_RENDER_PASS_DEPTH_STENCIL_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RENDER_PASS_DEPTH_STENCIL_DESC as ::windows::core::DefaultType>::DefaultType), flags).into()
        }
        unsafe extern "system" fn EndRenderPass<Impl: ID3D12GraphicsCommandList4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndRenderPass().into()
        }
        unsafe extern "system" fn InitializeMetaCommand<Impl: ID3D12GraphicsCommandList4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetacommand: ::windows::core::RawPtr, pinitializationparametersdata: *const ::core::ffi::c_void, initializationparametersdatasizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeMetaCommand(&*(&pmetacommand as *const <ID3D12MetaCommand as ::windows::core::Abi>::Abi as *const <ID3D12MetaCommand as ::windows::core::DefaultType>::DefaultType), &*(&pinitializationparametersdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), initializationparametersdatasizeinbytes).into()
        }
        unsafe extern "system" fn ExecuteMetaCommand<Impl: ID3D12GraphicsCommandList4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetacommand: ::windows::core::RawPtr, pexecutionparametersdata: *const ::core::ffi::c_void, executionparametersdatasizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteMetaCommand(&*(&pmetacommand as *const <ID3D12MetaCommand as ::windows::core::Abi>::Abi as *const <ID3D12MetaCommand as ::windows::core::DefaultType>::DefaultType), &*(&pexecutionparametersdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), executionparametersdatasizeinbytes).into()
        }
        unsafe extern "system" fn BuildRaytracingAccelerationStructure<Impl: ID3D12GraphicsCommandList4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, numpostbuildinfodescs: u32, ppostbuildinfodescs: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .BuildRaytracingAccelerationStructure(&*(&pdesc as *const <D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC as ::windows::core::DefaultType>::DefaultType), numpostbuildinfodescs, &*(&ppostbuildinfodescs as *const <D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn EmitRaytracingAccelerationStructurePostbuildInfo<Impl: ID3D12GraphicsCommandList4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, numsourceaccelerationstructures: u32, psourceaccelerationstructuredata: *const u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EmitRaytracingAccelerationStructurePostbuildInfo(&*(&pdesc as *const <D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC as ::windows::core::Abi>::Abi as *const <D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC as ::windows::core::DefaultType>::DefaultType), numsourceaccelerationstructures, psourceaccelerationstructuredata).into()
        }
        unsafe extern "system" fn CopyRaytracingAccelerationStructure<Impl: ID3D12GraphicsCommandList4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyRaytracingAccelerationStructure(destaccelerationstructuredata, sourceaccelerationstructuredata, mode).into()
        }
        unsafe extern "system" fn SetPipelineState1<Impl: ID3D12GraphicsCommandList4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateobject: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPipelineState1(&*(&pstateobject as *const <ID3D12StateObject as ::windows::core::Abi>::Abi as *const <ID3D12StateObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DispatchRays<Impl: ID3D12GraphicsCommandList4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_DISPATCH_RAYS_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DispatchRays(&*(&pdesc as *const <D3D12_DISPATCH_RAYS_DESC as ::windows::core::Abi>::Abi as *const <D3D12_DISPATCH_RAYS_DESC as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12GraphicsCommandList4>,
            ::windows::core::GetTrustLevel,
            BeginRenderPass::<Impl, OFFSET>,
            EndRenderPass::<Impl, OFFSET>,
            InitializeMetaCommand::<Impl, OFFSET>,
            ExecuteMetaCommand::<Impl, OFFSET>,
            BuildRaytracingAccelerationStructure::<Impl, OFFSET>,
            EmitRaytracingAccelerationStructurePostbuildInfo::<Impl, OFFSET>,
            CopyRaytracingAccelerationStructure::<Impl, OFFSET>,
            SetPipelineState1::<Impl, OFFSET>,
            DispatchRays::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D12GraphicsCommandList5Impl: Sized + ID3D12GraphicsCommandList4Impl + ID3D12GraphicsCommandList3Impl + ID3D12GraphicsCommandList2Impl + ID3D12GraphicsCommandList1Impl + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn RSSetShadingRate();
    fn RSSetShadingRateImage();
}
impl ::windows::core::RuntimeName for ID3D12GraphicsCommandList5 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12GraphicsCommandList5";
}
impl ID3D12GraphicsCommandList5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList5Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList5Vtbl {
        unsafe extern "system" fn RSSetShadingRate<Impl: ID3D12GraphicsCommandList5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseshadingrate: D3D12_SHADING_RATE, combiners: *const D3D12_SHADING_RATE_COMBINER) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetShadingRate(baseshadingrate, combiners).into()
        }
        unsafe extern "system" fn RSSetShadingRateImage<Impl: ID3D12GraphicsCommandList5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shadingrateimage: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetShadingRateImage(&*(&shadingrateimage as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12GraphicsCommandList5>, ::windows::core::GetTrustLevel, RSSetShadingRate::<Impl, OFFSET>, RSSetShadingRateImage::<Impl, OFFSET>)
    }
}
pub trait ID3D12GraphicsCommandList6Impl: Sized + ID3D12GraphicsCommandList5Impl + ID3D12GraphicsCommandList4Impl + ID3D12GraphicsCommandList3Impl + ID3D12GraphicsCommandList2Impl + ID3D12GraphicsCommandList1Impl + ID3D12GraphicsCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn DispatchMesh();
}
impl ::windows::core::RuntimeName for ID3D12GraphicsCommandList6 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12GraphicsCommandList6";
}
impl ID3D12GraphicsCommandList6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList6Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList6Vtbl {
        unsafe extern "system" fn DispatchMesh<Impl: ID3D12GraphicsCommandList6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DispatchMesh(threadgroupcountx, threadgroupcounty, threadgroupcountz).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12GraphicsCommandList6>, ::windows::core::GetTrustLevel, DispatchMesh::<Impl, OFFSET>)
    }
}
pub trait ID3D12HeapImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D12Heap {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Heap";
}
impl ID3D12HeapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12HeapImpl, const OFFSET: isize>() -> ID3D12HeapVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12HeapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_HEAP_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Heap>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D12Heap1Impl: Sized + ID3D12HeapImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetProtectedResourceSession();
}
impl ::windows::core::RuntimeName for ID3D12Heap1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Heap1";
}
impl ID3D12Heap1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Heap1Impl, const OFFSET: isize>() -> ID3D12Heap1Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Impl: ID3D12Heap1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtectedResourceSession(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppprotectedsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Heap1>, ::windows::core::GetTrustLevel, GetProtectedResourceSession::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for ID3D12InfoQueue {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12InfoQueue";
}
impl ID3D12InfoQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueueImpl, const OFFSET: isize>() -> ID3D12InfoQueueVtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMessageCountLimit(messagecountlimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStoredMessages().into()
        }
        unsafe extern "system" fn GetMessage<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D12_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(messageindex, ::core::mem::transmute_copy(&pmessage), pmessagebytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesAllowedByStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesDeniedByStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumStoredMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumStoredMessagesAllowedByRetrievalFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesDiscardedByMessageCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStorageFilterEntries(&*(&pfilter as *const <D3D12_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D12_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFilter(::core::mem::transmute_copy(&pfilter), pfilterbytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStorageFilter().into()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushEmptyStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushCopyOfStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushStorageFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushStorageFilter(&*(&pfilter as *const <D3D12_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D12_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopStorageFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopStorageFilter().into()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFilterStackSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRetrievalFilterEntries(&*(&pfilter as *const <D3D12_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D12_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRetrievalFilter(::core::mem::transmute_copy(&pfilter), pfilterbytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRetrievalFilter().into()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushEmptyRetrievalFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushCopyOfRetrievalFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushRetrievalFilter(&*(&pfilter as *const <D3D12_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D12_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopRetrievalFilter().into()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRetrievalFilterStackSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMessage<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMessage(category, severity, id, &*(&pdescription as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddApplicationMessage(severity, &*(&pdescription as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnCategory(category, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnSeverity(severity, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnID<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D12_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnID(id, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnCategory(category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnSeverity(severity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnID<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnID(id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuteDebugOutput(&*(&bmute as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: ID3D12InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMuteDebugOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12InfoQueue>,
            ::windows::core::GetTrustLevel,
            SetMessageCountLimit::<Impl, OFFSET>,
            ClearStoredMessages::<Impl, OFFSET>,
            GetMessage::<Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter::<Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter::<Impl, OFFSET>,
            GetNumStoredMessages::<Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter::<Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit::<Impl, OFFSET>,
            GetMessageCountLimit::<Impl, OFFSET>,
            AddStorageFilterEntries::<Impl, OFFSET>,
            GetStorageFilter::<Impl, OFFSET>,
            ClearStorageFilter::<Impl, OFFSET>,
            PushEmptyStorageFilter::<Impl, OFFSET>,
            PushCopyOfStorageFilter::<Impl, OFFSET>,
            PushStorageFilter::<Impl, OFFSET>,
            PopStorageFilter::<Impl, OFFSET>,
            GetStorageFilterStackSize::<Impl, OFFSET>,
            AddRetrievalFilterEntries::<Impl, OFFSET>,
            GetRetrievalFilter::<Impl, OFFSET>,
            ClearRetrievalFilter::<Impl, OFFSET>,
            PushEmptyRetrievalFilter::<Impl, OFFSET>,
            PushCopyOfRetrievalFilter::<Impl, OFFSET>,
            PushRetrievalFilter::<Impl, OFFSET>,
            PopRetrievalFilter::<Impl, OFFSET>,
            GetRetrievalFilterStackSize::<Impl, OFFSET>,
            AddMessage::<Impl, OFFSET>,
            AddApplicationMessage::<Impl, OFFSET>,
            SetBreakOnCategory::<Impl, OFFSET>,
            SetBreakOnSeverity::<Impl, OFFSET>,
            SetBreakOnID::<Impl, OFFSET>,
            GetBreakOnCategory::<Impl, OFFSET>,
            GetBreakOnSeverity::<Impl, OFFSET>,
            GetBreakOnID::<Impl, OFFSET>,
            SetMuteDebugOutput::<Impl, OFFSET>,
            GetMuteDebugOutput::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D12InfoQueue1Impl: Sized + ID3D12InfoQueueImpl {
    fn RegisterMessageCallback();
    fn UnregisterMessageCallback();
}
impl ::windows::core::RuntimeName for ID3D12InfoQueue1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12InfoQueue1";
}
impl ID3D12InfoQueue1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue1Impl, const OFFSET: isize>() -> ID3D12InfoQueue1Vtbl {
        unsafe extern "system" fn RegisterMessageCallback<Impl: ID3D12InfoQueue1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackfunc: ::windows::core::RawPtr, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *const ::core::ffi::c_void, pcallbackcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterMessageCallback(&*(&callbackfunc as *const <D3D12MessageFunc as ::windows::core::Abi>::Abi as *const <D3D12MessageFunc as ::windows::core::DefaultType>::DefaultType), callbackfilterflags, &*(&pcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), pcallbackcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterMessageCallback<Impl: ID3D12InfoQueue1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterMessageCallback(callbackcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12InfoQueue1>, ::windows::core::GetTrustLevel, RegisterMessageCallback::<Impl, OFFSET>, UnregisterMessageCallback::<Impl, OFFSET>)
    }
}
pub trait ID3D12LibraryReflectionImpl: Sized {
    fn GetDesc();
    fn GetFunctionByIndex();
}
impl ::windows::core::RuntimeName for ID3D12LibraryReflection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12LibraryReflection";
}
impl ID3D12LibraryReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LibraryReflectionImpl, const OFFSET: isize>() -> ID3D12LibraryReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12LibraryReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_LIBRARY_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionByIndex<Impl: ID3D12LibraryReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionindex: i32) -> ::core::option::Option<ID3D12FunctionReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionByIndex(functionindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12LibraryReflection>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, GetFunctionByIndex::<Impl, OFFSET>)
    }
}
pub trait ID3D12LifetimeOwnerImpl: Sized {
    fn LifetimeStateUpdated();
}
impl ::windows::core::RuntimeName for ID3D12LifetimeOwner {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12LifetimeOwner";
}
impl ID3D12LifetimeOwnerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LifetimeOwnerImpl, const OFFSET: isize>() -> ID3D12LifetimeOwnerVtbl {
        unsafe extern "system" fn LifetimeStateUpdated<Impl: ID3D12LifetimeOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: D3D12_LIFETIME_STATE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LifetimeStateUpdated(newstate).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12LifetimeOwner>, ::windows::core::GetTrustLevel, LifetimeStateUpdated::<Impl, OFFSET>)
    }
}
pub trait ID3D12LifetimeTrackerImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn DestroyOwnedObject();
}
impl ::windows::core::RuntimeName for ID3D12LifetimeTracker {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12LifetimeTracker";
}
impl ID3D12LifetimeTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LifetimeTrackerImpl, const OFFSET: isize>() -> ID3D12LifetimeTrackerVtbl {
        unsafe extern "system" fn DestroyOwnedObject<Impl: ID3D12LifetimeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroyOwnedObject(&*(&pobject as *const <ID3D12DeviceChild as ::windows::core::Abi>::Abi as *const <ID3D12DeviceChild as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12LifetimeTracker>, ::windows::core::GetTrustLevel, DestroyOwnedObject::<Impl, OFFSET>)
    }
}
pub trait ID3D12MetaCommandImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetRequiredParameterResourceSize();
}
impl ::windows::core::RuntimeName for ID3D12MetaCommand {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12MetaCommand";
}
impl ID3D12MetaCommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12MetaCommandImpl, const OFFSET: isize>() -> ID3D12MetaCommandVtbl {
        unsafe extern "system" fn GetRequiredParameterResourceSize<Impl: ID3D12MetaCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequiredParameterResourceSize(stage, parameterindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12MetaCommand>, ::windows::core::GetTrustLevel, GetRequiredParameterResourceSize::<Impl, OFFSET>)
    }
}
pub trait ID3D12ObjectImpl: Sized {
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn SetName();
}
impl ::windows::core::RuntimeName for ID3D12Object {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Object";
}
impl ID3D12ObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ObjectImpl, const OFFSET: isize>() -> ID3D12ObjectVtbl {
        unsafe extern "system" fn GetPrivateData<Impl: ID3D12ObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pdatasize, ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D12ObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D12ObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateDataInterface(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ID3D12ObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Object>, ::windows::core::GetTrustLevel, GetPrivateData::<Impl, OFFSET>, SetPrivateData::<Impl, OFFSET>, SetPrivateDataInterface::<Impl, OFFSET>, SetName::<Impl, OFFSET>)
    }
}
pub trait ID3D12PageableImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {}
impl ::windows::core::RuntimeName for ID3D12Pageable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Pageable";
}
impl ID3D12PageableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PageableImpl, const OFFSET: isize>() -> ID3D12PageableVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Pageable>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D12PipelineLibraryImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn StorePipeline();
    fn LoadGraphicsPipeline();
    fn LoadComputePipeline();
    fn GetSerializedSize();
    fn Serialize();
}
impl ::windows::core::RuntimeName for ID3D12PipelineLibrary {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12PipelineLibrary";
}
impl ID3D12PipelineLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibraryImpl, const OFFSET: isize>() -> ID3D12PipelineLibraryVtbl {
        unsafe extern "system" fn StorePipeline<Impl: ID3D12PipelineLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, ppipeline: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorePipeline(&*(&pname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&ppipeline as *const <ID3D12PipelineState as ::windows::core::Abi>::Abi as *const <ID3D12PipelineState as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadGraphicsPipeline<Impl: ID3D12PipelineLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadGraphicsPipeline(
                &*(&pname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdesc as *const <D3D12_GRAPHICS_PIPELINE_STATE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_GRAPHICS_PIPELINE_STATE_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pppipelinestate),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadComputePipeline<Impl: ID3D12PipelineLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadComputePipeline(
                &*(&pname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdesc as *const <D3D12_COMPUTE_PIPELINE_STATE_DESC as ::windows::core::Abi>::Abi as *const <D3D12_COMPUTE_PIPELINE_STATE_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pppipelinestate),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerializedSize<Impl: ID3D12PipelineLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerializedSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: ID3D12PipelineLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasizeinbytes: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Serialize(::core::mem::transmute_copy(&pdata), datasizeinbytes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12PipelineLibrary>, ::windows::core::GetTrustLevel, StorePipeline::<Impl, OFFSET>, LoadGraphicsPipeline::<Impl, OFFSET>, LoadComputePipeline::<Impl, OFFSET>, GetSerializedSize::<Impl, OFFSET>, Serialize::<Impl, OFFSET>)
    }
}
pub trait ID3D12PipelineLibrary1Impl: Sized + ID3D12PipelineLibraryImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn LoadPipeline();
}
impl ::windows::core::RuntimeName for ID3D12PipelineLibrary1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12PipelineLibrary1";
}
impl ID3D12PipelineLibrary1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary1Impl, const OFFSET: isize>() -> ID3D12PipelineLibrary1Vtbl {
        unsafe extern "system" fn LoadPipeline<Impl: ID3D12PipelineLibrary1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadPipeline(
                &*(&pname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdesc as *const <D3D12_PIPELINE_STATE_STREAM_DESC as ::windows::core::Abi>::Abi as *const <D3D12_PIPELINE_STATE_STREAM_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pppipelinestate),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12PipelineLibrary1>, ::windows::core::GetTrustLevel, LoadPipeline::<Impl, OFFSET>)
    }
}
pub trait ID3D12PipelineStateImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetCachedBlob();
}
impl ::windows::core::RuntimeName for ID3D12PipelineState {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12PipelineState";
}
impl ID3D12PipelineStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineStateImpl, const OFFSET: isize>() -> ID3D12PipelineStateVtbl {
        unsafe extern "system" fn GetCachedBlob<Impl: ID3D12PipelineStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedBlob(::core::mem::transmute_copy(&ppblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12PipelineState>, ::windows::core::GetTrustLevel, GetCachedBlob::<Impl, OFFSET>)
    }
}
pub trait ID3D12ProtectedResourceSessionImpl: Sized + ID3D12ProtectedSessionImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D12ProtectedResourceSession {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12ProtectedResourceSession";
}
impl ID3D12ProtectedResourceSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedResourceSessionImpl, const OFFSET: isize>() -> ID3D12ProtectedResourceSessionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ProtectedResourceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12ProtectedResourceSession>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D12ProtectedResourceSession1Impl: Sized + ID3D12ProtectedResourceSessionImpl + ID3D12ProtectedSessionImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D12ProtectedResourceSession1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12ProtectedResourceSession1";
}
impl ID3D12ProtectedResourceSession1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedResourceSession1Impl, const OFFSET: isize>() -> ID3D12ProtectedResourceSession1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D12ProtectedResourceSession1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12ProtectedResourceSession1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D12ProtectedSessionImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetStatusFence();
    fn GetSessionStatus();
}
impl ::windows::core::RuntimeName for ID3D12ProtectedSession {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12ProtectedSession";
}
impl ID3D12ProtectedSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedSessionImpl, const OFFSET: isize>() -> ID3D12ProtectedSessionVtbl {
        unsafe extern "system" fn GetStatusFence<Impl: ID3D12ProtectedSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatusFence(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionStatus<Impl: ID3D12ProtectedSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_PROTECTED_SESSION_STATUS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12ProtectedSession>, ::windows::core::GetTrustLevel, GetStatusFence::<Impl, OFFSET>, GetSessionStatus::<Impl, OFFSET>)
    }
}
pub trait ID3D12QueryHeapImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {}
impl ::windows::core::RuntimeName for ID3D12QueryHeap {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12QueryHeap";
}
impl ID3D12QueryHeapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12QueryHeapImpl, const OFFSET: isize>() -> ID3D12QueryHeapVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12QueryHeap>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D12ResourceImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn Map();
    fn Unmap();
    fn GetDesc();
    fn GetGPUVirtualAddress();
    fn WriteToSubresource();
    fn ReadFromSubresource();
    fn GetHeapProperties();
}
impl ::windows::core::RuntimeName for ID3D12Resource {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Resource";
}
impl ID3D12ResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ResourceImpl, const OFFSET: isize>() -> ID3D12ResourceVtbl {
        unsafe extern "system" fn Map<Impl: ID3D12ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, preadrange: *const D3D12_RANGE, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map(subresource, &*(&preadrange as *const <D3D12_RANGE as ::windows::core::Abi>::Abi as *const <D3D12_RANGE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID3D12ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, pwrittenrange: *const D3D12_RANGE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap(subresource, &*(&pwrittenrange as *const <D3D12_RANGE as ::windows::core::Abi>::Abi as *const <D3D12_RANGE as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D12ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPUVirtualAddress<Impl: ID3D12ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGPUVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToSubresource<Impl: ID3D12ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D12_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteToSubresource(dstsubresource, &*(&pdstbox as *const <D3D12_BOX as ::windows::core::Abi>::Abi as *const <D3D12_BOX as ::windows::core::DefaultType>::DefaultType), &*(&psrcdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), srcrowpitch, srcdepthpitch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadFromSubresource<Impl: ID3D12ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: *const D3D12_BOX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadFromSubresource(::core::mem::transmute_copy(&pdstdata), dstrowpitch, dstdepthpitch, srcsubresource, &*(&psrcbox as *const <D3D12_BOX as ::windows::core::Abi>::Abi as *const <D3D12_BOX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHeapProperties<Impl: ID3D12ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *mut D3D12_HEAP_PROPERTIES, pheapflags: *mut D3D12_HEAP_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeapProperties(::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&pheapflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Resource>, ::windows::core::GetTrustLevel, Map::<Impl, OFFSET>, Unmap::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>, GetGPUVirtualAddress::<Impl, OFFSET>, WriteToSubresource::<Impl, OFFSET>, ReadFromSubresource::<Impl, OFFSET>, GetHeapProperties::<Impl, OFFSET>)
    }
}
pub trait ID3D12Resource1Impl: Sized + ID3D12ResourceImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetProtectedResourceSession();
}
impl ::windows::core::RuntimeName for ID3D12Resource1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Resource1";
}
impl ID3D12Resource1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource1Impl, const OFFSET: isize>() -> ID3D12Resource1Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Impl: ID3D12Resource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtectedResourceSession(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppprotectedsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Resource1>, ::windows::core::GetTrustLevel, GetProtectedResourceSession::<Impl, OFFSET>)
    }
}
pub trait ID3D12Resource2Impl: Sized + ID3D12Resource1Impl + ID3D12ResourceImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D12Resource2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Resource2";
}
impl ID3D12Resource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource2Impl, const OFFSET: isize>() -> ID3D12Resource2Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D12Resource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Resource2>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D12RootSignatureImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {}
impl ::windows::core::RuntimeName for ID3D12RootSignature {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12RootSignature";
}
impl ID3D12RootSignatureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12RootSignatureImpl, const OFFSET: isize>() -> ID3D12RootSignatureVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12RootSignature>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D12RootSignatureDeserializerImpl: Sized {
    fn GetRootSignatureDesc();
}
impl ::windows::core::RuntimeName for ID3D12RootSignatureDeserializer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12RootSignatureDeserializer";
}
impl ID3D12RootSignatureDeserializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12RootSignatureDeserializerImpl, const OFFSET: isize>() -> ID3D12RootSignatureDeserializerVtbl {
        unsafe extern "system" fn GetRootSignatureDesc<Impl: ID3D12RootSignatureDeserializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut D3D12_ROOT_SIGNATURE_DESC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRootSignatureDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12RootSignatureDeserializer>, ::windows::core::GetTrustLevel, GetRootSignatureDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D12SDKConfigurationImpl: Sized {
    fn SetSDKVersion();
}
impl ::windows::core::RuntimeName for ID3D12SDKConfiguration {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12SDKConfiguration";
}
impl ID3D12SDKConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SDKConfigurationImpl, const OFFSET: isize>() -> ID3D12SDKConfigurationVtbl {
        unsafe extern "system" fn SetSDKVersion<Impl: ID3D12SDKConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sdkversion: u32, sdkpath: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSDKVersion(sdkversion, &*(&sdkpath as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12SDKConfiguration>, ::windows::core::GetTrustLevel, SetSDKVersion::<Impl, OFFSET>)
    }
}
pub trait ID3D12ShaderCacheSessionImpl: Sized + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn FindValue();
    fn StoreValue();
    fn SetDeleteOnDestroy();
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D12ShaderCacheSession {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12ShaderCacheSession";
}
impl ID3D12ShaderCacheSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderCacheSessionImpl, const OFFSET: isize>() -> ID3D12ShaderCacheSessionVtbl {
        unsafe extern "system" fn FindValue<Impl: ID3D12ShaderCacheSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *mut ::core::ffi::c_void, pvaluesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindValue(&*(&pkey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), keysize, ::core::mem::transmute_copy(&pvalue), pvaluesize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreValue<Impl: ID3D12ShaderCacheSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreValue(&*(&pkey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), keysize, &*(&pvalue as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), valuesize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeleteOnDestroy<Impl: ID3D12ShaderCacheSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeleteOnDestroy().into()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderCacheSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_SHADER_CACHE_SESSION_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12ShaderCacheSession>, ::windows::core::GetTrustLevel, FindValue::<Impl, OFFSET>, StoreValue::<Impl, OFFSET>, SetDeleteOnDestroy::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for ID3D12ShaderReflection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12ShaderReflection";
}
impl ID3D12ShaderReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>() -> ID3D12ShaderReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(resourceindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputParameterDesc(parameterindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputParameterDesc(parameterindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatchConstantParameterDesc(parameterindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDescByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMovInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovcInstructionCount<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMovcInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionInstructionCount<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitwiseInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGSInputPrimitive<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGSInputPrimitive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSampleFrequencyShader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumInterfaceSlots() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinFeatureLevel<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinFeatureLevel(::core::mem::transmute_copy(&plevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadGroupSize<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThreadGroupSize(::core::mem::transmute_copy(&psizex), ::core::mem::transmute_copy(&psizey), ::core::mem::transmute_copy(&psizez)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequiresFlags<Impl: ID3D12ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequiresFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12ShaderReflection>,
            ::windows::core::GetTrustLevel,
            GetDesc::<Impl, OFFSET>,
            GetConstantBufferByIndex::<Impl, OFFSET>,
            GetConstantBufferByName::<Impl, OFFSET>,
            GetResourceBindingDesc::<Impl, OFFSET>,
            GetInputParameterDesc::<Impl, OFFSET>,
            GetOutputParameterDesc::<Impl, OFFSET>,
            GetPatchConstantParameterDesc::<Impl, OFFSET>,
            GetVariableByName::<Impl, OFFSET>,
            GetResourceBindingDescByName::<Impl, OFFSET>,
            GetMovInstructionCount::<Impl, OFFSET>,
            GetMovcInstructionCount::<Impl, OFFSET>,
            GetConversionInstructionCount::<Impl, OFFSET>,
            GetBitwiseInstructionCount::<Impl, OFFSET>,
            GetGSInputPrimitive::<Impl, OFFSET>,
            IsSampleFrequencyShader::<Impl, OFFSET>,
            GetNumInterfaceSlots::<Impl, OFFSET>,
            GetMinFeatureLevel::<Impl, OFFSET>,
            GetThreadGroupSize::<Impl, OFFSET>,
            GetRequiresFlags::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D12ShaderReflectionConstantBufferImpl: Sized {
    fn GetDesc();
    fn GetVariableByIndex();
    fn GetVariableByName();
}
impl ::windows::core::RuntimeName for ID3D12ShaderReflectionConstantBuffer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12ShaderReflectionConstantBuffer";
}
impl ID3D12ShaderReflectionConstantBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionConstantBufferImpl, const OFFSET: isize>() -> ID3D12ShaderReflectionConstantBufferVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(&*(&pdesc as *const <D3D12_SHADER_BUFFER_DESC as ::windows::core::Abi>::Abi as *const <D3D12_SHADER_BUFFER_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D12ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D12ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12ShaderReflectionConstantBuffer>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, GetVariableByIndex::<Impl, OFFSET>, GetVariableByName::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for ID3D12ShaderReflectionType {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12ShaderReflectionType";
}
impl ID3D12ShaderReflectionTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>() -> ID3D12ShaderReflectionTypeVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeName(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&ptype as *const <ID3D12ShaderReflectionType as ::windows::core::Abi>::Abi as *const <ID3D12ShaderReflectionType as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubType<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBaseClass<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBaseClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumInterfaces<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceByIndex<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterfaceByIndex(uindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOfType<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOfType(&*(&ptype as *const <ID3D12ShaderReflectionType as ::windows::core::Abi>::Abi as *const <ID3D12ShaderReflectionType as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImplementsInterface<Impl: ID3D12ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbase: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImplementsInterface(&*(&pbase as *const <ID3D12ShaderReflectionType as ::windows::core::Abi>::Abi as *const <ID3D12ShaderReflectionType as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D12ShaderReflectionType>,
            ::windows::core::GetTrustLevel,
            GetDesc::<Impl, OFFSET>,
            GetMemberTypeByIndex::<Impl, OFFSET>,
            GetMemberTypeByName::<Impl, OFFSET>,
            GetMemberTypeName::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            GetSubType::<Impl, OFFSET>,
            GetBaseClass::<Impl, OFFSET>,
            GetNumInterfaces::<Impl, OFFSET>,
            GetInterfaceByIndex::<Impl, OFFSET>,
            IsOfType::<Impl, OFFSET>,
            ImplementsInterface::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D12ShaderReflectionVariableImpl: Sized {
    fn GetDesc();
    fn GetType();
    fn GetBuffer();
    fn GetInterfaceSlot();
}
impl ::windows::core::RuntimeName for ID3D12ShaderReflectionVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12ShaderReflectionVariable";
}
impl ID3D12ShaderReflectionVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionVariableImpl, const OFFSET: isize>() -> ID3D12ShaderReflectionVariableVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ID3D12ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuffer<Impl: ID3D12ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceSlot<Impl: ID3D12ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterfaceSlot(uarrayindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12ShaderReflectionVariable>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, GetType::<Impl, OFFSET>, GetBuffer::<Impl, OFFSET>, GetInterfaceSlot::<Impl, OFFSET>)
    }
}
pub trait ID3D12SharingContractImpl: Sized {
    fn Present();
    fn SharedFenceSignal();
    fn BeginCapturableWork();
    fn EndCapturableWork();
}
impl ::windows::core::RuntimeName for ID3D12SharingContract {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12SharingContract";
}
impl ID3D12SharingContractVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SharingContractImpl, const OFFSET: isize>() -> ID3D12SharingContractVtbl {
        unsafe extern "system" fn Present<Impl: ID3D12SharingContractImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, window: super::super::Foundation::HWND) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Present(&*(&presource as *const <ID3D12Resource as ::windows::core::Abi>::Abi as *const <ID3D12Resource as ::windows::core::DefaultType>::DefaultType), subresource, &*(&window as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SharedFenceSignal<Impl: ID3D12SharingContractImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, fencevalue: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SharedFenceSignal(&*(&pfence as *const <ID3D12Fence as ::windows::core::Abi>::Abi as *const <ID3D12Fence as ::windows::core::DefaultType>::DefaultType), fencevalue).into()
        }
        unsafe extern "system" fn BeginCapturableWork<Impl: ID3D12SharingContractImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginCapturableWork(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndCapturableWork<Impl: ID3D12SharingContractImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndCapturableWork(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12SharingContract>, ::windows::core::GetTrustLevel, Present::<Impl, OFFSET>, SharedFenceSignal::<Impl, OFFSET>, BeginCapturableWork::<Impl, OFFSET>, EndCapturableWork::<Impl, OFFSET>)
    }
}
pub trait ID3D12StateObjectImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {}
impl ::windows::core::RuntimeName for ID3D12StateObject {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12StateObject";
}
impl ID3D12StateObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12StateObjectImpl, const OFFSET: isize>() -> ID3D12StateObjectVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12StateObject>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D12StateObjectPropertiesImpl: Sized {
    fn GetShaderIdentifier();
    fn GetShaderStackSize();
    fn GetPipelineStackSize();
    fn SetPipelineStackSize();
}
impl ::windows::core::RuntimeName for ID3D12StateObjectProperties {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12StateObjectProperties";
}
impl ID3D12StateObjectPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12StateObjectPropertiesImpl, const OFFSET: isize>() -> ID3D12StateObjectPropertiesVtbl {
        unsafe extern "system" fn GetShaderIdentifier<Impl: ID3D12StateObjectPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexportname: super::super::Foundation::PWSTR) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShaderIdentifier(&*(&pexportname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShaderStackSize<Impl: ID3D12StateObjectPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexportname: super::super::Foundation::PWSTR) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShaderStackSize(&*(&pexportname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPipelineStackSize<Impl: ID3D12StateObjectPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPipelineStackSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPipelineStackSize<Impl: ID3D12StateObjectPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipelinestacksizeinbytes: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPipelineStackSize(pipelinestacksizeinbytes).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12StateObjectProperties>, ::windows::core::GetTrustLevel, GetShaderIdentifier::<Impl, OFFSET>, GetShaderStackSize::<Impl, OFFSET>, GetPipelineStackSize::<Impl, OFFSET>, SetPipelineStackSize::<Impl, OFFSET>)
    }
}
pub trait ID3D12SwapChainAssistantImpl: Sized {
    fn GetLUID();
    fn GetSwapChainObject();
    fn GetCurrentResourceAndCommandQueue();
    fn InsertImplicitSync();
}
impl ::windows::core::RuntimeName for ID3D12SwapChainAssistant {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12SwapChainAssistant";
}
impl ID3D12SwapChainAssistantVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SwapChainAssistantImpl, const OFFSET: isize>() -> ID3D12SwapChainAssistantVtbl {
        unsafe extern "system" fn GetLUID<Impl: ID3D12SwapChainAssistantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLUID() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSwapChainObject<Impl: ID3D12SwapChainAssistantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSwapChainObject(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentResourceAndCommandQueue<Impl: ID3D12SwapChainAssistantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void, riidqueue: *const ::windows::core::GUID, ppvqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentResourceAndCommandQueue(&*(&riidresource as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvresource), &*(&riidqueue as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertImplicitSync<Impl: ID3D12SwapChainAssistantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertImplicitSync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12SwapChainAssistant>, ::windows::core::GetTrustLevel, GetLUID::<Impl, OFFSET>, GetSwapChainObject::<Impl, OFFSET>, GetCurrentResourceAndCommandQueue::<Impl, OFFSET>, InsertImplicitSync::<Impl, OFFSET>)
    }
}
pub trait ID3D12ToolsImpl: Sized {
    fn EnableShaderInstrumentation();
    fn ShaderInstrumentationEnabled();
}
impl ::windows::core::RuntimeName for ID3D12Tools {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12Tools";
}
impl ID3D12ToolsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ToolsImpl, const OFFSET: isize>() -> ID3D12ToolsVtbl {
        unsafe extern "system" fn EnableShaderInstrumentation<Impl: ID3D12ToolsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableShaderInstrumentation(&*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShaderInstrumentationEnabled<Impl: ID3D12ToolsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShaderInstrumentationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12Tools>, ::windows::core::GetTrustLevel, EnableShaderInstrumentation::<Impl, OFFSET>, ShaderInstrumentationEnabled::<Impl, OFFSET>)
    }
}
pub trait ID3D12VersionedRootSignatureDeserializerImpl: Sized {
    fn GetRootSignatureDescAtVersion();
    fn GetUnconvertedRootSignatureDesc();
}
impl ::windows::core::RuntimeName for ID3D12VersionedRootSignatureDeserializer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D12.ID3D12VersionedRootSignatureDeserializer";
}
impl ID3D12VersionedRootSignatureDeserializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VersionedRootSignatureDeserializerImpl, const OFFSET: isize>() -> ID3D12VersionedRootSignatureDeserializerVtbl {
        unsafe extern "system" fn GetRootSignatureDescAtVersion<Impl: ID3D12VersionedRootSignatureDeserializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, converttoversion: D3D_ROOT_SIGNATURE_VERSION, ppdesc: *mut *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRootSignatureDescAtVersion(converttoversion, ::core::mem::transmute_copy(&ppdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnconvertedRootSignatureDesc<Impl: ID3D12VersionedRootSignatureDeserializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnconvertedRootSignatureDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D12VersionedRootSignatureDeserializer>, ::windows::core::GetTrustLevel, GetRootSignatureDescAtVersion::<Impl, OFFSET>, GetUnconvertedRootSignatureDesc::<Impl, OFFSET>)
    }
}
