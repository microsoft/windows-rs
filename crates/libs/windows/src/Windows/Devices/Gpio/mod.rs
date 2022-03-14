#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_Gpio_Provider")]
pub mod Provider;
#[repr(C)]
#[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct GpioChangeCount {
    pub Count: u64,
    pub RelativeTime: super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for GpioChangeCount {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for GpioChangeCount {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for GpioChangeCount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GpioChangeCount").field("Count", &self.Count).field("RelativeTime", &self.RelativeTime).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for GpioChangeCount {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for GpioChangeCount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.Gpio.GpioChangeCount;u8;struct(Windows.Foundation.TimeSpan;i8))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for GpioChangeCount {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GpioChangeCount>()) == 0 }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for GpioChangeCount {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for GpioChangeCount {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioChangeCounter(::windows::core::IUnknown);
impl GpioChangeCounter {
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn SetPolarity(&self, value: GpioChangePolarity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPolarity)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Polarity(&self) -> ::windows::core::Result<GpioChangePolarity> {
        let this = self;
        unsafe {
            let mut result__: GpioChangePolarity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Polarity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangePolarity>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn IsStarted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStarted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Read(&self) -> ::windows::core::Result<GpioChangeCount> {
        let this = self;
        unsafe {
            let mut result__: GpioChangeCount = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Read)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangeCount>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Reset(&self) -> ::windows::core::Result<GpioChangeCount> {
        let this = self;
        unsafe {
            let mut result__: GpioChangeCount = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangeCount>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, GpioPin>>(pin: Param0) -> ::windows::core::Result<GpioChangeCounter> {
        Self::IGpioChangeCounterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), pin.into_param().abi(), &mut result__).from_abi::<GpioChangeCounter>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGpioChangeCounterFactory<R, F: FnOnce(&IGpioChangeCounterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GpioChangeCounter, IGpioChangeCounterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GpioChangeCounter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GpioChangeCounter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioChangeCounter {}
impl ::core::fmt::Debug for GpioChangeCounter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioChangeCounter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioChangeCounter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioChangeCounter;{cb5ec0de-6801-43ff-803d-4576628a8b26})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GpioChangeCounter {
    type Vtable = IGpioChangeCounter_Vtbl;
    const IID: ::windows::core::GUID = <IGpioChangeCounter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GpioChangeCounter {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioChangeCounter";
}
impl ::core::convert::From<GpioChangeCounter> for ::windows::core::IUnknown {
    fn from(value: GpioChangeCounter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GpioChangeCounter> for ::windows::core::IUnknown {
    fn from(value: &GpioChangeCounter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GpioChangeCounter> for ::windows::core::IInspectable {
    fn from(value: GpioChangeCounter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GpioChangeCounter> for ::windows::core::IInspectable {
    fn from(value: &GpioChangeCounter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GpioChangeCounter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GpioChangeCounter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioChangeCounter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioChangeCounter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &GpioChangeCounter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GpioChangeCounter {}
unsafe impl ::core::marker::Sync for GpioChangeCounter {}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GpioChangePolarity(pub i32);
impl GpioChangePolarity {
    pub const Falling: Self = Self(0i32);
    pub const Rising: Self = Self(1i32);
    pub const Both: Self = Self(2i32);
}
impl ::core::marker::Copy for GpioChangePolarity {}
impl ::core::clone::Clone for GpioChangePolarity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioChangePolarity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioChangePolarity {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioChangePolarity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioChangePolarity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioChangePolarity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioChangePolarity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioChangeReader(::windows::core::IUnknown);
impl GpioChangeReader {
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Capacity(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Capacity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Length(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn IsEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEmpty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn IsOverflowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOverflowed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn SetPolarity(&self, value: GpioChangePolarity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPolarity)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Polarity(&self) -> ::windows::core::Result<GpioChangePolarity> {
        let this = self;
        unsafe {
            let mut result__: GpioChangePolarity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Polarity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangePolarity>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn IsStarted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStarted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetNextItem(&self) -> ::windows::core::Result<GpioChangeRecord> {
        let this = self;
        unsafe {
            let mut result__: GpioChangeRecord = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNextItem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangeRecord>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PeekNextItem(&self) -> ::windows::core::Result<GpioChangeRecord> {
        let this = self;
        unsafe {
            let mut result__: GpioChangeRecord = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PeekNextItem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioChangeRecord>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<GpioChangeRecord>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAllItems)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<GpioChangeRecord>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WaitForItemsAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WaitForItemsAsync)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, GpioPin>>(pin: Param0) -> ::windows::core::Result<GpioChangeReader> {
        Self::IGpioChangeReaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), pin.into_param().abi(), &mut result__).from_abi::<GpioChangeReader>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn CreateWithCapacity<'a, Param0: ::windows::core::IntoParam<'a, GpioPin>>(pin: Param0, mincapacity: i32) -> ::windows::core::Result<GpioChangeReader> {
        Self::IGpioChangeReaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithCapacity)(::core::mem::transmute_copy(this), pin.into_param().abi(), mincapacity, &mut result__).from_abi::<GpioChangeReader>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGpioChangeReaderFactory<R, F: FnOnce(&IGpioChangeReaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GpioChangeReader, IGpioChangeReaderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GpioChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GpioChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioChangeReader {}
impl ::core::fmt::Debug for GpioChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioChangeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioChangeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioChangeReader;{0abc885f-e031-48e8-8590-70de78363c6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GpioChangeReader {
    type Vtable = IGpioChangeReader_Vtbl;
    const IID: ::windows::core::GUID = <IGpioChangeReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GpioChangeReader {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioChangeReader";
}
impl ::core::convert::From<GpioChangeReader> for ::windows::core::IUnknown {
    fn from(value: GpioChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GpioChangeReader> for ::windows::core::IUnknown {
    fn from(value: &GpioChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GpioChangeReader> for ::windows::core::IInspectable {
    fn from(value: GpioChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GpioChangeReader> for ::windows::core::IInspectable {
    fn from(value: &GpioChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GpioChangeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GpioChangeReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioChangeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioChangeReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &GpioChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GpioChangeReader {}
unsafe impl ::core::marker::Sync for GpioChangeReader {}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct GpioChangeRecord {
    pub RelativeTime: super::super::Foundation::TimeSpan,
    pub Edge: GpioPinEdge,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for GpioChangeRecord {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for GpioChangeRecord {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for GpioChangeRecord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GpioChangeRecord").field("RelativeTime", &self.RelativeTime).field("Edge", &self.Edge).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for GpioChangeRecord {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for GpioChangeRecord {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.Gpio.GpioChangeRecord;struct(Windows.Foundation.TimeSpan;i8);enum(Windows.Devices.Gpio.GpioPinEdge;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for GpioChangeRecord {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GpioChangeRecord>()) == 0 }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for GpioChangeRecord {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for GpioChangeRecord {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioController(::windows::core::IUnknown);
impl GpioController {
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn PinCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn OpenPin(&self, pinnumber: i32) -> ::windows::core::Result<GpioPin> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenPin)(::core::mem::transmute_copy(this), pinnumber, &mut result__).from_abi::<GpioPin>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn OpenPinWithSharingMode(&self, pinnumber: i32, sharingmode: GpioSharingMode) -> ::windows::core::Result<GpioPin> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenPinWithSharingMode)(::core::mem::transmute_copy(this), pinnumber, sharingmode, &mut result__).from_abi::<GpioPin>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn TryOpenPin(&self, pinnumber: i32, sharingmode: GpioSharingMode, pin: &mut ::core::option::Option<GpioPin>, openstatus: &mut GpioOpenStatus) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryOpenPin)(::core::mem::transmute_copy(this), pinnumber, sharingmode, pin as *mut _ as _, openstatus, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<GpioController> {
        Self::IGpioControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioController>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Devices_Gpio_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<'a, Param0: ::windows::core::IntoParam<'a, Provider::IGpioProvider>>(provider: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<GpioController>>> {
        Self::IGpioControllerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetControllersAsync)(::core::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<GpioController>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GpioController>> {
        Self::IGpioControllerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GpioController>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGpioControllerStatics<R, F: FnOnce(&IGpioControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GpioController, IGpioControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IGpioControllerStatics2<R, F: FnOnce(&IGpioControllerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GpioController, IGpioControllerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GpioController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GpioController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioController {}
impl ::core::fmt::Debug for GpioController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioController;{284012e3-7461-469c-a8bc-61d69d08a53c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GpioController {
    type Vtable = IGpioController_Vtbl;
    const IID: ::windows::core::GUID = <IGpioController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GpioController {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioController";
}
impl ::core::convert::From<GpioController> for ::windows::core::IUnknown {
    fn from(value: GpioController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GpioController> for ::windows::core::IUnknown {
    fn from(value: &GpioController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GpioController> for ::windows::core::IInspectable {
    fn from(value: GpioController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GpioController> for ::windows::core::IInspectable {
    fn from(value: &GpioController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GpioController {}
unsafe impl ::core::marker::Sync for GpioController {}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GpioOpenStatus(pub i32);
impl GpioOpenStatus {
    pub const PinOpened: Self = Self(0i32);
    pub const PinUnavailable: Self = Self(1i32);
    pub const SharingViolation: Self = Self(2i32);
    pub const MuxingConflict: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for GpioOpenStatus {}
impl ::core::clone::Clone for GpioOpenStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioOpenStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioOpenStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioOpenStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioOpenStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioOpenStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioOpenStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioPin(::windows::core::IUnknown);
impl GpioPin {
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValueChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GpioPin, GpioPinValueChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValueChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveValueChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveValueChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DebounceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DebounceTimeout)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDebounceTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDebounceTimeout)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn PinNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn SharingMode(&self) -> ::windows::core::Result<GpioSharingMode> {
        let this = self;
        unsafe {
            let mut result__: GpioSharingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SharingMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioSharingMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn IsDriveModeSupported(&self, drivemode: GpioPinDriveMode) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDriveModeSupported)(::core::mem::transmute_copy(this), drivemode, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn GetDriveMode(&self) -> ::windows::core::Result<GpioPinDriveMode> {
        let this = self;
        unsafe {
            let mut result__: GpioPinDriveMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDriveMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioPinDriveMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn SetDriveMode(&self, value: GpioPinDriveMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDriveMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Write(&self, value: GpioPinValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Write)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Read(&self) -> ::windows::core::Result<GpioPinValue> {
        let this = self;
        unsafe {
            let mut result__: GpioPinValue = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Read)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioPinValue>(result__)
        }
    }
}
impl ::core::clone::Clone for GpioPin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GpioPin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioPin {}
impl ::core::fmt::Debug for GpioPin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioPin;{11d9b087-afae-4790-9ee9-e0eac942d201})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GpioPin {
    type Vtable = IGpioPin_Vtbl;
    const IID: ::windows::core::GUID = <IGpioPin as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GpioPin {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioPin";
}
impl ::core::convert::From<GpioPin> for ::windows::core::IUnknown {
    fn from(value: GpioPin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GpioPin> for ::windows::core::IUnknown {
    fn from(value: &GpioPin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GpioPin> for ::windows::core::IInspectable {
    fn from(value: GpioPin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GpioPin> for ::windows::core::IInspectable {
    fn from(value: &GpioPin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GpioPin> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GpioPin) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioPin> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioPin) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &GpioPin {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GpioPin {}
unsafe impl ::core::marker::Sync for GpioPin {}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GpioPinDriveMode(pub i32);
impl GpioPinDriveMode {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const InputPullUp: Self = Self(2i32);
    pub const InputPullDown: Self = Self(3i32);
    pub const OutputOpenDrain: Self = Self(4i32);
    pub const OutputOpenDrainPullUp: Self = Self(5i32);
    pub const OutputOpenSource: Self = Self(6i32);
    pub const OutputOpenSourcePullDown: Self = Self(7i32);
}
impl ::core::marker::Copy for GpioPinDriveMode {}
impl ::core::clone::Clone for GpioPinDriveMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioPinDriveMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioPinDriveMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioPinDriveMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPinDriveMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPinDriveMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinDriveMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GpioPinEdge(pub i32);
impl GpioPinEdge {
    pub const FallingEdge: Self = Self(0i32);
    pub const RisingEdge: Self = Self(1i32);
}
impl ::core::marker::Copy for GpioPinEdge {}
impl ::core::clone::Clone for GpioPinEdge {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioPinEdge {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioPinEdge {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioPinEdge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPinEdge").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPinEdge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinEdge;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GpioPinValue(pub i32);
impl GpioPinValue {
    pub const Low: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for GpioPinValue {}
impl ::core::clone::Clone for GpioPinValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioPinValue {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioPinValue {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioPinValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPinValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPinValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinValue;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioPinValueChangedEventArgs(::windows::core::IUnknown);
impl GpioPinValueChangedEventArgs {
    #[doc = "*Required features: `\"Devices_Gpio\"`*"]
    pub fn Edge(&self) -> ::windows::core::Result<GpioPinEdge> {
        let this = self;
        unsafe {
            let mut result__: GpioPinEdge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Edge)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GpioPinEdge>(result__)
        }
    }
}
impl ::core::clone::Clone for GpioPinValueChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GpioPinValueChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioPinValueChangedEventArgs {}
impl ::core::fmt::Debug for GpioPinValueChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPinValueChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPinValueChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioPinValueChangedEventArgs;{3137aae1-703d-4059-bd24-b5b25dffb84e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GpioPinValueChangedEventArgs {
    type Vtable = IGpioPinValueChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IGpioPinValueChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GpioPinValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioPinValueChangedEventArgs";
}
impl ::core::convert::From<GpioPinValueChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GpioPinValueChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GpioPinValueChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GpioPinValueChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioPinValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioPinValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GpioPinValueChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GpioPinValueChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GpioPinValueChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GpioPinValueChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioPinValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioPinValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GpioPinValueChangedEventArgs {}
unsafe impl ::core::marker::Sync for GpioPinValueChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GpioSharingMode(pub i32);
impl GpioSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for GpioSharingMode {}
impl ::core::clone::Clone for GpioSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioSharingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioSharingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioChangeCounter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGpioChangeCounter {
    type Vtable = IGpioChangeCounter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb5ec0de_6801_43ff_803d_4576628a8b26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeCounter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetPolarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GpioChangePolarity) -> ::windows::core::HRESULT,
    pub Polarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangePolarity) -> ::windows::core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeCount) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Read: usize,
    #[cfg(feature = "Foundation")]
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeCount) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Reset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioChangeCounterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGpioChangeCounterFactory {
    type Vtable = IGpioChangeCounterFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x147d94b6_0a9e_410c_b4fa_f89f4052084d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeCounterFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioChangeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGpioChangeReader {
    type Vtable = IGpioChangeReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0abc885f_e031_48e8_8590_70de78363c6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeReader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Capacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub IsEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsOverflowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPolarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GpioChangePolarity) -> ::windows::core::HRESULT,
    pub Polarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangePolarity) -> ::windows::core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeRecord) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetNextItem: usize,
    #[cfg(feature = "Foundation")]
    pub PeekNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeRecord) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PeekNextItem: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllItems: usize,
    #[cfg(feature = "Foundation")]
    pub WaitForItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitForItemsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioChangeReaderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGpioChangeReaderFactory {
    type Vtable = IGpioChangeReaderFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9598ef3_390e_441a_9d1c_e8de0b2df0df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeReaderFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWithCapacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, mincapacity: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGpioController {
    type Vtable = IGpioController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x284012e3_7461_469c_a8bc_61d69d08a53c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PinCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub OpenPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinnumber: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OpenPinWithSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinnumber: i32, sharingmode: GpioSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryOpenPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinnumber: i32, sharingmode: GpioSharingMode, pin: *mut ::windows::core::RawPtr, openstatus: *mut GpioOpenStatus, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGpioControllerStatics {
    type Vtable = IGpioControllerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ed6f42e_7af7_4116_9533_c43d99a1fb64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioControllerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioControllerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGpioControllerStatics2 {
    type Vtable = IGpioControllerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x912b7d20_6ca4_4106_a373_fffd346b0e5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioControllerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioPin(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGpioPin {
    type Vtable = IGpioPin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d9b087_afae_4790_9ee9_e0eac942d201);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPin_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DebounceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DebounceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetDebounceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDebounceTimeout: usize,
    pub PinNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioSharingMode) -> ::windows::core::HRESULT,
    pub IsDriveModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drivemode: GpioPinDriveMode, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetDriveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioPinDriveMode) -> ::windows::core::HRESULT,
    pub SetDriveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GpioPinDriveMode) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GpioPinValue) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioPinValue) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioPinValueChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGpioPinValueChangedEventArgs {
    type Vtable = IGpioPinValueChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3137aae1_703d_4059_bd24_b5b25dffb84e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinValueChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Edge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioPinEdge) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
