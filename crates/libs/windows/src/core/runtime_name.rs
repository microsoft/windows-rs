use super::*;

pub trait RuntimeName {
    // TODO: needs to use ConstBuffer like RuntimeType to allow generic interfaces to have names for GetRuntimeClassName
    const NAME: &'static str;
}

pub unsafe extern "system" fn GetRuntimeClassName<T: RuntimeName>(
    _: RawPtr,
    value: *mut RawPtr,
) -> HRESULT {
    let h : HSTRING = T::NAME.into(); // TODO: should be try_into
    *value = ::core::mem::transmute(h);
    HRESULT(0)
}

pub unsafe extern "system" fn GetIids(
    _: RawPtr,
    count: *mut u32,
    values: *mut *mut GUID,
) -> ::windows::core::HRESULT {
    // Note: even if we end up implementing this in future, it still doesn't need a this pointer
    // since the data to be returned is type- not instance-specific so can be shared for all
    // interfaces.
    *count = 0;
    *values = core::ptr::null_mut();
    HRESULT(0)
}

pub unsafe extern "system" fn GetTrustLevel(_: RawPtr, value: *mut i32) -> HRESULT {
    // Note: even if we end up implementing this in future, it still doesn't need a this pointer
    // since the data to be returned is type- not instance-specific so can be shared for all
    // interfaces.
    *value = 0;
    HRESULT(0)
}
