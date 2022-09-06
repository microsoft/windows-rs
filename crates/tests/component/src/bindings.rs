#[doc(hidden)]
#[repr(transparent)]
pub struct IClass(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClass {
    type Vtable = IClass_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce9054d_f33d_515f_b891_29e6cb101073);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClass_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Int32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_array_size: u32, a: *const i32, b_array_size: u32, b: *mut i32, c_array_size: *mut u32, c: *mut *mut i32, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::core::HRESULT,
    pub StringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_array_size: u32, a: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, b_array_size: u32, b: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>, c_array_size: *mut u32, c: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct Class(::windows::core::IUnknown);
impl Class {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Class, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Property(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Property)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetProperty(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProperty)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Int32Array(&self, a: &[i32], b: &mut [i32], c: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<::windows::core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Int32Array)(::windows::core::Interface::as_raw(this), a.len() as u32, a.as_ptr(), b.len() as u32, b.as_mut_ptr(), c.set_abi_len(), c as *mut _ as _, ::windows::core::Array::<i32>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn StringArray(&self, a: &[::windows::core::HSTRING], b: &mut [::windows::core::HSTRING], c: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StringArray)(::windows::core::Interface::as_raw(this), a.len() as u32, ::core::mem::transmute(a.as_ptr()), b.len() as u32, ::core::mem::transmute_copy(&b), c.set_abi_len(), c as *mut _ as _, ::windows::core::Array::<::windows::core::HSTRING>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::clone::Clone for Class {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Class {}
impl ::core::fmt::Debug for Class {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Class").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Class {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(test_component.Class;{4ce9054d-f33d-515f-b891-29e6cb101073})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Class {
    type Vtable = IClass_Vtbl;
    const IID: ::windows::core::GUID = <IClass as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Class {
    const NAME: &'static str = "test_component.Class";
}
impl ::core::convert::From<Class> for ::windows::core::IUnknown {
    fn from(value: Class) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Class> for ::windows::core::IUnknown {
    fn from(value: &Class) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Class> for &::windows::core::IUnknown {
    fn from(value: &Class) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<Class> for ::windows::core::IInspectable {
    fn from(value: Class) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Class> for ::windows::core::IInspectable {
    fn from(value: &Class) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Class> for &::windows::core::IInspectable {
    fn from(value: &Class) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for Class {}
unsafe impl ::core::marker::Sync for Class {}
pub trait IClass_Impl: Sized {
    fn Property(&self) -> ::windows::core::Result<i32>;
    fn SetProperty(&self, value: i32) -> ::windows::core::Result<()>;
    fn Int32Array(&self, a: &[i32], b: &mut [i32], c: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<::windows::core::Array<i32>>;
    fn StringArray(&self, a: &[::windows::core::HSTRING], b: &mut [::windows::core::HSTRING], c: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
}
impl ::windows::core::RuntimeName for IClass {
    const NAME: &'static str = "test_component.IClass";
}
impl IClass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClass_Impl, const OFFSET: isize>() -> IClass_Vtbl {
        unsafe extern "system" fn Property<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Property() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(value).into()
        }
        unsafe extern "system" fn Int32Array<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_array_size: u32, a: *const i32, b_array_size: u32, b: *mut i32, c_array_size: *mut u32, c: *mut *mut i32, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Int32Array(::core::slice::from_raw_parts(::core::mem::transmute_copy(&a), a_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&b), b_array_size as _), ::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&c), c_array_size).as_array()) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StringArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_array_size: u32, a: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, b_array_size: u32, b: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>, c_array_size: *mut u32, c: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StringArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&a), a_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&b), b_array_size as _), ::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&c), c_array_size).as_array()) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IClass, OFFSET>(),
            Property: Property::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Int32Array: Int32Array::<Identity, Impl, OFFSET>,
            StringArray: StringArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClass as ::windows::core::Interface>::IID
    }
}
