#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12CommandAllocator_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12CommandAllocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandAllocator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12CommandAllocator_Vtbl {
        unsafe extern "system" fn Reset<Impl: ID3D12CommandAllocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Reset: Reset::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12CommandAllocator as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12CommandList_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl {
    fn GetType(&mut self) -> D3D12_COMMAND_LIST_TYPE;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12CommandList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12CommandList_Vtbl {
        unsafe extern "system" fn GetType<Impl: ID3D12CommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_COMMAND_LIST_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType()
        }
        Self { base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetType: GetType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12CommandList as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12CommandQueue_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {
    fn UpdateTileMappings(&mut self, presource: &::core::option::Option<ID3D12Resource>, numresourceregions: u32, presourceregionstartcoordinates: *const D3D12_TILED_RESOURCE_COORDINATE, presourceregionsizes: *const D3D12_TILE_REGION_SIZE, pheap: &::core::option::Option<ID3D12Heap>, numranges: u32, prangeflags: *const D3D12_TILE_RANGE_FLAGS, pheaprangestartoffsets: *const u32, prangetilecounts: *const u32, flags: D3D12_TILE_MAPPING_FLAGS);
    fn CopyTileMappings(&mut self, pdstresource: &::core::option::Option<ID3D12Resource>, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: &::core::option::Option<ID3D12Resource>, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS);
    fn ExecuteCommandLists(&mut self, numcommandlists: u32, ppcommandlists: *const ::core::option::Option<ID3D12CommandList>);
    fn SetMarker(&mut self, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32);
    fn BeginEvent(&mut self, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32);
    fn EndEvent(&mut self);
    fn Signal(&mut self, pfence: &::core::option::Option<ID3D12Fence>, value: u64) -> ::windows::core::Result<()>;
    fn Wait(&mut self, pfence: &::core::option::Option<ID3D12Fence>, value: u64) -> ::windows::core::Result<()>;
    fn GetTimestampFrequency(&mut self) -> ::windows::core::Result<u64>;
    fn GetClockCalibration(&mut self, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> ::windows::core::Result<()>;
    fn GetDesc(&mut self) -> D3D12_COMMAND_QUEUE_DESC;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12CommandQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12CommandQueue_Vtbl {
        unsafe extern "system" fn UpdateTileMappings<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, numresourceregions: u32, presourceregionstartcoordinates: *const D3D12_TILED_RESOURCE_COORDINATE, presourceregionsizes: *const D3D12_TILE_REGION_SIZE, pheap: ::windows::core::RawPtr, numranges: u32, prangeflags: *const D3D12_TILE_RANGE_FLAGS, pheaprangestartoffsets: *const u32, prangetilecounts: *const u32, flags: D3D12_TILE_MAPPING_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateTileMappings(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&numresourceregions), ::core::mem::transmute_copy(&presourceregionstartcoordinates), ::core::mem::transmute_copy(&presourceregionsizes), ::core::mem::transmute(&pheap), ::core::mem::transmute_copy(&numranges), ::core::mem::transmute_copy(&prangeflags), ::core::mem::transmute_copy(&pheaprangestartoffsets), ::core::mem::transmute_copy(&prangetilecounts), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn CopyTileMappings<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: ::windows::core::RawPtr, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTileMappings(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&pdstregionstartcoordinate), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&psrcregionstartcoordinate), ::core::mem::transmute_copy(&pregionsize), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ExecuteCommandLists<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numcommandlists: u32, ppcommandlists: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteCommandLists(::core::mem::transmute_copy(&numcommandlists), ::core::mem::transmute_copy(&ppcommandlists))
        }
        unsafe extern "system" fn SetMarker<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMarker(::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn BeginEvent<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEvent(::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn EndEvent<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEvent()
        }
        unsafe extern "system" fn Signal<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Signal(::core::mem::transmute(&pfence), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Wait<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Wait(::core::mem::transmute(&pfence), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTimestampFrequency<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequency: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimestampFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *pfrequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClockCalibration<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClockCalibration(::core::mem::transmute_copy(&pgputimestamp), ::core::mem::transmute_copy(&pcputimestamp)).into()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_COMMAND_QUEUE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        Self {
            base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UpdateTileMappings: UpdateTileMappings::<Impl, IMPL_OFFSET>,
            CopyTileMappings: CopyTileMappings::<Impl, IMPL_OFFSET>,
            ExecuteCommandLists: ExecuteCommandLists::<Impl, IMPL_OFFSET>,
            SetMarker: SetMarker::<Impl, IMPL_OFFSET>,
            BeginEvent: BeginEvent::<Impl, IMPL_OFFSET>,
            EndEvent: EndEvent::<Impl, IMPL_OFFSET>,
            Signal: Signal::<Impl, IMPL_OFFSET>,
            Wait: Wait::<Impl, IMPL_OFFSET>,
            GetTimestampFrequency: GetTimestampFrequency::<Impl, IMPL_OFFSET>,
            GetClockCalibration: GetClockCalibration::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12CommandQueue as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12CommandSignature_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12CommandSignature_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandSignature_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12CommandSignature_Vtbl {
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12CommandSignature as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12Debug_Impl: Sized {
    fn EnableDebugLayer(&mut self);
}
impl ID3D12Debug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Debug_Vtbl {
        unsafe extern "system" fn EnableDebugLayer<Impl: ID3D12Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableDebugLayer()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), EnableDebugLayer: EnableDebugLayer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Debug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Debug1_Impl: Sized {
    fn EnableDebugLayer(&mut self);
    fn SetEnableGPUBasedValidation(&mut self, enable: super::super::Foundation::BOOL);
    fn SetEnableSynchronizedCommandQueueValidation(&mut self, enable: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Debug1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Debug1_Vtbl {
        unsafe extern "system" fn EnableDebugLayer<Impl: ID3D12Debug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableDebugLayer()
        }
        unsafe extern "system" fn SetEnableGPUBasedValidation<Impl: ID3D12Debug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableGPUBasedValidation(::core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn SetEnableSynchronizedCommandQueueValidation<Impl: ID3D12Debug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableSynchronizedCommandQueueValidation(::core::mem::transmute_copy(&enable))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnableDebugLayer: EnableDebugLayer::<Impl, IMPL_OFFSET>,
            SetEnableGPUBasedValidation: SetEnableGPUBasedValidation::<Impl, IMPL_OFFSET>,
            SetEnableSynchronizedCommandQueueValidation: SetEnableSynchronizedCommandQueueValidation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Debug1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12Debug2_Impl: Sized {
    fn SetGPUBasedValidationFlags(&mut self, flags: D3D12_GPU_BASED_VALIDATION_FLAGS);
}
impl ID3D12Debug2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Debug2_Vtbl {
        unsafe extern "system" fn SetGPUBasedValidationFlags<Impl: ID3D12Debug2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGPUBasedValidationFlags(::core::mem::transmute_copy(&flags))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetGPUBasedValidationFlags: SetGPUBasedValidationFlags::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Debug2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Debug3_Impl: Sized + ID3D12Debug_Impl {
    fn SetEnableGPUBasedValidation(&mut self, enable: super::super::Foundation::BOOL);
    fn SetEnableSynchronizedCommandQueueValidation(&mut self, enable: super::super::Foundation::BOOL);
    fn SetGPUBasedValidationFlags(&mut self, flags: D3D12_GPU_BASED_VALIDATION_FLAGS);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Debug3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Debug3_Vtbl {
        unsafe extern "system" fn SetEnableGPUBasedValidation<Impl: ID3D12Debug3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableGPUBasedValidation(::core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn SetEnableSynchronizedCommandQueueValidation<Impl: ID3D12Debug3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableSynchronizedCommandQueueValidation(::core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn SetGPUBasedValidationFlags<Impl: ID3D12Debug3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGPUBasedValidationFlags(::core::mem::transmute_copy(&flags))
        }
        Self {
            base: ID3D12Debug_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetEnableGPUBasedValidation: SetEnableGPUBasedValidation::<Impl, IMPL_OFFSET>,
            SetEnableSynchronizedCommandQueueValidation: SetEnableSynchronizedCommandQueueValidation::<Impl, IMPL_OFFSET>,
            SetGPUBasedValidationFlags: SetGPUBasedValidationFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Debug3 as ::windows::core::Interface>::IID || iid == &<ID3D12Debug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Debug4_Impl: Sized + ID3D12Debug_Impl + ID3D12Debug3_Impl {
    fn DisableDebugLayer(&mut self);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Debug4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Debug4_Vtbl {
        unsafe extern "system" fn DisableDebugLayer<Impl: ID3D12Debug4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableDebugLayer()
        }
        Self { base: ID3D12Debug3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), DisableDebugLayer: DisableDebugLayer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Debug4 as ::windows::core::Interface>::IID || iid == &<ID3D12Debug as ::windows::core::Interface>::IID || iid == &<ID3D12Debug3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Debug5_Impl: Sized + ID3D12Debug_Impl + ID3D12Debug3_Impl + ID3D12Debug4_Impl {
    fn SetEnableAutoName(&mut self, enable: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Debug5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Debug5_Vtbl {
        unsafe extern "system" fn SetEnableAutoName<Impl: ID3D12Debug5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableAutoName(::core::mem::transmute_copy(&enable))
        }
        Self { base: ID3D12Debug4_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetEnableAutoName: SetEnableAutoName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Debug5 as ::windows::core::Interface>::IID || iid == &<ID3D12Debug as ::windows::core::Interface>::IID || iid == &<ID3D12Debug3 as ::windows::core::Interface>::IID || iid == &<ID3D12Debug4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DebugCommandList_Impl: Sized {
    fn AssertResourceState(&mut self, presource: &::core::option::Option<ID3D12Resource>, subresource: u32, state: u32) -> super::super::Foundation::BOOL;
    fn SetFeatureMask(&mut self, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::Result<()>;
    fn GetFeatureMask(&mut self) -> D3D12_DEBUG_FEATURE;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12DebugCommandList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DebugCommandList_Vtbl {
        unsafe extern "system" fn AssertResourceState<Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AssertResourceState(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&state))
        }
        unsafe extern "system" fn SetFeatureMask<Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFeatureMask(::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_DEBUG_FEATURE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeatureMask()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AssertResourceState: AssertResourceState::<Impl, IMPL_OFFSET>,
            SetFeatureMask: SetFeatureMask::<Impl, IMPL_OFFSET>,
            GetFeatureMask: GetFeatureMask::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DebugCommandList as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DebugCommandList1_Impl: Sized {
    fn AssertResourceState(&mut self, presource: &::core::option::Option<ID3D12Resource>, subresource: u32, state: u32) -> super::super::Foundation::BOOL;
    fn SetDebugParameter(&mut self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()>;
    fn GetDebugParameter(&mut self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12DebugCommandList1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DebugCommandList1_Vtbl {
        unsafe extern "system" fn AssertResourceState<Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AssertResourceState(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&state))
        }
        unsafe extern "system" fn SetDebugParameter<Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AssertResourceState: AssertResourceState::<Impl, IMPL_OFFSET>,
            SetDebugParameter: SetDebugParameter::<Impl, IMPL_OFFSET>,
            GetDebugParameter: GetDebugParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DebugCommandList1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DebugCommandList2_Impl: Sized + ID3D12DebugCommandList_Impl {
    fn SetDebugParameter(&mut self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()>;
    fn GetDebugParameter(&mut self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12DebugCommandList2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DebugCommandList2_Vtbl {
        unsafe extern "system" fn SetDebugParameter<Impl: ID3D12DebugCommandList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Impl: ID3D12DebugCommandList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        Self {
            base: ID3D12DebugCommandList_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetDebugParameter: SetDebugParameter::<Impl, IMPL_OFFSET>,
            GetDebugParameter: GetDebugParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DebugCommandList2 as ::windows::core::Interface>::IID || iid == &<ID3D12DebugCommandList as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DebugCommandQueue_Impl: Sized {
    fn AssertResourceState(&mut self, presource: &::core::option::Option<ID3D12Resource>, subresource: u32, state: u32) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12DebugCommandQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DebugCommandQueue_Vtbl {
        unsafe extern "system" fn AssertResourceState<Impl: ID3D12DebugCommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AssertResourceState(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&state))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AssertResourceState: AssertResourceState::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DebugCommandQueue as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12DebugDevice_Impl: Sized {
    fn SetFeatureMask(&mut self, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::Result<()>;
    fn GetFeatureMask(&mut self) -> D3D12_DEBUG_FEATURE;
    fn ReportLiveDeviceObjects(&mut self, flags: D3D12_RLDO_FLAGS) -> ::windows::core::Result<()>;
}
impl ID3D12DebugDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DebugDevice_Vtbl {
        unsafe extern "system" fn SetFeatureMask<Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFeatureMask(::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_DEBUG_FEATURE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeatureMask()
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportLiveDeviceObjects(::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetFeatureMask: SetFeatureMask::<Impl, IMPL_OFFSET>,
            GetFeatureMask: GetFeatureMask::<Impl, IMPL_OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DebugDevice as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12DebugDevice1_Impl: Sized {
    fn SetDebugParameter(&mut self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()>;
    fn GetDebugParameter(&mut self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()>;
    fn ReportLiveDeviceObjects(&mut self, flags: D3D12_RLDO_FLAGS) -> ::windows::core::Result<()>;
}
impl ID3D12DebugDevice1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DebugDevice1_Vtbl {
        unsafe extern "system" fn SetDebugParameter<Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportLiveDeviceObjects(::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDebugParameter: SetDebugParameter::<Impl, IMPL_OFFSET>,
            GetDebugParameter: GetDebugParameter::<Impl, IMPL_OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DebugDevice1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12DebugDevice2_Impl: Sized + ID3D12DebugDevice_Impl {
    fn SetDebugParameter(&mut self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()>;
    fn GetDebugParameter(&mut self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()>;
}
impl ID3D12DebugDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DebugDevice2_Vtbl {
        unsafe extern "system" fn SetDebugParameter<Impl: ID3D12DebugDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Impl: ID3D12DebugDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        Self {
            base: ID3D12DebugDevice_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetDebugParameter: SetDebugParameter::<Impl, IMPL_OFFSET>,
            GetDebugParameter: GetDebugParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DebugDevice2 as ::windows::core::Interface>::IID || iid == &<ID3D12DebugDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DescriptorHeap_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {
    fn GetDesc(&mut self) -> D3D12_DESCRIPTOR_HEAP_DESC;
    fn GetCPUDescriptorHandleForHeapStart(&mut self) -> D3D12_CPU_DESCRIPTOR_HANDLE;
    fn GetGPUDescriptorHandleForHeapStart(&mut self) -> D3D12_GPU_DESCRIPTOR_HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12DescriptorHeap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DescriptorHeap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DescriptorHeap_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_DESCRIPTOR_HEAP_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        unsafe extern "system" fn GetCPUDescriptorHandleForHeapStart<Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetCPUDescriptorHandleForHeapStart()
        }
        unsafe extern "system" fn GetGPUDescriptorHandleForHeapStart<Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetGPUDescriptorHandleForHeapStart()
        }
        Self {
            base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetCPUDescriptorHandleForHeapStart: GetCPUDescriptorHandleForHeapStart::<Impl, IMPL_OFFSET>,
            GetGPUDescriptorHandleForHeapStart: GetGPUDescriptorHandleForHeapStart::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DescriptorHeap as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device_Impl: Sized + ID3D12Object_Impl {
    fn GetNodeCount(&mut self) -> u32;
    fn CreateCommandQueue(&mut self, pdesc: *const D3D12_COMMAND_QUEUE_DESC, riid: *const ::windows::core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateCommandAllocator(&mut self, r#type: D3D12_COMMAND_LIST_TYPE, riid: *const ::windows::core::GUID, ppcommandallocator: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateGraphicsPipelineState(&mut self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateComputePipelineState(&mut self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateCommandList(&mut self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: &::core::option::Option<ID3D12CommandAllocator>, pinitialstate: &::core::option::Option<ID3D12PipelineState>, riid: *const ::windows::core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CheckFeatureSupport(&mut self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()>;
    fn CreateDescriptorHeap(&mut self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDescriptorHandleIncrementSize(&mut self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32;
    fn CreateRootSignature(&mut self, nodemask: u32, pblobwithrootsignature: *const ::core::ffi::c_void, bloblengthinbytes: usize, riid: *const ::windows::core::GUID, ppvrootsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateConstantBufferView(&mut self, pdesc: *const D3D12_CONSTANT_BUFFER_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateShaderResourceView(&mut self, presource: &::core::option::Option<ID3D12Resource>, pdesc: *const D3D12_SHADER_RESOURCE_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateUnorderedAccessView(&mut self, presource: &::core::option::Option<ID3D12Resource>, pcounterresource: &::core::option::Option<ID3D12Resource>, pdesc: *const D3D12_UNORDERED_ACCESS_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateRenderTargetView(&mut self, presource: &::core::option::Option<ID3D12Resource>, pdesc: *const D3D12_RENDER_TARGET_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateDepthStencilView(&mut self, presource: &::core::option::Option<ID3D12Resource>, pdesc: *const D3D12_DEPTH_STENCIL_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateSampler(&mut self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CopyDescriptors(&mut self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: *const u32, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: *const u32, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE);
    fn CopyDescriptorsSimple(&mut self, numdescriptors: u32, destdescriptorrangestart: &D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: &D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE);
    fn GetResourceAllocationInfo(&mut self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC) -> D3D12_RESOURCE_ALLOCATION_INFO;
    fn GetCustomHeapProperties(&mut self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES;
    fn CreateCommittedResource(&mut self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateHeap(&mut self, pdesc: *const D3D12_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreatePlacedResource(&mut self, pheap: &::core::option::Option<ID3D12Heap>, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateReservedResource(&mut self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateSharedHandle(&mut self, pobject: &::core::option::Option<ID3D12DeviceChild>, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn OpenSharedHandle(&mut self, nthandle: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OpenSharedHandleByName(&mut self, name: super::super::Foundation::PWSTR, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn MakeResident(&mut self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>) -> ::windows::core::Result<()>;
    fn Evict(&mut self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>) -> ::windows::core::Result<()>;
    fn CreateFence(&mut self, initialvalue: u64, flags: D3D12_FENCE_FLAGS, riid: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceRemovedReason(&mut self) -> ::windows::core::Result<()>;
    fn GetCopyableFootprints(&mut self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64);
    fn CreateQueryHeap(&mut self, pdesc: *const D3D12_QUERY_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetStablePowerState(&mut self, enable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CreateCommandSignature(&mut self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: &::core::option::Option<ID3D12RootSignature>, riid: *const ::windows::core::GUID, ppvcommandsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetResourceTiling(&mut self, ptiledresource: &::core::option::Option<ID3D12Resource>, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D12_PACKED_MIP_INFO, pstandardtileshapefornonpackedmips: *mut D3D12_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING);
    fn GetAdapterLuid(&mut self) -> super::super::Foundation::LUID;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Device_Vtbl {
        unsafe extern "system" fn GetNodeCount<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNodeCount()
        }
        unsafe extern "system" fn CreateCommandQueue<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, riid: *const ::windows::core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommandQueue(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandqueue)).into()
        }
        unsafe extern "system" fn CreateCommandAllocator<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_COMMAND_LIST_TYPE, riid: *const ::windows::core::GUID, ppcommandallocator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommandAllocator(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandallocator)).into()
        }
        unsafe extern "system" fn CreateGraphicsPipelineState<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateGraphicsPipelineState(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn CreateComputePipelineState<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateComputePipelineState(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn CreateCommandList<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: ::windows::core::RawPtr, pinitialstate: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommandList(::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&pcommandallocator), ::core::mem::transmute(&pinitialstate), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandlist)).into()
        }
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        unsafe extern "system" fn CreateDescriptorHeap<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDescriptorHeap(::core::mem::transmute_copy(&pdescriptorheapdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn GetDescriptorHandleIncrementSize<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDescriptorHandleIncrementSize(::core::mem::transmute_copy(&descriptorheaptype))
        }
        unsafe extern "system" fn CreateRootSignature<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, pblobwithrootsignature: *const ::core::ffi::c_void, bloblengthinbytes: usize, riid: *const ::windows::core::GUID, ppvrootsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateRootSignature(::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&pblobwithrootsignature), ::core::mem::transmute_copy(&bloblengthinbytes), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvrootsignature)).into()
        }
        unsafe extern "system" fn CreateConstantBufferView<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_CONSTANT_BUFFER_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateConstantBufferView(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CreateShaderResourceView<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D12_SHADER_RESOURCE_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateShaderResourceView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pcounterresource: ::windows::core::RawPtr, pdesc: *const D3D12_UNORDERED_ACCESS_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateUnorderedAccessView(::core::mem::transmute(&presource), ::core::mem::transmute(&pcounterresource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CreateRenderTargetView<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D12_RENDER_TARGET_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateRenderTargetView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CreateDepthStencilView<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D12_DEPTH_STENCIL_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDepthStencilView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CreateSampler<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSampler(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CopyDescriptors<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: *const u32, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: *const u32, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyDescriptors(::core::mem::transmute_copy(&numdestdescriptorranges), ::core::mem::transmute_copy(&pdestdescriptorrangestarts), ::core::mem::transmute_copy(&pdestdescriptorrangesizes), ::core::mem::transmute_copy(&numsrcdescriptorranges), ::core::mem::transmute_copy(&psrcdescriptorrangestarts), ::core::mem::transmute_copy(&psrcdescriptorrangesizes), ::core::mem::transmute_copy(&descriptorheapstype))
        }
        unsafe extern "system" fn CopyDescriptorsSimple<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyDescriptorsSimple(::core::mem::transmute_copy(&numdescriptors), ::core::mem::transmute_copy(&destdescriptorrangestart), ::core::mem::transmute_copy(&srcdescriptorrangestart), ::core::mem::transmute_copy(&descriptorheapstype))
        }
        unsafe extern "system" fn GetResourceAllocationInfo<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetResourceAllocationInfo(::core::mem::transmute_copy(&visiblemask), ::core::mem::transmute_copy(&numresourcedescs), ::core::mem::transmute_copy(&presourcedescs))
        }
        unsafe extern "system" fn GetCustomHeapProperties<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_HEAP_PROPERTIES, nodemask: u32, heaptype: D3D12_HEAP_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetCustomHeapProperties(::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&heaptype))
        }
        unsafe extern "system" fn CreateCommittedResource<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommittedResource(::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&heapflags), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialresourcestate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateHeap<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateHeap(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn CreatePlacedResource<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheap: ::windows::core::RawPtr, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePlacedResource(::core::mem::transmute(&pheap), ::core::mem::transmute_copy(&heapoffset), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateReservedResource<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateReservedResource(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateSharedHandle<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: super::super::Foundation::PWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSharedHandle(::core::mem::transmute(&pobject), ::core::mem::transmute_copy(&pattributes), ::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedHandle<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nthandle: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenSharedHandle(::core::mem::transmute_copy(&nthandle), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn OpenSharedHandleByName<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, access: u32, pnthandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenSharedHandleByName(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&access)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnthandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeResident<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeResident(::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn Evict<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Evict(::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn CreateFence<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: u64, flags: D3D12_FENCE_FLAGS, riid: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateFence(::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppfence)).into()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceRemovedReason().into()
        }
        unsafe extern "system" fn GetCopyableFootprints<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCopyableFootprints(::core::mem::transmute_copy(&presourcedesc), ::core::mem::transmute_copy(&firstsubresource), ::core::mem::transmute_copy(&numsubresources), ::core::mem::transmute_copy(&baseoffset), ::core::mem::transmute_copy(&playouts), ::core::mem::transmute_copy(&pnumrows), ::core::mem::transmute_copy(&prowsizeinbytes), ::core::mem::transmute_copy(&ptotalbytes))
        }
        unsafe extern "system" fn CreateQueryHeap<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_QUERY_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateQueryHeap(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn SetStablePowerState<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStablePowerState(::core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn CreateCommandSignature<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvcommandsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommandSignature(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&prootsignature), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvcommandsignature)).into()
        }
        unsafe extern "system" fn GetResourceTiling<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D12_PACKED_MIP_INFO, pstandardtileshapefornonpackedmips: *mut D3D12_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResourceTiling(::core::mem::transmute(&ptiledresource), ::core::mem::transmute_copy(&pnumtilesforentireresource), ::core::mem::transmute_copy(&ppackedmipdesc), ::core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), ::core::mem::transmute_copy(&pnumsubresourcetilings), ::core::mem::transmute_copy(&firstsubresourcetilingtoget), ::core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips))
        }
        unsafe extern "system" fn GetAdapterLuid<Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetAdapterLuid()
        }
        Self {
            base: ID3D12Object_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetNodeCount: GetNodeCount::<Impl, IMPL_OFFSET>,
            CreateCommandQueue: CreateCommandQueue::<Impl, IMPL_OFFSET>,
            CreateCommandAllocator: CreateCommandAllocator::<Impl, IMPL_OFFSET>,
            CreateGraphicsPipelineState: CreateGraphicsPipelineState::<Impl, IMPL_OFFSET>,
            CreateComputePipelineState: CreateComputePipelineState::<Impl, IMPL_OFFSET>,
            CreateCommandList: CreateCommandList::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            CreateDescriptorHeap: CreateDescriptorHeap::<Impl, IMPL_OFFSET>,
            GetDescriptorHandleIncrementSize: GetDescriptorHandleIncrementSize::<Impl, IMPL_OFFSET>,
            CreateRootSignature: CreateRootSignature::<Impl, IMPL_OFFSET>,
            CreateConstantBufferView: CreateConstantBufferView::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateSampler: CreateSampler::<Impl, IMPL_OFFSET>,
            CopyDescriptors: CopyDescriptors::<Impl, IMPL_OFFSET>,
            CopyDescriptorsSimple: CopyDescriptorsSimple::<Impl, IMPL_OFFSET>,
            GetResourceAllocationInfo: GetResourceAllocationInfo::<Impl, IMPL_OFFSET>,
            GetCustomHeapProperties: GetCustomHeapProperties::<Impl, IMPL_OFFSET>,
            CreateCommittedResource: CreateCommittedResource::<Impl, IMPL_OFFSET>,
            CreateHeap: CreateHeap::<Impl, IMPL_OFFSET>,
            CreatePlacedResource: CreatePlacedResource::<Impl, IMPL_OFFSET>,
            CreateReservedResource: CreateReservedResource::<Impl, IMPL_OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Impl, IMPL_OFFSET>,
            OpenSharedHandle: OpenSharedHandle::<Impl, IMPL_OFFSET>,
            OpenSharedHandleByName: OpenSharedHandleByName::<Impl, IMPL_OFFSET>,
            MakeResident: MakeResident::<Impl, IMPL_OFFSET>,
            Evict: Evict::<Impl, IMPL_OFFSET>,
            CreateFence: CreateFence::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetCopyableFootprints: GetCopyableFootprints::<Impl, IMPL_OFFSET>,
            CreateQueryHeap: CreateQueryHeap::<Impl, IMPL_OFFSET>,
            SetStablePowerState: SetStablePowerState::<Impl, IMPL_OFFSET>,
            CreateCommandSignature: CreateCommandSignature::<Impl, IMPL_OFFSET>,
            GetResourceTiling: GetResourceTiling::<Impl, IMPL_OFFSET>,
            GetAdapterLuid: GetAdapterLuid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Device as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device1_Impl: Sized + ID3D12Object_Impl + ID3D12Device_Impl {
    fn CreatePipelineLibrary(&mut self, plibraryblob: *const ::core::ffi::c_void, bloblength: usize, riid: *const ::windows::core::GUID, pppipelinelibrary: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetEventOnMultipleFenceCompletion(&mut self, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetResidencyPriority(&mut self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Device1_Vtbl {
        unsafe extern "system" fn CreatePipelineLibrary<Impl: ID3D12Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibraryblob: *const ::core::ffi::c_void, bloblength: usize, riid: *const ::windows::core::GUID, pppipelinelibrary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePipelineLibrary(::core::mem::transmute_copy(&plibraryblob), ::core::mem::transmute_copy(&bloblength), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinelibrary)).into()
        }
        unsafe extern "system" fn SetEventOnMultipleFenceCompletion<Impl: ID3D12Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfences: *const ::windows::core::RawPtr, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventOnMultipleFenceCompletion(::core::mem::transmute_copy(&ppfences), ::core::mem::transmute_copy(&pfencevalues), ::core::mem::transmute_copy(&numfences), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn SetResidencyPriority<Impl: ID3D12Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const ::windows::core::RawPtr, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResidencyPriority(::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects), ::core::mem::transmute_copy(&ppriorities)).into()
        }
        Self {
            base: ID3D12Device_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreatePipelineLibrary: CreatePipelineLibrary::<Impl, IMPL_OFFSET>,
            SetEventOnMultipleFenceCompletion: SetEventOnMultipleFenceCompletion::<Impl, IMPL_OFFSET>,
            SetResidencyPriority: SetResidencyPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Device1 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device2_Impl: Sized + ID3D12Object_Impl + ID3D12Device_Impl + ID3D12Device1_Impl {
    fn CreatePipelineState(&mut self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Device2_Vtbl {
        unsafe extern "system" fn CreatePipelineState<Impl: ID3D12Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePipelineState(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        Self { base: ID3D12Device1_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreatePipelineState: CreatePipelineState::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Device2 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12Device as ::windows::core::Interface>::IID || iid == &<ID3D12Device1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device3_Impl: Sized + ID3D12Object_Impl + ID3D12Device_Impl + ID3D12Device1_Impl + ID3D12Device2_Impl {
    fn OpenExistingHeapFromAddress(&mut self, paddress: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OpenExistingHeapFromFileMapping(&mut self, hfilemapping: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn EnqueueMakeResident(&mut self, flags: D3D12_RESIDENCY_FLAGS, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, pfencetosignal: &::core::option::Option<ID3D12Fence>, fencevaluetosignal: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Device3_Vtbl {
        unsafe extern "system" fn OpenExistingHeapFromAddress<Impl: ID3D12Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenExistingHeapFromAddress(::core::mem::transmute_copy(&paddress), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn OpenExistingHeapFromFileMapping<Impl: ID3D12Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfilemapping: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenExistingHeapFromFileMapping(::core::mem::transmute_copy(&hfilemapping), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn EnqueueMakeResident<Impl: ID3D12Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_RESIDENCY_FLAGS, numobjects: u32, ppobjects: *const ::windows::core::RawPtr, pfencetosignal: ::windows::core::RawPtr, fencevaluetosignal: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnqueueMakeResident(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects), ::core::mem::transmute(&pfencetosignal), ::core::mem::transmute_copy(&fencevaluetosignal)).into()
        }
        Self {
            base: ID3D12Device2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OpenExistingHeapFromAddress: OpenExistingHeapFromAddress::<Impl, IMPL_OFFSET>,
            OpenExistingHeapFromFileMapping: OpenExistingHeapFromFileMapping::<Impl, IMPL_OFFSET>,
            EnqueueMakeResident: EnqueueMakeResident::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Device3 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12Device as ::windows::core::Interface>::IID || iid == &<ID3D12Device1 as ::windows::core::Interface>::IID || iid == &<ID3D12Device2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device4_Impl: Sized + ID3D12Object_Impl + ID3D12Device_Impl + ID3D12Device1_Impl + ID3D12Device2_Impl + ID3D12Device3_Impl {
    fn CreateCommandList1(&mut self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS, riid: *const ::windows::core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateProtectedResourceSession(&mut self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateCommittedResource1(&mut self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: &::core::option::Option<ID3D12ProtectedResourceSession>, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateHeap1(&mut self, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: &::core::option::Option<ID3D12ProtectedResourceSession>, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateReservedResource1(&mut self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: &::core::option::Option<ID3D12ProtectedResourceSession>, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetResourceAllocationInfo1(&mut self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) -> D3D12_RESOURCE_ALLOCATION_INFO;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Device4_Vtbl {
        unsafe extern "system" fn CreateCommandList1<Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS, riid: *const ::windows::core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommandList1(::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandlist)).into()
        }
        unsafe extern "system" fn CreateProtectedResourceSession<Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateProtectedResourceSession(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppsession)).into()
        }
        unsafe extern "system" fn CreateCommittedResource1<Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::windows::core::RawPtr, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommittedResource1(::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&heapflags), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialresourcestate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute(&pprotectedsession), ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateHeap1<Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateHeap1(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&pprotectedsession), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn CreateReservedResource1<Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateReservedResource1(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute(&pprotectedsession), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn GetResourceAllocationInfo1<Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetResourceAllocationInfo1(::core::mem::transmute_copy(&visiblemask), ::core::mem::transmute_copy(&numresourcedescs), ::core::mem::transmute_copy(&presourcedescs), ::core::mem::transmute_copy(&presourceallocationinfo1))
        }
        Self {
            base: ID3D12Device3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateCommandList1: CreateCommandList1::<Impl, IMPL_OFFSET>,
            CreateProtectedResourceSession: CreateProtectedResourceSession::<Impl, IMPL_OFFSET>,
            CreateCommittedResource1: CreateCommittedResource1::<Impl, IMPL_OFFSET>,
            CreateHeap1: CreateHeap1::<Impl, IMPL_OFFSET>,
            CreateReservedResource1: CreateReservedResource1::<Impl, IMPL_OFFSET>,
            GetResourceAllocationInfo1: GetResourceAllocationInfo1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Device4 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12Device as ::windows::core::Interface>::IID || iid == &<ID3D12Device1 as ::windows::core::Interface>::IID || iid == &<ID3D12Device2 as ::windows::core::Interface>::IID || iid == &<ID3D12Device3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device5_Impl: Sized + ID3D12Object_Impl + ID3D12Device_Impl + ID3D12Device1_Impl + ID3D12Device2_Impl + ID3D12Device3_Impl + ID3D12Device4_Impl {
    fn CreateLifetimeTracker(&mut self, powner: &::core::option::Option<ID3D12LifetimeOwner>, riid: *const ::windows::core::GUID, ppvtracker: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RemoveDevice(&mut self);
    fn EnumerateMetaCommands(&mut self, pnummetacommands: *mut u32, pdescs: *mut D3D12_META_COMMAND_DESC) -> ::windows::core::Result<()>;
    fn EnumerateMetaCommandParameters(&mut self, commandid: *const ::windows::core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: *mut u32, pparametercount: *mut u32, pparameterdescs: *mut D3D12_META_COMMAND_PARAMETER_DESC) -> ::windows::core::Result<()>;
    fn CreateMetaCommand(&mut self, commandid: *const ::windows::core::GUID, nodemask: u32, pcreationparametersdata: *const ::core::ffi::c_void, creationparametersdatasizeinbytes: usize, riid: *const ::windows::core::GUID, ppmetacommand: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateStateObject(&mut self, pdesc: *const D3D12_STATE_OBJECT_DESC, riid: *const ::windows::core::GUID, ppstateobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetRaytracingAccelerationStructurePrebuildInfo(&mut self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO);
    fn CheckDriverMatchingIdentifier(&mut self, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Device5_Vtbl {
        unsafe extern "system" fn CreateLifetimeTracker<Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, powner: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvtracker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateLifetimeTracker(::core::mem::transmute(&powner), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvtracker)).into()
        }
        unsafe extern "system" fn RemoveDevice<Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDevice()
        }
        unsafe extern "system" fn EnumerateMetaCommands<Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnummetacommands: *mut u32, pdescs: *mut D3D12_META_COMMAND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateMetaCommands(::core::mem::transmute_copy(&pnummetacommands), ::core::mem::transmute_copy(&pdescs)).into()
        }
        unsafe extern "system" fn EnumerateMetaCommandParameters<Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: *const ::windows::core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: *mut u32, pparametercount: *mut u32, pparameterdescs: *mut D3D12_META_COMMAND_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateMetaCommandParameters(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&ptotalstructuresizeinbytes), ::core::mem::transmute_copy(&pparametercount), ::core::mem::transmute_copy(&pparameterdescs)).into()
        }
        unsafe extern "system" fn CreateMetaCommand<Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: *const ::windows::core::GUID, nodemask: u32, pcreationparametersdata: *const ::core::ffi::c_void, creationparametersdatasizeinbytes: usize, riid: *const ::windows::core::GUID, ppmetacommand: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateMetaCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&pcreationparametersdata), ::core::mem::transmute_copy(&creationparametersdatasizeinbytes), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppmetacommand)).into()
        }
        unsafe extern "system" fn CreateStateObject<Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_STATE_OBJECT_DESC, riid: *const ::windows::core::GUID, ppstateobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateStateObject(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppstateobject)).into()
        }
        unsafe extern "system" fn GetRaytracingAccelerationStructurePrebuildInfo<Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRaytracingAccelerationStructurePrebuildInfo(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinfo))
        }
        unsafe extern "system" fn CheckDriverMatchingIdentifier<Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckDriverMatchingIdentifier(::core::mem::transmute_copy(&serializeddatatype), ::core::mem::transmute_copy(&pidentifiertocheck))
        }
        Self {
            base: ID3D12Device4_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateLifetimeTracker: CreateLifetimeTracker::<Impl, IMPL_OFFSET>,
            RemoveDevice: RemoveDevice::<Impl, IMPL_OFFSET>,
            EnumerateMetaCommands: EnumerateMetaCommands::<Impl, IMPL_OFFSET>,
            EnumerateMetaCommandParameters: EnumerateMetaCommandParameters::<Impl, IMPL_OFFSET>,
            CreateMetaCommand: CreateMetaCommand::<Impl, IMPL_OFFSET>,
            CreateStateObject: CreateStateObject::<Impl, IMPL_OFFSET>,
            GetRaytracingAccelerationStructurePrebuildInfo: GetRaytracingAccelerationStructurePrebuildInfo::<Impl, IMPL_OFFSET>,
            CheckDriverMatchingIdentifier: CheckDriverMatchingIdentifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Device5 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12Device as ::windows::core::Interface>::IID || iid == &<ID3D12Device1 as ::windows::core::Interface>::IID || iid == &<ID3D12Device2 as ::windows::core::Interface>::IID || iid == &<ID3D12Device3 as ::windows::core::Interface>::IID || iid == &<ID3D12Device4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device6_Impl: Sized + ID3D12Object_Impl + ID3D12Device_Impl + ID3D12Device1_Impl + ID3D12Device2_Impl + ID3D12Device3_Impl + ID3D12Device4_Impl + ID3D12Device5_Impl {
    fn SetBackgroundProcessingMode(&mut self, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: super::super::Foundation::HANDLE) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Device6_Vtbl {
        unsafe extern "system" fn SetBackgroundProcessingMode<Impl: ID3D12Device6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: super::super::Foundation::HANDLE, pbfurthermeasurementsdesired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackgroundProcessingMode(::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&measurementsaction), ::core::mem::transmute_copy(&heventtosignaluponcompletion)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbfurthermeasurementsdesired = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D12Device5_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetBackgroundProcessingMode: SetBackgroundProcessingMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Device6 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12Device as ::windows::core::Interface>::IID || iid == &<ID3D12Device1 as ::windows::core::Interface>::IID || iid == &<ID3D12Device2 as ::windows::core::Interface>::IID || iid == &<ID3D12Device3 as ::windows::core::Interface>::IID || iid == &<ID3D12Device4 as ::windows::core::Interface>::IID || iid == &<ID3D12Device5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device7_Impl: Sized + ID3D12Object_Impl + ID3D12Device_Impl + ID3D12Device1_Impl + ID3D12Device2_Impl + ID3D12Device3_Impl + ID3D12Device4_Impl + ID3D12Device5_Impl + ID3D12Device6_Impl {
    fn AddToStateObject(&mut self, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: &::core::option::Option<ID3D12StateObject>, riid: *const ::windows::core::GUID, ppnewstateobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateProtectedResourceSession1(&mut self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Device7_Vtbl {
        unsafe extern "system" fn AddToStateObject<Impl: ID3D12Device7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppnewstateobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToStateObject(::core::mem::transmute_copy(&paddition), ::core::mem::transmute(&pstateobjecttogrowfrom), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppnewstateobject)).into()
        }
        unsafe extern "system" fn CreateProtectedResourceSession1<Impl: ID3D12Device7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateProtectedResourceSession1(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppsession)).into()
        }
        Self {
            base: ID3D12Device6_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddToStateObject: AddToStateObject::<Impl, IMPL_OFFSET>,
            CreateProtectedResourceSession1: CreateProtectedResourceSession1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Device7 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12Device as ::windows::core::Interface>::IID || iid == &<ID3D12Device1 as ::windows::core::Interface>::IID || iid == &<ID3D12Device2 as ::windows::core::Interface>::IID || iid == &<ID3D12Device3 as ::windows::core::Interface>::IID || iid == &<ID3D12Device4 as ::windows::core::Interface>::IID || iid == &<ID3D12Device5 as ::windows::core::Interface>::IID || iid == &<ID3D12Device6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device8_Impl: Sized + ID3D12Object_Impl + ID3D12Device_Impl + ID3D12Device1_Impl + ID3D12Device2_Impl + ID3D12Device3_Impl + ID3D12Device4_Impl + ID3D12Device5_Impl + ID3D12Device6_Impl + ID3D12Device7_Impl {
    fn GetResourceAllocationInfo2(&mut self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) -> D3D12_RESOURCE_ALLOCATION_INFO;
    fn CreateCommittedResource2(&mut self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: &::core::option::Option<ID3D12ProtectedResourceSession>, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreatePlacedResource1(&mut self, pheap: &::core::option::Option<ID3D12Heap>, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateSamplerFeedbackUnorderedAccessView(&mut self, ptargetedresource: &::core::option::Option<ID3D12Resource>, pfeedbackresource: &::core::option::Option<ID3D12Resource>, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn GetCopyableFootprints1(&mut self, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Device8_Vtbl {
        unsafe extern "system" fn GetResourceAllocationInfo2<Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetResourceAllocationInfo2(::core::mem::transmute_copy(&visiblemask), ::core::mem::transmute_copy(&numresourcedescs), ::core::mem::transmute_copy(&presourcedescs), ::core::mem::transmute_copy(&presourceallocationinfo1))
        }
        unsafe extern "system" fn CreateCommittedResource2<Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::windows::core::RawPtr, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommittedResource2(::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&heapflags), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialresourcestate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute(&pprotectedsession), ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreatePlacedResource1<Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheap: ::windows::core::RawPtr, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePlacedResource1(::core::mem::transmute(&pheap), ::core::mem::transmute_copy(&heapoffset), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateSamplerFeedbackUnorderedAccessView<Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetedresource: ::windows::core::RawPtr, pfeedbackresource: ::windows::core::RawPtr, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSamplerFeedbackUnorderedAccessView(::core::mem::transmute(&ptargetedresource), ::core::mem::transmute(&pfeedbackresource), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn GetCopyableFootprints1<Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCopyableFootprints1(::core::mem::transmute_copy(&presourcedesc), ::core::mem::transmute_copy(&firstsubresource), ::core::mem::transmute_copy(&numsubresources), ::core::mem::transmute_copy(&baseoffset), ::core::mem::transmute_copy(&playouts), ::core::mem::transmute_copy(&pnumrows), ::core::mem::transmute_copy(&prowsizeinbytes), ::core::mem::transmute_copy(&ptotalbytes))
        }
        Self {
            base: ID3D12Device7_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetResourceAllocationInfo2: GetResourceAllocationInfo2::<Impl, IMPL_OFFSET>,
            CreateCommittedResource2: CreateCommittedResource2::<Impl, IMPL_OFFSET>,
            CreatePlacedResource1: CreatePlacedResource1::<Impl, IMPL_OFFSET>,
            CreateSamplerFeedbackUnorderedAccessView: CreateSamplerFeedbackUnorderedAccessView::<Impl, IMPL_OFFSET>,
            GetCopyableFootprints1: GetCopyableFootprints1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Device8 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12Device as ::windows::core::Interface>::IID || iid == &<ID3D12Device1 as ::windows::core::Interface>::IID || iid == &<ID3D12Device2 as ::windows::core::Interface>::IID || iid == &<ID3D12Device3 as ::windows::core::Interface>::IID || iid == &<ID3D12Device4 as ::windows::core::Interface>::IID || iid == &<ID3D12Device5 as ::windows::core::Interface>::IID || iid == &<ID3D12Device6 as ::windows::core::Interface>::IID || iid == &<ID3D12Device7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device9_Impl: Sized + ID3D12Object_Impl + ID3D12Device_Impl + ID3D12Device1_Impl + ID3D12Device2_Impl + ID3D12Device3_Impl + ID3D12Device4_Impl + ID3D12Device5_Impl + ID3D12Device6_Impl + ID3D12Device7_Impl + ID3D12Device8_Impl {
    fn CreateShaderCacheSession(&mut self, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, riid: *const ::windows::core::GUID, ppvsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShaderCacheControl(&mut self, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> ::windows::core::Result<()>;
    fn CreateCommandQueue1(&mut self, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Device9_Vtbl {
        unsafe extern "system" fn CreateShaderCacheSession<Impl: ID3D12Device9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, riid: *const ::windows::core::GUID, ppvsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateShaderCacheSession(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvsession)).into()
        }
        unsafe extern "system" fn ShaderCacheControl<Impl: ID3D12Device9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShaderCacheControl(::core::mem::transmute_copy(&kinds), ::core::mem::transmute_copy(&control)).into()
        }
        unsafe extern "system" fn CreateCommandQueue1<Impl: ID3D12Device9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommandQueue1(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&creatorid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandqueue)).into()
        }
        Self {
            base: ID3D12Device8_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateShaderCacheSession: CreateShaderCacheSession::<Impl, IMPL_OFFSET>,
            ShaderCacheControl: ShaderCacheControl::<Impl, IMPL_OFFSET>,
            CreateCommandQueue1: CreateCommandQueue1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Device9 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12Device as ::windows::core::Interface>::IID || iid == &<ID3D12Device1 as ::windows::core::Interface>::IID || iid == &<ID3D12Device2 as ::windows::core::Interface>::IID || iid == &<ID3D12Device3 as ::windows::core::Interface>::IID || iid == &<ID3D12Device4 as ::windows::core::Interface>::IID || iid == &<ID3D12Device5 as ::windows::core::Interface>::IID || iid == &<ID3D12Device6 as ::windows::core::Interface>::IID || iid == &<ID3D12Device7 as ::windows::core::Interface>::IID || iid == &<ID3D12Device8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DeviceChild_Impl: Sized + ID3D12Object_Impl {
    fn GetDevice(&mut self, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12DeviceChild_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceChild_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DeviceChild_Vtbl {
        unsafe extern "system" fn GetDevice<Impl: ID3D12DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvdevice)).into()
        }
        Self { base: ID3D12Object_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDevice: GetDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DeviceRemovedExtendedData_Impl: Sized {
    fn GetAutoBreadcrumbsOutput(&mut self) -> ::windows::core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT>;
    fn GetPageFaultAllocationOutput(&mut self) -> ::windows::core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12DeviceRemovedExtendedData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData_Vtbl {
        unsafe extern "system" fn GetAutoBreadcrumbsOutput<Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutoBreadcrumbsOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageFaultAllocationOutput<Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageFaultAllocationOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAutoBreadcrumbsOutput: GetAutoBreadcrumbsOutput::<Impl, IMPL_OFFSET>,
            GetPageFaultAllocationOutput: GetPageFaultAllocationOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DeviceRemovedExtendedData1_Impl: Sized + ID3D12DeviceRemovedExtendedData_Impl {
    fn GetAutoBreadcrumbsOutput1(&mut self) -> ::windows::core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1>;
    fn GetPageFaultAllocationOutput1(&mut self) -> ::windows::core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT1>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12DeviceRemovedExtendedData1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData1_Vtbl {
        unsafe extern "system" fn GetAutoBreadcrumbsOutput1<Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutoBreadcrumbsOutput1() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageFaultAllocationOutput1<Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageFaultAllocationOutput1() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D12DeviceRemovedExtendedData_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAutoBreadcrumbsOutput1: GetAutoBreadcrumbsOutput1::<Impl, IMPL_OFFSET>,
            GetPageFaultAllocationOutput1: GetPageFaultAllocationOutput1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedData1 as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceRemovedExtendedData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DeviceRemovedExtendedData2_Impl: Sized + ID3D12DeviceRemovedExtendedData_Impl + ID3D12DeviceRemovedExtendedData1_Impl {
    fn GetPageFaultAllocationOutput2(&mut self) -> ::windows::core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT2>;
    fn GetDeviceState(&mut self) -> D3D12_DRED_DEVICE_STATE;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12DeviceRemovedExtendedData2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData2_Vtbl {
        unsafe extern "system" fn GetPageFaultAllocationOutput2<Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageFaultAllocationOutput2() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceState<Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_DRED_DEVICE_STATE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceState()
        }
        Self {
            base: ID3D12DeviceRemovedExtendedData1_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPageFaultAllocationOutput2: GetPageFaultAllocationOutput2::<Impl, IMPL_OFFSET>,
            GetDeviceState: GetDeviceState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedData2 as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceRemovedExtendedData as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceRemovedExtendedData1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12DeviceRemovedExtendedDataSettings_Impl: Sized {
    fn SetAutoBreadcrumbsEnablement(&mut self, enablement: D3D12_DRED_ENABLEMENT);
    fn SetPageFaultEnablement(&mut self, enablement: D3D12_DRED_ENABLEMENT);
    fn SetWatsonDumpEnablement(&mut self, enablement: D3D12_DRED_ENABLEMENT);
}
impl ID3D12DeviceRemovedExtendedDataSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DeviceRemovedExtendedDataSettings_Vtbl {
        unsafe extern "system" fn SetAutoBreadcrumbsEnablement<Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoBreadcrumbsEnablement(::core::mem::transmute_copy(&enablement))
        }
        unsafe extern "system" fn SetPageFaultEnablement<Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageFaultEnablement(::core::mem::transmute_copy(&enablement))
        }
        unsafe extern "system" fn SetWatsonDumpEnablement<Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWatsonDumpEnablement(::core::mem::transmute_copy(&enablement))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAutoBreadcrumbsEnablement: SetAutoBreadcrumbsEnablement::<Impl, IMPL_OFFSET>,
            SetPageFaultEnablement: SetPageFaultEnablement::<Impl, IMPL_OFFSET>,
            SetWatsonDumpEnablement: SetWatsonDumpEnablement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedDataSettings as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12DeviceRemovedExtendedDataSettings1_Impl: Sized + ID3D12DeviceRemovedExtendedDataSettings_Impl {
    fn SetBreadcrumbContextEnablement(&mut self, enablement: D3D12_DRED_ENABLEMENT);
}
impl ID3D12DeviceRemovedExtendedDataSettings1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataSettings1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12DeviceRemovedExtendedDataSettings1_Vtbl {
        unsafe extern "system" fn SetBreadcrumbContextEnablement<Impl: ID3D12DeviceRemovedExtendedDataSettings1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreadcrumbContextEnablement(::core::mem::transmute_copy(&enablement))
        }
        Self {
            base: ID3D12DeviceRemovedExtendedDataSettings_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetBreadcrumbContextEnablement: SetBreadcrumbContextEnablement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedDataSettings1 as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceRemovedExtendedDataSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Fence_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {
    fn GetCompletedValue(&mut self) -> u64;
    fn SetEventOnCompletion(&mut self, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn Signal(&mut self, value: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Fence_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Fence_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Fence_Vtbl {
        unsafe extern "system" fn GetCompletedValue<Impl: ID3D12Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCompletedValue()
        }
        unsafe extern "system" fn SetEventOnCompletion<Impl: ID3D12Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventOnCompletion(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Signal<Impl: ID3D12Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Signal(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCompletedValue: GetCompletedValue::<Impl, IMPL_OFFSET>,
            SetEventOnCompletion: SetEventOnCompletion::<Impl, IMPL_OFFSET>,
            Signal: Signal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Fence as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Fence1_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl + ID3D12Fence_Impl {
    fn GetCreationFlags(&mut self) -> D3D12_FENCE_FLAGS;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Fence1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Fence1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Fence1_Vtbl {
        unsafe extern "system" fn GetCreationFlags<Impl: ID3D12Fence1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_FENCE_FLAGS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCreationFlags()
        }
        Self { base: ID3D12Fence_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetCreationFlags: GetCreationFlags::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Fence1 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID || iid == &<ID3D12Fence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D12FunctionParameterReflection_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D12_PARAMETER_DESC>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D12FunctionParameterReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionParameterReflection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12FunctionParameterReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12FunctionParameterReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12FunctionParameterReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D12FunctionReflection_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D12_FUNCTION_DESC>;
    fn GetConstantBufferByIndex(&mut self, bufferindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&mut self, resourceindex: u32) -> ::windows::core::Result<D3D12_SHADER_INPUT_BIND_DESC>;
    fn GetVariableByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&mut self, name: super::super::Foundation::PSTR) -> ::windows::core::Result<D3D12_SHADER_INPUT_BIND_DESC>;
    fn GetFunctionParameter(&mut self, parameterindex: i32) -> ::core::option::Option<ID3D12FunctionParameterReflection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D12FunctionReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionReflection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12FunctionReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_FUNCTION_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&bufferindex))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDescByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionParameter<Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: i32) -> ::core::option::Option<ID3D12FunctionParameterReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFunctionParameter(::core::mem::transmute_copy(&parameterindex))
        }
        Self {
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl, IMPL_OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Impl, IMPL_OFFSET>,
            GetVariableByName: GetVariableByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Impl, IMPL_OFFSET>,
            GetFunctionParameter: GetFunctionParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12FunctionReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12CommandList_Impl {
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self, pallocator: &::core::option::Option<ID3D12CommandAllocator>, pinitialstate: &::core::option::Option<ID3D12PipelineState>) -> ::windows::core::Result<()>;
    fn ClearState(&mut self, ppipelinestate: &::core::option::Option<ID3D12PipelineState>);
    fn DrawInstanced(&mut self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn DrawIndexedInstanced(&mut self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn Dispatch(&mut self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
    fn CopyBufferRegion(&mut self, pdstbuffer: &::core::option::Option<ID3D12Resource>, dstoffset: u64, psrcbuffer: &::core::option::Option<ID3D12Resource>, srcoffset: u64, numbytes: u64);
    fn CopyTextureRegion(&mut self, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: *const D3D12_BOX);
    fn CopyResource(&mut self, pdstresource: &::core::option::Option<ID3D12Resource>, psrcresource: &::core::option::Option<ID3D12Resource>);
    fn CopyTiles(&mut self, ptiledresource: &::core::option::Option<ID3D12Resource>, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: &::core::option::Option<ID3D12Resource>, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS);
    fn ResolveSubresource(&mut self, pdstresource: &::core::option::Option<ID3D12Resource>, dstsubresource: u32, psrcresource: &::core::option::Option<ID3D12Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn IASetPrimitiveTopology(&mut self, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn RSSetViewports(&mut self, numviewports: u32, pviewports: *const D3D12_VIEWPORT);
    fn RSSetScissorRects(&mut self, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn OMSetBlendFactor(&mut self, blendfactor: *const f32);
    fn OMSetStencilRef(&mut self, stencilref: u32);
    fn SetPipelineState(&mut self, ppipelinestate: &::core::option::Option<ID3D12PipelineState>);
    fn ResourceBarrier(&mut self, numbarriers: u32, pbarriers: *const D3D12_RESOURCE_BARRIER);
    fn ExecuteBundle(&mut self, pcommandlist: &::core::option::Option<ID3D12GraphicsCommandList>);
    fn SetDescriptorHeaps(&mut self, numdescriptorheaps: u32, ppdescriptorheaps: *const ::core::option::Option<ID3D12DescriptorHeap>);
    fn SetComputeRootSignature(&mut self, prootsignature: &::core::option::Option<ID3D12RootSignature>);
    fn SetGraphicsRootSignature(&mut self, prootsignature: &::core::option::Option<ID3D12RootSignature>);
    fn SetComputeRootDescriptorTable(&mut self, rootparameterindex: u32, basedescriptor: &D3D12_GPU_DESCRIPTOR_HANDLE);
    fn SetGraphicsRootDescriptorTable(&mut self, rootparameterindex: u32, basedescriptor: &D3D12_GPU_DESCRIPTOR_HANDLE);
    fn SetComputeRoot32BitConstant(&mut self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32);
    fn SetGraphicsRoot32BitConstant(&mut self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32);
    fn SetComputeRoot32BitConstants(&mut self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32);
    fn SetGraphicsRoot32BitConstants(&mut self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32);
    fn SetComputeRootConstantBufferView(&mut self, rootparameterindex: u32, bufferlocation: u64);
    fn SetGraphicsRootConstantBufferView(&mut self, rootparameterindex: u32, bufferlocation: u64);
    fn SetComputeRootShaderResourceView(&mut self, rootparameterindex: u32, bufferlocation: u64);
    fn SetGraphicsRootShaderResourceView(&mut self, rootparameterindex: u32, bufferlocation: u64);
    fn SetComputeRootUnorderedAccessView(&mut self, rootparameterindex: u32, bufferlocation: u64);
    fn SetGraphicsRootUnorderedAccessView(&mut self, rootparameterindex: u32, bufferlocation: u64);
    fn IASetIndexBuffer(&mut self, pview: *const D3D12_INDEX_BUFFER_VIEW);
    fn IASetVertexBuffers(&mut self, startslot: u32, numviews: u32, pviews: *const D3D12_VERTEX_BUFFER_VIEW);
    fn SOSetTargets(&mut self, startslot: u32, numviews: u32, pviews: *const D3D12_STREAM_OUTPUT_BUFFER_VIEW);
    fn OMSetRenderTargets(&mut self, numrendertargetdescriptors: u32, prendertargetdescriptors: *const D3D12_CPU_DESCRIPTOR_HANDLE, rtssinglehandletodescriptorrange: super::super::Foundation::BOOL, pdepthstencildescriptor: *const D3D12_CPU_DESCRIPTOR_HANDLE);
    fn ClearDepthStencilView(&mut self, depthstencilview: &D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn ClearRenderTargetView(&mut self, rendertargetview: &D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn ClearUnorderedAccessViewUint(&mut self, viewgpuhandleincurrentheap: &D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: &D3D12_CPU_DESCRIPTOR_HANDLE, presource: &::core::option::Option<ID3D12Resource>, values: *const u32, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn ClearUnorderedAccessViewFloat(&mut self, viewgpuhandleincurrentheap: &D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: &D3D12_CPU_DESCRIPTOR_HANDLE, presource: &::core::option::Option<ID3D12Resource>, values: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn DiscardResource(&mut self, presource: &::core::option::Option<ID3D12Resource>, pregion: *const D3D12_DISCARD_REGION);
    fn BeginQuery(&mut self, pqueryheap: &::core::option::Option<ID3D12QueryHeap>, r#type: D3D12_QUERY_TYPE, index: u32);
    fn EndQuery(&mut self, pqueryheap: &::core::option::Option<ID3D12QueryHeap>, r#type: D3D12_QUERY_TYPE, index: u32);
    fn ResolveQueryData(&mut self, pqueryheap: &::core::option::Option<ID3D12QueryHeap>, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: &::core::option::Option<ID3D12Resource>, aligneddestinationbufferoffset: u64);
    fn SetPredication(&mut self, pbuffer: &::core::option::Option<ID3D12Resource>, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP);
    fn SetMarker(&mut self, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32);
    fn BeginEvent(&mut self, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32);
    fn EndEvent(&mut self);
    fn ExecuteIndirect(&mut self, pcommandsignature: &::core::option::Option<ID3D12CommandSignature>, maxcommandcount: u32, pargumentbuffer: &::core::option::Option<ID3D12Resource>, argumentbufferoffset: u64, pcountbuffer: &::core::option::Option<ID3D12Resource>, countbufferoffset: u64);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12GraphicsCommandList_Vtbl {
        unsafe extern "system" fn Close<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Reset<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocator: ::windows::core::RawPtr, pinitialstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute(&pallocator), ::core::mem::transmute(&pinitialstate)).into()
        }
        unsafe extern "system" fn ClearState<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipelinestate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearState(::core::mem::transmute(&ppipelinestate))
        }
        unsafe extern "system" fn DrawInstanced<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawInstanced(::core::mem::transmute_copy(&vertexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startvertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexedInstanced(::core::mem::transmute_copy(&indexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn Dispatch<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Dispatch(::core::mem::transmute_copy(&threadgroupcountx), ::core::mem::transmute_copy(&threadgroupcounty), ::core::mem::transmute_copy(&threadgroupcountz))
        }
        unsafe extern "system" fn CopyBufferRegion<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstoffset: u64, psrcbuffer: ::windows::core::RawPtr, srcoffset: u64, numbytes: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyBufferRegion(::core::mem::transmute(&pdstbuffer), ::core::mem::transmute_copy(&dstoffset), ::core::mem::transmute(&psrcbuffer), ::core::mem::transmute_copy(&srcoffset), ::core::mem::transmute_copy(&numbytes))
        }
        unsafe extern "system" fn CopyTextureRegion<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: *const D3D12_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTextureRegion(::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&psrcbox))
        }
        unsafe extern "system" fn CopyResource<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyResource(::core::mem::transmute(&pdstresource), ::core::mem::transmute(&psrcresource))
        }
        unsafe extern "system" fn CopyTiles<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: ::windows::core::RawPtr, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTiles(::core::mem::transmute(&ptiledresource), ::core::mem::transmute_copy(&ptileregionstartcoordinate), ::core::mem::transmute_copy(&ptileregionsize), ::core::mem::transmute(&pbuffer), ::core::mem::transmute_copy(&bufferstartoffsetinbytes), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ResolveSubresource<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetPrimitiveTopology(::core::mem::transmute_copy(&primitivetopology))
        }
        unsafe extern "system" fn RSSetViewports<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D12_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetViewports(::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSSetScissorRects<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetScissorRects(::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn OMSetBlendFactor<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blendfactor: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetBlendFactor(::core::mem::transmute_copy(&blendfactor))
        }
        unsafe extern "system" fn OMSetStencilRef<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stencilref: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetStencilRef(::core::mem::transmute_copy(&stencilref))
        }
        unsafe extern "system" fn SetPipelineState<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipelinestate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPipelineState(::core::mem::transmute(&ppipelinestate))
        }
        unsafe extern "system" fn ResourceBarrier<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbarriers: u32, pbarriers: *const D3D12_RESOURCE_BARRIER) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResourceBarrier(::core::mem::transmute_copy(&numbarriers), ::core::mem::transmute_copy(&pbarriers))
        }
        unsafe extern "system" fn ExecuteBundle<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandlist: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteBundle(::core::mem::transmute(&pcommandlist))
        }
        unsafe extern "system" fn SetDescriptorHeaps<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numdescriptorheaps: u32, ppdescriptorheaps: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescriptorHeaps(::core::mem::transmute_copy(&numdescriptorheaps), ::core::mem::transmute_copy(&ppdescriptorheaps))
        }
        unsafe extern "system" fn SetComputeRootSignature<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootsignature: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRootSignature(::core::mem::transmute(&prootsignature))
        }
        unsafe extern "system" fn SetGraphicsRootSignature<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootsignature: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRootSignature(::core::mem::transmute(&prootsignature))
        }
        unsafe extern "system" fn SetComputeRootDescriptorTable<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRootDescriptorTable(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&basedescriptor))
        }
        unsafe extern "system" fn SetGraphicsRootDescriptorTable<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRootDescriptorTable(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&basedescriptor))
        }
        unsafe extern "system" fn SetComputeRoot32BitConstant<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRoot32BitConstant(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&srcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetGraphicsRoot32BitConstant<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRoot32BitConstant(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&srcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetComputeRoot32BitConstants<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRoot32BitConstants(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&num32bitvaluestoset), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetGraphicsRoot32BitConstants<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRoot32BitConstants(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&num32bitvaluestoset), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetComputeRootConstantBufferView<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRootConstantBufferView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetGraphicsRootConstantBufferView<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRootConstantBufferView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetComputeRootShaderResourceView<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRootShaderResourceView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetGraphicsRootShaderResourceView<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRootShaderResourceView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetComputeRootUnorderedAccessView<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputeRootUnorderedAccessView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetGraphicsRootUnorderedAccessView<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphicsRootUnorderedAccessView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn IASetIndexBuffer<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pview: *const D3D12_INDEX_BUFFER_VIEW) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetIndexBuffer(::core::mem::transmute_copy(&pview))
        }
        unsafe extern "system" fn IASetVertexBuffers<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_VERTEX_BUFFER_VIEW) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetVertexBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pviews))
        }
        unsafe extern "system" fn SOSetTargets<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_STREAM_OUTPUT_BUFFER_VIEW) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SOSetTargets(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pviews))
        }
        unsafe extern "system" fn OMSetRenderTargets<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrendertargetdescriptors: u32, prendertargetdescriptors: *const D3D12_CPU_DESCRIPTOR_HANDLE, rtssinglehandletodescriptorrange: super::super::Foundation::BOOL, pdepthstencildescriptor: *const D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetRenderTargets(::core::mem::transmute_copy(&numrendertargetdescriptors), ::core::mem::transmute_copy(&prendertargetdescriptors), ::core::mem::transmute_copy(&rtssinglehandletodescriptorrange), ::core::mem::transmute_copy(&pdepthstencildescriptor))
        }
        unsafe extern "system" fn ClearDepthStencilView<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearDepthStencilView(::core::mem::transmute_copy(&depthstencilview), ::core::mem::transmute_copy(&clearflags), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&stencil), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn ClearRenderTargetView<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRenderTargetView(::core::mem::transmute_copy(&rendertargetview), ::core::mem::transmute_copy(&colorrgba), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: ::windows::core::RawPtr, values: *const u32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearUnorderedAccessViewUint(::core::mem::transmute_copy(&viewgpuhandleincurrentheap), ::core::mem::transmute_copy(&viewcpuhandle), ::core::mem::transmute(&presource), ::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: ::windows::core::RawPtr, values: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearUnorderedAccessViewFloat(::core::mem::transmute_copy(&viewgpuhandleincurrentheap), ::core::mem::transmute_copy(&viewcpuhandle), ::core::mem::transmute(&presource), ::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn DiscardResource<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pregion: *const D3D12_DISCARD_REGION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardResource(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pregion))
        }
        unsafe extern "system" fn BeginQuery<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginQuery(::core::mem::transmute(&pqueryheap), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn EndQuery<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndQuery(::core::mem::transmute(&pqueryheap), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn ResolveQueryData<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: ::windows::core::RawPtr, aligneddestinationbufferoffset: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveQueryData(::core::mem::transmute(&pqueryheap), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&numqueries), ::core::mem::transmute(&pdestinationbuffer), ::core::mem::transmute_copy(&aligneddestinationbufferoffset))
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPredication(::core::mem::transmute(&pbuffer), ::core::mem::transmute_copy(&alignedbufferoffset), ::core::mem::transmute_copy(&operation))
        }
        unsafe extern "system" fn SetMarker<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMarker(::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn BeginEvent<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEvent(::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn EndEvent<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEvent()
        }
        unsafe extern "system" fn ExecuteIndirect<Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandsignature: ::windows::core::RawPtr, maxcommandcount: u32, pargumentbuffer: ::windows::core::RawPtr, argumentbufferoffset: u64, pcountbuffer: ::windows::core::RawPtr, countbufferoffset: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteIndirect(::core::mem::transmute(&pcommandsignature), ::core::mem::transmute_copy(&maxcommandcount), ::core::mem::transmute(&pargumentbuffer), ::core::mem::transmute_copy(&argumentbufferoffset), ::core::mem::transmute(&pcountbuffer), ::core::mem::transmute_copy(&countbufferoffset))
        }
        Self {
            base: ID3D12CommandList_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Close: Close::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            ClearState: ClearState::<Impl, IMPL_OFFSET>,
            DrawInstanced: DrawInstanced::<Impl, IMPL_OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Impl, IMPL_OFFSET>,
            Dispatch: Dispatch::<Impl, IMPL_OFFSET>,
            CopyBufferRegion: CopyBufferRegion::<Impl, IMPL_OFFSET>,
            CopyTextureRegion: CopyTextureRegion::<Impl, IMPL_OFFSET>,
            CopyResource: CopyResource::<Impl, IMPL_OFFSET>,
            CopyTiles: CopyTiles::<Impl, IMPL_OFFSET>,
            ResolveSubresource: ResolveSubresource::<Impl, IMPL_OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            RSSetViewports: RSSetViewports::<Impl, IMPL_OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Impl, IMPL_OFFSET>,
            OMSetBlendFactor: OMSetBlendFactor::<Impl, IMPL_OFFSET>,
            OMSetStencilRef: OMSetStencilRef::<Impl, IMPL_OFFSET>,
            SetPipelineState: SetPipelineState::<Impl, IMPL_OFFSET>,
            ResourceBarrier: ResourceBarrier::<Impl, IMPL_OFFSET>,
            ExecuteBundle: ExecuteBundle::<Impl, IMPL_OFFSET>,
            SetDescriptorHeaps: SetDescriptorHeaps::<Impl, IMPL_OFFSET>,
            SetComputeRootSignature: SetComputeRootSignature::<Impl, IMPL_OFFSET>,
            SetGraphicsRootSignature: SetGraphicsRootSignature::<Impl, IMPL_OFFSET>,
            SetComputeRootDescriptorTable: SetComputeRootDescriptorTable::<Impl, IMPL_OFFSET>,
            SetGraphicsRootDescriptorTable: SetGraphicsRootDescriptorTable::<Impl, IMPL_OFFSET>,
            SetComputeRoot32BitConstant: SetComputeRoot32BitConstant::<Impl, IMPL_OFFSET>,
            SetGraphicsRoot32BitConstant: SetGraphicsRoot32BitConstant::<Impl, IMPL_OFFSET>,
            SetComputeRoot32BitConstants: SetComputeRoot32BitConstants::<Impl, IMPL_OFFSET>,
            SetGraphicsRoot32BitConstants: SetGraphicsRoot32BitConstants::<Impl, IMPL_OFFSET>,
            SetComputeRootConstantBufferView: SetComputeRootConstantBufferView::<Impl, IMPL_OFFSET>,
            SetGraphicsRootConstantBufferView: SetGraphicsRootConstantBufferView::<Impl, IMPL_OFFSET>,
            SetComputeRootShaderResourceView: SetComputeRootShaderResourceView::<Impl, IMPL_OFFSET>,
            SetGraphicsRootShaderResourceView: SetGraphicsRootShaderResourceView::<Impl, IMPL_OFFSET>,
            SetComputeRootUnorderedAccessView: SetComputeRootUnorderedAccessView::<Impl, IMPL_OFFSET>,
            SetGraphicsRootUnorderedAccessView: SetGraphicsRootUnorderedAccessView::<Impl, IMPL_OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Impl, IMPL_OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Impl, IMPL_OFFSET>,
            SOSetTargets: SOSetTargets::<Impl, IMPL_OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Impl, IMPL_OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Impl, IMPL_OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat::<Impl, IMPL_OFFSET>,
            DiscardResource: DiscardResource::<Impl, IMPL_OFFSET>,
            BeginQuery: BeginQuery::<Impl, IMPL_OFFSET>,
            EndQuery: EndQuery::<Impl, IMPL_OFFSET>,
            ResolveQueryData: ResolveQueryData::<Impl, IMPL_OFFSET>,
            SetPredication: SetPredication::<Impl, IMPL_OFFSET>,
            SetMarker: SetMarker::<Impl, IMPL_OFFSET>,
            BeginEvent: BeginEvent::<Impl, IMPL_OFFSET>,
            EndEvent: EndEvent::<Impl, IMPL_OFFSET>,
            ExecuteIndirect: ExecuteIndirect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12CommandList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList1_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12CommandList_Impl + ID3D12GraphicsCommandList_Impl {
    fn AtomicCopyBufferUINT(&mut self, pdstbuffer: &::core::option::Option<ID3D12Resource>, dstoffset: u64, psrcbuffer: &::core::option::Option<ID3D12Resource>, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64);
    fn AtomicCopyBufferUINT64(&mut self, pdstbuffer: &::core::option::Option<ID3D12Resource>, dstoffset: u64, psrcbuffer: &::core::option::Option<ID3D12Resource>, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64);
    fn OMSetDepthBounds(&mut self, min: f32, max: f32);
    fn SetSamplePositions(&mut self, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION);
    fn ResolveSubresourceRegion(&mut self, pdstresource: &::core::option::Option<ID3D12Resource>, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: &::core::option::Option<ID3D12Resource>, srcsubresource: u32, psrcrect: *const super::super::Foundation::RECT, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE);
    fn SetViewInstanceMask(&mut self, mask: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12GraphicsCommandList1_Vtbl {
        unsafe extern "system" fn AtomicCopyBufferUINT<Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstoffset: u64, psrcbuffer: ::windows::core::RawPtr, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::windows::core::RawPtr, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AtomicCopyBufferUINT(::core::mem::transmute(&pdstbuffer), ::core::mem::transmute_copy(&dstoffset), ::core::mem::transmute(&psrcbuffer), ::core::mem::transmute_copy(&srcoffset), ::core::mem::transmute_copy(&dependencies), ::core::mem::transmute_copy(&ppdependentresources), ::core::mem::transmute_copy(&pdependentsubresourceranges))
        }
        unsafe extern "system" fn AtomicCopyBufferUINT64<Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstoffset: u64, psrcbuffer: ::windows::core::RawPtr, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::windows::core::RawPtr, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AtomicCopyBufferUINT64(::core::mem::transmute(&pdstbuffer), ::core::mem::transmute_copy(&dstoffset), ::core::mem::transmute(&psrcbuffer), ::core::mem::transmute_copy(&srcoffset), ::core::mem::transmute_copy(&dependencies), ::core::mem::transmute_copy(&ppdependentresources), ::core::mem::transmute_copy(&pdependentsubresourceranges))
        }
        unsafe extern "system" fn OMSetDepthBounds<Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, min: f32, max: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetDepthBounds(::core::mem::transmute_copy(&min), ::core::mem::transmute_copy(&max))
        }
        unsafe extern "system" fn SetSamplePositions<Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSamplePositions(::core::mem::transmute_copy(&numsamplesperpixel), ::core::mem::transmute_copy(&numpixels), ::core::mem::transmute_copy(&psamplepositions))
        }
        unsafe extern "system" fn ResolveSubresourceRegion<Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcrect: *const super::super::Foundation::RECT, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveSubresourceRegion(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcrect), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&resolvemode))
        }
        unsafe extern "system" fn SetViewInstanceMask<Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewInstanceMask(::core::mem::transmute_copy(&mask))
        }
        Self {
            base: ID3D12GraphicsCommandList_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AtomicCopyBufferUINT: AtomicCopyBufferUINT::<Impl, IMPL_OFFSET>,
            AtomicCopyBufferUINT64: AtomicCopyBufferUINT64::<Impl, IMPL_OFFSET>,
            OMSetDepthBounds: OMSetDepthBounds::<Impl, IMPL_OFFSET>,
            SetSamplePositions: SetSamplePositions::<Impl, IMPL_OFFSET>,
            ResolveSubresourceRegion: ResolveSubresourceRegion::<Impl, IMPL_OFFSET>,
            SetViewInstanceMask: SetViewInstanceMask::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList1 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12CommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList2_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12CommandList_Impl + ID3D12GraphicsCommandList_Impl + ID3D12GraphicsCommandList1_Impl {
    fn WriteBufferImmediate(&mut self, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const D3D12_WRITEBUFFERIMMEDIATE_MODE);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12GraphicsCommandList2_Vtbl {
        unsafe extern "system" fn WriteBufferImmediate<Impl: ID3D12GraphicsCommandList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteBufferImmediate(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&pmodes))
        }
        Self {
            base: ID3D12GraphicsCommandList1_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WriteBufferImmediate: WriteBufferImmediate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList2 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12CommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList3_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12CommandList_Impl + ID3D12GraphicsCommandList_Impl + ID3D12GraphicsCommandList1_Impl + ID3D12GraphicsCommandList2_Impl {
    fn SetProtectedResourceSession(&mut self, pprotectedresourcesession: &::core::option::Option<ID3D12ProtectedResourceSession>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12GraphicsCommandList3_Vtbl {
        unsafe extern "system" fn SetProtectedResourceSession<Impl: ID3D12GraphicsCommandList3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotectedresourcesession: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectedResourceSession(::core::mem::transmute(&pprotectedresourcesession))
        }
        Self {
            base: ID3D12GraphicsCommandList2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProtectedResourceSession: SetProtectedResourceSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList3 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12CommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList4_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12CommandList_Impl + ID3D12GraphicsCommandList_Impl + ID3D12GraphicsCommandList1_Impl + ID3D12GraphicsCommandList2_Impl + ID3D12GraphicsCommandList3_Impl {
    fn BeginRenderPass(&mut self, numrendertargets: u32, prendertargets: *const D3D12_RENDER_PASS_RENDER_TARGET_DESC, pdepthstencil: *const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, flags: D3D12_RENDER_PASS_FLAGS);
    fn EndRenderPass(&mut self);
    fn InitializeMetaCommand(&mut self, pmetacommand: &::core::option::Option<ID3D12MetaCommand>, pinitializationparametersdata: *const ::core::ffi::c_void, initializationparametersdatasizeinbytes: usize);
    fn ExecuteMetaCommand(&mut self, pmetacommand: &::core::option::Option<ID3D12MetaCommand>, pexecutionparametersdata: *const ::core::ffi::c_void, executionparametersdatasizeinbytes: usize);
    fn BuildRaytracingAccelerationStructure(&mut self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, numpostbuildinfodescs: u32, ppostbuildinfodescs: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC);
    fn EmitRaytracingAccelerationStructurePostbuildInfo(&mut self, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, numsourceaccelerationstructures: u32, psourceaccelerationstructuredata: *const u64);
    fn CopyRaytracingAccelerationStructure(&mut self, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE);
    fn SetPipelineState1(&mut self, pstateobject: &::core::option::Option<ID3D12StateObject>);
    fn DispatchRays(&mut self, pdesc: *const D3D12_DISPATCH_RAYS_DESC);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12GraphicsCommandList4_Vtbl {
        unsafe extern "system" fn BeginRenderPass<Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrendertargets: u32, prendertargets: *const D3D12_RENDER_PASS_RENDER_TARGET_DESC, pdepthstencil: *const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, flags: D3D12_RENDER_PASS_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginRenderPass(::core::mem::transmute_copy(&numrendertargets), ::core::mem::transmute_copy(&prendertargets), ::core::mem::transmute_copy(&pdepthstencil), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn EndRenderPass<Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndRenderPass()
        }
        unsafe extern "system" fn InitializeMetaCommand<Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetacommand: ::windows::core::RawPtr, pinitializationparametersdata: *const ::core::ffi::c_void, initializationparametersdatasizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeMetaCommand(::core::mem::transmute(&pmetacommand), ::core::mem::transmute_copy(&pinitializationparametersdata), ::core::mem::transmute_copy(&initializationparametersdatasizeinbytes))
        }
        unsafe extern "system" fn ExecuteMetaCommand<Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetacommand: ::windows::core::RawPtr, pexecutionparametersdata: *const ::core::ffi::c_void, executionparametersdatasizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteMetaCommand(::core::mem::transmute(&pmetacommand), ::core::mem::transmute_copy(&pexecutionparametersdata), ::core::mem::transmute_copy(&executionparametersdatasizeinbytes))
        }
        unsafe extern "system" fn BuildRaytracingAccelerationStructure<Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, numpostbuildinfodescs: u32, ppostbuildinfodescs: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BuildRaytracingAccelerationStructure(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&numpostbuildinfodescs), ::core::mem::transmute_copy(&ppostbuildinfodescs))
        }
        unsafe extern "system" fn EmitRaytracingAccelerationStructurePostbuildInfo<Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, numsourceaccelerationstructures: u32, psourceaccelerationstructuredata: *const u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EmitRaytracingAccelerationStructurePostbuildInfo(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&numsourceaccelerationstructures), ::core::mem::transmute_copy(&psourceaccelerationstructuredata))
        }
        unsafe extern "system" fn CopyRaytracingAccelerationStructure<Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyRaytracingAccelerationStructure(::core::mem::transmute_copy(&destaccelerationstructuredata), ::core::mem::transmute_copy(&sourceaccelerationstructuredata), ::core::mem::transmute_copy(&mode))
        }
        unsafe extern "system" fn SetPipelineState1<Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateobject: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPipelineState1(::core::mem::transmute(&pstateobject))
        }
        unsafe extern "system" fn DispatchRays<Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_DISPATCH_RAYS_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DispatchRays(::core::mem::transmute_copy(&pdesc))
        }
        Self {
            base: ID3D12GraphicsCommandList3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginRenderPass: BeginRenderPass::<Impl, IMPL_OFFSET>,
            EndRenderPass: EndRenderPass::<Impl, IMPL_OFFSET>,
            InitializeMetaCommand: InitializeMetaCommand::<Impl, IMPL_OFFSET>,
            ExecuteMetaCommand: ExecuteMetaCommand::<Impl, IMPL_OFFSET>,
            BuildRaytracingAccelerationStructure: BuildRaytracingAccelerationStructure::<Impl, IMPL_OFFSET>,
            EmitRaytracingAccelerationStructurePostbuildInfo: EmitRaytracingAccelerationStructurePostbuildInfo::<Impl, IMPL_OFFSET>,
            CopyRaytracingAccelerationStructure: CopyRaytracingAccelerationStructure::<Impl, IMPL_OFFSET>,
            SetPipelineState1: SetPipelineState1::<Impl, IMPL_OFFSET>,
            DispatchRays: DispatchRays::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList4 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12CommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList2 as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList5_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12CommandList_Impl + ID3D12GraphicsCommandList_Impl + ID3D12GraphicsCommandList1_Impl + ID3D12GraphicsCommandList2_Impl + ID3D12GraphicsCommandList3_Impl + ID3D12GraphicsCommandList4_Impl {
    fn RSSetShadingRate(&mut self, baseshadingrate: D3D12_SHADING_RATE, combiners: *const D3D12_SHADING_RATE_COMBINER);
    fn RSSetShadingRateImage(&mut self, shadingrateimage: &::core::option::Option<ID3D12Resource>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12GraphicsCommandList5_Vtbl {
        unsafe extern "system" fn RSSetShadingRate<Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseshadingrate: D3D12_SHADING_RATE, combiners: *const D3D12_SHADING_RATE_COMBINER) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetShadingRate(::core::mem::transmute_copy(&baseshadingrate), ::core::mem::transmute_copy(&combiners))
        }
        unsafe extern "system" fn RSSetShadingRateImage<Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shadingrateimage: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetShadingRateImage(::core::mem::transmute(&shadingrateimage))
        }
        Self {
            base: ID3D12GraphicsCommandList4_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RSSetShadingRate: RSSetShadingRate::<Impl, IMPL_OFFSET>,
            RSSetShadingRateImage: RSSetShadingRateImage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList5 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12CommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList2 as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList3 as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList6_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12CommandList_Impl + ID3D12GraphicsCommandList_Impl + ID3D12GraphicsCommandList1_Impl + ID3D12GraphicsCommandList2_Impl + ID3D12GraphicsCommandList3_Impl + ID3D12GraphicsCommandList4_Impl + ID3D12GraphicsCommandList5_Impl {
    fn DispatchMesh(&mut self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12GraphicsCommandList6_Vtbl {
        unsafe extern "system" fn DispatchMesh<Impl: ID3D12GraphicsCommandList6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DispatchMesh(::core::mem::transmute_copy(&threadgroupcountx), ::core::mem::transmute_copy(&threadgroupcounty), ::core::mem::transmute_copy(&threadgroupcountz))
        }
        Self { base: ID3D12GraphicsCommandList5_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), DispatchMesh: DispatchMesh::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList6 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12CommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList2 as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList3 as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList4 as ::windows::core::Interface>::IID || iid == &<ID3D12GraphicsCommandList5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Heap_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {
    fn GetDesc(&mut self) -> D3D12_HEAP_DESC;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Heap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Heap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Heap_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12Heap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_HEAP_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Heap as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Heap1_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl + ID3D12Heap_Impl {
    fn GetProtectedResourceSession(&mut self, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Heap1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Heap1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Heap1_Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Impl: ID3D12Heap1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProtectedResourceSession(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppprotectedsession)).into()
        }
        Self {
            base: ID3D12Heap_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetProtectedResourceSession: GetProtectedResourceSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Heap1 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID || iid == &<ID3D12Heap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12InfoQueue_Impl: Sized {
    fn SetMessageCountLimit(&mut self, messagecountlimit: u64) -> ::windows::core::Result<()>;
    fn ClearStoredMessages(&mut self);
    fn GetMessage(&mut self, messageindex: u64, pmessage: *mut D3D12_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(&mut self) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&mut self) -> u64;
    fn GetNumStoredMessages(&mut self) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(&mut self) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&mut self) -> u64;
    fn GetMessageCountLimit(&mut self) -> u64;
    fn AddStorageFilterEntries(&mut self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetStorageFilter(&mut self, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearStorageFilter(&mut self);
    fn PushEmptyStorageFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushCopyOfStorageFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushStorageFilter(&mut self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopStorageFilter(&mut self);
    fn GetStorageFilterStackSize(&mut self) -> u32;
    fn AddRetrievalFilterEntries(&mut self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetRetrievalFilter(&mut self, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearRetrievalFilter(&mut self);
    fn PushEmptyRetrievalFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushCopyOfRetrievalFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushRetrievalFilter(&mut self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopRetrievalFilter(&mut self);
    fn GetRetrievalFilterStackSize(&mut self) -> u32;
    fn AddMessage(&mut self, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn AddApplicationMessage(&mut self, severity: D3D12_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn SetBreakOnCategory(&mut self, category: D3D12_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnSeverity(&mut self, severity: D3D12_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnID(&mut self, id: D3D12_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBreakOnCategory(&mut self, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&mut self, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&mut self, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&mut self, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12InfoQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12InfoQueue_Vtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageCountLimit(::core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStoredMessages()
        }
        unsafe extern "system" fn GetMessage<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D12_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessage(::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesAllowedByStorageFilter()
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesDeniedByStorageFilter()
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumStoredMessages()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumStoredMessagesAllowedByRetrievalFilter()
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesDiscardedByMessageCountLimit()
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessageCountLimit()
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStorageFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStorageFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStorageFilter()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushEmptyStorageFilter().into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushCopyOfStorageFilter().into()
        }
        unsafe extern "system" fn PushStorageFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushStorageFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopStorageFilter()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStorageFilterStackSize()
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRetrievalFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRetrievalFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRetrievalFilter()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushEmptyRetrievalFilter().into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushCopyOfRetrievalFilter().into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushRetrievalFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopRetrievalFilter()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRetrievalFilterStackSize()
        }
        unsafe extern "system" fn AddMessage<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMessage(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddApplicationMessage(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnCategory(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnSeverity(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D12_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnID(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnCategory(::core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnSeverity(::core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnID(::core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuteDebugOutput(::core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMuteDebugOutput()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Impl, IMPL_OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Impl, IMPL_OFFSET>,
            GetMessage: GetMessage::<Impl, IMPL_OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Impl, IMPL_OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Impl, IMPL_OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Impl, IMPL_OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Impl, IMPL_OFFSET>,
            GetStorageFilter: GetStorageFilter::<Impl, IMPL_OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Impl, IMPL_OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Impl, IMPL_OFFSET>,
            PushStorageFilter: PushStorageFilter::<Impl, IMPL_OFFSET>,
            PopStorageFilter: PopStorageFilter::<Impl, IMPL_OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Impl, IMPL_OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Impl, IMPL_OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Impl, IMPL_OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Impl, IMPL_OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Impl, IMPL_OFFSET>,
            AddMessage: AddMessage::<Impl, IMPL_OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Impl, IMPL_OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Impl, IMPL_OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            SetBreakOnID: SetBreakOnID::<Impl, IMPL_OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Impl, IMPL_OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            GetBreakOnID: GetBreakOnID::<Impl, IMPL_OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Impl, IMPL_OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12InfoQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12InfoQueue1_Impl: Sized + ID3D12InfoQueue_Impl {
    fn RegisterMessageCallback(&mut self, callbackfunc: &D3D12MessageFunc, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *const ::core::ffi::c_void, pcallbackcookie: *mut u32) -> ::windows::core::Result<()>;
    fn UnregisterMessageCallback(&mut self, callbackcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12InfoQueue1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12InfoQueue1_Vtbl {
        unsafe extern "system" fn RegisterMessageCallback<Impl: ID3D12InfoQueue1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackfunc: ::windows::core::RawPtr, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *const ::core::ffi::c_void, pcallbackcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterMessageCallback(::core::mem::transmute_copy(&callbackfunc), ::core::mem::transmute_copy(&callbackfilterflags), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&pcallbackcookie)).into()
        }
        unsafe extern "system" fn UnregisterMessageCallback<Impl: ID3D12InfoQueue1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterMessageCallback(::core::mem::transmute_copy(&callbackcookie)).into()
        }
        Self {
            base: ID3D12InfoQueue_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegisterMessageCallback: RegisterMessageCallback::<Impl, IMPL_OFFSET>,
            UnregisterMessageCallback: UnregisterMessageCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12InfoQueue1 as ::windows::core::Interface>::IID || iid == &<ID3D12InfoQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12LibraryReflection_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D12_LIBRARY_DESC>;
    fn GetFunctionByIndex(&mut self, functionindex: i32) -> ::core::option::Option<ID3D12FunctionReflection>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12LibraryReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LibraryReflection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12LibraryReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_LIBRARY_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionByIndex<Impl: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionindex: i32) -> ::core::option::Option<ID3D12FunctionReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFunctionByIndex(::core::mem::transmute_copy(&functionindex))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetFunctionByIndex: GetFunctionByIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12LibraryReflection as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12LifetimeOwner_Impl: Sized {
    fn LifetimeStateUpdated(&mut self, newstate: D3D12_LIFETIME_STATE);
}
impl ID3D12LifetimeOwner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LifetimeOwner_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12LifetimeOwner_Vtbl {
        unsafe extern "system" fn LifetimeStateUpdated<Impl: ID3D12LifetimeOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: D3D12_LIFETIME_STATE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LifetimeStateUpdated(::core::mem::transmute_copy(&newstate))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), LifetimeStateUpdated: LifetimeStateUpdated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12LifetimeOwner as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12LifetimeTracker_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl {
    fn DestroyOwnedObject(&mut self, pobject: &::core::option::Option<ID3D12DeviceChild>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12LifetimeTracker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LifetimeTracker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12LifetimeTracker_Vtbl {
        unsafe extern "system" fn DestroyOwnedObject<Impl: ID3D12LifetimeTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestroyOwnedObject(::core::mem::transmute(&pobject)).into()
        }
        Self { base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), DestroyOwnedObject: DestroyOwnedObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12LifetimeTracker as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12MetaCommand_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {
    fn GetRequiredParameterResourceSize(&mut self, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12MetaCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12MetaCommand_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12MetaCommand_Vtbl {
        unsafe extern "system" fn GetRequiredParameterResourceSize<Impl: ID3D12MetaCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRequiredParameterResourceSize(::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&parameterindex))
        }
        Self {
            base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetRequiredParameterResourceSize: GetRequiredParameterResourceSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12MetaCommand as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Object_Impl: Sized {
    fn GetPrivateData(&mut self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&mut self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&mut self, guid: *const ::windows::core::GUID, pdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetName(&mut self, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Object_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Object_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Object_Vtbl {
        unsafe extern "system" fn GetPrivateData<Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        unsafe extern "system" fn SetName<Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Object as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Pageable_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Pageable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Pageable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Pageable_Vtbl {
        Self { base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Pageable as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12PipelineLibrary_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl {
    fn StorePipeline(&mut self, pname: super::super::Foundation::PWSTR, ppipeline: &::core::option::Option<ID3D12PipelineState>) -> ::windows::core::Result<()>;
    fn LoadGraphicsPipeline(&mut self, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn LoadComputePipeline(&mut self, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetSerializedSize(&mut self) -> usize;
    fn Serialize(&mut self, pdata: *mut ::core::ffi::c_void, datasizeinbytes: usize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12PipelineLibrary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12PipelineLibrary_Vtbl {
        unsafe extern "system" fn StorePipeline<Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, ppipeline: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StorePipeline(::core::mem::transmute_copy(&pname), ::core::mem::transmute(&ppipeline)).into()
        }
        unsafe extern "system" fn LoadGraphicsPipeline<Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadGraphicsPipeline(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn LoadComputePipeline<Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadComputePipeline(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn GetSerializedSize<Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSerializedSize()
        }
        unsafe extern "system" fn Serialize<Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasizeinbytes: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasizeinbytes)).into()
        }
        Self {
            base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StorePipeline: StorePipeline::<Impl, IMPL_OFFSET>,
            LoadGraphicsPipeline: LoadGraphicsPipeline::<Impl, IMPL_OFFSET>,
            LoadComputePipeline: LoadComputePipeline::<Impl, IMPL_OFFSET>,
            GetSerializedSize: GetSerializedSize::<Impl, IMPL_OFFSET>,
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12PipelineLibrary as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12PipelineLibrary1_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12PipelineLibrary_Impl {
    fn LoadPipeline(&mut self, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12PipelineLibrary1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12PipelineLibrary1_Vtbl {
        unsafe extern "system" fn LoadPipeline<Impl: ID3D12PipelineLibrary1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadPipeline(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        Self { base: ID3D12PipelineLibrary_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), LoadPipeline: LoadPipeline::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12PipelineLibrary1 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12PipelineLibrary as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D12PipelineState_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {
    fn GetCachedBlob(&mut self) -> ::windows::core::Result<super::Direct3D::ID3DBlob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D12PipelineState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12PipelineState_Vtbl {
        unsafe extern "system" fn GetCachedBlob<Impl: ID3D12PipelineState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedBlob() {
                ::core::result::Result::Ok(ok__) => {
                    *ppblob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetCachedBlob: GetCachedBlob::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12PipelineState as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12ProtectedResourceSession_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12ProtectedSession_Impl {
    fn GetDesc(&mut self) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12ProtectedResourceSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedResourceSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12ProtectedResourceSession_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ProtectedResourceSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        Self { base: ID3D12ProtectedSession_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12ProtectedResourceSession as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12ProtectedSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12ProtectedResourceSession1_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12ProtectedSession_Impl + ID3D12ProtectedResourceSession_Impl {
    fn GetDesc1(&mut self) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC1;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12ProtectedResourceSession1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedResourceSession1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12ProtectedResourceSession1_Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D12ProtectedResourceSession1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetDesc1()
        }
        Self { base: ID3D12ProtectedResourceSession_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12ProtectedResourceSession1 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12ProtectedSession as ::windows::core::Interface>::IID || iid == &<ID3D12ProtectedResourceSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12ProtectedSession_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl {
    fn GetStatusFence(&mut self, riid: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetSessionStatus(&mut self) -> D3D12_PROTECTED_SESSION_STATUS;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12ProtectedSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12ProtectedSession_Vtbl {
        unsafe extern "system" fn GetStatusFence<Impl: ID3D12ProtectedSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatusFence(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppfence)).into()
        }
        unsafe extern "system" fn GetSessionStatus<Impl: ID3D12ProtectedSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_PROTECTED_SESSION_STATUS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSessionStatus()
        }
        Self {
            base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStatusFence: GetStatusFence::<Impl, IMPL_OFFSET>,
            GetSessionStatus: GetSessionStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12ProtectedSession as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12QueryHeap_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12QueryHeap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12QueryHeap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12QueryHeap_Vtbl {
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12QueryHeap as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12Resource_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {
    fn Map(&mut self, subresource: u32, preadrange: *const D3D12_RANGE, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Unmap(&mut self, subresource: u32, pwrittenrange: *const D3D12_RANGE);
    fn GetDesc(&mut self) -> D3D12_RESOURCE_DESC;
    fn GetGPUVirtualAddress(&mut self) -> u64;
    fn WriteToSubresource(&mut self, dstsubresource: u32, pdstbox: *const D3D12_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> ::windows::core::Result<()>;
    fn ReadFromSubresource(&mut self, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: *const D3D12_BOX) -> ::windows::core::Result<()>;
    fn GetHeapProperties(&mut self, pheapproperties: *mut D3D12_HEAP_PROPERTIES, pheapflags: *mut D3D12_HEAP_FLAGS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12Resource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Resource_Vtbl {
        unsafe extern "system" fn Map<Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, preadrange: *const D3D12_RANGE, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Map(::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&preadrange), ::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn Unmap<Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, pwrittenrange: *const D3D12_RANGE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap(::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&pwrittenrange))
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        unsafe extern "system" fn GetGPUVirtualAddress<Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGPUVirtualAddress()
        }
        unsafe extern "system" fn WriteToSubresource<Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D12_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToSubresource(::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch)).into()
        }
        unsafe extern "system" fn ReadFromSubresource<Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: *const D3D12_BOX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadFromSubresource(::core::mem::transmute_copy(&pdstdata), ::core::mem::transmute_copy(&dstrowpitch), ::core::mem::transmute_copy(&dstdepthpitch), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox)).into()
        }
        unsafe extern "system" fn GetHeapProperties<Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *mut D3D12_HEAP_PROPERTIES, pheapflags: *mut D3D12_HEAP_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHeapProperties(::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&pheapflags)).into()
        }
        Self {
            base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Map: Map::<Impl, IMPL_OFFSET>,
            Unmap: Unmap::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetGPUVirtualAddress: GetGPUVirtualAddress::<Impl, IMPL_OFFSET>,
            WriteToSubresource: WriteToSubresource::<Impl, IMPL_OFFSET>,
            ReadFromSubresource: ReadFromSubresource::<Impl, IMPL_OFFSET>,
            GetHeapProperties: GetHeapProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Resource as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12Resource1_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl + ID3D12Resource_Impl {
    fn GetProtectedResourceSession(&mut self, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12Resource1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Resource1_Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Impl: ID3D12Resource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProtectedResourceSession(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppprotectedsession)).into()
        }
        Self {
            base: ID3D12Resource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetProtectedResourceSession: GetProtectedResourceSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Resource1 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID || iid == &<ID3D12Resource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12Resource2_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl + ID3D12Resource_Impl + ID3D12Resource1_Impl {
    fn GetDesc1(&mut self) -> D3D12_RESOURCE_DESC1;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12Resource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Resource2_Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D12Resource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetDesc1()
        }
        Self { base: ID3D12Resource1_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Resource2 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID || iid == &<ID3D12Resource as ::windows::core::Interface>::IID || iid == &<ID3D12Resource1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12RootSignature_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12RootSignature_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12RootSignature_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12RootSignature_Vtbl {
        Self { base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12RootSignature as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12RootSignatureDeserializer_Impl: Sized {
    fn GetRootSignatureDesc(&mut self) -> *mut D3D12_ROOT_SIGNATURE_DESC;
}
impl ID3D12RootSignatureDeserializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12RootSignatureDeserializer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12RootSignatureDeserializer_Vtbl {
        unsafe extern "system" fn GetRootSignatureDesc<Impl: ID3D12RootSignatureDeserializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut D3D12_ROOT_SIGNATURE_DESC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRootSignatureDesc()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetRootSignatureDesc: GetRootSignatureDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12RootSignatureDeserializer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12SDKConfiguration_Impl: Sized {
    fn SetSDKVersion(&mut self, sdkversion: u32, sdkpath: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12SDKConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SDKConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12SDKConfiguration_Vtbl {
        unsafe extern "system" fn SetSDKVersion<Impl: ID3D12SDKConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sdkversion: u32, sdkpath: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSDKVersion(::core::mem::transmute_copy(&sdkversion), ::core::mem::transmute_copy(&sdkpath)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetSDKVersion: SetSDKVersion::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12SDKConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12ShaderCacheSession_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl {
    fn FindValue(&mut self, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *mut ::core::ffi::c_void, pvaluesize: *mut u32) -> ::windows::core::Result<()>;
    fn StoreValue(&mut self, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::Result<()>;
    fn SetDeleteOnDestroy(&mut self);
    fn GetDesc(&mut self) -> D3D12_SHADER_CACHE_SESSION_DESC;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12ShaderCacheSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderCacheSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12ShaderCacheSession_Vtbl {
        unsafe extern "system" fn FindValue<Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *mut ::core::ffi::c_void, pvaluesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindValue(::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&keysize), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pvaluesize)).into()
        }
        unsafe extern "system" fn StoreValue<Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StoreValue(::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&keysize), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&valuesize)).into()
        }
        unsafe extern "system" fn SetDeleteOnDestroy<Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeleteOnDestroy()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_SHADER_CACHE_SESSION_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        Self {
            base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FindValue: FindValue::<Impl, IMPL_OFFSET>,
            StoreValue: StoreValue::<Impl, IMPL_OFFSET>,
            SetDeleteOnDestroy: SetDeleteOnDestroy::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12ShaderCacheSession as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D12ShaderReflection_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D12_SHADER_DESC>;
    fn GetConstantBufferByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&mut self, resourceindex: u32) -> ::windows::core::Result<D3D12_SHADER_INPUT_BIND_DESC>;
    fn GetInputParameterDesc(&mut self, parameterindex: u32) -> ::windows::core::Result<D3D12_SIGNATURE_PARAMETER_DESC>;
    fn GetOutputParameterDesc(&mut self, parameterindex: u32) -> ::windows::core::Result<D3D12_SIGNATURE_PARAMETER_DESC>;
    fn GetPatchConstantParameterDesc(&mut self, parameterindex: u32) -> ::windows::core::Result<D3D12_SIGNATURE_PARAMETER_DESC>;
    fn GetVariableByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&mut self, name: super::super::Foundation::PSTR) -> ::windows::core::Result<D3D12_SHADER_INPUT_BIND_DESC>;
    fn GetMovInstructionCount(&mut self) -> u32;
    fn GetMovcInstructionCount(&mut self) -> u32;
    fn GetConversionInstructionCount(&mut self) -> u32;
    fn GetBitwiseInstructionCount(&mut self) -> u32;
    fn GetGSInputPrimitive(&mut self) -> super::Direct3D::D3D_PRIMITIVE;
    fn IsSampleFrequencyShader(&mut self) -> super::super::Foundation::BOOL;
    fn GetNumInterfaceSlots(&mut self) -> u32;
    fn GetMinFeatureLevel(&mut self) -> ::windows::core::Result<super::Direct3D::D3D_FEATURE_LEVEL>;
    fn GetThreadGroupSize(&mut self, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32;
    fn GetRequiresFlags(&mut self) -> u64;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D12ShaderReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12ShaderReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatchConstantParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDescByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMovInstructionCount()
        }
        unsafe extern "system" fn GetMovcInstructionCount<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMovcInstructionCount()
        }
        unsafe extern "system" fn GetConversionInstructionCount<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConversionInstructionCount()
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBitwiseInstructionCount()
        }
        unsafe extern "system" fn GetGSInputPrimitive<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGSInputPrimitive()
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSampleFrequencyShader()
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumInterfaceSlots()
        }
        unsafe extern "system" fn GetMinFeatureLevel<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinFeatureLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadGroupSize<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetThreadGroupSize(::core::mem::transmute_copy(&psizex), ::core::mem::transmute_copy(&psizey), ::core::mem::transmute_copy(&psizez))
        }
        unsafe extern "system" fn GetRequiresFlags<Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRequiresFlags()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl, IMPL_OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Impl, IMPL_OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Impl, IMPL_OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Impl, IMPL_OFFSET>,
            GetPatchConstantParameterDesc: GetPatchConstantParameterDesc::<Impl, IMPL_OFFSET>,
            GetVariableByName: GetVariableByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Impl, IMPL_OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Impl, IMPL_OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Impl, IMPL_OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Impl, IMPL_OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Impl, IMPL_OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Impl, IMPL_OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Impl, IMPL_OFFSET>,
            GetNumInterfaceSlots: GetNumInterfaceSlots::<Impl, IMPL_OFFSET>,
            GetMinFeatureLevel: GetMinFeatureLevel::<Impl, IMPL_OFFSET>,
            GetThreadGroupSize: GetThreadGroupSize::<Impl, IMPL_OFFSET>,
            GetRequiresFlags: GetRequiresFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12ShaderReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D12ShaderReflectionConstantBuffer_Impl: Sized {
    fn GetDesc(&mut self, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> ::windows::core::Result<()>;
    fn GetVariableByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionVariable>;
    fn GetVariableByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D12ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionConstantBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D12ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D12ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        Self {
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Impl, IMPL_OFFSET>,
            GetVariableByName: GetVariableByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12ShaderReflectionConstantBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D12ShaderReflectionType_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D12_SHADER_TYPE_DESC>;
    fn GetMemberTypeByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn GetMemberTypeByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn GetMemberTypeName(&mut self, index: u32) -> super::super::Foundation::PSTR;
    fn IsEqual(&mut self, ptype: &::core::option::Option<ID3D12ShaderReflectionType>) -> ::windows::core::Result<()>;
    fn GetSubType(&mut self) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn GetBaseClass(&mut self) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn GetNumInterfaces(&mut self) -> u32;
    fn GetInterfaceByIndex(&mut self, uindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn IsOfType(&mut self, ptype: &::core::option::Option<ID3D12ShaderReflectionType>) -> ::windows::core::Result<()>;
    fn ImplementsInterface(&mut self, pbase: &::core::option::Option<ID3D12ShaderReflectionType>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D12ShaderReflectionType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeName(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn IsEqual<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsEqual(::core::mem::transmute(&ptype)).into()
        }
        unsafe extern "system" fn GetSubType<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSubType()
        }
        unsafe extern "system" fn GetBaseClass<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBaseClass()
        }
        unsafe extern "system" fn GetNumInterfaces<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumInterfaces()
        }
        unsafe extern "system" fn GetInterfaceByIndex<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInterfaceByIndex(::core::mem::transmute_copy(&uindex))
        }
        unsafe extern "system" fn IsOfType<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsOfType(::core::mem::transmute(&ptype)).into()
        }
        unsafe extern "system" fn ImplementsInterface<Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbase: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImplementsInterface(::core::mem::transmute(&pbase)).into()
        }
        Self {
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl, IMPL_OFFSET>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl, IMPL_OFFSET>,
            GetMemberTypeName: GetMemberTypeName::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            GetSubType: GetSubType::<Impl, IMPL_OFFSET>,
            GetBaseClass: GetBaseClass::<Impl, IMPL_OFFSET>,
            GetNumInterfaces: GetNumInterfaces::<Impl, IMPL_OFFSET>,
            GetInterfaceByIndex: GetInterfaceByIndex::<Impl, IMPL_OFFSET>,
            IsOfType: IsOfType::<Impl, IMPL_OFFSET>,
            ImplementsInterface: ImplementsInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12ShaderReflectionType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12ShaderReflectionVariable_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D12_SHADER_VARIABLE_DESC>;
    fn GetType(&mut self) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn GetBuffer(&mut self) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetInterfaceSlot(&mut self, uarrayindex: u32) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12ShaderReflectionVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionVariable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ID3D12ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType()
        }
        unsafe extern "system" fn GetBuffer<Impl: ID3D12ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuffer()
        }
        unsafe extern "system" fn GetInterfaceSlot<Impl: ID3D12ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInterfaceSlot(::core::mem::transmute_copy(&uarrayindex))
        }
        Self {
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            GetInterfaceSlot: GetInterfaceSlot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12ShaderReflectionVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12SharingContract_Impl: Sized {
    fn Present(&mut self, presource: &::core::option::Option<ID3D12Resource>, subresource: u32, window: super::super::Foundation::HWND);
    fn SharedFenceSignal(&mut self, pfence: &::core::option::Option<ID3D12Fence>, fencevalue: u64);
    fn BeginCapturableWork(&mut self, guid: *const ::windows::core::GUID);
    fn EndCapturableWork(&mut self, guid: *const ::windows::core::GUID);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12SharingContract_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SharingContract_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12SharingContract_Vtbl {
        unsafe extern "system" fn Present<Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, window: super::super::Foundation::HWND) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Present(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&window))
        }
        unsafe extern "system" fn SharedFenceSignal<Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, fencevalue: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SharedFenceSignal(::core::mem::transmute(&pfence), ::core::mem::transmute_copy(&fencevalue))
        }
        unsafe extern "system" fn BeginCapturableWork<Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginCapturableWork(::core::mem::transmute_copy(&guid))
        }
        unsafe extern "system" fn EndCapturableWork<Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndCapturableWork(::core::mem::transmute_copy(&guid))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Present: Present::<Impl, IMPL_OFFSET>,
            SharedFenceSignal: SharedFenceSignal::<Impl, IMPL_OFFSET>,
            BeginCapturableWork: BeginCapturableWork::<Impl, IMPL_OFFSET>,
            EndCapturableWork: EndCapturableWork::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12SharingContract as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12StateObject_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12StateObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12StateObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12StateObject_Vtbl {
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12StateObject as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12StateObjectProperties_Impl: Sized {
    fn GetShaderIdentifier(&mut self, pexportname: super::super::Foundation::PWSTR) -> *mut ::core::ffi::c_void;
    fn GetShaderStackSize(&mut self, pexportname: super::super::Foundation::PWSTR) -> u64;
    fn GetPipelineStackSize(&mut self) -> u64;
    fn SetPipelineStackSize(&mut self, pipelinestacksizeinbytes: u64);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12StateObjectProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12StateObjectProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12StateObjectProperties_Vtbl {
        unsafe extern "system" fn GetShaderIdentifier<Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexportname: super::super::Foundation::PWSTR) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetShaderIdentifier(::core::mem::transmute_copy(&pexportname))
        }
        unsafe extern "system" fn GetShaderStackSize<Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexportname: super::super::Foundation::PWSTR) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetShaderStackSize(::core::mem::transmute_copy(&pexportname))
        }
        unsafe extern "system" fn GetPipelineStackSize<Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPipelineStackSize()
        }
        unsafe extern "system" fn SetPipelineStackSize<Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipelinestacksizeinbytes: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPipelineStackSize(::core::mem::transmute_copy(&pipelinestacksizeinbytes))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetShaderIdentifier: GetShaderIdentifier::<Impl, IMPL_OFFSET>,
            GetShaderStackSize: GetShaderStackSize::<Impl, IMPL_OFFSET>,
            GetPipelineStackSize: GetPipelineStackSize::<Impl, IMPL_OFFSET>,
            SetPipelineStackSize: SetPipelineStackSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12StateObjectProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12SwapChainAssistant_Impl: Sized {
    fn GetLUID(&mut self) -> super::super::Foundation::LUID;
    fn GetSwapChainObject(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetCurrentResourceAndCommandQueue(&mut self, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void, riidqueue: *const ::windows::core::GUID, ppvqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn InsertImplicitSync(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12SwapChainAssistant_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SwapChainAssistant_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12SwapChainAssistant_Vtbl {
        unsafe extern "system" fn GetLUID<Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            *result__ = (*this).GetLUID()
        }
        unsafe extern "system" fn GetSwapChainObject<Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSwapChainObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetCurrentResourceAndCommandQueue<Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void, riidqueue: *const ::windows::core::GUID, ppvqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentResourceAndCommandQueue(::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource), ::core::mem::transmute_copy(&riidqueue), ::core::mem::transmute_copy(&ppvqueue)).into()
        }
        unsafe extern "system" fn InsertImplicitSync<Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertImplicitSync().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLUID: GetLUID::<Impl, IMPL_OFFSET>,
            GetSwapChainObject: GetSwapChainObject::<Impl, IMPL_OFFSET>,
            GetCurrentResourceAndCommandQueue: GetCurrentResourceAndCommandQueue::<Impl, IMPL_OFFSET>,
            InsertImplicitSync: InsertImplicitSync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12SwapChainAssistant as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Tools_Impl: Sized {
    fn EnableShaderInstrumentation(&mut self, benable: super::super::Foundation::BOOL);
    fn ShaderInstrumentationEnabled(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12Tools_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Tools_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12Tools_Vtbl {
        unsafe extern "system" fn EnableShaderInstrumentation<Impl: ID3D12Tools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableShaderInstrumentation(::core::mem::transmute_copy(&benable))
        }
        unsafe extern "system" fn ShaderInstrumentationEnabled<Impl: ID3D12Tools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShaderInstrumentationEnabled()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnableShaderInstrumentation: EnableShaderInstrumentation::<Impl, IMPL_OFFSET>,
            ShaderInstrumentationEnabled: ShaderInstrumentationEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Tools as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12VersionedRootSignatureDeserializer_Impl: Sized {
    fn GetRootSignatureDescAtVersion(&mut self, converttoversion: D3D_ROOT_SIGNATURE_VERSION) -> ::windows::core::Result<*mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC>;
    fn GetUnconvertedRootSignatureDesc(&mut self) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC;
}
impl ID3D12VersionedRootSignatureDeserializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D12VersionedRootSignatureDeserializer_Vtbl {
        unsafe extern "system" fn GetRootSignatureDescAtVersion<Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, converttoversion: D3D_ROOT_SIGNATURE_VERSION, ppdesc: *mut *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRootSignatureDescAtVersion(::core::mem::transmute_copy(&converttoversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnconvertedRootSignatureDesc<Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUnconvertedRootSignatureDesc()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRootSignatureDescAtVersion: GetRootSignatureDescAtVersion::<Impl, IMPL_OFFSET>,
            GetUnconvertedRootSignatureDesc: GetUnconvertedRootSignatureDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VersionedRootSignatureDeserializer as ::windows::core::Interface>::IID
    }
}
