#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub mod Windows {
    pub mod Foundation {
        windows_core::imp::define_interface!(
            IClosable,
            IClosable_Vtbl,
            0x30d5a829_7fa4_4026_83bb_d75bae4ea99e
        );
        impl windows_core::RuntimeType for IClosable {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        windows_core::imp::interface_hierarchy!(
            IClosable,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        impl IClosable {
            pub fn Close(&self) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(
                        this,
                    ))
                    .ok()
                }
            }
        }
        impl windows_core::RuntimeName for IClosable {
            const NAME: &'static str = "Windows.Foundation.IClosable";
        }
        pub trait IClosable_Impl: windows_core::IUnknownImpl {
            fn Close(&self) -> windows_core::Result<()>;
        }
        impl IClosable_Vtbl {
            pub const fn new<Identity: IClosable_Impl, const OFFSET: isize>() -> Self {
                unsafe extern "system" fn Close<Identity: IClosable_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity =
                            &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IClosable_Impl::Close(this).into()
                    }
                }
                Self {
                    base__: windows_core::IInspectable_Vtbl::new::<Identity, IClosable, OFFSET>(),
                    Close: Close::<Identity, OFFSET>,
                }
            }
            pub fn matches(iid: &windows_core::GUID) -> bool {
                iid == &<IClosable as windows_core::Interface>::IID
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IClosable_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            IMemoryBuffer,
            IMemoryBuffer_Vtbl,
            0xfbc4dd2a_245b_11e4_af98_689423260cf8
        );
        impl windows_core::RuntimeType for IMemoryBuffer {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        windows_core::imp::interface_hierarchy!(
            IMemoryBuffer,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        windows_core::imp::required_hierarchy!(IMemoryBuffer, IClosable);
        impl IMemoryBuffer {
            pub fn CreateReference(
                &self,
            ) -> windows_core::Result<
                crate::reference_dependency_full::Windows::Foundation::IMemoryBufferReference,
            > {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateReference)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .and_then(|| windows_core::Type::from_abi(result__))
                }
            }
            pub fn Close(&self) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IClosable>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(
                        this,
                    ))
                    .ok()
                }
            }
        }
        impl windows_core::RuntimeName for IMemoryBuffer {
            const NAME: &'static str = "Windows.Foundation.IMemoryBuffer";
        }
        pub trait IMemoryBuffer_Impl: IClosable_Impl {
            fn CreateReference(
                &self,
            ) -> windows_core::Result<
                crate::reference_dependency_full::Windows::Foundation::IMemoryBufferReference,
            >;
        }
        impl IMemoryBuffer_Vtbl {
            pub const fn new<Identity: IMemoryBuffer_Impl, const OFFSET: isize>() -> Self {
                unsafe extern "system" fn CreateReference<
                    Identity: IMemoryBuffer_Impl,
                    const OFFSET: isize,
                >(
                    this: *mut core::ffi::c_void,
                    result__: *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity =
                            &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IMemoryBuffer_Impl::CreateReference(this) {
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
                    base__: windows_core::IInspectable_Vtbl::new::<Identity, IMemoryBuffer, OFFSET>(
                    ),
                    CreateReference: CreateReference::<Identity, OFFSET>,
                }
            }
            pub fn matches(iid: &windows_core::GUID) -> bool {
                iid == &<IMemoryBuffer as windows_core::Interface>::IID
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IMemoryBuffer_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub CreateReference: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
    }
}
