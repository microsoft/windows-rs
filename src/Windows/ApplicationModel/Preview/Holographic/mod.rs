#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub struct HolographicApplicationPreview {}
impl HolographicApplicationPreview {
    pub fn IsCurrentViewPresentedOnHolographicDisplay() -> ::windows::runtime::Result<bool> {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn IsHolographicActivation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Activation::IActivatedEventArgs>,
    >(
        activatedeventargs: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                activatedeventargs.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn IHolographicApplicationPreviewStatics<
        R,
        F: FnOnce(&IHolographicApplicationPreviewStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            HolographicApplicationPreview,
            IHolographicApplicationPreviewStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for HolographicApplicationPreview {
    const NAME: &'static str =
        "Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HolographicKeyboardPlacementOverridePreview(::windows::runtime::IInspectable);
impl HolographicKeyboardPlacementOverridePreview {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverride<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Perception::Spatial::SpatialCoordinateSystem,
        >,
        Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        coordinatesystem: Param0,
        topcenterposition: Param1,
        normal: Param2,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                coordinatesystem.into_param().abi(),
                topcenterposition.into_param().abi(),
                normal.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverrideWithMaxSize<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Perception::Spatial::SpatialCoordinateSystem,
        >,
        Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>,
    >(
        &self,
        coordinatesystem: Param0,
        topcenterposition: Param1,
        normal: Param2,
        maxsize: Param3,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                coordinatesystem.into_param().abi(),
                topcenterposition.into_param().abi(),
                normal.into_param().abi(),
                maxsize.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ResetPlacementOverride(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView(
    ) -> ::windows::runtime::Result<HolographicKeyboardPlacementOverridePreview> {
        Self::IHolographicKeyboardPlacementOverridePreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<HolographicKeyboardPlacementOverridePreview>(result__)
        })
    }
    pub fn IHolographicKeyboardPlacementOverridePreviewStatics<
        R,
        F: FnOnce(
            &IHolographicKeyboardPlacementOverridePreviewStatics,
        ) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            HolographicKeyboardPlacementOverridePreview,
            IHolographicKeyboardPlacementOverridePreviewStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicKeyboardPlacementOverridePreview {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview;{c8a8ce3a-dfde-5a14-8d5f-182c526dd9c4})" ) ;
}
unsafe impl ::windows::runtime::Interface for HolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3366506042,
        57310,
        23060,
        [141, 95, 24, 44, 82, 109, 217, 196],
    );
}
impl ::windows::runtime::RuntimeName for HolographicKeyboardPlacementOverridePreview {
    const NAME: &'static str =
        "Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview";
}
impl ::std::convert::From<HolographicKeyboardPlacementOverridePreview>
    for ::windows::runtime::IUnknown
{
    fn from(value: HolographicKeyboardPlacementOverridePreview) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HolographicKeyboardPlacementOverridePreview>
    for ::windows::runtime::IUnknown
{
    fn from(value: &HolographicKeyboardPlacementOverridePreview) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HolographicKeyboardPlacementOverridePreview
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &HolographicKeyboardPlacementOverridePreview
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HolographicKeyboardPlacementOverridePreview>
    for ::windows::runtime::IInspectable
{
    fn from(value: HolographicKeyboardPlacementOverridePreview) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HolographicKeyboardPlacementOverridePreview>
    for ::windows::runtime::IInspectable
{
    fn from(value: &HolographicKeyboardPlacementOverridePreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HolographicKeyboardPlacementOverridePreview
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HolographicKeyboardPlacementOverridePreview
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HolographicKeyboardPlacementOverridePreview {}
unsafe impl ::std::marker::Sync for HolographicKeyboardPlacementOverridePreview {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHolographicApplicationPreviewStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicApplicationPreviewStatics {
    type Vtable = IHolographicApplicationPreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4261643921,
        10810,
        17833,
        [162, 8, 123, 237, 105, 25, 25, 243],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicApplicationPreviewStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        activatedeventargs: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3366506042,
        57310,
        23060,
        [141, 95, 24, 44, 82, 109, 217, 196],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreview_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        topcenterposition: super::super::super::Foundation::Numerics::Vector3,
        normal: super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        topcenterposition: super::super::super::Foundation::Numerics::Vector3,
        normal: super::super::super::Foundation::Numerics::Vector3,
        maxsize: super::super::super::Foundation::Numerics::Vector2,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicKeyboardPlacementOverridePreviewStatics {
    type Vtable = IHolographicKeyboardPlacementOverridePreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        539910201,
        8182,
        23046,
        [170, 196, 165, 226, 79, 163, 236, 75],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
