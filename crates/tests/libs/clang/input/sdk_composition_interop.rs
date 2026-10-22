#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]
include!("../bindings.rs");

use windows_core::{GUID, Interface};

#[windows_core::implement(ICompositionSurface)]
struct Surface;
impl ICompositionSurface_Impl for Surface_Impl {}

#[windows_core::implement(ICompositorInterop, ICompositorDesktopInterop)]
struct Compositor {
    surface: ICompositionSurface,
}

impl ICompositorInterop_Impl for Compositor_Impl {
    fn CreateCompositionSurfaceForHandle(&self, swapchain: HANDLE) -> windows_core::Result<ICompositionSurface> {
        assert_eq!(swapchain as usize, 0x1234);
        Ok(self.surface.clone())
    }

    fn CreateCompositionSurfaceForSwapChain(&self, _: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<ICompositionSurface> {
        Ok(self.surface.clone())
    }

    fn CreateGraphicsDevice(&self, _: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<ICompositionGraphicsDevice> {
        Err(windows_core::HRESULT(0x80004001_u32 as i32).into())
    }
}

impl ICompositorDesktopInterop_Impl for Compositor_Impl {
    fn CreateDesktopWindowTarget(&self, hwnd: HWND, topmost: windows_core::BOOL) -> windows_core::Result<IDesktopWindowTarget> {
        assert_eq!(hwnd as usize, 0x5678);
        assert!(topmost.as_bool());
        Err(windows_core::HRESULT(0x80004001_u32 as i32).into())
    }

    fn EnsureOnThread(&self, thread: u32) -> windows_core::Result<()> {
        assert_eq!(thread, 42);
        Ok(())
    }
}

fn main() {
    let pointer = size_of::<usize>();
    assert_eq!(size_of::<ICompositorInterop_Vtbl>(), 6 * pointer);
    assert_eq!(size_of::<ICompositionDrawingSurfaceInterop_Vtbl>(), 9 * pointer);
    assert_eq!(size_of::<ICompositionDrawingSurfaceInterop2_Vtbl>(), 10 * pointer);
    assert_eq!(size_of::<ICompositorDesktopInterop_Vtbl>(), 5 * pointer);
    assert_eq!(size_of::<IDesktopWindowTarget_Vtbl>(), 7 * pointer);
    assert_eq!(std::mem::offset_of!(ICompositorInterop_Vtbl, CreateGraphicsDevice), 5 * pointer);
    assert_eq!(std::mem::offset_of!(ICompositionDrawingSurfaceInterop_Vtbl, Resize), 5 * pointer);
    assert_eq!(size_of::<PDISPATCHERQUEUECONTROLLER>(), pointer);
    assert_eq!(size_of::<DispatcherQueueOptions>(), 12);
    assert_eq!(size_of::<SIZE>(), 8);
    assert_eq!(ICompositorInterop::IID, GUID::from_u128(0x25297d5c_3ad4_4c9c_b5cf_e36a38512330));
    assert_eq!(ICompositionDrawingSurfaceInterop::IID,
        GUID::from_u128(0xfd04e6e3_fe0c_4c3c_ab19_a07601a576ee));
    assert_eq!(IDesktopWindowTarget::IID,
        GUID::from_u128(0x6329d6ca_3366_490e_9db3_25312929ac51));
    let surface: ICompositionSurface = Surface.into();
    let compositor: ICompositorInterop = Compositor { surface: surface.clone() }.into();
    let unknown = surface.cast::<windows_core::IUnknown>().unwrap();
    assert!(surface.cast::<windows_core::IInspectable>().is_ok());
    unsafe {
        assert_eq!(compositor.CreateCompositionSurfaceForHandle(0x1234_usize as HANDLE).unwrap(), surface);
        assert_eq!(compositor.CreateCompositionSurfaceForSwapChain(&unknown).unwrap(), surface);
        assert_eq!(compositor.CreateGraphicsDevice(&unknown).unwrap_err().code(),
            windows_core::HRESULT(0x80004001_u32 as i32));
        let desktop = compositor.cast::<ICompositorDesktopInterop>().unwrap();
        assert_eq!(desktop.CreateDesktopWindowTarget(0x5678_usize as HWND, true).unwrap_err().code(),
            windows_core::HRESULT(0x80004001_u32 as i32));
        desktop.EnsureOnThread(42).unwrap();

        let options = DispatcherQueueOptions {
            dwSize: size_of::<DispatcherQueueOptions>() as u32,
            threadType: DQTYPE_THREAD_DEDICATED,
            apartmentType: DQTAT_COM_STA,
        };
        let mut controller = std::ptr::null_mut();
        CreateDispatcherQueueController(options, &mut controller).ok().unwrap();
        assert!(!controller.is_null());
        let controller = PDISPATCHERQUEUECONTROLLER::from_raw(controller);
        controller.ShutdownQueueAsync().unwrap().join().unwrap();
    }
}
