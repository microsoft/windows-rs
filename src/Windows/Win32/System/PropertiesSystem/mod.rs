#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn ClearPropVariantArray(rgpropvar: *mut super::Com::StructuredStorage::PROPVARIANT, cvars: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearPropVariantArray(rgpropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, cvars: u32);
        }
        ::std::mem::transmute(ClearPropVariantArray(::std::mem::transmute(rgpropvar), ::std::mem::transmute(cvars)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn ClearVariantArray(pvars: *mut super::Com::VARIANT, cvars: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearVariantArray(pvars: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, cvars: u32);
        }
        ::std::mem::transmute(ClearVariantArray(::std::mem::transmute(pvars), ::std::mem::transmute(cvars)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DRAWPROGRESSFLAGS(pub i32);
pub const DPF_NONE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(0i32);
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(1i32);
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(2i32);
pub const DPF_ERROR: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(4i32);
pub const DPF_WARNING: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(8i32);
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(16i32);
impl ::std::convert::From<i32> for DRAWPROGRESSFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DRAWPROGRESSFLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for GETPROPERTYSTOREFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GETPROPERTYSTOREFLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICreateObject(::windows::runtime::IUnknown);
impl ICreateObject {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn CreateObject<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(&self, clsid: *const ::windows::runtime::GUID, punkouter: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(clsid), punkouter.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICreateObject {
    type Vtable = ICreateObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1964120402, 57552, 17381, [147, 128, 29, 128, 72, 58, 207, 114]);
}
impl ::std::convert::From<ICreateObject> for ::windows::runtime::IUnknown {
    fn from(value: ICreateObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICreateObject> for ::windows::runtime::IUnknown {
    fn from(value: &ICreateObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICreateObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICreateObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDelayedPropertyStoreFactory(::windows::runtime::IUnknown);
impl IDelayedPropertyStoreFactory {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStore<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(flags), punkfactory.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStoreForKeys<T: ::windows::runtime::Interface>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(rgkeys), ::std::mem::transmute(ckeys), ::std::mem::transmute(flags), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDelayedPropertyStore<T: ::windows::runtime::Interface>(&self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(flags), ::std::mem::transmute(dwstoreid), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDelayedPropertyStoreFactory {
    type Vtable = IDelayedPropertyStoreFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1087657855, 57911, 19419, [189, 105, 88, 240, 137, 67, 27, 106]);
}
impl ::std::convert::From<IDelayedPropertyStoreFactory> for ::windows::runtime::IUnknown {
    fn from(value: IDelayedPropertyStoreFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDelayedPropertyStoreFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IDelayedPropertyStoreFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDelayedPropertyStoreFactory> for IPropertyStoreFactory {
    fn from(value: IDelayedPropertyStoreFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDelayedPropertyStoreFactory> for IPropertyStoreFactory {
    fn from(value: &IDelayedPropertyStoreFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyStoreFactory> for IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyStoreFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyStoreFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyStoreFactory> for &IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyStoreFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyStoreFactory>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDelayedPropertyStoreFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: GETPROPERTYSTOREFLAGS, punkfactory: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInitializeWithFile(::windows::runtime::IUnknown);
impl IInitializeWithFile {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfilepath: Param0, grfmode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszfilepath.into_param().abi(), ::std::mem::transmute(grfmode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInitializeWithFile {
    type Vtable = IInitializeWithFile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3083945318, 1289, 19662, [167, 31, 10, 85, 66, 51, 189, 155]);
}
impl ::std::convert::From<IInitializeWithFile> for ::windows::runtime::IUnknown {
    fn from(value: IInitializeWithFile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInitializeWithFile> for ::windows::runtime::IUnknown {
    fn from(value: &IInitializeWithFile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInitializeWithFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInitializeWithFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithFile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfilepath: super::super::Foundation::PWSTR, grfmode: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInitializeWithStream(::windows::runtime::IUnknown);
impl IInitializeWithStream {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_System_Com`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(&self, pstream: Param0, grfmode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pstream.into_param().abi(), ::std::mem::transmute(grfmode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInitializeWithStream {
    type Vtable = IInitializeWithStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3089413277, 8876, 16737, [172, 138, 153, 22, 232, 250, 63, 127]);
}
impl ::std::convert::From<IInitializeWithStream> for ::windows::runtime::IUnknown {
    fn from(value: IInitializeWithStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInitializeWithStream> for ::windows::runtime::IUnknown {
    fn from(value: &IInitializeWithStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInitializeWithStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInitializeWithStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, grfmode: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INamedPropertyStore(::windows::runtime::IUnknown);
impl INamedPropertyStore {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetNamedValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszname.into_param().abi(), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetNamedValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszname.into_param().abi(), ::std::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetNameCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetNameAt(&self, iprop: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(iprop), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INamedPropertyStore {
    type Vtable = INamedPropertyStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1902136079, 38832, 18276, [133, 119, 47, 19, 233, 138, 20, 34]);
}
impl ::std::convert::From<INamedPropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: INamedPropertyStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INamedPropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: &INamedPropertyStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INamedPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INamedPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPropertyStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iprop: u32, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IObjectWithPropertyKey(::windows::runtime::IUnknown);
impl IObjectWithPropertyKey {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(key)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::runtime::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IObjectWithPropertyKey {
    type Vtable = IObjectWithPropertyKey_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4228685991, 49942, 20434, [144, 49, 62, 98, 142, 109, 79, 35]);
}
impl ::std::convert::From<IObjectWithPropertyKey> for ::windows::runtime::IUnknown {
    fn from(value: IObjectWithPropertyKey) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IObjectWithPropertyKey> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectWithPropertyKey) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectWithPropertyKey {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IObjectWithPropertyKey {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithPropertyKey_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPersistSerializedPropStorage(::windows::runtime::IUnknown);
impl IPersistSerializedPropStorage {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetPropertyStorage(&self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(psps), ::std::mem::transmute(cb)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppsps), ::std::mem::transmute(pcb)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPersistSerializedPropStorage {
    type Vtable = IPersistSerializedPropStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3810045271, 2720, 17679, [172, 165, 111, 171, 113, 3, 217, 23]);
}
impl ::std::convert::From<IPersistSerializedPropStorage> for ::windows::runtime::IUnknown {
    fn from(value: IPersistSerializedPropStorage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPersistSerializedPropStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistSerializedPropStorage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistSerializedPropStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPersistSerializedPropStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPersistSerializedPropStorage2(::windows::runtime::IUnknown);
impl IPersistSerializedPropStorage2 {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetPropertyStorage(&self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(psps), ::std::mem::transmute(cb)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppsps), ::std::mem::transmute(pcb)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStorageSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStorageBuffer(&self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(psps), ::std::mem::transmute(cb), ::std::mem::transmute(pcbwritten)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPersistSerializedPropStorage2 {
    type Vtable = IPersistSerializedPropStorage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2012215912, 20376, 17254, [186, 114, 87, 59, 61, 136, 5, 113]);
}
impl ::std::convert::From<IPersistSerializedPropStorage2> for ::windows::runtime::IUnknown {
    fn from(value: IPersistSerializedPropStorage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPersistSerializedPropStorage2> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistSerializedPropStorage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPersistSerializedPropStorage2> for IPersistSerializedPropStorage {
    fn from(value: IPersistSerializedPropStorage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPersistSerializedPropStorage2> for IPersistSerializedPropStorage {
    fn from(value: &IPersistSerializedPropStorage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersistSerializedPropStorage> for IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersistSerializedPropStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPersistSerializedPropStorage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersistSerializedPropStorage> for &IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersistSerializedPropStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPersistSerializedPropStorage>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcb: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyChange(::windows::runtime::IUnknown);
impl IPropertyChange {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(key)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::runtime::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn ApplyToPropVariant(&self, propvarin: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvarin), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyChange {
    type Vtable = IPropertyChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4179082378, 7098, 17528, [162, 69, 27, 222, 3, 235, 148, 49]);
}
impl ::std::convert::From<IPropertyChange> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyChange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyChange> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyChange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPropertyChange> for IObjectWithPropertyKey {
    fn from(value: IPropertyChange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyChange> for IObjectWithPropertyKey {
    fn from(value: &IPropertyChange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObjectWithPropertyKey> for IPropertyChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObjectWithPropertyKey> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IObjectWithPropertyKey>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObjectWithPropertyKey> for &IPropertyChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObjectWithPropertyKey> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IObjectWithPropertyKey>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ppropvarout: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyChangeArray(::windows::runtime::IUnknown);
impl IPropertyChangeArray {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAt<T: ::windows::runtime::Interface>(&self, iindex: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(iindex), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IPropertyChange>>(&self, iindex: u32, ppropchange: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(iindex), ppropchange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IPropertyChange>>(&self, ppropchange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ppropchange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn AppendOrReplace<'a, Param0: ::windows::runtime::IntoParam<'a, IPropertyChange>>(&self, ppropchange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ppropchange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn RemoveAt(&self, iindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(iindex)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn IsKeyInArray(&self, key: *const PROPERTYKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(key)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyChangeArray {
    type Vtable = IPropertyChangeArray_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(940530861, 7006, 17138, [128, 93, 99, 127, 211, 146, 211, 30]);
}
impl ::std::convert::From<IPropertyChangeArray> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyChangeArray) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyChangeArray> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyChangeArray) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyChangeArray {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyChangeArray {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangeArray_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcoperations: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iindex: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iindex: u32, ppropchange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropchange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropchange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyDescription(::windows::runtime::IUnknown);
impl IPropertyDescription {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::runtime::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyType(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetEditInvitation(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::runtime::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__: <PROPDESC_TYPE_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(mask), &mut result__).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetViewFlags(&self) -> ::windows::runtime::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__: <PROPDESC_VIEW_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDisplayType(&self) -> ::windows::runtime::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__: <PROPDESC_DISPLAYTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetColumnState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetGroupingRange(&self) -> ::windows::runtime::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__: <PROPDESC_GROUPING_RANGE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::runtime::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__: <PROPDESC_RELATIVEDESCRIPTION_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar1), ::std::mem::transmute(propvar2), ::std::mem::transmute(ppszdesc1), ::std::mem::transmute(ppszdesc2)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetSortDescription(&self) -> ::windows::runtime::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__: <PROPDESC_SORTDESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), fdescending.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAggregationType(&self) -> ::windows::runtime::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__: <PROPDESC_AGGREGATION_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_System_Search`*"]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::Search::CONDITION_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcontype), ::std::mem::transmute(popdefault)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetEnumTypeList<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar), ::std::mem::transmute(pdfflags), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyDescription {
    type Vtable = IPropertyDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1870255448, 16022, 17737, [161, 209, 125, 117, 210, 40, 136, 20]);
}
impl ::std::convert::From<IPropertyDescription> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyDescription) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyDescription> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyDescription) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvartype: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszinvite: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcxchars: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcsflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar1: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fdescending: super::super::Foundation::BOOL, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::Search::CONDITION_OPERATION) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyDescription2(::windows::runtime::IUnknown);
impl IPropertyDescription2 {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::runtime::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyType(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetEditInvitation(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::runtime::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__: <PROPDESC_TYPE_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(mask), &mut result__).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetViewFlags(&self) -> ::windows::runtime::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__: <PROPDESC_VIEW_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDisplayType(&self) -> ::windows::runtime::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__: <PROPDESC_DISPLAYTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetColumnState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetGroupingRange(&self) -> ::windows::runtime::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__: <PROPDESC_GROUPING_RANGE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::runtime::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__: <PROPDESC_RELATIVEDESCRIPTION_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar1), ::std::mem::transmute(propvar2), ::std::mem::transmute(ppszdesc1), ::std::mem::transmute(ppszdesc2)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetSortDescription(&self) -> ::windows::runtime::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__: <PROPDESC_SORTDESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), fdescending.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAggregationType(&self) -> ::windows::runtime::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__: <PROPDESC_AGGREGATION_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_System_Search`*"]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::Search::CONDITION_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcontype), ::std::mem::transmute(popdefault)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetEnumTypeList<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar), ::std::mem::transmute(pdfflags), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetImageReferenceForValue(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyDescription2 {
    type Vtable = IPropertyDescription2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1473441261, 20578, 16398, [177, 7, 93, 174, 121, 254, 87, 166]);
}
impl ::std::convert::From<IPropertyDescription2> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyDescription2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyDescription2> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyDescription2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyDescription2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyDescription2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPropertyDescription2> for IPropertyDescription {
    fn from(value: IPropertyDescription2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyDescription2> for IPropertyDescription {
    fn from(value: &IPropertyDescription2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyDescription> for IPropertyDescription2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyDescription> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyDescription>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyDescription> for &IPropertyDescription2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyDescription> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyDescription>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvartype: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszinvite: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcxchars: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcsflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar1: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fdescending: super::super::Foundation::BOOL, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::Search::CONDITION_OPERATION) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ppszimageres: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyDescriptionAliasInfo(::windows::runtime::IUnknown);
impl IPropertyDescriptionAliasInfo {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::runtime::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyType(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetEditInvitation(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::runtime::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__: <PROPDESC_TYPE_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(mask), &mut result__).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetViewFlags(&self) -> ::windows::runtime::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__: <PROPDESC_VIEW_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDisplayType(&self) -> ::windows::runtime::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__: <PROPDESC_DISPLAYTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetColumnState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetGroupingRange(&self) -> ::windows::runtime::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__: <PROPDESC_GROUPING_RANGE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::runtime::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__: <PROPDESC_RELATIVEDESCRIPTION_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar1), ::std::mem::transmute(propvar2), ::std::mem::transmute(ppszdesc1), ::std::mem::transmute(ppszdesc2)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetSortDescription(&self) -> ::windows::runtime::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__: <PROPDESC_SORTDESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), fdescending.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAggregationType(&self) -> ::windows::runtime::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__: <PROPDESC_AGGREGATION_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_System_Search`*"]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::Search::CONDITION_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcontype), ::std::mem::transmute(popdefault)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetEnumTypeList<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar), ::std::mem::transmute(pdfflags), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetSortByAlias<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAdditionalSortByAliases<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyDescriptionAliasInfo {
    type Vtable = IPropertyDescriptionAliasInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4134601980, 11001, 18173, [179, 45, 36, 60, 20, 4, 243, 209]);
}
impl ::std::convert::From<IPropertyDescriptionAliasInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyDescriptionAliasInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyDescriptionAliasInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyDescriptionAliasInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPropertyDescriptionAliasInfo> for IPropertyDescription {
    fn from(value: IPropertyDescriptionAliasInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyDescriptionAliasInfo> for IPropertyDescription {
    fn from(value: &IPropertyDescriptionAliasInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyDescription> for IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyDescription> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyDescription>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyDescription> for &IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyDescription> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyDescription>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionAliasInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvartype: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszinvite: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcxchars: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcsflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar1: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fdescending: super::super::Foundation::BOOL, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::Search::CONDITION_OPERATION) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyDescriptionList(::windows::runtime::IUnknown);
impl IPropertyDescriptionList {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAt<T: ::windows::runtime::Interface>(&self, ielem: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ielem), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyDescriptionList {
    type Vtable = IPropertyDescriptionList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(530563536, 50075, 19238, [129, 127, 1, 25, 103, 211, 68, 14]);
}
impl ::std::convert::From<IPropertyDescriptionList> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyDescriptionList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyDescriptionList> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyDescriptionList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyDescriptionList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyDescriptionList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcelem: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ielem: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyDescriptionRelatedPropertyInfo(::windows::runtime::IUnknown);
impl IPropertyDescriptionRelatedPropertyInfo {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::runtime::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyType(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetEditInvitation(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::runtime::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__: <PROPDESC_TYPE_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(mask), &mut result__).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetViewFlags(&self) -> ::windows::runtime::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__: <PROPDESC_VIEW_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDisplayType(&self) -> ::windows::runtime::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__: <PROPDESC_DISPLAYTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetColumnState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetGroupingRange(&self) -> ::windows::runtime::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__: <PROPDESC_GROUPING_RANGE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::runtime::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__: <PROPDESC_RELATIVEDESCRIPTION_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar1), ::std::mem::transmute(propvar2), ::std::mem::transmute(ppszdesc1), ::std::mem::transmute(ppszdesc2)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetSortDescription(&self) -> ::windows::runtime::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__: <PROPDESC_SORTDESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), fdescending.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAggregationType(&self) -> ::windows::runtime::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__: <PROPDESC_AGGREGATION_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_System_Search`*"]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::Search::CONDITION_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcontype), ::std::mem::transmute(popdefault)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetEnumTypeList<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar), ::std::mem::transmute(pdfflags), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetRelatedProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::runtime::Interface>(&self, pszrelationshipname: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), pszrelationshipname.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyDescriptionRelatedPropertyInfo {
    type Vtable = IPropertyDescriptionRelatedPropertyInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1349751796, 10813, 19040, [181, 158, 217, 199, 87, 22, 194, 221]);
}
impl ::std::convert::From<IPropertyDescriptionRelatedPropertyInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyDescriptionRelatedPropertyInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyDescriptionRelatedPropertyInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyDescriptionRelatedPropertyInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPropertyDescriptionRelatedPropertyInfo> for IPropertyDescription {
    fn from(value: IPropertyDescriptionRelatedPropertyInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyDescriptionRelatedPropertyInfo> for IPropertyDescription {
    fn from(value: &IPropertyDescriptionRelatedPropertyInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyDescription> for IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyDescription> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyDescription>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyDescription> for &IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyDescription> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyDescription>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionRelatedPropertyInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvartype: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszinvite: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcxchars: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcsflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar1: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fdescending: super::super::Foundation::BOOL, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::Search::CONDITION_OPERATION) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszrelationshipname: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyDescriptionSearchInfo(::windows::runtime::IUnknown);
impl IPropertyDescriptionSearchInfo {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyKey(&self) -> ::windows::runtime::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyType(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetEditInvitation(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::runtime::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__: <PROPDESC_TYPE_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(mask), &mut result__).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetViewFlags(&self) -> ::windows::runtime::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__: <PROPDESC_VIEW_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDisplayType(&self) -> ::windows::runtime::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__: <PROPDESC_DISPLAYTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetColumnState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetGroupingRange(&self) -> ::windows::runtime::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__: <PROPDESC_GROUPING_RANGE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::runtime::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__: <PROPDESC_RELATIVEDESCRIPTION_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar1), ::std::mem::transmute(propvar2), ::std::mem::transmute(ppszdesc1), ::std::mem::transmute(ppszdesc2)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetSortDescription(&self) -> ::windows::runtime::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__: <PROPDESC_SORTDESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), fdescending.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAggregationType(&self) -> ::windows::runtime::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__: <PROPDESC_AGGREGATION_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_System_Search`*"]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::Search::CONDITION_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcontype), ::std::mem::transmute(popdefault)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetEnumTypeList<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar), ::std::mem::transmute(pdfflags), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetSearchInfoFlags(&self) -> ::windows::runtime::Result<PROPDESC_SEARCHINFO_FLAGS> {
        let mut result__: <PROPDESC_SEARCHINFO_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_SEARCHINFO_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetColumnIndexType(&self) -> ::windows::runtime::Result<PROPDESC_COLUMNINDEX_TYPE> {
        let mut result__: <PROPDESC_COLUMNINDEX_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPDESC_COLUMNINDEX_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetProjectionString(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetMaxSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyDescriptionSearchInfo {
    type Vtable = IPropertyDescriptionSearchInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(126849469, 10658, 17423, [146, 78, 70, 162, 145, 82, 69, 32]);
}
impl ::std::convert::From<IPropertyDescriptionSearchInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyDescriptionSearchInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyDescriptionSearchInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyDescriptionSearchInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPropertyDescriptionSearchInfo> for IPropertyDescription {
    fn from(value: IPropertyDescriptionSearchInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyDescriptionSearchInfo> for IPropertyDescription {
    fn from(value: &IPropertyDescriptionSearchInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyDescription> for IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyDescription> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyDescription>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyDescription> for &IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyDescription> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyDescription>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionSearchInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvartype: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszinvite: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcxchars: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcsflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar1: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fdescending: super::super::Foundation::BOOL, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::Search::CONDITION_OPERATION) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszprojection: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbmaxsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyEnumType(::windows::runtime::IUnknown);
impl IPropertyEnumType {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetEnumType(&self) -> ::windows::runtime::Result<PROPENUMTYPE> {
        let mut result__: <PROPENUMTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPENUMTYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetValue(&self) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetRangeMinValue(&self) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetRangeSetValue(&self) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayText(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyEnumType {
    type Vtable = IPropertyEnumType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(300022777, 11606, 19051, [141, 179, 124, 209, 147, 164, 113, 242]);
}
impl ::std::convert::From<IPropertyEnumType> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyEnumType) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyEnumType> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyEnumType) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyEnumType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyEnumType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penumtype: *mut PROPENUMTYPE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvarmin: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvarset: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszdisplay: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyEnumType2(::windows::runtime::IUnknown);
impl IPropertyEnumType2 {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetEnumType(&self) -> ::windows::runtime::Result<PROPENUMTYPE> {
        let mut result__: <PROPENUMTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPENUMTYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetValue(&self) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetRangeMinValue(&self) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetRangeSetValue(&self) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayText(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetImageReference(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyEnumType2 {
    type Vtable = IPropertyEnumType2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2607678748, 24029, 17185, [144, 112, 254, 42, 203, 85, 231, 148]);
}
impl ::std::convert::From<IPropertyEnumType2> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyEnumType2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyEnumType2> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyEnumType2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyEnumType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyEnumType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPropertyEnumType2> for IPropertyEnumType {
    fn from(value: IPropertyEnumType2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyEnumType2> for IPropertyEnumType {
    fn from(value: &IPropertyEnumType2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyEnumType> for IPropertyEnumType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyEnumType> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyEnumType>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyEnumType> for &IPropertyEnumType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyEnumType> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyEnumType>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penumtype: *mut PROPENUMTYPE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvarmin: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropvarset: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszdisplay: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszimageres: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyEnumTypeList(::windows::runtime::IUnknown);
impl IPropertyEnumTypeList {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAt<T: ::windows::runtime::Interface>(&self, itype: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itype), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetConditionAt<T: ::windows::runtime::Interface>(&self, nindex: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(nindex), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn FindMatchingIndex(&self, propvarcmp: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(propvarcmp), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyEnumTypeList {
    type Vtable = IPropertyEnumTypeList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2845049076, 15748, 17751, [148, 186, 18, 66, 251, 44, 201, 166]);
}
impl ::std::convert::From<IPropertyEnumTypeList> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyEnumTypeList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyEnumTypeList> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyEnumTypeList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyEnumTypeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyEnumTypeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumTypeList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctypes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itype: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propvarcmp: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pnindex: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyStore(::windows::runtime::IUnknown);
impl IPropertyStore {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows::runtime::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(iprop), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyStore {
    type Vtable = IPropertyStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2288881387, 36082, 17478, [141, 2, 205, 186, 29, 189, 207, 153]);
}
impl ::std::convert::From<IPropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cprops: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY, pv: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyStoreCache(::windows::runtime::IUnknown);
impl IPropertyStoreCache {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows::runtime::Result<PROPERTYKEY> {
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(iprop), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetState(&self, key: *const PROPERTYKEY) -> ::windows::runtime::Result<PSC_STATE> {
        let mut result__: <PSC_STATE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), &mut result__).from_abi::<PSC_STATE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(ppropvar), ::std::mem::transmute(pstate)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetState(&self, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(state)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(ppropvar), ::std::mem::transmute(state)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyStoreCache {
    type Vtable = IPropertyStoreCache_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(806815085, 39569, 20112, [147, 125, 116, 108, 114, 171, 191, 79]);
}
impl ::std::convert::From<IPropertyStoreCache> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyStoreCache) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyStoreCache> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyStoreCache) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyStoreCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyStoreCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPropertyStoreCache> for IPropertyStore {
    fn from(value: IPropertyStoreCache) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyStoreCache> for IPropertyStore {
    fn from(value: &IPropertyStoreCache) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyStore> for IPropertyStoreCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyStore> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyStore>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyStore> for &IPropertyStoreCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyStore> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPropertyStore>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCache_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cprops: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY, pv: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pstate: *mut PSC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY, ppropvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, state: PSC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyStoreCapabilities(::windows::runtime::IUnknown);
impl IPropertyStoreCapabilities {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn IsPropertyWritable(&self, key: *const PROPERTYKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(key)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyStoreCapabilities {
    type Vtable = IPropertyStoreCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3370308966, 6254, 19785, [191, 65, 105, 9, 234, 213, 106, 204]);
}
impl ::std::convert::From<IPropertyStoreCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyStoreCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyStoreCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyStoreCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyStoreCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyStoreCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyStoreFactory(::windows::runtime::IUnknown);
impl IPropertyStoreFactory {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStore<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(flags), punkfactory.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyStoreForKeys<T: ::windows::runtime::Interface>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(rgkeys), ::std::mem::transmute(ckeys), ::std::mem::transmute(flags), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyStoreFactory {
    type Vtable = IPropertyStoreFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3155233645, 22504, 16712, [169, 198, 145, 1, 90, 178, 243, 165]);
}
impl ::std::convert::From<IPropertyStoreFactory> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyStoreFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyStoreFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyStoreFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyStoreFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyStoreFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: GETPROPERTYSTOREFLAGS, punkfactory: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertySystem(::windows::runtime::IUnknown);
impl IPropertySystem {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetPropertyDescription<T: ::windows::runtime::Interface>(&self, propkey: *const PROPERTYKEY) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(propkey), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyDescriptionByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::runtime::Interface>(&self, pszcanonicalname: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszcanonicalname.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyDescriptionListFromString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::runtime::Interface>(&self, pszproplist: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszproplist.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn EnumeratePropertyDescriptions<T: ::windows::runtime::Interface>(&self, filteron: PROPDESC_ENUMFILTER) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(filteron), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn FormatForDisplay(&self, key: *const PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(propvar), ::std::mem::transmute(pdff), ::std::mem::transmute(psztext), ::std::mem::transmute(cchtext)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn FormatForDisplayAlloc(&self, key: *const PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(propvar), ::std::mem::transmute(pdff), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn RegisterPropertySchema<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pszpath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn UnregisterPropertySchema<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn RefreshPropertySchema(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertySystem {
    type Vtable = IPropertySystem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3396488842, 50150, 17451, [136, 164, 111, 176, 219, 128, 53, 163]);
}
impl ::std::convert::From<IPropertySystem> for ::windows::runtime::IUnknown {
    fn from(value: IPropertySystem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertySystem> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertySystem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertySystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertySystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propkey: *const PROPERTYKEY, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcanonicalname: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszproplist: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pdff: PROPDESC_FORMAT_FLAGS, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const PROPERTYKEY, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertySystemChangeNotify(::windows::runtime::IUnknown);
impl IPropertySystemChangeNotify {
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SchemaRefreshed(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertySystemChangeNotify {
    type Vtable = IPropertySystemChangeNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4204093401, 14526, 18553, [166, 206, 130, 76, 245, 45, 96, 159]);
}
impl ::std::convert::From<IPropertySystemChangeNotify> for ::windows::runtime::IUnknown {
    fn from(value: IPropertySystemChangeNotify) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertySystemChangeNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertySystemChangeNotify) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertySystemChangeNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertySystemChangeNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystemChangeNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyUI(::windows::runtime::IUnknown);
impl IPropertyUI {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn ParsePropertyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, pfmtid: *mut ::windows::runtime::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszname.into_param().abi(), ::std::mem::transmute(pfmtid), ::std::mem::transmute(ppid), ::std::mem::transmute(pcheaten)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetCannonicalName(&self, fmtid: *const ::windows::runtime::GUID, pid: u32, pwsztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(fmtid), ::std::mem::transmute(pid), ::std::mem::transmute(pwsztext), ::std::mem::transmute(cchtext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self, fmtid: *const ::windows::runtime::GUID, pid: u32, flags: u32, pwsztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(fmtid), ::std::mem::transmute(pid), ::std::mem::transmute(flags), ::std::mem::transmute(pwsztext), ::std::mem::transmute(cchtext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyDescription(&self, fmtid: *const ::windows::runtime::GUID, pid: u32, pwsztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(fmtid), ::std::mem::transmute(pid), ::std::mem::transmute(pwsztext), ::std::mem::transmute(cchtext)).ok()
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetDefaultWidth(&self, fmtid: *const ::windows::runtime::GUID, pid: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(fmtid), ::std::mem::transmute(pid), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetFlags(&self, fmtid: *const ::windows::runtime::GUID, pid: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(fmtid), ::std::mem::transmute(pid), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn FormatForDisplay(&self, fmtid: *const ::windows::runtime::GUID, pid: u32, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, puiff: u32, pwsztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(fmtid), ::std::mem::transmute(pid), ::std::mem::transmute(ppropvar), ::std::mem::transmute(puiff), ::std::mem::transmute(pwsztext), ::std::mem::transmute(cchtext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
    pub unsafe fn GetHelpInfo(&self, fmtid: *const ::windows::runtime::GUID, pid: u32, pwszhelpfile: super::super::Foundation::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(fmtid), ::std::mem::transmute(pid), ::std::mem::transmute(pwszhelpfile), ::std::mem::transmute(cch), ::std::mem::transmute(puhelpid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyUI {
    type Vtable = IPropertyUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1970961823, 37274, 16664, [153, 215, 219, 178, 8, 200, 204, 102]);
}
impl ::std::convert::From<IPropertyUI> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyUI) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyUI> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyUI) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyUI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, pfmtid: *mut ::windows::runtime::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pid: u32, pwsztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pid: u32, flags: u32, pwsztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pid: u32, pwsztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pid: u32, pcxchars: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pid: u32, pflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pid: u32, ppropvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, puiff: u32, pwsztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pid: u32, pwszhelpfile: super::super::Foundation::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const InMemoryPropertyStore: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2583879698, 25347, 19998, [185, 161, 99, 15, 128, 37, 146, 197]);
pub const InMemoryPropertyStoreMarshalByValue: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3570011693, 28071, 19317, [169, 124, 95, 48, 111, 14, 174, 220]);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromBooleanVector(prgf: *const super::super::Foundation::BOOL, celems: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromBooleanVector(prgf: *const super::super::Foundation::BOOL, celems: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromBooleanVector(::std::mem::transmute(prgf), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromBuffer(pv: *const ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromBuffer(pv: *const ::std::ffi::c_void, cb: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromBuffer(::std::mem::transmute(pv), ::std::mem::transmute(cb), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromCLSID(clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromCLSID(clsid: *const ::windows::runtime::GUID, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromCLSID(::std::mem::transmute(clsid), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromDoubleVector(prgn: *const f64, celems: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromDoubleVector(prgn: *const f64, celems: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromDoubleVector(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromFileTime(pftin: *const super::super::Foundation::FILETIME) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromFileTime(pftin: *const super::super::Foundation::FILETIME, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromFileTime(::std::mem::transmute(pftin), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromFileTimeVector(prgft: *const super::super::Foundation::FILETIME, celems: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromFileTimeVector(prgft: *const super::super::Foundation::FILETIME, celems: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromFileTimeVector(::std::mem::transmute(prgft), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromGUIDAsString(guid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromGUIDAsString(guid: *const ::windows::runtime::GUID, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromGUIDAsString(::std::mem::transmute(guid), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromInt16Vector(prgn: *const i16, celems: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromInt16Vector(prgn: *const i16, celems: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromInt16Vector(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromInt32Vector(prgn: *const i32, celems: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromInt32Vector(prgn: *const i32, celems: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromInt32Vector(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromInt64Vector(prgn: *const i64, celems: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromInt64Vector(prgn: *const i64, celems: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromInt64Vector(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromPropVariantVectorElem(propvarin: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromPropVariantVectorElem(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromPropVariantVectorElem(::std::mem::transmute(propvarin), ::std::mem::transmute(ielem), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hinst: Param0, id: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromResource(hinst: super::super::Foundation::HINSTANCE, id: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromResource(hinst.into_param().abi(), ::std::mem::transmute(id), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Shell"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_UI_Shell`*"]
#[inline]
pub unsafe fn InitPropVariantFromStrRet(pstrret: *mut super::super::UI::Shell::STRRET, pidl: *const super::super::UI::Shell::ITEMIDLIST, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromStrRet(pstrret: *mut super::super::UI::Shell::STRRET, pidl: *const super::super::UI::Shell::ITEMIDLIST, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        InitPropVariantFromStrRet(::std::mem::transmute(pstrret), ::std::mem::transmute(pidl), ::std::mem::transmute(ppropvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromStringAsVector<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(psz: Param0) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromStringAsVector(psz: super::super::Foundation::PWSTR, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromStringAsVector(psz.into_param().abi(), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromStringVector(prgsz: *const super::super::Foundation::PWSTR, celems: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromStringVector(prgsz: *const super::super::Foundation::PWSTR, celems: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromStringVector(::std::mem::transmute(prgsz), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromUInt16Vector(prgn: *const u16, celems: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromUInt16Vector(prgn: *const u16, celems: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromUInt16Vector(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromUInt32Vector(prgn: *const u32, celems: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromUInt32Vector(prgn: *const u32, celems: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromUInt32Vector(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantFromUInt64Vector(prgn: *const u64, celems: u32) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromUInt64Vector(prgn: *const u64, celems: u32, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantFromUInt64Vector(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitPropVariantVectorFromPropVariant(propvarsingle: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantVectorFromPropVariant(propvarsingle: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ppropvarvector: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitPropVariantVectorFromPropVariant(::std::mem::transmute(propvarsingle), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromBooleanArray(prgf: *const super::super::Foundation::BOOL, celems: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromBooleanArray(prgf: *const super::super::Foundation::BOOL, celems: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromBooleanArray(::std::mem::transmute(prgf), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromBuffer(pv: *const ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromBuffer(pv: *const ::std::ffi::c_void, cb: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromBuffer(::std::mem::transmute(pv), ::std::mem::transmute(cb), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromDoubleArray(prgn: *const f64, celems: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromDoubleArray(prgn: *const f64, celems: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromDoubleArray(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromFileTime(pft: *const super::super::Foundation::FILETIME) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromFileTime(pft: *const super::super::Foundation::FILETIME, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromFileTime(::std::mem::transmute(pft), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromFileTimeArray(prgft: *const super::super::Foundation::FILETIME, celems: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromFileTimeArray(prgft: *const super::super::Foundation::FILETIME, celems: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromFileTimeArray(::std::mem::transmute(prgft), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromGUIDAsString(guid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromGUIDAsString(guid: *const ::windows::runtime::GUID, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromGUIDAsString(::std::mem::transmute(guid), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromInt16Array(prgn: *const i16, celems: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromInt16Array(prgn: *const i16, celems: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromInt16Array(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromInt32Array(prgn: *const i32, celems: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromInt32Array(prgn: *const i32, celems: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromInt32Array(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromInt64Array(prgn: *const i64, celems: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromInt64Array(prgn: *const i64, celems: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromInt64Array(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hinst: Param0, id: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromResource(hinst: super::super::Foundation::HINSTANCE, id: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromResource(hinst.into_param().abi(), ::std::mem::transmute(id), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Shell"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`, `Win32_UI_Shell`*"]
#[inline]
pub unsafe fn InitVariantFromStrRet(pstrret: *const super::super::UI::Shell::STRRET, pidl: *const super::super::UI::Shell::ITEMIDLIST) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromStrRet(pstrret: *const super::super::UI::Shell::STRRET, pidl: *const super::super::UI::Shell::ITEMIDLIST, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromStrRet(::std::mem::transmute(pstrret), ::std::mem::transmute(pidl), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromStringArray(prgsz: *const super::super::Foundation::PWSTR, celems: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromStringArray(prgsz: *const super::super::Foundation::PWSTR, celems: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromStringArray(::std::mem::transmute(prgsz), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromUInt16Array(prgn: *const u16, celems: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromUInt16Array(prgn: *const u16, celems: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromUInt16Array(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromUInt32Array(prgn: *const u32, celems: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromUInt32Array(prgn: *const u32, celems: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromUInt32Array(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromUInt64Array(prgn: *const u64, celems: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromUInt64Array(prgn: *const u64, celems: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromUInt64Array(::std::mem::transmute(prgn), ::std::mem::transmute(celems), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn InitVariantFromVariantArrayElem(varin: *const super::Com::VARIANT, ielem: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromVariantArrayElem(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ielem: u32, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        InitVariantFromVariantArrayElem(::std::mem::transmute(varin), ::std::mem::transmute(ielem), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PDOPSTATUS(pub i32);
pub const PDOPS_RUNNING: PDOPSTATUS = PDOPSTATUS(1i32);
pub const PDOPS_PAUSED: PDOPSTATUS = PDOPSTATUS(2i32);
pub const PDOPS_CANCELLED: PDOPSTATUS = PDOPSTATUS(3i32);
pub const PDOPS_STOPPED: PDOPSTATUS = PDOPSTATUS(4i32);
pub const PDOPS_ERRORS: PDOPSTATUS = PDOPSTATUS(5i32);
impl ::std::convert::From<i32> for PDOPSTATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PDOPSTATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PKA_FLAGS(pub i32);
pub const PKA_SET: PKA_FLAGS = PKA_FLAGS(0i32);
pub const PKA_APPEND: PKA_FLAGS = PKA_FLAGS(1i32);
pub const PKA_DELETE: PKA_FLAGS = PKA_FLAGS(2i32);
impl ::std::convert::From<i32> for PKA_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PKA_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
pub const PKEY_PIDSTR_MAX: u32 = 10u32;
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PLACEHOLDER_STATES(pub i32);
pub const PS_NONE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(0i32);
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: PLACEHOLDER_STATES = PLACEHOLDER_STATES(1i32);
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(2i32);
pub const PS_CREATE_FILE_ACCESSIBLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(4i32);
pub const PS_CLOUDFILE_PLACEHOLDER: PLACEHOLDER_STATES = PLACEHOLDER_STATES(8i32);
pub const PS_DEFAULT: PLACEHOLDER_STATES = PLACEHOLDER_STATES(7i32);
pub const PS_ALL: PLACEHOLDER_STATES = PLACEHOLDER_STATES(15i32);
impl ::std::convert::From<i32> for PLACEHOLDER_STATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PLACEHOLDER_STATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PROPDESC_AGGREGATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_AGGREGATION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_COLUMNINDEX_TYPE(pub i32);
pub const PDCIT_NONE: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(0i32);
pub const PDCIT_ONDISK: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(1i32);
pub const PDCIT_INMEMORY: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(2i32);
pub const PDCIT_ONDEMAND: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(3i32);
pub const PDCIT_ONDISKALL: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(4i32);
pub const PDCIT_ONDISKVECTOR: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(5i32);
impl ::std::convert::From<i32> for PROPDESC_COLUMNINDEX_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_COLUMNINDEX_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_CONDITION_TYPE(pub i32);
pub const PDCOT_NONE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(0i32);
pub const PDCOT_STRING: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(1i32);
pub const PDCOT_SIZE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(2i32);
pub const PDCOT_DATETIME: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(3i32);
pub const PDCOT_BOOLEAN: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(4i32);
pub const PDCOT_NUMBER: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(5i32);
impl ::std::convert::From<i32> for PROPDESC_CONDITION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_CONDITION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_DISPLAYTYPE(pub i32);
pub const PDDT_STRING: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(0i32);
pub const PDDT_NUMBER: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(1i32);
pub const PDDT_BOOLEAN: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(2i32);
pub const PDDT_DATETIME: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(3i32);
pub const PDDT_ENUMERATED: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(4i32);
impl ::std::convert::From<i32> for PROPDESC_DISPLAYTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_DISPLAYTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_ENUMFILTER(pub i32);
pub const PDEF_ALL: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(0i32);
pub const PDEF_SYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(1i32);
pub const PDEF_NONSYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(2i32);
pub const PDEF_VIEWABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(3i32);
pub const PDEF_QUERYABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(4i32);
pub const PDEF_INFULLTEXTQUERY: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(5i32);
pub const PDEF_COLUMN: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(6i32);
impl ::std::convert::From<i32> for PROPDESC_ENUMFILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_ENUMFILTER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PROPDESC_FORMAT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_FORMAT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_GROUPING_RANGE(pub i32);
pub const PDGR_DISCRETE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(0i32);
pub const PDGR_ALPHANUMERIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(1i32);
pub const PDGR_SIZE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(2i32);
pub const PDGR_DYNAMIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(3i32);
pub const PDGR_DATE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(4i32);
pub const PDGR_PERCENT: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(5i32);
pub const PDGR_ENUMERATED: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(6i32);
impl ::std::convert::From<i32> for PROPDESC_GROUPING_RANGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_GROUPING_RANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_SEARCHINFO_FLAGS(pub i32);
pub const PDSIF_DEFAULT: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(0i32);
pub const PDSIF_ININVERTEDINDEX: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(1i32);
pub const PDSIF_ISCOLUMN: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(2i32);
pub const PDSIF_ISCOLUMNSPARSE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(4i32);
pub const PDSIF_ALWAYSINCLUDE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(8i32);
pub const PDSIF_USEFORTYPEAHEAD: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(16i32);
impl ::std::convert::From<i32> for PROPDESC_SEARCHINFO_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_SEARCHINFO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPDESC_SORTDESCRIPTION(pub i32);
pub const PDSD_GENERAL: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(0i32);
pub const PDSD_A_Z: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(1i32);
pub const PDSD_LOWEST_HIGHEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(2i32);
pub const PDSD_SMALLEST_BIGGEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(3i32);
pub const PDSD_OLDEST_NEWEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(4i32);
impl ::std::convert::From<i32> for PROPDESC_SORTDESCRIPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_SORTDESCRIPTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PROPDESC_TYPE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_TYPE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PROPDESC_VIEW_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPDESC_VIEW_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPENUMTYPE(pub i32);
pub const PET_DISCRETEVALUE: PROPENUMTYPE = PROPENUMTYPE(0i32);
pub const PET_RANGEDVALUE: PROPENUMTYPE = PROPENUMTYPE(1i32);
pub const PET_DEFAULTVALUE: PROPENUMTYPE = PROPENUMTYPE(2i32);
pub const PET_ENDRANGE: PROPENUMTYPE = PROPENUMTYPE(3i32);
impl ::std::convert::From<i32> for PROPENUMTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPENUMTYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
pub struct PROPERTYKEY {
    pub fmtid: ::windows::runtime::GUID,
    pub pid: u32,
}
impl PROPERTYKEY {}
impl ::std::default::Default for PROPERTYKEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROPERTYKEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROPERTYKEY").field("fmtid", &self.fmtid).field("pid", &self.pid).finish()
    }
}
impl ::std::cmp::PartialEq for PROPERTYKEY {
    fn eq(&self, other: &Self) -> bool {
        self.fmtid == other.fmtid && self.pid == other.pid
    }
}
impl ::std::cmp::Eq for PROPERTYKEY {}
unsafe impl ::windows::runtime::Abi for PROPERTYKEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [super::super::Foundation::CHAR; 30],
    pub achCmdLine: [super::super::Foundation::CHAR; 128],
    pub achWorkDir: [super::super::Foundation::CHAR; 64],
    pub wHotKey: u16,
    pub achIconFile: [super::super::Foundation::CHAR; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [super::super::Foundation::CHAR; 80],
    pub achPIFFile: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl PROPPRG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROPPRG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROPPRG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROPPRG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROPPRG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPVAR_CHANGE_FLAGS(pub i32);
pub const PVCHF_DEFAULT: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(0i32);
pub const PVCHF_NOVALUEPROP: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(1i32);
pub const PVCHF_ALPHABOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(2i32);
pub const PVCHF_NOUSEROVERRIDE: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(4i32);
pub const PVCHF_LOCALBOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(8i32);
pub const PVCHF_NOHEXSTRING: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(16i32);
impl ::std::convert::From<i32> for PROPVAR_CHANGE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPVAR_CHANGE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPVAR_COMPARE_FLAGS(pub i32);
pub const PVCF_DEFAULT: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(0i32);
pub const PVCF_TREATEMPTYASGREATERTHAN: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(1i32);
pub const PVCF_USESTRCMP: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(2i32);
pub const PVCF_USESTRCMPC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(4i32);
pub const PVCF_USESTRCMPI: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(8i32);
pub const PVCF_USESTRCMPIC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(16i32);
pub const PVCF_DIGITSASNUMBERS_CASESENSITIVE: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(32i32);
impl ::std::convert::From<i32> for PROPVAR_COMPARE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPVAR_COMPARE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPVAR_COMPARE_UNIT(pub i32);
pub const PVCU_DEFAULT: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(0i32);
pub const PVCU_SECOND: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(1i32);
pub const PVCU_MINUTE: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(2i32);
pub const PVCU_HOUR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(3i32);
pub const PVCU_DAY: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(4i32);
pub const PVCU_MONTH: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(5i32);
pub const PVCU_YEAR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(6i32);
impl ::std::convert::From<i32> for PROPVAR_COMPARE_UNIT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPVAR_COMPARE_UNIT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSC_STATE(pub i32);
pub const PSC_NORMAL: PSC_STATE = PSC_STATE(0i32);
pub const PSC_NOTINSOURCE: PSC_STATE = PSC_STATE(1i32);
pub const PSC_DIRTY: PSC_STATE = PSC_STATE(2i32);
pub const PSC_READONLY: PSC_STATE = PSC_STATE(3i32);
impl ::std::convert::From<i32> for PSC_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSC_STATE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        PSCoerceToCanonicalValue(::std::mem::transmute(key), ::std::mem::transmute(ppropvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSCreateAdapterFromPropertyStore<'a, Param0: ::windows::runtime::IntoParam<'a, IPropertyStore>>(pps: Param0, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateAdapterFromPropertyStore(pps: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSCreateAdapterFromPropertyStore(pps.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSCreateDelayedMultiplexPropertyStore<'a, Param1: ::windows::runtime::IntoParam<'a, IDelayedPropertyStoreFactory>>(flags: GETPROPERTYSTOREFLAGS, pdpsf: Param1, rgstoreids: *const u32, cstores: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateDelayedMultiplexPropertyStore(flags: GETPROPERTYSTOREFLAGS, pdpsf: ::windows::runtime::RawPtr, rgstoreids: *const u32, cstores: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSCreateDelayedMultiplexPropertyStore(::std::mem::transmute(flags), pdpsf.into_param().abi(), ::std::mem::transmute(rgstoreids), ::std::mem::transmute(cstores), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSCreateMemoryPropertyStore(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateMemoryPropertyStore(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSCreateMemoryPropertyStore(::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSCreateMultiplexPropertyStore(prgpunkstores: *const ::std::option::Option<::windows::runtime::IUnknown>, cstores: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateMultiplexPropertyStore(prgpunkstores: *const ::windows::runtime::RawPtr, cstores: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSCreateMultiplexPropertyStore(::std::mem::transmute(prgpunkstores), ::std::mem::transmute(cstores), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSCreatePropertyChangeArray(rgpropkey: *const PROPERTYKEY, rgflags: *const PKA_FLAGS, rgpropvar: *const super::Com::StructuredStorage::PROPVARIANT, cchanges: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreatePropertyChangeArray(rgpropkey: *const PROPERTYKEY, rgflags: *const PKA_FLAGS, rgpropvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, cchanges: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSCreatePropertyChangeArray(::std::mem::transmute(rgpropkey), ::std::mem::transmute(rgflags), ::std::mem::transmute(rgpropvar), ::std::mem::transmute(cchanges), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromObject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0, grfmode: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreatePropertyStoreFromObject(punk: ::windows::runtime::RawPtr, grfmode: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSCreatePropertyStoreFromObject(punk.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromPropertySetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertySetStorage>>(ppss: Param0, grfmode: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreatePropertyStoreFromPropertySetStorage(ppss: ::windows::runtime::RawPtr, grfmode: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSCreatePropertyStoreFromPropertySetStorage(ppss.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSCreateSimplePropertyChange(::std::mem::transmute(flags), ::std::mem::transmute(key), ::std::mem::transmute(propvar), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSEnumeratePropertyDescriptions(::std::mem::transmute(filteron), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::HRESULT;
        }
        PSFormatForDisplay(::std::mem::transmute(propkey), ::std::mem::transmute(propvar), ::std::mem::transmute(pdfflags), ::std::mem::transmute(pwsztext), ::std::mem::transmute(cchtext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSFormatForDisplayAlloc(::std::mem::transmute(key), ::std::mem::transmute(propvar), ::std::mem::transmute(pdff), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSFormatPropertyValue<'a, Param0: ::windows::runtime::IntoParam<'a, IPropertyStore>, Param1: ::windows::runtime::IntoParam<'a, IPropertyDescription>>(pps: Param0, ppd: Param1, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSFormatPropertyValue(pps: ::windows::runtime::RawPtr, ppd: ::windows::runtime::RawPtr, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSFormatPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), ::std::mem::transmute(pdff), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ppszimageres: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSGetImageReferenceForValue(::std::mem::transmute(propkey), ::std::mem::transmute(propvar), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSGetItemPropertyHandler<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(punkitem: Param0, freadwrite: Param1, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetItemPropertyHandler(punkitem: ::windows::runtime::RawPtr, freadwrite: super::super::Foundation::BOOL, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSGetItemPropertyHandler(punkitem.into_param().abi(), freadwrite.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSGetItemPropertyHandlerWithCreateObject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punkitem: Param0, freadwrite: Param1, punkcreateobject: Param2, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetItemPropertyHandlerWithCreateObject(punkitem: ::windows::runtime::RawPtr, freadwrite: super::super::Foundation::BOOL, punkcreateobject: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSGetItemPropertyHandlerWithCreateObject(punkitem.into_param().abi(), freadwrite.into_param().abi(), punkcreateobject.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY, ppszcanonicalname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSGetNameFromPropertyKey(::std::mem::transmute(propkey), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSGetNamedPropertyFromPropertyStorage<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: Param2) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetNamedPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: super::super::Foundation::PWSTR, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSGetNamedPropertyFromPropertyStorage(::std::mem::transmute(psps), ::std::mem::transmute(cb), pszname.into_param().abi(), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSGetPropertyDescription(::std::mem::transmute(propkey), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSGetPropertyDescriptionByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszcanonicalname: Param0, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyDescriptionByName(pszcanonicalname: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSGetPropertyDescriptionByName(pszcanonicalname.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSGetPropertyDescriptionListFromString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszproplist: Param0, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyDescriptionListFromString(pszproplist: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSGetPropertyDescriptionListFromString(pszproplist.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSGetPropertyFromPropertyStorage(::std::mem::transmute(psps), ::std::mem::transmute(cb), ::std::mem::transmute(rpkey), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSGetPropertyKeyFromName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszname: Param0) -> ::windows::runtime::Result<PROPERTYKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyKeyFromName(pszname: super::super::Foundation::PWSTR, ppropkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSGetPropertyKeyFromName(pszname.into_param().abi(), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSGetPropertySystem(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertySystem(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSGetPropertySystem(::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSGetPropertyValue<'a, Param0: ::windows::runtime::IntoParam<'a, IPropertyStore>, Param1: ::windows::runtime::IntoParam<'a, IPropertyDescription>>(pps: Param0, ppd: Param1) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyValue(pps: ::windows::runtime::RawPtr, ppd: ::windows::runtime::RawPtr, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSGetPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSLookupPropertyHandlerCLSID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszfilepath: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSLookupPropertyHandlerCLSID(pszfilepath: super::super::Foundation::PWSTR, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSLookupPropertyHandlerCLSID(pszfilepath.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_Delete<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_Delete(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_Delete(propbag.into_param().abi(), propname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadBOOL<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadBOOL(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadBOOL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadBSTR<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadBSTR(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadBSTR(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadDWORD<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadDWORD(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadDWORD(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadGUID<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadGUID(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadGUID(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadInt<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadInt(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadInt(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadLONG<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadLONG(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadLONG(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTL<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<super::super::Foundation::POINTL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadPOINTL(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut super::super::Foundation::POINTL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::POINTL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadPOINTL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::POINTL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTS<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<super::super::Foundation::POINTS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadPOINTS(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut super::super::Foundation::POINTS) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::POINTS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadPOINTS(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::POINTS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadPropertyKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<PROPERTYKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadPropertyKey(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadPropertyKey(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadRECTL<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<super::super::Foundation::RECTL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadRECTL(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut super::super::Foundation::RECTL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::RECTL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadRECTL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::RECTL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadSHORT<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadSHORT(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadSHORT(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadStr<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: super::super::Foundation::PWSTR, charactercount: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadStr(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, charactercount: i32) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_ReadStr(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value), ::std::mem::transmute(charactercount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadStrAlloc<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadStrAlloc(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadStrAlloc(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<super::Com::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadStream(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadStream(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<super::Com::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadType<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, var: *mut super::Com::VARIANT, r#type: u16) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadType(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, var: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, r#type: u16) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_ReadType(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(var), ::std::mem::transmute(r#type)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadULONGLONG<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadULONGLONG(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyBag_ReadULONGLONG(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_ReadUnknown<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadUnknown(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_ReadUnknown(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteBOOL<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteBOOL(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteBOOL(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteBSTR<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteBSTR(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteBSTR(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteDWORD<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteDWORD(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: u32) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteDWORD(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteGUID<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteGUID(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteGUID(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteInt<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteInt(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: i32) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteInt(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteLONG<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteLONG(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: i32) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteLONG(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTL<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: *const super::super::Foundation::POINTL) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WritePOINTL(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *const super::super::Foundation::POINTL) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WritePOINTL(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTS<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: *const super::super::Foundation::POINTS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WritePOINTS(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *const super::super::Foundation::POINTS) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WritePOINTS(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WritePropertyKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: *const PROPERTYKEY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WritePropertyKey(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *const PROPERTYKEY) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WritePropertyKey(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteRECTL<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: *const super::super::Foundation::RECTL) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteRECTL(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: *const super::super::Foundation::RECTL) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteRECTL(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteSHORT<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: i16) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteSHORT(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: i16) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteSHORT(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteStr<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteStr(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteStr(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteStream(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteStream(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteULONGLONG<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propbag: Param0, propname: Param1, value: u64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteULONGLONG(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, value: u64) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteULONGLONG(propbag.into_param().abi(), propname.into_param().abi(), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSPropertyBag_WriteUnknown<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(propbag: Param0, propname: Param1, punk: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteUnknown(propbag: ::windows::runtime::RawPtr, propname: super::super::Foundation::PWSTR, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        PSPropertyBag_WriteUnknown(propbag.into_param().abi(), propname.into_param().abi(), punk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSPropertyKeyFromString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszstring: Param0) -> ::windows::runtime::Result<PROPERTYKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyKeyFromString(pszstring: super::super::Foundation::PWSTR, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PSPropertyKeyFromString(pszstring.into_param().abi(), &mut result__).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[inline]
pub unsafe fn PSRefreshPropertySchema() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSRefreshPropertySchema() -> ::windows::runtime::HRESULT;
        }
        PSRefreshPropertySchema().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSRegisterPropertySchema<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszpath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSRegisterPropertySchema(pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        PSRegisterPropertySchema(pszpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PSSetPropertyValue<'a, Param0: ::windows::runtime::IntoParam<'a, IPropertyStore>, Param1: ::windows::runtime::IntoParam<'a, IPropertyDescription>>(pps: Param0, ppd: Param1, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSSetPropertyValue(pps: ::windows::runtime::RawPtr, ppd: ::windows::runtime::RawPtr, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        PSSetPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), ::std::mem::transmute(propvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::HRESULT;
        }
        PSStringFromPropertyKey(::std::mem::transmute(pkey), ::std::mem::transmute(psz), ::std::mem::transmute(cch)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSTIME_FLAGS(pub i32);
pub const PSTF_UTC: PSTIME_FLAGS = PSTIME_FLAGS(0i32);
pub const PSTF_LOCAL: PSTIME_FLAGS = PSTIME_FLAGS(1i32);
impl ::std::convert::From<i32> for PSTIME_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSTIME_FLAGS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PSUnregisterPropertySchema<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszpath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSUnregisterPropertySchema(pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        PSUnregisterPropertySchema(pszpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PifMgr_CloseProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprops: Param0, flopt: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_CloseProperties(hprops: super::super::Foundation::HANDLE, flopt: u32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(PifMgr_CloseProperties(hprops.into_param().abi(), ::std::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PifMgr_GetProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hprops: Param0, pszgroup: Param1, lpprops: *mut ::std::ffi::c_void, cbprops: i32, flopt: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_GetProperties(hprops: super::super::Foundation::HANDLE, pszgroup: super::super::Foundation::PSTR, lpprops: *mut ::std::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
        }
        ::std::mem::transmute(PifMgr_GetProperties(hprops.into_param().abi(), pszgroup.into_param().abi(), ::std::mem::transmute(lpprops), ::std::mem::transmute(cbprops), ::std::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PifMgr_OpenProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszapp: Param0, pszpif: Param1, hinf: u32, flopt: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_OpenProperties(pszapp: super::super::Foundation::PWSTR, pszpif: super::super::Foundation::PWSTR, hinf: u32, flopt: u32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(PifMgr_OpenProperties(pszapp.into_param().abi(), pszpif.into_param().abi(), ::std::mem::transmute(hinf), ::std::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PifMgr_SetProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hprops: Param0, pszgroup: Param1, lpprops: *const ::std::ffi::c_void, cbprops: i32, flopt: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_SetProperties(hprops: super::super::Foundation::HANDLE, pszgroup: super::super::Foundation::PSTR, lpprops: *const ::std::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
        }
        ::std::mem::transmute(PifMgr_SetProperties(hprops.into_param().abi(), pszgroup.into_param().abi(), ::std::mem::transmute(lpprops), ::std::mem::transmute(cbprops), ::std::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantChangeType(ppropvardest: *mut super::Com::StructuredStorage::PROPVARIANT, propvarsrc: *const super::Com::StructuredStorage::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: u16) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantChangeType(ppropvardest: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, propvarsrc: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, flags: PROPVAR_CHANGE_FLAGS, vt: u16) -> ::windows::runtime::HRESULT;
        }
        PropVariantChangeType(::std::mem::transmute(ppropvardest), ::std::mem::transmute(propvarsrc), ::std::mem::transmute(flags), ::std::mem::transmute(vt)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantCompareEx(propvar1: *const super::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::Com::StructuredStorage::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantCompareEx(propvar1: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32;
        }
        ::std::mem::transmute(PropVariantCompareEx(::std::mem::transmute(propvar1), ::std::mem::transmute(propvar2), ::std::mem::transmute(unit), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetBooleanElem(propvar: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetBooleanElem(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pfval: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantGetBooleanElem(::std::mem::transmute(propvar), ::std::mem::transmute(ielem), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetDoubleElem(propvar: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetDoubleElem(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantGetDoubleElem(::std::mem::transmute(propvar), ::std::mem::transmute(ielem), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetElementCount(propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetElementCount(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> u32;
        }
        ::std::mem::transmute(PropVariantGetElementCount(::std::mem::transmute(propvar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetFileTimeElem(propvar: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetFileTimeElem(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pftval: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantGetFileTimeElem(::std::mem::transmute(propvar), ::std::mem::transmute(ielem), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetInt16Elem(propvar: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInt16Elem(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantGetInt16Elem(::std::mem::transmute(propvar), ::std::mem::transmute(ielem), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetInt32Elem(propvar: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInt32Elem(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantGetInt32Elem(::std::mem::transmute(propvar), ::std::mem::transmute(ielem), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetInt64Elem(propvar: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInt64Elem(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantGetInt64Elem(::std::mem::transmute(propvar), ::std::mem::transmute(ielem), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetStringElem(propvar: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetStringElem(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, ppszval: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantGetStringElem(::std::mem::transmute(propvar), ::std::mem::transmute(ielem), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetUInt16Elem(propvar: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetUInt16Elem(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantGetUInt16Elem(::std::mem::transmute(propvar), ::std::mem::transmute(ielem), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetUInt32Elem(propvar: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetUInt32Elem(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantGetUInt32Elem(::std::mem::transmute(propvar), ::std::mem::transmute(ielem), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantGetUInt64Elem(propvar: *const super::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetUInt64Elem(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantGetUInt64Elem(::std::mem::transmute(propvar), ::std::mem::transmute(ielem), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToBSTR(propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBSTR(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToBSTR(::std::mem::transmute(propvar), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToBoolean(propvarin: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBoolean(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pfret: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToBoolean(::std::mem::transmute(propvarin), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToBooleanVector(propvar: *const super::Com::StructuredStorage::PROPVARIANT, prgf: *mut super::super::Foundation::BOOL, crgf: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBooleanVector(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, prgf: *mut super::super::Foundation::BOOL, crgf: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToBooleanVector(::std::mem::transmute(propvar), ::std::mem::transmute(prgf), ::std::mem::transmute(crgf), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToBooleanVectorAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pprgf: *mut *mut super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBooleanVectorAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pprgf: *mut *mut super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToBooleanVectorAlloc(::std::mem::transmute(propvar), ::std::mem::transmute(pprgf), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToBooleanWithDefault<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(propvarin: *const super::Com::StructuredStorage::PROPVARIANT, fdefault: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBooleanWithDefault(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, fdefault: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PropVariantToBooleanWithDefault(::std::mem::transmute(propvarin), fdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToBuffer(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pv: *mut ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBuffer(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pv: *mut ::std::ffi::c_void, cb: u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToBuffer(::std::mem::transmute(propvar), ::std::mem::transmute(pv), ::std::mem::transmute(cb)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToDouble(propvarin: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDouble(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pdblret: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToDouble(::std::mem::transmute(propvarin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToDoubleVector(propvar: *const super::Com::StructuredStorage::PROPVARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDoubleVector(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToDoubleVector(::std::mem::transmute(propvar), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToDoubleVectorAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDoubleVectorAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToDoubleVectorAlloc(::std::mem::transmute(propvar), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToDoubleWithDefault(propvarin: *const super::Com::StructuredStorage::PROPVARIANT, dbldefault: f64) -> f64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDoubleWithDefault(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, dbldefault: f64) -> f64;
        }
        ::std::mem::transmute(PropVariantToDoubleWithDefault(::std::mem::transmute(propvarin), ::std::mem::transmute(dbldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToFileTime(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pstfout: PSTIME_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToFileTime(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pstfout: PSTIME_FLAGS, pftout: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToFileTime(::std::mem::transmute(propvar), ::std::mem::transmute(pstfout), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToFileTimeVector(propvar: *const super::Com::StructuredStorage::PROPVARIANT, prgft: *mut super::super::Foundation::FILETIME, crgft: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToFileTimeVector(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, prgft: *mut super::super::Foundation::FILETIME, crgft: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToFileTimeVector(::std::mem::transmute(propvar), ::std::mem::transmute(prgft), ::std::mem::transmute(crgft), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToFileTimeVectorAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pprgft: *mut *mut super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToFileTimeVectorAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pprgft: *mut *mut super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToFileTimeVectorAlloc(::std::mem::transmute(propvar), ::std::mem::transmute(pprgft), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToGUID(propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToGUID(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToGUID(::std::mem::transmute(propvar), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt16(propvarin: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, piret: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToInt16(::std::mem::transmute(propvarin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt16Vector(propvar: *const super::Com::StructuredStorage::PROPVARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16Vector(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToInt16Vector(::std::mem::transmute(propvar), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt16VectorAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16VectorAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToInt16VectorAlloc(::std::mem::transmute(propvar), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt16WithDefault(propvarin: *const super::Com::StructuredStorage::PROPVARIANT, idefault: i16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16WithDefault(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, idefault: i16) -> i16;
        }
        ::std::mem::transmute(PropVariantToInt16WithDefault(::std::mem::transmute(propvarin), ::std::mem::transmute(idefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt32(propvarin: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, plret: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToInt32(::std::mem::transmute(propvarin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt32Vector(propvar: *const super::Com::StructuredStorage::PROPVARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32Vector(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToInt32Vector(::std::mem::transmute(propvar), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt32VectorAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32VectorAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToInt32VectorAlloc(::std::mem::transmute(propvar), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt32WithDefault(propvarin: *const super::Com::StructuredStorage::PROPVARIANT, ldefault: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32WithDefault(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ldefault: i32) -> i32;
        }
        ::std::mem::transmute(PropVariantToInt32WithDefault(::std::mem::transmute(propvarin), ::std::mem::transmute(ldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt64(propvarin: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pllret: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToInt64(::std::mem::transmute(propvarin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt64Vector(propvar: *const super::Com::StructuredStorage::PROPVARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64Vector(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToInt64Vector(::std::mem::transmute(propvar), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt64VectorAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64VectorAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToInt64VectorAlloc(::std::mem::transmute(propvar), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToInt64WithDefault(propvarin: *const super::Com::StructuredStorage::PROPVARIANT, lldefault: i64) -> i64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64WithDefault(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, lldefault: i64) -> i64;
        }
        ::std::mem::transmute(PropVariantToInt64WithDefault(::std::mem::transmute(propvarin), ::std::mem::transmute(lldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Shell"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_UI_Shell`*"]
#[inline]
pub unsafe fn PropVariantToStrRet(propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::super::UI::Shell::STRRET> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStrRet(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pstrret: *mut super::super::UI::Shell::STRRET) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::UI::Shell::STRRET as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToStrRet(::std::mem::transmute(propvar), &mut result__).from_abi::<super::super::UI::Shell::STRRET>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToString(propvar: *const super::Com::StructuredStorage::PROPVARIANT, psz: super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToString(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, psz: super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToString(::std::mem::transmute(propvar), ::std::mem::transmute(psz), ::std::mem::transmute(cch)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToStringAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ppszout: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToStringAlloc(::std::mem::transmute(propvar), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToStringVector(propvar: *const super::Com::StructuredStorage::PROPVARIANT, prgsz: *mut super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringVector(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, prgsz: *mut super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToStringVector(::std::mem::transmute(propvar), ::std::mem::transmute(prgsz), ::std::mem::transmute(crgsz), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToStringVectorAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pprgsz: *mut *mut super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringVectorAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pprgsz: *mut *mut super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToStringVectorAlloc(::std::mem::transmute(propvar), ::std::mem::transmute(pprgsz), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToStringWithDefault<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propvarin: *const super::Com::StructuredStorage::PROPVARIANT, pszdefault: Param1) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringWithDefault(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pszdefault: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
        }
        ::std::mem::transmute(PropVariantToStringWithDefault(::std::mem::transmute(propvarin), pszdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt16(propvarin: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, puiret: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToUInt16(::std::mem::transmute(propvarin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt16Vector(propvar: *const super::Com::StructuredStorage::PROPVARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16Vector(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToUInt16Vector(::std::mem::transmute(propvar), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt16VectorAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16VectorAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToUInt16VectorAlloc(::std::mem::transmute(propvar), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt16WithDefault(propvarin: *const super::Com::StructuredStorage::PROPVARIANT, uidefault: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16WithDefault(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, uidefault: u16) -> u16;
        }
        ::std::mem::transmute(PropVariantToUInt16WithDefault(::std::mem::transmute(propvarin), ::std::mem::transmute(uidefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt32(propvarin: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pulret: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToUInt32(::std::mem::transmute(propvarin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt32Vector(propvar: *const super::Com::StructuredStorage::PROPVARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32Vector(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToUInt32Vector(::std::mem::transmute(propvar), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt32VectorAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32VectorAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToUInt32VectorAlloc(::std::mem::transmute(propvar), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt32WithDefault(propvarin: *const super::Com::StructuredStorage::PROPVARIANT, uldefault: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32WithDefault(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, uldefault: u32) -> u32;
        }
        ::std::mem::transmute(PropVariantToUInt32WithDefault(::std::mem::transmute(propvarin), ::std::mem::transmute(uldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt64(propvarin: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pullret: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToUInt64(::std::mem::transmute(propvarin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt64Vector(propvar: *const super::Com::StructuredStorage::PROPVARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64Vector(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToUInt64Vector(::std::mem::transmute(propvar), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt64VectorAlloc(propvar: *const super::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64VectorAlloc(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PropVariantToUInt64VectorAlloc(::std::mem::transmute(propvar), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToUInt64WithDefault(propvarin: *const super::Com::StructuredStorage::PROPVARIANT, ulldefault: u64) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64WithDefault(propvarin: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, ulldefault: u64) -> u64;
        }
        ::std::mem::transmute(PropVariantToUInt64WithDefault(::std::mem::transmute(propvarin), ::std::mem::transmute(ulldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToVariant(ppropvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToVariant(ppropvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropVariantToVariant(::std::mem::transmute(ppropvar), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn PropVariantToWinRTPropertyValue<T: ::windows::runtime::Interface>(propvar: *const super::Com::StructuredStorage::PROPVARIANT, result__: *mut ::std::option::Option<T>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToWinRTPropertyValue(propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PropVariantToWinRTPropertyValue(::std::mem::transmute(propvar), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const PropertySystem: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3096870789, 22702, 20294, [159, 178, 93, 121, 4, 121, 143, 75]);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct SERIALIZEDPROPSTORAGE(pub u8);
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn SHAddDefaultPropertiesByExt<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IPropertyStore>>(pszext: Param0, ppropstore: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHAddDefaultPropertiesByExt(pszext: super::super::Foundation::PWSTR, ppropstore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        SHAddDefaultPropertiesByExt(pszext.into_param().abi(), ppropstore.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn SHGetPropertyStoreForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHGetPropertyStoreForWindow(hwnd: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        SHGetPropertyStoreForWindow(hwnd.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_Shell")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_UI_Shell`*"]
#[inline]
pub unsafe fn SHGetPropertyStoreFromIDList(pidl: *const super::super::UI::Shell::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHGetPropertyStoreFromIDList(pidl: *const super::super::UI::Shell::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        SHGetPropertyStoreFromIDList(::std::mem::transmute(pidl), ::std::mem::transmute(flags), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn SHGetPropertyStoreFromParsingName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::IBindCtx>>(pszpath: Param0, pbc: Param1, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHGetPropertyStoreFromParsingName(pszpath: super::super::Foundation::PWSTR, pbc: ::windows::runtime::RawPtr, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        SHGetPropertyStoreFromParsingName(pszpath.into_param().abi(), pbc.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn SHPropStgCreate<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertySetStorage>>(psstg: Param0, fmtid: *const ::windows::runtime::GUID, pclsid: *const ::windows::runtime::GUID, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut ::std::option::Option<super::Com::StructuredStorage::IPropertyStorage>, pucodepage: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHPropStgCreate(psstg: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pclsid: *const ::windows::runtime::GUID, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut ::windows::runtime::RawPtr, pucodepage: *mut u32) -> ::windows::runtime::HRESULT;
        }
        SHPropStgCreate(psstg.into_param().abi(), ::std::mem::transmute(fmtid), ::std::mem::transmute(pclsid), ::std::mem::transmute(grfflags), ::std::mem::transmute(grfmode), ::std::mem::transmute(dwdisposition), ::std::mem::transmute(ppstg), ::std::mem::transmute(pucodepage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn SHPropStgReadMultiple<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyStorage>>(pps: Param0, ucodepage: u32, cpspec: u32, rgpspec: *const super::Com::StructuredStorage::PROPSPEC) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHPropStgReadMultiple(pps: ::windows::runtime::RawPtr, ucodepage: u32, cpspec: u32, rgpspec: *const super::Com::StructuredStorage::PROPSPEC, rgvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SHPropStgReadMultiple(pps.into_param().abi(), ::std::mem::transmute(ucodepage), ::std::mem::transmute(cpspec), ::std::mem::transmute(rgpspec), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn SHPropStgWriteMultiple<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPropertyStorage>>(pps: Param0, pucodepage: *mut u32, cpspec: u32, rgpspec: *const super::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHPropStgWriteMultiple(pps: ::windows::runtime::RawPtr, pucodepage: *mut u32, cpspec: u32, rgpspec: *const super::Com::StructuredStorage::PROPSPEC, rgvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>, propidnamefirst: u32) -> ::windows::runtime::HRESULT;
        }
        SHPropStgWriteMultiple(pps.into_param().abi(), ::std::mem::transmute(pucodepage), ::std::mem::transmute(cpspec), ::std::mem::transmute(rgpspec), ::std::mem::transmute(rgvar), ::std::mem::transmute(propidnamefirst)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for SYNC_ENGINE_STATE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYNC_ENGINE_STATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for SYNC_TRANSFER_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYNC_TRANSFER_STATUS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantCompare(var1: *const super::Com::VARIANT, var2: *const super::Com::VARIANT) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantCompare(var1: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, var2: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> i32;
        }
        ::std::mem::transmute(VariantCompare(::std::mem::transmute(var1), ::std::mem::transmute(var2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantGetBooleanElem(var: *const super::Com::VARIANT, ielem: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetBooleanElem(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ielem: u32, pfval: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantGetBooleanElem(::std::mem::transmute(var), ::std::mem::transmute(ielem), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantGetDoubleElem(var: *const super::Com::VARIANT, ielem: u32) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetDoubleElem(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ielem: u32, pnval: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantGetDoubleElem(::std::mem::transmute(var), ::std::mem::transmute(ielem), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantGetElementCount(varin: *const super::Com::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetElementCount(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> u32;
        }
        ::std::mem::transmute(VariantGetElementCount(::std::mem::transmute(varin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantGetInt16Elem(var: *const super::Com::VARIANT, ielem: u32) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetInt16Elem(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ielem: u32, pnval: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantGetInt16Elem(::std::mem::transmute(var), ::std::mem::transmute(ielem), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantGetInt32Elem(var: *const super::Com::VARIANT, ielem: u32) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetInt32Elem(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ielem: u32, pnval: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantGetInt32Elem(::std::mem::transmute(var), ::std::mem::transmute(ielem), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantGetInt64Elem(var: *const super::Com::VARIANT, ielem: u32) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetInt64Elem(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ielem: u32, pnval: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantGetInt64Elem(::std::mem::transmute(var), ::std::mem::transmute(ielem), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantGetStringElem(var: *const super::Com::VARIANT, ielem: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetStringElem(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ielem: u32, ppszval: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantGetStringElem(::std::mem::transmute(var), ::std::mem::transmute(ielem), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantGetUInt16Elem(var: *const super::Com::VARIANT, ielem: u32) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetUInt16Elem(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ielem: u32, pnval: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantGetUInt16Elem(::std::mem::transmute(var), ::std::mem::transmute(ielem), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantGetUInt32Elem(var: *const super::Com::VARIANT, ielem: u32) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetUInt32Elem(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ielem: u32, pnval: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantGetUInt32Elem(::std::mem::transmute(var), ::std::mem::transmute(ielem), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantGetUInt64Elem(var: *const super::Com::VARIANT, ielem: u32) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetUInt64Elem(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ielem: u32, pnval: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantGetUInt64Elem(::std::mem::transmute(var), ::std::mem::transmute(ielem), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToBoolean(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBoolean(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pfret: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToBoolean(::std::mem::transmute(varin), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToBooleanArray(var: *const super::Com::VARIANT, prgf: *mut super::super::Foundation::BOOL, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBooleanArray(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, prgf: *mut super::super::Foundation::BOOL, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToBooleanArray(::std::mem::transmute(var), ::std::mem::transmute(prgf), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToBooleanArrayAlloc(var: *const super::Com::VARIANT, pprgf: *mut *mut super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBooleanArrayAlloc(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pprgf: *mut *mut super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToBooleanArrayAlloc(::std::mem::transmute(var), ::std::mem::transmute(pprgf), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToBooleanWithDefault<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(varin: *const super::Com::VARIANT, fdefault: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBooleanWithDefault(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, fdefault: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VariantToBooleanWithDefault(::std::mem::transmute(varin), fdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToBuffer(varin: *const super::Com::VARIANT, pv: *mut ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBuffer(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pv: *mut ::std::ffi::c_void, cb: u32) -> ::windows::runtime::HRESULT;
        }
        VariantToBuffer(::std::mem::transmute(varin), ::std::mem::transmute(pv), ::std::mem::transmute(cb)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToDosDateTime(varin: *const super::Com::VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDosDateTime(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pwdate: *mut u16, pwtime: *mut u16) -> ::windows::runtime::HRESULT;
        }
        VariantToDosDateTime(::std::mem::transmute(varin), ::std::mem::transmute(pwdate), ::std::mem::transmute(pwtime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToDouble(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDouble(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pdblret: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToDouble(::std::mem::transmute(varin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToDoubleArray(var: *const super::Com::VARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDoubleArray(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToDoubleArray(::std::mem::transmute(var), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToDoubleArrayAlloc(var: *const super::Com::VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDoubleArrayAlloc(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToDoubleArrayAlloc(::std::mem::transmute(var), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToDoubleWithDefault(varin: *const super::Com::VARIANT, dbldefault: f64) -> f64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDoubleWithDefault(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, dbldefault: f64) -> f64;
        }
        ::std::mem::transmute(VariantToDoubleWithDefault(::std::mem::transmute(varin), ::std::mem::transmute(dbldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToFileTime(varin: *const super::Com::VARIANT, stfout: PSTIME_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToFileTime(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, stfout: PSTIME_FLAGS, pftout: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToFileTime(::std::mem::transmute(varin), ::std::mem::transmute(stfout), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToGUID(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToGUID(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToGUID(::std::mem::transmute(varin), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt16(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, piret: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToInt16(::std::mem::transmute(varin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt16Array(var: *const super::Com::VARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16Array(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToInt16Array(::std::mem::transmute(var), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt16ArrayAlloc(var: *const super::Com::VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16ArrayAlloc(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToInt16ArrayAlloc(::std::mem::transmute(var), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt16WithDefault(varin: *const super::Com::VARIANT, idefault: i16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16WithDefault(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, idefault: i16) -> i16;
        }
        ::std::mem::transmute(VariantToInt16WithDefault(::std::mem::transmute(varin), ::std::mem::transmute(idefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt32(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, plret: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToInt32(::std::mem::transmute(varin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt32Array(var: *const super::Com::VARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32Array(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToInt32Array(::std::mem::transmute(var), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt32ArrayAlloc(var: *const super::Com::VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32ArrayAlloc(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToInt32ArrayAlloc(::std::mem::transmute(var), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt32WithDefault(varin: *const super::Com::VARIANT, ldefault: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32WithDefault(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ldefault: i32) -> i32;
        }
        ::std::mem::transmute(VariantToInt32WithDefault(::std::mem::transmute(varin), ::std::mem::transmute(ldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt64(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pllret: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToInt64(::std::mem::transmute(varin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt64Array(var: *const super::Com::VARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64Array(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToInt64Array(::std::mem::transmute(var), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt64ArrayAlloc(var: *const super::Com::VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64ArrayAlloc(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToInt64ArrayAlloc(::std::mem::transmute(var), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToInt64WithDefault(varin: *const super::Com::VARIANT, lldefault: i64) -> i64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64WithDefault(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, lldefault: i64) -> i64;
        }
        ::std::mem::transmute(VariantToInt64WithDefault(::std::mem::transmute(varin), ::std::mem::transmute(lldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToPropVariant(pvar: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToPropVariant(pvar: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToPropVariant(::std::mem::transmute(pvar), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Shell"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`, `Win32_UI_Shell`*"]
#[inline]
pub unsafe fn VariantToStrRet(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::super::UI::Shell::STRRET> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStrRet(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pstrret: *mut super::super::UI::Shell::STRRET) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::UI::Shell::STRRET as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToStrRet(::std::mem::transmute(varin), &mut result__).from_abi::<super::super::UI::Shell::STRRET>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToString(varin: *const super::Com::VARIANT, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToString(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows::runtime::HRESULT;
        }
        VariantToString(::std::mem::transmute(varin), ::std::mem::transmute(pszbuf), ::std::mem::transmute(cchbuf)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToStringAlloc(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringAlloc(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ppszbuf: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToStringAlloc(::std::mem::transmute(varin), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToStringArray(var: *const super::Com::VARIANT, prgsz: *mut super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringArray(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, prgsz: *mut super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToStringArray(::std::mem::transmute(var), ::std::mem::transmute(prgsz), ::std::mem::transmute(crgsz), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToStringArrayAlloc(var: *const super::Com::VARIANT, pprgsz: *mut *mut super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringArrayAlloc(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pprgsz: *mut *mut super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToStringArrayAlloc(::std::mem::transmute(var), ::std::mem::transmute(pprgsz), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToStringWithDefault<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(varin: *const super::Com::VARIANT, pszdefault: Param1) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringWithDefault(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pszdefault: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
        }
        ::std::mem::transmute(VariantToStringWithDefault(::std::mem::transmute(varin), pszdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt16(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, puiret: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToUInt16(::std::mem::transmute(varin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt16Array(var: *const super::Com::VARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16Array(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToUInt16Array(::std::mem::transmute(var), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt16ArrayAlloc(var: *const super::Com::VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16ArrayAlloc(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToUInt16ArrayAlloc(::std::mem::transmute(var), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt16WithDefault(varin: *const super::Com::VARIANT, uidefault: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16WithDefault(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, uidefault: u16) -> u16;
        }
        ::std::mem::transmute(VariantToUInt16WithDefault(::std::mem::transmute(varin), ::std::mem::transmute(uidefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt32(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pulret: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToUInt32(::std::mem::transmute(varin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt32Array(var: *const super::Com::VARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32Array(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToUInt32Array(::std::mem::transmute(var), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt32ArrayAlloc(var: *const super::Com::VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32ArrayAlloc(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToUInt32ArrayAlloc(::std::mem::transmute(var), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt32WithDefault(varin: *const super::Com::VARIANT, uldefault: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32WithDefault(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, uldefault: u32) -> u32;
        }
        ::std::mem::transmute(VariantToUInt32WithDefault(::std::mem::transmute(varin), ::std::mem::transmute(uldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt64(varin: *const super::Com::VARIANT) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pullret: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VariantToUInt64(::std::mem::transmute(varin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt64Array(var: *const super::Com::VARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64Array(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToUInt64Array(::std::mem::transmute(var), ::std::mem::transmute(prgn), ::std::mem::transmute(crgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt64ArrayAlloc(var: *const super::Com::VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64ArrayAlloc(var: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
        }
        VariantToUInt64ArrayAlloc(::std::mem::transmute(var), ::std::mem::transmute(pprgn), ::std::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn VariantToUInt64WithDefault(varin: *const super::Com::VARIANT, ulldefault: u64) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64WithDefault(varin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, ulldefault: u64) -> u64;
        }
        ::std::mem::transmute(VariantToUInt64WithDefault(::std::mem::transmute(varin), ::std::mem::transmute(ulldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn WinRTPropertyValueToPropVariant<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punkpropertyvalue: Param0) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinRTPropertyValueToPropVariant(punkpropertyvalue: ::windows::runtime::RawPtr, ppropvar: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WinRTPropertyValueToPropVariant(punkpropertyvalue.into_param().abi(), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _PERSIST_SPROPSTORE_FLAGS(pub i32);
pub const FPSPS_DEFAULT: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(0i32);
pub const FPSPS_READONLY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(1i32);
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(2i32);
impl ::std::convert::From<i32> for _PERSIST_SPROPSTORE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _PERSIST_SPROPSTORE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_PropertiesSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _PROPERTYUI_FLAGS(pub i32);
pub const PUIF_DEFAULT: _PROPERTYUI_FLAGS = _PROPERTYUI_FLAGS(0i32);
pub const PUIF_RIGHTALIGN: _PROPERTYUI_FLAGS = _PROPERTYUI_FLAGS(1i32);
pub const PUIF_NOLABELININFOTIP: _PROPERTYUI_FLAGS = _PROPERTYUI_FLAGS(2i32);
impl ::std::convert::From<i32> for _PROPERTYUI_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _PROPERTYUI_FLAGS {
    type Abi = Self;
}
