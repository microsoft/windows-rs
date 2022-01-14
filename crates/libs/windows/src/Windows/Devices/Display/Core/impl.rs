#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
pub trait IDisplayAdapter_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DisplayAdapterId>;
    fn DeviceInterfacePath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceCount(&mut self) -> ::windows::core::Result<u32>;
    fn PciVendorId(&mut self) -> ::windows::core::Result<u32>;
    fn PciDeviceId(&mut self) -> ::windows::core::Result<u32>;
    fn PciSubSystemId(&mut self) -> ::windows::core::Result<u32>;
    fn PciRevision(&mut self) -> ::windows::core::Result<u32>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayAdapter {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayAdapter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl IDisplayAdapter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayAdapter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayAdapter_Vtbl {
        unsafe extern "system" fn Id<Impl: IDisplayAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceInterfacePath<Impl: IDisplayAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SourceCount<Impl: IDisplayAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PciVendorId<Impl: IDisplayAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PciDeviceId<Impl: IDisplayAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PciSubSystemId<Impl: IDisplayAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PciRevision<Impl: IDisplayAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IDisplayAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayAdapter, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DeviceInterfacePath: DeviceInterfacePath::<Impl, IMPL_OFFSET>,
            SourceCount: SourceCount::<Impl, IMPL_OFFSET>,
            PciVendorId: PciVendorId::<Impl, IMPL_OFFSET>,
            PciDeviceId: PciDeviceId::<Impl, IMPL_OFFSET>,
            PciSubSystemId: PciSubSystemId::<Impl, IMPL_OFFSET>,
            PciRevision: PciRevision::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayAdapter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
pub trait IDisplayAdapterStatics_Impl: Sized {
    fn FromId(&mut self, id: &super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::Result<DisplayAdapter>;
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayAdapterStatics {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayAdapterStatics";
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl IDisplayAdapterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayAdapterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayAdapterStatics_Vtbl {
        unsafe extern "system" fn FromId<Impl: IDisplayAdapterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: super::super::super::Graphics::DisplayAdapterId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayAdapterStatics, BASE_OFFSET>(), FromId: FromId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayAdapterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayDevice_Impl: Sized {
    fn CreateScanoutSource(&mut self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplaySource>;
    fn CreatePrimary(&mut self, target: &::core::option::Option<DisplayTarget>, desc: &::core::option::Option<DisplayPrimaryDescription>) -> ::windows::core::Result<DisplaySurface>;
    fn CreateTaskPool(&mut self) -> ::windows::core::Result<DisplayTaskPool>;
    fn CreatePeriodicFence(&mut self, target: &::core::option::Option<DisplayTarget>, offsetfromvblank: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<DisplayFence>;
    fn WaitForVBlank(&mut self, source: &::core::option::Option<DisplaySource>) -> ::windows::core::Result<()>;
    fn CreateSimpleScanout(&mut self, psource: &::core::option::Option<DisplaySource>, psurface: &::core::option::Option<DisplaySurface>, subresourceindex: u32, syncinterval: u32) -> ::windows::core::Result<DisplayScanout>;
    fn IsCapabilitySupported(&mut self, capability: DisplayDeviceCapability) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayDevice {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayDevice_Vtbl {
        unsafe extern "system" fn CreateScanoutSource<Impl: IDisplayDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePrimary<Impl: IDisplayDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, desc: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateTaskPool<Impl: IDisplayDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePeriodicFence<Impl: IDisplayDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, offsetfromvblank: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WaitForVBlank<Impl: IDisplayDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForVBlank(&*(&source as *const <DisplaySource as ::windows::core::Abi>::Abi as *const <DisplaySource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateSimpleScanout<Impl: IDisplayDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psurface: ::windows::core::RawPtr, subresourceindex: u32, syncinterval: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsCapabilitySupported<Impl: IDisplayDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capability: DisplayDeviceCapability, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayDevice, BASE_OFFSET>(),
            CreateScanoutSource: CreateScanoutSource::<Impl, IMPL_OFFSET>,
            CreatePrimary: CreatePrimary::<Impl, IMPL_OFFSET>,
            CreateTaskPool: CreateTaskPool::<Impl, IMPL_OFFSET>,
            CreatePeriodicFence: CreatePeriodicFence::<Impl, IMPL_OFFSET>,
            WaitForVBlank: WaitForVBlank::<Impl, IMPL_OFFSET>,
            CreateSimpleScanout: CreateSimpleScanout::<Impl, IMPL_OFFSET>,
            IsCapabilitySupported: IsCapabilitySupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
pub trait IDisplayDevice2_Impl: Sized {
    fn CreateSimpleScanoutWithDirtyRectsAndOptions(&mut self, source: &::core::option::Option<DisplaySource>, surface: &::core::option::Option<DisplaySurface>, subresourceindex: u32, syncinterval: u32, dirtyrects: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Graphics::RectInt32>>, options: DisplayScanoutOptions) -> ::windows::core::Result<DisplayScanout>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayDevice2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayDevice2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl IDisplayDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayDevice2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayDevice2_Vtbl {
        unsafe extern "system" fn CreateSimpleScanoutWithDirtyRectsAndOptions<Impl: IDisplayDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, surface: ::windows::core::RawPtr, subresourceindex: u32, syncinterval: u32, dirtyrects: ::windows::core::RawPtr, options: DisplayScanoutOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayDevice2, BASE_OFFSET>(),
            CreateSimpleScanoutWithDirtyRectsAndOptions: CreateSimpleScanoutWithDirtyRectsAndOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayFence_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayFence {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayFence";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayFence_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayFence_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayFence_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayFence, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayFence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayManager_Impl: Sized {
    fn GetCurrentTargets(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>>;
    fn GetCurrentAdapters(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayAdapter>>;
    fn TryAcquireTarget(&mut self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayManagerResult>;
    fn ReleaseTarget(&mut self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<()>;
    fn TryReadCurrentStateForAllTargets(&mut self) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn TryAcquireTargetsAndReadCurrentState(&mut self, targets: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn TryAcquireTargetsAndCreateEmptyState(&mut self, targets: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn TryAcquireTargetsAndCreateSubstate(&mut self, existingstate: &::core::option::Option<DisplayState>, targets: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn CreateDisplayDevice(&mut self, adapter: &::core::option::Option<DisplayAdapter>) -> ::windows::core::Result<DisplayDevice>;
    fn Enabled(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerEnabledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnabled(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Disabled(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerDisabledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisabled(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PathsFailedOrInvalidated(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerPathsFailedOrInvalidatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePathsFailedOrInvalidated(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayManager {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManager_Vtbl {
        unsafe extern "system" fn GetCurrentTargets<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentAdapters<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryAcquireTarget<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut DisplayManagerResult) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReleaseTarget<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseTarget(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryReadCurrentStateForAllTargets<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryAcquireTargetsAndReadCurrentState<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryAcquireTargetsAndCreateEmptyState<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryAcquireTargetsAndCreateSubstate<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, existingstate: ::windows::core::RawPtr, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateDisplayDevice<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Enabled<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveEnabled<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnabled(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disabled<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDisabled<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisabled(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Changed<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveChanged<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PathsFailedOrInvalidated<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePathsFailedOrInvalidated<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePathsFailedOrInvalidated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IDisplayManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayManager, BASE_OFFSET>(),
            GetCurrentTargets: GetCurrentTargets::<Impl, IMPL_OFFSET>,
            GetCurrentAdapters: GetCurrentAdapters::<Impl, IMPL_OFFSET>,
            TryAcquireTarget: TryAcquireTarget::<Impl, IMPL_OFFSET>,
            ReleaseTarget: ReleaseTarget::<Impl, IMPL_OFFSET>,
            TryReadCurrentStateForAllTargets: TryReadCurrentStateForAllTargets::<Impl, IMPL_OFFSET>,
            TryAcquireTargetsAndReadCurrentState: TryAcquireTargetsAndReadCurrentState::<Impl, IMPL_OFFSET>,
            TryAcquireTargetsAndCreateEmptyState: TryAcquireTargetsAndCreateEmptyState::<Impl, IMPL_OFFSET>,
            TryAcquireTargetsAndCreateSubstate: TryAcquireTargetsAndCreateSubstate::<Impl, IMPL_OFFSET>,
            CreateDisplayDevice: CreateDisplayDevice::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            RemoveEnabled: RemoveEnabled::<Impl, IMPL_OFFSET>,
            Disabled: Disabled::<Impl, IMPL_OFFSET>,
            RemoveDisabled: RemoveDisabled::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
            PathsFailedOrInvalidated: PathsFailedOrInvalidated::<Impl, IMPL_OFFSET>,
            RemovePathsFailedOrInvalidated: RemovePathsFailedOrInvalidated::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayManagerChangedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayManagerChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayManagerChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerChangedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IDisplayManagerChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IDisplayManagerChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDisplayManagerChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayManagerChangedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayManagerDisabledEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayManagerDisabledEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerDisabledEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayManagerDisabledEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerDisabledEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerDisabledEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IDisplayManagerDisabledEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IDisplayManagerDisabledEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDisplayManagerDisabledEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayManagerDisabledEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerDisabledEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayManagerEnabledEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayManagerEnabledEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerEnabledEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayManagerEnabledEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerEnabledEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerEnabledEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IDisplayManagerEnabledEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IDisplayManagerEnabledEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDisplayManagerEnabledEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayManagerEnabledEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerEnabledEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayManagerPathsFailedOrInvalidatedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayManagerPathsFailedOrInvalidatedEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerPathsFailedOrInvalidatedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayManagerPathsFailedOrInvalidatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerPathsFailedOrInvalidatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerPathsFailedOrInvalidatedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IDisplayManagerPathsFailedOrInvalidatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IDisplayManagerPathsFailedOrInvalidatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDisplayManagerPathsFailedOrInvalidatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayManagerPathsFailedOrInvalidatedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerPathsFailedOrInvalidatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerResultWithState_Impl: Sized {
    fn ErrorCode(&mut self) -> ::windows::core::Result<DisplayManagerResult>;
    fn ExtendedErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn State(&mut self) -> ::windows::core::Result<DisplayState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayManagerResultWithState {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerResultWithState";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayManagerResultWithState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerResultWithState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerResultWithState_Vtbl {
        unsafe extern "system" fn ErrorCode<Impl: IDisplayManagerResultWithState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayManagerResult) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedErrorCode<Impl: IDisplayManagerResultWithState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn State<Impl: IDisplayManagerResultWithState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayManagerResultWithState, BASE_OFFSET>(),
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            ExtendedErrorCode: ExtendedErrorCode::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerResultWithState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerStatics_Impl: Sized {
    fn Create(&mut self, options: DisplayManagerOptions) -> ::windows::core::Result<DisplayManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayManagerStatics {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayManagerStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: IDisplayManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: DisplayManagerOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayManagerStatics, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait IDisplayModeInfo_Impl: Sized {
    fn SourceResolution(&mut self) -> ::windows::core::Result<super::super::super::Graphics::SizeInt32>;
    fn IsStereo(&mut self) -> ::windows::core::Result<bool>;
    fn SourcePixelFormat(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn TargetResolution(&mut self) -> ::windows::core::Result<super::super::super::Graphics::SizeInt32>;
    fn PresentationRate(&mut self) -> ::windows::core::Result<DisplayPresentationRate>;
    fn IsInterlaced(&mut self) -> ::windows::core::Result<bool>;
    fn GetWireFormatSupportedBitsPerChannel(&mut self, encoding: DisplayWireFormatPixelEncoding) -> ::windows::core::Result<DisplayBitsPerChannel>;
    fn IsWireFormatSupported(&mut self, wireformat: &::core::option::Option<DisplayWireFormat>) -> ::windows::core::Result<bool>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayModeInfo {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayModeInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl IDisplayModeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayModeInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayModeInfo_Vtbl {
        unsafe extern "system" fn SourceResolution<Impl: IDisplayModeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsStereo<Impl: IDisplayModeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SourcePixelFormat<Impl: IDisplayModeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TargetResolution<Impl: IDisplayModeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PresentationRate<Impl: IDisplayModeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayPresentationRate) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInterlaced<Impl: IDisplayModeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetWireFormatSupportedBitsPerChannel<Impl: IDisplayModeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: DisplayWireFormatPixelEncoding, result__: *mut DisplayBitsPerChannel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsWireFormatSupported<Impl: IDisplayModeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wireformat: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IDisplayModeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayModeInfo, BASE_OFFSET>(),
            SourceResolution: SourceResolution::<Impl, IMPL_OFFSET>,
            IsStereo: IsStereo::<Impl, IMPL_OFFSET>,
            SourcePixelFormat: SourcePixelFormat::<Impl, IMPL_OFFSET>,
            TargetResolution: TargetResolution::<Impl, IMPL_OFFSET>,
            PresentationRate: PresentationRate::<Impl, IMPL_OFFSET>,
            IsInterlaced: IsInterlaced::<Impl, IMPL_OFFSET>,
            GetWireFormatSupportedBitsPerChannel: GetWireFormatSupportedBitsPerChannel::<Impl, IMPL_OFFSET>,
            IsWireFormatSupported: IsWireFormatSupported::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayModeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IDisplayModeInfo2_Impl: Sized {
    fn PhysicalPresentationRate(&mut self) -> ::windows::core::Result<DisplayPresentationRate>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayModeInfo2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayModeInfo2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IDisplayModeInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayModeInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayModeInfo2_Vtbl {
        unsafe extern "system" fn PhysicalPresentationRate<Impl: IDisplayModeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayPresentationRate) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayModeInfo2, BASE_OFFSET>(),
            PhysicalPresentationRate: PhysicalPresentationRate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayModeInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait IDisplayPath_Impl: Sized {
    fn View(&mut self) -> ::windows::core::Result<DisplayView>;
    fn Target(&mut self) -> ::windows::core::Result<DisplayTarget>;
    fn Status(&mut self) -> ::windows::core::Result<DisplayPathStatus>;
    fn SourceResolution(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>;
    fn SetSourceResolution(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>) -> ::windows::core::Result<()>;
    fn SourcePixelFormat(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SetSourcePixelFormat(&mut self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn IsStereo(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsStereo(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TargetResolution(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>;
    fn SetTargetResolution(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>) -> ::windows::core::Result<()>;
    fn PresentationRate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>>;
    fn SetPresentationRate(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<DisplayPresentationRate>>) -> ::windows::core::Result<()>;
    fn IsInterlaced(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<bool>>;
    fn SetIsInterlaced(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn WireFormat(&mut self) -> ::windows::core::Result<DisplayWireFormat>;
    fn SetWireFormat(&mut self, value: &::core::option::Option<DisplayWireFormat>) -> ::windows::core::Result<()>;
    fn Rotation(&mut self) -> ::windows::core::Result<DisplayRotation>;
    fn SetRotation(&mut self, value: DisplayRotation) -> ::windows::core::Result<()>;
    fn Scaling(&mut self) -> ::windows::core::Result<DisplayPathScaling>;
    fn SetScaling(&mut self, value: DisplayPathScaling) -> ::windows::core::Result<()>;
    fn FindModes(&mut self, flags: DisplayModeQueryOptions) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayModeInfo>>;
    fn ApplyPropertiesFromMode(&mut self, moderesult: &::core::option::Option<DisplayModeInfo>) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPath {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayPath";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl IDisplayPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPath_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPath_Vtbl {
        unsafe extern "system" fn View<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Target<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Status<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayPathStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SourceResolution<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSourceResolution<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceResolution(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourcePixelFormat<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSourcePixelFormat<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourcePixelFormat(value).into()
        }
        unsafe extern "system" fn IsStereo<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsStereo<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStereo(value).into()
        }
        unsafe extern "system" fn TargetResolution<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetResolution<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetResolution(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PresentationRate<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPresentationRate<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresentationRate(&*(&value as *const <super::super::super::Foundation::IReference<DisplayPresentationRate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<DisplayPresentationRate> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsInterlaced<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsInterlaced<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInterlaced(&*(&value as *const <super::super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WireFormat<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWireFormat<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWireFormat(&*(&value as *const <DisplayWireFormat as ::windows::core::Abi>::Abi as *const <DisplayWireFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Rotation<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayRotation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotation<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DisplayRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn Scaling<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayPathScaling) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScaling<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DisplayPathScaling) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaling(value).into()
        }
        unsafe extern "system" fn FindModes<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: DisplayModeQueryOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApplyPropertiesFromMode<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moderesult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyPropertiesFromMode(&*(&moderesult as *const <DisplayModeInfo as ::windows::core::Abi>::Abi as *const <DisplayModeInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IDisplayPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayPath, BASE_OFFSET>(),
            View: View::<Impl, IMPL_OFFSET>,
            Target: Target::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            SourceResolution: SourceResolution::<Impl, IMPL_OFFSET>,
            SetSourceResolution: SetSourceResolution::<Impl, IMPL_OFFSET>,
            SourcePixelFormat: SourcePixelFormat::<Impl, IMPL_OFFSET>,
            SetSourcePixelFormat: SetSourcePixelFormat::<Impl, IMPL_OFFSET>,
            IsStereo: IsStereo::<Impl, IMPL_OFFSET>,
            SetIsStereo: SetIsStereo::<Impl, IMPL_OFFSET>,
            TargetResolution: TargetResolution::<Impl, IMPL_OFFSET>,
            SetTargetResolution: SetTargetResolution::<Impl, IMPL_OFFSET>,
            PresentationRate: PresentationRate::<Impl, IMPL_OFFSET>,
            SetPresentationRate: SetPresentationRate::<Impl, IMPL_OFFSET>,
            IsInterlaced: IsInterlaced::<Impl, IMPL_OFFSET>,
            SetIsInterlaced: SetIsInterlaced::<Impl, IMPL_OFFSET>,
            WireFormat: WireFormat::<Impl, IMPL_OFFSET>,
            SetWireFormat: SetWireFormat::<Impl, IMPL_OFFSET>,
            Rotation: Rotation::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
            Scaling: Scaling::<Impl, IMPL_OFFSET>,
            SetScaling: SetScaling::<Impl, IMPL_OFFSET>,
            FindModes: FindModes::<Impl, IMPL_OFFSET>,
            ApplyPropertiesFromMode: ApplyPropertiesFromMode::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IDisplayPath2_Impl: Sized {
    fn PhysicalPresentationRate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>>;
    fn SetPhysicalPresentationRate(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<DisplayPresentationRate>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPath2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayPath2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IDisplayPath2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPath2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPath2_Vtbl {
        unsafe extern "system" fn PhysicalPresentationRate<Impl: IDisplayPath2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPhysicalPresentationRate<Impl: IDisplayPath2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhysicalPresentationRate(&*(&value as *const <super::super::super::Foundation::IReference<DisplayPresentationRate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<DisplayPresentationRate> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayPath2, BASE_OFFSET>(),
            PhysicalPresentationRate: PhysicalPresentationRate::<Impl, IMPL_OFFSET>,
            SetPhysicalPresentationRate: SetPhysicalPresentationRate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPath2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDisplayPrimaryDescription_Impl: Sized {
    fn Width(&mut self) -> ::windows::core::Result<u32>;
    fn Height(&mut self) -> ::windows::core::Result<u32>;
    fn Format(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn ColorSpace(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXColorSpace>;
    fn IsStereo(&mut self) -> ::windows::core::Result<bool>;
    fn MultisampleDescription(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPrimaryDescription {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayPrimaryDescription";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDisplayPrimaryDescription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPrimaryDescription_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPrimaryDescription_Vtbl {
        unsafe extern "system" fn Width<Impl: IDisplayPrimaryDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Height<Impl: IDisplayPrimaryDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Format<Impl: IDisplayPrimaryDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ColorSpace<Impl: IDisplayPrimaryDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXColorSpace) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsStereo<Impl: IDisplayPrimaryDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MultisampleDescription<Impl: IDisplayPrimaryDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IDisplayPrimaryDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayPrimaryDescription, BASE_OFFSET>(),
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            Format: Format::<Impl, IMPL_OFFSET>,
            ColorSpace: ColorSpace::<Impl, IMPL_OFFSET>,
            IsStereo: IsStereo::<Impl, IMPL_OFFSET>,
            MultisampleDescription: MultisampleDescription::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPrimaryDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDisplayPrimaryDescriptionFactory_Impl: Sized {
    fn CreateInstance(&mut self, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: &super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows::core::Result<DisplayPrimaryDescription>;
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPrimaryDescriptionFactory {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayPrimaryDescriptionFactory";
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDisplayPrimaryDescriptionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPrimaryDescriptionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPrimaryDescriptionFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDisplayPrimaryDescriptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayPrimaryDescriptionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPrimaryDescriptionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDisplayPrimaryDescriptionStatics_Impl: Sized {
    fn CreateWithProperties(&mut self, extraproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: &super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows::core::Result<DisplayPrimaryDescription>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPrimaryDescriptionStatics {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayPrimaryDescriptionStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDisplayPrimaryDescriptionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPrimaryDescriptionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPrimaryDescriptionStatics_Vtbl {
        unsafe extern "system" fn CreateWithProperties<Impl: IDisplayPrimaryDescriptionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extraproperties: ::windows::core::RawPtr, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayPrimaryDescriptionStatics, BASE_OFFSET>(),
            CreateWithProperties: CreateWithProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPrimaryDescriptionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayScanout_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayScanout {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayScanout";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayScanout_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayScanout_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayScanout_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayScanout, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayScanout as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDisplaySource_Impl: Sized {
    fn AdapterId(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DisplayAdapterId>;
    fn SourceId(&mut self) -> ::windows::core::Result<u32>;
    fn GetMetadata(&mut self, key: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Graphics", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplaySource {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplaySource";
}
#[cfg(all(feature = "Graphics", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDisplaySource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplaySource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplaySource_Vtbl {
        unsafe extern "system" fn AdapterId<Impl: IDisplaySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SourceId<Impl: IDisplaySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMetadata<Impl: IDisplaySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplaySource, BASE_OFFSET>(),
            AdapterId: AdapterId::<Impl, IMPL_OFFSET>,
            SourceId: SourceId::<Impl, IMPL_OFFSET>,
            GetMetadata: GetMetadata::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplaySource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplaySource2_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<DisplaySourceStatus>;
    fn StatusChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplaySource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplaySource2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplaySource2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplaySource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplaySource2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplaySource2_Vtbl {
        unsafe extern "system" fn Status<Impl: IDisplaySource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplaySourceStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StatusChanged<Impl: IDisplaySource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStatusChanged<Impl: IDisplaySource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplaySource2, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            StatusChanged: StatusChanged::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged: RemoveStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplaySource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayState_Impl: Sized {
    fn IsReadOnly(&mut self) -> ::windows::core::Result<bool>;
    fn IsStale(&mut self) -> ::windows::core::Result<bool>;
    fn Targets(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>>;
    fn Views(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayView>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn ConnectTarget(&mut self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayPath>;
    fn ConnectTargetToView(&mut self, target: &::core::option::Option<DisplayTarget>, view: &::core::option::Option<DisplayView>) -> ::windows::core::Result<DisplayPath>;
    fn CanConnectTargetToView(&mut self, target: &::core::option::Option<DisplayTarget>, view: &::core::option::Option<DisplayView>) -> ::windows::core::Result<bool>;
    fn GetViewForTarget(&mut self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayView>;
    fn GetPathForTarget(&mut self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayPath>;
    fn DisconnectTarget(&mut self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<()>;
    fn TryFunctionalize(&mut self, options: DisplayStateFunctionalizeOptions) -> ::windows::core::Result<DisplayStateOperationResult>;
    fn TryApply(&mut self, options: DisplayStateApplyOptions) -> ::windows::core::Result<DisplayStateOperationResult>;
    fn Clone(&mut self) -> ::windows::core::Result<DisplayState>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayState {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayState";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayState_Vtbl {
        unsafe extern "system" fn IsReadOnly<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsStale<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Targets<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Views<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectTarget<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectTargetToView<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanConnectTargetToView<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetViewForTarget<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPathForTarget<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisconnectTarget<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectTarget(&*(&target as *const <DisplayTarget as ::windows::core::Abi>::Abi as *const <DisplayTarget as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryFunctionalize<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: DisplayStateFunctionalizeOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryApply<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: DisplayStateApplyOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IDisplayState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayState, BASE_OFFSET>(),
            IsReadOnly: IsReadOnly::<Impl, IMPL_OFFSET>,
            IsStale: IsStale::<Impl, IMPL_OFFSET>,
            Targets: Targets::<Impl, IMPL_OFFSET>,
            Views: Views::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            ConnectTarget: ConnectTarget::<Impl, IMPL_OFFSET>,
            ConnectTargetToView: ConnectTargetToView::<Impl, IMPL_OFFSET>,
            CanConnectTargetToView: CanConnectTargetToView::<Impl, IMPL_OFFSET>,
            GetViewForTarget: GetViewForTarget::<Impl, IMPL_OFFSET>,
            GetPathForTarget: GetPathForTarget::<Impl, IMPL_OFFSET>,
            DisconnectTarget: DisconnectTarget::<Impl, IMPL_OFFSET>,
            TryFunctionalize: TryFunctionalize::<Impl, IMPL_OFFSET>,
            TryApply: TryApply::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayStateOperationResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<DisplayStateOperationStatus>;
    fn ExtendedErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayStateOperationResult {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayStateOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayStateOperationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayStateOperationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayStateOperationResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IDisplayStateOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayStateOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedErrorCode<Impl: IDisplayStateOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayStateOperationResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedErrorCode: ExtendedErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayStateOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplaySurface_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplaySurface {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplaySurface";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplaySurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplaySurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplaySurface_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplaySurface, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplaySurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayTarget_Impl: Sized {
    fn Adapter(&mut self) -> ::windows::core::Result<DisplayAdapter>;
    fn DeviceInterfacePath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdapterRelativeId(&mut self) -> ::windows::core::Result<u32>;
    fn IsConnected(&mut self) -> ::windows::core::Result<bool>;
    fn IsVirtualModeEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsVirtualTopologyEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn UsageKind(&mut self) -> ::windows::core::Result<super::DisplayMonitorUsageKind>;
    fn MonitorPersistence(&mut self) -> ::windows::core::Result<DisplayTargetPersistence>;
    fn StableMonitorId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryGetMonitor(&mut self) -> ::windows::core::Result<super::DisplayMonitor>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn IsStale(&mut self) -> ::windows::core::Result<bool>;
    fn IsSame(&mut self, othertarget: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<bool>;
    fn IsEqual(&mut self, othertarget: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayTarget {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTarget";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTarget_Vtbl {
        unsafe extern "system" fn Adapter<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceInterfacePath<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdapterRelativeId<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsConnected<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsVirtualModeEnabled<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsVirtualTopologyEnabled<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsageKind<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::DisplayMonitorUsageKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MonitorPersistence<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayTargetPersistence) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StableMonitorId<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetMonitor<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsStale<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSame<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othertarget: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEqual<Impl: IDisplayTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othertarget: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayTarget, BASE_OFFSET>(),
            Adapter: Adapter::<Impl, IMPL_OFFSET>,
            DeviceInterfacePath: DeviceInterfacePath::<Impl, IMPL_OFFSET>,
            AdapterRelativeId: AdapterRelativeId::<Impl, IMPL_OFFSET>,
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
            IsVirtualModeEnabled: IsVirtualModeEnabled::<Impl, IMPL_OFFSET>,
            IsVirtualTopologyEnabled: IsVirtualTopologyEnabled::<Impl, IMPL_OFFSET>,
            UsageKind: UsageKind::<Impl, IMPL_OFFSET>,
            MonitorPersistence: MonitorPersistence::<Impl, IMPL_OFFSET>,
            StableMonitorId: StableMonitorId::<Impl, IMPL_OFFSET>,
            TryGetMonitor: TryGetMonitor::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            IsStale: IsStale::<Impl, IMPL_OFFSET>,
            IsSame: IsSame::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTask_Impl: Sized {
    fn SetScanout(&mut self, scanout: &::core::option::Option<DisplayScanout>) -> ::windows::core::Result<()>;
    fn SetWait(&mut self, readyfence: &::core::option::Option<DisplayFence>, readyfencevalue: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayTask {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTask";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTask_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTask_Vtbl {
        unsafe extern "system" fn SetScanout<Impl: IDisplayTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanout: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScanout(&*(&scanout as *const <DisplayScanout as ::windows::core::Abi>::Abi as *const <DisplayScanout as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetWait<Impl: IDisplayTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readyfence: ::windows::core::RawPtr, readyfencevalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWait(&*(&readyfence as *const <DisplayFence as ::windows::core::Abi>::Abi as *const <DisplayFence as ::windows::core::DefaultType>::DefaultType), readyfencevalue).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayTask, BASE_OFFSET>(),
            SetScanout: SetScanout::<Impl, IMPL_OFFSET>,
            SetWait: SetWait::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTask as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTask2_Impl: Sized {
    fn SetSignal(&mut self, signalkind: DisplayTaskSignalKind, fence: &::core::option::Option<DisplayFence>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayTask2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTask2";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayTask2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTask2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTask2_Vtbl {
        unsafe extern "system" fn SetSignal<Impl: IDisplayTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalkind: DisplayTaskSignalKind, fence: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignal(signalkind, &*(&fence as *const <DisplayFence as ::windows::core::Abi>::Abi as *const <DisplayFence as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayTask2, BASE_OFFSET>(), SetSignal: SetSignal::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTask2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskPool_Impl: Sized {
    fn CreateTask(&mut self) -> ::windows::core::Result<DisplayTask>;
    fn ExecuteTask(&mut self, task: &::core::option::Option<DisplayTask>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayTaskPool {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTaskPool";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayTaskPool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTaskPool_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTaskPool_Vtbl {
        unsafe extern "system" fn CreateTask<Impl: IDisplayTaskPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExecuteTask<Impl: IDisplayTaskPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteTask(&*(&task as *const <DisplayTask as ::windows::core::Abi>::Abi as *const <DisplayTask as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayTaskPool, BASE_OFFSET>(),
            CreateTask: CreateTask::<Impl, IMPL_OFFSET>,
            ExecuteTask: ExecuteTask::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTaskPool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskPool2_Impl: Sized {
    fn TryExecuteTask(&mut self, task: &::core::option::Option<DisplayTask>) -> ::windows::core::Result<DisplayTaskResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayTaskPool2 {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTaskPool2";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayTaskPool2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTaskPool2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTaskPool2_Vtbl {
        unsafe extern "system" fn TryExecuteTask<Impl: IDisplayTaskPool2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayTaskPool2, BASE_OFFSET>(), TryExecuteTask: TryExecuteTask::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTaskPool2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskResult_Impl: Sized {
    fn PresentStatus(&mut self) -> ::windows::core::Result<DisplayPresentStatus>;
    fn PresentId(&mut self) -> ::windows::core::Result<u64>;
    fn SourceStatus(&mut self) -> ::windows::core::Result<DisplaySourceStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayTaskResult {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayTaskResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayTaskResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayTaskResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayTaskResult_Vtbl {
        unsafe extern "system" fn PresentStatus<Impl: IDisplayTaskResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayPresentStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PresentId<Impl: IDisplayTaskResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SourceStatus<Impl: IDisplayTaskResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplaySourceStatus) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayTaskResult, BASE_OFFSET>(),
            PresentStatus: PresentStatus::<Impl, IMPL_OFFSET>,
            PresentId: PresentId::<Impl, IMPL_OFFSET>,
            SourceStatus: SourceStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayTaskResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
pub trait IDisplayView_Impl: Sized {
    fn Paths(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayPath>>;
    fn ContentResolution(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>;
    fn SetContentResolution(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>) -> ::windows::core::Result<()>;
    fn SetPrimaryPath(&mut self, path: &::core::option::Option<DisplayPath>) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayView {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayView";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics", feature = "implement_exclusive"))]
impl IDisplayView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayView_Vtbl {
        unsafe extern "system" fn Paths<Impl: IDisplayView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentResolution<Impl: IDisplayView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentResolution<Impl: IDisplayView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentResolution(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetPrimaryPath<Impl: IDisplayView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrimaryPath(&*(&path as *const <DisplayPath as ::windows::core::Abi>::Abi as *const <DisplayPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IDisplayView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayView, BASE_OFFSET>(),
            Paths: Paths::<Impl, IMPL_OFFSET>,
            ContentResolution: ContentResolution::<Impl, IMPL_OFFSET>,
            SetContentResolution: SetContentResolution::<Impl, IMPL_OFFSET>,
            SetPrimaryPath: SetPrimaryPath::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayWireFormat_Impl: Sized {
    fn PixelEncoding(&mut self) -> ::windows::core::Result<DisplayWireFormatPixelEncoding>;
    fn BitsPerChannel(&mut self) -> ::windows::core::Result<i32>;
    fn ColorSpace(&mut self) -> ::windows::core::Result<DisplayWireFormatColorSpace>;
    fn Eotf(&mut self) -> ::windows::core::Result<DisplayWireFormatEotf>;
    fn HdrMetadata(&mut self) -> ::windows::core::Result<DisplayWireFormatHdrMetadata>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayWireFormat {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayWireFormat";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayWireFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayWireFormat_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayWireFormat_Vtbl {
        unsafe extern "system" fn PixelEncoding<Impl: IDisplayWireFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatPixelEncoding) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BitsPerChannel<Impl: IDisplayWireFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ColorSpace<Impl: IDisplayWireFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatColorSpace) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Eotf<Impl: IDisplayWireFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatEotf) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HdrMetadata<Impl: IDisplayWireFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatHdrMetadata) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IDisplayWireFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayWireFormat, BASE_OFFSET>(),
            PixelEncoding: PixelEncoding::<Impl, IMPL_OFFSET>,
            BitsPerChannel: BitsPerChannel::<Impl, IMPL_OFFSET>,
            ColorSpace: ColorSpace::<Impl, IMPL_OFFSET>,
            Eotf: Eotf::<Impl, IMPL_OFFSET>,
            HdrMetadata: HdrMetadata::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayWireFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayWireFormatFactory_Impl: Sized {
    fn CreateInstance(&mut self, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> ::windows::core::Result<DisplayWireFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayWireFormatFactory {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayWireFormatFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayWireFormatFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayWireFormatFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayWireFormatFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDisplayWireFormatFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayWireFormatFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayWireFormatFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayWireFormatStatics_Impl: Sized {
    fn CreateWithProperties(&mut self, extraproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> ::windows::core::Result<DisplayWireFormat>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayWireFormatStatics {
    const NAME: &'static str = "Windows.Devices.Display.Core.IDisplayWireFormatStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayWireFormatStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayWireFormatStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayWireFormatStatics_Vtbl {
        unsafe extern "system" fn CreateWithProperties<Impl: IDisplayWireFormatStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extraproperties: ::windows::core::RawPtr, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayWireFormatStatics, BASE_OFFSET>(),
            CreateWithProperties: CreateWithProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayWireFormatStatics as ::windows::core::Interface>::IID
    }
}
