#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IStringable,
    IStringable_Vtbl,
    0x96369f54_8eb6_48f0_abce_c1b211e627c3
);
windows_core::imp::interface_hierarchy!(
    IStringable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IStringable {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl IStringable {
    pub fn ToString(&self) {}
}
impl windows_core::RuntimeType for IStringable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStringable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IUriEscapeStatics,
    IUriEscapeStatics_Vtbl,
    0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1
);
windows_core::imp::interface_hierarchy!(
    IUriEscapeStatics,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IUriEscapeStatics {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeType for IUriEscapeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriEscapeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IUriRuntimeClass,
    IUriRuntimeClass_Vtbl,
    0x9e365e57_48b2_4160_956f_c7385120bbfc
);
windows_core::imp::interface_hierarchy!(
    IUriRuntimeClass,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IUriRuntimeClass {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeType for IUriRuntimeClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IUriRuntimeClassFactory,
    IUriRuntimeClassFactory_Vtbl,
    0x44a9796f_723e_4fdf_a218_033e75b0c084
);
windows_core::imp::interface_hierarchy!(
    IUriRuntimeClassFactory,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IUriRuntimeClassFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeType for IUriRuntimeClassFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IUriRuntimeClassWithAbsoluteCanonicalUri,
    IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl,
    0x758d9661_221c_480f_a339_50656673f46f
);
windows_core::imp::interface_hierarchy!(
    IUriRuntimeClassWithAbsoluteCanonicalUri,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IUriRuntimeClassWithAbsoluteCanonicalUri {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeType for IUriRuntimeClassWithAbsoluteCanonicalUri {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Uri(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Uri, windows_core::IUnknown, windows_core::IInspectable);
impl Uri {
    pub fn ToString(&self) {}
    pub fn UnescapeComponent(&self) {}
    pub fn EscapeComponent(&self) {}
    pub fn AbsoluteUri(&self) {}
    pub fn DisplayUri(&self) {}
    pub fn Domain(&self) {}
    pub fn Extension(&self) {}
    pub fn Fragment(&self) {}
    pub fn Host(&self) {}
    pub fn Password(&self) {}
    pub fn Path(&self) {}
    pub fn Query(&self) {}
    pub fn RawUri(&self) {}
    pub fn SchemeName(&self) {}
    pub fn UserName(&self) {}
    pub fn Port(&self) {}
    pub fn Suspicious(&self) {}
    pub fn Equals(&self) {}
    pub fn CombineUri(&self) {}
    pub fn CreateUri(&self) {}
    pub fn CreateWithRelativeUri(&self) {}
    pub fn AbsoluteCanonicalUri(&self) {}
    pub fn DisplayIri(&self) {}
}
impl windows_core::RuntimeType for Uri {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUriRuntimeClass>();
}
unsafe impl windows_core::Interface for Uri {
    type Vtable = <IUriRuntimeClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUriRuntimeClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Uri {
    const NAME: &'static str = "Windows.Foundation.Uri";
}
