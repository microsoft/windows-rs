use super::*;
use windows_core::EventRevoker;

const INPUT_PROBE_SUBCLASS_ID: usize = 0x0052_5449_4E50_5554;
const WM_KEYDOWN: u32 = 0x0100;
const WM_SYSKEYDOWN: u32 = 0x0104;

type SubclassProc = Option<
    unsafe extern "system" fn(*mut std::ffi::c_void, u32, usize, isize, usize, usize) -> isize,
>;

windows_core::link!("comctl32.dll" "system" fn DefSubclassProc(hwnd: *mut std::ffi::c_void, message: u32, wparam: usize, lparam: isize) -> isize);
windows_core::link!("comctl32.dll" "system" fn RemoveWindowSubclass(hwnd: *mut std::ffi::c_void, callback: SubclassProc, id: usize) -> i32);
windows_core::link!("comctl32.dll" "system" fn SetWindowSubclass(hwnd: *mut std::ffi::c_void, callback: SubclassProc, id: usize, data: usize) -> i32);
windows_core::link!("user32.dll" "system" fn GetFocus() -> *mut std::ffi::c_void);

thread_local! {
    static INPUT_PROBE_CALLBACK: RefCell<Option<Rc<dyn Fn(LiveInputProbeStage)>>> =
        const { RefCell::new(None) };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveInputProbeStage {
    RawWindowMessage,
    InputKeyboardSource,
}

pub struct LiveInputProbe {
    input_hwnd: *mut std::ffi::c_void,
    window_hwnd: *mut std::ffi::c_void,
    _keyboard: EventRevoker,
}

impl LiveInputProbe {
    pub fn window_handle(&self) -> isize {
        self.window_hwnd as isize
    }

    pub fn retarget_raw_input(&mut self) -> windows_core::Result<()> {
        let input_hwnd = unsafe { GetFocus() };
        if input_hwnd.is_null() {
            return Err(windows_core::Error::from_thread());
        }
        if input_hwnd == self.input_hwnd {
            return Ok(());
        }

        if unsafe {
            SetWindowSubclass(
                input_hwnd,
                Some(live_input_probe_subclass),
                INPUT_PROBE_SUBCLASS_ID,
                0,
            )
        } == 0
        {
            return Err(windows_core::Error::from_thread());
        }
        unsafe {
            RemoveWindowSubclass(
                self.input_hwnd,
                Some(live_input_probe_subclass),
                INPUT_PROBE_SUBCLASS_ID,
            );
        }
        self.input_hwnd = input_hwnd;
        Ok(())
    }
}

impl Drop for LiveInputProbe {
    fn drop(&mut self) {
        unsafe {
            RemoveWindowSubclass(
                self.input_hwnd,
                Some(live_input_probe_subclass),
                INPUT_PROBE_SUBCLASS_ID,
            );
        }
        let _ = INPUT_PROBE_CALLBACK.try_with(|callback| callback.borrow_mut().take());
    }
}

unsafe extern "system" fn live_input_probe_subclass(
    hwnd: *mut std::ffi::c_void,
    message: u32,
    wparam: usize,
    lparam: isize,
    _id: usize,
    _data: usize,
) -> isize {
    if matches!(message, WM_KEYDOWN | WM_SYSKEYDOWN) {
        let _ = INPUT_PROBE_CALLBACK.try_with(|callback| {
            if let Some(callback) = callback.borrow().as_ref() {
                callback(LiveInputProbeStage::RawWindowMessage);
            }
        });
    }
    unsafe { DefSubclassProc(hwnd, message, wparam, lparam) }
}

pub(crate) fn native_window_handle(window: &Window) -> windows_core::Result<isize> {
    let mut hwnd = std::ptr::null_mut();
    unsafe {
        window
            .cast::<IWindowNative>()?
            .WindowHandle(&mut hwnd)
            .ok()?;
    }
    Ok(hwnd as isize)
}

pub(crate) fn subscribe_live_input_probe(
    window: &Window,
    callback: impl Fn(LiveInputProbeStage) + 'static,
) -> windows_core::Result<LiveInputProbe> {
    let callback: Rc<dyn Fn(LiveInputProbeStage)> = Rc::new(callback);
    let content = window.Content()?.cast::<UIElement>()?;
    let island = content.XamlRoot()?.cast::<IXamlRoot4>()?.ContentIsland()?;
    let keyboard = InputKeyboardSource::GetForIsland(&island)?
        .cast::<IInputKeyboardSource2>()?
        .KeyDown({
            let callback = Rc::clone(&callback);
            move |_, _| callback(LiveInputProbeStage::InputKeyboardSource)
        })?;
    let hwnd = native_window_handle(window)? as *mut std::ffi::c_void;

    INPUT_PROBE_CALLBACK.with(|slot| {
        assert!(
            slot.borrow_mut().replace(callback).is_none(),
            "only one live input probe may be active"
        );
    });
    if unsafe {
        SetWindowSubclass(
            hwnd,
            Some(live_input_probe_subclass),
            INPUT_PROBE_SUBCLASS_ID,
            0,
        )
    } == 0
    {
        INPUT_PROBE_CALLBACK.with(|callback| callback.borrow_mut().take());
        return Err(windows_core::Error::from_thread());
    }

    Ok(LiveInputProbe {
        input_hwnd: hwnd,
        window_hwnd: hwnd,
        _keyboard: keyboard,
    })
}

pub fn subscribe_live_rendering<F>(rendering: F) -> windows_core::Result<EventRevoker>
where
    F: Fn() + 'static,
{
    CompositionTarget::Rendering(move |_, _| {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(&rendering)).is_err() {
            std::process::abort();
        }
    })
}

pub fn schedule_live_test_exit(success: bool) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let handler = DispatcherQueueHandler::new(move || {
        std::process::exit(i32::from(!success));
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)? {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
