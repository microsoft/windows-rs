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
}
impl windows_core::RuntimeName for IPropertyValue {
    const NAME: &'static str = "Windows.Foundation.IPropertyValue";
}
pub trait IPropertyValue_Impl: windows_core::IUnknownImpl {
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
}
impl IPropertyValue_Vtbl {
    pub const fn new<Identity: IPropertyValue_Impl, const OFFSET: isize>() -> Self {
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
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
                    &mut windows_core::imp::array_proxy(
                        core::mem::transmute_copy(&value),
                        value_array_size,
                    ),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertyValue, OFFSET>(),
            Type: 0,
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
            GetDateTime: 0,
            GetTimeSpan: 0,
            GetPoint: 0,
            GetSize: 0,
            GetRect: 0,
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
            GetDateTimeArray: 0,
            GetTimeSpanArray: 0,
            GetPointArray: 0,
            GetSizeArray: 0,
            GetRectArray: 0,
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
    GetDateTime: usize,
    GetTimeSpan: usize,
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
    GetDateTimeArray: usize,
    GetTimeSpanArray: usize,
    GetPointArray: usize,
    GetSizeArray: usize,
    GetRectArray: usize,
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RefWithFloat {
    pub Value: Option<IReference<f32>>,
}
impl windows_core::TypeKind for RefWithFloat {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for RefWithFloat {
    const SIGNATURE :windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice ( b"struct(test_reference_float.RefWithFloat;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};f4))" ) ;
}
