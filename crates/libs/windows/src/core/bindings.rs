use super::*;
use std::ffi::c_void;

link!("kernel32.dll" "system" fn CreateEventW(attributes: *const c_void, manual_reset: i32, initial_state: i32, name: *const c_void) -> isize);
link!("kernel32.dll" "system" fn EncodePointer(ptr: *const c_void) -> *mut c_void);
link!("kernel32.dll" "system" fn FormatMessageW(flags: u32, source: *const c_void, code: u32, lang: u32, buffer: PWSTR, len: u32, args: *const *const i8) -> u32);
link!("kernel32.dll" "system" fn FreeLibrary(library: isize) -> i32);
link!("kernel32.dll" "system" fn GetLastError() -> u32);
link!("kernel32.dll" "system" fn GetProcAddress(library: isize, name: PCSTR) -> *const c_void);
link!("kernel32.dll" "system" fn GetProcessHeap() -> isize);
link!("kernel32.dll" "system" fn HeapAlloc(heap: isize, flags: u32, len: usize) -> *mut c_void);
link!("kernel32.dll" "system" fn HeapFree(heap: isize, flags: u32, ptr: *const c_void) -> i32);
link!("kernel32.dll" "system" fn LoadLibraryA(name: PCSTR) -> isize);
link!("kernel32.dll" "system" fn SetEvent(handle: isize) -> i32);
link!("kernel32.dll" "system" fn WaitForSingleObject(handle: isize, milliseconds: u32) -> u32);
link!("kernel32.dll""system" fn CloseHandle(handle: isize) -> i32);
link!("ole32.dll" "system" fn CoCreateGuid(guid: *mut GUID) -> HRESULT);
link!("ole32.dll" "system" fn CoTaskMemAlloc(len: usize) -> *mut c_void);
link!("ole32.dll" "system" fn CoTaskMemFree(ptr: *const c_void) -> ());
link!("ole32.dll" "system" fn RoGetAgileReference(options: i32, iid: &GUID, object: *const c_void, reference: *mut *mut c_void) -> HRESULT);
link!("oleaut32.dll" "system" fn GetErrorInfo(reserved: u32, info: *mut *mut c_void) -> HRESULT);
link!("oleaut32.dll" "system" fn SetErrorInfo(reserved: u32, info: *const c_void) -> HRESULT);
link!("oleaut32.dll" "system" fn SysAllocStringLen(input: *const u16, len: u32) -> *const u16);
link!("oleaut32.dll" "system" fn SysFreeString(bstr: *const u16) -> ());
link!("oleaut32.dll" "system" fn SysStringLen(bstr: *const u16) -> u32);

pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 256;
pub const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 4096;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 512;
pub const AGILEREFERENCE_DEFAULT: i32 = 0;

pub const ERROR_NO_UNICODE_TRANSLATION: u32 = 1113u32;
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = HRESULT(-2147221231i32);
pub const CO_E_NOTINITIALIZED: HRESULT = HRESULT(-2147221008i32);
pub const E_NOINTERFACE: HRESULT = HRESULT(-2147467262i32);
pub const E_OUTOFMEMORY: HRESULT = HRESULT(-2147024882i32);
pub const RPC_E_DISCONNECTED: HRESULT = HRESULT(-2147417848i32);
pub const JSCRIPT_E_CANTEXECUTE: HRESULT = HRESULT(-1996357631i32);
pub const S_OK: HRESULT = HRESULT(0i32);

#[doc(hidden)]
#[repr(transparent)]
pub struct IPropertyValueStatics(IUnknown);
unsafe impl Vtable for IPropertyValueStatics {
    type Vtable = IPropertyValueStatics_Vtbl;
}
unsafe impl Interface for IPropertyValueStatics {
    const IID: GUID = GUID::from_u128(0x629bdbc8_d932_4ff4_96b9_8d96c5c1e858);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValueStatics_Vtbl {
    pub base__: IInspectable_Vtbl,
    pub CreateEmpty: unsafe extern "system" fn(this: *mut c_void, result__: *mut *mut c_void) -> HRESULT,
    pub CreateUInt8: unsafe extern "system" fn(this: *mut c_void, value: u8, result__: *mut *mut c_void) -> HRESULT,
    pub CreateInt16: unsafe extern "system" fn(this: *mut c_void, value: i16, result__: *mut *mut c_void) -> HRESULT,
    pub CreateUInt16: unsafe extern "system" fn(this: *mut c_void, value: u16, result__: *mut *mut c_void) -> HRESULT,
    pub CreateInt32: unsafe extern "system" fn(this: *mut c_void, value: i32, result__: *mut *mut c_void) -> HRESULT,
    pub CreateUInt32: unsafe extern "system" fn(this: *mut c_void, value: u32, result__: *mut *mut c_void) -> HRESULT,
    pub CreateInt64: unsafe extern "system" fn(this: *mut c_void, value: i64, result__: *mut *mut c_void) -> HRESULT,
    pub CreateUInt64: unsafe extern "system" fn(this: *mut c_void, value: u64, result__: *mut *mut c_void) -> HRESULT,
    pub CreateSingle: unsafe extern "system" fn(this: *mut c_void, value: f32, result__: *mut *mut c_void) -> HRESULT,
    pub CreateDouble: unsafe extern "system" fn(this: *mut c_void, value: f64, result__: *mut *mut c_void) -> HRESULT,
    pub CreateChar16: unsafe extern "system" fn(this: *mut c_void, value: u16, result__: *mut *mut c_void) -> HRESULT,
    pub CreateBoolean: unsafe extern "system" fn(this: *mut c_void, value: bool, result__: *mut *mut c_void) -> HRESULT,
    pub CreateString: unsafe extern "system" fn(this: *mut c_void, value: *mut c_void, result__: *mut *mut c_void) -> HRESULT,
}

#[repr(transparent)]
pub struct IReference<T>(IUnknown, std::marker::PhantomData<T>)
where
    T: RuntimeType + 'static;
impl<T: RuntimeType + 'static> IReference<T> {
    pub fn Value(&self) -> Result<T> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).Value)(Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl<T: RuntimeType + 'static> std::clone::Clone for IReference<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), std::marker::PhantomData::<T>)
    }
}
impl<T: RuntimeType + 'static> std::cmp::PartialEq for IReference<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: RuntimeType + 'static> std::cmp::Eq for IReference<T> {}
unsafe impl<T: RuntimeType + 'static> RuntimeType for IReference<T> {
    const SIGNATURE: ConstBuffer = { ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{61c17706-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(<T as RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = std::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> Result<Self> {
        from.as_ref().cloned().ok_or(Error::OK)
    }
}
unsafe impl<T: RuntimeType + 'static> Vtable for IReference<T> {
    type Vtable = IReference_Vtbl<T>;
}
unsafe impl<T: RuntimeType + 'static> Interface for IReference<T> {
    const IID: GUID = GUID::from_signature(<Self as RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReference_Vtbl<T>
where
    T: RuntimeType + 'static,
{
    pub base__: IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(this: *mut c_void, result__: *mut <T as Abi>::Abi) -> HRESULT,
    pub T: std::marker::PhantomData<T>,
}

#[repr(transparent)]
pub struct IStringable(IUnknown);
impl IStringable {
    pub fn ToString(&self) -> Result<HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).ToString)(Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl std::clone::Clone for IStringable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IStringable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IStringable {}
unsafe impl RuntimeType for IStringable {
    const SIGNATURE: ConstBuffer = ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
    type DefaultType = std::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> Result<Self> {
        from.as_ref().cloned().ok_or(Error::OK)
    }
}
unsafe impl Vtable for IStringable {
    type Vtable = IStringable_Vtbl;
}
unsafe impl Interface for IStringable {
    const IID: GUID = GUID::from_u128(0x96369f54_8eb6_48f0_abce_c1b211e627c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringable_Vtbl {
    pub base__: IInspectable_Vtbl,
    pub ToString: unsafe extern "system" fn(this: *mut c_void, result__: *mut *mut c_void) -> HRESULT,
}

pub struct PropertyValue;
impl PropertyValue {
    pub fn CreateUInt8(value: u8) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateUInt8)(Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInt16(value: i16) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateInt16)(Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt16(value: u16) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateUInt16)(Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInt32(value: i32) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateInt32)(Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt32(value: u32) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateUInt32)(Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInt64(value: i64) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateInt64)(Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt64(value: u64) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateUInt64)(Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateSingle(value: f32) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateSingle)(Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateDouble(value: f64) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateDouble)(Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateBoolean(value: bool) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateBoolean)(Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateString(value: &HSTRING) -> Result<IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Vtable::vtable(this).CreateString)(Vtable::as_raw(this), std::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IPropertyValueStatics<R, F: FnOnce(&IPropertyValueStatics) -> Result<R>>(callback: F) -> Result<R> {
        static SHARED: FactoryCache<PropertyValue, IPropertyValueStatics> = FactoryCache::new();
        SHARED.call(callback)
    }
}
impl RuntimeName for PropertyValue {
    const NAME: &'static str = "Windows.Foundation.PropertyValue";
}

#[repr(transparent)]
pub struct IAgileObject(IUnknown);
impl std::clone::Clone for IAgileObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IAgileObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IAgileObject {}
unsafe impl Vtable for IAgileObject {
    type Vtable = IAgileObject_Vtbl;
}
unsafe impl Interface for IAgileObject {
    const IID: GUID = GUID::from_u128(0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_Vtbl {
    pub base__: IUnknown_Vtbl,
}

#[repr(transparent)]
pub struct IErrorInfo(IUnknown);
impl IErrorInfo {
    pub unsafe fn GetDescription(&self) -> Result<BSTR> {
        let mut result__ = std::mem::MaybeUninit::zeroed();
        (Vtable::vtable(self).GetDescription)(Vtable::as_raw(self), std::mem::transmute(result__.as_mut_ptr())).from_abi(result__)
    }
}

impl std::clone::Clone for IErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IErrorInfo {}
unsafe impl Vtable for IErrorInfo {
    type Vtable = IErrorInfo_Vtbl;
}
unsafe impl Interface for IErrorInfo {
    const IID: GUID = GUID::from_u128(0x1cf2b120_547d_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorInfo_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(this: *mut c_void, pguid: *mut GUID) -> HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut c_void, pbstrsource: *mut BSTR) -> HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut c_void, pbstrdescription: *mut BSTR) -> HRESULT,
    pub GetHelpFile: unsafe extern "system" fn(this: *mut c_void, pbstrhelpfile: *mut BSTR) -> HRESULT,
    pub GetHelpContext: unsafe extern "system" fn(this: *mut c_void, pdwhelpcontext: *mut u32) -> HRESULT,
}

#[repr(transparent)]
pub struct IAgileReference(pub(crate) IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> Result<T>
    where
        T: Interface,
    {
        let mut result__ = std::mem::MaybeUninit::zeroed();
        (Vtable::vtable(self).Resolve)(Vtable::as_raw(self), &<T as Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}

impl std::clone::Clone for IAgileReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IAgileReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IAgileReference {}
unsafe impl Vtable for IAgileReference {
    type Vtable = IAgileReference_Vtbl;
}
unsafe impl Interface for IAgileReference {
    const IID: GUID = GUID::from_u128(0xc03f6a43_65a4_9818_987e_e0b810d2a6f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut c_void, riid: *const GUID, ppvobjectreference: *mut *mut c_void) -> HRESULT,
}

#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo(IUnknown);
impl std::clone::Clone for ILanguageExceptionErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for ILanguageExceptionErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for ILanguageExceptionErrorInfo {}
unsafe impl Vtable for ILanguageExceptionErrorInfo {
    type Vtable = ILanguageExceptionErrorInfo_Vtbl;
}
unsafe impl Interface for ILanguageExceptionErrorInfo {
    const IID: GUID = GUID::from_u128(0x04a2dbf3_df83_116c_0946_0812abf6e07d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetLanguageException: unsafe extern "system" fn(this: *mut c_void, languageexception: *mut *mut c_void) -> HRESULT,
}

#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo2(IUnknown);
impl ILanguageExceptionErrorInfo2 {
    pub unsafe fn CapturePropagationContext<P0>(&self, languageexception: P0) -> Result<()>
    where
        P0: std::convert::Into<InParam<IUnknown>>,
    {
        (Vtable::vtable(self).CapturePropagationContext)(Vtable::as_raw(self), languageexception.into().abi()).ok()
    }
}
impl std::clone::Clone for ILanguageExceptionErrorInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for ILanguageExceptionErrorInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for ILanguageExceptionErrorInfo2 {}
unsafe impl Vtable for ILanguageExceptionErrorInfo2 {
    type Vtable = ILanguageExceptionErrorInfo2_Vtbl;
}
unsafe impl Interface for ILanguageExceptionErrorInfo2 {
    const IID: GUID = GUID::from_u128(0x5746e5c4_5b97_424c_b620_2822915734dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo2_Vtbl {
    pub base__: ILanguageExceptionErrorInfo_Vtbl,
    pub GetPreviousLanguageExceptionErrorInfo: unsafe extern "system" fn(this: *mut c_void, previouslanguageexceptionerrorinfo: *mut *mut c_void) -> HRESULT,
    pub CapturePropagationContext: unsafe extern "system" fn(this: *mut c_void, languageexception: *mut c_void) -> HRESULT,
    pub GetPropagationContextHead: unsafe extern "system" fn(this: *mut c_void, propagatedlanguageexceptionerrorinfohead: *mut *mut c_void) -> HRESULT,
}

#[repr(transparent)]
pub struct IRestrictedErrorInfo(IUnknown);
impl IRestrictedErrorInfo {
    pub unsafe fn GetErrorDetails(&self, description: &mut BSTR, error: &mut HRESULT, restricteddescription: &mut BSTR, capabilitysid: &mut BSTR) -> Result<()> {
        (Vtable::vtable(self).GetErrorDetails)(Vtable::as_raw(self), description as *mut _, error as *mut _, restricteddescription as *mut _, capabilitysid as *mut _).ok()
    }
    pub unsafe fn GetReference(&self) -> Result<BSTR> {
        let mut result__ = std::mem::MaybeUninit::zeroed();
        (Vtable::vtable(self).GetReference)(Vtable::as_raw(self), std::mem::transmute(result__.as_mut_ptr())).from_abi(result__)
    }
}
impl std::clone::Clone for IRestrictedErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IRestrictedErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IRestrictedErrorInfo {}
unsafe impl std::marker::Send for IRestrictedErrorInfo {}
unsafe impl std::marker::Sync for IRestrictedErrorInfo {}
unsafe impl Vtable for IRestrictedErrorInfo {
    type Vtable = IRestrictedErrorInfo_Vtbl;
}
unsafe impl Interface for IRestrictedErrorInfo {
    const IID: GUID = GUID::from_u128(0x82ba7092_4c88_427d_a7bc_16dd93feb67e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetErrorDetails: unsafe extern "system" fn(this: *mut c_void, description: *mut BSTR, error: *mut HRESULT, restricteddescription: *mut BSTR, capabilitysid: *mut BSTR) -> HRESULT,
    pub GetReference: unsafe extern "system" fn(this: *mut c_void, reference: *mut BSTR) -> HRESULT,
}

#[repr(transparent)]
pub struct IWeakReference(IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> Result<T>
    where
        T: Interface,
    {
        let mut result__ = std::mem::MaybeUninit::zeroed();
        (Vtable::vtable(self).Resolve)(Vtable::as_raw(self), &<T as Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl std::clone::Clone for IWeakReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IWeakReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IWeakReference {}
unsafe impl Vtable for IWeakReference {
    type Vtable = IWeakReference_Vtbl;
}
unsafe impl Interface for IWeakReference {
    const IID: GUID = GUID::from_u128(0x00000037_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut c_void, riid: *const GUID, objectreference: *mut *mut c_void) -> HRESULT,
}

#[repr(transparent)]
pub struct IWeakReferenceSource(IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> Result<IWeakReference> {
        let mut result__ = std::mem::MaybeUninit::zeroed();
        (Vtable::vtable(self).GetWeakReference)(Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl std::clone::Clone for IWeakReferenceSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IWeakReferenceSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IWeakReferenceSource {}
unsafe impl Vtable for IWeakReferenceSource {
    type Vtable = IWeakReferenceSource_Vtbl;
}
unsafe impl Interface for IWeakReferenceSource {
    const IID: GUID = GUID::from_u128(0x00000038_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetWeakReference: unsafe extern "system" fn(this: *mut c_void, weakreference: *mut *mut c_void) -> HRESULT,
}
