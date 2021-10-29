#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CompositeTransform3D(::windows::runtime::IInspectable);
impl CompositeTransform3D {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CompositeTransform3D, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterZ(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterZ(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn RotationX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn RotationY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn RotationZ(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationZ(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaleX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaleY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaleZ(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleZ(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn TranslateX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn TranslateY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn TranslateZ(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateZ(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CenterZProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RotationXProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RotationYProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RotationZProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleXProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleYProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleZProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateXProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateYProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateZProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn ICompositeTransform3DStatics<R, F: FnOnce(&ICompositeTransform3DStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CompositeTransform3D, ICompositeTransform3DStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositeTransform3D {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.CompositeTransform3D;{8977cb01-af8d-4af5-b084-c08eb9704abe})");
}
unsafe impl ::windows::runtime::Interface for CompositeTransform3D {
    type Vtable = ICompositeTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2306329345, 44941, 19189, [176, 132, 192, 142, 185, 112, 74, 190]);
}
impl ::windows::runtime::RuntimeName for CompositeTransform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.CompositeTransform3D";
}
impl ::std::convert::From<CompositeTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: CompositeTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CompositeTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: &CompositeTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CompositeTransform3D> for ::windows::runtime::IInspectable {
    fn from(value: CompositeTransform3D) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CompositeTransform3D> for ::windows::runtime::IInspectable {
    fn from(value: &CompositeTransform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<CompositeTransform3D> for Transform3D {
    fn from(value: CompositeTransform3D) -> Self {
        ::std::convert::Into::<Transform3D>::into(&value)
    }
}
impl ::std::convert::From<&CompositeTransform3D> for Transform3D {
    fn from(value: &CompositeTransform3D) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Transform3D> for CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, Transform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Transform3D>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Transform3D> for &CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, Transform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Transform3D>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CompositeTransform3D> for super::super::DependencyObject {
    fn from(value: CompositeTransform3D) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&CompositeTransform3D> for super::super::DependencyObject {
    fn from(value: &CompositeTransform3D) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for CompositeTransform3D {}
unsafe impl ::std::marker::Sync for CompositeTransform3D {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICompositeTransform3D(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositeTransform3D {
    type Vtable = ICompositeTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2306329345, 44941, 19189, [176, 132, 192, 142, 185, 112, 74, 190]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform3D_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICompositeTransform3DStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositeTransform3DStatics {
    type Vtable = ICompositeTransform3DStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3720301927, 10789, 18675, [152, 8, 197, 30, 195, 213, 91, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform3DStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMatrix3DHelper(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMatrix3DHelper {
    type Vtable = IMatrix3DHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3834384623, 39207, 19611, [130, 19, 7, 119, 85, 18, 186, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMatrix3DHelperStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMatrix3DHelperStatics {
    type Vtable = IMatrix3DHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2456048734, 57688, 20084, [136, 153, 104, 145, 96, 189, 47, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Matrix3D) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix1: Matrix3D, matrix2: Matrix3D, result__: *mut Matrix3D) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, offsetx: f64, offsety: f64, offsetz: f64, m44: f64, result__: *mut Matrix3D) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: Matrix3D, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: Matrix3D, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: Matrix3D, result__: *mut Matrix3D) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPerspectiveTransform3D(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerspectiveTransform3D {
    type Vtable = IPerspectiveTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2591773482, 12537, 16545, [150, 201, 197, 157, 135, 249, 90, 195]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerspectiveTransform3D_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPerspectiveTransform3DStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerspectiveTransform3DStatics {
    type Vtable = IPerspectiveTransform3DStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2389664768, 25100, 18631, [132, 77, 63, 9, 132, 218, 91, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerspectiveTransform3DStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITransform3D(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITransform3D {
    type Vtable = ITransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2923353146, 43516, 19505, [134, 205, 86, 217, 202, 37, 26, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform3D_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITransform3DFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITransform3DFactory {
    type Vtable = ITransform3DFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(86777722, 36211, 18637, [187, 184, 208, 4, 52, 202, 174, 93]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform3DFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl Matrix3D {}
impl ::std::default::Default for Matrix3D {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Matrix3D {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Matrix3D")
            .field("M11", &self.M11)
            .field("M12", &self.M12)
            .field("M13", &self.M13)
            .field("M14", &self.M14)
            .field("M21", &self.M21)
            .field("M22", &self.M22)
            .field("M23", &self.M23)
            .field("M24", &self.M24)
            .field("M31", &self.M31)
            .field("M32", &self.M32)
            .field("M33", &self.M33)
            .field("M34", &self.M34)
            .field("OffsetX", &self.OffsetX)
            .field("OffsetY", &self.OffsetY)
            .field("OffsetZ", &self.OffsetZ)
            .field("M44", &self.M44)
            .finish()
    }
}
impl ::std::cmp::PartialEq for Matrix3D {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M13 == other.M13 && self.M14 == other.M14 && self.M21 == other.M21 && self.M22 == other.M22 && self.M23 == other.M23 && self.M24 == other.M24 && self.M31 == other.M31 && self.M32 == other.M32 && self.M33 == other.M33 && self.M34 == other.M34 && self.OffsetX == other.OffsetX && self.OffsetY == other.OffsetY && self.OffsetZ == other.OffsetZ && self.M44 == other.M44
    }
}
impl ::std::cmp::Eq for Matrix3D {}
unsafe impl ::windows::runtime::Abi for Matrix3D {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Matrix3D {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Media.Media3D.Matrix3D;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Matrix3DHelper(::windows::runtime::IInspectable);
impl Matrix3DHelper {
    pub fn Identity() -> ::windows::runtime::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn Multiply<'a, Param0: ::windows::runtime::IntoParam<'a, Matrix3D>, Param1: ::windows::runtime::IntoParam<'a, Matrix3D>>(matrix1: Param0, matrix2: Param1) -> ::windows::runtime::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), matrix1.into_param().abi(), matrix2.into_param().abi(), &mut result__).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn FromElements(m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, offsetx: f64, offsety: f64, offsetz: f64, m44: f64) -> ::windows::runtime::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, offsetx, offsety, offsetz, m44, &mut result__).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn GetHasInverse<'a, Param0: ::windows::runtime::IntoParam<'a, Matrix3D>>(target: Param0) -> ::windows::runtime::Result<bool> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn GetIsIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, Matrix3D>>(target: Param0) -> ::windows::runtime::Result<bool> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn Invert<'a, Param0: ::windows::runtime::IntoParam<'a, Matrix3D>>(target: Param0) -> ::windows::runtime::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn IMatrix3DHelperStatics<R, F: FnOnce(&IMatrix3DHelperStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Matrix3DHelper, IMatrix3DHelperStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Matrix3DHelper {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.Matrix3DHelper;{e48c10ef-9927-4c9b-8213-07775512ba04})");
}
unsafe impl ::windows::runtime::Interface for Matrix3DHelper {
    type Vtable = IMatrix3DHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3834384623, 39207, 19611, [130, 19, 7, 119, 85, 18, 186, 4]);
}
impl ::windows::runtime::RuntimeName for Matrix3DHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.Matrix3DHelper";
}
impl ::std::convert::From<Matrix3DHelper> for ::windows::runtime::IUnknown {
    fn from(value: Matrix3DHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Matrix3DHelper> for ::windows::runtime::IUnknown {
    fn from(value: &Matrix3DHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Matrix3DHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Matrix3DHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Matrix3DHelper> for ::windows::runtime::IInspectable {
    fn from(value: Matrix3DHelper) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Matrix3DHelper> for ::windows::runtime::IInspectable {
    fn from(value: &Matrix3DHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Matrix3DHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Matrix3DHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Matrix3DHelper {}
unsafe impl ::std::marker::Sync for Matrix3DHelper {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerspectiveTransform3D(::windows::runtime::IInspectable);
impl PerspectiveTransform3D {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerspectiveTransform3D, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Depth(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDepth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn OffsetX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffsetX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn OffsetY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffsetY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn DepthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetXProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetYProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn IPerspectiveTransform3DStatics<R, F: FnOnce(&IPerspectiveTransform3DStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerspectiveTransform3D, IPerspectiveTransform3DStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerspectiveTransform3D {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.PerspectiveTransform3D;{9a7b532a-30f9-40a1-96c9-c59d87f95ac3})");
}
unsafe impl ::windows::runtime::Interface for PerspectiveTransform3D {
    type Vtable = IPerspectiveTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2591773482, 12537, 16545, [150, 201, 197, 157, 135, 249, 90, 195]);
}
impl ::windows::runtime::RuntimeName for PerspectiveTransform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.PerspectiveTransform3D";
}
impl ::std::convert::From<PerspectiveTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: PerspectiveTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PerspectiveTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PerspectiveTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerspectiveTransform3D> for ::windows::runtime::IInspectable {
    fn from(value: PerspectiveTransform3D) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PerspectiveTransform3D> for ::windows::runtime::IInspectable {
    fn from(value: &PerspectiveTransform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PerspectiveTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PerspectiveTransform3D> for Transform3D {
    fn from(value: PerspectiveTransform3D) -> Self {
        ::std::convert::Into::<Transform3D>::into(&value)
    }
}
impl ::std::convert::From<&PerspectiveTransform3D> for Transform3D {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Transform3D> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, Transform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Transform3D>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Transform3D> for &PerspectiveTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, Transform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Transform3D>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerspectiveTransform3D> for super::super::DependencyObject {
    fn from(value: PerspectiveTransform3D) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&PerspectiveTransform3D> for super::super::DependencyObject {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &PerspectiveTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for PerspectiveTransform3D {}
unsafe impl ::std::marker::Sync for PerspectiveTransform3D {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Transform3D(::windows::runtime::IInspectable);
impl Transform3D {
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Transform3D {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.Transform3D;{ae3ed43a-a9fc-4c31-86cd-56d9ca251a69})");
}
unsafe impl ::windows::runtime::Interface for Transform3D {
    type Vtable = ITransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2923353146, 43516, 19505, [134, 205, 86, 217, 202, 37, 26, 105]);
}
impl ::windows::runtime::RuntimeName for Transform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.Transform3D";
}
impl ::std::convert::From<Transform3D> for ::windows::runtime::IUnknown {
    fn from(value: Transform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Transform3D> for ::windows::runtime::IUnknown {
    fn from(value: &Transform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Transform3D> for ::windows::runtime::IInspectable {
    fn from(value: Transform3D) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Transform3D> for ::windows::runtime::IInspectable {
    fn from(value: &Transform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Transform3D> for super::super::DependencyObject {
    fn from(value: Transform3D) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Transform3D> for super::super::DependencyObject {
    fn from(value: &Transform3D) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for Transform3D {}
unsafe impl ::std::marker::Sync for Transform3D {}
