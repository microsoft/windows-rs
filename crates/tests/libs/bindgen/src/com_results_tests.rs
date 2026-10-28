use super::com_results::*;
use std::{cell::Cell, rc::Rc};
use windows_core::{GUID, HRESULT, Interface, OutRef, implement};

const E_FAIL: HRESULT = HRESULT(0x80004005_u32 as i32);

#[derive(Clone, Copy)]
enum Mode {
    Value,
    Null,
    Failure,
}

#[implement(Tracked)]
struct Value(Rc<Cell<u32>>);

impl Tracked_Impl for Value_Impl {}

impl Drop for Value {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[implement(Results)]
struct Provider {
    mode: Rc<Cell<Mode>>,
    drops: Rc<Cell<u32>>,
}

impl Provider {
    fn value(&self) -> windows_core::Result<Option<Tracked>> {
        match self.mode.get() {
            Mode::Value => Ok(Some(Value(self.drops.clone()).into())),
            Mode::Null => Ok(None),
            Mode::Failure => Err(E_FAIL.into()),
        }
    }
}

impl Results_Impl for Provider_Impl {
    fn Nullable(&self) -> windows_core::Result<Option<ObjectAlias>> {
        self.value()
    }

    fn Reserved(&self, reserved: usize) -> windows_core::Result<Option<Tracked>> {
        assert_eq!(reserved, 0);
        self.value()
    }

    fn Required(&self) -> windows_core::Result<Tracked> {
        self.value()?.ok_or_else(|| E_FAIL.into())
    }

    fn UnspecifiedFailure(&self) -> windows_core::Result<Tracked> {
        self.Required()
    }

    fn NullableVoid(&self, value: OutRef<Tracked>) {
        value.write(self.value().unwrap()).unwrap();
    }

    fn Query(
        &self,
        iid: *const GUID,
        value: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            assert_eq!(*iid, Tracked::IID);
            value.write(core::ptr::null_mut());
            if let Some(result) = self.value()? {
                value.write(result.into_raw());
            }
        }
        Ok(())
    }

    fn OptionalStorage(
        &self,
        iid: *const GUID,
        value: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        if value.is_null() {
            Ok(())
        } else {
            self.Query(iid, value)
        }
    }

    fn InOutQuery(
        &self,
        iid: *const GUID,
        value: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            assert_eq!(*iid, Tracked::IID);
            assert!(!(*value).is_null());
        }
        Ok(())
    }

    fn Data(&self) -> windows_core::Result<*mut u8> {
        match self.mode.get() {
            Mode::Failure => Err(E_FAIL.into()),
            _ => Ok(core::ptr::null_mut()),
        }
    }

    fn ReadThenFail(&self, input: *const *mut core::ffi::c_void) -> windows_core::Result<Tracked> {
        assert_eq!(unsafe { *input }, core::ptr::without_provenance_mut(1));
        Err(E_FAIL.into())
    }
}

fn provider(mode: &Rc<Cell<Mode>>, drops: &Rc<Cell<u32>>) -> Results {
    Provider {
        mode: mode.clone(),
        drops: drops.clone(),
    }
    .into()
}

#[test]
fn nullable_results_transfer_owned_references() {
    let mode = Rc::new(Cell::new(Mode::Value));
    let drops = Rc::new(Cell::new(0));
    let result = provider(&mode, &drops);
    unsafe {
        drop(result.Nullable().unwrap().unwrap());
        assert_eq!(drops.get(), 1);
        drop(result.Required().unwrap());
        assert_eq!(drops.get(), 2);
        drop(result.NullableVoid().unwrap());
        assert_eq!(drops.get(), 3);
        drop(result.Query::<Tracked>().unwrap().unwrap());
        assert_eq!(drops.get(), 4);
        let mut optional = None;
        result.OptionalStorage::<Tracked>(&mut optional).unwrap();
        drop(optional.unwrap());
        assert_eq!(drops.get(), 5);
        drop(result.Reserved(None).unwrap().unwrap());
        assert_eq!(drops.get(), 6);

        mode.set(Mode::Null);
        assert!(result.Nullable().unwrap().is_none());
        assert!(result.Reserved(None).unwrap().is_none());
        assert!(result.NullableVoid().is_none());
        assert!(result.Query::<Tracked>().unwrap().is_none());
        let mut optional = None;
        result.OptionalStorage::<Tracked>(&mut optional).unwrap();
        assert!(optional.is_none());
        result
            .OptionalStorage::<Tracked>(core::ptr::null_mut())
            .unwrap();
        assert!(result.Data().unwrap().is_null());
        assert_eq!(drops.get(), 6);

        mode.set(Mode::Failure);
        assert_eq!(result.Nullable().unwrap_err().code(), E_FAIL);
        assert_eq!(result.Query::<Tracked>().unwrap_err().code(), E_FAIL);
    }
}

#[test]
fn producer_failure_initializes_only_promised_outputs() {
    let result = provider(&Rc::new(Cell::new(Mode::Failure)), &Rc::new(Cell::new(0)));
    let vtable = result.vtable();
    for (method, clears) in [
        (vtable.Nullable, true),
        (vtable.Required, true),
        (vtable.UnspecifiedFailure, false),
    ] {
        let sentinel = core::ptr::without_provenance_mut(1);
        let mut value = sentinel;
        let hr = unsafe { method(result.as_raw(), &mut value) };
        assert_eq!(hr, E_FAIL);
        assert_eq!(
            value,
            if clears {
                core::ptr::null_mut()
            } else {
                sentinel
            }
        );
    }
    let mut data = core::ptr::without_provenance_mut(1);
    assert_eq!(unsafe { (vtable.Data)(result.as_raw(), &mut data) }, E_FAIL);
    assert!(data.is_null());

    let mut value = core::ptr::without_provenance_mut(1);
    let storage = &raw mut value;
    assert_eq!(
        unsafe { (vtable.ReadThenFail)(result.as_raw(), storage, storage) },
        E_FAIL
    );
    assert!(value.is_null());
}

#[test]
fn inout_query_keeps_incoming_storage() {
    let drops = Rc::new(Cell::new(0));
    let result = provider(&Rc::new(Cell::new(Mode::Value)), &drops);
    let value: Tracked = Value(drops.clone()).into();
    let mut raw = value.as_raw();
    assert_eq!(
        unsafe { result.InOutQuery(&Tracked::IID, &mut raw) },
        HRESULT(0)
    );
    assert_eq!(raw, value.as_raw());
    drop(value);
    assert_eq!(drops.get(), 1);
}

#[test]
fn nullable_success_status_does_not_become_an_error() {
    unsafe extern "system" fn absent(
        _: *mut core::ffi::c_void,
        value: *mut *mut core::ffi::c_void,
    ) -> HRESULT {
        unsafe { value.write(core::ptr::null_mut()) };
        HRESULT(1)
    }
    let mut vtable = Results_Vtbl::new::<Provider_Impl, 0>();
    vtable.Nullable = absent;
    vtable.Required = absent;
    // Only the replaced slots are called; generated identity callbacks do not use this object.
    let object = &vtable as *const _;
    let result = std::mem::ManuallyDrop::new(unsafe {
        Results::from_raw((&object as *const *const Results_Vtbl).cast_mut().cast())
    });
    assert!(unsafe { result.Nullable() }.unwrap().is_none());
    assert!(unsafe { result.Required() }.is_err());
}
