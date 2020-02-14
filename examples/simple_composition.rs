use com::{ com_interface, interfaces::iunknown::IUnknown, InterfacePtr, InterfaceRc };
use core::ffi::c_void;
use std::ptr;
use raw_window_handle::{RawWindowHandle, HasRawWindowHandle};
use winrt::*;
use winapi::{
    ENUM,
    shared::{
        guiddef::IID,
        minwindef::{DWORD, BOOL},
        ntdef::{HRESULT, NULL},
    },
    winrt::roapi::{RoInitialize, RO_INIT_MULTITHREADED, RO_INIT_SINGLETHREADED},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// Dispatcher Queue interop
ENUM!{enum DISPATCHERQUEUE_THREAD_TYPE {
    DQTYPE_THREAD_DEDICATED = 1,
    DQTYPE_THREAD_CURRENT = 2,
}}

ENUM!{enum DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    DQTAT_COM_NONE = 0,
    DQTAT_COM_ASTA = 1,
    DQTAT_COM_STA = 2,
}}

#[repr(C)]
struct DispatcherQueueOptions {
    size: DWORD,
    thread_type: DISPATCHERQUEUE_THREAD_TYPE,
    apartment_type: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}

#[repr(C)]
struct IDispatcherQueueController {
    _dummy: u32
}

#[link(name = "coremessaging")]
extern "stdcall" {
    fn CreateDispatcherQueueController(
        options: DispatcherQueueOptions,
        dispatcherQueueController: *mut *mut IDispatcherQueueController,
    ) -> HRESULT;
}

// ICompositorInterop
#[repr(C)]
pub struct ABIDesktopWindowTargetInterop {
    _dummy: u32
}

// Taken from windows.ui.composition.interop.h in the windows sdk
#[com_interface("35DBF59E-E3F9-45B0-81E7-FE75F4145DC9")]
trait IDesktopWindowTargetInterop: IUnknown {
    unsafe fn get_hwnd(
        &self,
        value: *mut *mut c_void,
    ) -> HRESULT;
}

// Taken from windows.ui.composition.interop.h in the windows sdk
#[com_interface("29E691FA-4567-4DCA-B319-D0F207EB6807")]
trait ICompositorDesktopInterop: IUnknown {
    unsafe fn create_desktop_window_target(
        &self,
        hwnd_target: *mut c_void,
        is_topmost: BOOL,
        result: *mut *mut ABIDesktopWindowTargetInterop,
    ) -> HRESULT;

    unsafe fn ensure_on_thread(
        &self,
        thread_id: DWORD,
    ) -> HRESULT;

    unsafe fn get_trust_level(
        &self,
        trust_level: *mut *mut c_void,
    ) -> HRESULT;
}

// main
import!(
    dependencies
        "os"
    modules
        "windows.foundation.collections"
        "windows.foundation.numerics"
        "windows.ui.composition"
);

use windows::foundation::*;
use windows::foundation::numerics::*;
use windows::ui::*;
use windows::ui::composition::*;

fn run() -> Result<()> {
    unsafe {
        // winit seems to want single threaded
        winrt::ErrorCode(RoInitialize(RO_INIT_SINGLETHREADED)).unwrap();
    };

    // TODO: Don't leak this
    let _dispatcher_queue_controller = unsafe {
        let options = {
            let mut options = DispatcherQueueOptions {
                size: 0,
                thread_type: DQTYPE_THREAD_CURRENT,
                apartment_type: DQTAT_COM_NONE,
            };
            options.size = std::mem::size_of::<DispatcherQueueOptions>() as u32;
            options
        };
        let mut dispatcher_queue_controller: *mut IDispatcherQueueController = ptr::null_mut();
        winrt::ErrorCode(CreateDispatcherQueueController(options, &mut dispatcher_queue_controller as *mut _ as *mut _)).unwrap();

        dispatcher_queue_controller
    };

    // Create our window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("simple_composition");

    // Get the window handle
    let window_handle = match window.raw_window_handle() {
        raw_window_handle::RawWindowHandle::Windows(window_handle) => window_handle,
        _ => panic!("Unsupported platform!"),
    };

    // Create our compositor and a target for our window
    let compositor = Compositor::new()?;
    let compositor_ptr = compositor.abi();
    let compositor_interop = unsafe {
        let unknown = InterfaceRc::<dyn IUnknown>::new(InterfacePtr::<dyn IUnknown>::new(compositor_ptr));
        let compositor_interop = unknown.get_interface::<dyn ICompositorDesktopInterop>().unwrap();
        compositor_interop
    };
    let (desktop_target, _desktop_target_interop) = unsafe {
        let mut interop_ptr: *mut ABIDesktopWindowTargetInterop = ptr::null_mut();
        winrt::ErrorCode(compositor_interop.create_desktop_window_target(window_handle.hwnd, 0, &mut interop_ptr as *mut _ as *mut _)).unwrap();
        let unknown = InterfaceRc::<dyn IUnknown>::new(InterfacePtr::<dyn IUnknown>::new(interop_ptr as *mut c_void));
        let desktop_target_interop = unknown.get_interface::<dyn IDesktopWindowTargetInterop>().unwrap();

        let winrt_unknown = winrt::ComPtr::addref(interop_ptr as *mut c_void);
        let desktop_target = winrt_unknown.query::<CompositionTarget>();
        let desktop_target = CompositionTarget { ptr: desktop_target };

        (desktop_target, desktop_target_interop)
    };

    // Build our visual tree
    let visual = compositor.create_sprite_visual()?;
    visual.set_relative_size_adjustment(Vector2{ x: 1.0, y: 1.0 })?;
    let brush = compositor.create_color_brush_with_color(Colors::red()?)?;
    visual.set_brush(brush)?;

    desktop_target.set_root(visual)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });

    Ok(())
}

fn main() {
    let result = run();

    let code = match result {
        Ok(_) => winrt::ErrorCode(0),
        Err(code) => code.code(),
    };
    code.unwrap();
}
