#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BitmapCreateOptions(pub u32);
impl BitmapCreateOptions {
    pub const None: BitmapCreateOptions = BitmapCreateOptions(0u32);
    pub const IgnoreImageCache: BitmapCreateOptions = BitmapCreateOptions(8u32);
}
impl ::std::convert::From<u32> for BitmapCreateOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BitmapCreateOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BitmapCreateOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Imaging.BitmapCreateOptions;u4)");
}
impl ::std::ops::BitOr for BitmapCreateOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for BitmapCreateOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for BitmapCreateOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for BitmapCreateOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for BitmapCreateOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BitmapImage(::windows::runtime::IInspectable);
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
            let mut result__: BitmapCreateOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BitmapCreateOptions>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetCreateOptions(&self, value: BitmapCreateOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn UriSource(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn SetUriSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetDecodePixelWidth(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetDecodePixelHeight(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn DownloadProgress<'a, Param0: ::windows::runtime::IntoParam<'a, DownloadProgressEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RemoveDownloadProgress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn ImageOpened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RemoveImageOpened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn ImageFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ExceptionRoutedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RemoveImageFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelType(&self) -> ::windows::runtime::Result<DecodePixelType> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage2>(self)?;
        unsafe {
            let mut result__: DecodePixelType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DecodePixelType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetDecodePixelType(&self, value: DecodePixelType) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn CreateInstanceWithUriSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Uri>>(urisource: Param0) -> ::windows::runtime::Result<BitmapImage> {
        Self::IBitmapImageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), urisource.into_param().abi(), &mut result__).from_abi::<BitmapImage>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateOptionsProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UriSourceProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelHeightProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn DecodePixelTypeProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn IsAnimatedBitmap(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn IsPlaying(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn AutoPlay(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetAutoPlay(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Play(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapImage3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn IsAnimatedBitmapProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn IsPlayingProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn AutoPlayProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Storage_Streams`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), streamsource.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Storage_Streams`*"]
    pub fn SetSourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), streamsource.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(833565297, 58292, 17453, [163, 65, 76, 2, 38, 178, 114, 91]);
}
impl ::windows::runtime::RuntimeName for BitmapImage {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.BitmapImage";
}
impl ::std::convert::From<BitmapImage> for ::windows::runtime::IUnknown {
    fn from(value: BitmapImage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BitmapImage> for ::windows::runtime::IUnknown {
    fn from(value: &BitmapImage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BitmapImage> for ::windows::runtime::IInspectable {
    fn from(value: BitmapImage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BitmapImage> for ::windows::runtime::IInspectable {
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
impl ::std::convert::From<BitmapImage> for BitmapSource {
    fn from(value: BitmapImage) -> Self {
        ::std::convert::Into::<BitmapSource>::into(&value)
    }
}
impl ::std::convert::From<&BitmapImage> for BitmapSource {
    fn from(value: &BitmapImage) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<BitmapSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<BitmapSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BitmapImage> for super::ImageSource {
    fn from(value: BitmapImage) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&BitmapImage> for super::ImageSource {
    fn from(value: &BitmapImage) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BitmapImage> for super::super::DependencyObject {
    fn from(value: BitmapImage) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&BitmapImage> for super::super::DependencyObject {
    fn from(value: &BitmapImage) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for BitmapImage {}
unsafe impl ::std::marker::Sync for BitmapImage {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BitmapSource(::windows::runtime::IInspectable);
impl BitmapSource {
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Storage_Streams`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), streamsource.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Storage_Streams`*"]
    pub fn SetSourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), streamsource.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeightProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(601383953, 8239, 16818, [140, 91, 168, 163, 179, 51, 128, 11]);
}
impl ::windows::runtime::RuntimeName for BitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.BitmapSource";
}
impl ::std::convert::From<BitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: BitmapSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: &BitmapSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BitmapSource> for ::windows::runtime::IInspectable {
    fn from(value: BitmapSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BitmapSource> for ::windows::runtime::IInspectable {
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
impl ::std::convert::From<BitmapSource> for super::ImageSource {
    fn from(value: BitmapSource) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&BitmapSource> for super::ImageSource {
    fn from(value: &BitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BitmapSource> for super::super::DependencyObject {
    fn from(value: BitmapSource) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&BitmapSource> for super::super::DependencyObject {
    fn from(value: &BitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for BitmapSource {}
unsafe impl ::std::marker::Sync for BitmapSource {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DecodePixelType(pub i32);
impl DecodePixelType {
    pub const Physical: DecodePixelType = DecodePixelType(0i32);
    pub const Logical: DecodePixelType = DecodePixelType(1i32);
}
impl ::std::convert::From<i32> for DecodePixelType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DecodePixelType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DecodePixelType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Imaging.DecodePixelType;i4)");
}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DownloadProgressEventArgs(::windows::runtime::IInspectable);
impl DownloadProgressEventArgs {
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Progress(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetProgress(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DownloadProgressEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs;{7311e0d4-fe94-4e70-9b90-cdd47ac23afb})");
}
unsafe impl ::windows::runtime::Interface for DownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1930551508, 65172, 20080, [155, 144, 205, 212, 122, 194, 58, 251]);
}
impl ::windows::runtime::RuntimeName for DownloadProgressEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs";
}
impl ::std::convert::From<DownloadProgressEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DownloadProgressEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DownloadProgressEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DownloadProgressEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DownloadProgressEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DownloadProgressEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DownloadProgressEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DownloadProgressEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DownloadProgressEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for DownloadProgressEventArgs {}
unsafe impl ::std::marker::Sync for DownloadProgressEventArgs {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DownloadProgressEventHandler(::windows::runtime::IUnknown);
impl DownloadProgressEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<DownloadProgressEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = DownloadProgressEventHandler_box::<F> {
            vtable: &DownloadProgressEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, DownloadProgressEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DownloadProgressEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({1abaee23-74ee-4cc7-99ba-b171e3cda61e})");
}
unsafe impl ::windows::runtime::Interface for DownloadProgressEventHandler {
    type Vtable = DownloadProgressEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(448458275, 29934, 19655, [153, 186, 177, 113, 227, 205, 166, 30]);
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
struct DownloadProgressEventHandler_box<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<DownloadProgressEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const DownloadProgressEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<DownloadProgressEventArgs>) -> ::windows::runtime::Result<()> + 'static> DownloadProgressEventHandler_box<F> {
    const VTABLE: DownloadProgressEventHandler_abi = DownloadProgressEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<DownloadProgressEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
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
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::DefaultType),
            &*(&e as *const <DownloadProgressEventArgs as ::windows::runtime::Abi>::Abi as *const <DownloadProgressEventArgs as ::windows::runtime::Abi>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBitmapImage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImage {
    type Vtable = IBitmapImage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(833565297, 58292, 17453, [163, 65, 76, 2, 38, 178, 114, 91]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBitmapImage2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImage2 {
    type Vtable = IBitmapImage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(275366326, 35995, 18274, [190, 61, 117, 159, 86, 152, 242, 179]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBitmapImage3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImage3 {
    type Vtable = IBitmapImage3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4057886502, 15475, 17727, [167, 186, 155, 133, 193, 139, 55, 51]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBitmapImageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImageFactory {
    type Vtable = IBitmapImageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3373476216, 18448, 20062, [128, 135, 3, 103, 30, 230, 13, 133]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBitmapImageStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImageStatics {
    type Vtable = IBitmapImageStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2653430083, 28904, 17276, [159, 164, 44, 191, 41, 92, 255, 132]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBitmapImageStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImageStatics2 {
    type Vtable = IBitmapImageStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3321190250, 30127, 16804, [184, 147, 143, 233, 31, 238, 40, 130]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBitmapImageStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImageStatics3 {
    type Vtable = IBitmapImageStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(725934861, 63189, 17425, [168, 205, 191, 118, 3, 196, 250, 160]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBitmapSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapSource {
    type Vtable = IBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(601383953, 8239, 16818, [140, 91, 168, 163, 179, 51, 128, 11]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBitmapSourceFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapSourceFactory {
    type Vtable = IBitmapSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3795862030, 54439, 18852, [160, 180, 165, 159, 221, 119, 229, 8]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBitmapSourceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapSourceStatics {
    type Vtable = IBitmapSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2593954177, 33403, 20049, [137, 27, 138, 21, 181, 17, 132, 45]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IDownloadProgressEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1930551508, 65172, 20080, [155, 144, 205, 212, 122, 194, 58, 251]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRenderTargetBitmap(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1343090305, 35132, 19466, [143, 236, 70, 120, 172, 113, 117, 137]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRenderTargetBitmapStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRenderTargetBitmapStatics {
    type Vtable = IRenderTargetBitmapStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4037144558, 49457, 19776, [156, 71, 247, 215, 207, 43, 7, 127]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISoftwareBitmapSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3537739472, 54213, 16470, [145, 181, 183, 193, 209, 232, 19, 14]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISurfaceImageSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISurfaceImageSource {
    type Vtable = ISurfaceImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1660408854, 50964, 19532, [130, 115, 248, 57, 188, 88, 19, 92]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISurfaceImageSourceFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISurfaceImageSourceFactory {
    type Vtable = ISurfaceImageSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(984752426, 61285, 19039, [191, 172, 115, 153, 62, 140, 18, 201]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISvgImageSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSource {
    type Vtable = ISvgImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(65130179, 3240, 19022, [141, 124, 200, 8, 160, 131, 133, 134]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISvgImageSourceFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceFactory {
    type Vtable = ISvgImageSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3348425191, 53027, 19826, [191, 26, 223, 170, 22, 216, 234, 82]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISvgImageSourceFailedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1757098352, 15564, 16437, [172, 1, 152, 52, 84, 61, 116, 78]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISvgImageSourceOpenedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2247052310, 29838, 16392, [149, 199, 106, 35, 221, 115, 22, 219]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISvgImageSourceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceStatics {
    type Vtable = ISvgImageSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2623944910, 48849, 19115, [172, 187, 211, 226, 24, 93, 49, 90]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1248927722, 49068, 4576, [160, 106, 157, 228, 71, 36, 1, 155]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVirtualSurfaceImageSourceFactory {
    type Vtable = IVirtualSurfaceImageSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(984752426, 49068, 4576, [138, 146, 105, 228, 71, 36, 1, 155]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWriteableBitmap(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWriteableBitmap {
    type Vtable = IWriteableBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3205201519, 57212, 19077, [132, 19, 161, 33, 98, 133, 131, 92]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWriteableBitmapFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWriteableBitmapFactory {
    type Vtable = IWriteableBitmapFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1432611761, 16114, 17093, [156, 109, 28, 245, 220, 192, 65, 255]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTask(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1566566826, 21310, 17592, [169, 117, 252, 95, 30, 59, 255, 82]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlRenderingBackgroundTaskFactory {
    type Vtable = IXamlRenderingBackgroundTaskFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2748431203, 14584, 19875, [159, 202, 253, 129, 40, 162, 203, 249]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskOverrides(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlRenderingBackgroundTaskOverrides {
    type Vtable = IXamlRenderingBackgroundTaskOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2620025239, 43272, 18193, [180, 178, 169, 96, 219, 61, 142, 90]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RenderTargetBitmap(::windows::runtime::IInspectable);
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
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RenderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UIElement>>(&self, element: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RenderToSizeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UIElement>>(&self, element: Param0, scaledwidth: i32, scaledheight: i32) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), element.into_param().abi(), scaledwidth, scaledheight, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Storage_Streams`*"]
    pub fn GetPixelsAsync(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeightProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1343090305, 35132, 19466, [143, 236, 70, 120, 172, 113, 117, 137]);
}
impl ::windows::runtime::RuntimeName for RenderTargetBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap";
}
impl ::std::convert::From<RenderTargetBitmap> for ::windows::runtime::IUnknown {
    fn from(value: RenderTargetBitmap) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RenderTargetBitmap> for ::windows::runtime::IUnknown {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RenderTargetBitmap> for ::windows::runtime::IInspectable {
    fn from(value: RenderTargetBitmap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RenderTargetBitmap> for ::windows::runtime::IInspectable {
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
impl ::std::convert::From<RenderTargetBitmap> for super::ImageSource {
    fn from(value: RenderTargetBitmap) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&RenderTargetBitmap> for super::ImageSource {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RenderTargetBitmap> for super::super::DependencyObject {
    fn from(value: RenderTargetBitmap) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&RenderTargetBitmap> for super::super::DependencyObject {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for RenderTargetBitmap {}
unsafe impl ::std::marker::Sync for RenderTargetBitmap {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SoftwareBitmapSource(::windows::runtime::IInspectable);
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
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), softwarebitmap.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SoftwareBitmapSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource;{d2dd9ed0-d3c5-4056-91b5-b7c1d1e8130e})");
}
unsafe impl ::windows::runtime::Interface for SoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3537739472, 54213, 16470, [145, 181, 183, 193, 209, 232, 19, 14]);
}
impl ::windows::runtime::RuntimeName for SoftwareBitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource";
}
impl ::std::convert::From<SoftwareBitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: SoftwareBitmapSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SoftwareBitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SoftwareBitmapSource> for ::windows::runtime::IInspectable {
    fn from(value: SoftwareBitmapSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SoftwareBitmapSource> for ::windows::runtime::IInspectable {
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
impl ::std::convert::TryFrom<SoftwareBitmapSource> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SoftwareBitmapSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&SoftwareBitmapSource> for super::super::super::super::Foundation::IClosable {
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
        ::std::convert::TryInto::<super::super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SoftwareBitmapSource> for super::ImageSource {
    fn from(value: SoftwareBitmapSource) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&SoftwareBitmapSource> for super::ImageSource {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SoftwareBitmapSource> for super::super::DependencyObject {
    fn from(value: SoftwareBitmapSource) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&SoftwareBitmapSource> for super::super::DependencyObject {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for SoftwareBitmapSource {}
unsafe impl ::std::marker::Sync for SoftwareBitmapSource {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SurfaceImageSource(::windows::runtime::IInspectable);
impl SurfaceImageSource {
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateInstanceWithDimensions(pixelwidth: i32, pixelheight: i32) -> ::windows::runtime::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), pixelwidth, pixelheight, ::std::ptr::null_mut(), &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<SurfaceImageSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateInstanceWithDimensionsAndOpacity(pixelwidth: i32, pixelheight: i32, isopaque: bool) -> ::windows::runtime::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), pixelwidth, pixelheight, isopaque, ::std::ptr::null_mut(), &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<SurfaceImageSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1660408854, 50964, 19532, [130, 115, 248, 57, 188, 88, 19, 92]);
}
impl ::windows::runtime::RuntimeName for SurfaceImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SurfaceImageSource";
}
impl ::std::convert::From<SurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: SurfaceImageSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: &SurfaceImageSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SurfaceImageSource> for ::windows::runtime::IInspectable {
    fn from(value: SurfaceImageSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SurfaceImageSource> for ::windows::runtime::IInspectable {
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
impl ::std::convert::From<SurfaceImageSource> for super::ImageSource {
    fn from(value: SurfaceImageSource) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&SurfaceImageSource> for super::ImageSource {
    fn from(value: &SurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SurfaceImageSource> for super::super::DependencyObject {
    fn from(value: SurfaceImageSource) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&SurfaceImageSource> for super::super::DependencyObject {
    fn from(value: &SurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for SurfaceImageSource {}
unsafe impl ::std::marker::Sync for SurfaceImageSource {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SvgImageSource(::windows::runtime::IInspectable);
impl SvgImageSource {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn UriSource(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn SetUriSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RasterizePixelWidth(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetRasterizePixelWidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RasterizePixelHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetRasterizePixelHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn Opened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceOpenedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RemoveOpened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn OpenFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceFailedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn RemoveOpenFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Storage_Streams`*"]
    pub fn SetSourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<SvgImageSourceLoadStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), streamsource.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SvgImageSourceLoadStatus>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UriSourceProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RasterizePixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RasterizePixelHeightProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn new() -> ::windows::runtime::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::std::ptr::null_mut(), &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<SvgImageSource>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`*"]
    pub fn CreateInstanceWithUriSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Uri>>(urisource: Param0) -> ::windows::runtime::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), urisource.into_param().abi(), ::std::ptr::null_mut(), &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<SvgImageSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(65130179, 3240, 19022, [141, 124, 200, 8, 160, 131, 133, 134]);
}
impl ::windows::runtime::RuntimeName for SvgImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SvgImageSource";
}
impl ::std::convert::From<SvgImageSource> for ::windows::runtime::IUnknown {
    fn from(value: SvgImageSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SvgImageSource> for ::windows::runtime::IUnknown {
    fn from(value: &SvgImageSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SvgImageSource> for ::windows::runtime::IInspectable {
    fn from(value: SvgImageSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SvgImageSource> for ::windows::runtime::IInspectable {
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
impl ::std::convert::From<SvgImageSource> for super::ImageSource {
    fn from(value: SvgImageSource) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&SvgImageSource> for super::ImageSource {
    fn from(value: &SvgImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SvgImageSource> for super::super::DependencyObject {
    fn from(value: SvgImageSource) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&SvgImageSource> for super::super::DependencyObject {
    fn from(value: &SvgImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for SvgImageSource {}
unsafe impl ::std::marker::Sync for SvgImageSource {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SvgImageSourceFailedEventArgs(::windows::runtime::IInspectable);
impl SvgImageSourceFailedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<SvgImageSourceLoadStatus> {
        let this = self;
        unsafe {
            let mut result__: SvgImageSourceLoadStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SvgImageSourceLoadStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSourceFailedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs;{68bb3170-3ccc-4035-ac01-9834543d744e})");
}
unsafe impl ::windows::runtime::Interface for SvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1757098352, 15564, 16437, [172, 1, 152, 52, 84, 61, 116, 78]);
}
impl ::windows::runtime::RuntimeName for SvgImageSourceFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs";
}
impl ::std::convert::From<SvgImageSourceFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SvgImageSourceFailedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SvgImageSourceFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SvgImageSourceFailedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SvgImageSourceFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SvgImageSourceFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SvgImageSourceFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SvgImageSourceFailedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SvgImageSourceFailedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SvgImageSourceFailedEventArgs {}
unsafe impl ::std::marker::Sync for SvgImageSourceFailedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SvgImageSourceLoadStatus(pub i32);
impl SvgImageSourceLoadStatus {
    pub const Success: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(0i32);
    pub const NetworkError: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(1i32);
    pub const InvalidFormat: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(2i32);
    pub const Other: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(3i32);
}
impl ::std::convert::From<i32> for SvgImageSourceLoadStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SvgImageSourceLoadStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSourceLoadStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Imaging.SvgImageSourceLoadStatus;i4)");
}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SvgImageSourceOpenedEventArgs(::windows::runtime::IInspectable);
impl SvgImageSourceOpenedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSourceOpenedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs;{85ef4c16-748e-4008-95c7-6a23dd7316db})");
}
unsafe impl ::windows::runtime::Interface for SvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2247052310, 29838, 16392, [149, 199, 106, 35, 221, 115, 22, 219]);
}
impl ::windows::runtime::RuntimeName for SvgImageSourceOpenedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs";
}
impl ::std::convert::From<SvgImageSourceOpenedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SvgImageSourceOpenedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SvgImageSourceOpenedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SvgImageSourceOpenedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SvgImageSourceOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SvgImageSourceOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SvgImageSourceOpenedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SvgImageSourceOpenedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SvgImageSourceOpenedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SvgImageSourceOpenedEventArgs {}
unsafe impl ::std::marker::Sync for SvgImageSourceOpenedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VirtualSurfaceImageSource(::windows::runtime::IInspectable);
impl VirtualSurfaceImageSource {
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateInstanceWithDimensions(pixelwidth: i32, pixelheight: i32) -> ::windows::runtime::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), pixelwidth, pixelheight, &mut result__).from_abi::<VirtualSurfaceImageSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateInstanceWithDimensionsAndOpacity(pixelwidth: i32, pixelheight: i32, isopaque: bool) -> ::windows::runtime::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), pixelwidth, pixelheight, isopaque, &mut result__).from_abi::<VirtualSurfaceImageSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1248927722, 49068, 4576, [160, 106, 157, 228, 71, 36, 1, 155]);
}
impl ::windows::runtime::RuntimeName for VirtualSurfaceImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource";
}
impl ::std::convert::From<VirtualSurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VirtualSurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VirtualSurfaceImageSource> for ::windows::runtime::IInspectable {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VirtualSurfaceImageSource> for ::windows::runtime::IInspectable {
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
impl ::std::convert::From<VirtualSurfaceImageSource> for SurfaceImageSource {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::std::convert::Into::<SurfaceImageSource>::into(&value)
    }
}
impl ::std::convert::From<&VirtualSurfaceImageSource> for SurfaceImageSource {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SurfaceImageSource> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, SurfaceImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SurfaceImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SurfaceImageSource> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, SurfaceImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SurfaceImageSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VirtualSurfaceImageSource> for super::ImageSource {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&VirtualSurfaceImageSource> for super::ImageSource {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VirtualSurfaceImageSource> for super::super::DependencyObject {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&VirtualSurfaceImageSource> for super::super::DependencyObject {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for VirtualSurfaceImageSource {}
unsafe impl ::std::marker::Sync for VirtualSurfaceImageSource {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WriteableBitmap(::windows::runtime::IInspectable);
impl WriteableBitmap {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Storage_Streams`*"]
    pub fn PixelBuffer(&self) -> ::windows::runtime::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn Invalidate(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn CreateInstanceWithDimensions(pixelwidth: i32, pixelheight: i32) -> ::windows::runtime::Result<WriteableBitmap> {
        Self::IWriteableBitmapFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), pixelwidth, pixelheight, &mut result__).from_abi::<WriteableBitmap>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Storage_Streams`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), streamsource.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `Foundation`, `Storage_Streams`*"]
    pub fn SetSourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), streamsource.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3205201519, 57212, 19077, [132, 19, 161, 33, 98, 133, 131, 92]);
}
impl ::windows::runtime::RuntimeName for WriteableBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.WriteableBitmap";
}
impl ::std::convert::From<WriteableBitmap> for ::windows::runtime::IUnknown {
    fn from(value: WriteableBitmap) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WriteableBitmap> for ::windows::runtime::IUnknown {
    fn from(value: &WriteableBitmap) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WriteableBitmap> for ::windows::runtime::IInspectable {
    fn from(value: WriteableBitmap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WriteableBitmap> for ::windows::runtime::IInspectable {
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
impl ::std::convert::From<WriteableBitmap> for BitmapSource {
    fn from(value: WriteableBitmap) -> Self {
        ::std::convert::Into::<BitmapSource>::into(&value)
    }
}
impl ::std::convert::From<&WriteableBitmap> for BitmapSource {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<BitmapSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<BitmapSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WriteableBitmap> for super::ImageSource {
    fn from(value: WriteableBitmap) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&WriteableBitmap> for super::ImageSource {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WriteableBitmap> for super::super::DependencyObject {
    fn from(value: WriteableBitmap) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&WriteableBitmap> for super::super::DependencyObject {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for WriteableBitmap {}
unsafe impl ::std::marker::Sync for WriteableBitmap {}
#[doc = "*Required features: `UI_Xaml_Media_Imaging`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XamlRenderingBackgroundTask(::windows::runtime::IInspectable);
impl XamlRenderingBackgroundTask {
    #[cfg(feature = "ApplicationModel_Background")]
    #[doc = "*Required features: `UI_Xaml_Media_Imaging`, `ApplicationModel_Background`*"]
    pub fn OnRun<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance>>(&self, taskinstance: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlRenderingBackgroundTaskOverrides>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), taskinstance.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlRenderingBackgroundTask {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask;{5d5fe9aa-533e-44b8-a975-fc5f1e3bff52})");
}
unsafe impl ::windows::runtime::Interface for XamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1566566826, 21310, 17592, [169, 117, 252, 95, 30, 59, 255, 82]);
}
impl ::windows::runtime::RuntimeName for XamlRenderingBackgroundTask {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask";
}
impl ::std::convert::From<XamlRenderingBackgroundTask> for ::windows::runtime::IUnknown {
    fn from(value: XamlRenderingBackgroundTask) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XamlRenderingBackgroundTask> for ::windows::runtime::IUnknown {
    fn from(value: &XamlRenderingBackgroundTask) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XamlRenderingBackgroundTask> for ::windows::runtime::IInspectable {
    fn from(value: XamlRenderingBackgroundTask) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XamlRenderingBackgroundTask> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for XamlRenderingBackgroundTask {}
unsafe impl ::std::marker::Sync for XamlRenderingBackgroundTask {}
