#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct II2cControllerProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for II2cControllerProvider {
    type Vtable = II2cControllerProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1640151938,
        17680,
        16739,
        [168, 124, 78, 21, 169, 85, 137, 128],
    );
}
impl II2cControllerProvider {
    pub fn GetDeviceProvider<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ProviderI2cConnectionSettings>,
    >(
        &self,
        settings: Param0,
    ) -> ::windows::runtime::Result<II2cDeviceProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                settings.into_param().abi(),
                &mut result__,
            )
            .from_abi::<II2cDeviceProvider>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for II2cControllerProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{61c2bb82-4510-4163-a87c-4e15a9558980}");
}
impl ::std::convert::From<II2cControllerProvider> for ::windows::runtime::IUnknown {
    fn from(value: II2cControllerProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&II2cControllerProvider> for ::windows::runtime::IUnknown {
    fn from(value: &II2cControllerProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for II2cControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &II2cControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<II2cControllerProvider> for ::windows::runtime::IInspectable {
    fn from(value: II2cControllerProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&II2cControllerProvider> for ::windows::runtime::IInspectable {
    fn from(value: &II2cControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for II2cControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a II2cControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cControllerProvider_abi(
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
        settings: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct II2cDeviceProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for II2cDeviceProvider {
    type Vtable = II2cDeviceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2905876052,
        22504,
        17726,
        [131, 41, 209, 228, 71, 209, 3, 169],
    );
}
impl II2cDeviceProvider {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Write(
        &self,
        buffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                buffer.len() as u32,
                ::std::mem::transmute(buffer.as_ptr()),
            )
            .ok()
        }
    }
    pub fn WritePartial(
        &self,
        buffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<ProviderI2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__: ProviderI2cTransferResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                buffer.len() as u32,
                ::std::mem::transmute(buffer.as_ptr()),
                &mut result__,
            )
            .from_abi::<ProviderI2cTransferResult>(result__)
        }
    }
    pub fn Read(
        &self,
        buffer: &mut [<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                buffer.len() as u32,
                ::std::mem::transmute_copy(&buffer),
            )
            .ok()
        }
    }
    pub fn ReadPartial(
        &self,
        buffer: &mut [<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<ProviderI2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__: ProviderI2cTransferResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                buffer.len() as u32,
                ::std::mem::transmute_copy(&buffer),
                &mut result__,
            )
            .from_abi::<ProviderI2cTransferResult>(result__)
        }
    }
    pub fn WriteRead(
        &self,
        writebuffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
        readbuffer: &mut [<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                writebuffer.len() as u32,
                ::std::mem::transmute(writebuffer.as_ptr()),
                readbuffer.len() as u32,
                ::std::mem::transmute_copy(&readbuffer),
            )
            .ok()
        }
    }
    pub fn WriteReadPartial(
        &self,
        writebuffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
        readbuffer: &mut [<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<ProviderI2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__: ProviderI2cTransferResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                writebuffer.len() as u32,
                ::std::mem::transmute(writebuffer.as_ptr()),
                readbuffer.len() as u32,
                ::std::mem::transmute_copy(&readbuffer),
                &mut result__,
            )
            .from_abi::<ProviderI2cTransferResult>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for II2cDeviceProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{ad342654-57e8-453e-8329-d1e447d103a9}");
}
impl ::std::convert::From<II2cDeviceProvider> for ::windows::runtime::IUnknown {
    fn from(value: II2cDeviceProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&II2cDeviceProvider> for ::windows::runtime::IUnknown {
    fn from(value: &II2cDeviceProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for II2cDeviceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &II2cDeviceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<II2cDeviceProvider> for ::windows::runtime::IInspectable {
    fn from(value: II2cDeviceProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&II2cDeviceProvider> for ::windows::runtime::IInspectable {
    fn from(value: &II2cDeviceProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for II2cDeviceProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a II2cDeviceProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<II2cDeviceProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: II2cDeviceProvider) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&II2cDeviceProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &II2cDeviceProvider) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for II2cDeviceProvider
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &II2cDeviceProvider
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDeviceProvider_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffer_array_size: u32,
        buffer: *const u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffer_array_size: u32,
        buffer: *const u8,
        result__: *mut ProviderI2cTransferResult,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffer_array_size: u32,
        buffer: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffer_array_size: u32,
        buffer: *mut u8,
        result__: *mut ProviderI2cTransferResult,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        writeBuffer_array_size: u32,
        writebuffer: *const u8,
        readBuffer_array_size: u32,
        readbuffer: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        writeBuffer_array_size: u32,
        writebuffer: *const u8,
        readBuffer_array_size: u32,
        readbuffer: *mut u8,
        result__: *mut ProviderI2cTransferResult,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct II2cProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for II2cProvider {
    type Vtable = II2cProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1863518270,
        48994,
        20450,
        [169, 90, 240, 137, 153, 102, 152, 24],
    );
}
impl II2cProvider {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IAsyncOperation<
            super::super::super::Foundation::Collections::IVectorView<II2cControllerProvider>,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IAsyncOperation<
                super::super::super::Foundation::Collections::IVectorView<II2cControllerProvider>,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for II2cProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{6f13083e-bf62-4fe2-a95a-f08999669818}");
}
impl ::std::convert::From<II2cProvider> for ::windows::runtime::IUnknown {
    fn from(value: II2cProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&II2cProvider> for ::windows::runtime::IUnknown {
    fn from(value: &II2cProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for II2cProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &II2cProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<II2cProvider> for ::windows::runtime::IInspectable {
    fn from(value: II2cProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&II2cProvider> for ::windows::runtime::IInspectable {
    fn from(value: &II2cProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for II2cProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a II2cProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cProvider_abi(
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
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProviderI2cConnectionSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProviderI2cConnectionSettings {
    type Vtable = IProviderI2cConnectionSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3923463732,
        58640,
        17591,
        [128, 157, 242, 248, 91, 85, 83, 57],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderI2cConnectionSettings_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ProviderI2cBusSpeed,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ProviderI2cBusSpeed,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ProviderI2cSharingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ProviderI2cSharingMode,
    ) -> ::windows::runtime::HRESULT,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ProviderI2cBusSpeed(pub i32);
impl ProviderI2cBusSpeed {
    pub const StandardMode: ProviderI2cBusSpeed = ProviderI2cBusSpeed(0i32);
    pub const FastMode: ProviderI2cBusSpeed = ProviderI2cBusSpeed(1i32);
}
impl ::std::convert::From<i32> for ProviderI2cBusSpeed {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProviderI2cBusSpeed {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProviderI2cBusSpeed {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Devices.I2c.Provider.ProviderI2cBusSpeed;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ProviderI2cConnectionSettings(::windows::runtime::IInspectable);
impl ProviderI2cConnectionSettings {
    pub fn SlaveAddress(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetSlaveAddress(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn BusSpeed(&self) -> ::windows::runtime::Result<ProviderI2cBusSpeed> {
        let this = self;
        unsafe {
            let mut result__: ProviderI2cBusSpeed = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ProviderI2cBusSpeed>(result__)
        }
    }
    pub fn SetBusSpeed(&self, value: ProviderI2cBusSpeed) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn SharingMode(&self) -> ::windows::runtime::Result<ProviderI2cSharingMode> {
        let this = self;
        unsafe {
            let mut result__: ProviderI2cSharingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ProviderI2cSharingMode>(result__)
        }
    }
    pub fn SetSharingMode(&self, value: ProviderI2cSharingMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProviderI2cConnectionSettings {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings;{e9db4e34-e510-44b7-809d-f2f85b555339})" ) ;
}
unsafe impl ::windows::runtime::Interface for ProviderI2cConnectionSettings {
    type Vtable = IProviderI2cConnectionSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3923463732,
        58640,
        17591,
        [128, 157, 242, 248, 91, 85, 83, 57],
    );
}
impl ::windows::runtime::RuntimeName for ProviderI2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings";
}
impl ::std::convert::From<ProviderI2cConnectionSettings> for ::windows::runtime::IUnknown {
    fn from(value: ProviderI2cConnectionSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProviderI2cConnectionSettings> for ::windows::runtime::IUnknown {
    fn from(value: &ProviderI2cConnectionSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ProviderI2cConnectionSettings
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ProviderI2cConnectionSettings
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ProviderI2cConnectionSettings> for ::windows::runtime::IInspectable {
    fn from(value: ProviderI2cConnectionSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProviderI2cConnectionSettings> for ::windows::runtime::IInspectable {
    fn from(value: &ProviderI2cConnectionSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ProviderI2cConnectionSettings
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ProviderI2cConnectionSettings
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProviderI2cConnectionSettings {}
unsafe impl ::std::marker::Sync for ProviderI2cConnectionSettings {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ProviderI2cSharingMode(pub i32);
impl ProviderI2cSharingMode {
    pub const Exclusive: ProviderI2cSharingMode = ProviderI2cSharingMode(0i32);
    pub const Shared: ProviderI2cSharingMode = ProviderI2cSharingMode(1i32);
}
impl ::std::convert::From<i32> for ProviderI2cSharingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProviderI2cSharingMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProviderI2cSharingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Devices.I2c.Provider.ProviderI2cSharingMode;i4)",
    );
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ProviderI2cTransferResult {
    pub Status: ProviderI2cTransferStatus,
    pub BytesTransferred: u32,
}
impl ProviderI2cTransferResult {}
impl ::std::default::Default for ProviderI2cTransferResult {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ProviderI2cTransferResult {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ProviderI2cTransferResult")
            .field("Status", &self.Status)
            .field("BytesTransferred", &self.BytesTransferred)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ProviderI2cTransferResult {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.BytesTransferred == other.BytesTransferred
    }
}
impl ::std::cmp::Eq for ProviderI2cTransferResult {}
unsafe impl ::windows::runtime::Abi for ProviderI2cTransferResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProviderI2cTransferResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Windows.Devices.I2c.Provider.ProviderI2cTransferResult;enum(Windows.Devices.I2c.Provider.ProviderI2cTransferStatus;i4);u4)" ) ;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ProviderI2cTransferStatus(pub i32);
impl ProviderI2cTransferStatus {
    pub const FullTransfer: ProviderI2cTransferStatus = ProviderI2cTransferStatus(0i32);
    pub const PartialTransfer: ProviderI2cTransferStatus = ProviderI2cTransferStatus(1i32);
    pub const SlaveAddressNotAcknowledged: ProviderI2cTransferStatus =
        ProviderI2cTransferStatus(2i32);
}
impl ::std::convert::From<i32> for ProviderI2cTransferStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProviderI2cTransferStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProviderI2cTransferStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Devices.I2c.Provider.ProviderI2cTransferStatus;i4)",
    );
}
