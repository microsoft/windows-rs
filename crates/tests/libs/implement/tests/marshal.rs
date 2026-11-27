#![cfg(windows)]
use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self, ThreadId},
    time::{Duration, Instant},
};
use test_implement::*;
use windows_core::*;

#[implement(IClassFactory)]
struct Factory;

impl IClassFactory_Impl for Factory_Impl {
    fn CreateInstance(
        &self,
        outer: Ref<IUnknown>,
        iid: *const GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
        unsafe {
            assert!(outer.is_null(), "aggregation not supported");
            let unknown: IUnknown = Stringable.into();
            unknown.query(iid, interface).ok()
        }
    }

    fn LockServer(&self, lock: BOOL) -> HRESULT {
        assert!(lock.as_bool());
        S_OK
    }
}

#[implement(IStringable)]
struct Stringable;

impl IStringable_Impl for Stringable_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Stringable".into())
    }
}

const CLSID_SAMPLE: GUID = GUID::from_u128(0x58d39529_4975_4440_b7a6_6d71bfb59559);

#[test]
fn test() {
    unsafe {
        let mut cookie = core::ptr::null_mut();
        CoIncrementMTAUsage(&mut cookie).unwrap();
        let factory: IClassFactory = Factory.into();
        factory.cast::<IAgileObject>().expect("IAgileObject");
        factory.cast::<IMarshal>().expect("IMarshal");

        let mut register = 0;
        CoRegisterClassObject(
            &CLSID_SAMPLE,
            &factory,
            CLSCTX_LOCAL_SERVER as u32,
            (REGCLS_AGILE | REGCLS_MULTIPLEUSE) as u32,
            &mut register,
        )
        .ok()
        .expect("CoRegisterClassObject");

        let stringable: IStringable =
            CoCreateInstance(&CLSID_SAMPLE, None, CLSCTX_LOCAL_SERVER as u32)
                .expect("CoCreateInstance");

        assert_eq!(stringable.ToString().unwrap(), "Stringable");

        let stream = CoMarshalInterThreadInterfaceInStream(&IStringable::IID, &stringable).unwrap();
        let copy: IStringable = CoUnmarshalInterface(&stream).unwrap();

        assert_eq!(copy.ToString().unwrap(), "Stringable");
        assert_eq!(stringable, copy);

        assert_eq!(copy, copy.downgrade().unwrap().upgrade().unwrap());
    }
}

struct ThreadProbe {
    owner: ThreadId,
    drops: Arc<Mutex<Vec<ThreadId>>>,
}

impl ThreadProbe {
    fn called_on_owner(&self) -> HSTRING {
        (thread::current().id() == self.owner).to_string().into()
    }
}

impl Drop for ThreadProbe {
    fn drop(&mut self) {
        self.drops.lock().unwrap().push(thread::current().id());
    }
}

#[implement(IStringable, Agile = false)]
struct ApartmentObject(ThreadProbe);

impl IStringable_Impl for ApartmentObject_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(self.0.called_on_owner())
    }
}

#[implement(IStringable)]
struct AgileObject(ThreadProbe);

impl IStringable_Impl for AgileObject_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(self.0.called_on_owner())
    }
}

fn receive_with_dispatch<T>(receive: &mpsc::Receiver<T>) -> T {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        unsafe {
            let mut message = MSG::default();
            while PeekMessageW(&mut message, None, 0, 0, PM_REMOVE as u32).as_bool() {
                _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
        match receive.try_recv() {
            Ok(value) => return value,
            Err(mpsc::TryRecvError::Empty) => {
                assert!(Instant::now() < deadline, "COM marshaling timed out");
                thread::yield_now();
            }
            Err(error) => panic!("{error}"),
        }
    }
}

fn weak_marshaling(agile: bool) {
    let _apartment = init_sta().unwrap();
    let owner = thread::current().id();
    let drops = Arc::new(Mutex::new(Vec::new()));
    let probe = ThreadProbe {
        owner,
        drops: drops.clone(),
    };
    let object: IStringable = if agile {
        AgileObject(probe).into()
    } else {
        ApartmentObject(probe).into()
    };
    let weak = object.downgrade().unwrap();
    let source = object.cast::<IWeakReferenceSource>().unwrap();
    let native_weak = unsafe { source.GetWeakReference().unwrap() };
    drop(source);
    let reference = AgileReference::new(&native_weak).unwrap();
    let (ready_send, ready_receive) = mpsc::channel();
    let (release_send, release_receive) = mpsc::channel();
    let (done_send, done_receive) = mpsc::channel();
    let worker = thread::spawn(move || {
        let _apartment = init_sta().unwrap();
        let native_weak = reference.resolve().unwrap();
        let mut object = None;
        unsafe {
            native_weak
                .Resolve(&IInspectable::IID, &mut object)
                .unwrap();
        }
        let object = object.unwrap().cast::<IStringable>().unwrap();
        ready_send.send(object.ToString().unwrap()).unwrap();
        release_receive
            .recv_timeout(Duration::from_secs(10))
            .unwrap();
        drop(object);
        // The retained weak proxy must not keep the target alive.
        let mut expired = None;
        unsafe {
            native_weak
                .Resolve(&IInspectable::IID, &mut expired)
                .unwrap();
        }
        assert!(expired.is_none());
        drop(native_weak);
        drop(reference);
        done_send.send(thread::current().id()).unwrap();
    });
    let called_on_owner = receive_with_dispatch(&ready_receive);
    drop(object);
    release_send.send(()).unwrap();
    let worker_id = receive_with_dispatch(&done_receive);
    worker.join().unwrap();
    assert_eq!(called_on_owner, if agile { "false" } else { "true" });
    assert_eq!(
        *drops.lock().unwrap(),
        [if agile { worker_id } else { owner }]
    );
    assert!(weak.upgrade().is_none());
}

#[test]
fn non_agile_weak_marshaling() {
    weak_marshaling(false);
}

#[test]
fn agile_weak_marshaling() {
    weak_marshaling(true);
}
