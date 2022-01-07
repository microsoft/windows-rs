#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionFrameKindStaticsImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Depth(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Infrared(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownPerceptionFrameKindStatics {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IKnownPerceptionFrameKindStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownPerceptionFrameKindStaticsVtbl {
    pub const fn new<Impl: IKnownPerceptionFrameKindStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKnownPerceptionFrameKindStaticsVtbl {
        unsafe extern "system" fn Color<Impl: IKnownPerceptionFrameKindStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Depth<Impl: IKnownPerceptionFrameKindStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Depth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Infrared<Impl: IKnownPerceptionFrameKindStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Infrared() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKnownPerceptionFrameKindStatics>, base.5, Color::<Impl, OFFSET>, Depth::<Impl, OFFSET>, Infrared::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionControlGroupImpl: Sized {
    fn FrameProviderIds(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionControlGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionControlGroup";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionControlGroupVtbl {
    pub const fn new<Impl: IPerceptionControlGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionControlGroupVtbl {
        unsafe extern "system" fn FrameProviderIds<Impl: IPerceptionControlGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameProviderIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionControlGroup>, base.5, FrameProviderIds::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionControlGroupFactoryImpl: Sized {
    fn Create(&self, ids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<PerceptionControlGroup>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionControlGroupFactory {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionControlGroupFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionControlGroupFactoryVtbl {
    pub const fn new<Impl: IPerceptionControlGroupFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionControlGroupFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPerceptionControlGroupFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&ids as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionControlGroupFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationImpl: Sized {
    fn TargetId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Orientation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Quaternion>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionCorrelation {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionCorrelation";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionCorrelationVtbl {
    pub const fn new<Impl: IPerceptionCorrelationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionCorrelationVtbl {
        unsafe extern "system" fn TargetId<Impl: IPerceptionCorrelationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IPerceptionCorrelationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orientation<Impl: IPerceptionCorrelationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionCorrelation>, base.5, TargetId::<Impl, OFFSET>, Position::<Impl, OFFSET>, Orientation::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationFactoryImpl: Sized {
    fn Create(&self, targetid: &::windows::core::HSTRING, position: &super::super::super::Foundation::Numerics::Vector3, orientation: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<PerceptionCorrelation>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionCorrelationFactory {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionCorrelationFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionCorrelationFactoryVtbl {
    pub const fn new<Impl: IPerceptionCorrelationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionCorrelationFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPerceptionCorrelationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, position: super::super::super::Foundation::Numerics::Vector3, orientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionCorrelationFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationGroupImpl: Sized {
    fn RelativeLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PerceptionCorrelation>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionCorrelationGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionCorrelationGroup";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionCorrelationGroupVtbl {
    pub const fn new<Impl: IPerceptionCorrelationGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionCorrelationGroupVtbl {
        unsafe extern "system" fn RelativeLocations<Impl: IPerceptionCorrelationGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativeLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionCorrelationGroup>, base.5, RelativeLocations::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationGroupFactoryImpl: Sized {
    fn Create(&self, relativelocations: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<PerceptionCorrelation>>) -> ::windows::core::Result<PerceptionCorrelationGroup>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionCorrelationGroupFactory {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionCorrelationGroupFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionCorrelationGroupFactoryVtbl {
    pub const fn new<Impl: IPerceptionCorrelationGroupFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionCorrelationGroupFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPerceptionCorrelationGroupFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativelocations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&relativelocations as *const <super::super::super::Foundation::Collections::IIterable<PerceptionCorrelation> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<PerceptionCorrelation> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionCorrelationGroupFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFaceAuthenticationGroupImpl: Sized {
    fn FrameProviderIds(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFaceAuthenticationGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroup";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFaceAuthenticationGroupVtbl {
    pub const fn new<Impl: IPerceptionFaceAuthenticationGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionFaceAuthenticationGroupVtbl {
        unsafe extern "system" fn FrameProviderIds<Impl: IPerceptionFaceAuthenticationGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameProviderIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionFaceAuthenticationGroup>, base.5, FrameProviderIds::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFaceAuthenticationGroupFactoryImpl: Sized {
    fn Create(&self, ids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, starthandler: &::core::option::Option<PerceptionStartFaceAuthenticationHandler>, stophandler: &::core::option::Option<PerceptionStopFaceAuthenticationHandler>) -> ::windows::core::Result<PerceptionFaceAuthenticationGroup>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFaceAuthenticationGroupFactory {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroupFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFaceAuthenticationGroupFactoryVtbl {
    pub const fn new<Impl: IPerceptionFaceAuthenticationGroupFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionFaceAuthenticationGroupFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPerceptionFaceAuthenticationGroupFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ids: ::windows::core::RawPtr, starthandler: ::windows::core::RawPtr, stophandler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionFaceAuthenticationGroupFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameImpl: Sized {
    fn RelativeTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetRelativeTime(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::ValueSet>;
    fn FrameData(&self) -> ::windows::core::Result<super::super::super::Foundation::IMemoryBuffer>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFrame {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrame";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFrameVtbl {
    pub const fn new<Impl: IPerceptionFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionFrameVtbl {
        unsafe extern "system" fn RelativeTime<Impl: IPerceptionFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeTime<Impl: IPerceptionFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRelativeTime(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IPerceptionFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameData<Impl: IPerceptionFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionFrame>, base.5, RelativeTime::<Impl, OFFSET>, SetRelativeTime::<Impl, OFFSET>, Properties::<Impl, OFFSET>, FrameData::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait IPerceptionFrameProviderImpl: Sized + IClosableImpl {
    fn FrameProviderInfo(&self) -> ::windows::core::Result<PerceptionFrameProviderInfo>;
    fn Available(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IPropertySet>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn SetProperty(&self, value: &::core::option::Option<PerceptionPropertyChangeRequest>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProvider {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProvider";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl IPerceptionFrameProviderVtbl {
    pub const fn new<Impl: IPerceptionFrameProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionFrameProviderVtbl {
        unsafe extern "system" fn FrameProviderInfo<Impl: IPerceptionFrameProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameProviderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Available<Impl: IPerceptionFrameProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Available() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPerceptionFrameProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IPerceptionFrameProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IPerceptionFrameProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn SetProperty<Impl: IPerceptionFrameProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProperty(&*(&value as *const <PerceptionPropertyChangeRequest as ::windows::core::Abi>::Abi as *const <PerceptionPropertyChangeRequest as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionFrameProvider>, base.5, FrameProviderInfo::<Impl, OFFSET>, Available::<Impl, OFFSET>, Properties::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameProviderInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeviceKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDeviceKind(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FrameKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrameKind(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Hidden(&self) -> ::windows::core::Result<bool>;
    fn SetHidden(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProviderInfo {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProviderInfo";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFrameProviderInfoVtbl {
    pub const fn new<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionFrameProviderInfoVtbl {
        unsafe extern "system" fn Id<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceKind<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceKind<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDeviceKind(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FrameKind<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameKind<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFrameKind(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hidden<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hidden() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Impl: IPerceptionFrameProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHidden(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionFrameProviderInfo>, base.5, Id::<Impl, OFFSET>, SetId::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, SetDisplayName::<Impl, OFFSET>, DeviceKind::<Impl, OFFSET>, SetDeviceKind::<Impl, OFFSET>, FrameKind::<Impl, OFFSET>, SetFrameKind::<Impl, OFFSET>, Hidden::<Impl, OFFSET>, SetHidden::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait IPerceptionFrameProviderManagerImpl: Sized + IClosableImpl {
    fn GetFrameProvider(&self, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<IPerceptionFrameProvider>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProviderManager {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProviderManager";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl IPerceptionFrameProviderManagerVtbl {
    pub const fn new<Impl: IPerceptionFrameProviderManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionFrameProviderManagerVtbl {
        unsafe extern "system" fn GetFrameProvider<Impl: IPerceptionFrameProviderManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frameproviderinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFrameProvider(&*(&frameproviderinfo as *const <PerceptionFrameProviderInfo as ::windows::core::Abi>::Abi as *const <PerceptionFrameProviderInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionFrameProviderManager>, base.5, GetFrameProvider::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameProviderManagerServiceStaticsImpl: Sized {
    fn RegisterFrameProviderInfo(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<()>;
    fn UnregisterFrameProviderInfo(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<()>;
    fn RegisterFaceAuthenticationGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, faceauthenticationgroup: &::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<()>;
    fn UnregisterFaceAuthenticationGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, faceauthenticationgroup: &::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<()>;
    fn RegisterControlGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, controlgroup: &::core::option::Option<PerceptionControlGroup>) -> ::windows::core::Result<()>;
    fn UnregisterControlGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, controlgroup: &::core::option::Option<PerceptionControlGroup>) -> ::windows::core::Result<()>;
    fn RegisterCorrelationGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, correlationgroup: &::core::option::Option<PerceptionCorrelationGroup>) -> ::windows::core::Result<()>;
    fn UnregisterCorrelationGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, correlationgroup: &::core::option::Option<PerceptionCorrelationGroup>) -> ::windows::core::Result<()>;
    fn UpdateAvailabilityForProvider(&self, provider: &::core::option::Option<IPerceptionFrameProvider>, available: bool) -> ::windows::core::Result<()>;
    fn PublishFrameForProvider(&self, provider: &::core::option::Option<IPerceptionFrameProvider>, frame: &::core::option::Option<PerceptionFrame>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProviderManagerServiceStatics {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProviderManagerServiceStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFrameProviderManagerServiceStaticsVtbl {
    pub const fn new<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionFrameProviderManagerServiceStaticsVtbl {
        unsafe extern "system" fn RegisterFrameProviderInfo<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, frameproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RegisterFrameProviderInfo(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&frameproviderinfo as *const <PerceptionFrameProviderInfo as ::windows::core::Abi>::Abi as *const <PerceptionFrameProviderInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterFrameProviderInfo<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, frameproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UnregisterFrameProviderInfo(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&frameproviderinfo as *const <PerceptionFrameProviderInfo as ::windows::core::Abi>::Abi as *const <PerceptionFrameProviderInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterFaceAuthenticationGroup<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, faceauthenticationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RegisterFaceAuthenticationGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&faceauthenticationgroup as *const <PerceptionFaceAuthenticationGroup as ::windows::core::Abi>::Abi as *const <PerceptionFaceAuthenticationGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterFaceAuthenticationGroup<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, faceauthenticationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UnregisterFaceAuthenticationGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&faceauthenticationgroup as *const <PerceptionFaceAuthenticationGroup as ::windows::core::Abi>::Abi as *const <PerceptionFaceAuthenticationGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterControlGroup<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, controlgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RegisterControlGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&controlgroup as *const <PerceptionControlGroup as ::windows::core::Abi>::Abi as *const <PerceptionControlGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterControlGroup<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, controlgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UnregisterControlGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&controlgroup as *const <PerceptionControlGroup as ::windows::core::Abi>::Abi as *const <PerceptionControlGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterCorrelationGroup<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, correlationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RegisterCorrelationGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&correlationgroup as *const <PerceptionCorrelationGroup as ::windows::core::Abi>::Abi as *const <PerceptionCorrelationGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterCorrelationGroup<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, correlationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UnregisterCorrelationGroup(&*(&manager as *const <IPerceptionFrameProviderManager as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProviderManager as ::windows::core::DefaultType>::DefaultType), &*(&correlationgroup as *const <PerceptionCorrelationGroup as ::windows::core::Abi>::Abi as *const <PerceptionCorrelationGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateAvailabilityForProvider<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, available: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateAvailabilityForProvider(&*(&provider as *const <IPerceptionFrameProvider as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProvider as ::windows::core::DefaultType>::DefaultType), available).into()
        }
        unsafe extern "system" fn PublishFrameForProvider<Impl: IPerceptionFrameProviderManagerServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, frame: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PublishFrameForProvider(&*(&provider as *const <IPerceptionFrameProvider as ::windows::core::Abi>::Abi as *const <IPerceptionFrameProvider as ::windows::core::DefaultType>::DefaultType), &*(&frame as *const <PerceptionFrame as ::windows::core::Abi>::Abi as *const <PerceptionFrame as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPerceptionFrameProviderManagerServiceStatics>,
            base.5,
            RegisterFrameProviderInfo::<Impl, OFFSET>,
            UnregisterFrameProviderInfo::<Impl, OFFSET>,
            RegisterFaceAuthenticationGroup::<Impl, OFFSET>,
            UnregisterFaceAuthenticationGroup::<Impl, OFFSET>,
            RegisterControlGroup::<Impl, OFFSET>,
            UnregisterControlGroup::<Impl, OFFSET>,
            RegisterCorrelationGroup::<Impl, OFFSET>,
            UnregisterCorrelationGroup::<Impl, OFFSET>,
            UpdateAvailabilityForProvider::<Impl, OFFSET>,
            PublishFrameForProvider::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionPropertyChangeRequestImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Status(&self) -> ::windows::core::Result<super::PerceptionFrameSourcePropertyChangeStatus>;
    fn SetStatus(&self, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionPropertyChangeRequest {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionPropertyChangeRequest";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionPropertyChangeRequestVtbl {
    pub const fn new<Impl: IPerceptionPropertyChangeRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionPropertyChangeRequestVtbl {
        unsafe extern "system" fn Name<Impl: IPerceptionPropertyChangeRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IPerceptionPropertyChangeRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IPerceptionPropertyChangeRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IPerceptionPropertyChangeRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPerceptionPropertyChangeRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionPropertyChangeRequest>, base.5, Name::<Impl, OFFSET>, Value::<Impl, OFFSET>, Status::<Impl, OFFSET>, SetStatus::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionVideoFrameAllocatorImpl: Sized + IClosableImpl {
    fn AllocateFrame(&self) -> ::windows::core::Result<PerceptionFrame>;
    fn CopyFromVideoFrame(&self, frame: &::core::option::Option<super::super::super::Media::VideoFrame>) -> ::windows::core::Result<PerceptionFrame>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionVideoFrameAllocator {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocator";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionVideoFrameAllocatorVtbl {
    pub const fn new<Impl: IPerceptionVideoFrameAllocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionVideoFrameAllocatorVtbl {
        unsafe extern "system" fn AllocateFrame<Impl: IPerceptionVideoFrameAllocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocateFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromVideoFrame<Impl: IPerceptionVideoFrameAllocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyFromVideoFrame(&*(&frame as *const <super::super::super::Media::VideoFrame as ::windows::core::Abi>::Abi as *const <super::super::super::Media::VideoFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionVideoFrameAllocator>, base.5, AllocateFrame::<Impl, OFFSET>, CopyFromVideoFrame::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionVideoFrameAllocatorFactoryImpl: Sized {
    fn Create(&self, maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: &super::super::super::Foundation::Size, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::Result<PerceptionVideoFrameAllocator>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionVideoFrameAllocatorFactory {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocatorFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionVideoFrameAllocatorFactoryVtbl {
    pub const fn new<Impl: IPerceptionVideoFrameAllocatorFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionVideoFrameAllocatorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPerceptionVideoFrameAllocatorFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: super::super::super::Foundation::Size, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(maxoutstandingframecountforwrite, format, &*(&resolution as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType), alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionVideoFrameAllocatorFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
