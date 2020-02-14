use com::{ com_interface, interfaces::iunknown::IUnknown, InterfacePtr, InterfaceRc };
use core::ffi::c_void;
use raw_window_handle::{HasRawWindowHandle};
use winrt::*;
use winapi::{
    ENUM,
    shared::{
        minwindef::{DWORD, BOOL},
        ntdef::HRESULT,
    },
    winrt::roapi::{RoInitialize, RO_INIT_SINGLETHREADED},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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
        winrt::ErrorCode(RoInitialize(RO_INIT_SINGLETHREADED)).ok()?;
    };

    let _dispatcher_queue_controller = create_dispatcher_queue_controller_for_current_thread()?;

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
    let target = create_desktop_window_target(&compositor, window_handle.hwnd)?;

    // Build our visual tree
    let visual = compositor.create_sprite_visual()?;
    visual.set_relative_size_adjustment(Vector2{ x: 1.0, y: 1.0 })?;
    let brush = compositor.create_color_brush_with_color(Colors::red()?)?;
    visual.set_brush(brush)?;

    target.set_root(visual)?;

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

#[link(name = "coremessaging")]
extern "stdcall" {
    fn CreateDispatcherQueueController(
        options: DispatcherQueueOptions,
        dispatcherQueueController: *mut winrt::RawPtr,
    ) -> HRESULT;
}

fn create_dispatcher_queue_controller_for_current_thread() -> Result<winrt::ComPtr> {
    let options = {
        let mut options = DispatcherQueueOptions {
            size: 0,
            thread_type: DQTYPE_THREAD_CURRENT,
            apartment_type: DQTAT_COM_NONE,
        };
        options.size = std::mem::size_of::<DispatcherQueueOptions>() as u32;
        options
    };

    let dispatcher_queue_controller = unsafe {
        let mut interop_ptr = winrt::ComPtr::default();
        winrt::ErrorCode(CreateDispatcherQueueController(options, interop_ptr.set())).ok()?;       
        //let dispatcher_queue_controller = interop_ptr.query::<DispatcherQueueController>();
        //let dispatcher_queue_controller = std::mem::transmute::<winrt::ComPtr, DispatcherQueueController>(dispatcher_queue_controller);
        // dispatcher_queue_controller
        interop_ptr
    };

    Ok(dispatcher_queue_controller)
}

// Taken from windows.ui.composition.interop.h in the windows sdk
#[com_interface("29E691FA-4567-4DCA-B319-D0F207EB6807")]
trait ICompositorDesktopInterop: IUnknown {
    unsafe fn create_desktop_window_target(
        &self,
        hwnd_target: *mut c_void,
        is_topmost: BOOL,
        result: *mut winrt::RawPtr,
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

fn create_desktop_window_target(compositor: &Compositor, window_handle: *mut c_void) -> Result<CompositionTarget> {
    let compositor_ptr = compositor.abi();
    let compositor_interop = unsafe {
        let unknown = InterfaceRc::<dyn IUnknown>::new(InterfacePtr::<dyn IUnknown>::new(compositor_ptr));
        let compositor_interop = unknown.get_interface::<dyn ICompositorDesktopInterop>().unwrap();
        compositor_interop
    };
    let desktop_target = unsafe {
        let mut interop_ptr = winrt::ComPtr::default();
        winrt::ErrorCode(compositor_interop.create_desktop_window_target(window_handle, 0, interop_ptr.set())).ok()?;
        let desktop_target = interop_ptr.query::<CompositionTarget>();
        let desktop_target = std::mem::transmute::<winrt::ComPtr, CompositionTarget>(desktop_target);

        desktop_target
    };

    Ok(desktop_target)
}