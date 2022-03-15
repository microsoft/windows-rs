#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
#[repr(transparent)]
pub struct Ellipse(::windows::core::IUnknown);
impl Ellipse {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Ellipse, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Ellipse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Ellipse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Ellipse {}
impl ::core::fmt::Debug for Ellipse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Ellipse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Ellipse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Ellipse;{70e05ac4-d38d-4bab-831f-4a22ef52ac86})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Ellipse {
    type Vtable = IEllipse_Vtbl;
    const IID: ::windows::core::GUID = <IEllipse as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Ellipse {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Ellipse";
}
impl ::core::convert::From<Ellipse> for ::windows::core::IUnknown {
    fn from(value: Ellipse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Ellipse> for ::windows::core::IUnknown {
    fn from(value: &Ellipse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Ellipse> for ::windows::core::IInspectable {
    fn from(value: Ellipse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Ellipse> for ::windows::core::IInspectable {
    fn from(value: &Ellipse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Ellipse> for Shape {
    fn from(value: &Ellipse) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl ::core::convert::From<Ellipse> for super::FrameworkElement {
    fn from(value: Ellipse) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Ellipse> for super::FrameworkElement {
    fn from(value: &Ellipse) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Ellipse> for super::UIElement {
    fn from(value: Ellipse) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Ellipse> for super::UIElement {
    fn from(value: &Ellipse) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Ellipse> for super::DependencyObject {
    fn from(value: Ellipse) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Ellipse> for super::DependencyObject {
    fn from(value: &Ellipse) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Ellipse {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Ellipse {}
unsafe impl ::core::marker::Sync for Ellipse {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEllipse(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEllipse {
    type Vtable = IEllipse_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70e05ac4_d38d_4bab_831f_4a22ef52ac86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipse_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILine(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILine {
    type Vtable = ILine_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46a5433d_4ffb_48df_8732_4e15c834816b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILine_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub X1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetX1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Y1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetY1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub X2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Y2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineStatics {
    type Vtable = ILineStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x267c123d_6ea4_4c50_8b1d_50207aff1e8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub X1Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Y1Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub X2Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Y2Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPath(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPath {
    type Vtable = IPath_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78883609_3d57_4f3c_b8a5_6cabcac9711f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPath_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Data: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPathFactory {
    type Vtable = IPathFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2340a4e3_5a86_4fc6_9a50_cbb93b828766);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPathStatics {
    type Vtable = IPathStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf627e59d_87dc_4142_81f1_97fc7ff8641c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DataProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolygon(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPolygon {
    type Vtable = IPolygon_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3755c19_2e4d_4bcc_8d34_86871957fa01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolygon_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Media::FillRule) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FillRule: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Media::FillRule) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFillRule: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub Points: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    Points: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub SetPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    SetPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolygonStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPolygonStatics {
    type Vtable = IPolygonStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x362a8aab_d463_4366_9e1a_beba72810fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolygonStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FillRuleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PointsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyline(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPolyline {
    type Vtable = IPolyline_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91dc62f8_42b3_47f3_8476_c55124a7c4c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyline_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Media::FillRule) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FillRule: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Media::FillRule) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFillRule: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub Points: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    Points: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub SetPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    SetPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolylineStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPolylineStatics {
    type Vtable = IPolylineStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7aa2cd1_a26c_43b0_aaa5_822fa64a11b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolylineStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FillRuleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PointsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRectangle(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRectangle {
    type Vtable = IRectangle_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x855bc230_8a11_4e18_a136_4bc21c7827b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangle_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRectangleStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRectangleStatics {
    type Vtable = IRectangleStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f25aa53_bb3a_4c3c_89db_6fbc0d1fa0cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RadiusXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RadiusYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShape(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShape {
    type Vtable = IShape_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x786f2b75_9aa0_454d_ae06_a2466e37c832);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShape_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Fill: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Fill: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFill: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFill: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stroke: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStroke: usize,
    pub StrokeMiterLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetStrokeMiterLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub StrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StrokeStartLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StrokeStartLineCap: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStrokeStartLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStrokeStartLineCap: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StrokeEndLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StrokeEndLineCap: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStrokeEndLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStrokeEndLineCap: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StrokeLineJoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Media::PenLineJoin) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StrokeLineJoin: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStrokeLineJoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Media::PenLineJoin) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStrokeLineJoin: usize,
    pub StrokeDashOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetStrokeDashOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StrokeDashCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StrokeDashCap: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStrokeDashCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Media::PenLineCap) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStrokeDashCap: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub StrokeDashArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    StrokeDashArray: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub SetStrokeDashArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    SetStrokeDashArray: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Media::Stretch) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Media::Stretch) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub GeometryTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    GeometryTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShape2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShape2 {
    type Vtable = IShape2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97248dba_49f2_49a4_a5dd_164df824db14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShape2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub GetAlphaMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetAlphaMask: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShapeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShapeFactory {
    type Vtable = IShapeFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b717613_f6aa_48d5_9588_e1d188eacbc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShapeFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShapeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShapeStatics {
    type Vtable = IShapeStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d7b4c55_9df3_48dc_9194_9d306faa6089);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShapeStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FillProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeMiterLimitProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeThicknessProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeStartLineCapProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeEndLineCapProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeLineJoinProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeDashOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeDashCapProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeDashArrayProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
#[repr(transparent)]
pub struct Line(::windows::core::IUnknown);
impl Line {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Line, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn X1(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).X1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn SetX1(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetX1)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn Y1(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Y1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn SetY1(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetY1)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn X2(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).X2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn SetX2(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetX2)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn Y2(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Y2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn SetY2(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetY2)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn X1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).X1Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn Y1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Y1Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn X2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).X2Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn Y2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ILineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Y2Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILineStatics<R, F: FnOnce(&ILineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Line, ILineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Line {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Line {}
impl ::core::fmt::Debug for Line {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Line").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Line {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Line;{46a5433d-4ffb-48df-8732-4e15c834816b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Line {
    type Vtable = ILine_Vtbl;
    const IID: ::windows::core::GUID = <ILine as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Line {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Line";
}
impl ::core::convert::From<Line> for ::windows::core::IUnknown {
    fn from(value: Line) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Line> for ::windows::core::IUnknown {
    fn from(value: &Line) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Line {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Line> for ::windows::core::IInspectable {
    fn from(value: Line) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Line> for ::windows::core::IInspectable {
    fn from(value: &Line) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Line {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Line> for Shape {
    fn from(value: &Line) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Line {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl ::core::convert::From<Line> for super::FrameworkElement {
    fn from(value: Line) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Line> for super::FrameworkElement {
    fn from(value: &Line) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Line> for super::UIElement {
    fn from(value: Line) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Line> for super::UIElement {
    fn from(value: &Line) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Line> for super::DependencyObject {
    fn from(value: Line) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Line> for super::DependencyObject {
    fn from(value: &Line) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Line {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Line {}
unsafe impl ::core::marker::Sync for Line {}
#[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
#[repr(transparent)]
pub struct Path(::windows::core::IUnknown);
impl Path {
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Data(&self) -> ::windows::core::Result<super::Media::Geometry> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Geometry>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Geometry>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn new() -> ::windows::core::Result<Path> {
        Self::IPathFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<Path>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<Path> {
        Self::IPathFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<Path>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn DataProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPathStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPathFactory<R, F: FnOnce(&IPathFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Path, IPathFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPathStatics<R, F: FnOnce(&IPathStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Path, IPathStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Path {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Path {}
impl ::core::fmt::Debug for Path {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Path").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Path {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Path;{78883609-3d57-4f3c-b8a5-6cabcac9711f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Path {
    type Vtable = IPath_Vtbl;
    const IID: ::windows::core::GUID = <IPath as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Path {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Path";
}
impl ::core::convert::From<Path> for ::windows::core::IUnknown {
    fn from(value: Path) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Path> for ::windows::core::IUnknown {
    fn from(value: &Path) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Path {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Path> for ::windows::core::IInspectable {
    fn from(value: Path) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Path> for ::windows::core::IInspectable {
    fn from(value: &Path) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Path {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Path> for Shape {
    fn from(value: &Path) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Path {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl ::core::convert::From<Path> for super::FrameworkElement {
    fn from(value: Path) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Path> for super::FrameworkElement {
    fn from(value: &Path) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Path> for super::UIElement {
    fn from(value: Path) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Path> for super::UIElement {
    fn from(value: &Path) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Path> for super::DependencyObject {
    fn from(value: Path) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Path> for super::DependencyObject {
    fn from(value: &Path) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Path {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Path {}
unsafe impl ::core::marker::Sync for Path {}
#[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
#[repr(transparent)]
pub struct Polygon(::windows::core::IUnknown);
impl Polygon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Polygon, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FillRule(&self) -> ::windows::core::Result<super::Media::FillRule> {
        let this = self;
        unsafe {
            let mut result__: super::Media::FillRule = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FillRule)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::FillRule>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFillRule(&self, value: super::Media::FillRule) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFillRule)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"Foundation_Collections\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn Points(&self) -> ::windows::core::Result<super::Media::PointCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Points)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PointCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"Foundation_Collections\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn SetPoints<'a, Param0: ::windows::core::IntoParam<'a, super::Media::PointCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPoints)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn FillRuleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolygonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FillRuleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn PointsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolygonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPolygonStatics<R, F: FnOnce(&IPolygonStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Polygon, IPolygonStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Polygon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Polygon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Polygon {}
impl ::core::fmt::Debug for Polygon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Polygon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Polygon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Polygon;{e3755c19-2e4d-4bcc-8d34-86871957fa01})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Polygon {
    type Vtable = IPolygon_Vtbl;
    const IID: ::windows::core::GUID = <IPolygon as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Polygon {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Polygon";
}
impl ::core::convert::From<Polygon> for ::windows::core::IUnknown {
    fn from(value: Polygon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Polygon> for ::windows::core::IUnknown {
    fn from(value: &Polygon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Polygon> for ::windows::core::IInspectable {
    fn from(value: Polygon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Polygon> for ::windows::core::IInspectable {
    fn from(value: &Polygon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Polygon> for Shape {
    fn from(value: &Polygon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl ::core::convert::From<Polygon> for super::FrameworkElement {
    fn from(value: Polygon) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Polygon> for super::FrameworkElement {
    fn from(value: &Polygon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Polygon> for super::UIElement {
    fn from(value: Polygon) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Polygon> for super::UIElement {
    fn from(value: &Polygon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Polygon> for super::DependencyObject {
    fn from(value: Polygon) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Polygon> for super::DependencyObject {
    fn from(value: &Polygon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Polygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Polygon {}
unsafe impl ::core::marker::Sync for Polygon {}
#[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
#[repr(transparent)]
pub struct Polyline(::windows::core::IUnknown);
impl Polyline {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Polyline, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FillRule(&self) -> ::windows::core::Result<super::Media::FillRule> {
        let this = self;
        unsafe {
            let mut result__: super::Media::FillRule = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FillRule)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::FillRule>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFillRule(&self, value: super::Media::FillRule) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFillRule)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"Foundation_Collections\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn Points(&self) -> ::windows::core::Result<super::Media::PointCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Points)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PointCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"Foundation_Collections\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn SetPoints<'a, Param0: ::windows::core::IntoParam<'a, super::Media::PointCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPoints)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn FillRuleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolylineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FillRuleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn PointsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPolylineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPolylineStatics<R, F: FnOnce(&IPolylineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Polyline, IPolylineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Polyline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Polyline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Polyline {}
impl ::core::fmt::Debug for Polyline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Polyline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Polyline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Polyline;{91dc62f8-42b3-47f3-8476-c55124a7c4c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Polyline {
    type Vtable = IPolyline_Vtbl;
    const IID: ::windows::core::GUID = <IPolyline as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Polyline {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Polyline";
}
impl ::core::convert::From<Polyline> for ::windows::core::IUnknown {
    fn from(value: Polyline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Polyline> for ::windows::core::IUnknown {
    fn from(value: &Polyline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Polyline> for ::windows::core::IInspectable {
    fn from(value: Polyline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Polyline> for ::windows::core::IInspectable {
    fn from(value: &Polyline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Polyline> for Shape {
    fn from(value: &Polyline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl ::core::convert::From<Polyline> for super::FrameworkElement {
    fn from(value: Polyline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Polyline> for super::FrameworkElement {
    fn from(value: &Polyline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Polyline> for super::UIElement {
    fn from(value: Polyline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Polyline> for super::UIElement {
    fn from(value: &Polyline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Polyline> for super::DependencyObject {
    fn from(value: Polyline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Polyline> for super::DependencyObject {
    fn from(value: &Polyline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Polyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Polyline {}
unsafe impl ::core::marker::Sync for Polyline {}
#[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
#[repr(transparent)]
pub struct Rectangle(::windows::core::IUnknown);
impl Rectangle {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Rectangle, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn RadiusX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RadiusX)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn SetRadiusX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRadiusX)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn RadiusY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RadiusY)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn SetRadiusY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRadiusY)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn RadiusXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRectangleStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RadiusXProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn RadiusYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRectangleStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RadiusYProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRectangleStatics<R, F: FnOnce(&IRectangleStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Rectangle, IRectangleStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Rectangle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Rectangle {}
impl ::core::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Rectangle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Rectangle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Rectangle;{855bc230-8a11-4e18-a136-4bc21c7827b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Rectangle {
    type Vtable = IRectangle_Vtbl;
    const IID: ::windows::core::GUID = <IRectangle as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Rectangle {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Rectangle";
}
impl ::core::convert::From<Rectangle> for ::windows::core::IUnknown {
    fn from(value: Rectangle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Rectangle> for ::windows::core::IUnknown {
    fn from(value: &Rectangle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Rectangle> for ::windows::core::IInspectable {
    fn from(value: Rectangle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Rectangle> for ::windows::core::IInspectable {
    fn from(value: &Rectangle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Rectangle> for Shape {
    fn from(value: &Rectangle) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Shape> for &Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, Shape> {
        ::windows::core::Param::Owned(::core::convert::Into::<Shape>::into(self))
    }
}
impl ::core::convert::From<Rectangle> for super::FrameworkElement {
    fn from(value: Rectangle) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Rectangle> for super::FrameworkElement {
    fn from(value: &Rectangle) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Rectangle> for super::UIElement {
    fn from(value: Rectangle) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Rectangle> for super::UIElement {
    fn from(value: &Rectangle) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Rectangle> for super::DependencyObject {
    fn from(value: Rectangle) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Rectangle> for super::DependencyObject {
    fn from(value: &Rectangle) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Rectangle {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Rectangle {}
unsafe impl ::core::marker::Sync for Rectangle {}
#[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
#[repr(transparent)]
pub struct Shape(::windows::core::IUnknown);
impl Shape {
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Fill)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFill)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Stroke(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stroke)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStroke<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStroke)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeMiterLimit(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeMiterLimit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn SetStrokeMiterLimit(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeMiterLimit)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeThickness(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeThickness)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn SetStrokeThickness(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeThickness)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StrokeStartLineCap(&self) -> ::windows::core::Result<super::Media::PenLineCap> {
        let this = self;
        unsafe {
            let mut result__: super::Media::PenLineCap = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeStartLineCap)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PenLineCap>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStrokeStartLineCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeStartLineCap)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StrokeEndLineCap(&self) -> ::windows::core::Result<super::Media::PenLineCap> {
        let this = self;
        unsafe {
            let mut result__: super::Media::PenLineCap = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeEndLineCap)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PenLineCap>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStrokeEndLineCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeEndLineCap)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StrokeLineJoin(&self) -> ::windows::core::Result<super::Media::PenLineJoin> {
        let this = self;
        unsafe {
            let mut result__: super::Media::PenLineJoin = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeLineJoin)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PenLineJoin>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStrokeLineJoin(&self, value: super::Media::PenLineJoin) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeLineJoin)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeDashOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeDashOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn SetStrokeDashOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeDashOffset)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StrokeDashCap(&self) -> ::windows::core::Result<super::Media::PenLineCap> {
        let this = self;
        unsafe {
            let mut result__: super::Media::PenLineCap = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeDashCap)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::PenLineCap>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStrokeDashCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeDashCap)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"Foundation_Collections\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn StrokeDashArray(&self) -> ::windows::core::Result<super::Media::DoubleCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeDashArray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::DoubleCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"Foundation_Collections\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub fn SetStrokeDashArray<'a, Param0: ::windows::core::IntoParam<'a, super::Media::DoubleCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeDashArray)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Stretch(&self) -> ::windows::core::Result<super::Media::Stretch> {
        let this = self;
        unsafe {
            let mut result__: super::Media::Stretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stretch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Stretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStretch(&self, value: super::Media::Stretch) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStretch)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn GeometryTransform(&self) -> ::windows::core::Result<super::Media::Transform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GeometryTransform)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Transform>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetAlphaMask(&self) -> ::windows::core::Result<super::super::Composition::CompositionBrush> {
        let this = &::windows::core::Interface::cast::<IShape2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAlphaMask)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Composition::CompositionBrush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn FillProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FillProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeMiterLimitProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeMiterLimitProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeThicknessProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeThicknessProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeStartLineCapProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeStartLineCapProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeEndLineCapProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeEndLineCapProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeLineJoinProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeLineJoinProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeDashOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeDashOffsetProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeDashCapProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeDashCapProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StrokeDashArrayProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeDashArrayProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Shapes\"`*"]
    pub fn StretchProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StretchProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IShapeStatics<R, F: FnOnce(&IShapeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Shape, IShapeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Shape {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Shape {}
impl ::core::fmt::Debug for Shape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Shape").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Shape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Shapes.Shape;{786f2b75-9aa0-454d-ae06-a2466e37c832})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Shape {
    type Vtable = IShape_Vtbl;
    const IID: ::windows::core::GUID = <IShape as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Shape {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Shape";
}
impl ::core::convert::From<Shape> for ::windows::core::IUnknown {
    fn from(value: Shape) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Shape> for ::windows::core::IUnknown {
    fn from(value: &Shape) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Shape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Shape> for ::windows::core::IInspectable {
    fn from(value: Shape) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Shape> for ::windows::core::IInspectable {
    fn from(value: &Shape) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Shape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Shape> for super::FrameworkElement {
    fn from(value: &Shape) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Shape> for super::UIElement {
    fn from(value: Shape) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Shape> for super::UIElement {
    fn from(value: &Shape) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Shape> for super::DependencyObject {
    fn from(value: Shape) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Shape> for super::DependencyObject {
    fn from(value: &Shape) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Shape {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Shape {}
unsafe impl ::core::marker::Sync for Shape {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
