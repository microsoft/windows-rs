#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BitmapCreateOptions(pub u32);
impl BitmapCreateOptions {
    pub const None: BitmapCreateOptions = BitmapCreateOptions(0u32);
    pub const IgnoreImageCache: BitmapCreateOptions = BitmapCreateOptions(8u32);
}
impl ::core::convert::From<u32> for BitmapCreateOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BitmapCreateOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BitmapCreateOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Imaging.BitmapCreateOptions;u4)");
}
impl ::windows::runtime::DefaultType for BitmapCreateOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for BitmapCreateOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for BitmapCreateOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for BitmapCreateOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for BitmapCreateOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for BitmapCreateOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapImage(pub ::windows::runtime::IInspectable);
impl BitmapImage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BitmapImage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateOptions(&self) -> ::windows::runtime::Result<BitmapCreateOptions> {
        let this = self;
        unsafe {
            let mut result__: BitmapCreateOptions = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapCreateOptions>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetCreateOptions(&self, value: BitmapCreateOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn UriSource(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn SetUriSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetDecodePixelWidth(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetDecodePixelHeight(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn DownloadProgress<'a, Param0: ::windows::runtime::IntoParam<'a, DownloadProgressEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RemoveDownloadProgress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn ImageOpened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RemoveImageOpened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn ImageFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ExceptionRoutedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RemoveImageFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelType(&self) -> ::windows::runtime::Result<DecodePixelType> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage2>(self)?;
        unsafe {
            let mut result__: DecodePixelType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DecodePixelType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetDecodePixelType(&self, value: DecodePixelType) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn CreateInstanceWithUriSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Uri>>(urisource: Param0) -> ::windows::runtime::Result<BitmapImage> {
        Self::IBitmapImageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), urisource.into_param().abi(), &mut result__).from_abi::<BitmapImage>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateOptionsProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UriSourceProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelHeightProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelTypeProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn IsAnimatedBitmap(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn IsPlaying(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn AutoPlay(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetAutoPlay(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Play(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn IsAnimatedBitmapProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn IsPlayingProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn AutoPlayProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Storage_Streams`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), streamsource.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Storage_Streams`*"]
    pub fn SetSourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), streamsource.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn IBitmapImageFactory<R, F: FnOnce(&IBitmapImageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BitmapImage, IBitmapImageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapImageStatics<R, F: FnOnce(&IBitmapImageStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BitmapImage, IBitmapImageStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapImageStatics2<R, F: FnOnce(&IBitmapImageStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BitmapImage, IBitmapImageStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapImageStatics3<R, F: FnOnce(&IBitmapImageStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BitmapImage, IBitmapImageStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BitmapImage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.BitmapImage;{31af3271-e3b4-442d-a341-4c0226b2725b})");
}
unsafe impl ::windows::runtime::Interface for BitmapImage {
    type Vtable = IBitmapImage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x31af3271_e3b4_442d_a341_4c0226b2725b);
}
impl ::windows::runtime::RuntimeName for BitmapImage {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.BitmapImage";
}
impl ::core::convert::From<BitmapImage> for ::windows::runtime::IUnknown {
    fn from(value: BitmapImage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapImage> for ::windows::runtime::IUnknown {
    fn from(value: &BitmapImage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapImage> for ::windows::runtime::IInspectable {
    fn from(value: BitmapImage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapImage> for ::windows::runtime::IInspectable {
    fn from(value: &BitmapImage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BitmapImage> for BitmapSource {
    fn from(value: BitmapImage) -> Self {
        ::core::convert::Into::<BitmapSource>::into(&value)
    }
}
impl ::core::convert::From<&BitmapImage> for BitmapSource {
    fn from(value: &BitmapImage) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<BitmapSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<BitmapSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<BitmapImage> for super::ImageSource {
    fn from(value: BitmapImage) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&BitmapImage> for super::ImageSource {
    fn from(value: &BitmapImage) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<BitmapImage> for super::super::DependencyObject {
    fn from(value: BitmapImage) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&BitmapImage> for super::super::DependencyObject {
    fn from(value: &BitmapImage) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for BitmapImage {}
unsafe impl ::core::marker::Sync for BitmapImage {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapSource(pub ::windows::runtime::IInspectable);
impl BitmapSource {
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Storage_Streams`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), streamsource.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Storage_Streams`*"]
    pub fn SetSourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), streamsource.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeightProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn IBitmapSourceStatics<R, F: FnOnce(&IBitmapSourceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BitmapSource, IBitmapSourceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BitmapSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.BitmapSource;{23d86411-202f-41b2-8c5b-a8a3b333800b})");
}
unsafe impl ::windows::runtime::Interface for BitmapSource {
    type Vtable = IBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x23d86411_202f_41b2_8c5b_a8a3b333800b);
}
impl ::windows::runtime::RuntimeName for BitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.BitmapSource";
}
impl ::core::convert::From<BitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: BitmapSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: &BitmapSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapSource> for ::windows::runtime::IInspectable {
    fn from(value: BitmapSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapSource> for ::windows::runtime::IInspectable {
    fn from(value: &BitmapSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BitmapSource> for super::ImageSource {
    fn from(value: BitmapSource) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&BitmapSource> for super::ImageSource {
    fn from(value: &BitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<BitmapSource> for super::super::DependencyObject {
    fn from(value: BitmapSource) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&BitmapSource> for super::super::DependencyObject {
    fn from(value: &BitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for BitmapSource {}
unsafe impl ::core::marker::Sync for BitmapSource {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DecodePixelType(pub i32);
impl DecodePixelType {
    pub const Physical: DecodePixelType = DecodePixelType(0i32);
    pub const Logical: DecodePixelType = DecodePixelType(1i32);
}
impl ::core::convert::From<i32> for DecodePixelType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DecodePixelType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DecodePixelType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Imaging.DecodePixelType;i4)");
}
impl ::windows::runtime::DefaultType for DecodePixelType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DownloadProgressEventArgs(pub ::windows::runtime::IInspectable);
impl DownloadProgressEventArgs {
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Progress(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetProgress(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DownloadProgressEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs;{7311e0d4-fe94-4e70-9b90-cdd47ac23afb})");
}
unsafe impl ::windows::runtime::Interface for DownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7311e0d4_fe94_4e70_9b90_cdd47ac23afb);
}
impl ::windows::runtime::RuntimeName for DownloadProgressEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs";
}
impl ::core::convert::From<DownloadProgressEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DownloadProgressEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DownloadProgressEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DownloadProgressEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DownloadProgressEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DownloadProgressEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DownloadProgressEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DownloadProgressEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DownloadProgressEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DownloadProgressEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DownloadProgressEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DownloadProgressEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DownloadProgressEventArgs {}
unsafe impl ::core::marker::Sync for DownloadProgressEventArgs {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DownloadProgressEventHandler(::windows::runtime::IUnknown);
impl DownloadProgressEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<DownloadProgressEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = DownloadProgressEventHandler_box::<F> {
            vtable: &DownloadProgressEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, DownloadProgressEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DownloadProgressEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({1abaee23-74ee-4cc7-99ba-b171e3cda61e})");
}
unsafe impl ::windows::runtime::Interface for DownloadProgressEventHandler {
    type Vtable = DownloadProgressEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1abaee23_74ee_4cc7_99ba_b171e3cda61e);
}
#[repr(C)]
#[doc(hidden)]
pub struct DownloadProgressEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct DownloadProgressEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<DownloadProgressEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const DownloadProgressEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<DownloadProgressEventArgs>) -> ::windows::runtime::Result<()> + 'static> DownloadProgressEventHandler_box<F> {
    const VTABLE: DownloadProgressEventHandler_abi = DownloadProgressEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<DownloadProgressEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <DownloadProgressEventArgs as ::windows::runtime::Abi>::Abi as *const <DownloadProgressEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapImage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImage {
    type Vtable = IBitmapImage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x31af3271_e3b4_442d_a341_4c0226b2725b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BitmapCreateOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: BitmapCreateOptions) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapImage2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImage2 {
    type Vtable = IBitmapImage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1069c1b6_8c9b_4762_be3d_759f5698f2b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DecodePixelType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DecodePixelType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapImage3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImage3 {
    type Vtable = IBitmapImage3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf1de6f26_3c73_453f_a7ba_9b85c18b3733);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImage3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapImageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImageFactory {
    type Vtable = IBitmapImageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc9132978_4810_4e5e_8087_03671ee60d85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, urisource: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapImageStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImageStatics {
    type Vtable = IBitmapImageStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9e282143_70e8_437c_9fa4_2cbf295cff84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapImageStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImageStatics2 {
    type Vtable = IBitmapImageStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc5f5576a_75af_41a4_b893_8fe91fee2882);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapImageStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImageStatics3 {
    type Vtable = IBitmapImageStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2b44e30d_f6d5_4411_a8cd_bf7603c4faa0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageStatics3_abi(
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
#[doc(hidden)]
pub struct IBitmapSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapSource {
    type Vtable = IBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x23d86411_202f_41b2_8c5b_a8a3b333800b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamsource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamsource: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapSourceFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapSourceFactory {
    type Vtable = IBitmapSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe240420e_d4a7_49a4_a0b4_a59fdd77e508);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapSourceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapSourceStatics {
    type Vtable = IBitmapSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9a9c9981_827b_4e51_891b_8a15b511842d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDownloadProgressEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7311e0d4_fe94_4e70_9b90_cdd47ac23afb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgressEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRenderTargetBitmap(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x500dee81_893c_4c0a_8fec_4678ac717589);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderTargetBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, element: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, element: ::windows::runtime::RawPtr, scaledwidth: i32, scaledheight: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRenderTargetBitmapStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRenderTargetBitmapStatics {
    type Vtable = IRenderTargetBitmapStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf0a1efee_c131_4d40_9c47_f7d7cf2b077f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderTargetBitmapStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISoftwareBitmapSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd2dd9ed0_d3c5_4056_91b5_b7c1d1e8130e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, softwarebitmap: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISurfaceImageSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISurfaceImageSource {
    type Vtable = ISurfaceImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x62f7d416_c714_4c4c_8273_f839bc58135c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISurfaceImageSourceFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISurfaceImageSourceFactory {
    type Vtable = ISurfaceImageSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3ab2212a_ef65_4a5f_bfac_73993e8c12c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pixelwidth: i32, pixelheight: i32, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pixelwidth: i32, pixelheight: i32, isopaque: bool, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISvgImageSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSource {
    type Vtable = ISvgImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x03e1cec3_0ca8_4a4e_8d7c_c808a0838586);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamsource: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISvgImageSourceFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceFactory {
    type Vtable = ISvgImageSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc794e9e7_cf23_4d72_bf1a_dfaa16d8ea52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, urisource: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISvgImageSourceFailedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x68bb3170_3ccc_4035_ac01_9834543d744e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SvgImageSourceLoadStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISvgImageSourceOpenedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x85ef4c16_748e_4008_95c7_6a23dd7316db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceOpenedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISvgImageSourceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceStatics {
    type Vtable = ISvgImageSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9c6638ce_bed1_4aab_acbb_d3e2185d315a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceStatics_abi(
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
#[doc(hidden)]
pub struct IVirtualSurfaceImageSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4a711fea_bfac_11e0_a06a_9de44724019b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVirtualSurfaceImageSourceFactory {
    type Vtable = IVirtualSurfaceImageSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3ab2212a_bfac_11e0_8a92_69e44724019b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pixelwidth: i32, pixelheight: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pixelwidth: i32, pixelheight: i32, isopaque: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWriteableBitmap(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWriteableBitmap {
    type Vtable = IWriteableBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbf0b7e6f_df7c_4a85_8413_a1216285835c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWriteableBitmapFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWriteableBitmapFactory {
    type Vtable = IWriteableBitmapFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5563ebb1_3ef2_42c5_9c6d_1cf5dcc041ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmapFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pixelwidth: i32, pixelheight: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTask(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5d5fe9aa_533e_44b8_a975_fc5f1e3bff52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTask_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlRenderingBackgroundTaskFactory {
    type Vtable = IXamlRenderingBackgroundTaskFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa3d1bb63_38f8_4da3_9fca_fd8128a2cbf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskOverrides(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlRenderingBackgroundTaskOverrides {
    type Vtable = IXamlRenderingBackgroundTaskOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9c2a6997_a908_4711_b4b2_a960db3d8e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Background")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, taskinstance: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))] usize,
);
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RenderTargetBitmap(pub ::windows::runtime::IInspectable);
impl RenderTargetBitmap {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RenderTargetBitmap, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RenderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UIElement>>(&self, element: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RenderToSizeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UIElement>>(&self, element: Param0, scaledwidth: i32, scaledheight: i32) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), element.into_param().abi(), scaledwidth, scaledheight, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Storage_Streams`*"]
    pub fn GetPixelsAsync(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeightProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn IRenderTargetBitmapStatics<R, F: FnOnce(&IRenderTargetBitmapStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RenderTargetBitmap, IRenderTargetBitmapStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RenderTargetBitmap {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap;{500dee81-893c-4c0a-8fec-4678ac717589})");
}
unsafe impl ::windows::runtime::Interface for RenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x500dee81_893c_4c0a_8fec_4678ac717589);
}
impl ::windows::runtime::RuntimeName for RenderTargetBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap";
}
impl ::core::convert::From<RenderTargetBitmap> for ::windows::runtime::IUnknown {
    fn from(value: RenderTargetBitmap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for ::windows::runtime::IUnknown {
    fn from(value: &RenderTargetBitmap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RenderTargetBitmap> for ::windows::runtime::IInspectable {
    fn from(value: RenderTargetBitmap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for ::windows::runtime::IInspectable {
    fn from(value: &RenderTargetBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RenderTargetBitmap> for super::ImageSource {
    fn from(value: RenderTargetBitmap) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for super::ImageSource {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RenderTargetBitmap> for super::super::DependencyObject {
    fn from(value: RenderTargetBitmap) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for super::super::DependencyObject {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RenderTargetBitmap {}
unsafe impl ::core::marker::Sync for RenderTargetBitmap {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SoftwareBitmapSource(pub ::windows::runtime::IInspectable);
impl SoftwareBitmapSource {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SoftwareBitmapSource, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Graphics_Imaging`*"]
    pub fn SetBitmapAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, softwarebitmap: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), softwarebitmap.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SoftwareBitmapSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource;{d2dd9ed0-d3c5-4056-91b5-b7c1d1e8130e})");
}
unsafe impl ::windows::runtime::Interface for SoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd2dd9ed0_d3c5_4056_91b5_b7c1d1e8130e);
}
impl ::windows::runtime::RuntimeName for SoftwareBitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource";
}
impl ::core::convert::From<SoftwareBitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: SoftwareBitmapSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: &SoftwareBitmapSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SoftwareBitmapSource> for ::windows::runtime::IInspectable {
    fn from(value: SoftwareBitmapSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for ::windows::runtime::IInspectable {
    fn from(value: &SoftwareBitmapSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SoftwareBitmapSource> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SoftwareBitmapSource) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SoftwareBitmapSource> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SoftwareBitmapSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::IClosable> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::IClosable> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SoftwareBitmapSource> for super::ImageSource {
    fn from(value: SoftwareBitmapSource) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for super::ImageSource {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SoftwareBitmapSource> for super::super::DependencyObject {
    fn from(value: SoftwareBitmapSource) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for super::super::DependencyObject {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SoftwareBitmapSource {}
unsafe impl ::core::marker::Sync for SoftwareBitmapSource {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SurfaceImageSource(pub ::windows::runtime::IInspectable);
impl SurfaceImageSource {
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateInstanceWithDimensions(pixelwidth: i32, pixelheight: i32) -> ::windows::runtime::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pixelwidth, pixelheight, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<SurfaceImageSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateInstanceWithDimensionsAndOpacity(pixelwidth: i32, pixelheight: i32, isopaque: bool) -> ::windows::runtime::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pixelwidth, pixelheight, isopaque, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<SurfaceImageSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn ISurfaceImageSourceFactory<R, F: FnOnce(&ISurfaceImageSourceFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SurfaceImageSource, ISurfaceImageSourceFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SurfaceImageSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SurfaceImageSource;{62f7d416-c714-4c4c-8273-f839bc58135c})");
}
unsafe impl ::windows::runtime::Interface for SurfaceImageSource {
    type Vtable = ISurfaceImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x62f7d416_c714_4c4c_8273_f839bc58135c);
}
impl ::windows::runtime::RuntimeName for SurfaceImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SurfaceImageSource";
}
impl ::core::convert::From<SurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: SurfaceImageSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: &SurfaceImageSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SurfaceImageSource> for ::windows::runtime::IInspectable {
    fn from(value: SurfaceImageSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SurfaceImageSource> for ::windows::runtime::IInspectable {
    fn from(value: &SurfaceImageSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SurfaceImageSource> for super::ImageSource {
    fn from(value: SurfaceImageSource) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&SurfaceImageSource> for super::ImageSource {
    fn from(value: &SurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SurfaceImageSource> for super::super::DependencyObject {
    fn from(value: SurfaceImageSource) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SurfaceImageSource> for super::super::DependencyObject {
    fn from(value: &SurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SurfaceImageSource {}
unsafe impl ::core::marker::Sync for SurfaceImageSource {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SvgImageSource(pub ::windows::runtime::IInspectable);
impl SvgImageSource {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn UriSource(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn SetUriSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RasterizePixelWidth(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetRasterizePixelWidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RasterizePixelHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetRasterizePixelHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn Opened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceOpenedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RemoveOpened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn OpenFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceFailedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RemoveOpenFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Storage_Streams`*"]
    pub fn SetSourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<SvgImageSourceLoadStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), streamsource.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SvgImageSourceLoadStatus>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UriSourceProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RasterizePixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RasterizePixelHeightProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn new() -> ::windows::runtime::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<SvgImageSource>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn CreateInstanceWithUriSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Uri>>(urisource: Param0) -> ::windows::runtime::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), urisource.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<SvgImageSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn ISvgImageSourceStatics<R, F: FnOnce(&ISvgImageSourceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SvgImageSource, ISvgImageSourceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISvgImageSourceFactory<R, F: FnOnce(&ISvgImageSourceFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SvgImageSource, ISvgImageSourceFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SvgImageSource;{03e1cec3-0ca8-4a4e-8d7c-c808a0838586})");
}
unsafe impl ::windows::runtime::Interface for SvgImageSource {
    type Vtable = ISvgImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x03e1cec3_0ca8_4a4e_8d7c_c808a0838586);
}
impl ::windows::runtime::RuntimeName for SvgImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SvgImageSource";
}
impl ::core::convert::From<SvgImageSource> for ::windows::runtime::IUnknown {
    fn from(value: SvgImageSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SvgImageSource> for ::windows::runtime::IUnknown {
    fn from(value: &SvgImageSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SvgImageSource> for ::windows::runtime::IInspectable {
    fn from(value: SvgImageSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SvgImageSource> for ::windows::runtime::IInspectable {
    fn from(value: &SvgImageSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SvgImageSource> for super::ImageSource {
    fn from(value: SvgImageSource) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&SvgImageSource> for super::ImageSource {
    fn from(value: &SvgImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SvgImageSource> for super::super::DependencyObject {
    fn from(value: SvgImageSource) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SvgImageSource> for super::super::DependencyObject {
    fn from(value: &SvgImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SvgImageSource {}
unsafe impl ::core::marker::Sync for SvgImageSource {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SvgImageSourceFailedEventArgs(pub ::windows::runtime::IInspectable);
impl SvgImageSourceFailedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<SvgImageSourceLoadStatus> {
        let this = self;
        unsafe {
            let mut result__: SvgImageSourceLoadStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SvgImageSourceLoadStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSourceFailedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs;{68bb3170-3ccc-4035-ac01-9834543d744e})");
}
unsafe impl ::windows::runtime::Interface for SvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x68bb3170_3ccc_4035_ac01_9834543d744e);
}
impl ::windows::runtime::RuntimeName for SvgImageSourceFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs";
}
impl ::core::convert::From<SvgImageSourceFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SvgImageSourceFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SvgImageSourceFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SvgImageSourceFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SvgImageSourceFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SvgImageSourceFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SvgImageSourceFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SvgImageSourceFailedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SvgImageSourceFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SvgImageSourceFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SvgImageSourceFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SvgImageSourceFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SvgImageSourceFailedEventArgs {}
unsafe impl ::core::marker::Sync for SvgImageSourceFailedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SvgImageSourceLoadStatus(pub i32);
impl SvgImageSourceLoadStatus {
    pub const Success: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(0i32);
    pub const NetworkError: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(1i32);
    pub const InvalidFormat: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(2i32);
    pub const Other: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(3i32);
}
impl ::core::convert::From<i32> for SvgImageSourceLoadStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SvgImageSourceLoadStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSourceLoadStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Imaging.SvgImageSourceLoadStatus;i4)");
}
impl ::windows::runtime::DefaultType for SvgImageSourceLoadStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SvgImageSourceOpenedEventArgs(pub ::windows::runtime::IInspectable);
impl SvgImageSourceOpenedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSourceOpenedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs;{85ef4c16-748e-4008-95c7-6a23dd7316db})");
}
unsafe impl ::windows::runtime::Interface for SvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x85ef4c16_748e_4008_95c7_6a23dd7316db);
}
impl ::windows::runtime::RuntimeName for SvgImageSourceOpenedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs";
}
impl ::core::convert::From<SvgImageSourceOpenedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SvgImageSourceOpenedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SvgImageSourceOpenedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SvgImageSourceOpenedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SvgImageSourceOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SvgImageSourceOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SvgImageSourceOpenedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SvgImageSourceOpenedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SvgImageSourceOpenedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SvgImageSourceOpenedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SvgImageSourceOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SvgImageSourceOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SvgImageSourceOpenedEventArgs {}
unsafe impl ::core::marker::Sync for SvgImageSourceOpenedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VirtualSurfaceImageSource(pub ::windows::runtime::IInspectable);
impl VirtualSurfaceImageSource {
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateInstanceWithDimensions(pixelwidth: i32, pixelheight: i32) -> ::windows::runtime::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pixelwidth, pixelheight, &mut result__).from_abi::<VirtualSurfaceImageSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateInstanceWithDimensionsAndOpacity(pixelwidth: i32, pixelheight: i32, isopaque: bool) -> ::windows::runtime::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pixelwidth, pixelheight, isopaque, &mut result__).from_abi::<VirtualSurfaceImageSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn IVirtualSurfaceImageSourceFactory<R, F: FnOnce(&IVirtualSurfaceImageSourceFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VirtualSurfaceImageSource, IVirtualSurfaceImageSourceFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VirtualSurfaceImageSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource;{4a711fea-bfac-11e0-a06a-9de44724019b})");
}
unsafe impl ::windows::runtime::Interface for VirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4a711fea_bfac_11e0_a06a_9de44724019b);
}
impl ::windows::runtime::RuntimeName for VirtualSurfaceImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource";
}
impl ::core::convert::From<VirtualSurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for ::windows::runtime::IInspectable {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for ::windows::runtime::IInspectable {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for SurfaceImageSource {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::core::convert::Into::<SurfaceImageSource>::into(&value)
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for SurfaceImageSource {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SurfaceImageSource> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, SurfaceImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SurfaceImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SurfaceImageSource> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, SurfaceImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SurfaceImageSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for super::ImageSource {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for super::ImageSource {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for super::super::DependencyObject {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for super::super::DependencyObject {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for VirtualSurfaceImageSource {}
unsafe impl ::core::marker::Sync for VirtualSurfaceImageSource {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WriteableBitmap(pub ::windows::runtime::IInspectable);
impl WriteableBitmap {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Storage_Streams`*"]
    pub fn PixelBuffer(&self) -> ::windows::runtime::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Invalidate(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateInstanceWithDimensions(pixelwidth: i32, pixelheight: i32) -> ::windows::runtime::Result<WriteableBitmap> {
        Self::IWriteableBitmapFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pixelwidth, pixelheight, &mut result__).from_abi::<WriteableBitmap>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Storage_Streams`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), streamsource.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Storage_Streams`*"]
    pub fn SetSourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), streamsource.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn IWriteableBitmapFactory<R, F: FnOnce(&IWriteableBitmapFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WriteableBitmap, IWriteableBitmapFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WriteableBitmap {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.WriteableBitmap;{bf0b7e6f-df7c-4a85-8413-a1216285835c})");
}
unsafe impl ::windows::runtime::Interface for WriteableBitmap {
    type Vtable = IWriteableBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbf0b7e6f_df7c_4a85_8413_a1216285835c);
}
impl ::windows::runtime::RuntimeName for WriteableBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.WriteableBitmap";
}
impl ::core::convert::From<WriteableBitmap> for ::windows::runtime::IUnknown {
    fn from(value: WriteableBitmap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WriteableBitmap> for ::windows::runtime::IUnknown {
    fn from(value: &WriteableBitmap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WriteableBitmap> for ::windows::runtime::IInspectable {
    fn from(value: WriteableBitmap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WriteableBitmap> for ::windows::runtime::IInspectable {
    fn from(value: &WriteableBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<WriteableBitmap> for BitmapSource {
    fn from(value: WriteableBitmap) -> Self {
        ::core::convert::Into::<BitmapSource>::into(&value)
    }
}
impl ::core::convert::From<&WriteableBitmap> for BitmapSource {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<BitmapSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<BitmapSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<WriteableBitmap> for super::ImageSource {
    fn from(value: WriteableBitmap) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&WriteableBitmap> for super::ImageSource {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ImageSource>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<WriteableBitmap> for super::super::DependencyObject {
    fn from(value: WriteableBitmap) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&WriteableBitmap> for super::super::DependencyObject {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for WriteableBitmap {}
unsafe impl ::core::marker::Sync for WriteableBitmap {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlRenderingBackgroundTask(pub ::windows::runtime::IInspectable);
impl XamlRenderingBackgroundTask {}
unsafe impl ::windows::runtime::RuntimeType for XamlRenderingBackgroundTask {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask;{5d5fe9aa-533e-44b8-a975-fc5f1e3bff52})");
}
unsafe impl ::windows::runtime::Interface for XamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5d5fe9aa_533e_44b8_a975_fc5f1e3bff52);
}
impl ::windows::runtime::RuntimeName for XamlRenderingBackgroundTask {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask";
}
impl ::core::convert::From<XamlRenderingBackgroundTask> for ::windows::runtime::IUnknown {
    fn from(value: XamlRenderingBackgroundTask) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlRenderingBackgroundTask> for ::windows::runtime::IUnknown {
    fn from(value: &XamlRenderingBackgroundTask) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlRenderingBackgroundTask> for ::windows::runtime::IInspectable {
    fn from(value: XamlRenderingBackgroundTask) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlRenderingBackgroundTask> for ::windows::runtime::IInspectable {
    fn from(value: &XamlRenderingBackgroundTask) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlRenderingBackgroundTask {}
unsafe impl ::core::marker::Sync for XamlRenderingBackgroundTask {}
