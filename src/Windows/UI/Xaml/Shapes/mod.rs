#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Ellipse(pub ::windows::core::IInspectable);
impl Ellipse {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Ellipse, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Ellipse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Ellipse;{70e05ac4-d38d-4bab-831f-4a22ef52ac86})");
}
unsafe impl ::windows::core::Interface for Ellipse {
    type Vtable = IEllipse_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70e05ac4_d38d_4bab_831f_4a22ef52ac86);
}
impl ::windows::core::RuntimeName for Ellipse {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Ellipse";
}
impl ::core::convert::From<Ellipse> for ::windows::core::IUnknown {
    fn from(value: Ellipse) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Ellipse> for ::windows::core::IUnknown {
    fn from(value: &Ellipse) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Ellipse> for ::windows::core::IInspectable {
    fn from(value: Ellipse) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Ellipse> for ::windows::core::IInspectable {
    fn from(value: &Ellipse) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Ellipse> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Ellipse) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Ellipse> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Ellipse) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Ellipse> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Ellipse) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Ellipse> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Ellipse) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for &Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Ellipse> for Shape {
    fn from(value: Ellipse) -> Self {
        ::core::convert::Into::<Shape>::into(&value)
    }
}
impl ::core::convert::From<&Ellipse> for Shape {
    fn from(value: &Ellipse) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Ellipse> for super::FrameworkElement {
    fn from(value: Ellipse) -> Self {
        ::core::convert::Into::<super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Ellipse> for super::FrameworkElement {
    fn from(value: &Ellipse) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Ellipse> for super::UIElement {
    fn from(value: Ellipse) -> Self {
        ::core::convert::Into::<super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Ellipse> for super::UIElement {
    fn from(value: &Ellipse) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Ellipse> for super::DependencyObject {
    fn from(value: Ellipse) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Ellipse> for super::DependencyObject {
    fn from(value: &Ellipse) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Ellipse {}
unsafe impl ::core::marker::Sync for Ellipse {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IEllipse(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEllipse {
    type Vtable = IEllipse_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70e05ac4_d38d_4bab_831f_4a22ef52ac86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipse_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILine(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILine {
    type Vtable = ILine_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46a5433d_4ffb_48df_8732_4e15c834816b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILine_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineStatics {
    type Vtable = ILineStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x267c123d_6ea4_4c50_8b1d_50207aff1e8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPath(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPath {
    type Vtable = IPath_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78883609_3d57_4f3c_b8a5_6cabcac9711f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPath_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPathFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPathFactory {
    type Vtable = IPathFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2340a4e3_5a86_4fc6_9a50_cbb93b828766);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPathStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPathStatics {
    type Vtable = IPathStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf627e59d_87dc_4142_81f1_97fc7ff8641c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPolygon(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPolygon {
    type Vtable = IPolygon_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3755c19_2e4d_4bcc_8d34_86871957fa01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolygon_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Media::FillRule) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Media::FillRule) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPolygonStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPolygonStatics {
    type Vtable = IPolygonStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x362a8aab_d463_4366_9e1a_beba72810fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolygonStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPolyline(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPolyline {
    type Vtable = IPolyline_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91dc62f8_42b3_47f3_8476_c55124a7c4c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyline_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Media::FillRule) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Media::FillRule) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPolylineStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPolylineStatics {
    type Vtable = IPolylineStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7aa2cd1_a26c_43b0_aaa5_822fa64a11b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolylineStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRectangle(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRectangle {
    type Vtable = IRectangle_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x855bc230_8a11_4e18_a136_4bc21c7827b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangle_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRectangleStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRectangleStatics {
    type Vtable = IRectangleStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f25aa53_bb3a_4c3c_89db_6fbc0d1fa0cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShape(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShape {
    type Vtable = IShape_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x786f2b75_9aa0_454d_ae06_a2466e37c832);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShape_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Media::PenLineJoin) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Media::PenLineJoin) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Media::Stretch) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Media::Stretch) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShape2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShape2 {
    type Vtable = IShape2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97248dba_49f2_49a4_a5dd_164df824db14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShape2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShapeFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShapeFactory {
    type Vtable = IShapeFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b717613_f6aa_48d5_9588_e1d188eacbc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShapeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShapeStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShapeStatics {
    type Vtable = IShapeStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d7b4c55_9df3_48dc_9194_9d306faa6089);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShapeStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Line(pub ::windows::core::IInspectable);
impl Line {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Line, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn X1(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetX1(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Y1(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetY1(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn X2(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetX2(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Y2(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetY2(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn X1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Y1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn X2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Y2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ILineStatics<R, F: FnOnce(&ILineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Line, ILineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Line {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Line;{46a5433d-4ffb-48df-8732-4e15c834816b})");
}
unsafe impl ::windows::core::Interface for Line {
    type Vtable = ILine_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46a5433d_4ffb_48df_8732_4e15c834816b);
}
impl ::windows::core::RuntimeName for Line {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Line";
}
impl ::core::convert::From<Line> for ::windows::core::IUnknown {
    fn from(value: Line) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Line> for ::windows::core::IUnknown {
    fn from(value: &Line) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Line {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Line> for ::windows::core::IInspectable {
    fn from(value: Line) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Line> for ::windows::core::IInspectable {
    fn from(value: &Line) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Line {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Line> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Line) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Line> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Line) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Line> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Line) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Line> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Line) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for &Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Line> for Shape {
    fn from(value: Line) -> Self {
        ::core::convert::Into::<Shape>::into(&value)
    }
}
impl ::core::convert::From<&Line> for Shape {
    fn from(value: &Line) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Line {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Line> for super::FrameworkElement {
    fn from(value: Line) -> Self {
        ::core::convert::Into::<super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Line> for super::FrameworkElement {
    fn from(value: &Line) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Line> for super::UIElement {
    fn from(value: Line) -> Self {
        ::core::convert::Into::<super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Line> for super::UIElement {
    fn from(value: &Line) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Line> for super::DependencyObject {
    fn from(value: Line) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Line> for super::DependencyObject {
    fn from(value: &Line) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Line {}
unsafe impl ::core::marker::Sync for Line {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Path(pub ::windows::core::IInspectable);
impl Path {
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Data(&self) -> ::windows::core::Result<super::Media::Geometry> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Geometry>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Geometry>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DataProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPathStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<Path> {
        Self::IPathFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<Path>(result__)
        })
    }
    pub fn IPathStatics<R, F: FnOnce(&IPathStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Path, IPathStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPathFactory<R, F: FnOnce(&IPathFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Path, IPathFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Path {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Path;{78883609-3d57-4f3c-b8a5-6cabcac9711f})");
}
unsafe impl ::windows::core::Interface for Path {
    type Vtable = IPath_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78883609_3d57_4f3c_b8a5_6cabcac9711f);
}
impl ::windows::core::RuntimeName for Path {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Path";
}
impl ::core::convert::From<Path> for ::windows::core::IUnknown {
    fn from(value: Path) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Path> for ::windows::core::IUnknown {
    fn from(value: &Path) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Path {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Path> for ::windows::core::IInspectable {
    fn from(value: Path) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Path> for ::windows::core::IInspectable {
    fn from(value: &Path) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Path {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Path> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Path) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Path> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Path) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Path> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Path) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Path> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Path) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for &Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Path> for Shape {
    fn from(value: Path) -> Self {
        ::core::convert::Into::<Shape>::into(&value)
    }
}
impl ::core::convert::From<&Path> for Shape {
    fn from(value: &Path) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Path {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Path> for super::FrameworkElement {
    fn from(value: Path) -> Self {
        ::core::convert::Into::<super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Path> for super::FrameworkElement {
    fn from(value: &Path) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Path> for super::UIElement {
    fn from(value: Path) -> Self {
        ::core::convert::Into::<super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Path> for super::UIElement {
    fn from(value: &Path) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Path> for super::DependencyObject {
    fn from(value: Path) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Path> for super::DependencyObject {
    fn from(value: &Path) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Path {}
unsafe impl ::core::marker::Sync for Path {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Polygon(pub ::windows::core::IInspectable);
impl Polygon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Polygon, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FillRule(&self) -> ::windows::core::Result<super::Media::FillRule> {
        let this = self;
        unsafe {
            let mut result__: super::Media::FillRule = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::FillRule>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFillRule(&self, value: super::Media::FillRule) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn Points(&self) -> ::windows::core::Result<super::Media::PointCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PointCollection>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn SetPoints<'a, Param0: ::windows::core::IntoParam<'a, super::Media::PointCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FillRuleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolygonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn PointsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolygonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPolygonStatics<R, F: FnOnce(&IPolygonStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Polygon, IPolygonStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Polygon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Polygon;{e3755c19-2e4d-4bcc-8d34-86871957fa01})");
}
unsafe impl ::windows::core::Interface for Polygon {
    type Vtable = IPolygon_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3755c19_2e4d_4bcc_8d34_86871957fa01);
}
impl ::windows::core::RuntimeName for Polygon {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Polygon";
}
impl ::core::convert::From<Polygon> for ::windows::core::IUnknown {
    fn from(value: Polygon) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Polygon> for ::windows::core::IUnknown {
    fn from(value: &Polygon) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Polygon> for ::windows::core::IInspectable {
    fn from(value: Polygon) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Polygon> for ::windows::core::IInspectable {
    fn from(value: &Polygon) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Polygon> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Polygon) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Polygon> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Polygon) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Polygon> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Polygon) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Polygon> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Polygon) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for &Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        ::core::convert::Into::<Shape>::into(&value)
    }
}
impl ::core::convert::From<&Polygon> for Shape {
    fn from(value: &Polygon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Polygon> for super::FrameworkElement {
    fn from(value: Polygon) -> Self {
        ::core::convert::Into::<super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Polygon> for super::FrameworkElement {
    fn from(value: &Polygon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Polygon> for super::UIElement {
    fn from(value: Polygon) -> Self {
        ::core::convert::Into::<super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Polygon> for super::UIElement {
    fn from(value: &Polygon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Polygon> for super::DependencyObject {
    fn from(value: Polygon) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Polygon> for super::DependencyObject {
    fn from(value: &Polygon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Polygon {}
unsafe impl ::core::marker::Sync for Polygon {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Polyline(pub ::windows::core::IInspectable);
impl Polyline {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Polyline, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FillRule(&self) -> ::windows::core::Result<super::Media::FillRule> {
        let this = self;
        unsafe {
            let mut result__: super::Media::FillRule = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::FillRule>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFillRule(&self, value: super::Media::FillRule) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn Points(&self) -> ::windows::core::Result<super::Media::PointCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PointCollection>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn SetPoints<'a, Param0: ::windows::core::IntoParam<'a, super::Media::PointCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FillRuleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolylineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn PointsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolylineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPolylineStatics<R, F: FnOnce(&IPolylineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Polyline, IPolylineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Polyline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Polyline;{91dc62f8-42b3-47f3-8476-c55124a7c4c6})");
}
unsafe impl ::windows::core::Interface for Polyline {
    type Vtable = IPolyline_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91dc62f8_42b3_47f3_8476_c55124a7c4c6);
}
impl ::windows::core::RuntimeName for Polyline {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Polyline";
}
impl ::core::convert::From<Polyline> for ::windows::core::IUnknown {
    fn from(value: Polyline) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Polyline> for ::windows::core::IUnknown {
    fn from(value: &Polyline) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Polyline> for ::windows::core::IInspectable {
    fn from(value: Polyline) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Polyline> for ::windows::core::IInspectable {
    fn from(value: &Polyline) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Polyline> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Polyline) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Polyline> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Polyline) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Polyline> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Polyline) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Polyline> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Polyline) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for &Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Polyline> for Shape {
    fn from(value: Polyline) -> Self {
        ::core::convert::Into::<Shape>::into(&value)
    }
}
impl ::core::convert::From<&Polyline> for Shape {
    fn from(value: &Polyline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Polyline> for super::FrameworkElement {
    fn from(value: Polyline) -> Self {
        ::core::convert::Into::<super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Polyline> for super::FrameworkElement {
    fn from(value: &Polyline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Polyline> for super::UIElement {
    fn from(value: Polyline) -> Self {
        ::core::convert::Into::<super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Polyline> for super::UIElement {
    fn from(value: &Polyline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Polyline> for super::DependencyObject {
    fn from(value: Polyline) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Polyline> for super::DependencyObject {
    fn from(value: &Polyline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Polyline {}
unsafe impl ::core::marker::Sync for Polyline {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Rectangle(pub ::windows::core::IInspectable);
impl Rectangle {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Rectangle, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn RadiusX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRadiusX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RadiusY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRadiusY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RadiusXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRectangleStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RadiusYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRectangleStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IRectangleStatics<R, F: FnOnce(&IRectangleStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Rectangle, IRectangleStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Rectangle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Rectangle;{855bc230-8a11-4e18-a136-4bc21c7827b0})");
}
unsafe impl ::windows::core::Interface for Rectangle {
    type Vtable = IRectangle_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x855bc230_8a11_4e18_a136_4bc21c7827b0);
}
impl ::windows::core::RuntimeName for Rectangle {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Rectangle";
}
impl ::core::convert::From<Rectangle> for ::windows::core::IUnknown {
    fn from(value: Rectangle) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Rectangle> for ::windows::core::IUnknown {
    fn from(value: &Rectangle) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Rectangle> for ::windows::core::IInspectable {
    fn from(value: Rectangle) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Rectangle> for ::windows::core::IInspectable {
    fn from(value: &Rectangle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Rectangle> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Rectangle) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Rectangle> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Rectangle) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Rectangle> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Rectangle) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Rectangle> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Rectangle) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for &Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Rectangle> for Shape {
    fn from(value: Rectangle) -> Self {
        ::core::convert::Into::<Shape>::into(&value)
    }
}
impl ::core::convert::From<&Rectangle> for Shape {
    fn from(value: &Rectangle) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Rectangle> for super::FrameworkElement {
    fn from(value: Rectangle) -> Self {
        ::core::convert::Into::<super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Rectangle> for super::FrameworkElement {
    fn from(value: &Rectangle) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Rectangle> for super::UIElement {
    fn from(value: Rectangle) -> Self {
        ::core::convert::Into::<super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Rectangle> for super::UIElement {
    fn from(value: &Rectangle) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Rectangle> for super::DependencyObject {
    fn from(value: Rectangle) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Rectangle> for super::DependencyObject {
    fn from(value: &Rectangle) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Rectangle {}
unsafe impl ::core::marker::Sync for Rectangle {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Shape(pub ::windows::core::IInspectable);
impl Shape {
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Stroke(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStroke<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn StrokeMiterLimit(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetStrokeMiterLimit(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn StrokeThickness(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetStrokeThickness(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StrokeStartLineCap(&self) -> ::windows::core::Result<super::Media::PenLineCap> {
        let this = self;
        unsafe {
            let mut result__: super::Media::PenLineCap = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PenLineCap>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStrokeStartLineCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StrokeEndLineCap(&self) -> ::windows::core::Result<super::Media::PenLineCap> {
        let this = self;
        unsafe {
            let mut result__: super::Media::PenLineCap = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PenLineCap>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStrokeEndLineCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StrokeLineJoin(&self) -> ::windows::core::Result<super::Media::PenLineJoin> {
        let this = self;
        unsafe {
            let mut result__: super::Media::PenLineJoin = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PenLineJoin>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStrokeLineJoin(&self, value: super::Media::PenLineJoin) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn StrokeDashOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetStrokeDashOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StrokeDashCap(&self) -> ::windows::core::Result<super::Media::PenLineCap> {
        let this = self;
        unsafe {
            let mut result__: super::Media::PenLineCap = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PenLineCap>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStrokeDashCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn StrokeDashArray(&self) -> ::windows::core::Result<super::Media::DoubleCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::DoubleCollection>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn SetStrokeDashArray<'a, Param0: ::windows::core::IntoParam<'a, super::Media::DoubleCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Stretch(&self) -> ::windows::core::Result<super::Media::Stretch> {
        let this = self;
        unsafe {
            let mut result__: super::Media::Stretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Stretch>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStretch(&self, value: super::Media::Stretch) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn GeometryTransform(&self) -> ::windows::core::Result<super::Media::Transform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Transform>(result__)
        }
    }
    pub fn FillProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StrokeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StrokeMiterLimitProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StrokeThicknessProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StrokeStartLineCapProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StrokeEndLineCapProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StrokeLineJoinProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StrokeDashOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StrokeDashCapProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StrokeDashArrayProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StretchProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn GetAlphaMask(&self) -> ::windows::core::Result<super::super::Composition::CompositionBrush> {
        let this = &::windows::core::Interface::cast::<IShape2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Composition::CompositionBrush>(result__)
        }
    }
    pub fn IShapeStatics<R, F: FnOnce(&IShapeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Shape, IShapeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Shape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Shape;{786f2b75-9aa0-454d-ae06-a2466e37c832})");
}
unsafe impl ::windows::core::Interface for Shape {
    type Vtable = IShape_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x786f2b75_9aa0_454d_ae06_a2466e37c832);
}
impl ::windows::core::RuntimeName for Shape {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Shape";
}
impl ::core::convert::From<Shape> for ::windows::core::IUnknown {
    fn from(value: Shape) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Shape> for ::windows::core::IUnknown {
    fn from(value: &Shape) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Shape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Shape> for ::windows::core::IInspectable {
    fn from(value: Shape) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Shape> for ::windows::core::IInspectable {
    fn from(value: &Shape) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Shape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Shape> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Shape) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Shape> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Shape) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Shape> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Shape) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Shape> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Shape) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for &Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Shape> for super::FrameworkElement {
    fn from(value: Shape) -> Self {
        ::core::convert::Into::<super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Shape> for super::FrameworkElement {
    fn from(value: &Shape) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Shape> for super::UIElement {
    fn from(value: Shape) -> Self {
        ::core::convert::Into::<super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Shape> for super::UIElement {
    fn from(value: &Shape) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Shape> for super::DependencyObject {
    fn from(value: Shape) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Shape> for super::DependencyObject {
    fn from(value: &Shape) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Shape {}
unsafe impl ::core::marker::Sync for Shape {}
