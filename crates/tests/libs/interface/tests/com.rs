#![expect(non_snake_case)]

use windows::{core::*, Win32::Foundation::*, Win32::System::Com::*};

/// A custom declaration of implementation of `IUri`
#[interface("a39ee748-6a27-4817-a6f2-13914bef5890")]
unsafe trait ICustomUri: IUnknown {
    unsafe fn GetPropertyBSTR(
        &self,
        property: Uri_PROPERTY,
        value: *mut BSTR,
        flags: u32,
    ) -> HRESULT;
    unsafe fn GetPropertyLength(&self) -> HRESULT;
    unsafe fn GetPropertyDWORD(
        &self,
        property: Uri_PROPERTY,
        value: *mut u32,
        flags: u32,
    ) -> HRESULT;
    unsafe fn HasProperty(&self); // Note: this definition is missing its return value
    unsafe fn GetAbsoluteUri(&self) -> HRESULT;
    unsafe fn GetAuthority(&self) -> HRESULT;
    unsafe fn GetDisplayUri(&self) -> i32;
    unsafe fn GetDomain(&self, value: *mut BSTR) -> HRESULT;
    // etc
}

/// A custom declaration of implementation of `IPersist`
#[interface("0000010c-0000-0000-C000-000000000046")]
unsafe trait ICustomPersist: windows::core::IUnknown {
    unsafe fn GetClassID(&self, clsid: *mut GUID) -> HRESULT;
}

/// A custom declaration of implementation of `IPersistMemory`
#[interface("BD1AE5E0-A6AE-11CE-BD37-504200C10000")]
unsafe trait ICustomPersistMemory: ICustomPersist {
    unsafe fn IsDirty(&self) -> HRESULT;
    unsafe fn Load(&self, input: *const core::ffi::c_void, size: u32) -> HRESULT;
    unsafe fn Save(&self, output: *mut core::ffi::c_void, clear_dirty: BOOL, size: u32) -> HRESULT;
    unsafe fn GetSizeMax(&self, len: *mut u32) -> HRESULT;
    unsafe fn InitNew(&self) -> HRESULT;
}

/// A custom in-memory store
#[implement(ICustomPersistMemory, ICustomPersist)]
#[derive(Default)]
struct Persist(std::sync::RwLock<PersistState>);

impl Persist {
    fn new() -> Self {
        Self(std::sync::RwLock::new(PersistState::default()))
    }
}

#[derive(Default)]
struct PersistState {
    memory: [u8; 10],
    dirty: bool,
}

impl ICustomPersist_Impl for Persist_Impl {
    unsafe fn GetClassID(&self, clsid: *mut GUID) -> HRESULT {
        unsafe {
            *clsid = "117fb826-2155-483a-b50d-bc99a2c7cca3".try_into().unwrap();
            S_OK
        }
    }
}

impl ICustomPersistMemory_Impl for Persist_Impl {
    unsafe fn IsDirty(&self) -> HRESULT {
        let reader = self.0.read().unwrap();
        if reader.dirty {
            S_OK
        } else {
            S_FALSE
        }
    }

    unsafe fn Load(&self, input: *const core::ffi::c_void, size: u32) -> HRESULT {
        let mut writer = self.0.write().unwrap();
        if size <= writer.memory.len() as u32 {
            unsafe {
                std::ptr::copy(input, writer.memory.as_mut_ptr() as _, size as usize);
            }
            writer.dirty = true;
            S_OK
        } else {
            E_OUTOFMEMORY
        }
    }

    unsafe fn Save(&self, output: *mut core::ffi::c_void, clear_dirty: BOOL, size: u32) -> HRESULT {
        let mut writer = self.0.write().unwrap();
        if size <= writer.memory.len() as u32 {
            unsafe {
                std::ptr::copy(writer.memory.as_mut_ptr() as _, output, size as usize);
            }
            if clear_dirty.as_bool() {
                writer.dirty = false;
            }
            S_OK
        } else {
            E_OUTOFMEMORY
        }
    }

    unsafe fn GetSizeMax(&self, len: *mut u32) -> HRESULT {
        let reader = self.0.read().unwrap();
        unsafe {
            *len = reader.memory.len() as u32;
        }
        S_OK
    }

    unsafe fn InitNew(&self) -> HRESULT {
        let mut writer = self.0.write().unwrap();
        writer.memory = Default::default();
        writer.dirty = false;
        S_OK
    }
}

#[test]
fn test_custom_interface() -> windows::core::Result<()> {
    unsafe {
        // Use the OS implementation of Uri through the custom `ICustomUri` interface
        let a: IUri = CreateUri(w!("http://kennykerr.ca"), URI_CREATE_FLAGS::default(), None)?;
        let b: ICustomUri = a.cast()?;
        let mut domain = BSTR::new();
        b.GetDomain(&mut domain).ok()?;
        assert_eq!(domain, "kennykerr.ca");
        let mut property = BSTR::new();
        a.GetPropertyBSTR(Uri_PROPERTY_DOMAIN, &mut property, 0)?;
        assert_eq!(property, "kennykerr.ca");
        let mut property = 0;
        a.GetPropertyDWORD(Uri_PROPERTY_PORT, &mut property, 0)?;
        assert_eq!(property, 80);

        // Use the custom implementation of `Persist` through the OS `IPersistMemory` interface
        let p: ICustomPersistMemory = Persist::new().into();
        // This works because `ICustomPersistMemory` and `IPersistMemory` share the same guid
        let p: IPersistMemory = p.cast()?;
        assert_eq!(
            p.GetClassID()?,
            "117fb826-2155-483a-b50d-bc99a2c7cca3".try_into()?,
        );
        assert_eq!(p.GetSizeMax()?, 10);
        assert_eq!(p.IsDirty(), S_FALSE);
        p.Load(&[0xAAu8, 0xBB, 0xCC])?;
        assert_eq!(p.IsDirty(), S_OK);
        let mut memory = [0x00u8, 0x00, 0x00, 0x00];
        p.Save(&mut memory, true)?;
        assert_eq!(memory, [0xAAu8, 0xBB, 0xCC, 0x00]);
        assert_eq!(p.IsDirty(), S_FALSE);

        // Use the custom implementation of `Persist` through the custom interface of `ICustomPersist`
        let p: ICustomPersistMemory = p.cast()?;
        let mut size = 0;
        p.GetSizeMax(&mut size).ok()?;
        assert_eq!(size, 10);
        assert_eq!(p.IsDirty(), S_FALSE);
        p.Load(&[0xAAu8, 0xBB, 0xCC] as *const _ as *const _, 3)
            .ok()?;
        assert_eq!(p.IsDirty(), S_OK);
        let mut memory = [0x00u8, 0x00, 0x00, 0x00];
        p.Save(&mut memory as *mut _ as *mut _, true.into(), 4)
            .ok()?;
        assert_eq!(p.IsDirty(), S_FALSE);
        assert_eq!(memory, [0xAAu8, 0xBB, 0xCC, 0x00]);

        let p: ICustomPersist = p.cast()?;
        let mut b = GUID::default();
        p.GetClassID(&mut b).ok()?;
        assert_eq!(b, "117fb826-2155-483a-b50d-bc99a2c7cca3".try_into()?);

        Ok(())
    }
}
