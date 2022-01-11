#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
pub trait IDisplayAdapterImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<super::super::super::Graphics::DisplayAdapterId>;
    fn DeviceInterfacePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceCount(&self) -> ::windows::core::Result<u32>;
    fn PciVendorId(&self) -> ::windows::core::Result<u32>;
    fn PciDeviceId(&self) -> ::windows::core::Result<u32>;
    fn PciSubSystemId(&self) -> ::windows::core::Result<u32>;
    fn PciRevision(&self) -> ::windows::core::Result<u32>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayAdapter {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayAdapter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl IDisplayAdapterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayAdapterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayAdapterVtbl {
        unsafe extern "system" fn Id<Impl: IDisplayAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceInterfacePath<Impl: IDisplayAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceInterfacePath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceCount<Impl: IDisplayAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PciVendorId<Impl: IDisplayAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PciVendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PciDeviceId<Impl: IDisplayAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PciDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PciSubSystemId<Impl: IDisplayAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PciSubSystemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PciRevision<Impl: IDisplayAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PciRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IDisplayAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDisplayAdapter>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, IMPL_OFFSET>,
            DeviceInterfacePath::<Impl, IMPL_OFFSET>,
            SourceCount::<Impl, IMPL_OFFSET>,
            PciVendorId::<Impl, IMPL_OFFSET>,
            PciDeviceId::<Impl, IMPL_OFFSET>,
            PciSubSystemId::<Impl, IMPL_OFFSET>,
            PciRevision::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayAdapter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
pub trait IDisplayAdapterStaticsImpl: Sized {
    fn FromId(&self, id: &super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::Result<DisplayAdapter>;
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayAdapterStatics {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayAdapterStatics";
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl IDisplayAdapterStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayAdapterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayAdapterStaticsVtbl {
        unsafe extern "system" fn FromId<Impl: IDisplayAdapterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: super::super::super::Graphics::DisplayAdapterId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromId(&*(&id as *const <super::super::super::Graphics::DisplayAdapterId as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::DisplayAdapterId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayAdapterStatics>, ::windows::core::GetTrustLevel, FromId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayAdapterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayDeviceImpl: Sized {
    fn CreateScanoutSource(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplaySource>;
    fn CreatePrimary(&self, target: &::core::option::Option<DisplayTarget>, desc: &::core::option::Option<DisplayPrimaryDescription>) -> ::windows::core::Result<DisplaySurface>;
    fn CreateTaskPool(&self) -> ::windows::core::Result<DisplayTaskPool>;
    fn CreatePeriodicFence(&self, target: &::core::option::Option<DisplayTarget>, offsetfromvblank: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<DisplayFence>;
    fn WaitForVBlank(&self, source: &::core::option::Option<DisplaySource>) -> ::windows::core::Result<()>;
    fn CreateSimpleScanout(&self, psource: &::core::option::Option<DisplaySource>, psurface: &::core::option::Option<DisplaySurface>, subresourceindex: u32, syncinterval: u32) -> ::windows::core::Result<DisplayScanout>;
    fn IsCapabilitySupported(&self, capability: DisplayDeviceCapability) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayDevice {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayDeviceVtbl {
        unsafe extern "system" fn CreateScanoutSource<Impl: IDisplayDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScanoutSource(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrimary<Impl: IDisplayDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, desc: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePrimary(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType), &*(&desc as *const <DisplayPrimaryDescription as ::windows::core::Abi>::Abi as *const <DisplayPrimaryDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTaskPool<Impl: IDisplayDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTaskPool() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePeriodicFence<Impl: IDisplayDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, offsetfromvblank: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePeriodicFence(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType), &*(&offsetfromvblank as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForVBlank<Impl: IDisplayDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForVBlank(&*(&source as *const <DisplaySource as ::windows::core::Abi>::Abi as *const <DisplaySource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateSimpleScanout<Impl: IDisplayDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psurface: ::windows::core::RawPtr, subresourceindex: u32, syncinterval: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSimpleScanout(&*(&psource as *const <DisplaySource as ::windows::core::Abi>::Abi as *const <DisplaySource as ::windows::core::DefaultType>::DefaultType), &*(&psurface as *const <DisplaySurface as ::windows::core::Abi>::Abi as *const <DisplaySurface as ::windows::core::DefaultType>::DefaultType), subresourceindex, syncinterval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCapabilitySupported<Impl: IDisplayDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capability: DisplayDeviceCapability, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCapabilitySupported(capability) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDisplayDevice>,
            ::windows::core::GetTrustLevel,
            CreateScanoutSource::<Impl, IMPL_OFFSET>,
            CreatePrimary::<Impl, IMPL_OFFSET>,
            CreateTaskPool::<Impl, IMPL_OFFSET>,
            CreatePeriodicFence::<Impl, IMPL_OFFSET>,
            WaitForVBlank::<Impl, IMPL_OFFSET>,
            CreateSimpleScanout::<Impl, IMPL_OFFSET>,
            IsCapabilitySupported::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
pub trait IDisplayDevice2Impl: Sized {
    fn CreateSimpleScanoutWithDirtyRectsAndOptions(&self, source: &::core::option::Option<DisplaySource>, surface: &::core::option::Option<DisplaySurface>, subresourceindex: u32, syncinterval: u32, dirtyrects: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Graphics::RectInt32>>, options: DisplayScanoutOptions) -> ::windows::core::Result<DisplayScanout>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayDevice2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayDevice2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl IDisplayDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayDevice2Vtbl {
        unsafe extern "system" fn CreateSimpleScanoutWithDirtyRectsAndOptions<Impl: IDisplayDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, surface: ::windows::core::RawPtr, subresourceindex: u32, syncinterval: u32, dirtyrects: ::windows::core::RawPtr, options: DisplayScanoutOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSimpleScanoutWithDirtyRectsAndOptions(
                &*(&source as *const <DisplaySource as ::windows::core::Abi>::Abi as *const <DisplaySource as ::windows::core::DefaultType>::DefaultType),
                &*(&surface as *const <DisplaySurface as ::windows::core::Abi>::Abi as *const <DisplaySurface as ::windows::core::DefaultType>::DefaultType),
                subresourceindex,
                syncinterval,
                &*(&dirtyrects as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Graphics::RectInt32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Graphics::RectInt32> as ::windows::core::DefaultType>::DefaultType),
                options,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayDevice2>, ::windows::core::GetTrustLevel, CreateSimpleScanoutWithDirtyRectsAndOptions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayFenceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayFence {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayFence";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayFenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayFenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayFenceVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayFence>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayFence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayManagerImpl: Sized {
    fn GetCurrentTargets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>>;
    fn GetCurrentAdapters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayAdapter>>;
    fn TryAcquireTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayManagerResult>;
    fn ReleaseTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<()>;
    fn TryReadCurrentStateForAllTargets(&self) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn TryAcquireTargetsAndReadCurrentState(&self, targets: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn TryAcquireTargetsAndCreateEmptyState(&self, targets: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn TryAcquireTargetsAndCreateSubstate(&self, existingstate: &::core::option::Option<DisplayState>, targets: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn CreateDisplayDevice(&self, adapter: &::core::option::Option<DisplayAdapter>) -> ::windows::core::Result<DisplayDevice>;
    fn Enabled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerEnabledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnabled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Disabled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerDisabledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisabled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Changed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PathsFailedOrInvalidated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerPathsFailedOrInvalidatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePathsFailedOrInvalidated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayManager {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerVtbl {
        unsafe extern "system" fn GetCurrentTargets<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentTargets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentAdapters<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentAdapters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryAcquireTarget<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut DisplayManagerResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryAcquireTarget(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseTarget<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseTarget(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryReadCurrentStateForAllTargets<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryReadCurrentStateForAllTargets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryAcquireTargetsAndReadCurrentState<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryAcquireTargetsAndReadCurrentState(&*(&targets as *const <super::super::super::Foundation::Collections::IIterable<DisplayTarget> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<DisplayTarget> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryAcquireTargetsAndCreateEmptyState<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryAcquireTargetsAndCreateEmptyState(&*(&targets as *const <super::super::super::Foundation::Collections::IIterable<DisplayTarget> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<DisplayTarget> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryAcquireTargetsAndCreateSubstate<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, existingstate: ::windows::core::RawPtr, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryAcquireTargetsAndCreateSubstate(&*(&existingstate as *const <DisplayState as ::windows::core::Abi>::Abi as *const <DisplayState as ::windows::core::DefaultType>::DefaultType), &*(&targets as *const <super::super::super::Foundation::Collections::IIterable<DisplayTarget> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<DisplayTarget> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDisplayDevice<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDisplayDevice(&*(&adapter as *const <DisplayAdapter as ::windows::core::Abi>::Abi as *const <DisplayAdapter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerEnabledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerEnabledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnabled<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnabled(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disabled<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disabled(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerDisabledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerDisabledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisabled<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisabled(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Changed<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PathsFailedOrInvalidated<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathsFailedOrInvalidated(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerPathsFailedOrInvalidatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerPathsFailedOrInvalidatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePathsFailedOrInvalidated<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePathsFailedOrInvalidated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IDisplayManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDisplayManager>,
            ::windows::core::GetTrustLevel,
            GetCurrentTargets::<Impl, IMPL_OFFSET>,
            GetCurrentAdapters::<Impl, IMPL_OFFSET>,
            TryAcquireTarget::<Impl, IMPL_OFFSET>,
            ReleaseTarget::<Impl, IMPL_OFFSET>,
            TryReadCurrentStateForAllTargets::<Impl, IMPL_OFFSET>,
            TryAcquireTargetsAndReadCurrentState::<Impl, IMPL_OFFSET>,
            TryAcquireTargetsAndCreateEmptyState::<Impl, IMPL_OFFSET>,
            TryAcquireTargetsAndCreateSubstate::<Impl, IMPL_OFFSET>,
            CreateDisplayDevice::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            RemoveEnabled::<Impl, IMPL_OFFSET>,
            Disabled::<Impl, IMPL_OFFSET>,
            RemoveDisabled::<Impl, IMPL_OFFSET>,
            Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged::<Impl, IMPL_OFFSET>,
            PathsFailedOrInvalidated::<Impl, IMPL_OFFSET>,
            RemovePathsFailedOrInvalidated::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayManagerChangedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayManagerChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayManagerChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerChangedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IDisplayManagerChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IDisplayManagerChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDisplayManagerChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayManagerChangedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayManagerDisabledEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayManagerDisabledEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerDisabledEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayManagerDisabledEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerDisabledEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerDisabledEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IDisplayManagerDisabledEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IDisplayManagerDisabledEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDisplayManagerDisabledEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayManagerDisabledEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerDisabledEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayManagerEnabledEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayManagerEnabledEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerEnabledEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayManagerEnabledEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerEnabledEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerEnabledEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IDisplayManagerEnabledEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IDisplayManagerEnabledEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDisplayManagerEnabledEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayManagerEnabledEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerEnabledEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayManagerPathsFailedOrInvalidatedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayManagerPathsFailedOrInvalidatedEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerPathsFailedOrInvalidatedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayManagerPathsFailedOrInvalidatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerPathsFailedOrInvalidatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerPathsFailedOrInvalidatedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IDisplayManagerPathsFailedOrInvalidatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IDisplayManagerPathsFailedOrInvalidatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDisplayManagerPathsFailedOrInvalidatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayManagerPathsFailedOrInvalidatedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerPathsFailedOrInvalidatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerResultWithStateImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<DisplayManagerResult>;
    fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn State(&self) -> ::windows::core::Result<DisplayState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayManagerResultWithState {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerResultWithState";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayManagerResultWithStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerResultWithStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerResultWithStateVtbl {
        unsafe extern "system" fn ErrorCode<Impl: IDisplayManagerResultWithStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayManagerResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedErrorCode<Impl: IDisplayManagerResultWithStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IDisplayManagerResultWithStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayManagerResultWithState>, ::windows::core::GetTrustLevel, ErrorCode::<Impl, IMPL_OFFSET>, ExtendedErrorCode::<Impl, IMPL_OFFSET>, State::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerResultWithState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerStaticsImpl: Sized {
    fn Create(&self, options: DisplayManagerOptions) -> ::windows::core::Result<DisplayManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayManagerStatics {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IDisplayManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: DisplayManagerOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayManagerStatics>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait IDisplayModeInfoImpl: Sized {
    fn SourceResolution(&self) -> ::windows::core::Result<super::super::super::Graphics::SizeInt32>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn SourcePixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn TargetResolution(&self) -> ::windows::core::Result<super::super::super::Graphics::SizeInt32>;
    fn PresentationRate(&self) -> ::windows::core::Result<DisplayPresentationRate>;
    fn IsInterlaced(&self) -> ::windows::core::Result<bool>;
    fn GetWireFormatSupportedBitsPerChannel(&self, encoding: DisplayWireFormatPixelEncoding) -> ::windows::core::Result<DisplayBitsPerChannel>;
    fn IsWireFormatSupported(&self, wireformat: &::core::option::Option<DisplayWireFormat>) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayModeInfo {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayModeInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl IDisplayModeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayModeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayModeInfoVtbl {
        unsafe extern "system" fn SourceResolution<Impl: IDisplayModeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStereo<Impl: IDisplayModeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStereo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePixelFormat<Impl: IDisplayModeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetResolution<Impl: IDisplayModeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationRate<Impl: IDisplayModeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayPresentationRate) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInterlaced<Impl: IDisplayModeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInterlaced() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWireFormatSupportedBitsPerChannel<Impl: IDisplayModeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: DisplayWireFormatPixelEncoding, result__: *mut DisplayBitsPerChannel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWireFormatSupportedBitsPerChannel(encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWireFormatSupported<Impl: IDisplayModeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wireformat: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWireFormatSupported(&*(&wireformat as *const <DisplayWireFormat as ::windows::core::Abi>::Abi as *const <DisplayWireFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IDisplayModeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDisplayModeInfo>,
            ::windows::core::GetTrustLevel,
            SourceResolution::<Impl, IMPL_OFFSET>,
            IsStereo::<Impl, IMPL_OFFSET>,
            SourcePixelFormat::<Impl, IMPL_OFFSET>,
            TargetResolution::<Impl, IMPL_OFFSET>,
            PresentationRate::<Impl, IMPL_OFFSET>,
            IsInterlaced::<Impl, IMPL_OFFSET>,
            GetWireFormatSupportedBitsPerChannel::<Impl, IMPL_OFFSET>,
            IsWireFormatSupported::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayModeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IDisplayModeInfo2Impl: Sized {
    fn PhysicalPresentationRate(&self) -> ::windows::core::Result<DisplayPresentationRate>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayModeInfo2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayModeInfo2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IDisplayModeInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayModeInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayModeInfo2Vtbl {
        unsafe extern "system" fn PhysicalPresentationRate<Impl: IDisplayModeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayPresentationRate) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalPresentationRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayModeInfo2>, ::windows::core::GetTrustLevel, PhysicalPresentationRate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayModeInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait IDisplayPathImpl: Sized {
    fn View(&self) -> ::windows::core::Result<DisplayView>;
    fn Target(&self) -> ::windows::core::Result<DisplayTarget>;
    fn Status(&self) -> ::windows::core::Result<DisplayPathStatus>;
    fn SourceResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>;
    fn SetSourceResolution(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>) -> ::windows::core::Result<()>;
    fn SourcePixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SetSourcePixelFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn SetIsStereo(&self, value: bool) -> ::windows::core::Result<()>;
    fn TargetResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>;
    fn SetTargetResolution(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>) -> ::windows::core::Result<()>;
    fn PresentationRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>>;
    fn SetPresentationRate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<DisplayPresentationRate>>) -> ::windows::core::Result<()>;
    fn IsInterlaced(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<bool>>;
    fn SetIsInterlaced(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn WireFormat(&self) -> ::windows::core::Result<DisplayWireFormat>;
    fn SetWireFormat(&self, value: &::core::option::Option<DisplayWireFormat>) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<DisplayRotation>;
    fn SetRotation(&self, value: DisplayRotation) -> ::windows::core::Result<()>;
    fn Scaling(&self) -> ::windows::core::Result<DisplayPathScaling>;
    fn SetScaling(&self, value: DisplayPathScaling) -> ::windows::core::Result<()>;
    fn FindModes(&self, flags: DisplayModeQueryOptions) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayModeInfo>>;
    fn ApplyPropertiesFromMode(&self, moderesult: &::core::option::Option<DisplayModeInfo>) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPath {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayPath";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl IDisplayPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPathVtbl {
        unsafe extern "system" fn View<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).View() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Target<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayPathStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceResolution<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceResolution<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceResolution(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourcePixelFormat<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourcePixelFormat<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourcePixelFormat(value).into()
        }
        unsafe extern "system" fn IsStereo<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStereo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStereo<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStereo(value).into()
        }
        unsafe extern "system" fn TargetResolution<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetResolution<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetResolution(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PresentationRate<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentationRate<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresentationRate(&*(&value as *const <super::super::super::Foundation::IReference<DisplayPresentationRate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<DisplayPresentationRate> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsInterlaced<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInterlaced() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInterlaced<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInterlaced(&*(&value as *const <super::super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WireFormat<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WireFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWireFormat<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWireFormat(&*(&value as *const <DisplayWireFormat as ::windows::core::Abi>::Abi as *const <DisplayWireFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Rotation<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DisplayRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn Scaling<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayPathScaling) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scaling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaling<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DisplayPathScaling) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaling(value).into()
        }
        unsafe extern "system" fn FindModes<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: DisplayModeQueryOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindModes(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyPropertiesFromMode<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moderesult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyPropertiesFromMode(&*(&moderesult as *const <DisplayModeInfo as ::windows::core::Abi>::Abi as *const <DisplayModeInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IDisplayPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDisplayPath>,
            ::windows::core::GetTrustLevel,
            View::<Impl, IMPL_OFFSET>,
            Target::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            SourceResolution::<Impl, IMPL_OFFSET>,
            SetSourceResolution::<Impl, IMPL_OFFSET>,
            SourcePixelFormat::<Impl, IMPL_OFFSET>,
            SetSourcePixelFormat::<Impl, IMPL_OFFSET>,
            IsStereo::<Impl, IMPL_OFFSET>,
            SetIsStereo::<Impl, IMPL_OFFSET>,
            TargetResolution::<Impl, IMPL_OFFSET>,
            SetTargetResolution::<Impl, IMPL_OFFSET>,
            PresentationRate::<Impl, IMPL_OFFSET>,
            SetPresentationRate::<Impl, IMPL_OFFSET>,
            IsInterlaced::<Impl, IMPL_OFFSET>,
            SetIsInterlaced::<Impl, IMPL_OFFSET>,
            WireFormat::<Impl, IMPL_OFFSET>,
            SetWireFormat::<Impl, IMPL_OFFSET>,
            Rotation::<Impl, IMPL_OFFSET>,
            SetRotation::<Impl, IMPL_OFFSET>,
            Scaling::<Impl, IMPL_OFFSET>,
            SetScaling::<Impl, IMPL_OFFSET>,
            FindModes::<Impl, IMPL_OFFSET>,
            ApplyPropertiesFromMode::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IDisplayPath2Impl: Sized {
    fn PhysicalPresentationRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>>;
    fn SetPhysicalPresentationRate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<DisplayPresentationRate>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPath2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayPath2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IDisplayPath2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPath2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPath2Vtbl {
        unsafe extern "system" fn PhysicalPresentationRate<Impl: IDisplayPath2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalPresentationRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhysicalPresentationRate<Impl: IDisplayPath2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhysicalPresentationRate(&*(&value as *const <super::super::super::Foundation::IReference<DisplayPresentationRate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<DisplayPresentationRate> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayPath2>, ::windows::core::GetTrustLevel, PhysicalPresentationRate::<Impl, IMPL_OFFSET>, SetPhysicalPresentationRate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPath2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDisplayPrimaryDescriptionImpl: Sized {
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn ColorSpace(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXColorSpace>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn MultisampleDescription(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPrimaryDescription {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayPrimaryDescription";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDisplayPrimaryDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPrimaryDescriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPrimaryDescriptionVtbl {
        unsafe extern "system" fn Width<Impl: IDisplayPrimaryDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IDisplayPrimaryDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: IDisplayPrimaryDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorSpace<Impl: IDisplayPrimaryDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXColorSpace) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStereo<Impl: IDisplayPrimaryDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStereo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultisampleDescription<Impl: IDisplayPrimaryDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultisampleDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IDisplayPrimaryDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDisplayPrimaryDescription>,
            ::windows::core::GetTrustLevel,
            Width::<Impl, IMPL_OFFSET>,
            Height::<Impl, IMPL_OFFSET>,
            Format::<Impl, IMPL_OFFSET>,
            ColorSpace::<Impl, IMPL_OFFSET>,
            IsStereo::<Impl, IMPL_OFFSET>,
            MultisampleDescription::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPrimaryDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDisplayPrimaryDescriptionFactoryImpl: Sized {
    fn CreateInstance(&self, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: &super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows::core::Result<DisplayPrimaryDescription>;
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPrimaryDescriptionFactory {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayPrimaryDescriptionFactory";
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDisplayPrimaryDescriptionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPrimaryDescriptionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPrimaryDescriptionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDisplayPrimaryDescriptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(width, height, pixelformat, colorspace, isstereo, &*(&multisampledescription as *const <super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayPrimaryDescriptionFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPrimaryDescriptionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDisplayPrimaryDescriptionStaticsImpl: Sized {
    fn CreateWithProperties(&self, extraproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: &super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows::core::Result<DisplayPrimaryDescription>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPrimaryDescriptionStatics {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayPrimaryDescriptionStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDisplayPrimaryDescriptionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPrimaryDescriptionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPrimaryDescriptionStaticsVtbl {
        unsafe extern "system" fn CreateWithProperties<Impl: IDisplayPrimaryDescriptionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extraproperties: ::windows::core::RawPtr, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithProperties(
                &*(&extraproperties as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>> as ::windows::core::DefaultType>::DefaultType),
                width,
                height,
                pixelformat,
                colorspace,
                isstereo,
                &*(&multisampledescription as *const <super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayPrimaryDescriptionStatics>, ::windows::core::GetTrustLevel, CreateWithProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPrimaryDescriptionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayScanoutImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayScanout {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayScanout";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayScanoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayScanoutImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayScanoutVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayScanout>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayScanout as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDisplaySourceImpl: Sized {
    fn AdapterId(&self) -> ::windows::core::Result<super::super::super::Graphics::DisplayAdapterId>;
    fn SourceId(&self) -> ::windows::core::Result<u32>;
    fn GetMetadata(&self, key: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Graphics", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplaySource {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplaySource";
}
#[cfg(all(feature = "Graphics", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDisplaySourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplaySourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplaySourceVtbl {
        unsafe extern "system" fn AdapterId<Impl: IDisplaySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceId<Impl: IDisplaySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadata<Impl: IDisplaySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadata(&*(&key as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplaySource>, ::windows::core::GetTrustLevel, AdapterId::<Impl, IMPL_OFFSET>, SourceId::<Impl, IMPL_OFFSET>, GetMetadata::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplaySource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplaySource2Impl: Sized {
    fn Status(&self) -> ::windows::core::Result<DisplaySourceStatus>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplaySource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplaySource2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplaySource2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplaySource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplaySource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplaySource2Vtbl {
        unsafe extern "system" fn Status<Impl: IDisplaySource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplaySourceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusChanged<Impl: IDisplaySource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DisplaySource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DisplaySource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IDisplaySource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplaySource2>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, StatusChanged::<Impl, IMPL_OFFSET>, RemoveStatusChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplaySource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayStateImpl: Sized {
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn IsStale(&self) -> ::windows::core::Result<bool>;
    fn Targets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>>;
    fn Views(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayView>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn ConnectTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayPath>;
    fn ConnectTargetToView(&self, target: &::core::option::Option<DisplayTarget>, view: &::core::option::Option<DisplayView>) -> ::windows::core::Result<DisplayPath>;
    fn CanConnectTargetToView(&self, target: &::core::option::Option<DisplayTarget>, view: &::core::option::Option<DisplayView>) -> ::windows::core::Result<bool>;
    fn GetViewForTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayView>;
    fn GetPathForTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayPath>;
    fn DisconnectTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<()>;
    fn TryFunctionalize(&self, options: DisplayStateFunctionalizeOptions) -> ::windows::core::Result<DisplayStateOperationResult>;
    fn TryApply(&self, options: DisplayStateApplyOptions) -> ::windows::core::Result<DisplayStateOperationResult>;
    fn Clone(&self) -> ::windows::core::Result<DisplayState>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayState {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayState";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayStateVtbl {
        unsafe extern "system" fn IsReadOnly<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStale<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Targets<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Targets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Views<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Views() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTarget<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectTarget(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTargetToView<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectTargetToView(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType), &*(&view as *const <DisplayView as ::windows::core::Abi>::Abi as *const <DisplayView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanConnectTargetToView<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanConnectTargetToView(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType), &*(&view as *const <DisplayView as ::windows::core::Abi>::Abi as *const <DisplayView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewForTarget<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewForTarget(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPathForTarget<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPathForTarget(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectTarget<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectTarget(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryFunctionalize<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: DisplayStateFunctionalizeOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryFunctionalize(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryApply<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: DisplayStateApplyOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryApply(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IDisplayStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDisplayState>,
            ::windows::core::GetTrustLevel,
            IsReadOnly::<Impl, IMPL_OFFSET>,
            IsStale::<Impl, IMPL_OFFSET>,
            Targets::<Impl, IMPL_OFFSET>,
            Views::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            ConnectTarget::<Impl, IMPL_OFFSET>,
            ConnectTargetToView::<Impl, IMPL_OFFSET>,
            CanConnectTargetToView::<Impl, IMPL_OFFSET>,
            GetViewForTarget::<Impl, IMPL_OFFSET>,
            GetPathForTarget::<Impl, IMPL_OFFSET>,
            DisconnectTarget::<Impl, IMPL_OFFSET>,
            TryFunctionalize::<Impl, IMPL_OFFSET>,
            TryApply::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayStateOperationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DisplayStateOperationStatus>;
    fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayStateOperationResult {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayStateOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayStateOperationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayStateOperationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayStateOperationResultVtbl {
        unsafe extern "system" fn Status<Impl: IDisplayStateOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayStateOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedErrorCode<Impl: IDisplayStateOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayStateOperationResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, ExtendedErrorCode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayStateOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplaySurfaceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplaySurface {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplaySurface";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplaySurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplaySurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplaySurfaceVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplaySurface>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplaySurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayTargetImpl: Sized {
    fn Adapter(&self) -> ::windows::core::Result<DisplayAdapter>;
    fn DeviceInterfacePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdapterRelativeId(&self) -> ::windows::core::Result<u32>;
    fn IsConnected(&self) -> ::windows::core::Result<bool>;
    fn IsVirtualModeEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsVirtualTopologyEnabled(&self) -> ::windows::core::Result<bool>;
    fn UsageKind(&self) -> ::windows::core::Result<super::DisplayMonitorUsageKind>;
    fn MonitorPersistence(&self) -> ::windows::core::Result<DisplayTargetPersistence>;
    fn StableMonitorId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryGetMonitor(&self) -> ::windows::core::Result<super::DisplayMonitor>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn IsStale(&self) -> ::windows::core::Result<bool>;
    fn IsSame(&self, othertarget: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<bool>;
    fn IsEqual(&self, othertarget: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayTarget {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTarget";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTargetVtbl {
        unsafe extern "system" fn Adapter<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Adapter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceInterfacePath<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceInterfacePath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdapterRelativeId<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdapterRelativeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVirtualModeEnabled<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVirtualModeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVirtualTopologyEnabled<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVirtualTopologyEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsageKind<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::DisplayMonitorUsageKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsageKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MonitorPersistence<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayTargetPersistence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MonitorPersistence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StableMonitorId<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StableMonitorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetMonitor<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetMonitor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStale<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSame<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othertarget: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSame(&*(&othertarget as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IDisplayTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othertarget: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&othertarget as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDisplayTarget>,
            ::windows::core::GetTrustLevel,
            Adapter::<Impl, IMPL_OFFSET>,
            DeviceInterfacePath::<Impl, IMPL_OFFSET>,
            AdapterRelativeId::<Impl, IMPL_OFFSET>,
            IsConnected::<Impl, IMPL_OFFSET>,
            IsVirtualModeEnabled::<Impl, IMPL_OFFSET>,
            IsVirtualTopologyEnabled::<Impl, IMPL_OFFSET>,
            UsageKind::<Impl, IMPL_OFFSET>,
            MonitorPersistence::<Impl, IMPL_OFFSET>,
            StableMonitorId::<Impl, IMPL_OFFSET>,
            TryGetMonitor::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            IsStale::<Impl, IMPL_OFFSET>,
            IsSame::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskImpl: Sized {
    fn SetScanout(&self, scanout: &::core::option::Option<DisplayScanout>) -> ::windows::core::Result<()>;
    fn SetWait(&self, readyfence: &::core::option::Option<DisplayFence>, readyfencevalue: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayTask {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTask";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayTaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTaskImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTaskVtbl {
        unsafe extern "system" fn SetScanout<Impl: IDisplayTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanout: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScanout(&*(&scanout as *const <DisplayScanout as ::windows::core::Abi>::Abi as *const <DisplayScanout as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetWait<Impl: IDisplayTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readyfence: ::windows::core::RawPtr, readyfencevalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWait(&*(&readyfence as *const <DisplayFence as ::windows::core::Abi>::Abi as *const <DisplayFence as ::windows::core::DefaultType>::DefaultType), readyfencevalue).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayTask>, ::windows::core::GetTrustLevel, SetScanout::<Impl, IMPL_OFFSET>, SetWait::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTask as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTask2Impl: Sized {
    fn SetSignal(&self, signalkind: DisplayTaskSignalKind, fence: &::core::option::Option<DisplayFence>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayTask2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTask2";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayTask2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTask2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTask2Vtbl {
        unsafe extern "system" fn SetSignal<Impl: IDisplayTask2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalkind: DisplayTaskSignalKind, fence: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignal(signalkind, &*(&fence as *const <DisplayFence as ::windows::core::Abi>::Abi as *const <DisplayFence as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayTask2>, ::windows::core::GetTrustLevel, SetSignal::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTask2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskPoolImpl: Sized {
    fn CreateTask(&self) -> ::windows::core::Result<DisplayTask>;
    fn ExecuteTask(&self, task: &::core::option::Option<DisplayTask>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayTaskPool {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTaskPool";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayTaskPoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTaskPoolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTaskPoolVtbl {
        unsafe extern "system" fn CreateTask<Impl: IDisplayTaskPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteTask<Impl: IDisplayTaskPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteTask(&*(&task as *const <DisplayTask as ::windows::core::Abi>::Abi as *const <DisplayTask as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayTaskPool>, ::windows::core::GetTrustLevel, CreateTask::<Impl, IMPL_OFFSET>, ExecuteTask::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTaskPool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskPool2Impl: Sized {
    fn TryExecuteTask(&self, task: &::core::option::Option<DisplayTask>) -> ::windows::core::Result<DisplayTaskResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayTaskPool2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTaskPool2";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayTaskPool2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTaskPool2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTaskPool2Vtbl {
        unsafe extern "system" fn TryExecuteTask<Impl: IDisplayTaskPool2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryExecuteTask(&*(&task as *const <DisplayTask as ::windows::core::Abi>::Abi as *const <DisplayTask as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayTaskPool2>, ::windows::core::GetTrustLevel, TryExecuteTask::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTaskPool2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskResultImpl: Sized {
    fn PresentStatus(&self) -> ::windows::core::Result<DisplayPresentStatus>;
    fn PresentId(&self) -> ::windows::core::Result<u64>;
    fn SourceStatus(&self) -> ::windows::core::Result<DisplaySourceStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayTaskResult {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTaskResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayTaskResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTaskResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTaskResultVtbl {
        unsafe extern "system" fn PresentStatus<Impl: IDisplayTaskResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayPresentStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentId<Impl: IDisplayTaskResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceStatus<Impl: IDisplayTaskResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplaySourceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayTaskResult>, ::windows::core::GetTrustLevel, PresentStatus::<Impl, IMPL_OFFSET>, PresentId::<Impl, IMPL_OFFSET>, SourceStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTaskResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
pub trait IDisplayViewImpl: Sized {
    fn Paths(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayPath>>;
    fn ContentResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>;
    fn SetContentResolution(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>) -> ::windows::core::Result<()>;
    fn SetPrimaryPath(&self, path: &::core::option::Option<DisplayPath>) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayView {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayView";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl IDisplayViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayViewVtbl {
        unsafe extern "system" fn Paths<Impl: IDisplayViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Paths() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentResolution<Impl: IDisplayViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentResolution<Impl: IDisplayViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentResolution(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetPrimaryPath<Impl: IDisplayViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrimaryPath(&*(&path as *const <DisplayPath as ::windows::core::Abi>::Abi as *const <DisplayPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IDisplayViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayView>, ::windows::core::GetTrustLevel, Paths::<Impl, IMPL_OFFSET>, ContentResolution::<Impl, IMPL_OFFSET>, SetContentResolution::<Impl, IMPL_OFFSET>, SetPrimaryPath::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayWireFormatImpl: Sized {
    fn PixelEncoding(&self) -> ::windows::core::Result<DisplayWireFormatPixelEncoding>;
    fn BitsPerChannel(&self) -> ::windows::core::Result<i32>;
    fn ColorSpace(&self) -> ::windows::core::Result<DisplayWireFormatColorSpace>;
    fn Eotf(&self) -> ::windows::core::Result<DisplayWireFormatEotf>;
    fn HdrMetadata(&self) -> ::windows::core::Result<DisplayWireFormatHdrMetadata>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayWireFormat {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayWireFormat";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayWireFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayWireFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayWireFormatVtbl {
        unsafe extern "system" fn PixelEncoding<Impl: IDisplayWireFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatPixelEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitsPerChannel<Impl: IDisplayWireFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitsPerChannel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorSpace<Impl: IDisplayWireFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatColorSpace) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Eotf<Impl: IDisplayWireFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatEotf) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Eotf() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HdrMetadata<Impl: IDisplayWireFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatHdrMetadata) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HdrMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IDisplayWireFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayWireFormat>, ::windows::core::GetTrustLevel, PixelEncoding::<Impl, IMPL_OFFSET>, BitsPerChannel::<Impl, IMPL_OFFSET>, ColorSpace::<Impl, IMPL_OFFSET>, Eotf::<Impl, IMPL_OFFSET>, HdrMetadata::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayWireFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayWireFormatFactoryImpl: Sized {
    fn CreateInstance(&self, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> ::windows::core::Result<DisplayWireFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayWireFormatFactory {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayWireFormatFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayWireFormatFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayWireFormatFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayWireFormatFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDisplayWireFormatFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(pixelencoding, bitsperchannel, colorspace, eotf, hdrmetadata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayWireFormatFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayWireFormatFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayWireFormatStaticsImpl: Sized {
    fn CreateWithProperties(&self, extraproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> ::windows::core::Result<DisplayWireFormat>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayWireFormatStatics {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayWireFormatStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayWireFormatStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayWireFormatStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayWireFormatStaticsVtbl {
        unsafe extern "system" fn CreateWithProperties<Impl: IDisplayWireFormatStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extraproperties: ::windows::core::RawPtr, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithProperties(
                &*(&extraproperties as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>> as ::windows::core::DefaultType>::DefaultType),
                pixelencoding,
                bitsperchannel,
                colorspace,
                eotf,
                hdrmetadata,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayWireFormatStatics>, ::windows::core::GetTrustLevel, CreateWithProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayWireFormatStatics as ::windows::core::Interface>::IID
    }
}
