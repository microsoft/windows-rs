#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionFrameKindStatics_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Depth(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Infrared(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownPerceptionFrameKindStatics {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IKnownPerceptionFrameKindStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownPerceptionFrameKindStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownPerceptionFrameKindStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownPerceptionFrameKindStatics_Vtbl {
        unsafe extern "system" fn Color<Impl: IKnownPerceptionFrameKindStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Depth<Impl: IKnownPerceptionFrameKindStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Depth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Infrared<Impl: IKnownPerceptionFrameKindStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Infrared() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownPerceptionFrameKindStatics, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            Depth: Depth::<Impl, IMPL_OFFSET>,
            Infrared: Infrared::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownPerceptionFrameKindStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionControlGroup_Impl: Sized {
    fn FrameProviderIds(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionControlGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionControlGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionControlGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionControlGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionControlGroup_Vtbl {
        unsafe extern "system" fn FrameProviderIds<Impl: IPerceptionControlGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameProviderIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionControlGroup, BASE_OFFSET>(),
            FrameProviderIds: FrameProviderIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionControlGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionControlGroupFactory_Impl: Sized {
    fn Create(&mut self, ids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<PerceptionControlGroup>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionControlGroupFactory {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionControlGroupFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionControlGroupFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionControlGroupFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionControlGroupFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IPerceptionControlGroupFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&ids as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionControlGroupFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionControlGroupFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelation_Impl: Sized {
    fn TargetId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Orientation(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Quaternion>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionCorrelation {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionCorrelation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionCorrelation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionCorrelation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionCorrelation_Vtbl {
        unsafe extern "system" fn TargetId<Impl: IPerceptionCorrelation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IPerceptionCorrelation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orientation<Impl: IPerceptionCorrelation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionCorrelation, BASE_OFFSET>(),
            TargetId: TargetId::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionCorrelation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationFactory_Impl: Sized {
    fn Create(&mut self, targetid: &::windows::core::HSTRING, position: &super::super::super::Foundation::Numerics::Vector3, orientation: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<PerceptionCorrelation>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionCorrelationFactory {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionCorrelationFactory";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionCorrelationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionCorrelationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionCorrelationFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IPerceptionCorrelationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, position: super::super::super::Foundation::Numerics::Vector3, orientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&position as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                &*(&orientation as *const <super::super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionCorrelationFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionCorrelationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationGroup_Impl: Sized {
    fn RelativeLocations(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PerceptionCorrelation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionCorrelationGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionCorrelationGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionCorrelationGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionCorrelationGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionCorrelationGroup_Vtbl {
        unsafe extern "system" fn RelativeLocations<Impl: IPerceptionCorrelationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionCorrelationGroup, BASE_OFFSET>(),
            RelativeLocations: RelativeLocations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionCorrelationGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationGroupFactory_Impl: Sized {
    fn Create(&mut self, relativelocations: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<PerceptionCorrelation>>) -> ::windows::core::Result<PerceptionCorrelationGroup>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionCorrelationGroupFactory {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionCorrelationGroupFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionCorrelationGroupFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionCorrelationGroupFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionCorrelationGroupFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IPerceptionCorrelationGroupFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativelocations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&relativelocations as *const <super::super::super::Foundation::Collections::IIterable<PerceptionCorrelation> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<PerceptionCorrelation> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionCorrelationGroupFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionCorrelationGroupFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFaceAuthenticationGroup_Impl: Sized {
    fn FrameProviderIds(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFaceAuthenticationGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFaceAuthenticationGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFaceAuthenticationGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFaceAuthenticationGroup_Vtbl {
        unsafe extern "system" fn FrameProviderIds<Impl: IPerceptionFaceAuthenticationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameProviderIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFaceAuthenticationGroup, BASE_OFFSET>(),
            FrameProviderIds: FrameProviderIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFaceAuthenticationGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFaceAuthenticationGroupFactory_Impl: Sized {
    fn Create(&mut self, ids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, starthandler: &::core::option::Option<PerceptionStartFaceAuthenticationHandler>, stophandler: &::core::option::Option<PerceptionStopFaceAuthenticationHandler>) -> ::windows::core::Result<PerceptionFaceAuthenticationGroup>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFaceAuthenticationGroupFactory {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroupFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFaceAuthenticationGroupFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFaceAuthenticationGroupFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFaceAuthenticationGroupFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IPerceptionFaceAuthenticationGroupFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ids: ::windows::core::RawPtr, starthandler: ::windows::core::RawPtr, stophandler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&ids as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&starthandler as *const <PerceptionStartFaceAuthenticationHandler as ::windows::core::Abi>::Abi as *const <PerceptionStartFaceAuthenticationHandler as ::windows::core::DefaultType>::DefaultType),
                &*(&stophandler as *const <PerceptionStopFaceAuthenticationHandler as ::windows::core::Abi>::Abi as *const <PerceptionStopFaceAuthenticationHandler as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFaceAuthenticationGroupFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFaceAuthenticationGroupFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrame_Impl: Sized {
    fn RelativeTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetRelativeTime(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::ValueSet>;
    fn FrameData(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IMemoryBuffer>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFrame {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFrame_Vtbl {
        unsafe extern "system" fn RelativeTime<Impl: IPerceptionFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeTime<Impl: IPerceptionFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelativeTime(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IPerceptionFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FrameData<Impl: IPerceptionFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrame, BASE_OFFSET>(),
            RelativeTime: RelativeTime::<Impl, IMPL_OFFSET>,
            SetRelativeTime: SetRelativeTime::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            FrameData: FrameData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
pub trait IPerceptionFrameProvider_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn FrameProviderInfo(&mut self) -> ::windows::core::Result<PerceptionFrameProviderInfo>;
    fn Available(&mut self) -> ::windows::core::Result<bool>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IPropertySet>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, value: &::core::option::Option<PerceptionPropertyChangeRequest>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProvider {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProvider";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
impl IPerceptionFrameProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFrameProvider_Vtbl {
        unsafe extern "system" fn FrameProviderInfo<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameProviderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Available<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Available() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn SetProperty<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(&*(&value as *const <PerceptionPropertyChangeRequest as ::windows::core::Abi>::Abi as *const <PerceptionPropertyChangeRequest as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrameProvider, BASE_OFFSET>(),
            FrameProviderInfo: FrameProviderInfo::<Impl, IMPL_OFFSET>,
            Available: Available::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrameProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameProviderInfo_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeviceKind(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDeviceKind(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FrameKind(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrameKind(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Hidden(&mut self) -> ::windows::core::Result<bool>;
    fn SetHidden(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProviderInfo {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProviderInfo";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFrameProviderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProviderInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFrameProviderInfo_Vtbl {
        unsafe extern "system" fn Id<Impl: IPerceptionFrameProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IPerceptionFrameProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IPerceptionFrameProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IPerceptionFrameProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceKind<Impl: IPerceptionFrameProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceKind<Impl: IPerceptionFrameProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceKind(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FrameKind<Impl: IPerceptionFrameProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameKind<Impl: IPerceptionFrameProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameKind(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hidden<Impl: IPerceptionFrameProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hidden() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Impl: IPerceptionFrameProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHidden(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrameProviderInfo, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            DeviceKind: DeviceKind::<Impl, IMPL_OFFSET>,
            SetDeviceKind: SetDeviceKind::<Impl, IMPL_OFFSET>,
            FrameKind: FrameKind::<Impl, IMPL_OFFSET>,
            SetFrameKind: SetFrameKind::<Impl, IMPL_OFFSET>,
            Hidden: Hidden::<Impl, IMPL_OFFSET>,
            SetHidden: SetHidden::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrameProviderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait IPerceptionFrameProviderManager_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn GetFrameProvider(&mut self, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<IPerceptionFrameProvider>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProviderManager {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProviderManager";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl IPerceptionFrameProviderManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProviderManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFrameProviderManager_Vtbl {
        unsafe extern "system" fn GetFrameProvider<Impl: IPerceptionFrameProviderManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameproviderinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameProvider(&*(&frameproviderinfo as *const <PerceptionFrameProviderInfo as ::windows::core::Abi>::Abi as *const <PerceptionFrameProviderInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrameProviderManager, BASE_OFFSET>(),
            GetFrameProvider: GetFrameProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrameProviderManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameProviderManagerServiceStatics_Impl: Sized {
    fn RegisterFrameProviderInfo(&mut self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<()>;
    fn UnregisterFrameProviderInfo(&mut self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<()>;
    fn RegisterFaceAuthenticationGroup(&mut self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, faceauthenticationgroup: &::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<()>;
    fn UnregisterFaceAuthenticationGroup(&mut self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, faceauthenticationgroup: &::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<()>;
    fn RegisterControlGroup(&mut self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, controlgroup: &::core::option::Option<PerceptionControlGroup>) -> ::windows::core::Result<()>;
    fn UnregisterControlGroup(&mut self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, controlgroup: &::core::option::Option<PerceptionControlGroup>) -> ::windows::core::Result<()>;
    fn RegisterCorrelationGroup(&mut self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, correlationgroup: &::core::option::Option<PerceptionCorrelationGroup>) -> ::windows::core::Result<()>;
    fn UnregisterCorrelationGroup(&mut self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, correlationgroup: &::core::option::Option<PerceptionCorrelationGroup>) -> ::windows::core::Result<()>;
    fn UpdateAvailabilityForProvider(&mut self, provider: &::core::option::Option<IPerceptionFrameProvider>, available: bool) -> ::windows::core::Result<()>;
    fn PublishFrameForProvider(&mut self, provider: &::core::option::Option<IPerceptionFrameProvider>, frame: &::core::option::Option<PerceptionFrame>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProviderManagerServiceStatics {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProviderManagerServiceStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFrameProviderManagerServiceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFrameProviderManagerServiceStatics_Vtbl {
        unsafe extern "system" fn RegisterFrameProviderInfo<Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, frameproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterFrameProviderInfo(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&frameproviderinfo as *const <PerceptionFrameProviderInfo as ::windows::core::Abi>::Abi as *const <PerceptionFrameProviderInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterFrameProviderInfo<Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, frameproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterFrameProviderInfo(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&frameproviderinfo as *const <PerceptionFrameProviderInfo as ::windows::core::Abi>::Abi as *const <PerceptionFrameProviderInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterFaceAuthenticationGroup<Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, faceauthenticationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterFaceAuthenticationGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&faceauthenticationgroup as *const <PerceptionFaceAuthenticationGroup as ::windows::core::Abi>::Abi as *const <PerceptionFaceAuthenticationGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterFaceAuthenticationGroup<Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, faceauthenticationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterFaceAuthenticationGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&faceauthenticationgroup as *const <PerceptionFaceAuthenticationGroup as ::windows::core::Abi>::Abi as *const <PerceptionFaceAuthenticationGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterControlGroup<Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, controlgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterControlGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&controlgroup as *const <PerceptionControlGroup as ::windows::core::Abi>::Abi as *const <PerceptionControlGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterControlGroup<Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, controlgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterControlGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&controlgroup as *const <PerceptionControlGroup as ::windows::core::Abi>::Abi as *const <PerceptionControlGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterCorrelationGroup<Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, correlationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCorrelationGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&correlationgroup as *const <PerceptionCorrelationGroup as ::windows::core::Abi>::Abi as *const <PerceptionCorrelationGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterCorrelationGroup<Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, correlationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterCorrelationGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&correlationgroup as *const <PerceptionCorrelationGroup as ::windows::core::Abi>::Abi as *const <PerceptionCorrelationGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateAvailabilityForProvider<Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, available: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateAvailabilityForProvider(&*(&provider as *const <IPerceptionFrameProvider as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProvider as ::windows::core::DefaultType>::DefaultType), available).into()
        }
        unsafe extern "system" fn PublishFrameForProvider<Impl: IPerceptionFrameProviderManagerServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, frame: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PublishFrameForProvider(&*(&provider as *const <IPerceptionFrameProvider as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProvider as ::windows::core::DefaultType>::DefaultType), &*(&frame as *const <PerceptionFrame as ::windows::core::Abi>::Abi as *const <PerceptionFrame as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrameProviderManagerServiceStatics, BASE_OFFSET>(),
            RegisterFrameProviderInfo: RegisterFrameProviderInfo::<Impl, IMPL_OFFSET>,
            UnregisterFrameProviderInfo: UnregisterFrameProviderInfo::<Impl, IMPL_OFFSET>,
            RegisterFaceAuthenticationGroup: RegisterFaceAuthenticationGroup::<Impl, IMPL_OFFSET>,
            UnregisterFaceAuthenticationGroup: UnregisterFaceAuthenticationGroup::<Impl, IMPL_OFFSET>,
            RegisterControlGroup: RegisterControlGroup::<Impl, IMPL_OFFSET>,
            UnregisterControlGroup: UnregisterControlGroup::<Impl, IMPL_OFFSET>,
            RegisterCorrelationGroup: RegisterCorrelationGroup::<Impl, IMPL_OFFSET>,
            UnregisterCorrelationGroup: UnregisterCorrelationGroup::<Impl, IMPL_OFFSET>,
            UpdateAvailabilityForProvider: UpdateAvailabilityForProvider::<Impl, IMPL_OFFSET>,
            PublishFrameForProvider: PublishFrameForProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrameProviderManagerServiceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionPropertyChangeRequest_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Status(&mut self) -> ::windows::core::Result<super::PerceptionFrameSourcePropertyChangeStatus>;
    fn SetStatus(&mut self, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionPropertyChangeRequest {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionPropertyChangeRequest";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionPropertyChangeRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionPropertyChangeRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionPropertyChangeRequest_Vtbl {
        unsafe extern "system" fn Name<Impl: IPerceptionPropertyChangeRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IPerceptionPropertyChangeRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IPerceptionPropertyChangeRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStatus<Impl: IPerceptionPropertyChangeRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPerceptionPropertyChangeRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionPropertyChangeRequest, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionPropertyChangeRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionVideoFrameAllocator_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn AllocateFrame(&mut self) -> ::windows::core::Result<PerceptionFrame>;
    fn CopyFromVideoFrame(&mut self, frame: &::core::option::Option<super::super::super::Media::VideoFrame>) -> ::windows::core::Result<PerceptionFrame>;
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionVideoFrameAllocator {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocator";
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionVideoFrameAllocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionVideoFrameAllocator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionVideoFrameAllocator_Vtbl {
        unsafe extern "system" fn AllocateFrame<Impl: IPerceptionVideoFrameAllocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromVideoFrame<Impl: IPerceptionVideoFrameAllocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyFromVideoFrame(&*(&frame as *const <super::super::super::Media::VideoFrame as ::windows::core::Abi>::Abi as *const <super::super::super::Media::VideoFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionVideoFrameAllocator, BASE_OFFSET>(),
            AllocateFrame: AllocateFrame::<Impl, IMPL_OFFSET>,
            CopyFromVideoFrame: CopyFromVideoFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionVideoFrameAllocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionVideoFrameAllocatorFactory_Impl: Sized {
    fn Create(&mut self, maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: &super::super::super::Foundation::Size, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::Result<PerceptionVideoFrameAllocator>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionVideoFrameAllocatorFactory {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocatorFactory";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionVideoFrameAllocatorFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionVideoFrameAllocatorFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionVideoFrameAllocatorFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IPerceptionVideoFrameAllocatorFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: super::super::super::Foundation::Size, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(maxoutstandingframecountforwrite, format, &*(&resolution as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType), alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionVideoFrameAllocatorFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionVideoFrameAllocatorFactory as ::windows::core::Interface>::IID
    }
}
