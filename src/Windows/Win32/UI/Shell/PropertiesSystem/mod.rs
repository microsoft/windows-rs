#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ClearPropVariantArray(rgpropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, cvars: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearPropVariantArray(rgpropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, cvars: u32);
        }
        ::core::mem::transmute(ClearPropVariantArray(::core::mem::transmute(rgpropvar), ::core::mem::transmute(cvars)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ClearVariantArray(pvars: *mut super::super::super::System::Com::VARIANT, cvars: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearVariantArray(pvars: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, cvars: u32);
        }
        ::core::mem::transmute(ClearVariantArray(::core::mem::transmute(pvars), ::core::mem::transmute(cvars)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DRAWPROGRESSFLAGS(pub i32);
pub const DPF_NONE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(0i32);
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(1i32);
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(2i32);
pub const DPF_ERROR: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(4i32);
pub const DPF_WARNING: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(8i32);
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(16i32);
impl ::core::convert::From<i32> for DRAWPROGRESSFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DRAWPROGRESSFLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GETPROPERTYSTOREFLAGS(pub i32);
pub const GPS_DEFAULT: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(0i32);
pub const GPS_HANDLERPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(1i32);
pub const GPS_READWRITE: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(2i32);
pub const GPS_TEMPORARY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(4i32);
pub const GPS_FASTPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(8i32);
pub const GPS_OPENSLOWITEM: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(16i32);
pub const GPS_DELAYCREATION: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(32i32);
pub const GPS_BESTEFFORT: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(64i32);
pub const GPS_NO_OPLOCK: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(128i32);
pub const GPS_PREFERQUERYPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(256i32);
pub const GPS_EXTRINSICPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(512i32);
pub const GPS_EXTRINSICPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(1024i32);
pub const GPS_VOLATILEPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(2048i32);
pub const GPS_VOLATILEPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(4096i32);
pub const GPS_MASK_VALID: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(8191i32);
impl ::core::convert::From<i32> for GETPROPERTYSTOREFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GETPROPERTYSTOREFLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICreateObject(pub ::windows::core::IUnknown);
impl ICreateObject {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn CreateObject<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID, punkouter: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for ICreateObject {
    type Vtable = ICreateObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75121952_e0d0_43e5_9380_1d80483acf72);
}
impl ::core::convert::From<ICreateObject> for ::windows::core::IUnknown {
    fn from(value: ICreateObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICreateObject> for ::windows::core::IUnknown {
    fn from(value: &ICreateObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICreateObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICreateObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, punkouter: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDelayedPropertyStoreFactory(pub ::windows::core::IUnknown);
impl IDelayedPropertyStoreFactory {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStore<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), punkfactory.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStoreForKeys<T: ::windows::core::Interface>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rgkeys), ::core::mem::transmute(ckeys), ::core::mem::transmute(flags), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDelayedPropertyStore<T: ::windows::core::Interface>(&self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(dwstoreid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IDelayedPropertyStoreFactory {
    type Vtable = IDelayedPropertyStoreFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40d4577f_e237_4bdb_bd69_58f089431b6a);
}
impl ::core::convert::From<IDelayedPropertyStoreFactory> for ::windows::core::IUnknown {
    fn from(value: IDelayedPropertyStoreFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDelayedPropertyStoreFactory> for ::windows::core::IUnknown {
    fn from(value: &IDelayedPropertyStoreFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDelayedPropertyStoreFactory> for IPropertyStoreFactory {
    fn from(value: IDelayedPropertyStoreFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDelayedPropertyStoreFactory> for IPropertyStoreFactory {
    fn from(value: &IDelayedPropertyStoreFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyStoreFactory> for IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyStoreFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyStoreFactory> for &IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyStoreFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDelayedPropertyStoreFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: GETPROPERTYSTOREFLAGS, punkfactory: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInitializeWithFile(pub ::windows::core::IUnknown);
impl IInitializeWithFile {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pszfilepath: Param0, grfmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszfilepath.into_param().abi(), ::core::mem::transmute(grfmode)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInitializeWithFile {
    type Vtable = IInitializeWithFile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7d14566_0509_4cce_a71f_0a554233bd9b);
}
impl ::core::convert::From<IInitializeWithFile> for ::windows::core::IUnknown {
    fn from(value: IInitializeWithFile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInitializeWithFile> for ::windows::core::IUnknown {
    fn from(value: &IInitializeWithFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInitializeWithFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInitializeWithFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithFile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszfilepath: super::super::super::Foundation::PWSTR, grfmode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInitializeWithStream(pub ::windows::core::IUnknown);
impl IInitializeWithStream {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Com`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, pstream: Param0, grfmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ::core::mem::transmute(grfmode)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInitializeWithStream {
    type Vtable = IInitializeWithStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb824b49d_22ac_4161_ac8a_9916e8fa3f7f);
}
impl ::core::convert::From<IInitializeWithStream> for ::windows::core::IUnknown {
    fn from(value: IInitializeWithStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInitializeWithStream> for ::windows::core::IUnknown {
    fn from(value: &IInitializeWithStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInitializeWithStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInitializeWithStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstream: ::windows::core::RawPtr, grfmode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INamedPropertyStore(pub ::windows::core::IUnknown);
impl INamedPropertyStore {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetNamedValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pszname: Param0) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszname.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetNamedValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pszname: Param0, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetNameCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetNameAt(&self, iprop: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iprop), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for INamedPropertyStore {
    type Vtable = INamedPropertyStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71604b0f_97b0_4764_8577_2f13e98a1422);
}
impl ::core::convert::From<INamedPropertyStore> for ::windows::core::IUnknown {
    fn from(value: INamedPropertyStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INamedPropertyStore> for ::windows::core::IUnknown {
    fn from(value: &INamedPropertyStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INamedPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INamedPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPropertyStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszname: super::super::super::Foundation::PWSTR, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszname: super::super::super::Foundation::PWSTR, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iprop: u32, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectWithPropertyKey(pub ::windows::core::IUnknown);
impl IObjectWithPropertyKey {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(key)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::core::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
}
unsafe impl ::windows::core::Interface for IObjectWithPropertyKey {
    type Vtable = IObjectWithPropertyKey_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc0ca0a7_c316_4fd2_9031_3e628e6d4f23);
}
impl ::core::convert::From<IObjectWithPropertyKey> for ::windows::core::IUnknown {
    fn from(value: IObjectWithPropertyKey) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectWithPropertyKey> for ::windows::core::IUnknown {
    fn from(value: &IObjectWithPropertyKey) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IObjectWithPropertyKey {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IObjectWithPropertyKey {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithPropertyKey_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersistSerializedPropStorage(pub ::windows::core::IUnknown);
impl IPersistSerializedPropStorage {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetPropertyStorage(&self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(psps), ::core::mem::transmute(cb)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppsps), ::core::mem::transmute(pcb)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPersistSerializedPropStorage {
    type Vtable = IPersistSerializedPropStorage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe318ad57_0aa0_450f_aca5_6fab7103d917);
}
impl ::core::convert::From<IPersistSerializedPropStorage> for ::windows::core::IUnknown {
    fn from(value: IPersistSerializedPropStorage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersistSerializedPropStorage> for ::windows::core::IUnknown {
    fn from(value: &IPersistSerializedPropStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistSerializedPropStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistSerializedPropStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersistSerializedPropStorage2(pub ::windows::core::IUnknown);
impl IPersistSerializedPropStorage2 {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetPropertyStorage(&self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(psps), ::core::mem::transmute(cb)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppsps), ::core::mem::transmute(pcb)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStorageSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStorageBuffer(&self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(psps), ::core::mem::transmute(cb), ::core::mem::transmute(pcbwritten)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPersistSerializedPropStorage2 {
    type Vtable = IPersistSerializedPropStorage2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77effa68_4f98_4366_ba72_573b3d880571);
}
impl ::core::convert::From<IPersistSerializedPropStorage2> for ::windows::core::IUnknown {
    fn from(value: IPersistSerializedPropStorage2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersistSerializedPropStorage2> for ::windows::core::IUnknown {
    fn from(value: &IPersistSerializedPropStorage2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPersistSerializedPropStorage2> for IPersistSerializedPropStorage {
    fn from(value: IPersistSerializedPropStorage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistSerializedPropStorage2> for IPersistSerializedPropStorage {
    fn from(value: &IPersistSerializedPropStorage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersistSerializedPropStorage> for IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPersistSerializedPropStorage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersistSerializedPropStorage> for &IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPersistSerializedPropStorage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcb: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyChange(pub ::windows::core::IUnknown);
impl IPropertyChange {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(key)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::core::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn ApplyToPropVariant(&self, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvarin), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPropertyChange {
    type Vtable = IPropertyChange_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf917bc8a_1bba_4478_a245_1bde03eb9431);
}
impl ::core::convert::From<IPropertyChange> for ::windows::core::IUnknown {
    fn from(value: IPropertyChange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyChange> for ::windows::core::IUnknown {
    fn from(value: &IPropertyChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPropertyChange> for IObjectWithPropertyKey {
    fn from(value: IPropertyChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyChange> for IObjectWithPropertyKey {
    fn from(value: &IPropertyChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IObjectWithPropertyKey> for IPropertyChange {
    fn into_param(self) -> ::windows::core::Param<'a, IObjectWithPropertyKey> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IObjectWithPropertyKey> for &IPropertyChange {
    fn into_param(self) -> ::windows::core::Param<'a, IObjectWithPropertyKey> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChange_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppropvarout: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyChangeArray(pub ::windows::core::IUnknown);
impl IPropertyChangeArray {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAt<T: ::windows::core::Interface>(&self, iindex: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IPropertyChange>>(&self, iindex: u32, ppropchange: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ppropchange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IPropertyChange>>(&self, ppropchange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ppropchange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn AppendOrReplace<'a, Param0: ::windows::core::IntoParam<'a, IPropertyChange>>(&self, ppropchange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ppropchange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn RemoveAt(&self, iindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn IsKeyInArray(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(key)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPropertyChangeArray {
    type Vtable = IPropertyChangeArray_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380f5cad_1b5e_42f2_805d_637fd392d31e);
}
impl ::core::convert::From<IPropertyChangeArray> for ::windows::core::IUnknown {
    fn from(value: IPropertyChangeArray) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyChangeArray> for ::windows::core::IUnknown {
    fn from(value: &IPropertyChangeArray) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyChangeArray {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyChangeArray {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangeArray_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcoperations: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iindex: u32, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyDescription(pub ::windows::core::IUnknown);
impl IPropertyDescription {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::core::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyType(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetEditInvitation(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__: <PROPDESC_TYPE_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(mask), &mut result__).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__: <PROPDESC_VIEW_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDisplayType(&self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__: <PROPDESC_DISPLAYTYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetColumnState(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetGroupingRange(&self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__: <PROPDESC_GROUPING_RANGE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__: <PROPDESC_RELATIVEDESCRIPTION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(ppszdesc1), ::core::mem::transmute(ppszdesc2)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetSortDescription(&self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__: <PROPDESC_SORTDESCRIPTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), fdescending.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAggregationType(&self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__: <PROPDESC_AGGREGATION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Search_Common`*"]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontype), ::core::mem::transmute(popdefault)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetEnumTypeList<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescription {
    type Vtable = IPropertyDescription_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f79d558_3e96_4549_a1d1_7d75d2288814);
}
impl ::core::convert::From<IPropertyDescription> for ::windows::core::IUnknown {
    fn from(value: IPropertyDescription) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyDescription> for ::windows::core::IUnknown {
    fn from(value: &IPropertyDescription) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvartype: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszinvite: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcxchars: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcsflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar1: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Search_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyDescription2(pub ::windows::core::IUnknown);
impl IPropertyDescription2 {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::core::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyType(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetEditInvitation(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__: <PROPDESC_TYPE_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(mask), &mut result__).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__: <PROPDESC_VIEW_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDisplayType(&self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__: <PROPDESC_DISPLAYTYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetColumnState(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetGroupingRange(&self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__: <PROPDESC_GROUPING_RANGE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__: <PROPDESC_RELATIVEDESCRIPTION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(ppszdesc1), ::core::mem::transmute(ppszdesc2)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetSortDescription(&self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__: <PROPDESC_SORTDESCRIPTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), fdescending.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAggregationType(&self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__: <PROPDESC_AGGREGATION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Search_Common`*"]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontype), ::core::mem::transmute(popdefault)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetEnumTypeList<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetImageReferenceForValue(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescription2 {
    type Vtable = IPropertyDescription2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57d2eded_5062_400e_b107_5dae79fe57a6);
}
impl ::core::convert::From<IPropertyDescription2> for ::windows::core::IUnknown {
    fn from(value: IPropertyDescription2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyDescription2> for ::windows::core::IUnknown {
    fn from(value: &IPropertyDescription2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyDescription2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyDescription2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPropertyDescription2> for IPropertyDescription {
    fn from(value: IPropertyDescription2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescription2> for IPropertyDescription {
    fn from(value: &IPropertyDescription2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyDescription> for IPropertyDescription2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyDescription> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyDescription> for &IPropertyDescription2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyDescription> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvartype: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszinvite: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcxchars: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcsflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar1: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Search_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyDescriptionAliasInfo(pub ::windows::core::IUnknown);
impl IPropertyDescriptionAliasInfo {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::core::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyType(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetEditInvitation(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__: <PROPDESC_TYPE_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(mask), &mut result__).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__: <PROPDESC_VIEW_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDisplayType(&self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__: <PROPDESC_DISPLAYTYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetColumnState(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetGroupingRange(&self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__: <PROPDESC_GROUPING_RANGE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__: <PROPDESC_RELATIVEDESCRIPTION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(ppszdesc1), ::core::mem::transmute(ppszdesc2)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetSortDescription(&self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__: <PROPDESC_SORTDESCRIPTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), fdescending.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAggregationType(&self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__: <PROPDESC_AGGREGATION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Search_Common`*"]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontype), ::core::mem::transmute(popdefault)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetEnumTypeList<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetSortByAlias<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAdditionalSortByAliases<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescriptionAliasInfo {
    type Vtable = IPropertyDescriptionAliasInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf67104fc_2af9_46fd_b32d_243c1404f3d1);
}
impl ::core::convert::From<IPropertyDescriptionAliasInfo> for ::windows::core::IUnknown {
    fn from(value: IPropertyDescriptionAliasInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyDescriptionAliasInfo> for ::windows::core::IUnknown {
    fn from(value: &IPropertyDescriptionAliasInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPropertyDescriptionAliasInfo> for IPropertyDescription {
    fn from(value: IPropertyDescriptionAliasInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescriptionAliasInfo> for IPropertyDescription {
    fn from(value: &IPropertyDescriptionAliasInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyDescription> for IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyDescription> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyDescription> for &IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyDescription> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionAliasInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvartype: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszinvite: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcxchars: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcsflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar1: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Search_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyDescriptionList(pub ::windows::core::IUnknown);
impl IPropertyDescriptionList {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAt<T: ::windows::core::Interface>(&self, ielem: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ielem), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescriptionList {
    type Vtable = IPropertyDescriptionList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f9fc1d0_c39b_4b26_817f_011967d3440e);
}
impl ::core::convert::From<IPropertyDescriptionList> for ::windows::core::IUnknown {
    fn from(value: IPropertyDescriptionList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyDescriptionList> for ::windows::core::IUnknown {
    fn from(value: &IPropertyDescriptionList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyDescriptionList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyDescriptionList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcelem: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ielem: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyDescriptionRelatedPropertyInfo(pub ::windows::core::IUnknown);
impl IPropertyDescriptionRelatedPropertyInfo {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::core::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyType(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetEditInvitation(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__: <PROPDESC_TYPE_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(mask), &mut result__).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__: <PROPDESC_VIEW_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDisplayType(&self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__: <PROPDESC_DISPLAYTYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetColumnState(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetGroupingRange(&self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__: <PROPDESC_GROUPING_RANGE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__: <PROPDESC_RELATIVEDESCRIPTION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(ppszdesc1), ::core::mem::transmute(ppszdesc2)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetSortDescription(&self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__: <PROPDESC_SORTDESCRIPTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), fdescending.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAggregationType(&self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__: <PROPDESC_AGGREGATION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Search_Common`*"]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontype), ::core::mem::transmute(popdefault)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetEnumTypeList<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetRelatedProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, T: ::windows::core::Interface>(&self, pszrelationshipname: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pszrelationshipname.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescriptionRelatedPropertyInfo {
    type Vtable = IPropertyDescriptionRelatedPropertyInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x507393f4_2a3d_4a60_b59e_d9c75716c2dd);
}
impl ::core::convert::From<IPropertyDescriptionRelatedPropertyInfo> for ::windows::core::IUnknown {
    fn from(value: IPropertyDescriptionRelatedPropertyInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyDescriptionRelatedPropertyInfo> for ::windows::core::IUnknown {
    fn from(value: &IPropertyDescriptionRelatedPropertyInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPropertyDescriptionRelatedPropertyInfo> for IPropertyDescription {
    fn from(value: IPropertyDescriptionRelatedPropertyInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescriptionRelatedPropertyInfo> for IPropertyDescription {
    fn from(value: &IPropertyDescriptionRelatedPropertyInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyDescription> for IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyDescription> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyDescription> for &IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyDescription> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionRelatedPropertyInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvartype: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszinvite: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcxchars: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcsflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar1: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Search_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszrelationshipname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyDescriptionSearchInfo(pub ::windows::core::IUnknown);
impl IPropertyDescriptionSearchInfo {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::core::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyType(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetEditInvitation(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__: <PROPDESC_TYPE_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(mask), &mut result__).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__: <PROPDESC_VIEW_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDisplayType(&self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__: <PROPDESC_DISPLAYTYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetColumnState(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetGroupingRange(&self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__: <PROPDESC_GROUPING_RANGE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__: <PROPDESC_RELATIVEDESCRIPTION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(ppszdesc1), ::core::mem::transmute(ppszdesc2)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetSortDescription(&self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__: <PROPDESC_SORTDESCRIPTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), fdescending.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAggregationType(&self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__: <PROPDESC_AGGREGATION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Search_Common`*"]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontype), ::core::mem::transmute(popdefault)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetEnumTypeList<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetSearchInfoFlags(&self) -> ::windows::core::Result<PROPDESC_SEARCHINFO_FLAGS> {
        let mut result__: <PROPDESC_SEARCHINFO_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SEARCHINFO_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetColumnIndexType(&self) -> ::windows::core::Result<PROPDESC_COLUMNINDEX_TYPE> {
        let mut result__: <PROPDESC_COLUMNINDEX_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_COLUMNINDEX_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetProjectionString(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetMaxSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescriptionSearchInfo {
    type Vtable = IPropertyDescriptionSearchInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x078f91bd_29a2_440f_924e_46a291524520);
}
impl ::core::convert::From<IPropertyDescriptionSearchInfo> for ::windows::core::IUnknown {
    fn from(value: IPropertyDescriptionSearchInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyDescriptionSearchInfo> for ::windows::core::IUnknown {
    fn from(value: &IPropertyDescriptionSearchInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPropertyDescriptionSearchInfo> for IPropertyDescription {
    fn from(value: IPropertyDescriptionSearchInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescriptionSearchInfo> for IPropertyDescription {
    fn from(value: &IPropertyDescriptionSearchInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyDescription> for IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyDescription> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyDescription> for &IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyDescription> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionSearchInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvartype: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszinvite: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcxchars: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcsflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar1: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Search_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszprojection: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbmaxsize: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyEnumType(pub ::windows::core::IUnknown);
impl IPropertyEnumType {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetEnumType(&self) -> ::windows::core::Result<PROPENUMTYPE> {
        let mut result__: <PROPENUMTYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPENUMTYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetRangeMinValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetRangeSetValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayText(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPropertyEnumType {
    type Vtable = IPropertyEnumType_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11e1fbf9_2d56_4a6b_8db3_7cd193a471f2);
}
impl ::core::convert::From<IPropertyEnumType> for ::windows::core::IUnknown {
    fn from(value: IPropertyEnumType) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyEnumType> for ::windows::core::IUnknown {
    fn from(value: &IPropertyEnumType) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyEnumType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyEnumType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penumtype: *mut PROPENUMTYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvarmin: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvarset: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyEnumType2(pub ::windows::core::IUnknown);
impl IPropertyEnumType2 {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetEnumType(&self) -> ::windows::core::Result<PROPENUMTYPE> {
        let mut result__: <PROPENUMTYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PROPENUMTYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetRangeMinValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetRangeSetValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayText(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetImageReference(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPropertyEnumType2 {
    type Vtable = IPropertyEnumType2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b6e051c_5ddd_4321_9070_fe2acb55e794);
}
impl ::core::convert::From<IPropertyEnumType2> for ::windows::core::IUnknown {
    fn from(value: IPropertyEnumType2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyEnumType2> for ::windows::core::IUnknown {
    fn from(value: &IPropertyEnumType2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyEnumType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyEnumType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPropertyEnumType2> for IPropertyEnumType {
    fn from(value: IPropertyEnumType2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyEnumType2> for IPropertyEnumType {
    fn from(value: &IPropertyEnumType2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyEnumType> for IPropertyEnumType2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyEnumType> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyEnumType> for &IPropertyEnumType2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyEnumType> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penumtype: *mut PROPENUMTYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvarmin: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppropvarset: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyEnumTypeList(pub ::windows::core::IUnknown);
impl IPropertyEnumTypeList {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAt<T: ::windows::core::Interface>(&self, itype: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itype), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetConditionAt<T: ::windows::core::Interface>(&self, nindex: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FindMatchingIndex(&self, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(propvarcmp), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPropertyEnumTypeList {
    type Vtable = IPropertyEnumTypeList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa99400f4_3d84_4557_94ba_1242fb2cc9a6);
}
impl ::core::convert::From<IPropertyEnumTypeList> for ::windows::core::IUnknown {
    fn from(value: IPropertyEnumTypeList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyEnumTypeList> for ::windows::core::IUnknown {
    fn from(value: &IPropertyEnumTypeList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyEnumTypeList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyEnumTypeList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumTypeList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctypes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itype: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propvarcmp: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pnindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyStore(pub ::windows::core::IUnknown);
impl IPropertyStore {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows::core::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(iprop), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPropertyStore {
    type Vtable = IPropertyStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x886d8eeb_8cf2_4446_8d02_cdba1dbdcf99);
}
impl ::core::convert::From<IPropertyStore> for ::windows::core::IUnknown {
    fn from(value: IPropertyStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyStore> for ::windows::core::IUnknown {
    fn from(value: &IPropertyStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cprops: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY, pv: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyStoreCache(pub ::windows::core::IUnknown);
impl IPropertyStoreCache {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows::core::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(iprop), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetState(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<PSC_STATE> {
        let mut result__: <PSC_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<PSC_STATE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pstate)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetState(&self, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(state)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(ppropvar), ::core::mem::transmute(state)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPropertyStoreCache {
    type Vtable = IPropertyStoreCache_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3017056d_9a91_4e90_937d_746c72abbf4f);
}
impl ::core::convert::From<IPropertyStoreCache> for ::windows::core::IUnknown {
    fn from(value: IPropertyStoreCache) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyStoreCache> for ::windows::core::IUnknown {
    fn from(value: &IPropertyStoreCache) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyStoreCache {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyStoreCache {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPropertyStoreCache> for IPropertyStore {
    fn from(value: IPropertyStoreCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyStoreCache> for IPropertyStore {
    fn from(value: &IPropertyStoreCache) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyStore> for IPropertyStoreCache {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyStore> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyStore> for &IPropertyStoreCache {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyStore> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCache_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cprops: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY, pv: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pstate: *mut PSC_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY, ppropvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, state: PSC_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyStoreCapabilities(pub ::windows::core::IUnknown);
impl IPropertyStoreCapabilities {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn IsPropertyWritable(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(key)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPropertyStoreCapabilities {
    type Vtable = IPropertyStoreCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8e2d566_186e_4d49_bf41_6909ead56acc);
}
impl ::core::convert::From<IPropertyStoreCapabilities> for ::windows::core::IUnknown {
    fn from(value: IPropertyStoreCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyStoreCapabilities> for ::windows::core::IUnknown {
    fn from(value: &IPropertyStoreCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyStoreCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyStoreCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyStoreFactory(pub ::windows::core::IUnknown);
impl IPropertyStoreFactory {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStore<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), punkfactory.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStoreForKeys<T: ::windows::core::Interface>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rgkeys), ::core::mem::transmute(ckeys), ::core::mem::transmute(flags), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IPropertyStoreFactory {
    type Vtable = IPropertyStoreFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc110b6d_57e8_4148_a9c6_91015ab2f3a5);
}
impl ::core::convert::From<IPropertyStoreFactory> for ::windows::core::IUnknown {
    fn from(value: IPropertyStoreFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyStoreFactory> for ::windows::core::IUnknown {
    fn from(value: &IPropertyStoreFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyStoreFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyStoreFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: GETPROPERTYSTOREFLAGS, punkfactory: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertySystem(pub ::windows::core::IUnknown);
impl IPropertySystem {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyDescription<T: ::windows::core::Interface>(&self, propkey: *const PROPERTYKEY) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(propkey), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyDescriptionByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, T: ::windows::core::Interface>(&self, pszcanonicalname: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszcanonicalname.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyDescriptionListFromString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, T: ::windows::core::Interface>(&self, pszproplist: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszproplist.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn EnumeratePropertyDescriptions<T: ::windows::core::Interface>(&self, filteron: PROPDESC_ENUMFILTER) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(filteron), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FormatForDisplay(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar), ::core::mem::transmute(pdff), ::core::mem::transmute(psztext), ::core::mem::transmute(cchtext)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FormatForDisplayAlloc(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar), ::core::mem::transmute(pdff), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn RegisterPropertySchema<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pszpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszpath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn UnregisterPropertySchema<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pszpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pszpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn RefreshPropertySchema(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPropertySystem {
    type Vtable = IPropertySystem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca724e8a_c3e6_442b_88a4_6fb0db8035a3);
}
impl ::core::convert::From<IPropertySystem> for ::windows::core::IUnknown {
    fn from(value: IPropertySystem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertySystem> for ::windows::core::IUnknown {
    fn from(value: &IPropertySystem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertySystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertySystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propkey: *const PROPERTYKEY, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszcanonicalname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszproplist: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdff: PROPDESC_FORMAT_FLAGS, psztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertySystemChangeNotify(pub ::windows::core::IUnknown);
impl IPropertySystemChangeNotify {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SchemaRefreshed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPropertySystemChangeNotify {
    type Vtable = IPropertySystemChangeNotify_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa955fd9_38be_4879_a6ce_824cf52d609f);
}
impl ::core::convert::From<IPropertySystemChangeNotify> for ::windows::core::IUnknown {
    fn from(value: IPropertySystemChangeNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertySystemChangeNotify> for ::windows::core::IUnknown {
    fn from(value: &IPropertySystemChangeNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertySystemChangeNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertySystemChangeNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystemChangeNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyUI(pub ::windows::core::IUnknown);
impl IPropertyUI {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn ParsePropertyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pszname: Param0, pfmtid: *mut ::windows::core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(pfmtid), ::core::mem::transmute(ppid), ::core::mem::transmute(pcheaten)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCannonicalName(&self, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(pwsztext), ::core::mem::transmute(cchtext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self, fmtid: *const ::windows::core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(flags), ::core::mem::transmute(pwsztext), ::core::mem::transmute(cchtext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyDescription(&self, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(pwsztext), ::core::mem::transmute(cchtext)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetDefaultWidth(&self, fmtid: *const ::windows::core::GUID, pid: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetFlags(&self, fmtid: *const ::windows::core::GUID, pid: u32) -> ::windows::core::Result<PROPERTYUI_FLAGS> {
        let mut result__: <PROPERTYUI_FLAGS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), &mut result__).from_abi::<PROPERTYUI_FLAGS>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FormatForDisplay(&self, fmtid: *const ::windows::core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(ppropvar), ::core::mem::transmute(puiff), ::core::mem::transmute(pwsztext), ::core::mem::transmute(cchtext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetHelpInfo(&self, fmtid: *const ::windows::core::GUID, pid: u32, pwszhelpfile: super::super::super::Foundation::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(pwszhelpfile), ::core::mem::transmute(cch), ::core::mem::transmute(puhelpid)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPropertyUI {
    type Vtable = IPropertyUI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x757a7d9f_919a_4118_99d7_dbb208c8cc66);
}
impl ::core::convert::From<IPropertyUI> for ::windows::core::IUnknown {
    fn from(value: IPropertyUI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyUI> for ::windows::core::IUnknown {
    fn from(value: &IPropertyUI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyUI_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszname: super::super::super::Foundation::PWSTR, pfmtid: *mut ::windows::core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fmtid: *const ::windows::core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fmtid: *const ::windows::core::GUID, pid: u32, pcxchars: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fmtid: *const ::windows::core::GUID, pid: u32, pflags: *mut PROPERTYUI_FLAGS) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fmtid: *const ::windows::core::GUID, pid: u32, ppropvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fmtid: *const ::windows::core::GUID, pid: u32, pwszhelpfile: super::super::super::Foundation::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const InMemoryPropertyStore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a02e012_6303_4e1e_b9a1_630f802592c5);
pub const InMemoryPropertyStoreMarshalByValue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4ca0e2d_6da7_4b75_a97c_5f306f0eaedc);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromBooleanVector(prgf: *const super::super::super::Foundation::BOOL, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromBooleanVector(prgf: *const super::super::super::Foundation::BOOL, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromBooleanVector(::core::mem::transmute(prgf), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromBuffer(::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromCLSID(clsid: *const ::windows::core::GUID, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromCLSID(::core::mem::transmute(clsid), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromDoubleVector(prgn: *const f64, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromDoubleVector(prgn: *const f64, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromDoubleVector(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromFileTime(pftin: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromFileTime(pftin: *const super::super::super::Foundation::FILETIME, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromFileTime(::core::mem::transmute(pftin), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromFileTimeVector(prgft: *const super::super::super::Foundation::FILETIME, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromFileTimeVector(prgft: *const super::super::super::Foundation::FILETIME, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromFileTimeVector(::core::mem::transmute(prgft), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromGUIDAsString(guid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromGUIDAsString(guid: *const ::windows::core::GUID, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromGUIDAsString(::core::mem::transmute(guid), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromInt16Vector(prgn: *const i16, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromInt16Vector(prgn: *const i16, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromInt16Vector(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromInt32Vector(prgn: *const i32, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromInt32Vector(prgn: *const i32, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromInt32Vector(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromInt64Vector(prgn: *const i64, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromInt64Vector(prgn: *const i64, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromInt64Vector(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromPropVariantVectorElem(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromPropVariantVectorElem(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromPropVariantVectorElem(::core::mem::transmute(propvarin), ::core::mem::transmute(ielem), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HINSTANCE>>(hinst: Param0, id: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromResource(hinst: super::super::super::Foundation::HINSTANCE, id: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromResource(hinst.into_param().abi(), ::core::mem::transmute(id), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_Common`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn InitPropVariantFromStrRet(pstrret: *mut super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromStrRet(pstrret: *mut super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        InitPropVariantFromStrRet(::core::mem::transmute(pstrret), ::core::mem::transmute(pidl), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromStringAsVector<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(psz: Param0) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromStringAsVector(psz: super::super::super::Foundation::PWSTR, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromStringAsVector(psz.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromStringVector(prgsz: *const super::super::super::Foundation::PWSTR, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromStringVector(prgsz: *const super::super::super::Foundation::PWSTR, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromStringVector(::core::mem::transmute(prgsz), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt16Vector(prgn: *const u16, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromUInt16Vector(prgn: *const u16, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromUInt16Vector(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt32Vector(prgn: *const u32, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromUInt32Vector(prgn: *const u32, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromUInt32Vector(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt64Vector(prgn: *const u64, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromUInt64Vector(prgn: *const u64, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantFromUInt64Vector(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantVectorFromPropVariant(propvarsingle: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantVectorFromPropVariant(propvarsingle: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppropvarvector: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitPropVariantVectorFromPropVariant(::core::mem::transmute(propvarsingle), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromBooleanArray(prgf: *const super::super::super::Foundation::BOOL, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromBooleanArray(prgf: *const super::super::super::Foundation::BOOL, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromBooleanArray(::core::mem::transmute(prgf), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromBuffer(::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromDoubleArray(prgn: *const f64, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromDoubleArray(prgn: *const f64, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromDoubleArray(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromFileTime(pft: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromFileTime(pft: *const super::super::super::Foundation::FILETIME, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromFileTime(::core::mem::transmute(pft), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromFileTimeArray(prgft: *const super::super::super::Foundation::FILETIME, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromFileTimeArray(prgft: *const super::super::super::Foundation::FILETIME, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromFileTimeArray(::core::mem::transmute(prgft), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromGUIDAsString(guid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromGUIDAsString(guid: *const ::windows::core::GUID, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromGUIDAsString(::core::mem::transmute(guid), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt16Array(prgn: *const i16, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromInt16Array(prgn: *const i16, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromInt16Array(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt32Array(prgn: *const i32, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromInt32Array(prgn: *const i32, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromInt32Array(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt64Array(prgn: *const i64, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromInt64Array(prgn: *const i64, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromInt64Array(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HINSTANCE>>(hinst: Param0, id: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromResource(hinst: super::super::super::Foundation::HINSTANCE, id: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromResource(hinst.into_param().abi(), ::core::mem::transmute(id), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_UI_Shell_Common`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn InitVariantFromStrRet(pstrret: *const super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromStrRet(pstrret: *const super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromStrRet(::core::mem::transmute(pstrret), ::core::mem::transmute(pidl), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromStringArray(prgsz: *const super::super::super::Foundation::PWSTR, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromStringArray(prgsz: *const super::super::super::Foundation::PWSTR, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromStringArray(::core::mem::transmute(prgsz), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt16Array(prgn: *const u16, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromUInt16Array(prgn: *const u16, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromUInt16Array(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt32Array(prgn: *const u32, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromUInt32Array(prgn: *const u32, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromUInt32Array(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt64Array(prgn: *const u64, celems: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromUInt64Array(prgn: *const u64, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromUInt64Array(::core::mem::transmute(prgn), ::core::mem::transmute(celems), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromVariantArrayElem(varin: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromVariantArrayElem(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        InitVariantFromVariantArrayElem(::core::mem::transmute(varin), ::core::mem::transmute(ielem), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PDOPSTATUS(pub i32);
pub const PDOPS_RUNNING: PDOPSTATUS = PDOPSTATUS(1i32);
pub const PDOPS_PAUSED: PDOPSTATUS = PDOPSTATUS(2i32);
pub const PDOPS_CANCELLED: PDOPSTATUS = PDOPSTATUS(3i32);
pub const PDOPS_STOPPED: PDOPSTATUS = PDOPSTATUS(4i32);
pub const PDOPS_ERRORS: PDOPSTATUS = PDOPSTATUS(5i32);
impl ::core::convert::From<i32> for PDOPSTATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PDOPSTATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PKA_FLAGS(pub i32);
pub const PKA_SET: PKA_FLAGS = PKA_FLAGS(0i32);
pub const PKA_APPEND: PKA_FLAGS = PKA_FLAGS(1i32);
pub const PKA_DELETE: PKA_FLAGS = PKA_FLAGS(2i32);
impl ::core::convert::From<i32> for PKA_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PKA_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PIDSTR_MAX: u32 = 10u32;
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PLACEHOLDER_STATES(pub i32);
pub const PS_NONE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(0i32);
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: PLACEHOLDER_STATES = PLACEHOLDER_STATES(1i32);
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(2i32);
pub const PS_CREATE_FILE_ACCESSIBLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(4i32);
pub const PS_CLOUDFILE_PLACEHOLDER: PLACEHOLDER_STATES = PLACEHOLDER_STATES(8i32);
pub const PS_DEFAULT: PLACEHOLDER_STATES = PLACEHOLDER_STATES(7i32);
pub const PS_ALL: PLACEHOLDER_STATES = PLACEHOLDER_STATES(15i32);
impl ::core::convert::From<i32> for PLACEHOLDER_STATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PLACEHOLDER_STATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_AGGREGATION_TYPE(pub i32);
pub const PDAT_DEFAULT: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(0i32);
pub const PDAT_FIRST: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(1i32);
pub const PDAT_SUM: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(2i32);
pub const PDAT_AVERAGE: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(3i32);
pub const PDAT_DATERANGE: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(4i32);
pub const PDAT_UNION: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(5i32);
pub const PDAT_MAX: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(6i32);
pub const PDAT_MIN: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(7i32);
impl ::core::convert::From<i32> for PROPDESC_AGGREGATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_AGGREGATION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_COLUMNINDEX_TYPE(pub i32);
pub const PDCIT_NONE: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(0i32);
pub const PDCIT_ONDISK: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(1i32);
pub const PDCIT_INMEMORY: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(2i32);
pub const PDCIT_ONDEMAND: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(3i32);
pub const PDCIT_ONDISKALL: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(4i32);
pub const PDCIT_ONDISKVECTOR: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(5i32);
impl ::core::convert::From<i32> for PROPDESC_COLUMNINDEX_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_COLUMNINDEX_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_CONDITION_TYPE(pub i32);
pub const PDCOT_NONE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(0i32);
pub const PDCOT_STRING: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(1i32);
pub const PDCOT_SIZE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(2i32);
pub const PDCOT_DATETIME: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(3i32);
pub const PDCOT_BOOLEAN: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(4i32);
pub const PDCOT_NUMBER: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(5i32);
impl ::core::convert::From<i32> for PROPDESC_CONDITION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_CONDITION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_DISPLAYTYPE(pub i32);
pub const PDDT_STRING: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(0i32);
pub const PDDT_NUMBER: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(1i32);
pub const PDDT_BOOLEAN: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(2i32);
pub const PDDT_DATETIME: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(3i32);
pub const PDDT_ENUMERATED: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(4i32);
impl ::core::convert::From<i32> for PROPDESC_DISPLAYTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_DISPLAYTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_ENUMFILTER(pub i32);
pub const PDEF_ALL: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(0i32);
pub const PDEF_SYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(1i32);
pub const PDEF_NONSYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(2i32);
pub const PDEF_VIEWABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(3i32);
pub const PDEF_QUERYABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(4i32);
pub const PDEF_INFULLTEXTQUERY: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(5i32);
pub const PDEF_COLUMN: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(6i32);
impl ::core::convert::From<i32> for PROPDESC_ENUMFILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_ENUMFILTER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_FORMAT_FLAGS(pub i32);
pub const PDFF_DEFAULT: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(0i32);
pub const PDFF_PREFIXNAME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(1i32);
pub const PDFF_FILENAME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(2i32);
pub const PDFF_ALWAYSKB: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(4i32);
pub const PDFF_RESERVED_RIGHTTOLEFT: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(8i32);
pub const PDFF_SHORTTIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(16i32);
pub const PDFF_LONGTIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(32i32);
pub const PDFF_HIDETIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(64i32);
pub const PDFF_SHORTDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(128i32);
pub const PDFF_LONGDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(256i32);
pub const PDFF_HIDEDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(512i32);
pub const PDFF_RELATIVEDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(1024i32);
pub const PDFF_USEEDITINVITATION: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(2048i32);
pub const PDFF_READONLY: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(4096i32);
pub const PDFF_NOAUTOREADINGORDER: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(8192i32);
impl ::core::convert::From<i32> for PROPDESC_FORMAT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_FORMAT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_GROUPING_RANGE(pub i32);
pub const PDGR_DISCRETE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(0i32);
pub const PDGR_ALPHANUMERIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(1i32);
pub const PDGR_SIZE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(2i32);
pub const PDGR_DYNAMIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(3i32);
pub const PDGR_DATE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(4i32);
pub const PDGR_PERCENT: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(5i32);
pub const PDGR_ENUMERATED: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(6i32);
impl ::core::convert::From<i32> for PROPDESC_GROUPING_RANGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_GROUPING_RANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_RELATIVEDESCRIPTION_TYPE(pub i32);
pub const PDRDT_GENERAL: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(0i32);
pub const PDRDT_DATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(1i32);
pub const PDRDT_SIZE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(2i32);
pub const PDRDT_COUNT: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(3i32);
pub const PDRDT_REVISION: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(4i32);
pub const PDRDT_LENGTH: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(5i32);
pub const PDRDT_DURATION: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(6i32);
pub const PDRDT_SPEED: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(7i32);
pub const PDRDT_RATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(8i32);
pub const PDRDT_RATING: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(9i32);
pub const PDRDT_PRIORITY: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(10i32);
impl ::core::convert::From<i32> for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_SEARCHINFO_FLAGS(pub i32);
pub const PDSIF_DEFAULT: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(0i32);
pub const PDSIF_ININVERTEDINDEX: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(1i32);
pub const PDSIF_ISCOLUMN: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(2i32);
pub const PDSIF_ISCOLUMNSPARSE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(4i32);
pub const PDSIF_ALWAYSINCLUDE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(8i32);
pub const PDSIF_USEFORTYPEAHEAD: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(16i32);
impl ::core::convert::From<i32> for PROPDESC_SEARCHINFO_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_SEARCHINFO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_SORTDESCRIPTION(pub i32);
pub const PDSD_GENERAL: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(0i32);
pub const PDSD_A_Z: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(1i32);
pub const PDSD_LOWEST_HIGHEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(2i32);
pub const PDSD_SMALLEST_BIGGEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(3i32);
pub const PDSD_OLDEST_NEWEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(4i32);
impl ::core::convert::From<i32> for PROPDESC_SORTDESCRIPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_SORTDESCRIPTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_TYPE_FLAGS(pub i32);
pub const PDTF_DEFAULT: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(0i32);
pub const PDTF_MULTIPLEVALUES: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(1i32);
pub const PDTF_ISINNATE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2i32);
pub const PDTF_ISGROUP: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(4i32);
pub const PDTF_CANGROUPBY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(8i32);
pub const PDTF_CANSTACKBY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(16i32);
pub const PDTF_ISTREEPROPERTY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(32i32);
pub const PDTF_INCLUDEINFULLTEXTQUERY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(64i32);
pub const PDTF_ISVIEWABLE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(128i32);
pub const PDTF_ISQUERYABLE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(256i32);
pub const PDTF_CANBEPURGED: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(512i32);
pub const PDTF_SEARCHRAWVALUE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(1024i32);
pub const PDTF_DONTCOERCEEMPTYSTRINGS: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2048i32);
pub const PDTF_ALWAYSINSUPPLEMENTALSTORE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(4096i32);
pub const PDTF_ISSYSTEMPROPERTY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(-2147483648i32);
pub const PDTF_MASK_ALL: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(-2147475457i32);
impl ::core::convert::From<i32> for PROPDESC_TYPE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_TYPE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_VIEW_FLAGS(pub i32);
pub const PDVF_DEFAULT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(0i32);
pub const PDVF_CENTERALIGN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(1i32);
pub const PDVF_RIGHTALIGN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(2i32);
pub const PDVF_BEGINNEWGROUP: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(4i32);
pub const PDVF_FILLAREA: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(8i32);
pub const PDVF_SORTDESCENDING: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(16i32);
pub const PDVF_SHOWONLYIFPRESENT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(32i32);
pub const PDVF_SHOWBYDEFAULT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(64i32);
pub const PDVF_SHOWINPRIMARYLIST: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(128i32);
pub const PDVF_SHOWINSECONDARYLIST: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(256i32);
pub const PDVF_HIDELABEL: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(512i32);
pub const PDVF_HIDDEN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(2048i32);
pub const PDVF_CANWRAP: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(4096i32);
pub const PDVF_MASK_ALL: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(7167i32);
impl ::core::convert::From<i32> for PROPDESC_VIEW_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPDESC_VIEW_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPENUMTYPE(pub i32);
pub const PET_DISCRETEVALUE: PROPENUMTYPE = PROPENUMTYPE(0i32);
pub const PET_RANGEDVALUE: PROPENUMTYPE = PROPENUMTYPE(1i32);
pub const PET_DEFAULTVALUE: PROPENUMTYPE = PROPENUMTYPE(2i32);
pub const PET_ENDRANGE: PROPENUMTYPE = PROPENUMTYPE(3i32);
impl ::core::convert::From<i32> for PROPENUMTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPENUMTYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
pub struct PROPERTYKEY {
    pub fmtid: ::windows::core::GUID,
    pub pid: u32,
}
impl PROPERTYKEY {}
impl ::core::default::Default for PROPERTYKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROPERTYKEY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROPERTYKEY").field("fmtid", &self.fmtid).field("pid", &self.pid).finish()
    }
}
impl ::core::cmp::PartialEq for PROPERTYKEY {
    fn eq(&self, other: &Self) -> bool {
        self.fmtid == other.fmtid && self.pid == other.pid
    }
}
impl ::core::cmp::Eq for PROPERTYKEY {}
unsafe impl ::windows::core::Abi for PROPERTYKEY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPERTYUI_FLAGS(pub i32);
pub const PUIF_DEFAULT: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(0i32);
pub const PUIF_RIGHTALIGN: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(1i32);
pub const PUIF_NOLABELININFOTIP: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(2i32);
impl ::core::convert::From<i32> for PROPERTYUI_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPERTYUI_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPERTYUI_FORMAT_FLAGS(pub i32);
pub const PUIFFDF_DEFAULT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(0i32);
pub const PUIFFDF_RIGHTTOLEFT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(1i32);
pub const PUIFFDF_SHORTFORMAT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(2i32);
pub const PUIFFDF_NOTIME: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(4i32);
pub const PUIFFDF_FRIENDLYDATE: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(8i32);
impl ::core::convert::From<i32> for PROPERTYUI_FORMAT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPERTYUI_FORMAT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPERTYUI_NAME_FLAGS(pub i32);
pub const PUIFNF_DEFAULT: PROPERTYUI_NAME_FLAGS = PROPERTYUI_NAME_FLAGS(0i32);
pub const PUIFNF_MNEMONIC: PROPERTYUI_NAME_FLAGS = PROPERTYUI_NAME_FLAGS(1i32);
impl ::core::convert::From<i32> for PROPERTYUI_NAME_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPERTYUI_NAME_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [super::super::super::Foundation::CHAR; 30],
    pub achCmdLine: [super::super::super::Foundation::CHAR; 128],
    pub achWorkDir: [super::super::super::Foundation::CHAR; 64],
    pub wHotKey: u16,
    pub achIconFile: [super::super::super::Foundation::CHAR; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [super::super::super::Foundation::CHAR; 80],
    pub achPIFFile: [super::super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl PROPPRG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPPRG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPPRG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPPRG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROPPRG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPVAR_CHANGE_FLAGS(pub i32);
pub const PVCHF_DEFAULT: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(0i32);
pub const PVCHF_NOVALUEPROP: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(1i32);
pub const PVCHF_ALPHABOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(2i32);
pub const PVCHF_NOUSEROVERRIDE: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(4i32);
pub const PVCHF_LOCALBOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(8i32);
pub const PVCHF_NOHEXSTRING: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(16i32);
impl ::core::convert::From<i32> for PROPVAR_CHANGE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPVAR_CHANGE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPVAR_COMPARE_FLAGS(pub i32);
pub const PVCF_DEFAULT: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(0i32);
pub const PVCF_TREATEMPTYASGREATERTHAN: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(1i32);
pub const PVCF_USESTRCMP: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(2i32);
pub const PVCF_USESTRCMPC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(4i32);
pub const PVCF_USESTRCMPI: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(8i32);
pub const PVCF_USESTRCMPIC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(16i32);
pub const PVCF_DIGITSASNUMBERS_CASESENSITIVE: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(32i32);
impl ::core::convert::From<i32> for PROPVAR_COMPARE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPVAR_COMPARE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPVAR_COMPARE_UNIT(pub i32);
pub const PVCU_DEFAULT: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(0i32);
pub const PVCU_SECOND: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(1i32);
pub const PVCU_MINUTE: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(2i32);
pub const PVCU_HOUR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(3i32);
pub const PVCU_DAY: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(4i32);
pub const PVCU_MONTH: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(5i32);
pub const PVCU_YEAR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(6i32);
impl ::core::convert::From<i32> for PROPVAR_COMPARE_UNIT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPVAR_COMPARE_UNIT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSC_STATE(pub i32);
pub const PSC_NORMAL: PSC_STATE = PSC_STATE(0i32);
pub const PSC_NOTINSOURCE: PSC_STATE = PSC_STATE(1i32);
pub const PSC_DIRTY: PSC_STATE = PSC_STATE(2i32);
pub const PSC_READONLY: PSC_STATE = PSC_STATE(3i32);
impl ::core::convert::From<i32> for PSC_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PSC_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        PSCoerceToCanonicalValue(::core::mem::transmute(key), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSCreateAdapterFromPropertyStore<'a, Param0: ::windows::core::IntoParam<'a, IPropertyStore>>(pps: Param0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateAdapterFromPropertyStore(pps: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSCreateAdapterFromPropertyStore(pps.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSCreateDelayedMultiplexPropertyStore<'a, Param1: ::windows::core::IntoParam<'a, IDelayedPropertyStoreFactory>>(flags: GETPROPERTYSTOREFLAGS, pdpsf: Param1, rgstoreids: *const u32, cstores: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateDelayedMultiplexPropertyStore(flags: GETPROPERTYSTOREFLAGS, pdpsf: ::windows::core::RawPtr, rgstoreids: *const u32, cstores: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSCreateDelayedMultiplexPropertyStore(::core::mem::transmute(flags), pdpsf.into_param().abi(), ::core::mem::transmute(rgstoreids), ::core::mem::transmute(cstores), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSCreateMemoryPropertyStore(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateMemoryPropertyStore(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSCreateMemoryPropertyStore(::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSCreateMultiplexPropertyStore(prgpunkstores: *const ::core::option::Option<::windows::core::IUnknown>, cstores: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateMultiplexPropertyStore(prgpunkstores: *const ::windows::core::RawPtr, cstores: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSCreateMultiplexPropertyStore(::core::mem::transmute(prgpunkstores), ::core::mem::transmute(cstores), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSCreatePropertyChangeArray(rgpropkey: *const PROPERTYKEY, rgflags: *const PKA_FLAGS, rgpropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, cchanges: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreatePropertyChangeArray(rgpropkey: *const PROPERTYKEY, rgflags: *const PKA_FLAGS, rgpropvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, cchanges: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSCreatePropertyChangeArray(::core::mem::transmute(rgpropkey), ::core::mem::transmute(rgflags), ::core::mem::transmute(rgpropvar), ::core::mem::transmute(cchanges), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0, grfmode: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreatePropertyStoreFromObject(punk: ::windows::core::RawPtr, grfmode: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSCreatePropertyStoreFromObject(punk.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromPropertySetStorage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertySetStorage>>(ppss: Param0, grfmode: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreatePropertyStoreFromPropertySetStorage(ppss: ::windows::core::RawPtr, grfmode: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSCreatePropertyStoreFromPropertySetStorage(ppss.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSCreateSimplePropertyChange(::core::mem::transmute(flags), ::core::mem::transmute(key), ::core::mem::transmute(propvar), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSEnumeratePropertyDescriptions(::core::mem::transmute(filteron), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT;
        }
        PSFormatForDisplay(::core::mem::transmute(propkey), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), ::core::mem::transmute(pwsztext), ::core::mem::transmute(cchtext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSFormatForDisplayAlloc(::core::mem::transmute(key), ::core::mem::transmute(propvar), ::core::mem::transmute(pdff), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSFormatPropertyValue<'a, Param0: ::windows::core::IntoParam<'a, IPropertyStore>, Param1: ::windows::core::IntoParam<'a, IPropertyDescription>>(pps: Param0, ppd: Param1, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSFormatPropertyValue(pps: ::windows::core::RawPtr, ppd: ::windows::core::RawPtr, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSFormatPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), ::core::mem::transmute(pdff), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSGetImageReferenceForValue(::core::mem::transmute(propkey), ::core::mem::transmute(propvar), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetItemPropertyHandler<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(punkitem: Param0, freadwrite: Param1, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetItemPropertyHandler(punkitem: ::windows::core::RawPtr, freadwrite: super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSGetItemPropertyHandler(punkitem.into_param().abi(), freadwrite.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetItemPropertyHandlerWithCreateObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkitem: Param0, freadwrite: Param1, punkcreateobject: Param2, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetItemPropertyHandlerWithCreateObject(punkitem: ::windows::core::RawPtr, freadwrite: super::super::super::Foundation::BOOL, punkcreateobject: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSGetItemPropertyHandlerWithCreateObject(punkitem.into_param().abi(), freadwrite.into_param().abi(), punkcreateobject.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY, ppszcanonicalname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSGetNameFromPropertyKey(::core::mem::transmute(propkey), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetNamedPropertyFromPropertyStorage<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: Param2) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetNamedPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: super::super::super::Foundation::PWSTR, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSGetNamedPropertyFromPropertyStorage(::core::mem::transmute(psps), ::core::mem::transmute(cb), pszname.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSGetPropertyDescription(::core::mem::transmute(propkey), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetPropertyDescriptionByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszcanonicalname: Param0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyDescriptionByName(pszcanonicalname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSGetPropertyDescriptionByName(pszcanonicalname.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetPropertyDescriptionListFromString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszproplist: Param0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyDescriptionListFromString(pszproplist: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSGetPropertyDescriptionListFromString(pszproplist.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSGetPropertyFromPropertyStorage(::core::mem::transmute(psps), ::core::mem::transmute(cb), ::core::mem::transmute(rpkey), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetPropertyKeyFromName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszname: Param0) -> ::windows::core::Result<PROPERTYKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyKeyFromName(pszname: super::super::super::Foundation::PWSTR, ppropkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT;
        }
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSGetPropertyKeyFromName(pszname.into_param().abi(), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSGetPropertySystem(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertySystem(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSGetPropertySystem(::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetPropertyValue<'a, Param0: ::windows::core::IntoParam<'a, IPropertyStore>, Param1: ::windows::core::IntoParam<'a, IPropertyDescription>>(pps: Param0, ppd: Param1) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyValue(pps: ::windows::core::RawPtr, ppd: ::windows::core::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSGetPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSLookupPropertyHandlerCLSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszfilepath: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSLookupPropertyHandlerCLSID(pszfilepath: super::super::super::Foundation::PWSTR, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSLookupPropertyHandlerCLSID(pszfilepath.into_param().abi(), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_Delete<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_Delete(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_Delete(propbag.into_param().abi(), propname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadBOOL<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadBOOL(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadBOOL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadBSTR<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadBSTR(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadBSTR(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadDWORD<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadDWORD(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadDWORD(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadGUID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadGUID(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadGUID(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadInt<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadInt(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut i32) -> ::windows::core::HRESULT;
        }
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadInt(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadLONG<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadLONG(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut i32) -> ::windows::core::HRESULT;
        }
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadLONG(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTL<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<super::super::super::Foundation::POINTL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadPOINTL(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::POINTL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::POINTL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadPOINTL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::POINTL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTS<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<super::super::super::Foundation::POINTS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadPOINTS(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::POINTS) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::POINTS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadPOINTS(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::POINTS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPropertyKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<PROPERTYKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadPropertyKey(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut PROPERTYKEY) -> ::windows::core::HRESULT;
        }
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadPropertyKey(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadRECTL<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<super::super::super::Foundation::RECTL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadRECTL(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::RECTL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::RECTL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadRECTL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::RECTL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadSHORT<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadSHORT(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut i16) -> ::windows::core::HRESULT;
        }
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadSHORT(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadStr<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: super::super::super::Foundation::PWSTR, charactercount: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadStr(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR, charactercount: i32) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_ReadStr(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value), ::core::mem::transmute(charactercount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadStrAlloc<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadStrAlloc(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadStrAlloc(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadStream(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadStream(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, var: *mut super::super::super::System::Com::VARIANT, r#type: u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadType(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, var: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, r#type: u16) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_ReadType(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(var), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadULONGLONG<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadULONGLONG(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyBag_ReadULONGLONG(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadUnknown<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadUnknown(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_ReadUnknown(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteBOOL<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteBOOL(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteBOOL(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteBSTR<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteBSTR(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteBSTR(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteDWORD<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteDWORD(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: u32) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteDWORD(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteGUID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteGUID(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteGUID(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteInt<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteInt(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: i32) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteInt(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteLONG<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteLONG(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: i32) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteLONG(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTL<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: *const super::super::super::Foundation::POINTL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WritePOINTL(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::POINTL) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WritePOINTL(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTS<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: *const super::super::super::Foundation::POINTS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WritePOINTS(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::POINTS) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WritePOINTS(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePropertyKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: *const PROPERTYKEY) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WritePropertyKey(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *const PROPERTYKEY) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WritePropertyKey(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteRECTL<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: *const super::super::super::Foundation::RECTL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteRECTL(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::RECTL) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteRECTL(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteSHORT<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: i16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteSHORT(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: i16) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteSHORT(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteStr<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteStr(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteStr(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteStream(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteStream(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteULONGLONG<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: u64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteULONGLONG(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, value: u64) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteULONGLONG(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteUnknown<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(propbag: Param0, propname: Param1, punk: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteUnknown(propbag: ::windows::core::RawPtr, propname: super::super::super::Foundation::PWSTR, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        PSPropertyBag_WriteUnknown(propbag.into_param().abi(), propname.into_param().abi(), punk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSPropertyKeyFromString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszstring: Param0) -> ::windows::core::Result<PROPERTYKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyKeyFromString(pszstring: super::super::super::Foundation::PWSTR, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT;
        }
        let mut result__: <PROPERTYKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PSPropertyKeyFromString(pszstring.into_param().abi(), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSRefreshPropertySchema() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSRefreshPropertySchema() -> ::windows::core::HRESULT;
        }
        PSRefreshPropertySchema().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSRegisterPropertySchema<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszpath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSRegisterPropertySchema(pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PSRegisterPropertySchema(pszpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSSetPropertyValue<'a, Param0: ::windows::core::IntoParam<'a, IPropertyStore>, Param1: ::windows::core::IntoParam<'a, IPropertyDescription>>(pps: Param0, ppd: Param1, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSSetPropertyValue(pps: ::windows::core::RawPtr, ppd: ::windows::core::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        PSSetPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), ::core::mem::transmute(propvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: super::super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: super::super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT;
        }
        PSStringFromPropertyKey(::core::mem::transmute(pkey), ::core::mem::transmute(psz), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSTIME_FLAGS(pub i32);
pub const PSTF_UTC: PSTIME_FLAGS = PSTIME_FLAGS(0i32);
pub const PSTF_LOCAL: PSTIME_FLAGS = PSTIME_FLAGS(1i32);
impl ::core::convert::From<i32> for PSTIME_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PSTIME_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSUnregisterPropertySchema<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszpath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSUnregisterPropertySchema(pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PSUnregisterPropertySchema(pszpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_CloseProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hprops: Param0, flopt: u32) -> super::super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_CloseProperties(hprops: super::super::super::Foundation::HANDLE, flopt: u32) -> super::super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(PifMgr_CloseProperties(hprops.into_param().abi(), ::core::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_GetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(hprops: Param0, pszgroup: Param1, lpprops: *mut ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_GetProperties(hprops: super::super::super::Foundation::HANDLE, pszgroup: super::super::super::Foundation::PSTR, lpprops: *mut ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
        }
        ::core::mem::transmute(PifMgr_GetProperties(hprops.into_param().abi(), pszgroup.into_param().abi(), ::core::mem::transmute(lpprops), ::core::mem::transmute(cbprops), ::core::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_OpenProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszapp: Param0, pszpif: Param1, hinf: u32, flopt: u32) -> super::super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_OpenProperties(pszapp: super::super::super::Foundation::PWSTR, pszpif: super::super::super::Foundation::PWSTR, hinf: u32, flopt: u32) -> super::super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(PifMgr_OpenProperties(pszapp.into_param().abi(), pszpif.into_param().abi(), ::core::mem::transmute(hinf), ::core::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_SetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(hprops: Param0, pszgroup: Param1, lpprops: *const ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_SetProperties(hprops: super::super::super::Foundation::HANDLE, pszgroup: super::super::super::Foundation::PSTR, lpprops: *const ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
        }
        ::core::mem::transmute(PifMgr_SetProperties(hprops.into_param().abi(), pszgroup.into_param().abi(), ::core::mem::transmute(lpprops), ::core::mem::transmute(cbprops), ::core::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantChangeType(ppropvardest: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvarsrc: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantChangeType(ppropvardest: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propvarsrc: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, flags: PROPVAR_CHANGE_FLAGS, vt: u16) -> ::windows::core::HRESULT;
        }
        PropVariantChangeType(::core::mem::transmute(ppropvardest), ::core::mem::transmute(propvarsrc), ::core::mem::transmute(flags), ::core::mem::transmute(vt)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantCompareEx(propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantCompareEx(propvar1: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32;
        }
        ::core::mem::transmute(PropVariantCompareEx(::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(unit), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetBooleanElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetBooleanElem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pfval: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantGetBooleanElem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetDoubleElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetDoubleElem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut f64) -> ::windows::core::HRESULT;
        }
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantGetDoubleElem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetElementCount(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetElementCount(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> u32;
        }
        ::core::mem::transmute(PropVariantGetElementCount(::core::mem::transmute(propvar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetFileTimeElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetFileTimeElem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pftval: *mut super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantGetFileTimeElem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), &mut result__).from_abi::<super::super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetInt16Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInt16Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut i16) -> ::windows::core::HRESULT;
        }
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantGetInt16Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetInt32Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInt32Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut i32) -> ::windows::core::HRESULT;
        }
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantGetInt32Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetInt64Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInt64Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut i64) -> ::windows::core::HRESULT;
        }
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantGetInt64Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetStringElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetStringElem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, ppszval: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantGetStringElem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetUInt16Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetUInt16Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut u16) -> ::windows::core::HRESULT;
        }
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantGetUInt16Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetUInt32Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetUInt32Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantGetUInt32Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetUInt64Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetUInt64Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantGetUInt64Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBSTR(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBSTR(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToBSTR(::core::mem::transmute(propvar), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBoolean(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBoolean(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pfret: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToBoolean(::core::mem::transmute(propvarin), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBooleanVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgf: *mut super::super::super::Foundation::BOOL, crgf: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBooleanVector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgf: *mut super::super::super::Foundation::BOOL, crgf: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToBooleanVector(::core::mem::transmute(propvar), ::core::mem::transmute(prgf), ::core::mem::transmute(crgf), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBooleanVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBooleanVectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToBooleanVectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgf), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBooleanWithDefault<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, fdefault: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBooleanWithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, fdefault: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PropVariantToBooleanWithDefault(::core::mem::transmute(propvarin), fdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBuffer(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBuffer(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::HRESULT;
        }
        PropVariantToBuffer(::core::mem::transmute(propvar), ::core::mem::transmute(pv), ::core::mem::transmute(cb)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDouble(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDouble(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdblret: *mut f64) -> ::windows::core::HRESULT;
        }
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToDouble(::core::mem::transmute(propvarin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDoubleVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDoubleVector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToDoubleVector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDoubleVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDoubleVectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToDoubleVectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDoubleWithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, dbldefault: f64) -> f64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDoubleWithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, dbldefault: f64) -> f64;
        }
        ::core::mem::transmute(PropVariantToDoubleWithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(dbldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToFileTime(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstfout: PSTIME_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToFileTime(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pstfout: PSTIME_FLAGS, pftout: *mut super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToFileTime(::core::mem::transmute(propvar), ::core::mem::transmute(pstfout), &mut result__).from_abi::<super::super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToFileTimeVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgft: *mut super::super::super::Foundation::FILETIME, crgft: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToFileTimeVector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgft: *mut super::super::super::Foundation::FILETIME, crgft: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToFileTimeVector(::core::mem::transmute(propvar), ::core::mem::transmute(prgft), ::core::mem::transmute(crgft), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToFileTimeVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgft: *mut *mut super::super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToFileTimeVectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgft: *mut *mut super::super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToFileTimeVectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgft), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToGUID(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToGUID(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToGUID(::core::mem::transmute(propvar), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, piret: *mut i16) -> ::windows::core::HRESULT;
        }
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToInt16(::core::mem::transmute(propvarin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToInt16Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToInt16VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, idefault: i16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, idefault: i16) -> i16;
        }
        ::core::mem::transmute(PropVariantToInt16WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(idefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, plret: *mut i32) -> ::windows::core::HRESULT;
        }
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToInt32(::core::mem::transmute(propvarin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToInt32Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToInt32VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ldefault: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ldefault: i32) -> i32;
        }
        ::core::mem::transmute(PropVariantToInt32WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(ldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pllret: *mut i64) -> ::windows::core::HRESULT;
        }
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToInt64(::core::mem::transmute(propvarin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToInt64Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToInt64VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, lldefault: i64) -> i64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, lldefault: i64) -> i64;
        }
        ::core::mem::transmute(PropVariantToInt64WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(lldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_Common`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn PropVariantToStrRet(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::Common::STRRET> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStrRet(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pstrret: *mut super::Common::STRRET) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::Common::STRRET as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToStrRet(::core::mem::transmute(propvar), &mut result__).from_abi::<super::Common::STRRET>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToString(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, psz: super::super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToString(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, psz: super::super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT;
        }
        PropVariantToString(::core::mem::transmute(propvar), ::core::mem::transmute(psz), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszout: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToStringAlloc(::core::mem::transmute(propvar), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgsz: *mut super::super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringVector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgsz: *mut super::super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToStringVector(::core::mem::transmute(propvar), ::core::mem::transmute(prgsz), ::core::mem::transmute(crgsz), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgsz: *mut *mut super::super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringVectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgsz: *mut *mut super::super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToStringVectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgsz), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringWithDefault<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pszdefault: Param1) -> super::super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringWithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pszdefault: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(PropVariantToStringWithDefault(::core::mem::transmute(propvarin), pszdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, puiret: *mut u16) -> ::windows::core::HRESULT;
        }
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToUInt16(::core::mem::transmute(propvarin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToUInt16Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToUInt16VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uidefault: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, uidefault: u16) -> u16;
        }
        ::core::mem::transmute(PropVariantToUInt16WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(uidefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pulret: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToUInt32(::core::mem::transmute(propvarin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToUInt32Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToUInt32VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uldefault: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, uldefault: u32) -> u32;
        }
        ::core::mem::transmute(PropVariantToUInt32WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(uldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pullret: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToUInt64(::core::mem::transmute(propvarin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToUInt64Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        PropVariantToUInt64VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ulldefault: u64) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ulldefault: u64) -> u64;
        }
        ::core::mem::transmute(PropVariantToUInt64WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(ulldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn PropVariantToVariant(ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToVariant(ppropvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        PropVariantToVariant(::core::mem::transmute(ppropvar), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToWinRTPropertyValue<T: ::windows::core::Interface>(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToWinRTPropertyValue(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PropVariantToWinRTPropertyValue(::core::mem::transmute(propvar), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const PropertySystem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8967f85_58ae_4f46_9fb2_5d7904798f4b);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct SERIALIZEDPROPSTORAGE(pub u8);
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SHAddDefaultPropertiesByExt<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IPropertyStore>>(pszext: Param0, ppropstore: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHAddDefaultPropertiesByExt(pszext: super::super::super::Foundation::PWSTR, ppropstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        SHAddDefaultPropertiesByExt(pszext.into_param().abi(), ppropstore.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SHGetPropertyStoreForWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHGetPropertyStoreForWindow(hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        SHGetPropertyStoreForWindow(hwnd.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_UI_Shell_Common`*"]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[inline]
pub unsafe fn SHGetPropertyStoreFromIDList(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHGetPropertyStoreFromIDList(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        SHGetPropertyStoreFromIDList(::core::mem::transmute(pidl), ::core::mem::transmute(flags), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn SHGetPropertyStoreFromParsingName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IBindCtx>>(pszpath: Param0, pbc: Param1, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHGetPropertyStoreFromParsingName(pszpath: super::super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        SHGetPropertyStoreFromParsingName(pszpath.into_param().abi(), pbc.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn SHPropStgCreate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertySetStorage>>(psstg: Param0, fmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut ::core::option::Option<super::super::super::System::Com::StructuredStorage::IPropertyStorage>, pucodepage: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHPropStgCreate(psstg: ::windows::core::RawPtr, fmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut ::windows::core::RawPtr, pucodepage: *mut u32) -> ::windows::core::HRESULT;
        }
        SHPropStgCreate(psstg.into_param().abi(), ::core::mem::transmute(fmtid), ::core::mem::transmute(pclsid), ::core::mem::transmute(grfflags), ::core::mem::transmute(grfmode), ::core::mem::transmute(dwdisposition), ::core::mem::transmute(ppstg), ::core::mem::transmute(pucodepage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn SHPropStgReadMultiple<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyStorage>>(pps: Param0, ucodepage: u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHPropStgReadMultiple(pps: ::windows::core::RawPtr, ucodepage: u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        SHPropStgReadMultiple(pps.into_param().abi(), ::core::mem::transmute(ucodepage), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn SHPropStgWriteMultiple<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyStorage>>(pps: Param0, pucodepage: *mut u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHPropStgWriteMultiple(pps: ::windows::core::RawPtr, pucodepage: *mut u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propidnamefirst: u32) -> ::windows::core::HRESULT;
        }
        SHPropStgWriteMultiple(pps.into_param().abi(), ::core::mem::transmute(pucodepage), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), ::core::mem::transmute(rgvar), ::core::mem::transmute(propidnamefirst)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYNC_ENGINE_STATE_FLAGS(pub i32);
pub const SESF_NONE: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(0i32);
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(1i32);
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(2i32);
pub const SESF_AUTHENTICATION_ERROR: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(4i32);
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(8i32);
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(16i32);
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(32i32);
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(64i32);
pub const SESF_SERVICE_UNAVAILABLE: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(128i32);
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(256i32);
pub const SESF_ALL_FLAGS: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(511i32);
impl ::core::convert::From<i32> for SYNC_ENGINE_STATE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SYNC_ENGINE_STATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYNC_TRANSFER_STATUS(pub i32);
pub const STS_NONE: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(0i32);
pub const STS_NEEDSUPLOAD: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(1i32);
pub const STS_NEEDSDOWNLOAD: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(2i32);
pub const STS_TRANSFERRING: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(4i32);
pub const STS_PAUSED: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(8i32);
pub const STS_HASERROR: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(16i32);
pub const STS_FETCHING_METADATA: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(32i32);
pub const STS_USER_REQUESTED_REFRESH: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(64i32);
pub const STS_HASWARNING: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(128i32);
pub const STS_EXCLUDED: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(256i32);
pub const STS_INCOMPLETE: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(512i32);
pub const STS_PLACEHOLDER_IFEMPTY: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(1024i32);
impl ::core::convert::From<i32> for SYNC_TRANSFER_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SYNC_TRANSFER_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantCompare(var1: *const super::super::super::System::Com::VARIANT, var2: *const super::super::super::System::Com::VARIANT) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantCompare(var1: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var2: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> i32;
        }
        ::core::mem::transmute(VariantCompare(::core::mem::transmute(var1), ::core::mem::transmute(var2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetBooleanElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetBooleanElem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pfval: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantGetBooleanElem(::core::mem::transmute(var), ::core::mem::transmute(ielem), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetDoubleElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetDoubleElem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut f64) -> ::windows::core::HRESULT;
        }
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantGetDoubleElem(::core::mem::transmute(var), ::core::mem::transmute(ielem), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetElementCount(varin: *const super::super::super::System::Com::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetElementCount(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> u32;
        }
        ::core::mem::transmute(VariantGetElementCount(::core::mem::transmute(varin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt16Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetInt16Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut i16) -> ::windows::core::HRESULT;
        }
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantGetInt16Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt32Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetInt32Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut i32) -> ::windows::core::HRESULT;
        }
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantGetInt32Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt64Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetInt64Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut i64) -> ::windows::core::HRESULT;
        }
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantGetInt64Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetStringElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetStringElem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, ppszval: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantGetStringElem(::core::mem::transmute(var), ::core::mem::transmute(ielem), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt16Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetUInt16Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut u16) -> ::windows::core::HRESULT;
        }
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantGetUInt16Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt32Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetUInt32Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantGetUInt32Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt64Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetUInt64Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantGetUInt64Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBoolean(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBoolean(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pfret: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToBoolean(::core::mem::transmute(varin), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanArray(var: *const super::super::super::System::Com::VARIANT, prgf: *mut super::super::super::Foundation::BOOL, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBooleanArray(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgf: *mut super::super::super::Foundation::BOOL, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToBooleanArray(::core::mem::transmute(var), ::core::mem::transmute(prgf), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBooleanArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToBooleanArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgf), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanWithDefault<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(varin: *const super::super::super::System::Com::VARIANT, fdefault: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBooleanWithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, fdefault: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VariantToBooleanWithDefault(::core::mem::transmute(varin), fdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBuffer(varin: *const super::super::super::System::Com::VARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBuffer(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::HRESULT;
        }
        VariantToBuffer(::core::mem::transmute(varin), ::core::mem::transmute(pv), ::core::mem::transmute(cb)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDosDateTime(varin: *const super::super::super::System::Com::VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDosDateTime(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pwdate: *mut u16, pwtime: *mut u16) -> ::windows::core::HRESULT;
        }
        VariantToDosDateTime(::core::mem::transmute(varin), ::core::mem::transmute(pwdate), ::core::mem::transmute(pwtime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDouble(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDouble(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pdblret: *mut f64) -> ::windows::core::HRESULT;
        }
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToDouble(::core::mem::transmute(varin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleArray(var: *const super::super::super::System::Com::VARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDoubleArray(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToDoubleArray(::core::mem::transmute(var), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDoubleArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToDoubleArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleWithDefault(varin: *const super::super::super::System::Com::VARIANT, dbldefault: f64) -> f64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDoubleWithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, dbldefault: f64) -> f64;
        }
        ::core::mem::transmute(VariantToDoubleWithDefault(::core::mem::transmute(varin), ::core::mem::transmute(dbldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToFileTime(varin: *const super::super::super::System::Com::VARIANT, stfout: PSTIME_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToFileTime(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, stfout: PSTIME_FLAGS, pftout: *mut super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToFileTime(::core::mem::transmute(varin), ::core::mem::transmute(stfout), &mut result__).from_abi::<super::super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToGUID(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToGUID(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToGUID(::core::mem::transmute(varin), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, piret: *mut i16) -> ::windows::core::HRESULT;
        }
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToInt16(::core::mem::transmute(varin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToInt16Array(::core::mem::transmute(var), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToInt16ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16WithDefault(varin: *const super::super::super::System::Com::VARIANT, idefault: i16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, idefault: i16) -> i16;
        }
        ::core::mem::transmute(VariantToInt16WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(idefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, plret: *mut i32) -> ::windows::core::HRESULT;
        }
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToInt32(::core::mem::transmute(varin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToInt32Array(::core::mem::transmute(var), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToInt32ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32WithDefault(varin: *const super::super::super::System::Com::VARIANT, ldefault: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ldefault: i32) -> i32;
        }
        ::core::mem::transmute(VariantToInt32WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(ldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pllret: *mut i64) -> ::windows::core::HRESULT;
        }
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToInt64(::core::mem::transmute(varin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToInt64Array(::core::mem::transmute(var), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToInt64ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64WithDefault(varin: *const super::super::super::System::Com::VARIANT, lldefault: i64) -> i64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, lldefault: i64) -> i64;
        }
        ::core::mem::transmute(VariantToInt64WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(lldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToPropVariant(pvar: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToPropVariant(pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToPropVariant(::core::mem::transmute(pvar), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_UI_Shell_Common`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn VariantToStrRet(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::Common::STRRET> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStrRet(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pstrret: *mut super::Common::STRRET) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::Common::STRRET as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToStrRet(::core::mem::transmute(varin), &mut result__).from_abi::<super::Common::STRRET>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToString(varin: *const super::super::super::System::Com::VARIANT, pszbuf: super::super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToString(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pszbuf: super::super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows::core::HRESULT;
        }
        VariantToString(::core::mem::transmute(varin), ::core::mem::transmute(pszbuf), ::core::mem::transmute(cchbuf)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringAlloc(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringAlloc(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppszbuf: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToStringAlloc(::core::mem::transmute(varin), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringArray(var: *const super::super::super::System::Com::VARIANT, prgsz: *mut super::super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringArray(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgsz: *mut super::super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToStringArray(::core::mem::transmute(var), ::core::mem::transmute(prgsz), ::core::mem::transmute(crgsz), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgsz: *mut *mut super::super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgsz: *mut *mut super::super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToStringArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgsz), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringWithDefault<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(varin: *const super::super::super::System::Com::VARIANT, pszdefault: Param1) -> super::super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringWithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pszdefault: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(VariantToStringWithDefault(::core::mem::transmute(varin), pszdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, puiret: *mut u16) -> ::windows::core::HRESULT;
        }
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToUInt16(::core::mem::transmute(varin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToUInt16Array(::core::mem::transmute(var), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToUInt16ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16WithDefault(varin: *const super::super::super::System::Com::VARIANT, uidefault: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, uidefault: u16) -> u16;
        }
        ::core::mem::transmute(VariantToUInt16WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(uidefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pulret: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToUInt32(::core::mem::transmute(varin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToUInt32Array(::core::mem::transmute(var), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToUInt32ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32WithDefault(varin: *const super::super::super::System::Com::VARIANT, uldefault: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, uldefault: u32) -> u32;
        }
        ::core::mem::transmute(VariantToUInt32WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(uldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pullret: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        VariantToUInt64(::core::mem::transmute(varin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToUInt64Array(::core::mem::transmute(var), ::core::mem::transmute(prgn), ::core::mem::transmute(crgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::core::HRESULT;
        }
        VariantToUInt64ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64WithDefault(varin: *const super::super::super::System::Com::VARIANT, ulldefault: u64) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ulldefault: u64) -> u64;
        }
        ::core::mem::transmute(VariantToUInt64WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(ulldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn WinRTPropertyValueToPropVariant<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkpropertyvalue: Param0) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinRTPropertyValueToPropVariant(punkpropertyvalue: ::windows::core::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WinRTPropertyValueToPropVariant(punkpropertyvalue.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _PERSIST_SPROPSTORE_FLAGS(pub i32);
pub const FPSPS_DEFAULT: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(0i32);
pub const FPSPS_READONLY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(1i32);
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(2i32);
impl ::core::convert::From<i32> for _PERSIST_SPROPSTORE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _PERSIST_SPROPSTORE_FLAGS {
    type Abi = Self;
}
