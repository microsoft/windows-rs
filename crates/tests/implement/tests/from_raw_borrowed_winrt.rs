#![allow(non_snake_case)]

use std::sync::*;
use windows::{core::*, ApplicationModel::Background::*, Foundation::*, Win32::Foundation::*};

#[interface("a563f463-3d23-42cd-a2b5-6d21ee898aae")]
unsafe trait IBorrowed: IUnknown {
    unsafe fn Call(&self) -> u32;
}

#[implement(IBackgroundTask, IBorrowed, IBackgroundTaskInstance)]
struct Borrowed(RwLock<u32>);

impl IBorrowed_Impl for Borrowed {
    unsafe fn Call(&self) -> u32 {
        *self.0.read().unwrap()
    }
}

impl IBackgroundTask_Impl for Borrowed {
    fn Run(&self, instance: Option<&IBackgroundTaskInstance>) -> Result<()> {
        if let Some(instance) = instance {
            assert_eq!(instance.SuspendedCount()?, *self.0.read().unwrap());
            Ok(())
        } else {
            Err(E_INVALIDARG.into())
        }
    }
}

impl IBackgroundTaskInstance_Impl for Borrowed {
    fn InstanceId(&self) -> Result<GUID> {
        todo!()
    }
    fn Task(&self) -> Result<BackgroundTaskRegistration> {
        todo!()
    }
    fn Progress(&self) -> Result<u32> {
        todo!()
    }
    fn SetProgress(&self, _value: u32) -> Result<()> {
        todo!()
    }
    fn TriggerDetails(&self) -> Result<IInspectable> {
        todo!()
    }
    fn Canceled(
        &self,
        _cancelhandler: Option<&BackgroundTaskCanceledEventHandler>,
    ) -> Result<EventRegistrationToken> {
        todo!()
    }
    fn RemoveCanceled(&self, _cookie: &EventRegistrationToken) -> Result<()> {
        todo!()
    }
    fn SuspendedCount(&self) -> Result<u32> {
        Ok(*self.0.read().unwrap())
    }
    fn GetDeferral(&self) -> Result<BackgroundTaskDeferral> {
        todo!()
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let one_two_three: IBorrowed = Borrowed(RwLock::new(123)).into();
        assert_eq!(one_two_three.Call(), 123);

        let task = one_two_three.cast::<IBackgroundTask>()?;
        let instance = one_two_three.cast::<IBackgroundTaskInstance>()?;

        assert_eq!(task.Run(None).unwrap_err().code(), E_INVALIDARG);
        task.Run(&instance)?;

        let handler = BackgroundTaskCanceledEventHandler::new(|instance, reason| {
            if let Some(instance) = instance {
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
