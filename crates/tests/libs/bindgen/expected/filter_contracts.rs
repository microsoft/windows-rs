#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color(pub i32);
impl Color {
    pub const Green: Self = Self(1);
    pub const Blue: Self = Self(2);
}
impl windows_core::imp::TypeKind for Color {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Color {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Test.Color;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Color");
}
windows_core::imp::define_interface!(INativeResult, INativeResult_Vtbl);
#[repr(C)]
pub struct INativeResult_Vtbl {
    NativeRead: usize,
}
windows_core::imp::define_interface!(INativeSource, INativeSource_Vtbl);
impl INativeSource {
    pub unsafe fn GetNativeResult(&self, result: *mut *mut Option<INativeResult>) {
        unsafe {
            (windows_core::Interface::vtable(self).GetNativeResult)(
                windows_core::Interface::as_raw(self),
                result as _,
            );
        }
    }
}
#[repr(C)]
pub struct INativeSource_Vtbl {
    pub GetNativeResult:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut *mut core::ffi::c_void),
}
windows_core::imp::define_winrt_interface!(
    IResult,
    IResult_Vtbl,
    0xe7219288_216a_5fcf_9abc_4a9875a85bb4
);
windows_core::imp::interface_hierarchy!(
    IResult,
    windows_core::IUnknown,
    windows_core::IInspectable
);
#[repr(C)]
pub struct IResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_winrt_interface!(
    ISource,
    ISource_Vtbl,
    0x153d776d_996c_5df3_b7d5_6bcbfe401014
);
windows_core::imp::interface_hierarchy!(
    ISource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ISource {
    pub fn GetResult(&self) -> windows_core::Result<IResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResult)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ISource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetResult: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(
    IWidget,
    IWidget_Vtbl,
    0x59530da3_0659_5d65_8fda_64bba4849f32
);
windows_core::imp::interface_hierarchy!(
    IWidget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidget {
    pub fn Other(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Other)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IWidget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Value: usize,
    put_Value: usize,
    pub Other: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const NativeBlue: NativeColor = 2;
pub type NativeColor = i32;
pub const NativeGreen: NativeColor = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Token {
    pub value: i32,
}
impl windows_core::imp::TypeKind for Token {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Token {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Other.Token;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Other.Token");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Widget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Widget,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IWidget
);
impl windows_core::RuntimeType for Widget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidget>();
}
unsafe impl windows_core::Interface for Widget {
    type Vtable = <IWidget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidget as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl core::ops::Deref for Widget {
    type Target = IWidget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Widget {
    const NAME: &'static str = "Test.Widget";
}
