windows_core::imp::define_interface!(
    IPropertyValue,
    IPropertyValue_Vtbl,
    0x4bd682dd_7554_40e9_9a9b_82654ede7e62
);
impl windows_core::RuntimeType for IPropertyValue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IPropertyValue");
}
windows_core::imp::interface_hierarchy!(
    IPropertyValue,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IPropertyValue {
    pub fn IsNumericScalar(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsNumericScalar)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt8(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUInt8)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetInt16(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInt16)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt16(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUInt16)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetInt32(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInt32)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt32(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUInt32)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetInt64(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInt64)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt64(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUInt64)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetSingle(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSingle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetDouble(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDouble)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetChar16(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChar16)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetBoolean(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoolean)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetString)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGuid)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetDateTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDateTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetTimeSpan(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimeSpan)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetUInt8Array)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetInt16Array)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetUInt16Array)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetInt32Array)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetUInt32Array)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetInt64Array)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetUInt64Array)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetSingleArray)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetDoubleArray)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetChar16Array)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetBooleanArray(
        &self,
        value: &mut windows_core::Array<bool>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetBooleanArray)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetStringArray(
        &self,
        value: &mut windows_core::Array<windows_core::HSTRING>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetStringArray)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetInspectableArray(
        &self,
        value: &mut windows_core::Array<windows_core::IInspectable>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetInspectableArray)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetGuidArray(
        &self,
        value: &mut windows_core::Array<windows_core::GUID>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetGuidArray)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetDateTimeArray(
        &self,
        value: &mut windows_core::Array<windows_time::DateTime>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetDateTimeArray)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetTimeSpanArray(
        &self,
        value: &mut windows_core::Array<windows_time::TimeSpan>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetTimeSpanArray)(
                windows_core::Interface::as_raw(self),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IPropertyValue {
    const NAME: &'static str = "Windows.Foundation.IPropertyValue";
}
#[repr(C)]
pub struct IPropertyValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Type: usize,
    pub IsNumericScalar:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetUInt8:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub GetInt16:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub GetUInt16:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetInt32:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetUInt32:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetInt64:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetUInt64:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetSingle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetDouble:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetChar16:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetBoolean:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub GetDateTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::DateTime,
    ) -> windows_core::HRESULT,
    pub GetTimeSpan: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    GetPoint: usize,
    GetSize: usize,
    GetRect: usize,
    pub GetUInt8Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut u8,
    ) -> windows_core::HRESULT,
    pub GetInt16Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut i16,
    ) -> windows_core::HRESULT,
    pub GetUInt16Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut u16,
    ) -> windows_core::HRESULT,
    pub GetInt32Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut i32,
    ) -> windows_core::HRESULT,
    pub GetUInt32Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut u32,
    ) -> windows_core::HRESULT,
    pub GetInt64Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut i64,
    ) -> windows_core::HRESULT,
    pub GetUInt64Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut u64,
    ) -> windows_core::HRESULT,
    pub GetSingleArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut f32,
    ) -> windows_core::HRESULT,
    pub GetDoubleArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut f64,
    ) -> windows_core::HRESULT,
    pub GetChar16Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut u16,
    ) -> windows_core::HRESULT,
    pub GetBooleanArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut bool,
    ) -> windows_core::HRESULT,
    pub GetStringArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut windows_core::HSTRING,
    ) -> windows_core::HRESULT,
    pub GetInspectableArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut windows_core::IInspectable,
    ) -> windows_core::HRESULT,
    pub GetGuidArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub GetDateTimeArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut windows_time::DateTime,
    ) -> windows_core::HRESULT,
    pub GetTimeSpanArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RefWithFloat {
    pub Value: Option<windows_reference::IReference<f32>>,
}
impl windows_core::TypeKind for RefWithFloat {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for RefWithFloat {
    const SIGNATURE : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"struct(test_reference_float.RefWithFloat;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};f4))") ;
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"test_reference_float.RefWithFloat");
}
