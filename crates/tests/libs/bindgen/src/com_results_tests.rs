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
    fn Status(&self, status: HRESULT) -> StatusCode {
        status
    }

    fn Read(&self, status: HRESULT, value: *mut u32) -> HRESULT {
        if status != E_FAIL {
            unsafe { value.write(42) };
        }
        status
    }

    fn ExplicitObject(&self, status: HRESULT, value: OutRef<Tracked>) -> HRESULT {
        value.write(self.value().unwrap()).unwrap();
        status
    }

    fn Object(&self) -> windows_core::Result<ObjectAlias> {
        self.value()?.ok_or_else(windows_core::Error::empty)
    }

    fn Reserved(&self, reserved: usize) -> windows_core::Result<Tracked> {
        assert_eq!(reserved, 0);
        self.Object()
    }

    fn Required(&self) -> windows_core::Result<Tracked> {
        self.value()?.ok_or_else(|| E_FAIL.into())
    }

    fn Number(&self) -> windows_core::Result<u32> {
        Err(E_FAIL.into())
    }

    fn ObjectVoid(&self, value: OutRef<Tracked>) {
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

    fn InOutQuery(&self, iid: *const GUID, value: *mut *mut core::ffi::c_void) -> HRESULT {
        unsafe {
            assert_eq!(*iid, Tracked::IID);
            assert!(!(*value).is_null());
        }
        HRESULT(1)
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

    fn TypedefSlot(&self) -> windows_core::Result<Tracked> {
        self.Required()
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
fn native_results_preserve_status_and_explicit_outputs() {
    let mode = Rc::new(Cell::new(Mode::Value));
    let drops = Rc::new(Cell::new(0));
    let object = provider(&mode, &drops);
    unsafe {
        for (index, status) in [
            HRESULT(0),
            HRESULT(1),
            E_FAIL,
            HRESULT(0x887A0002u32 as i32),
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(object.Status(status), status);
            let mut value = 17;
            assert_eq!(object.Read(status, &mut value), status);
            assert_eq!(value, if status == E_FAIL { 17 } else { 42 });
            let mut value = None;
            assert_eq!(object.ExplicitObject(status, &mut value), status);
            assert!(value.is_some());
            drop(value);
            assert_eq!(drops.get(), index as u32 + 1);
        }
        mode.set(Mode::Null);
        let mut value = None;
        assert_eq!(object.ExplicitObject(HRESULT(1), &mut value), HRESULT(1));
        assert!(value.is_none());
    }
}

#[test]
fn results_transfer_owned_references() {
    let mode = Rc::new(Cell::new(Mode::Value));
    let drops = Rc::new(Cell::new(0));
    let result = provider(&mode, &drops);
    unsafe {
        drop(result.Object().unwrap());
        assert_eq!(drops.get(), 1);
        drop(result.Required().unwrap());
        assert_eq!(drops.get(), 2);
        drop(result.ObjectVoid().unwrap());
        assert_eq!(drops.get(), 3);
        drop(result.Query::<Tracked>().unwrap());
        assert_eq!(drops.get(), 4);
        let mut optional = None;
        result.OptionalStorage::<Tracked>(&mut optional).unwrap();
        drop(optional.unwrap());
        assert_eq!(drops.get(), 5);
        drop(result.Reserved(None).unwrap());
        assert_eq!(drops.get(), 6);
        drop(result.TypedefSlot().unwrap());
        assert_eq!(drops.get(), 7);

        mode.set(Mode::Null);
        assert_eq!(result.Object().unwrap_err().code(), HRESULT(0));
        assert_eq!(result.Reserved(None).unwrap_err().code(), HRESULT(0));
        assert_eq!(result.ObjectVoid().unwrap_err().code(), HRESULT(0));
        assert_eq!(result.Query::<Tracked>().unwrap_err().code(), HRESULT(0));
        let mut optional = None;
        result.OptionalStorage::<Tracked>(&mut optional).unwrap();
        assert!(optional.is_none());
        result
            .OptionalStorage::<Tracked>(core::ptr::null_mut())
            .unwrap();
        assert!(result.Data().unwrap().is_null());
        assert_eq!(drops.get(), 7);

        mode.set(Mode::Failure);
        assert_eq!(result.Object().unwrap_err().code(), E_FAIL);
        assert_eq!(result.Query::<Tracked>().unwrap_err().code(), E_FAIL);
    }
}

#[test]
fn producer_failure_clears_result_slots_without_metadata() {
    let result = provider(&Rc::new(Cell::new(Mode::Failure)), &Rc::new(Cell::new(0)));
    let vtable = result.vtable();
    for method in [vtable.Object, vtable.Required] {
        let sentinel = core::ptr::without_provenance_mut(1);
        let mut value = sentinel;
        let hr = unsafe { method(result.as_raw(), &mut value) };
        assert_eq!(hr, E_FAIL);
        assert!(value.is_null());
    }
    let mut number = 99;
    assert_eq!(
        unsafe { (vtable.Number)(result.as_raw(), &mut number) },
        E_FAIL
    );
    assert_eq!(number, 0);
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
fn winrt_failure_clears_results_but_preserves_partial_outputs() {
    use super::interface_void::{Interface, Interface_Impl};

    #[implement(Interface)]
    struct Failure;

    impl Interface_Impl for Failure_Impl {
        fn Execute(&self) -> windows_core::Result<()> {
            Err(E_FAIL.into())
        }

        fn Method(&self) -> windows_core::Result<i32> {
            Err(E_FAIL.into())
        }

        fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
            Err(E_FAIL.into())
        }

        fn Values(&self) -> windows_core::Result<windows_core::Array<u8>> {
            Err(E_FAIL.into())
        }

        fn Partial(&self, value: &mut u32) -> windows_core::Result<u32> {
            *value = 17;
            Err(E_FAIL.into())
        }
    }

    let object: Interface = Failure.into();
    let vtable = object.vtable();
    let mut scalar = -1;
    let mut text = core::ptr::without_provenance_mut(1);
    let mut values = core::ptr::without_provenance_mut(1);
    let mut count = 99;
    let mut partial = 0;
    let mut result = 99;
    unsafe {
        assert_eq!((vtable.Method)(object.as_raw(), &mut scalar), E_FAIL);
        assert_eq!((vtable.Text)(object.as_raw(), &mut text), E_FAIL);
        assert_eq!(
            (vtable.Values)(object.as_raw(), &mut count, &mut values),
            E_FAIL
        );
        assert_eq!(
            (vtable.Partial)(object.as_raw(), &mut partial, &mut result),
            E_FAIL
        );
    }
    assert_eq!(scalar, 0);
    assert!(text.is_null());
    assert!(values.is_null());
    assert_eq!(count, 0);
    assert_eq!(partial, 17);
    assert_eq!(result, 0);
}

#[test]
fn inout_query_keeps_incoming_storage() {
    let drops = Rc::new(Cell::new(0));
    let result = provider(&Rc::new(Cell::new(Mode::Value)), &drops);
    let value: Tracked = Value(drops.clone()).into();
    let mut raw = value.as_raw();
    assert_eq!(
        unsafe { result.InOutQuery(&Tracked::IID, &mut raw) },
        HRESULT(1)
    );
    assert_eq!(raw, value.as_raw());
    drop(value);
    assert_eq!(drops.get(), 1);
}

#[test]
fn null_interface_results_use_empty_errors() {
    unsafe extern "system" fn absent(
        _: *mut core::ffi::c_void,
        value: *mut *mut core::ffi::c_void,
    ) -> HRESULT {
        unsafe { value.write(core::ptr::null_mut()) };
        HRESULT(1)
    }
    let mut vtable = Results_Vtbl::new::<Provider_Impl, 0>();
    vtable.Object = absent;
    vtable.Required = absent;
    // Only the replaced slots are called; generated identity callbacks do not use this object.
    let object = &vtable as *const _;
    let result = std::mem::ManuallyDrop::new(unsafe {
        Results::from_raw((&object as *const *const Results_Vtbl).cast_mut().cast())
    });
    assert_eq!(unsafe { result.Object() }.unwrap_err().code(), HRESULT(0));
    assert_eq!(unsafe { result.Required() }.unwrap_err().code(), HRESULT(0));
}
