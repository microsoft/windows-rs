#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'UI_Xaml_Media_Media3D'*"]
#[repr(transparent)]
pub struct CompositeTransform3D(::windows::core::IUnknown);
impl CompositeTransform3D {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CompositeTransform3D, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterZ(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RotationX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RotationY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RotationZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationZ(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaleX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaleY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaleZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleZ(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TranslateX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TranslateY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TranslateZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateZ(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CenterZProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RotationXProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RotationYProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RotationZProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleXProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleYProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleZProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateXProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateYProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateZProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ICompositeTransform3DStatics<R, F: FnOnce(&ICompositeTransform3DStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CompositeTransform3D, ICompositeTransform3DStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CompositeTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositeTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositeTransform3D {}
unsafe impl ::windows::core::RuntimeType for CompositeTransform3D {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.CompositeTransform3D;{8977cb01-af8d-4af5-b084-c08eb9704abe})");
}
unsafe impl ::windows::core::Interface for CompositeTransform3D {
    type Vtable = ICompositeTransform3DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8977cb01_af8d_4af5_b084_c08eb9704abe);
}
impl ::windows::core::RuntimeName for CompositeTransform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.CompositeTransform3D";
}
impl ::core::convert::From<CompositeTransform3D> for ::windows::core::IUnknown {
    fn from(value: CompositeTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositeTransform3D> for ::windows::core::IUnknown {
    fn from(value: &CompositeTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositeTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CompositeTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositeTransform3D> for ::windows::core::IInspectable {
    fn from(value: CompositeTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositeTransform3D> for ::windows::core::IInspectable {
    fn from(value: &CompositeTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompositeTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CompositeTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositeTransform3D> for Transform3D {
    fn from(value: CompositeTransform3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositeTransform3D> for Transform3D {
    fn from(value: &CompositeTransform3D) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform3D> for CompositeTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, Transform3D> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform3D> for &CompositeTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, Transform3D> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform3D>::into(self))
    }
}
impl ::core::convert::From<CompositeTransform3D> for super::super::DependencyObject {
    fn from(value: CompositeTransform3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositeTransform3D> for super::super::DependencyObject {
    fn from(value: &CompositeTransform3D) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CompositeTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CompositeTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CompositeTransform3D {}
unsafe impl ::core::marker::Sync for CompositeTransform3D {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositeTransform3D(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompositeTransform3D {
    type Vtable = ICompositeTransform3DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8977cb01_af8d_4af5_b084_c08eb9704abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform3DVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositeTransform3DStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompositeTransform3DStatics {
    type Vtable = ICompositeTransform3DStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddbf4d67_2a25_48f3_9808_c51ec3d55bec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform3DStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrix3DHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMatrix3DHelper {
    type Vtable = IMatrix3DHelperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe48c10ef_9927_4c9b_8213_07775512ba04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DHelperVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrix3DHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMatrix3DHelperStatics {
    type Vtable = IMatrix3DHelperStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9264545e_e158_4e74_8899_689160bd2f8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DHelperStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Matrix3D) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix1: Matrix3D, matrix2: Matrix3D, result__: *mut Matrix3D) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, offsetx: f64, offsety: f64, offsetz: f64, m44: f64, result__: *mut Matrix3D) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: Matrix3D, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: Matrix3D, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: Matrix3D, result__: *mut Matrix3D) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerspectiveTransform3D(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerspectiveTransform3D {
    type Vtable = IPerspectiveTransform3DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a7b532a_30f9_40a1_96c9_c59d87f95ac3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerspectiveTransform3DVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerspectiveTransform3DStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerspectiveTransform3DStatics {
    type Vtable = IPerspectiveTransform3DStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e6f6400_620c_48c7_844d_3f0984da5b17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerspectiveTransform3DStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransform3D(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITransform3D {
    type Vtable = ITransform3DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae3ed43a_a9fc_4c31_86cd_56d9ca251a69);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform3DVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransform3DFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITransform3DFactory {
    type Vtable = ITransform3DFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x052c1f7a_8d73_48cd_bbb8_d00434caae5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform3DFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[doc = "*Required features: 'UI_Xaml_Media_Media3D'*"]
pub struct Matrix3D {
    pub M11: f64,
    pub M12: f64,
    pub M13: f64,
    pub M14: f64,
    pub M21: f64,
    pub M22: f64,
    pub M23: f64,
    pub M24: f64,
    pub M31: f64,
    pub M32: f64,
    pub M33: f64,
    pub M34: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
    pub OffsetZ: f64,
    pub M44: f64,
}
impl ::core::marker::Copy for Matrix3D {}
impl ::core::clone::Clone for Matrix3D {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Matrix3D {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Matrix3D {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Media.Media3D.Matrix3D;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8)");
}
impl ::windows::core::DefaultType for Matrix3D {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for Matrix3D {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Matrix3D>()) == 0 }
    }
}
impl ::core::cmp::Eq for Matrix3D {}
impl ::core::default::Default for Matrix3D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'UI_Xaml_Media_Media3D'*"]
#[repr(transparent)]
pub struct Matrix3DHelper(::windows::core::IUnknown);
impl Matrix3DHelper {
    pub fn Identity() -> ::windows::core::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn Multiply<'a, Param0: ::windows::core::IntoParam<'a, Matrix3D>, Param1: ::windows::core::IntoParam<'a, Matrix3D>>(matrix1: Param0, matrix2: Param1) -> ::windows::core::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), matrix1.into_param().abi(), matrix2.into_param().abi(), &mut result__).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn FromElements(m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, offsetx: f64, offsety: f64, offsetz: f64, m44: f64) -> ::windows::core::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, offsetx, offsety, offsetz, m44, &mut result__).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn GetHasInverse<'a, Param0: ::windows::core::IntoParam<'a, Matrix3D>>(target: Param0) -> ::windows::core::Result<bool> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn GetIsIdentity<'a, Param0: ::windows::core::IntoParam<'a, Matrix3D>>(target: Param0) -> ::windows::core::Result<bool> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn Invert<'a, Param0: ::windows::core::IntoParam<'a, Matrix3D>>(target: Param0) -> ::windows::core::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn IMatrix3DHelperStatics<R, F: FnOnce(&IMatrix3DHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Matrix3DHelper, IMatrix3DHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Matrix3DHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Matrix3DHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Matrix3DHelper {}
unsafe impl ::windows::core::RuntimeType for Matrix3DHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.Matrix3DHelper;{e48c10ef-9927-4c9b-8213-07775512ba04})");
}
unsafe impl ::windows::core::Interface for Matrix3DHelper {
    type Vtable = IMatrix3DHelperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe48c10ef_9927_4c9b_8213_07775512ba04);
}
impl ::windows::core::RuntimeName for Matrix3DHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.Matrix3DHelper";
}
impl ::core::convert::From<Matrix3DHelper> for ::windows::core::IUnknown {
    fn from(value: Matrix3DHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Matrix3DHelper> for ::windows::core::IUnknown {
    fn from(value: &Matrix3DHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Matrix3DHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Matrix3DHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Matrix3DHelper> for ::windows::core::IInspectable {
    fn from(value: Matrix3DHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Matrix3DHelper> for ::windows::core::IInspectable {
    fn from(value: &Matrix3DHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Matrix3DHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Matrix3DHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Matrix3DHelper {}
unsafe impl ::core::marker::Sync for Matrix3DHelper {}
#[doc = "*Required features: 'UI_Xaml_Media_Media3D'*"]
#[repr(transparent)]
pub struct PerspectiveTransform3D(::windows::core::IUnknown);
impl PerspectiveTransform3D {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PerspectiveTransform3D, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Depth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDepth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OffsetX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffsetX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OffsetY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffsetY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DepthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetXProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetYProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPerspectiveTransform3DStatics<R, F: FnOnce(&IPerspectiveTransform3DStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PerspectiveTransform3D, IPerspectiveTransform3DStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PerspectiveTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerspectiveTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerspectiveTransform3D {}
unsafe impl ::windows::core::RuntimeType for PerspectiveTransform3D {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.PerspectiveTransform3D;{9a7b532a-30f9-40a1-96c9-c59d87f95ac3})");
}
unsafe impl ::windows::core::Interface for PerspectiveTransform3D {
    type Vtable = IPerspectiveTransform3DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a7b532a_30f9_40a1_96c9_c59d87f95ac3);
}
impl ::windows::core::RuntimeName for PerspectiveTransform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.PerspectiveTransform3D";
}
impl ::core::convert::From<PerspectiveTransform3D> for ::windows::core::IUnknown {
    fn from(value: PerspectiveTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for ::windows::core::IUnknown {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PerspectiveTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerspectiveTransform3D> for ::windows::core::IInspectable {
    fn from(value: PerspectiveTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for ::windows::core::IInspectable {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PerspectiveTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerspectiveTransform3D> for Transform3D {
    fn from(value: PerspectiveTransform3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for Transform3D {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform3D> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, Transform3D> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transform3D> for &PerspectiveTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, Transform3D> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transform3D>::into(self))
    }
}
impl ::core::convert::From<PerspectiveTransform3D> for super::super::DependencyObject {
    fn from(value: PerspectiveTransform3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for super::super::DependencyObject {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PerspectiveTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PerspectiveTransform3D {}
unsafe impl ::core::marker::Sync for PerspectiveTransform3D {}
#[doc = "*Required features: 'UI_Xaml_Media_Media3D'*"]
#[repr(transparent)]
pub struct Transform3D(::windows::core::IUnknown);
impl Transform3D {}
impl ::core::clone::Clone for Transform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Transform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Transform3D {}
unsafe impl ::windows::core::RuntimeType for Transform3D {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.Transform3D;{ae3ed43a-a9fc-4c31-86cd-56d9ca251a69})");
}
unsafe impl ::windows::core::Interface for Transform3D {
    type Vtable = ITransform3DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae3ed43a_a9fc_4c31_86cd_56d9ca251a69);
}
impl ::windows::core::RuntimeName for Transform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.Transform3D";
}
impl ::core::convert::From<Transform3D> for ::windows::core::IUnknown {
    fn from(value: Transform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Transform3D> for ::windows::core::IUnknown {
    fn from(value: &Transform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Transform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Transform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Transform3D> for ::windows::core::IInspectable {
    fn from(value: Transform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Transform3D> for ::windows::core::IInspectable {
    fn from(value: &Transform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Transform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Transform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Transform3D> for super::super::DependencyObject {
    fn from(value: Transform3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Transform3D> for super::super::DependencyObject {
    fn from(value: &Transform3D) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for Transform3D {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &Transform3D {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Transform3D {}
unsafe impl ::core::marker::Sync for Transform3D {}
