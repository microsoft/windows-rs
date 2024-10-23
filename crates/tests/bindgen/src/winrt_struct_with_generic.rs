#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IPropertyValue,
    IPropertyValue_Vtbl,
    0x4bd682dd_7554_40e9_9a9b_82654ede7e62
);
windows_core::imp::interface_hierarchy!(
    IPropertyValue,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IPropertyValue {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl IPropertyValue {
    pub fn IsNumericScalar(&self) {}
    pub fn GetUInt8(&self) {}
    pub fn GetInt16(&self) {}
    pub fn GetUInt16(&self) {}
    pub fn GetInt32(&self) {}
    pub fn GetUInt32(&self) {}
    pub fn GetInt64(&self) {}
    pub fn GetUInt64(&self) {}
    pub fn GetSingle(&self) {}
    pub fn GetDouble(&self) {}
    pub fn GetChar16(&self) {}
    pub fn GetBoolean(&self) {}
    pub fn GetString(&self) {}
    pub fn GetGuid(&self) {}
    pub fn GetUInt8Array(&self) {}
    pub fn GetInt16Array(&self) {}
    pub fn GetUInt16Array(&self) {}
    pub fn GetInt32Array(&self) {}
    pub fn GetUInt32Array(&self) {}
    pub fn GetInt64Array(&self) {}
    pub fn GetUInt64Array(&self) {}
    pub fn GetSingleArray(&self) {}
    pub fn GetDoubleArray(&self) {}
    pub fn GetChar16Array(&self) {}
    pub fn GetBooleanArray(&self) {}
    pub fn GetStringArray(&self) {}
    pub fn GetInspectableArray(&self) {}
    pub fn GetGuidArray(&self) {}
}
impl windows_core::RuntimeType for IPropertyValue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPropertyValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IReference<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IReference<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IReference<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IPropertyValue>
    for IReference<T>
{
    const QUERY: bool = true;
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IReference<T> {
    type Vtable = IReference_Vtbl;
    const IID: windows_core::GUID =
        windows_core::GUID::from_u128(0x61c17706_2d65_11e0_9ae8_d48564015472);
}
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IReference<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> IReference<T> {
    pub fn Value(&self) {}
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IReference<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HttpProgressStage(pub i32);
impl HttpProgressStage {
    pub const None: Self = Self(0i32);
    pub const DetectingProxy: Self = Self(10i32);
    pub const ResolvingName: Self = Self(20i32);
    pub const ConnectingToServer: Self = Self(30i32);
    pub const NegotiatingSsl: Self = Self(40i32);
    pub const SendingHeaders: Self = Self(50i32);
    pub const SendingContent: Self = Self(60i32);
    pub const WaitingForResponse: Self = Self(70i32);
    pub const ReceivingHeaders: Self = Self(80i32);
    pub const ReceivingContent: Self = Self(90i32);
}
impl windows_core::TypeKind for HttpProgressStage {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HttpProgressStage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpProgressStage;i4)");
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HttpProgress {
    pub Stage: HttpProgressStage,
    pub BytesSent: u64,
    pub TotalBytesToSend: Option<IReference<u64>>,
    pub BytesReceived: u64,
    pub TotalBytesToReceive: Option<IReference<u64>>,
    pub Retries: u32,
}
impl windows_core::TypeKind for HttpProgress {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for HttpProgress {
    const SIGNATURE :windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice ( b"struct(Windows.Web.Http.HttpProgress;enum(Windows.Web.Http.HttpProgressStage;i4);u8;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};u8);u8;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};u8);u4)" ) ;
}
