#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DateTime {
    pub UniversalTime: i64,
}
impl windows_core::TypeKind for DateTime {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DateTime {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
}
pub const E_NOTIMPL: windows_core::HRESULT = windows_core::HRESULT(0x80004001_u32 as _);
windows_core::imp::define_interface!(
    IPropertyValue,
    IPropertyValue_Vtbl,
    0x4bd682dd_7554_40e9_9a9b_82654ede7e62
);
impl windows_core::RuntimeType for IPropertyValue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IPropertyValue,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IPropertyValue {
    pub fn Type(&self) -> windows_core::Result<PropertyType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsNumericScalar(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNumericScalar)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt8(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt8)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetInt16(&self) -> windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt16)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt16(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt16)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetInt32(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt32)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt32(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt32)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetInt64(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt64)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt64(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt64)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetSingle(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSingle)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetDouble(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDouble)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetChar16(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChar16)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetBoolean(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBoolean)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGuid)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetDateTime(&self) -> windows_core::Result<DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDateTime)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetTimeSpan(&self) -> windows_core::Result<TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTimeSpan)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetPoint(&self) -> windows_core::Result<Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetSize(&self) -> windows_core::Result<Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetRect(&self) -> windows_core::Result<Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRect)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetUInt8Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetInt16Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetUInt16Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetInt32Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetUInt32Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetInt64Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetUInt64Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetSingleArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetDoubleArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetChar16Array)(
                windows_core::Interface::as_raw(this),
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
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetBooleanArray)(
                windows_core::Interface::as_raw(this),
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
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetStringArray)(
                windows_core::Interface::as_raw(this),
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
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetInspectableArray)(
                windows_core::Interface::as_raw(this),
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
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetGuidArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetDateTimeArray(
        &self,
        value: &mut windows_core::Array<DateTime>,
    ) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetDateTimeArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetTimeSpanArray(
        &self,
        value: &mut windows_core::Array<TimeSpan>,
    ) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetTimeSpanArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetPointArray(
        &self,
        value: &mut windows_core::Array<Point>,
    ) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetPointArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetSizeArray(&self, value: &mut windows_core::Array<Size>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetSizeArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetRectArray(&self, value: &mut windows_core::Array<Rect>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetRectArray)(
                windows_core::Interface::as_raw(this),
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
pub trait IPropertyValue_Impl: windows_core::IUnknownImpl {
    fn Type(&self) -> windows_core::Result<PropertyType>;
    fn IsNumericScalar(&self) -> windows_core::Result<bool>;
    fn GetUInt8(&self) -> windows_core::Result<u8>;
    fn GetInt16(&self) -> windows_core::Result<i16>;
    fn GetUInt16(&self) -> windows_core::Result<u16>;
    fn GetInt32(&self) -> windows_core::Result<i32>;
    fn GetUInt32(&self) -> windows_core::Result<u32>;
    fn GetInt64(&self) -> windows_core::Result<i64>;
    fn GetUInt64(&self) -> windows_core::Result<u64>;
    fn GetSingle(&self) -> windows_core::Result<f32>;
    fn GetDouble(&self) -> windows_core::Result<f64>;
    fn GetChar16(&self) -> windows_core::Result<u16>;
    fn GetBoolean(&self) -> windows_core::Result<bool>;
    fn GetString(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDateTime(&self) -> windows_core::Result<DateTime>;
    fn GetTimeSpan(&self) -> windows_core::Result<TimeSpan>;
    fn GetPoint(&self) -> windows_core::Result<Point>;
    fn GetSize(&self) -> windows_core::Result<Size>;
    fn GetRect(&self) -> windows_core::Result<Rect>;
    fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()>;
    fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()>;
    fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()>;
    fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()>;
    fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()>;
    fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()>;
    fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()>;
    fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()>;
    fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()>;
    fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()>;
    fn GetBooleanArray(&self, value: &mut windows_core::Array<bool>) -> windows_core::Result<()>;
    fn GetStringArray(
        &self,
        value: &mut windows_core::Array<windows_core::HSTRING>,
    ) -> windows_core::Result<()>;
    fn GetInspectableArray(
        &self,
        value: &mut windows_core::Array<windows_core::IInspectable>,
    ) -> windows_core::Result<()>;
    fn GetGuidArray(
        &self,
        value: &mut windows_core::Array<windows_core::GUID>,
    ) -> windows_core::Result<()>;
    fn GetDateTimeArray(
        &self,
        value: &mut windows_core::Array<DateTime>,
    ) -> windows_core::Result<()>;
    fn GetTimeSpanArray(
        &self,
        value: &mut windows_core::Array<TimeSpan>,
    ) -> windows_core::Result<()>;
    fn GetPointArray(&self, value: &mut windows_core::Array<Point>) -> windows_core::Result<()>;
    fn GetSizeArray(&self, value: &mut windows_core::Array<Size>) -> windows_core::Result<()>;
    fn GetRectArray(&self, value: &mut windows_core::Array<Rect>) -> windows_core::Result<()>;
}
impl IPropertyValue_Vtbl {
    pub const fn new<Identity: IPropertyValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut PropertyType,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::Type(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsNumericScalar<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::IsNumericScalar(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUInt8<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetUInt8(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInt16<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i16,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetInt16(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUInt16<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u16,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetUInt16(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInt32<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetInt32(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUInt32<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetUInt32(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInt64<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetInt64(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUInt64<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetUInt64(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSingle<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetSingle(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDouble<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetDouble(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChar16<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u16,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetChar16(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBoolean<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetBoolean(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetString(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGuid<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetGuid(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDateTime<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut DateTime,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetDateTime(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTimeSpan<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut TimeSpan,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetTimeSpan(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPoint<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut Point,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetPoint(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSize<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut Size,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetSize(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRect<Identity: IPropertyValue_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut Rect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetRect(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUInt8Array<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetUInt8Array(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetInt16Array<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut i16,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetInt16Array(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetUInt16Array<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut u16,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetUInt16Array(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetInt32Array<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetInt32Array(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetUInt32Array<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetUInt32Array(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetInt64Array<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetInt64Array(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetUInt64Array<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetUInt64Array(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetSingleArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetSingleArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetDoubleArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetDoubleArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetChar16Array<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut u16,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetChar16Array(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetBooleanArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetBooleanArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetStringArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut windows_core::HSTRING,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetStringArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetInspectableArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut windows_core::IInspectable,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetInspectableArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetGuidArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetGuidArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetDateTimeArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut DateTime,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetDateTimeArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetTimeSpanArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut TimeSpan,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetTimeSpanArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetPointArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut Point,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetPointArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetSizeArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut Size,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetSizeArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetRectArray<
            Identity: IPropertyValue_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value_array_size: *mut u32,
            value: *mut *mut Rect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetRectArray(
                    this,
                    windows_core::ArrayProxy::from_raw_parts(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    )
                    .as_array(),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertyValue, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            IsNumericScalar: IsNumericScalar::<Identity, OFFSET>,
            GetUInt8: GetUInt8::<Identity, OFFSET>,
            GetInt16: GetInt16::<Identity, OFFSET>,
            GetUInt16: GetUInt16::<Identity, OFFSET>,
            GetInt32: GetInt32::<Identity, OFFSET>,
            GetUInt32: GetUInt32::<Identity, OFFSET>,
            GetInt64: GetInt64::<Identity, OFFSET>,
            GetUInt64: GetUInt64::<Identity, OFFSET>,
            GetSingle: GetSingle::<Identity, OFFSET>,
            GetDouble: GetDouble::<Identity, OFFSET>,
            GetChar16: GetChar16::<Identity, OFFSET>,
            GetBoolean: GetBoolean::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetGuid: GetGuid::<Identity, OFFSET>,
            GetDateTime: GetDateTime::<Identity, OFFSET>,
            GetTimeSpan: GetTimeSpan::<Identity, OFFSET>,
            GetPoint: GetPoint::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetRect: GetRect::<Identity, OFFSET>,
            GetUInt8Array: GetUInt8Array::<Identity, OFFSET>,
            GetInt16Array: GetInt16Array::<Identity, OFFSET>,
            GetUInt16Array: GetUInt16Array::<Identity, OFFSET>,
            GetInt32Array: GetInt32Array::<Identity, OFFSET>,
            GetUInt32Array: GetUInt32Array::<Identity, OFFSET>,
            GetInt64Array: GetInt64Array::<Identity, OFFSET>,
            GetUInt64Array: GetUInt64Array::<Identity, OFFSET>,
            GetSingleArray: GetSingleArray::<Identity, OFFSET>,
            GetDoubleArray: GetDoubleArray::<Identity, OFFSET>,
            GetChar16Array: GetChar16Array::<Identity, OFFSET>,
            GetBooleanArray: GetBooleanArray::<Identity, OFFSET>,
            GetStringArray: GetStringArray::<Identity, OFFSET>,
            GetInspectableArray: GetInspectableArray::<Identity, OFFSET>,
            GetGuidArray: GetGuidArray::<Identity, OFFSET>,
            GetDateTimeArray: GetDateTimeArray::<Identity, OFFSET>,
            GetTimeSpanArray: GetTimeSpanArray::<Identity, OFFSET>,
            GetPointArray: GetPointArray::<Identity, OFFSET>,
            GetSizeArray: GetSizeArray::<Identity, OFFSET>,
            GetRectArray: GetRectArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyValue as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PropertyType,
    ) -> windows_core::HRESULT,
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
    pub GetDateTime:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut DateTime) -> windows_core::HRESULT,
    pub GetTimeSpan:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut TimeSpan) -> windows_core::HRESULT,
    pub GetPoint:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub GetSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Size) -> windows_core::HRESULT,
    pub GetRect:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Rect) -> windows_core::HRESULT,
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
        *mut *mut DateTime,
    ) -> windows_core::HRESULT,
    pub GetTimeSpanArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut TimeSpan,
    ) -> windows_core::HRESULT,
    pub GetPointArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut Point,
    ) -> windows_core::HRESULT,
    pub GetSizeArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut Size,
    ) -> windows_core::HRESULT,
    pub GetRectArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut Rect,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
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
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IReference<T> {
    type Vtable = IReference_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IReference<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({61c17706-2d65-11e0-9ae8-d48564015472}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IPropertyValue>
    for IReference<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IReference<T> {
    pub fn Value(&self) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<PropertyType> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsNumericScalar(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNumericScalar)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt8(&self) -> windows_core::Result<u8> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt8)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetInt16(&self) -> windows_core::Result<i16> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt16)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt16(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt16)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetInt32(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt32)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt32(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt32)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetInt64(&self) -> windows_core::Result<i64> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt64)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt64(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt64)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetSingle(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSingle)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetDouble(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDouble)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetChar16(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChar16)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetBoolean(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBoolean)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGuid)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetDateTime(&self) -> windows_core::Result<DateTime> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDateTime)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetTimeSpan(&self) -> windows_core::Result<TimeSpan> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTimeSpan)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetPoint(&self) -> windows_core::Result<Point> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetSize(&self) -> windows_core::Result<Size> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetRect(&self) -> windows_core::Result<Rect> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRect)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetUInt8Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetInt16Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetUInt16Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetInt32Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetUInt32Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetInt64Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetUInt64Array)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetSingleArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetDoubleArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetChar16Array)(
                windows_core::Interface::as_raw(this),
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
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetBooleanArray)(
                windows_core::Interface::as_raw(this),
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
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetStringArray)(
                windows_core::Interface::as_raw(this),
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
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetInspectableArray)(
                windows_core::Interface::as_raw(this),
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
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetGuidArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetDateTimeArray(
        &self,
        value: &mut windows_core::Array<DateTime>,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetDateTimeArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetTimeSpanArray(
        &self,
        value: &mut windows_core::Array<TimeSpan>,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetTimeSpanArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetPointArray(
        &self,
        value: &mut windows_core::Array<Point>,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetPointArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetSizeArray(&self, value: &mut windows_core::Array<Size>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetSizeArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetRectArray(&self, value: &mut windows_core::Array<Rect>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).GetRectArray)(
                windows_core::Interface::as_raw(this),
                value.set_abi_len(),
                value as *mut _ as _,
            )
            .ok()
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IReference<T> {
    const NAME: &'static str = "Windows.Foundation.IReference";
}
pub trait IReference_Impl<T>: IPropertyValue_Impl
where
    T: windows_core::RuntimeType + 'static,
{
    fn Value(&self) -> windows_core::Result<T>;
}
impl<T: windows_core::RuntimeType + 'static> IReference_Vtbl<T> {
    pub const fn new<Identity: IReference_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<
            T: windows_core::RuntimeType + 'static,
            Identity: IReference_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReference_Impl::Value(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IReference<T>, OFFSET>(),
            Value: Value::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReference<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReference_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub X: f32,
    pub Y: f32,
}
impl windows_core::TypeKind for Point {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Point {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PropertyType(pub i32);
impl PropertyType {
    pub const Empty: Self = Self(0i32);
    pub const UInt8: Self = Self(1i32);
    pub const Int16: Self = Self(2i32);
    pub const UInt16: Self = Self(3i32);
    pub const Int32: Self = Self(4i32);
    pub const UInt32: Self = Self(5i32);
    pub const Int64: Self = Self(6i32);
    pub const UInt64: Self = Self(7i32);
    pub const Single: Self = Self(8i32);
    pub const Double: Self = Self(9i32);
    pub const Char16: Self = Self(10i32);
    pub const Boolean: Self = Self(11i32);
    pub const String: Self = Self(12i32);
    pub const Inspectable: Self = Self(13i32);
    pub const DateTime: Self = Self(14i32);
    pub const TimeSpan: Self = Self(15i32);
    pub const Guid: Self = Self(16i32);
    pub const Point: Self = Self(17i32);
    pub const Size: Self = Self(18i32);
    pub const Rect: Self = Self(19i32);
    pub const OtherType: Self = Self(20i32);
    pub const UInt8Array: Self = Self(1025i32);
    pub const Int16Array: Self = Self(1026i32);
    pub const UInt16Array: Self = Self(1027i32);
    pub const Int32Array: Self = Self(1028i32);
    pub const UInt32Array: Self = Self(1029i32);
    pub const Int64Array: Self = Self(1030i32);
    pub const UInt64Array: Self = Self(1031i32);
    pub const SingleArray: Self = Self(1032i32);
    pub const DoubleArray: Self = Self(1033i32);
    pub const Char16Array: Self = Self(1034i32);
    pub const BooleanArray: Self = Self(1035i32);
    pub const StringArray: Self = Self(1036i32);
    pub const InspectableArray: Self = Self(1037i32);
    pub const DateTimeArray: Self = Self(1038i32);
    pub const TimeSpanArray: Self = Self(1039i32);
    pub const GuidArray: Self = Self(1040i32);
    pub const PointArray: Self = Self(1041i32);
    pub const SizeArray: Self = Self(1042i32);
    pub const RectArray: Self = Self(1043i32);
    pub const OtherTypeArray: Self = Self(1044i32);
}
impl windows_core::TypeKind for PropertyType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PropertyType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.PropertyType;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl windows_core::TypeKind for Rect {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Rect {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub Width: f32,
    pub Height: f32,
}
impl windows_core::TypeKind for Size {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Size {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TimeSpan {
    pub Duration: i64,
}
impl windows_core::TypeKind for TimeSpan {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TimeSpan {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
}
