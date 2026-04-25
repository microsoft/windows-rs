mod bindings;
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[no_mangle]
extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "Robotics.Robot" {
        factory.write(Some(ROBOT_FACTORY.to_interface())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[no_mangle]
extern "system" fn CreateRobotFromHandle(
    handle: *mut std::ffi::c_void,
    robot: OutRef<bindings::Robot>,
) -> windows_core::HRESULT {
    robot.write(Some(Robot { handle }.into())).into()
}

static ROBOT_FACTORY: StaticComObject<RobotFactory> = RobotFactory.into_static();

#[implement(bindings::Robot, bindings::IRobotInterop)]
struct Robot {
    handle: *mut std::ffi::c_void,
}

impl bindings::IRobot_Impl for Robot_Impl {
    fn Speak(&self, message: &HSTRING) -> Result<()> {
        if self.handle.is_null() {
            println!("{}", message.display());
        } else {
            println!("{} ({:?})", message.display(), self.handle);
        }

        // Flush stdout explicitly so that output is not lost when captured via a pipe.
        let _ = std::io::Write::flush(&mut std::io::stdout());

        Ok(())
    }
}

impl bindings::IRobotInterop_Impl for Robot_Impl {
    fn Handle(&self) -> *mut std::ffi::c_void {
        self.handle
    }
}

#[implement(IActivationFactory)]
struct RobotFactory;

impl IActivationFactory_Impl for RobotFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Robot {
            handle: std::ptr::null_mut(),
        }
        .into())
    }
}
