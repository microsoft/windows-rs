#![allow(non_snake_case)]

use std::sync::*;
use windows::{core::*, ApplicationModel::Background::*, Win32::Foundation::*};

#[interface("a563f463-3d23-42cd-a2b5-6d21ee898aae")]
unsafe trait IBorrowed: IUnknown {
    unsafe fn Call(&self) -> u32;
}

#[implement(IBackgroundTask, IBorrowed, IBackgroundTaskInstance)]
struct Borrowed(RwLock<u32>);

impl IBorrowed_Impl for Borrowed_Impl {
    unsafe fn Call(&self) -> u32 {
        *self.0.read().unwrap()
    }
}

impl IBackgroundTask_Impl for Borrowed_Impl {
    fn Run(&self, instance: Ref<IBackgroundTaskInstance>) -> Result<(), HRESULT> {
        assert_eq!(instance.ok()?.SuspendedCount()?, *self.0.read().unwrap());
        Ok(())
    }
}

impl IBackgroundTaskInstance_Impl for Borrowed_Impl {
    fn InstanceId(&self) -> Result<GUID, HRESULT> {
        unimplemented!()
    }
    fn Task(&self) -> Result<BackgroundTaskRegistration, HRESULT> {
        unimplemented!()
    }
    fn Progress(&self) -> Result<u32, HRESULT> {
        unimplemented!()
    }
    fn SetProgress(&self, _value: u32) -> Result<(), HRESULT> {
        unimplemented!()
    }
    fn TriggerDetails(&self) -> Result<IInspectable, HRESULT> {
        unimplemented!()
    }
    fn Canceled(&self, _cancelhandler: Ref<BackgroundTaskCanceledEventHandler>) -> Result<i64, HRESULT> {
        unimplemented!()
    }
    fn RemoveCanceled(&self, _cookie: i64) -> Result<(), HRESULT> {
        unimplemented!()
    }
    fn SuspendedCount(&self) -> Result<u32, HRESULT> {
        Ok(*self.0.read().unwrap())
    }
    fn GetDeferral(&self) -> Result<BackgroundTaskDeferral, HRESULT> {
        unimplemented!()
    }
}

#[test]
fn test() -> Result<(), HRESULT> {
    unsafe {
        let one_two_three: IBorrowed = Borrowed(RwLock::new(123)).into();
        assert_eq!(one_two_three.Call(), 123);

        let task = one_two_three.cast::<IBackgroundTask>()?;
        let instance = one_two_three.cast::<IBackgroundTaskInstance>()?;

        assert_eq!(task.Run(None).unwrap_err(), E_POINTER);
        task.Run(&instance)?;

        let handler = BackgroundTaskCanceledEventHandler::new(|instance, reason| {
            if let Some(instance) = &*instance {
                assert_eq!(
                    instance.SuspendedCount()?,
                    instance.cast::<IBorrowed>()?.Call()
                );
                assert_eq!(reason, BackgroundTaskCancellationReason::Abort);
            } else {
                assert_eq!(reason, BackgroundTaskCancellationReason::Terminating);
            }
            Ok(())
        });

        handler.Invoke(&instance, BackgroundTaskCancellationReason::Abort)?;
        handler.Invoke(None, BackgroundTaskCancellationReason::Terminating)?;

        Ok(())
    }
}
