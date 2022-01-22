#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12CommandAllocator_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl + ID3D12Pageable_Impl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12CommandAllocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandAllocator_Impl, const OFFSET: isize>() -> ID3D12CommandAllocator_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandAllocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(), Reset: Reset::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandList_Impl, const OFFSET: isize>() -> ID3D12CommandList_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_COMMAND_LIST_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetType()
        }
        Self { base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetType: GetType::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>() -> ID3D12CommandQueue_Vtbl {
        unsafe extern "system" fn UpdateTileMappings<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, numresourceregions: u32, presourceregionstartcoordinates: *const D3D12_TILED_RESOURCE_COORDINATE, presourceregionsizes: *const D3D12_TILE_REGION_SIZE, pheap: ::windows::core::RawPtr, numranges: u32, prangeflags: *const D3D12_TILE_RANGE_FLAGS, pheaprangestartoffsets: *const u32, prangetilecounts: *const u32, flags: D3D12_TILE_MAPPING_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateTileMappings(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&numresourceregions), ::core::mem::transmute_copy(&presourceregionstartcoordinates), ::core::mem::transmute_copy(&presourceregionsizes), ::core::mem::transmute(&pheap), ::core::mem::transmute_copy(&numranges), ::core::mem::transmute_copy(&prangeflags), ::core::mem::transmute_copy(&pheaprangestartoffsets), ::core::mem::transmute_copy(&prangetilecounts), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn CopyTileMappings<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: ::windows::core::RawPtr, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyTileMappings(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&pdstregionstartcoordinate), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&psrcregionstartcoordinate), ::core::mem::transmute_copy(&pregionsize), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ExecuteCommandLists<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numcommandlists: u32, ppcommandlists: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExecuteCommandLists(::core::mem::transmute_copy(&numcommandlists), ::core::mem::transmute_copy(&ppcommandlists))
        }
        unsafe extern "system" fn SetMarker<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMarker(::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn BeginEvent<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginEvent(::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn EndEvent<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndEvent()
        }
        unsafe extern "system" fn Signal<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Signal(::core::mem::transmute(&pfence), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Wait<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Wait(::core::mem::transmute(&pfence), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTimestampFrequency<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequency: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTimestampFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *pfrequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClockCalibration<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClockCalibration(::core::mem::transmute_copy(&pgputimestamp), ::core::mem::transmute_copy(&pcputimestamp)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_COMMAND_QUEUE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        Self {
            base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateTileMappings: UpdateTileMappings::<Identity, Impl, OFFSET>,
            CopyTileMappings: CopyTileMappings::<Identity, Impl, OFFSET>,
            ExecuteCommandLists: ExecuteCommandLists::<Identity, Impl, OFFSET>,
            SetMarker: SetMarker::<Identity, Impl, OFFSET>,
            BeginEvent: BeginEvent::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
            Signal: Signal::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            GetTimestampFrequency: GetTimestampFrequency::<Identity, Impl, OFFSET>,
            GetClockCalibration: GetClockCalibration::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12CommandSignature_Impl, const OFFSET: isize>() -> ID3D12CommandSignature_Vtbl {
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12CommandSignature as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12Debug_Impl: Sized {
    fn EnableDebugLayer(&mut self);
}
impl ID3D12Debug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug_Impl, const OFFSET: isize>() -> ID3D12Debug_Vtbl {
        unsafe extern "system" fn EnableDebugLayer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableDebugLayer()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), EnableDebugLayer: EnableDebugLayer::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug1_Impl, const OFFSET: isize>() -> ID3D12Debug1_Vtbl {
        unsafe extern "system" fn EnableDebugLayer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableDebugLayer()
        }
        unsafe extern "system" fn SetEnableGPUBasedValidation<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableGPUBasedValidation(::core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn SetEnableSynchronizedCommandQueueValidation<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableSynchronizedCommandQueueValidation(::core::mem::transmute_copy(&enable))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnableDebugLayer: EnableDebugLayer::<Identity, Impl, OFFSET>,
            SetEnableGPUBasedValidation: SetEnableGPUBasedValidation::<Identity, Impl, OFFSET>,
            SetEnableSynchronizedCommandQueueValidation: SetEnableSynchronizedCommandQueueValidation::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug2_Impl, const OFFSET: isize>() -> ID3D12Debug2_Vtbl {
        unsafe extern "system" fn SetGPUBasedValidationFlags<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGPUBasedValidationFlags(::core::mem::transmute_copy(&flags))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetGPUBasedValidationFlags: SetGPUBasedValidationFlags::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug3_Impl, const OFFSET: isize>() -> ID3D12Debug3_Vtbl {
        unsafe extern "system" fn SetEnableGPUBasedValidation<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableGPUBasedValidation(::core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn SetEnableSynchronizedCommandQueueValidation<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableSynchronizedCommandQueueValidation(::core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn SetGPUBasedValidationFlags<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGPUBasedValidationFlags(::core::mem::transmute_copy(&flags))
        }
        Self {
            base: ID3D12Debug_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetEnableGPUBasedValidation: SetEnableGPUBasedValidation::<Identity, Impl, OFFSET>,
            SetEnableSynchronizedCommandQueueValidation: SetEnableSynchronizedCommandQueueValidation::<Identity, Impl, OFFSET>,
            SetGPUBasedValidationFlags: SetGPUBasedValidationFlags::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug4_Impl, const OFFSET: isize>() -> ID3D12Debug4_Vtbl {
        unsafe extern "system" fn DisableDebugLayer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableDebugLayer()
        }
        Self { base: ID3D12Debug3_Vtbl::new::<Identity, Impl, OFFSET>(), DisableDebugLayer: DisableDebugLayer::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug5_Impl, const OFFSET: isize>() -> ID3D12Debug5_Vtbl {
        unsafe extern "system" fn SetEnableAutoName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Debug5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableAutoName(::core::mem::transmute_copy(&enable))
        }
        Self { base: ID3D12Debug4_Vtbl::new::<Identity, Impl, OFFSET>(), SetEnableAutoName: SetEnableAutoName::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>() -> ID3D12DebugCommandList_Vtbl {
        unsafe extern "system" fn AssertResourceState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssertResourceState(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&state))
        }
        unsafe extern "system" fn SetFeatureMask<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFeatureMask(::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_DEBUG_FEATURE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFeatureMask()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AssertResourceState: AssertResourceState::<Identity, Impl, OFFSET>,
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>() -> ID3D12DebugCommandList1_Vtbl {
        unsafe extern "system" fn AssertResourceState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssertResourceState(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&state))
        }
        unsafe extern "system" fn SetDebugParameter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AssertResourceState: AssertResourceState::<Identity, Impl, OFFSET>,
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList2_Impl, const OFFSET: isize>() -> ID3D12DebugCommandList2_Vtbl {
        unsafe extern "system" fn SetDebugParameter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        Self {
            base: ID3D12DebugCommandList_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandQueue_Impl, const OFFSET: isize>() -> ID3D12DebugCommandQueue_Vtbl {
        unsafe extern "system" fn AssertResourceState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugCommandQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssertResourceState(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&state))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AssertResourceState: AssertResourceState::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>() -> ID3D12DebugDevice_Vtbl {
        unsafe extern "system" fn SetFeatureMask<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFeatureMask(::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_DEBUG_FEATURE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFeatureMask()
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportLiveDeviceObjects(::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>() -> ID3D12DebugDevice1_Vtbl {
        unsafe extern "system" fn SetDebugParameter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportLiveDeviceObjects(::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice2_Impl, const OFFSET: isize>() -> ID3D12DebugDevice2_Vtbl {
        unsafe extern "system" fn SetDebugParameter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DebugDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDebugParameter(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into()
        }
        Self {
            base: ID3D12DebugDevice_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>() -> ID3D12DescriptorHeap_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_DESCRIPTOR_HEAP_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        unsafe extern "system" fn GetCPUDescriptorHandleForHeapStart<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetCPUDescriptorHandleForHeapStart()
        }
        unsafe extern "system" fn GetGPUDescriptorHandleForHeapStart<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetGPUDescriptorHandleForHeapStart()
        }
        Self {
            base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetCPUDescriptorHandleForHeapStart: GetCPUDescriptorHandleForHeapStart::<Identity, Impl, OFFSET>,
            GetGPUDescriptorHandleForHeapStart: GetGPUDescriptorHandleForHeapStart::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>() -> ID3D12Device_Vtbl {
        unsafe extern "system" fn GetNodeCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNodeCount()
        }
        unsafe extern "system" fn CreateCommandQueue<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, riid: *const ::windows::core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCommandQueue(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandqueue)).into()
        }
        unsafe extern "system" fn CreateCommandAllocator<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3D12_COMMAND_LIST_TYPE, riid: *const ::windows::core::GUID, ppcommandallocator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCommandAllocator(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandallocator)).into()
        }
        unsafe extern "system" fn CreateGraphicsPipelineState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateGraphicsPipelineState(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn CreateComputePipelineState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateComputePipelineState(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn CreateCommandList<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: ::windows::core::RawPtr, pinitialstate: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCommandList(::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&pcommandallocator), ::core::mem::transmute(&pinitialstate), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandlist)).into()
        }
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        unsafe extern "system" fn CreateDescriptorHeap<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDescriptorHeap(::core::mem::transmute_copy(&pdescriptorheapdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn GetDescriptorHandleIncrementSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDescriptorHandleIncrementSize(::core::mem::transmute_copy(&descriptorheaptype))
        }
        unsafe extern "system" fn CreateRootSignature<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, pblobwithrootsignature: *const ::core::ffi::c_void, bloblengthinbytes: usize, riid: *const ::windows::core::GUID, ppvrootsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateRootSignature(::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&pblobwithrootsignature), ::core::mem::transmute_copy(&bloblengthinbytes), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvrootsignature)).into()
        }
        unsafe extern "system" fn CreateConstantBufferView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_CONSTANT_BUFFER_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateConstantBufferView(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D12_SHADER_RESOURCE_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateShaderResourceView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pcounterresource: ::windows::core::RawPtr, pdesc: *const D3D12_UNORDERED_ACCESS_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateUnorderedAccessView(::core::mem::transmute(&presource), ::core::mem::transmute(&pcounterresource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D12_RENDER_TARGET_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateRenderTargetView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D12_DEPTH_STENCIL_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDepthStencilView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CreateSampler<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSampler(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn CopyDescriptors<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: *const u32, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: *const u32, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyDescriptors(::core::mem::transmute_copy(&numdestdescriptorranges), ::core::mem::transmute_copy(&pdestdescriptorrangestarts), ::core::mem::transmute_copy(&pdestdescriptorrangesizes), ::core::mem::transmute_copy(&numsrcdescriptorranges), ::core::mem::transmute_copy(&psrcdescriptorrangestarts), ::core::mem::transmute_copy(&psrcdescriptorrangesizes), ::core::mem::transmute_copy(&descriptorheapstype))
        }
        unsafe extern "system" fn CopyDescriptorsSimple<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyDescriptorsSimple(::core::mem::transmute_copy(&numdescriptors), ::core::mem::transmute_copy(&destdescriptorrangestart), ::core::mem::transmute_copy(&srcdescriptorrangestart), ::core::mem::transmute_copy(&descriptorheapstype))
        }
        unsafe extern "system" fn GetResourceAllocationInfo<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetResourceAllocationInfo(::core::mem::transmute_copy(&visiblemask), ::core::mem::transmute_copy(&numresourcedescs), ::core::mem::transmute_copy(&presourcedescs))
        }
        unsafe extern "system" fn GetCustomHeapProperties<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_HEAP_PROPERTIES, nodemask: u32, heaptype: D3D12_HEAP_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetCustomHeapProperties(::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&heaptype))
        }
        unsafe extern "system" fn CreateCommittedResource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCommittedResource(::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&heapflags), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialresourcestate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateHeap<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateHeap(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn CreatePlacedResource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheap: ::windows::core::RawPtr, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreatePlacedResource(::core::mem::transmute(&pheap), ::core::mem::transmute_copy(&heapoffset), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateReservedResource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateReservedResource(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: super::super::Foundation::PWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSharedHandle(::core::mem::transmute(&pobject), ::core::mem::transmute_copy(&pattributes), ::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedHandle<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nthandle: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenSharedHandle(::core::mem::transmute_copy(&nthandle), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn OpenSharedHandleByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, access: u32, pnthandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenSharedHandleByName(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&access)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnthandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeResident<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MakeResident(::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn Evict<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Evict(::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn CreateFence<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: u64, flags: D3D12_FENCE_FLAGS, riid: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateFence(::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppfence)).into()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceRemovedReason().into()
        }
        unsafe extern "system" fn GetCopyableFootprints<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCopyableFootprints(::core::mem::transmute_copy(&presourcedesc), ::core::mem::transmute_copy(&firstsubresource), ::core::mem::transmute_copy(&numsubresources), ::core::mem::transmute_copy(&baseoffset), ::core::mem::transmute_copy(&playouts), ::core::mem::transmute_copy(&pnumrows), ::core::mem::transmute_copy(&prowsizeinbytes), ::core::mem::transmute_copy(&ptotalbytes))
        }
        unsafe extern "system" fn CreateQueryHeap<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_QUERY_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateQueryHeap(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn SetStablePowerState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStablePowerState(::core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn CreateCommandSignature<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvcommandsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCommandSignature(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&prootsignature), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvcommandsignature)).into()
        }
        unsafe extern "system" fn GetResourceTiling<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D12_PACKED_MIP_INFO, pstandardtileshapefornonpackedmips: *mut D3D12_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResourceTiling(::core::mem::transmute(&ptiledresource), ::core::mem::transmute_copy(&pnumtilesforentireresource), ::core::mem::transmute_copy(&ppackedmipdesc), ::core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), ::core::mem::transmute_copy(&pnumsubresourcetilings), ::core::mem::transmute_copy(&firstsubresourcetilingtoget), ::core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips))
        }
        unsafe extern "system" fn GetAdapterLuid<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetAdapterLuid()
        }
        Self {
            base: ID3D12Object_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNodeCount: GetNodeCount::<Identity, Impl, OFFSET>,
            CreateCommandQueue: CreateCommandQueue::<Identity, Impl, OFFSET>,
            CreateCommandAllocator: CreateCommandAllocator::<Identity, Impl, OFFSET>,
            CreateGraphicsPipelineState: CreateGraphicsPipelineState::<Identity, Impl, OFFSET>,
            CreateComputePipelineState: CreateComputePipelineState::<Identity, Impl, OFFSET>,
            CreateCommandList: CreateCommandList::<Identity, Impl, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            CreateDescriptorHeap: CreateDescriptorHeap::<Identity, Impl, OFFSET>,
            GetDescriptorHandleIncrementSize: GetDescriptorHandleIncrementSize::<Identity, Impl, OFFSET>,
            CreateRootSignature: CreateRootSignature::<Identity, Impl, OFFSET>,
            CreateConstantBufferView: CreateConstantBufferView::<Identity, Impl, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, Impl, OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Identity, Impl, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, Impl, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, Impl, OFFSET>,
            CreateSampler: CreateSampler::<Identity, Impl, OFFSET>,
            CopyDescriptors: CopyDescriptors::<Identity, Impl, OFFSET>,
            CopyDescriptorsSimple: CopyDescriptorsSimple::<Identity, Impl, OFFSET>,
            GetResourceAllocationInfo: GetResourceAllocationInfo::<Identity, Impl, OFFSET>,
            GetCustomHeapProperties: GetCustomHeapProperties::<Identity, Impl, OFFSET>,
            CreateCommittedResource: CreateCommittedResource::<Identity, Impl, OFFSET>,
            CreateHeap: CreateHeap::<Identity, Impl, OFFSET>,
            CreatePlacedResource: CreatePlacedResource::<Identity, Impl, OFFSET>,
            CreateReservedResource: CreateReservedResource::<Identity, Impl, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
            OpenSharedHandle: OpenSharedHandle::<Identity, Impl, OFFSET>,
            OpenSharedHandleByName: OpenSharedHandleByName::<Identity, Impl, OFFSET>,
            MakeResident: MakeResident::<Identity, Impl, OFFSET>,
            Evict: Evict::<Identity, Impl, OFFSET>,
            CreateFence: CreateFence::<Identity, Impl, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, Impl, OFFSET>,
            GetCopyableFootprints: GetCopyableFootprints::<Identity, Impl, OFFSET>,
            CreateQueryHeap: CreateQueryHeap::<Identity, Impl, OFFSET>,
            SetStablePowerState: SetStablePowerState::<Identity, Impl, OFFSET>,
            CreateCommandSignature: CreateCommandSignature::<Identity, Impl, OFFSET>,
            GetResourceTiling: GetResourceTiling::<Identity, Impl, OFFSET>,
            GetAdapterLuid: GetAdapterLuid::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device1_Impl, const OFFSET: isize>() -> ID3D12Device1_Vtbl {
        unsafe extern "system" fn CreatePipelineLibrary<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibraryblob: *const ::core::ffi::c_void, bloblength: usize, riid: *const ::windows::core::GUID, pppipelinelibrary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreatePipelineLibrary(::core::mem::transmute_copy(&plibraryblob), ::core::mem::transmute_copy(&bloblength), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinelibrary)).into()
        }
        unsafe extern "system" fn SetEventOnMultipleFenceCompletion<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfences: *const ::windows::core::RawPtr, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventOnMultipleFenceCompletion(::core::mem::transmute_copy(&ppfences), ::core::mem::transmute_copy(&pfencevalues), ::core::mem::transmute_copy(&numfences), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn SetResidencyPriority<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const ::windows::core::RawPtr, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetResidencyPriority(::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects), ::core::mem::transmute_copy(&ppriorities)).into()
        }
        Self {
            base: ID3D12Device_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreatePipelineLibrary: CreatePipelineLibrary::<Identity, Impl, OFFSET>,
            SetEventOnMultipleFenceCompletion: SetEventOnMultipleFenceCompletion::<Identity, Impl, OFFSET>,
            SetResidencyPriority: SetResidencyPriority::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device2_Impl, const OFFSET: isize>() -> ID3D12Device2_Vtbl {
        unsafe extern "system" fn CreatePipelineState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreatePipelineState(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        Self { base: ID3D12Device1_Vtbl::new::<Identity, Impl, OFFSET>(), CreatePipelineState: CreatePipelineState::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device3_Impl, const OFFSET: isize>() -> ID3D12Device3_Vtbl {
        unsafe extern "system" fn OpenExistingHeapFromAddress<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenExistingHeapFromAddress(::core::mem::transmute_copy(&paddress), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn OpenExistingHeapFromFileMapping<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfilemapping: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenExistingHeapFromFileMapping(::core::mem::transmute_copy(&hfilemapping), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn EnqueueMakeResident<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D12_RESIDENCY_FLAGS, numobjects: u32, ppobjects: *const ::windows::core::RawPtr, pfencetosignal: ::windows::core::RawPtr, fencevaluetosignal: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnqueueMakeResident(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects), ::core::mem::transmute(&pfencetosignal), ::core::mem::transmute_copy(&fencevaluetosignal)).into()
        }
        Self {
            base: ID3D12Device2_Vtbl::new::<Identity, Impl, OFFSET>(),
            OpenExistingHeapFromAddress: OpenExistingHeapFromAddress::<Identity, Impl, OFFSET>,
            OpenExistingHeapFromFileMapping: OpenExistingHeapFromFileMapping::<Identity, Impl, OFFSET>,
            EnqueueMakeResident: EnqueueMakeResident::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device4_Impl, const OFFSET: isize>() -> ID3D12Device4_Vtbl {
        unsafe extern "system" fn CreateCommandList1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS, riid: *const ::windows::core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCommandList1(::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandlist)).into()
        }
        unsafe extern "system" fn CreateProtectedResourceSession<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateProtectedResourceSession(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppsession)).into()
        }
        unsafe extern "system" fn CreateCommittedResource1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::windows::core::RawPtr, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCommittedResource1(::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&heapflags), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialresourcestate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute(&pprotectedsession), ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateHeap1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateHeap1(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&pprotectedsession), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn CreateReservedResource1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateReservedResource1(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute(&pprotectedsession), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn GetResourceAllocationInfo1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetResourceAllocationInfo1(::core::mem::transmute_copy(&visiblemask), ::core::mem::transmute_copy(&numresourcedescs), ::core::mem::transmute_copy(&presourcedescs), ::core::mem::transmute_copy(&presourceallocationinfo1))
        }
        Self {
            base: ID3D12Device3_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateCommandList1: CreateCommandList1::<Identity, Impl, OFFSET>,
            CreateProtectedResourceSession: CreateProtectedResourceSession::<Identity, Impl, OFFSET>,
            CreateCommittedResource1: CreateCommittedResource1::<Identity, Impl, OFFSET>,
            CreateHeap1: CreateHeap1::<Identity, Impl, OFFSET>,
            CreateReservedResource1: CreateReservedResource1::<Identity, Impl, OFFSET>,
            GetResourceAllocationInfo1: GetResourceAllocationInfo1::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5_Impl, const OFFSET: isize>() -> ID3D12Device5_Vtbl {
        unsafe extern "system" fn CreateLifetimeTracker<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, powner: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvtracker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateLifetimeTracker(::core::mem::transmute(&powner), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvtracker)).into()
        }
        unsafe extern "system" fn RemoveDevice<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveDevice()
        }
        unsafe extern "system" fn EnumerateMetaCommands<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnummetacommands: *mut u32, pdescs: *mut D3D12_META_COMMAND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumerateMetaCommands(::core::mem::transmute_copy(&pnummetacommands), ::core::mem::transmute_copy(&pdescs)).into()
        }
        unsafe extern "system" fn EnumerateMetaCommandParameters<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: *const ::windows::core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: *mut u32, pparametercount: *mut u32, pparameterdescs: *mut D3D12_META_COMMAND_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumerateMetaCommandParameters(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&ptotalstructuresizeinbytes), ::core::mem::transmute_copy(&pparametercount), ::core::mem::transmute_copy(&pparameterdescs)).into()
        }
        unsafe extern "system" fn CreateMetaCommand<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: *const ::windows::core::GUID, nodemask: u32, pcreationparametersdata: *const ::core::ffi::c_void, creationparametersdatasizeinbytes: usize, riid: *const ::windows::core::GUID, ppmetacommand: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateMetaCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&pcreationparametersdata), ::core::mem::transmute_copy(&creationparametersdatasizeinbytes), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppmetacommand)).into()
        }
        unsafe extern "system" fn CreateStateObject<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_STATE_OBJECT_DESC, riid: *const ::windows::core::GUID, ppstateobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateStateObject(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppstateobject)).into()
        }
        unsafe extern "system" fn GetRaytracingAccelerationStructurePrebuildInfo<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRaytracingAccelerationStructurePrebuildInfo(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinfo))
        }
        unsafe extern "system" fn CheckDriverMatchingIdentifier<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckDriverMatchingIdentifier(::core::mem::transmute_copy(&serializeddatatype), ::core::mem::transmute_copy(&pidentifiertocheck))
        }
        Self {
            base: ID3D12Device4_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateLifetimeTracker: CreateLifetimeTracker::<Identity, Impl, OFFSET>,
            RemoveDevice: RemoveDevice::<Identity, Impl, OFFSET>,
            EnumerateMetaCommands: EnumerateMetaCommands::<Identity, Impl, OFFSET>,
            EnumerateMetaCommandParameters: EnumerateMetaCommandParameters::<Identity, Impl, OFFSET>,
            CreateMetaCommand: CreateMetaCommand::<Identity, Impl, OFFSET>,
            CreateStateObject: CreateStateObject::<Identity, Impl, OFFSET>,
            GetRaytracingAccelerationStructurePrebuildInfo: GetRaytracingAccelerationStructurePrebuildInfo::<Identity, Impl, OFFSET>,
            CheckDriverMatchingIdentifier: CheckDriverMatchingIdentifier::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device6_Impl, const OFFSET: isize>() -> ID3D12Device6_Vtbl {
        unsafe extern "system" fn SetBackgroundProcessingMode<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: super::super::Foundation::HANDLE, pbfurthermeasurementsdesired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetBackgroundProcessingMode(::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&measurementsaction), ::core::mem::transmute_copy(&heventtosignaluponcompletion)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbfurthermeasurementsdesired = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ID3D12Device5_Vtbl::new::<Identity, Impl, OFFSET>(), SetBackgroundProcessingMode: SetBackgroundProcessingMode::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device7_Impl, const OFFSET: isize>() -> ID3D12Device7_Vtbl {
        unsafe extern "system" fn AddToStateObject<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppnewstateobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddToStateObject(::core::mem::transmute_copy(&paddition), ::core::mem::transmute(&pstateobjecttogrowfrom), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppnewstateobject)).into()
        }
        unsafe extern "system" fn CreateProtectedResourceSession1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateProtectedResourceSession1(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppsession)).into()
        }
        Self {
            base: ID3D12Device6_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddToStateObject: AddToStateObject::<Identity, Impl, OFFSET>,
            CreateProtectedResourceSession1: CreateProtectedResourceSession1::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device8_Impl, const OFFSET: isize>() -> ID3D12Device8_Vtbl {
        unsafe extern "system" fn GetResourceAllocationInfo2<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetResourceAllocationInfo2(::core::mem::transmute_copy(&visiblemask), ::core::mem::transmute_copy(&numresourcedescs), ::core::mem::transmute_copy(&presourcedescs), ::core::mem::transmute_copy(&presourceallocationinfo1))
        }
        unsafe extern "system" fn CreateCommittedResource2<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::windows::core::RawPtr, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCommittedResource2(::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&heapflags), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialresourcestate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute(&pprotectedsession), ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreatePlacedResource1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheap: ::windows::core::RawPtr, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreatePlacedResource1(::core::mem::transmute(&pheap), ::core::mem::transmute_copy(&heapoffset), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateSamplerFeedbackUnorderedAccessView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetedresource: ::windows::core::RawPtr, pfeedbackresource: ::windows::core::RawPtr, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSamplerFeedbackUnorderedAccessView(::core::mem::transmute(&ptargetedresource), ::core::mem::transmute(&pfeedbackresource), ::core::mem::transmute_copy(&destdescriptor))
        }
        unsafe extern "system" fn GetCopyableFootprints1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCopyableFootprints1(::core::mem::transmute_copy(&presourcedesc), ::core::mem::transmute_copy(&firstsubresource), ::core::mem::transmute_copy(&numsubresources), ::core::mem::transmute_copy(&baseoffset), ::core::mem::transmute_copy(&playouts), ::core::mem::transmute_copy(&pnumrows), ::core::mem::transmute_copy(&prowsizeinbytes), ::core::mem::transmute_copy(&ptotalbytes))
        }
        Self {
            base: ID3D12Device7_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetResourceAllocationInfo2: GetResourceAllocationInfo2::<Identity, Impl, OFFSET>,
            CreateCommittedResource2: CreateCommittedResource2::<Identity, Impl, OFFSET>,
            CreatePlacedResource1: CreatePlacedResource1::<Identity, Impl, OFFSET>,
            CreateSamplerFeedbackUnorderedAccessView: CreateSamplerFeedbackUnorderedAccessView::<Identity, Impl, OFFSET>,
            GetCopyableFootprints1: GetCopyableFootprints1::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device9_Impl, const OFFSET: isize>() -> ID3D12Device9_Vtbl {
        unsafe extern "system" fn CreateShaderCacheSession<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, riid: *const ::windows::core::GUID, ppvsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateShaderCacheSession(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvsession)).into()
        }
        unsafe extern "system" fn ShaderCacheControl<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShaderCacheControl(::core::mem::transmute_copy(&kinds), ::core::mem::transmute_copy(&control)).into()
        }
        unsafe extern "system" fn CreateCommandQueue1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Device9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCommandQueue1(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&creatorid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandqueue)).into()
        }
        Self {
            base: ID3D12Device8_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateShaderCacheSession: CreateShaderCacheSession::<Identity, Impl, OFFSET>,
            ShaderCacheControl: ShaderCacheControl::<Identity, Impl, OFFSET>,
            CreateCommandQueue1: CreateCommandQueue1::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceChild_Impl, const OFFSET: isize>() -> ID3D12DeviceChild_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvdevice)).into()
        }
        Self { base: ID3D12Object_Vtbl::new::<Identity, Impl, OFFSET>(), GetDevice: GetDevice::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData_Vtbl {
        unsafe extern "system" fn GetAutoBreadcrumbsOutput<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAutoBreadcrumbsOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageFaultAllocationOutput<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPageFaultAllocationOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAutoBreadcrumbsOutput: GetAutoBreadcrumbsOutput::<Identity, Impl, OFFSET>,
            GetPageFaultAllocationOutput: GetPageFaultAllocationOutput::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData1_Vtbl {
        unsafe extern "system" fn GetAutoBreadcrumbsOutput1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAutoBreadcrumbsOutput1() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageFaultAllocationOutput1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPageFaultAllocationOutput1() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D12DeviceRemovedExtendedData_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAutoBreadcrumbsOutput1: GetAutoBreadcrumbsOutput1::<Identity, Impl, OFFSET>,
            GetPageFaultAllocationOutput1: GetPageFaultAllocationOutput1::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData2_Vtbl {
        unsafe extern "system" fn GetPageFaultAllocationOutput2<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPageFaultAllocationOutput2() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_DRED_DEVICE_STATE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceState()
        }
        Self {
            base: ID3D12DeviceRemovedExtendedData1_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPageFaultAllocationOutput2: GetPageFaultAllocationOutput2::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedDataSettings_Vtbl {
        unsafe extern "system" fn SetAutoBreadcrumbsEnablement<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoBreadcrumbsEnablement(::core::mem::transmute_copy(&enablement))
        }
        unsafe extern "system" fn SetPageFaultEnablement<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPageFaultEnablement(::core::mem::transmute_copy(&enablement))
        }
        unsafe extern "system" fn SetWatsonDumpEnablement<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWatsonDumpEnablement(::core::mem::transmute_copy(&enablement))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAutoBreadcrumbsEnablement: SetAutoBreadcrumbsEnablement::<Identity, Impl, OFFSET>,
            SetPageFaultEnablement: SetPageFaultEnablement::<Identity, Impl, OFFSET>,
            SetWatsonDumpEnablement: SetWatsonDumpEnablement::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataSettings1_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedDataSettings1_Vtbl {
        unsafe extern "system" fn SetBreadcrumbContextEnablement<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12DeviceRemovedExtendedDataSettings1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBreadcrumbContextEnablement(::core::mem::transmute_copy(&enablement))
        }
        Self {
            base: ID3D12DeviceRemovedExtendedDataSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBreadcrumbContextEnablement: SetBreadcrumbContextEnablement::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Fence_Impl, const OFFSET: isize>() -> ID3D12Fence_Vtbl {
        unsafe extern "system" fn GetCompletedValue<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCompletedValue()
        }
        unsafe extern "system" fn SetEventOnCompletion<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventOnCompletion(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Signal<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Signal(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCompletedValue: GetCompletedValue::<Identity, Impl, OFFSET>,
            SetEventOnCompletion: SetEventOnCompletion::<Identity, Impl, OFFSET>,
            Signal: Signal::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Fence1_Impl, const OFFSET: isize>() -> ID3D12Fence1_Vtbl {
        unsafe extern "system" fn GetCreationFlags<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Fence1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_FENCE_FLAGS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCreationFlags()
        }
        Self { base: ID3D12Fence_Vtbl::new::<Identity, Impl, OFFSET>(), GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionParameterReflection_Impl, const OFFSET: isize>() -> ID3D12FunctionParameterReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionParameterReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { GetDesc: GetDesc::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>() -> ID3D12FunctionReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_FUNCTION_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&bufferindex))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResourceBindingDescByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionParameter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: i32) -> ::core::option::Option<ID3D12FunctionParameterReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFunctionParameter(::core::mem::transmute_copy(&parameterindex))
        }
        Self {
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, Impl, OFFSET>,
            GetFunctionParameter: GetFunctionParameter::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList_Vtbl {
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocator: ::windows::core::RawPtr, pinitialstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset(::core::mem::transmute(&pallocator), ::core::mem::transmute(&pinitialstate)).into()
        }
        unsafe extern "system" fn ClearState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipelinestate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearState(::core::mem::transmute(&ppipelinestate))
        }
        unsafe extern "system" fn DrawInstanced<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawInstanced(::core::mem::transmute_copy(&vertexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startvertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawIndexedInstanced(::core::mem::transmute_copy(&indexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn Dispatch<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Dispatch(::core::mem::transmute_copy(&threadgroupcountx), ::core::mem::transmute_copy(&threadgroupcounty), ::core::mem::transmute_copy(&threadgroupcountz))
        }
        unsafe extern "system" fn CopyBufferRegion<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstoffset: u64, psrcbuffer: ::windows::core::RawPtr, srcoffset: u64, numbytes: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyBufferRegion(::core::mem::transmute(&pdstbuffer), ::core::mem::transmute_copy(&dstoffset), ::core::mem::transmute(&psrcbuffer), ::core::mem::transmute_copy(&srcoffset), ::core::mem::transmute_copy(&numbytes))
        }
        unsafe extern "system" fn CopyTextureRegion<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: *const D3D12_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyTextureRegion(::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&psrcbox))
        }
        unsafe extern "system" fn CopyResource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyResource(::core::mem::transmute(&pdstresource), ::core::mem::transmute(&psrcresource))
        }
        unsafe extern "system" fn CopyTiles<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: ::windows::core::RawPtr, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyTiles(::core::mem::transmute(&ptiledresource), ::core::mem::transmute_copy(&ptileregionstartcoordinate), ::core::mem::transmute_copy(&ptileregionsize), ::core::mem::transmute(&pbuffer), ::core::mem::transmute_copy(&bufferstartoffsetinbytes), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ResolveSubresource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResolveSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IASetPrimitiveTopology(::core::mem::transmute_copy(&primitivetopology))
        }
        unsafe extern "system" fn RSSetViewports<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D12_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RSSetViewports(::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RSSetScissorRects(::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn OMSetBlendFactor<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blendfactor: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMSetBlendFactor(::core::mem::transmute_copy(&blendfactor))
        }
        unsafe extern "system" fn OMSetStencilRef<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stencilref: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMSetStencilRef(::core::mem::transmute_copy(&stencilref))
        }
        unsafe extern "system" fn SetPipelineState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipelinestate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPipelineState(::core::mem::transmute(&ppipelinestate))
        }
        unsafe extern "system" fn ResourceBarrier<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbarriers: u32, pbarriers: *const D3D12_RESOURCE_BARRIER) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResourceBarrier(::core::mem::transmute_copy(&numbarriers), ::core::mem::transmute_copy(&pbarriers))
        }
        unsafe extern "system" fn ExecuteBundle<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandlist: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExecuteBundle(::core::mem::transmute(&pcommandlist))
        }
        unsafe extern "system" fn SetDescriptorHeaps<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numdescriptorheaps: u32, ppdescriptorheaps: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescriptorHeaps(::core::mem::transmute_copy(&numdescriptorheaps), ::core::mem::transmute_copy(&ppdescriptorheaps))
        }
        unsafe extern "system" fn SetComputeRootSignature<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootsignature: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComputeRootSignature(::core::mem::transmute(&prootsignature))
        }
        unsafe extern "system" fn SetGraphicsRootSignature<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootsignature: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraphicsRootSignature(::core::mem::transmute(&prootsignature))
        }
        unsafe extern "system" fn SetComputeRootDescriptorTable<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComputeRootDescriptorTable(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&basedescriptor))
        }
        unsafe extern "system" fn SetGraphicsRootDescriptorTable<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraphicsRootDescriptorTable(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&basedescriptor))
        }
        unsafe extern "system" fn SetComputeRoot32BitConstant<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComputeRoot32BitConstant(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&srcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetGraphicsRoot32BitConstant<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraphicsRoot32BitConstant(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&srcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetComputeRoot32BitConstants<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComputeRoot32BitConstants(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&num32bitvaluestoset), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetGraphicsRoot32BitConstants<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraphicsRoot32BitConstants(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&num32bitvaluestoset), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetComputeRootConstantBufferView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComputeRootConstantBufferView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetGraphicsRootConstantBufferView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraphicsRootConstantBufferView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetComputeRootShaderResourceView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComputeRootShaderResourceView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetGraphicsRootShaderResourceView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraphicsRootShaderResourceView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetComputeRootUnorderedAccessView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComputeRootUnorderedAccessView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetGraphicsRootUnorderedAccessView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraphicsRootUnorderedAccessView(::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pview: *const D3D12_INDEX_BUFFER_VIEW) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IASetIndexBuffer(::core::mem::transmute_copy(&pview))
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_VERTEX_BUFFER_VIEW) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IASetVertexBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pviews))
        }
        unsafe extern "system" fn SOSetTargets<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_STREAM_OUTPUT_BUFFER_VIEW) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SOSetTargets(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pviews))
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrendertargetdescriptors: u32, prendertargetdescriptors: *const D3D12_CPU_DESCRIPTOR_HANDLE, rtssinglehandletodescriptorrange: super::super::Foundation::BOOL, pdepthstencildescriptor: *const D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMSetRenderTargets(::core::mem::transmute_copy(&numrendertargetdescriptors), ::core::mem::transmute_copy(&prendertargetdescriptors), ::core::mem::transmute_copy(&rtssinglehandletodescriptorrange), ::core::mem::transmute_copy(&pdepthstencildescriptor))
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearDepthStencilView(::core::mem::transmute_copy(&depthstencilview), ::core::mem::transmute_copy(&clearflags), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&stencil), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearRenderTargetView(::core::mem::transmute_copy(&rendertargetview), ::core::mem::transmute_copy(&colorrgba), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: ::windows::core::RawPtr, values: *const u32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearUnorderedAccessViewUint(::core::mem::transmute_copy(&viewgpuhandleincurrentheap), ::core::mem::transmute_copy(&viewcpuhandle), ::core::mem::transmute(&presource), ::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: ::windows::core::RawPtr, values: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearUnorderedAccessViewFloat(::core::mem::transmute_copy(&viewgpuhandleincurrentheap), ::core::mem::transmute_copy(&viewcpuhandle), ::core::mem::transmute(&presource), ::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn DiscardResource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pregion: *const D3D12_DISCARD_REGION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardResource(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pregion))
        }
        unsafe extern "system" fn BeginQuery<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginQuery(::core::mem::transmute(&pqueryheap), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn EndQuery<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndQuery(::core::mem::transmute(&pqueryheap), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn ResolveQueryData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqueryheap: ::windows::core::RawPtr, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: ::windows::core::RawPtr, aligneddestinationbufferoffset: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResolveQueryData(::core::mem::transmute(&pqueryheap), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&numqueries), ::core::mem::transmute(&pdestinationbuffer), ::core::mem::transmute_copy(&aligneddestinationbufferoffset))
        }
        unsafe extern "system" fn SetPredication<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPredication(::core::mem::transmute(&pbuffer), ::core::mem::transmute_copy(&alignedbufferoffset), ::core::mem::transmute_copy(&operation))
        }
        unsafe extern "system" fn SetMarker<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMarker(::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn BeginEvent<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginEvent(::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn EndEvent<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndEvent()
        }
        unsafe extern "system" fn ExecuteIndirect<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandsignature: ::windows::core::RawPtr, maxcommandcount: u32, pargumentbuffer: ::windows::core::RawPtr, argumentbufferoffset: u64, pcountbuffer: ::windows::core::RawPtr, countbufferoffset: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExecuteIndirect(::core::mem::transmute(&pcommandsignature), ::core::mem::transmute_copy(&maxcommandcount), ::core::mem::transmute(&pargumentbuffer), ::core::mem::transmute_copy(&argumentbufferoffset), ::core::mem::transmute(&pcountbuffer), ::core::mem::transmute_copy(&countbufferoffset))
        }
        Self {
            base: ID3D12CommandList_Vtbl::new::<Identity, Impl, OFFSET>(),
            Close: Close::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ClearState: ClearState::<Identity, Impl, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, Impl, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, Impl, OFFSET>,
            Dispatch: Dispatch::<Identity, Impl, OFFSET>,
            CopyBufferRegion: CopyBufferRegion::<Identity, Impl, OFFSET>,
            CopyTextureRegion: CopyTextureRegion::<Identity, Impl, OFFSET>,
            CopyResource: CopyResource::<Identity, Impl, OFFSET>,
            CopyTiles: CopyTiles::<Identity, Impl, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, Impl, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, Impl, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, Impl, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, Impl, OFFSET>,
            OMSetBlendFactor: OMSetBlendFactor::<Identity, Impl, OFFSET>,
            OMSetStencilRef: OMSetStencilRef::<Identity, Impl, OFFSET>,
            SetPipelineState: SetPipelineState::<Identity, Impl, OFFSET>,
            ResourceBarrier: ResourceBarrier::<Identity, Impl, OFFSET>,
            ExecuteBundle: ExecuteBundle::<Identity, Impl, OFFSET>,
            SetDescriptorHeaps: SetDescriptorHeaps::<Identity, Impl, OFFSET>,
            SetComputeRootSignature: SetComputeRootSignature::<Identity, Impl, OFFSET>,
            SetGraphicsRootSignature: SetGraphicsRootSignature::<Identity, Impl, OFFSET>,
            SetComputeRootDescriptorTable: SetComputeRootDescriptorTable::<Identity, Impl, OFFSET>,
            SetGraphicsRootDescriptorTable: SetGraphicsRootDescriptorTable::<Identity, Impl, OFFSET>,
            SetComputeRoot32BitConstant: SetComputeRoot32BitConstant::<Identity, Impl, OFFSET>,
            SetGraphicsRoot32BitConstant: SetGraphicsRoot32BitConstant::<Identity, Impl, OFFSET>,
            SetComputeRoot32BitConstants: SetComputeRoot32BitConstants::<Identity, Impl, OFFSET>,
            SetGraphicsRoot32BitConstants: SetGraphicsRoot32BitConstants::<Identity, Impl, OFFSET>,
            SetComputeRootConstantBufferView: SetComputeRootConstantBufferView::<Identity, Impl, OFFSET>,
            SetGraphicsRootConstantBufferView: SetGraphicsRootConstantBufferView::<Identity, Impl, OFFSET>,
            SetComputeRootShaderResourceView: SetComputeRootShaderResourceView::<Identity, Impl, OFFSET>,
            SetGraphicsRootShaderResourceView: SetGraphicsRootShaderResourceView::<Identity, Impl, OFFSET>,
            SetComputeRootUnorderedAccessView: SetComputeRootUnorderedAccessView::<Identity, Impl, OFFSET>,
            SetGraphicsRootUnorderedAccessView: SetGraphicsRootUnorderedAccessView::<Identity, Impl, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, Impl, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, Impl, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, Impl, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, Impl, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, Impl, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat::<Identity, Impl, OFFSET>,
            DiscardResource: DiscardResource::<Identity, Impl, OFFSET>,
            BeginQuery: BeginQuery::<Identity, Impl, OFFSET>,
            EndQuery: EndQuery::<Identity, Impl, OFFSET>,
            ResolveQueryData: ResolveQueryData::<Identity, Impl, OFFSET>,
            SetPredication: SetPredication::<Identity, Impl, OFFSET>,
            SetMarker: SetMarker::<Identity, Impl, OFFSET>,
            BeginEvent: BeginEvent::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
            ExecuteIndirect: ExecuteIndirect::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList1_Vtbl {
        unsafe extern "system" fn AtomicCopyBufferUINT<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstoffset: u64, psrcbuffer: ::windows::core::RawPtr, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::windows::core::RawPtr, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AtomicCopyBufferUINT(::core::mem::transmute(&pdstbuffer), ::core::mem::transmute_copy(&dstoffset), ::core::mem::transmute(&psrcbuffer), ::core::mem::transmute_copy(&srcoffset), ::core::mem::transmute_copy(&dependencies), ::core::mem::transmute_copy(&ppdependentresources), ::core::mem::transmute_copy(&pdependentsubresourceranges))
        }
        unsafe extern "system" fn AtomicCopyBufferUINT64<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstoffset: u64, psrcbuffer: ::windows::core::RawPtr, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::windows::core::RawPtr, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AtomicCopyBufferUINT64(::core::mem::transmute(&pdstbuffer), ::core::mem::transmute_copy(&dstoffset), ::core::mem::transmute(&psrcbuffer), ::core::mem::transmute_copy(&srcoffset), ::core::mem::transmute_copy(&dependencies), ::core::mem::transmute_copy(&ppdependentresources), ::core::mem::transmute_copy(&pdependentsubresourceranges))
        }
        unsafe extern "system" fn OMSetDepthBounds<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, min: f32, max: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMSetDepthBounds(::core::mem::transmute_copy(&min), ::core::mem::transmute_copy(&max))
        }
        unsafe extern "system" fn SetSamplePositions<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSamplePositions(::core::mem::transmute_copy(&numsamplesperpixel), ::core::mem::transmute_copy(&numpixels), ::core::mem::transmute_copy(&psamplepositions))
        }
        unsafe extern "system" fn ResolveSubresourceRegion<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcrect: *const super::super::Foundation::RECT, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResolveSubresourceRegion(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcrect), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&resolvemode))
        }
        unsafe extern "system" fn SetViewInstanceMask<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetViewInstanceMask(::core::mem::transmute_copy(&mask))
        }
        Self {
            base: ID3D12GraphicsCommandList_Vtbl::new::<Identity, Impl, OFFSET>(),
            AtomicCopyBufferUINT: AtomicCopyBufferUINT::<Identity, Impl, OFFSET>,
            AtomicCopyBufferUINT64: AtomicCopyBufferUINT64::<Identity, Impl, OFFSET>,
            OMSetDepthBounds: OMSetDepthBounds::<Identity, Impl, OFFSET>,
            SetSamplePositions: SetSamplePositions::<Identity, Impl, OFFSET>,
            ResolveSubresourceRegion: ResolveSubresourceRegion::<Identity, Impl, OFFSET>,
            SetViewInstanceMask: SetViewInstanceMask::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList2_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList2_Vtbl {
        unsafe extern "system" fn WriteBufferImmediate<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteBufferImmediate(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&pmodes))
        }
        Self { base: ID3D12GraphicsCommandList1_Vtbl::new::<Identity, Impl, OFFSET>(), WriteBufferImmediate: WriteBufferImmediate::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList3_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList3_Vtbl {
        unsafe extern "system" fn SetProtectedResourceSession<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotectedresourcesession: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProtectedResourceSession(::core::mem::transmute(&pprotectedresourcesession))
        }
        Self {
            base: ID3D12GraphicsCommandList2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProtectedResourceSession: SetProtectedResourceSession::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList4_Vtbl {
        unsafe extern "system" fn BeginRenderPass<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrendertargets: u32, prendertargets: *const D3D12_RENDER_PASS_RENDER_TARGET_DESC, pdepthstencil: *const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, flags: D3D12_RENDER_PASS_FLAGS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginRenderPass(::core::mem::transmute_copy(&numrendertargets), ::core::mem::transmute_copy(&prendertargets), ::core::mem::transmute_copy(&pdepthstencil), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn EndRenderPass<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndRenderPass()
        }
        unsafe extern "system" fn InitializeMetaCommand<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetacommand: ::windows::core::RawPtr, pinitializationparametersdata: *const ::core::ffi::c_void, initializationparametersdatasizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeMetaCommand(::core::mem::transmute(&pmetacommand), ::core::mem::transmute_copy(&pinitializationparametersdata), ::core::mem::transmute_copy(&initializationparametersdatasizeinbytes))
        }
        unsafe extern "system" fn ExecuteMetaCommand<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetacommand: ::windows::core::RawPtr, pexecutionparametersdata: *const ::core::ffi::c_void, executionparametersdatasizeinbytes: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExecuteMetaCommand(::core::mem::transmute(&pmetacommand), ::core::mem::transmute_copy(&pexecutionparametersdata), ::core::mem::transmute_copy(&executionparametersdatasizeinbytes))
        }
        unsafe extern "system" fn BuildRaytracingAccelerationStructure<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, numpostbuildinfodescs: u32, ppostbuildinfodescs: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BuildRaytracingAccelerationStructure(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&numpostbuildinfodescs), ::core::mem::transmute_copy(&ppostbuildinfodescs))
        }
        unsafe extern "system" fn EmitRaytracingAccelerationStructurePostbuildInfo<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, numsourceaccelerationstructures: u32, psourceaccelerationstructuredata: *const u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EmitRaytracingAccelerationStructurePostbuildInfo(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&numsourceaccelerationstructures), ::core::mem::transmute_copy(&psourceaccelerationstructuredata))
        }
        unsafe extern "system" fn CopyRaytracingAccelerationStructure<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyRaytracingAccelerationStructure(::core::mem::transmute_copy(&destaccelerationstructuredata), ::core::mem::transmute_copy(&sourceaccelerationstructuredata), ::core::mem::transmute_copy(&mode))
        }
        unsafe extern "system" fn SetPipelineState1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateobject: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPipelineState1(::core::mem::transmute(&pstateobject))
        }
        unsafe extern "system" fn DispatchRays<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_DISPATCH_RAYS_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DispatchRays(::core::mem::transmute_copy(&pdesc))
        }
        Self {
            base: ID3D12GraphicsCommandList3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRenderPass: BeginRenderPass::<Identity, Impl, OFFSET>,
            EndRenderPass: EndRenderPass::<Identity, Impl, OFFSET>,
            InitializeMetaCommand: InitializeMetaCommand::<Identity, Impl, OFFSET>,
            ExecuteMetaCommand: ExecuteMetaCommand::<Identity, Impl, OFFSET>,
            BuildRaytracingAccelerationStructure: BuildRaytracingAccelerationStructure::<Identity, Impl, OFFSET>,
            EmitRaytracingAccelerationStructurePostbuildInfo: EmitRaytracingAccelerationStructurePostbuildInfo::<Identity, Impl, OFFSET>,
            CopyRaytracingAccelerationStructure: CopyRaytracingAccelerationStructure::<Identity, Impl, OFFSET>,
            SetPipelineState1: SetPipelineState1::<Identity, Impl, OFFSET>,
            DispatchRays: DispatchRays::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList5_Vtbl {
        unsafe extern "system" fn RSSetShadingRate<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseshadingrate: D3D12_SHADING_RATE, combiners: *const D3D12_SHADING_RATE_COMBINER) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RSSetShadingRate(::core::mem::transmute_copy(&baseshadingrate), ::core::mem::transmute_copy(&combiners))
        }
        unsafe extern "system" fn RSSetShadingRateImage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shadingrateimage: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RSSetShadingRateImage(::core::mem::transmute(&shadingrateimage))
        }
        Self {
            base: ID3D12GraphicsCommandList4_Vtbl::new::<Identity, Impl, OFFSET>(),
            RSSetShadingRate: RSSetShadingRate::<Identity, Impl, OFFSET>,
            RSSetShadingRateImage: RSSetShadingRateImage::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList6_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList6_Vtbl {
        unsafe extern "system" fn DispatchMesh<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12GraphicsCommandList6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DispatchMesh(::core::mem::transmute_copy(&threadgroupcountx), ::core::mem::transmute_copy(&threadgroupcounty), ::core::mem::transmute_copy(&threadgroupcountz))
        }
        Self { base: ID3D12GraphicsCommandList5_Vtbl::new::<Identity, Impl, OFFSET>(), DispatchMesh: DispatchMesh::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Heap_Impl, const OFFSET: isize>() -> ID3D12Heap_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Heap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_HEAP_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Heap1_Impl, const OFFSET: isize>() -> ID3D12Heap1_Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Heap1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProtectedResourceSession(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppprotectedsession)).into()
        }
        Self { base: ID3D12Heap_Vtbl::new::<Identity, Impl, OFFSET>(), GetProtectedResourceSession: GetProtectedResourceSession::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>() -> ID3D12InfoQueue_Vtbl {
        unsafe extern "system" fn SetMessageCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMessageCountLimit(::core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearStoredMessages()
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D12_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMessage(::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumMessagesAllowedByStorageFilter()
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumMessagesDeniedByStorageFilter()
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumStoredMessages()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumStoredMessagesAllowedByRetrievalFilter()
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumMessagesDiscardedByMessageCountLimit()
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMessageCountLimit()
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStorageFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStorageFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearStorageFilter()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushEmptyStorageFilter().into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushCopyOfStorageFilter().into()
        }
        unsafe extern "system" fn PushStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushStorageFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PopStorageFilter()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStorageFilterStackSize()
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRetrievalFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRetrievalFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearRetrievalFilter()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushEmptyRetrievalFilter().into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushCopyOfRetrievalFilter().into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushRetrievalFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PopRetrievalFilter()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRetrievalFilterStackSize()
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMessage(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddApplicationMessage(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBreakOnCategory(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBreakOnSeverity(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D12_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBreakOnID(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBreakOnCategory(::core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBreakOnSeverity(::core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBreakOnID(::core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMuteDebugOutput(::core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMuteDebugOutput()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, Impl, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, Impl, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, Impl, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, Impl, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, Impl, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, Impl, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, Impl, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, Impl, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, Impl, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, Impl, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, Impl, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, Impl, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, Impl, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, Impl, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, Impl, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, Impl, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, Impl, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, Impl, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, Impl, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, Impl, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, Impl, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, Impl, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, Impl, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, Impl, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, Impl, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue1_Impl, const OFFSET: isize>() -> ID3D12InfoQueue1_Vtbl {
        unsafe extern "system" fn RegisterMessageCallback<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackfunc: ::windows::core::RawPtr, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *const ::core::ffi::c_void, pcallbackcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterMessageCallback(::core::mem::transmute_copy(&callbackfunc), ::core::mem::transmute_copy(&callbackfilterflags), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&pcallbackcookie)).into()
        }
        unsafe extern "system" fn UnregisterMessageCallback<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12InfoQueue1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterMessageCallback(::core::mem::transmute_copy(&callbackcookie)).into()
        }
        Self {
            base: ID3D12InfoQueue_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterMessageCallback: RegisterMessageCallback::<Identity, Impl, OFFSET>,
            UnregisterMessageCallback: UnregisterMessageCallback::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LibraryReflection_Impl, const OFFSET: isize>() -> ID3D12LibraryReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_LIBRARY_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionindex: i32) -> ::core::option::Option<ID3D12FunctionReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFunctionByIndex(::core::mem::transmute_copy(&functionindex))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetFunctionByIndex: GetFunctionByIndex::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LifetimeOwner_Impl, const OFFSET: isize>() -> ID3D12LifetimeOwner_Vtbl {
        unsafe extern "system" fn LifetimeStateUpdated<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LifetimeOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: D3D12_LIFETIME_STATE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LifetimeStateUpdated(::core::mem::transmute_copy(&newstate))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), LifetimeStateUpdated: LifetimeStateUpdated::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LifetimeTracker_Impl, const OFFSET: isize>() -> ID3D12LifetimeTracker_Vtbl {
        unsafe extern "system" fn DestroyOwnedObject<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12LifetimeTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestroyOwnedObject(::core::mem::transmute(&pobject)).into()
        }
        Self { base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), DestroyOwnedObject: DestroyOwnedObject::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12MetaCommand_Impl, const OFFSET: isize>() -> ID3D12MetaCommand_Vtbl {
        unsafe extern "system" fn GetRequiredParameterResourceSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12MetaCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRequiredParameterResourceSize(::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&parameterindex))
        }
        Self {
            base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRequiredParameterResourceSize: GetRequiredParameterResourceSize::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Object_Impl, const OFFSET: isize>() -> ID3D12Object_Vtbl {
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Pageable_Impl, const OFFSET: isize>() -> ID3D12Pageable_Vtbl {
        Self { base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>() -> ID3D12PipelineLibrary_Vtbl {
        unsafe extern "system" fn StorePipeline<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, ppipeline: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StorePipeline(::core::mem::transmute_copy(&pname), ::core::mem::transmute(&ppipeline)).into()
        }
        unsafe extern "system" fn LoadGraphicsPipeline<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadGraphicsPipeline(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn LoadComputePipeline<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadComputePipeline(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn GetSerializedSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSerializedSize()
        }
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasizeinbytes: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasizeinbytes)).into()
        }
        Self {
            base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            StorePipeline: StorePipeline::<Identity, Impl, OFFSET>,
            LoadGraphicsPipeline: LoadGraphicsPipeline::<Identity, Impl, OFFSET>,
            LoadComputePipeline: LoadComputePipeline::<Identity, Impl, OFFSET>,
            GetSerializedSize: GetSerializedSize::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary1_Impl, const OFFSET: isize>() -> ID3D12PipelineLibrary1_Vtbl {
        unsafe extern "system" fn LoadPipeline<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineLibrary1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadPipeline(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into()
        }
        Self { base: ID3D12PipelineLibrary_Vtbl::new::<Identity, Impl, OFFSET>(), LoadPipeline: LoadPipeline::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineState_Impl, const OFFSET: isize>() -> ID3D12PipelineState_Vtbl {
        unsafe extern "system" fn GetCachedBlob<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12PipelineState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedBlob() {
                ::core::result::Result::Ok(ok__) => {
                    *ppblob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(), GetCachedBlob: GetCachedBlob::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedResourceSession_Impl, const OFFSET: isize>() -> ID3D12ProtectedResourceSession_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedResourceSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        Self { base: ID3D12ProtectedSession_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedResourceSession1_Impl, const OFFSET: isize>() -> ID3D12ProtectedResourceSession1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedResourceSession1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetDesc1()
        }
        Self { base: ID3D12ProtectedResourceSession_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedSession_Impl, const OFFSET: isize>() -> ID3D12ProtectedSession_Vtbl {
        unsafe extern "system" fn GetStatusFence<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatusFence(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppfence)).into()
        }
        unsafe extern "system" fn GetSessionStatus<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ProtectedSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D12_PROTECTED_SESSION_STATUS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSessionStatus()
        }
        Self {
            base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStatusFence: GetStatusFence::<Identity, Impl, OFFSET>,
            GetSessionStatus: GetSessionStatus::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12QueryHeap_Impl, const OFFSET: isize>() -> ID3D12QueryHeap_Vtbl {
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>() }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource_Impl, const OFFSET: isize>() -> ID3D12Resource_Vtbl {
        unsafe extern "system" fn Map<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, preadrange: *const D3D12_RANGE, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Map(::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&preadrange), ::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn Unmap<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subresource: u32, pwrittenrange: *const D3D12_RANGE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unmap(::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&pwrittenrange))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        unsafe extern "system" fn GetGPUVirtualAddress<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGPUVirtualAddress()
        }
        unsafe extern "system" fn WriteToSubresource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D12_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteToSubresource(::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch)).into()
        }
        unsafe extern "system" fn ReadFromSubresource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: *const D3D12_BOX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadFromSubresource(::core::mem::transmute_copy(&pdstdata), ::core::mem::transmute_copy(&dstrowpitch), ::core::mem::transmute_copy(&dstdepthpitch), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox)).into()
        }
        unsafe extern "system" fn GetHeapProperties<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheapproperties: *mut D3D12_HEAP_PROPERTIES, pheapflags: *mut D3D12_HEAP_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHeapProperties(::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&pheapflags)).into()
        }
        Self {
            base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(),
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetGPUVirtualAddress: GetGPUVirtualAddress::<Identity, Impl, OFFSET>,
            WriteToSubresource: WriteToSubresource::<Identity, Impl, OFFSET>,
            ReadFromSubresource: ReadFromSubresource::<Identity, Impl, OFFSET>,
            GetHeapProperties: GetHeapProperties::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource1_Impl, const OFFSET: isize>() -> ID3D12Resource1_Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProtectedResourceSession(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppprotectedsession)).into()
        }
        Self { base: ID3D12Resource_Vtbl::new::<Identity, Impl, OFFSET>(), GetProtectedResourceSession: GetProtectedResourceSession::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource2_Impl, const OFFSET: isize>() -> ID3D12Resource2_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Resource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetDesc1()
        }
        Self { base: ID3D12Resource1_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12Resource2 as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D12Pageable as ::windows::core::Interface>::IID || iid == &<ID3D12Resource as ::windows::core::Interface>::IID || iid == &<ID3D12Resource1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12RootSignature_Impl: Sized + ID3D12Object_Impl + ID3D12DeviceChild_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ID3D12RootSignature_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12RootSignature_Impl, const OFFSET: isize>() -> ID3D12RootSignature_Vtbl {
        Self { base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12RootSignature as ::windows::core::Interface>::IID || iid == &<ID3D12Object as ::windows::core::Interface>::IID || iid == &<ID3D12DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D12RootSignatureDeserializer_Impl: Sized {
    fn GetRootSignatureDesc(&mut self) -> *mut D3D12_ROOT_SIGNATURE_DESC;
}
impl ID3D12RootSignatureDeserializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12RootSignatureDeserializer_Impl, const OFFSET: isize>() -> ID3D12RootSignatureDeserializer_Vtbl {
        unsafe extern "system" fn GetRootSignatureDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12RootSignatureDeserializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut D3D12_ROOT_SIGNATURE_DESC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRootSignatureDesc()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetRootSignatureDesc: GetRootSignatureDesc::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SDKConfiguration_Impl, const OFFSET: isize>() -> ID3D12SDKConfiguration_Vtbl {
        unsafe extern "system" fn SetSDKVersion<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SDKConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sdkversion: u32, sdkpath: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSDKVersion(::core::mem::transmute_copy(&sdkversion), ::core::mem::transmute_copy(&sdkpath)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetSDKVersion: SetSDKVersion::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>() -> ID3D12ShaderCacheSession_Vtbl {
        unsafe extern "system" fn FindValue<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *mut ::core::ffi::c_void, pvaluesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindValue(::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&keysize), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pvaluesize)).into()
        }
        unsafe extern "system" fn StoreValue<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StoreValue(::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&keysize), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&valuesize)).into()
        }
        unsafe extern "system" fn SetDeleteOnDestroy<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDeleteOnDestroy()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_SHADER_CACHE_SESSION_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetDesc()
        }
        Self {
            base: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            FindValue: FindValue::<Identity, Impl, OFFSET>,
            StoreValue: StoreValue::<Identity, Impl, OFFSET>,
            SetDeleteOnDestroy: SetDeleteOnDestroy::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>() -> ID3D12ShaderReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPatchConstantParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResourceBindingDescByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMovInstructionCount()
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMovcInstructionCount()
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversionInstructionCount()
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBitwiseInstructionCount()
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGSInputPrimitive()
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsSampleFrequencyShader()
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumInterfaceSlots()
        }
        unsafe extern "system" fn GetMinFeatureLevel<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMinFeatureLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadGroupSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetThreadGroupSize(::core::mem::transmute_copy(&psizex), ::core::mem::transmute_copy(&psizey), ::core::mem::transmute_copy(&psizez))
        }
        unsafe extern "system" fn GetRequiresFlags<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRequiresFlags()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, Impl, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, Impl, OFFSET>,
            GetPatchConstantParameterDesc: GetPatchConstantParameterDesc::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, Impl, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, Impl, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, Impl, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, Impl, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, Impl, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, Impl, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, Impl, OFFSET>,
            GetNumInterfaceSlots: GetNumInterfaceSlots::<Identity, Impl, OFFSET>,
            GetMinFeatureLevel: GetMinFeatureLevel::<Identity, Impl, OFFSET>,
            GetThreadGroupSize: GetThreadGroupSize::<Identity, Impl, OFFSET>,
            GetRequiresFlags: GetRequiresFlags::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>() -> ID3D12ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVariableByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        Self {
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>() -> ID3D12ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMemberTypeByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMemberTypeByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMemberTypeName(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsEqual(::core::mem::transmute(&ptype)).into()
        }
        unsafe extern "system" fn GetSubType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSubType()
        }
        unsafe extern "system" fn GetBaseClass<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBaseClass()
        }
        unsafe extern "system" fn GetNumInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumInterfaces()
        }
        unsafe extern "system" fn GetInterfaceByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInterfaceByIndex(::core::mem::transmute_copy(&uindex))
        }
        unsafe extern "system" fn IsOfType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsOfType(::core::mem::transmute(&ptype)).into()
        }
        unsafe extern "system" fn ImplementsInterface<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbase: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ImplementsInterface(::core::mem::transmute(&pbase)).into()
        }
        Self {
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Identity, Impl, OFFSET>,
            GetMemberTypeByName: GetMemberTypeByName::<Identity, Impl, OFFSET>,
            GetMemberTypeName: GetMemberTypeName::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            GetSubType: GetSubType::<Identity, Impl, OFFSET>,
            GetBaseClass: GetBaseClass::<Identity, Impl, OFFSET>,
            GetNumInterfaces: GetNumInterfaces::<Identity, Impl, OFFSET>,
            GetInterfaceByIndex: GetInterfaceByIndex::<Identity, Impl, OFFSET>,
            IsOfType: IsOfType::<Identity, Impl, OFFSET>,
            ImplementsInterface: ImplementsInterface::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionVariable_Impl, const OFFSET: isize>() -> ID3D12ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetType()
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBuffer()
        }
        unsafe extern "system" fn GetInterfaceSlot<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInterfaceSlot(::core::mem::transmute_copy(&uarrayindex))
        }
        Self {
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            GetInterfaceSlot: GetInterfaceSlot::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SharingContract_Impl, const OFFSET: isize>() -> ID3D12SharingContract_Vtbl {
        unsafe extern "system" fn Present<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, window: super::super::Foundation::HWND) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Present(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&window))
        }
        unsafe extern "system" fn SharedFenceSignal<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, fencevalue: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SharedFenceSignal(::core::mem::transmute(&pfence), ::core::mem::transmute_copy(&fencevalue))
        }
        unsafe extern "system" fn BeginCapturableWork<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginCapturableWork(::core::mem::transmute_copy(&guid))
        }
        unsafe extern "system" fn EndCapturableWork<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndCapturableWork(::core::mem::transmute_copy(&guid))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, Impl, OFFSET>,
            SharedFenceSignal: SharedFenceSignal::<Identity, Impl, OFFSET>,
            BeginCapturableWork: BeginCapturableWork::<Identity, Impl, OFFSET>,
            EndCapturableWork: EndCapturableWork::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12StateObject_Impl, const OFFSET: isize>() -> ID3D12StateObject_Vtbl {
        Self { base: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>() }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>() -> ID3D12StateObjectProperties_Vtbl {
        unsafe extern "system" fn GetShaderIdentifier<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexportname: super::super::Foundation::PWSTR) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetShaderIdentifier(::core::mem::transmute_copy(&pexportname))
        }
        unsafe extern "system" fn GetShaderStackSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexportname: super::super::Foundation::PWSTR) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetShaderStackSize(::core::mem::transmute_copy(&pexportname))
        }
        unsafe extern "system" fn GetPipelineStackSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPipelineStackSize()
        }
        unsafe extern "system" fn SetPipelineStackSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipelinestacksizeinbytes: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPipelineStackSize(::core::mem::transmute_copy(&pipelinestacksizeinbytes))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetShaderIdentifier: GetShaderIdentifier::<Identity, Impl, OFFSET>,
            GetShaderStackSize: GetShaderStackSize::<Identity, Impl, OFFSET>,
            GetPipelineStackSize: GetPipelineStackSize::<Identity, Impl, OFFSET>,
            SetPipelineStackSize: SetPipelineStackSize::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>() -> ID3D12SwapChainAssistant_Vtbl {
        unsafe extern "system" fn GetLUID<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetLUID()
        }
        unsafe extern "system" fn GetSwapChainObject<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSwapChainObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetCurrentResourceAndCommandQueue<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void, riidqueue: *const ::windows::core::GUID, ppvqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentResourceAndCommandQueue(::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource), ::core::mem::transmute_copy(&riidqueue), ::core::mem::transmute_copy(&ppvqueue)).into()
        }
        unsafe extern "system" fn InsertImplicitSync<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertImplicitSync().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLUID: GetLUID::<Identity, Impl, OFFSET>,
            GetSwapChainObject: GetSwapChainObject::<Identity, Impl, OFFSET>,
            GetCurrentResourceAndCommandQueue: GetCurrentResourceAndCommandQueue::<Identity, Impl, OFFSET>,
            InsertImplicitSync: InsertImplicitSync::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Tools_Impl, const OFFSET: isize>() -> ID3D12Tools_Vtbl {
        unsafe extern "system" fn EnableShaderInstrumentation<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Tools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableShaderInstrumentation(::core::mem::transmute_copy(&benable))
        }
        unsafe extern "system" fn ShaderInstrumentationEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12Tools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShaderInstrumentationEnabled()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnableShaderInstrumentation: EnableShaderInstrumentation::<Identity, Impl, OFFSET>,
            ShaderInstrumentationEnabled: ShaderInstrumentationEnabled::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: isize>() -> ID3D12VersionedRootSignatureDeserializer_Vtbl {
        unsafe extern "system" fn GetRootSignatureDescAtVersion<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, converttoversion: D3D_ROOT_SIGNATURE_VERSION, ppdesc: *mut *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRootSignatureDescAtVersion(::core::mem::transmute_copy(&converttoversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnconvertedRootSignatureDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUnconvertedRootSignatureDesc()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRootSignatureDescAtVersion: GetRootSignatureDescAtVersion::<Identity, Impl, OFFSET>,
            GetUnconvertedRootSignatureDesc: GetUnconvertedRootSignatureDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D12VersionedRootSignatureDeserializer as ::windows::core::Interface>::IID
    }
}
