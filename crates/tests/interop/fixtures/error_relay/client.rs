use super::*;
use ErrorRelay::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    let callback: ICallback = Callback::default().into();
    for (code, message) in [
        (E_BOUNDS, "relay-bounds"),
        (HRESULT(0x8004_a123_u32 as i32), "relay-custom"),
    ] {
        let error = widget
            .Relay(&callback, code.0, &HSTRING::from(message))
            .unwrap_err();
        assert_eq!(error.code(), code);
        assert_eq!(error.message(), message);
    }

    let (send0, receive0) = mpsc::channel();
    let (send1, receive1) = mpsc::channel();
    let checkpoints = Arc::new(AtomicUsize::new(0));
    std::thread::scope(|scope| {
        let workers: Vec<_> = [(send0, receive1), (send1, receive0)]
            .into_iter()
            .enumerate()
            .map(|(index, gate)| {
                let widget = &widget;
                let checkpoints = checkpoints.clone();
                scope.spawn(move || -> Result<()> {
                    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()? };
                    let calls = Arc::new(AtomicUsize::new(0));
                    let callback: ICallback = Callback {
                        calls: calls.clone(),
                        checkpoint: Some(Mutex::new(gate)),
                        checkpoints,
                    }
                    .into();
                    let code = HRESULT(0x8004_a123_u32 as i32 + index as i32);
                    let message = HSTRING::from(format!("thread-{index}"));
                    let result = widget
                        .RelayPreserved(&callback, code.0, &message)
                        .map_err(|error| (error.code(), error.message()));
                    let calls = calls.load(Ordering::Relaxed);
                    drop(callback);
                    unsafe { CoUninitialize() };
                    let (actual_code, actual_message) = result.unwrap_err();
                    assert_eq!(actual_code, code);
                    assert_eq!(actual_message, message.to_string_lossy());
                    assert_eq!(calls, 2);
                    Ok(())
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap()?;
        }
        Ok::<_, Error>(())
    })?;
    assert_eq!(checkpoints.load(Ordering::Relaxed), 2);
    Ok(())
}

#[implement(ICallback)]
#[derive(Default)]
struct Callback {
    calls: Arc<AtomicUsize>,
    checkpoint: Option<Mutex<(mpsc::Sender<()>, mpsc::Receiver<()>)>>,
    checkpoints: Arc<AtomicUsize>,
}

impl ICallback_Impl for Callback_Impl {
    fn Fail(&self, code: i32, message: &HSTRING) -> Result<()> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        Err(Error::new(HRESULT(code), message.to_string_lossy()))
    }

    fn Checkpoint(&self) -> Result<()> {
        let gate = self.checkpoint.as_ref().ok_or_else(Error::empty)?;
        let (send, receive) = &*gate.lock().unwrap();
        self.checkpoints.fetch_add(1, Ordering::Relaxed);
        send.send(())
            .map_err(|error| Error::new(E_FAIL, error.to_string()))?;
        receive
            .recv_timeout(Duration::from_secs(10))
            .map_err(|error| Error::new(E_FAIL, error.to_string()))
    }
}
