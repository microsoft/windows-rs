pub const CCertEncodeAltName: windows_core::GUID = windows_core::GUID::from_u128(0x1cfc4cda_1271_11d1_9bd4_00c04fb683fa);
pub const CCertEncodeBitString: windows_core::GUID = windows_core::GUID::from_u128(0x6d6b3cd8_1278_11d1_9bd4_00c04fb683fa);
pub const CCertEncodeCRLDistInfo: windows_core::GUID = windows_core::GUID::from_u128(0x01fa60a0_bbff_11d0_8825_00a0c903b83c);
pub const CCertEncodeDateArray: windows_core::GUID = windows_core::GUID::from_u128(0x301f77b0_a470_11d0_8821_00a0c903b83c);
pub const CCertEncodeLongArray: windows_core::GUID = windows_core::GUID::from_u128(0x4e0680a0_a0a2_11d0_8821_00a0c903b83c);
pub const CCertEncodeStringArray: windows_core::GUID = windows_core::GUID::from_u128(0x19a76fe0_7494_11d0_8816_00a0c903b83c);
pub const EANR_SUPPRESS_IA5CONVERSION: u32 = 2147483648;
pub const EAN_NAMEOBJECTID: u32 = 2147483648;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeAltName, ICertEncodeAltName_Vtbl, 0x1c9a8c70_1271_11d1_9bd4_00c04fb683fa);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeAltName {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeAltName, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ICertEncodeAltName {
    pub unsafe fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strbinary)) }
    }
    pub unsafe fn GetNameCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNameChoice(&self, nameindex: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNameChoice)(windows_core::Interface::as_raw(self), nameindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetName(&self, nameindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), nameindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Reset(&self, namecount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), namecount) }
    }
    pub unsafe fn SetNameEntry(&self, nameindex: i32, namechoice: i32, strname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNameEntry)(windows_core::Interface::as_raw(self), nameindex, namechoice, core::mem::transmute_copy(strname)) }
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeAltName_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetNameChoice: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetNameEntry: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeAltName_Impl: super::oaidl::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetNameCount(&self) -> windows_core::Result<i32>;
    fn GetNameChoice(&self, nameindex: i32) -> windows_core::Result<i32>;
    fn GetName(&self, nameindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Reset(&self, namecount: i32) -> windows_core::Result<()>;
    fn SetNameEntry(&self, nameindex: i32, namechoice: i32, strname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeAltName_Vtbl {
    pub const fn new<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Decode<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeAltName_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
            }
        }
        unsafe extern "system" fn GetNameCount<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamecount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeAltName_Impl::GetNameCount(this) {
                    Ok(ok__) => {
                        pnamecount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNameChoice<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nameindex: i32, pnamechoice: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeAltName_Impl::GetNameChoice(this, core::mem::transmute_copy(&nameindex)) {
                    Ok(ok__) => {
                        pnamechoice.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nameindex: i32, pstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeAltName_Impl::GetName(this, core::mem::transmute_copy(&nameindex)) {
                    Ok(ok__) => {
                        pstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namecount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeAltName_Impl::Reset(this, core::mem::transmute_copy(&namecount)).into()
            }
        }
        unsafe extern "system" fn SetNameEntry<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nameindex: i32, namechoice: i32, strname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeAltName_Impl::SetNameEntry(this, core::mem::transmute_copy(&nameindex), core::mem::transmute_copy(&namechoice), core::mem::transmute(&strname)).into()
            }
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbinary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeAltName_Impl::Encode(this) {
                    Ok(ok__) => {
                        pstrbinary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetNameCount: GetNameCount::<Identity, OFFSET>,
            GetNameChoice: GetNameChoice::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetNameEntry: SetNameEntry::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeAltName as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeAltName {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeAltName2, ICertEncodeAltName2_Vtbl, 0xf67fe177_5ef1_4535_b4ce_29df15e2e0c3);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeAltName2 {
    type Target = ICertEncodeAltName;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeAltName2, windows_core::IUnknown, super::oaidl::IDispatch, ICertEncodeAltName);
#[cfg(feature = "oaidl")]
impl ICertEncodeAltName2 {
    #[cfg(feature = "certenroll")]
    pub unsafe fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strencodeddata), encoding) }
    }
    #[cfg(feature = "certenroll")]
    pub unsafe fn EncodeBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "certenroll")]
    pub unsafe fn GetNameBlob(&self, nameindex: i32, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNameBlob)(windows_core::Interface::as_raw(self), nameindex, encoding, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "certenroll")]
    pub unsafe fn SetNameEntryBlob(&self, nameindex: i32, namechoice: i32, strname: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNameEntryBlob)(windows_core::Interface::as_raw(self), nameindex, namechoice, core::mem::transmute_copy(strname), encoding) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeAltName2_Vtbl {
    pub base__: ICertEncodeAltName_Vtbl,
    #[cfg(feature = "certenroll")]
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::certenroll::EncodingType) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    DecodeBlob: usize,
    #[cfg(feature = "certenroll")]
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, super::certenroll::EncodingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    EncodeBlob: usize,
    #[cfg(feature = "certenroll")]
    pub GetNameBlob: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::certenroll::EncodingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    GetNameBlob: usize,
    #[cfg(feature = "certenroll")]
    pub SetNameEntryBlob: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, super::certenroll::EncodingType) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    SetNameEntryBlob: usize,
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeAltName2_Impl: ICertEncodeAltName_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn GetNameBlob(&self, nameindex: i32, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn SetNameEntryBlob(&self, nameindex: i32, namechoice: i32, strname: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::Result<()>;
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeAltName2_Vtbl {
    pub const fn new<Identity: ICertEncodeAltName2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeAltName2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeAltName2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
            }
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeAltName2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType, pstrencodeddata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeAltName2_Impl::EncodeBlob(this, core::mem::transmute_copy(&encoding)) {
                    Ok(ok__) => {
                        pstrencodeddata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNameBlob<Identity: ICertEncodeAltName2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nameindex: i32, encoding: super::certenroll::EncodingType, pstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeAltName2_Impl::GetNameBlob(this, core::mem::transmute_copy(&nameindex), core::mem::transmute_copy(&encoding)) {
                    Ok(ok__) => {
                        pstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNameEntryBlob<Identity: ICertEncodeAltName2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nameindex: i32, namechoice: i32, strname: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeAltName2_Impl::SetNameEntryBlob(this, core::mem::transmute_copy(&nameindex), core::mem::transmute_copy(&namechoice), core::mem::transmute(&strname), core::mem::transmute_copy(&encoding)).into()
            }
        }
        Self {
            base__: ICertEncodeAltName_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
            GetNameBlob: GetNameBlob::<Identity, OFFSET>,
            SetNameEntryBlob: SetNameEntryBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeAltName2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeAltName as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeAltName2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeBitString, ICertEncodeBitString_Vtbl, 0x6db525be_1278_11d1_9bd4_00c04fb683fa);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeBitString {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeBitString, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ICertEncodeBitString {
    pub unsafe fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strbinary)) }
    }
    pub unsafe fn GetBitCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBitString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Encode(&self, bitcount: i32, strbitstring: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), bitcount, core::mem::transmute_copy(strbitstring), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeBitString_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBitCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetBitString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeBitString_Impl: super::oaidl::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBitCount(&self) -> windows_core::Result<i32>;
    fn GetBitString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Encode(&self, bitcount: i32, strbitstring: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeBitString_Vtbl {
    pub const fn new<Identity: ICertEncodeBitString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Decode<Identity: ICertEncodeBitString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeBitString_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
            }
        }
        unsafe extern "system" fn GetBitCount<Identity: ICertEncodeBitString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeBitString_Impl::GetBitCount(this) {
                    Ok(ok__) => {
                        pbitcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBitString<Identity: ICertEncodeBitString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbitstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeBitString_Impl::GetBitString(this) {
                    Ok(ok__) => {
                        pstrbitstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeBitString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitcount: i32, strbitstring: *mut core::ffi::c_void, pstrbinary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeBitString_Impl::Encode(this, core::mem::transmute_copy(&bitcount), core::mem::transmute(&strbitstring)) {
                    Ok(ok__) => {
                        pstrbinary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetBitCount: GetBitCount::<Identity, OFFSET>,
            GetBitString: GetBitString::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeBitString as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeBitString {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeBitString2, ICertEncodeBitString2_Vtbl, 0xe070d6e7_23ef_4dd2_8242_ebd9c928cb30);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeBitString2 {
    type Target = ICertEncodeBitString;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeBitString2, windows_core::IUnknown, super::oaidl::IDispatch, ICertEncodeBitString);
#[cfg(feature = "oaidl")]
impl ICertEncodeBitString2 {
    #[cfg(feature = "certenroll")]
    pub unsafe fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strencodeddata), encoding) }
    }
    #[cfg(feature = "certenroll")]
    pub unsafe fn EncodeBlob(&self, bitcount: i32, strbitstring: &windows_core::BSTR, encodingin: super::certenroll::EncodingType, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), bitcount, core::mem::transmute_copy(strbitstring), encodingin, encoding, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "certenroll")]
    pub unsafe fn GetBitStringBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitStringBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeBitString2_Vtbl {
    pub base__: ICertEncodeBitString_Vtbl,
    #[cfg(feature = "certenroll")]
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::certenroll::EncodingType) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    DecodeBlob: usize,
    #[cfg(feature = "certenroll")]
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::certenroll::EncodingType, super::certenroll::EncodingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    EncodeBlob: usize,
    #[cfg(feature = "certenroll")]
    pub GetBitStringBlob: unsafe extern "system" fn(*mut core::ffi::c_void, super::certenroll::EncodingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    GetBitStringBlob: usize,
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeBitString2_Impl: ICertEncodeBitString_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, bitcount: i32, strbitstring: &windows_core::BSTR, encodingin: super::certenroll::EncodingType, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn GetBitStringBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeBitString2_Vtbl {
    pub const fn new<Identity: ICertEncodeBitString2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeBitString2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeBitString2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
            }
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeBitString2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitcount: i32, strbitstring: *mut core::ffi::c_void, encodingin: super::certenroll::EncodingType, encoding: super::certenroll::EncodingType, pstrencodeddata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeBitString2_Impl::EncodeBlob(this, core::mem::transmute_copy(&bitcount), core::mem::transmute(&strbitstring), core::mem::transmute_copy(&encodingin), core::mem::transmute_copy(&encoding)) {
                    Ok(ok__) => {
                        pstrencodeddata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBitStringBlob<Identity: ICertEncodeBitString2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType, pstrbitstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeBitString2_Impl::GetBitStringBlob(this, core::mem::transmute_copy(&encoding)) {
                    Ok(ok__) => {
                        pstrbitstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICertEncodeBitString_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
            GetBitStringBlob: GetBitStringBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeBitString2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeBitString as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeBitString2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeCRLDistInfo, ICertEncodeCRLDistInfo_Vtbl, 0x01958640_bbff_11d0_8825_00a0c903b83c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeCRLDistInfo {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeCRLDistInfo, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ICertEncodeCRLDistInfo {
    pub unsafe fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strbinary)) }
    }
    pub unsafe fn GetDistPointCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDistPointCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNameCount(&self, distpointindex: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNameCount)(windows_core::Interface::as_raw(self), distpointindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNameChoice(&self, distpointindex: i32, nameindex: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNameChoice)(windows_core::Interface::as_raw(self), distpointindex, nameindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetName(&self, distpointindex: i32, nameindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), distpointindex, nameindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Reset(&self, distpointcount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), distpointcount) }
    }
    pub unsafe fn SetNameCount(&self, distpointindex: i32, namecount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNameCount)(windows_core::Interface::as_raw(self), distpointindex, namecount) }
    }
    pub unsafe fn SetNameEntry(&self, distpointindex: i32, nameindex: i32, namechoice: i32, strname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNameEntry)(windows_core::Interface::as_raw(self), distpointindex, nameindex, namechoice, core::mem::transmute_copy(strname)) }
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeCRLDistInfo_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDistPointCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetNameChoice: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetNameEntry: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeCRLDistInfo_Impl: super::oaidl::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDistPointCount(&self) -> windows_core::Result<i32>;
    fn GetNameCount(&self, distpointindex: i32) -> windows_core::Result<i32>;
    fn GetNameChoice(&self, distpointindex: i32, nameindex: i32) -> windows_core::Result<i32>;
    fn GetName(&self, distpointindex: i32, nameindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Reset(&self, distpointcount: i32) -> windows_core::Result<()>;
    fn SetNameCount(&self, distpointindex: i32, namecount: i32) -> windows_core::Result<()>;
    fn SetNameEntry(&self, distpointindex: i32, nameindex: i32, namechoice: i32, strname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeCRLDistInfo_Vtbl {
    pub const fn new<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Decode<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeCRLDistInfo_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
            }
        }
        unsafe extern "system" fn GetDistPointCount<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdistpointcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeCRLDistInfo_Impl::GetDistPointCount(this) {
                    Ok(ok__) => {
                        pdistpointcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNameCount<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointindex: i32, pnamecount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeCRLDistInfo_Impl::GetNameCount(this, core::mem::transmute_copy(&distpointindex)) {
                    Ok(ok__) => {
                        pnamecount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNameChoice<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointindex: i32, nameindex: i32, pnamechoice: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeCRLDistInfo_Impl::GetNameChoice(this, core::mem::transmute_copy(&distpointindex), core::mem::transmute_copy(&nameindex)) {
                    Ok(ok__) => {
                        pnamechoice.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointindex: i32, nameindex: i32, pstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeCRLDistInfo_Impl::GetName(this, core::mem::transmute_copy(&distpointindex), core::mem::transmute_copy(&nameindex)) {
                    Ok(ok__) => {
                        pstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointcount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeCRLDistInfo_Impl::Reset(this, core::mem::transmute_copy(&distpointcount)).into()
            }
        }
        unsafe extern "system" fn SetNameCount<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointindex: i32, namecount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeCRLDistInfo_Impl::SetNameCount(this, core::mem::transmute_copy(&distpointindex), core::mem::transmute_copy(&namecount)).into()
            }
        }
        unsafe extern "system" fn SetNameEntry<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointindex: i32, nameindex: i32, namechoice: i32, strname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeCRLDistInfo_Impl::SetNameEntry(this, core::mem::transmute_copy(&distpointindex), core::mem::transmute_copy(&nameindex), core::mem::transmute_copy(&namechoice), core::mem::transmute(&strname)).into()
            }
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbinary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeCRLDistInfo_Impl::Encode(this) {
                    Ok(ok__) => {
                        pstrbinary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetDistPointCount: GetDistPointCount::<Identity, OFFSET>,
            GetNameCount: GetNameCount::<Identity, OFFSET>,
            GetNameChoice: GetNameChoice::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetNameCount: SetNameCount::<Identity, OFFSET>,
            SetNameEntry: SetNameEntry::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeCRLDistInfo as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeCRLDistInfo {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeCRLDistInfo2, ICertEncodeCRLDistInfo2_Vtbl, 0xb4275d4b_3e30_446f_ad36_09d03120b078);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeCRLDistInfo2 {
    type Target = ICertEncodeCRLDistInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeCRLDistInfo2, windows_core::IUnknown, super::oaidl::IDispatch, ICertEncodeCRLDistInfo);
#[cfg(feature = "oaidl")]
impl ICertEncodeCRLDistInfo2 {
    #[cfg(feature = "certenroll")]
    pub unsafe fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strencodeddata), encoding) }
    }
    #[cfg(feature = "certenroll")]
    pub unsafe fn EncodeBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeCRLDistInfo2_Vtbl {
    pub base__: ICertEncodeCRLDistInfo_Vtbl,
    #[cfg(feature = "certenroll")]
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::certenroll::EncodingType) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    DecodeBlob: usize,
    #[cfg(feature = "certenroll")]
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, super::certenroll::EncodingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    EncodeBlob: usize,
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeCRLDistInfo2_Impl: ICertEncodeCRLDistInfo_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeCRLDistInfo2_Vtbl {
    pub const fn new<Identity: ICertEncodeCRLDistInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeCRLDistInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeCRLDistInfo2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
            }
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeCRLDistInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType, pstrencodeddata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeCRLDistInfo2_Impl::EncodeBlob(this, core::mem::transmute_copy(&encoding)) {
                    Ok(ok__) => {
                        pstrencodeddata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICertEncodeCRLDistInfo_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeCRLDistInfo2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeCRLDistInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeCRLDistInfo2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeDateArray, ICertEncodeDateArray_Vtbl, 0x2f9469a0_a470_11d0_8821_00a0c903b83c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeDateArray {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeDateArray, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ICertEncodeDateArray {
    pub unsafe fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strbinary)) }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetValue(&self, index: i32) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self, count: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), count) }
    }
    pub unsafe fn SetValue(&self, index: i32, value: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), index, value) }
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeDateArray_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut f64) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, f64) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeDateArray_Impl: super::oaidl::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetValue(&self, index: i32) -> windows_core::Result<f64>;
    fn Reset(&self, count: i32) -> windows_core::Result<()>;
    fn SetValue(&self, index: i32, value: f64) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeDateArray_Vtbl {
    pub const fn new<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Decode<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeDateArray_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeDateArray_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvalue: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeDateArray_Impl::GetValue(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeDateArray_Impl::Reset(this, core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeDateArray_Impl::SetValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbinary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeDateArray_Impl::Encode(this) {
                    Ok(ok__) => {
                        pstrbinary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeDateArray as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeDateArray {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeDateArray2, ICertEncodeDateArray2_Vtbl, 0x99a4edb5_2b8e_448d_bf95_bba8d7789dc8);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeDateArray2 {
    type Target = ICertEncodeDateArray;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeDateArray2, windows_core::IUnknown, super::oaidl::IDispatch, ICertEncodeDateArray);
#[cfg(feature = "oaidl")]
impl ICertEncodeDateArray2 {
    #[cfg(feature = "certenroll")]
    pub unsafe fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strencodeddata), encoding) }
    }
    #[cfg(feature = "certenroll")]
    pub unsafe fn EncodeBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeDateArray2_Vtbl {
    pub base__: ICertEncodeDateArray_Vtbl,
    #[cfg(feature = "certenroll")]
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::certenroll::EncodingType) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    DecodeBlob: usize,
    #[cfg(feature = "certenroll")]
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, super::certenroll::EncodingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    EncodeBlob: usize,
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeDateArray2_Impl: ICertEncodeDateArray_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeDateArray2_Vtbl {
    pub const fn new<Identity: ICertEncodeDateArray2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeDateArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeDateArray2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
            }
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeDateArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType, pstrencodeddata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeDateArray2_Impl::EncodeBlob(this, core::mem::transmute_copy(&encoding)) {
                    Ok(ok__) => {
                        pstrencodeddata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICertEncodeDateArray_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeDateArray2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeDateArray as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeDateArray2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeLongArray, ICertEncodeLongArray_Vtbl, 0x15e2f230_a0a2_11d0_8821_00a0c903b83c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeLongArray {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeLongArray, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ICertEncodeLongArray {
    pub unsafe fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strbinary)) }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetValue(&self, index: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self, count: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), count) }
    }
    pub unsafe fn SetValue(&self, index: i32, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), index, value) }
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeLongArray_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeLongArray_Impl: super::oaidl::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetValue(&self, index: i32) -> windows_core::Result<i32>;
    fn Reset(&self, count: i32) -> windows_core::Result<()>;
    fn SetValue(&self, index: i32, value: i32) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeLongArray_Vtbl {
    pub const fn new<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Decode<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeLongArray_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeLongArray_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeLongArray_Impl::GetValue(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeLongArray_Impl::Reset(this, core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeLongArray_Impl::SetValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbinary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeLongArray_Impl::Encode(this) {
                    Ok(ok__) => {
                        pstrbinary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeLongArray as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeLongArray {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeLongArray2, ICertEncodeLongArray2_Vtbl, 0x4efde84a_bd9b_4fc2_a108_c347d478840f);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeLongArray2 {
    type Target = ICertEncodeLongArray;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeLongArray2, windows_core::IUnknown, super::oaidl::IDispatch, ICertEncodeLongArray);
#[cfg(feature = "oaidl")]
impl ICertEncodeLongArray2 {
    #[cfg(feature = "certenroll")]
    pub unsafe fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strencodeddata), encoding) }
    }
    #[cfg(feature = "certenroll")]
    pub unsafe fn EncodeBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeLongArray2_Vtbl {
    pub base__: ICertEncodeLongArray_Vtbl,
    #[cfg(feature = "certenroll")]
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::certenroll::EncodingType) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    DecodeBlob: usize,
    #[cfg(feature = "certenroll")]
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, super::certenroll::EncodingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    EncodeBlob: usize,
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeLongArray2_Impl: ICertEncodeLongArray_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeLongArray2_Vtbl {
    pub const fn new<Identity: ICertEncodeLongArray2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeLongArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeLongArray2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
            }
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeLongArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType, pstrencodeddata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeLongArray2_Impl::EncodeBlob(this, core::mem::transmute_copy(&encoding)) {
                    Ok(ok__) => {
                        pstrencodeddata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICertEncodeLongArray_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeLongArray2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeLongArray as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeLongArray2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeStringArray, ICertEncodeStringArray_Vtbl, 0x12a88820_7494_11d0_8816_00a0c903b83c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeStringArray {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeStringArray, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ICertEncodeStringArray {
    pub unsafe fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strbinary)) }
    }
    pub unsafe fn GetStringType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetValue(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Reset(&self, count: i32, stringtype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), count, stringtype) }
    }
    pub unsafe fn SetValue(&self, index: i32, str: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), index, core::mem::transmute_copy(str)) }
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeStringArray_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStringType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeStringArray_Impl: super::oaidl::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetStringType(&self) -> windows_core::Result<i32>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetValue(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Reset(&self, count: i32, stringtype: i32) -> windows_core::Result<()>;
    fn SetValue(&self, index: i32, str: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeStringArray_Vtbl {
    pub const fn new<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Decode<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeStringArray_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
            }
        }
        unsafe extern "system" fn GetStringType<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstringtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeStringArray_Impl::GetStringType(this) {
                    Ok(ok__) => {
                        pstringtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeStringArray_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeStringArray_Impl::GetValue(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32, stringtype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeStringArray_Impl::Reset(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&stringtype)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, str: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeStringArray_Impl::SetValue(this, core::mem::transmute_copy(&index), core::mem::transmute(&str)).into()
            }
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbinary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeStringArray_Impl::Encode(this) {
                    Ok(ok__) => {
                        pstrbinary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetStringType: GetStringType::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeStringArray as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeStringArray {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertEncodeStringArray2, ICertEncodeStringArray2_Vtbl, 0x9c680d93_9b7d_4e95_9018_4ffe10ba5ada);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertEncodeStringArray2 {
    type Target = ICertEncodeStringArray;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertEncodeStringArray2, windows_core::IUnknown, super::oaidl::IDispatch, ICertEncodeStringArray);
#[cfg(feature = "oaidl")]
impl ICertEncodeStringArray2 {
    #[cfg(feature = "certenroll")]
    pub unsafe fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strencodeddata), encoding) }
    }
    #[cfg(feature = "certenroll")]
    pub unsafe fn EncodeBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertEncodeStringArray2_Vtbl {
    pub base__: ICertEncodeStringArray_Vtbl,
    #[cfg(feature = "certenroll")]
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::certenroll::EncodingType) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    DecodeBlob: usize,
    #[cfg(feature = "certenroll")]
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, super::certenroll::EncodingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "certenroll"))]
    EncodeBlob: usize,
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertEncodeStringArray2_Impl: ICertEncodeStringArray_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: super::certenroll::EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, encoding: super::certenroll::EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertEncodeStringArray2_Vtbl {
    pub const fn new<Identity: ICertEncodeStringArray2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeStringArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertEncodeStringArray2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
            }
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeStringArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: super::certenroll::EncodingType, pstrencodeddata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertEncodeStringArray2_Impl::EncodeBlob(this, core::mem::transmute_copy(&encoding)) {
                    Ok(ok__) => {
                        pstrencodeddata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICertEncodeStringArray_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeStringArray2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeStringArray as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "certenroll", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertEncodeStringArray2 {}
