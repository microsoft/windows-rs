use bindings::{
    Windows::Foundation::Numerics::*, Windows::Win32::Direct2D::*, Windows::Win32::Direct3D11::*,
    Windows::Win32::Dxgi::*, Windows::Win32::Gdi::*, Windows::Win32::SystemServices::*,
    Windows::Win32::UIAnimation::*, Windows::Win32::WindowsAndMessaging::*,
    Windows::Win32::WindowsProgramming::*,
};

use windows::*;

fn main() -> Result<()> {
    initialize_sta()?;
    let mut window = Window::new()?;
    window.run()
}

struct Window {
    handle: HWND,
    factory: ID2D1Factory1,
    dxfactory: IDXGIFactory2,
    style: ID2D1StrokeStyle,
    manager: IUIAnimationManager,
    variable: IUIAnimationVariable,

    target: Option<ID2D1DeviceContext>,
    swapchain: Option<IDXGISwapChain1>,
    brush: Option<ID2D1SolidColorBrush>,
    shadow: Option<ID2D1Effect>,
    clock: Option<ID2D1Bitmap1>,
    dpi: f32,
    visible: bool,
    occlusion: u32,
    frequency: i64,
    angles: Angles,
}

#[derive(Default)]
struct Angles {
    second: f32,
    minute: f32,
    hour: f32,
}

impl Angles {
    fn now() -> Self {
        let mut time = Default::default();
        unsafe { GetLocalTime(&mut time) };

        let second = (time.wSecond as f32 + time.wMilliseconds as f32 / 1000.0) * 6.0;
        let minute = time.wMinute as f32 * 6.0 + second / 60.0;
        let hour = (time.wHour % 12) as f32 * 30.0 + minute / 12.0;

        Self {
            second,
            minute,
            hour,
        }
    }
}

impl Window {
    fn new() -> Result<Self> {
        let factory = create_factory()?;
        let dxfactory: IDXGIFactory2 = unsafe { CreateDXGIFactory1()? };
        let style = create_style(&factory)?;
        let manager: IUIAnimationManager = create_instance(&UIAnimationManager)?;
        let transition = create_transition()?;

        let mut dpi = 0.0;
        let mut dpiy = 0.0;
        unsafe { factory.GetDesktopDpi(&mut dpi, &mut dpiy) };

        let mut frequency = 0;
        unsafe { QueryPerformanceFrequency(&mut frequency).ok()? };

        let variable = unsafe {
            let mut variable = None;
            let variable = manager
                .CreateAnimationVariable(0.0, &mut variable)
                .and_some(variable)?;

            manager
                .ScheduleTransition(&variable, transition, get_time(frequency)?)
                .ok()?;
            variable
        };

        Ok(Window {
            handle: HWND(0),
            factory,
            dxfactory,
            style,
            manager,
            variable,
            target: None,
            swapchain: None,
            brush: None,
            shadow: None,
            clock: None,
            dpi,
            visible: false,
            occlusion: 0,
            frequency,
            angles: Angles::now(),
        })
    }

    fn render(&mut self) -> Result<()> {
        if self.target.is_none() {
            let device = create_device()?;
            let target = create_render_target(&self.factory, &device)?;
            unsafe { target.SetDpi(self.dpi, self.dpi) };

            let swapchain = create_swapchain(&device, self.handle)?;
            create_swapchain_bitmap(&swapchain, &target)?;

            self.brush = create_brush(&target).ok();
            self.target = Some(target);
            self.swapchain = Some(swapchain);
            self.create_device_size_resources()?;
        }

        let target = self.target.as_ref().unwrap();
        unsafe { target.BeginDraw() };
        self.draw(target)?;

        unsafe { target.EndDraw(std::ptr::null_mut(), std::ptr::null_mut()) }.ok()?;

        let error = self.present(1, 0);

        if error.is_err() {
            if error == DXGI_STATUS_OCCLUDED {
                unsafe {
                    self.dxfactory
                        .RegisterOcclusionStatusWindow(self.handle, WM_USER, &mut self.occlusion)
                        .ok()?;
                }
                self.visible = false;
            } else {
                self.release_device();
            }
        }

        Ok(())
    }

    fn release_device(&mut self) {
        self.target = None;
        self.swapchain = None;
        self.release_device_resources();
    }

    fn release_device_resources(&mut self) {
        self.brush = None;
        self.clock = None;
        self.shadow = None;
    }

    fn present(&self, sync: u32, flags: u32) -> HRESULT {
        unsafe { self.swapchain.as_ref().unwrap().Present(sync, flags) }
    }

    fn draw(&self, target: &ID2D1DeviceContext) -> Result<()> {
        let clock = self.clock.as_ref().unwrap();
        let shadow = self.shadow.as_ref().unwrap();

        unsafe {
            self.manager
                .Update(get_time(self.frequency)?, std::ptr::null_mut())
                .ok()?;

            target.Clear(&D2D1_COLOR_F {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            });

            let mut previous = None;
            target.GetTarget(&mut previous);
            target.SetTarget(clock);
            target.Clear(std::ptr::null());
            self.draw_clock()?;
            target.SetTarget(previous);
            target.SetTransform(&Matrix3x2::translation(5.0, 5.0));

            let mut output = None;
            shadow.GetOutput(&mut output);

            target.DrawImage(
                output,
                std::ptr::null(),
                std::ptr::null(),
                D2D1_INTERPOLATION_MODE::D2D1_INTERPOLATION_MODE_LINEAR,
                D2D1_COMPOSITE_MODE::D2D1_COMPOSITE_MODE_SOURCE_OVER,
            );

            target.SetTransform(&Matrix3x2::identity());

            target.DrawImage(
                clock,
                std::ptr::null(),
                std::ptr::null(),
                D2D1_INTERPOLATION_MODE::D2D1_INTERPOLATION_MODE_LINEAR,
                D2D1_COMPOSITE_MODE::D2D1_COMPOSITE_MODE_SOURCE_OVER,
            );
        }

        Ok(())
    }

    fn draw_clock(&self) -> Result<()> {
        let target = self.target.as_ref().unwrap();
        let brush = self.brush.as_ref().unwrap();

        let size = unsafe { target.GetSize() };

        let radius = size.width.min(size.height).max(200.0) / 2.0 - 50.0;
        let translation = Matrix3x2::translation(size.width / 2.0, size.height / 2.0);
        unsafe { target.SetTransform(&translation) };

        let ellipse = D2D1_ELLIPSE {
            point: D2D_POINT_2F::default(),
            radiusX: radius,
            radiusY: radius,
        };

        let mut swing = 0.0;
        unsafe {
            target.DrawEllipse(&ellipse, brush, radius / 20.0, None);
            self.variable.GetValue(&mut swing).ok()?;
        }
        let mut angles = Angles::now();

        if swing < 1.0 {
            if self.angles.second > angles.second {
                angles.second += 360.0;
            }
            if self.angles.minute > angles.minute {
                angles.minute += 360.0;
            }
            if self.angles.hour > angles.hour {
                angles.hour += 360.0;
            }

            angles.second *= swing as f32;
            angles.minute *= swing as f32;
            angles.hour *= swing as f32;
        }

        unsafe {
            target.SetTransform(&(Matrix3x2::rotation(angles.second, 0.0, 0.0) * &translation));

            target.DrawLine(
                D2D_POINT_2F::default(),
                D2D_POINT_2F {
                    x: 0.0,
                    y: -(radius * 0.75),
                },
                brush,
                radius / 25.0,
                &self.style,
            );

            target.SetTransform(&(Matrix3x2::rotation(angles.minute, 0.0, 0.0) * &translation));

            target.DrawLine(
                D2D_POINT_2F::default(),
                D2D_POINT_2F {
                    x: 0.0,
                    y: -(radius * 0.75),
                },
                brush,
                radius / 15.0,
                &self.style,
            );

            target.SetTransform(&(Matrix3x2::rotation(angles.hour, 0.0, 0.0) * &translation));

            target.DrawLine(
                D2D_POINT_2F::default(),
                D2D_POINT_2F {
                    x: 0.0,
                    y: -(radius * 0.5),
                },
                brush,
                radius / 10.0,
                &self.style,
            );
        }

        Ok(())
    }

    fn create_device_size_resources(&mut self) -> Result<()> {
        let target = self.target.as_ref().unwrap();
        let clock = self.create_clock(target)?;
        self.shadow = create_shadow(target, &clock).ok();
        self.clock = Some(clock);

        Ok(())
    }

    fn create_clock(&self, target: &ID2D1DeviceContext) -> Result<ID2D1Bitmap1> {
        let size_f = unsafe { target.GetSize() };

        let size_u = D2D_SIZE_U {
            width: (size_f.width * self.dpi / 96.0) as u32,
            height: (size_f.height * self.dpi / 96.0) as u32,
        };

        let properties = D2D1_BITMAP_PROPERTIES1 {
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE::D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: self.dpi,
            dpiY: self.dpi,
            bitmapOptions: D2D1_BITMAP_OPTIONS::D2D1_BITMAP_OPTIONS_TARGET,
            colorContext: None,
        };

        let mut bitmap = None;

        unsafe {
            target
                .CreateBitmap2(size_u, std::ptr::null(), 0, &properties, &mut bitmap)
                .and_some(bitmap)
        }
    }

    fn resize_swapchain_bitmap(&mut self) -> Result<()> {
        if let Some(target) = &self.target {
            let swapchain = self.swapchain.as_ref().unwrap();
            unsafe { target.SetTarget(None) };

            if unsafe {
                swapchain
                    .ResizeBuffers(0, 0, 0, DXGI_FORMAT::DXGI_FORMAT_UNKNOWN, 0)
                    .is_ok()
            } {
                create_swapchain_bitmap(swapchain, &target)?;
                self.create_device_size_resources()?;
            } else {
                self.release_device();
            }

            self.render()?;
        }

        Ok(())
    }

    fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_PAINT => {
                    let mut ps = PAINTSTRUCT::default();
                    BeginPaint(self.handle, &mut ps);
                    self.render().unwrap();
                    EndPaint(self.handle, &ps);
                    LRESULT(0)
                }
                WM_SIZE => {
                    if wparam.0 != SIZE_MINIMIZED as usize {
                        self.resize_swapchain_bitmap().unwrap();
                    }
                    LRESULT(0)
                }
                WM_DISPLAYCHANGE => {
                    self.render().unwrap();
                    LRESULT(0)
                }
                WM_USER => {
                    if self.present(0, DXGI_PRESENT_TEST).is_ok() {
                        self.dxfactory.UnregisterOcclusionStatus(self.occlusion);
                        self.occlusion = 0;
                        self.visible = true;
                    }
                    LRESULT(0)
                }
                WM_ACTIVATE => {
                    self.visible = true; // TODO: unpack !HIWORD(wparam);
                    LRESULT(0)
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(self.handle, message, wparam, lparam),
            }
        }
    }

    fn run(&mut self) -> Result<()> {
        unsafe {
            let instance = HINSTANCE(GetModuleHandleA(None));
            debug_assert!(instance.0 != 0);

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_HAND),
                hInstance: instance,
                lpszClassName: PSTR(b"window\0".as_ptr() as _),

                style: WNDCLASS_STYLES::CS_HREDRAW | WNDCLASS_STYLES::CS_VREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExA(
                Default::default(),
                "window",
                "Sample Window",
                WINDOW_STYLE::WS_OVERLAPPEDWINDOW | WINDOW_STYLE::WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                self as *mut _ as _,
            );

            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);
            let mut message = MSG::default();

            loop {
                if self.visible {
                    self.render()?;

                    while PeekMessageA(
                        &mut message,
                        None,
                        0,
                        0,
                        PEEK_MESSAGE_REMOVE_TYPE::PM_REMOVE,
                    )
                    .into()
                    {
                        if message.message == WM_QUIT {
                            return Ok(());
                        }
                        DispatchMessageA(&message);
                    }
                } else {
                    GetMessageA(&mut message, None, 0, 0);

                    if message.message == WM_QUIT {
                        return Ok(());
                    }

                    DispatchMessageA(&message);
                }
            }
        }
    }

    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lpCreateParams as *mut Self;
                (*this).handle = window;

                SetWindowLong(window, WINDOW_LONG_PTR_INDEX::GWLP_USERDATA, this as _);
            } else {
                let this = GetWindowLong(window, WINDOW_LONG_PTR_INDEX::GWLP_USERDATA) as *mut Self;

                if !this.is_null() {
                    return (*this).message_handler(message, wparam, lparam);
                }
            }

            DefWindowProcA(window, message, wparam, lparam)
        }
    }
}

fn get_time(frequency: i64) -> Result<f64> {
    unsafe {
        let mut time = 0;
        QueryPerformanceCounter(&mut time).ok()?;
        Ok(time as f64 / frequency as f64)
    }
}

fn create_brush(target: &ID2D1DeviceContext) -> Result<ID2D1SolidColorBrush> {
    let color = D2D1_COLOR_F {
        r: 0.92,
        g: 0.38,
        b: 0.208,
        a: 1.0,
    };

    let properties = D2D1_BRUSH_PROPERTIES {
        opacity: 0.8,
        transform: Matrix3x2::identity(),
    };

    let mut brush = None;

    unsafe {
        target
            .CreateSolidColorBrush(&color, &properties, &mut brush)
            .and_some(brush)
    }
}

fn create_shadow(target: &ID2D1DeviceContext, clock: &ID2D1Bitmap1) -> Result<ID2D1Effect> {
    let mut shadow = None;
    unsafe {
        let shadow = target
            .CreateEffect(&CLSID_D2D1Shadow, &mut shadow)
            .and_some(shadow)?;

        shadow.SetInput(0, clock, true);
        Ok(shadow)
    }
}

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL::D2D1_DEBUG_LEVEL_INFORMATION;
    }

    let mut result = None;

    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE::D2D1_FACTORY_TYPE_SINGLE_THREADED,
            &ID2D1Factory1::IID,
            &options,
            result.set_abi(),
        )
        .and_some(result)
    }
}

fn create_style(factory: &ID2D1Factory1) -> Result<ID2D1StrokeStyle> {
    let props = D2D1_STROKE_STYLE_PROPERTIES {
        startCap: D2D1_CAP_STYLE::D2D1_CAP_STYLE_ROUND,
        endCap: D2D1_CAP_STYLE::D2D1_CAP_STYLE_TRIANGLE,
        ..Default::default()
    };

    let mut style = None;

    unsafe {
        factory
            .CreateStrokeStyle(&props, std::ptr::null(), 0, &mut style)
            .and_some(style)
    }
}

fn create_transition() -> Result<IUIAnimationTransition> {
    let library: IUIAnimationTransitionLibrary = create_instance(&UIAnimationTransitionLibrary)?;

    let mut transition = None;

    unsafe {
        library
            .CreateAccelerateDecelerateTransition(5.0, 1.0, 0.2, 0.8, &mut transition)
            .and_some(transition)
    }
}

fn create_device_with_type(drive_type: D3D_DRIVER_TYPE) -> Result<ID3D11Device> {
    let mut flags = D3D11_CREATE_DEVICE_FLAG::D3D11_CREATE_DEVICE_BGRA_SUPPORT;

    if cfg!(debug_assertions) {
        flags |= D3D11_CREATE_DEVICE_FLAG::D3D11_CREATE_DEVICE_DEBUG;
    }

    let mut device = None;

    unsafe {
        D3D11CreateDevice(
            None,
            drive_type,
            0,
            flags,
            std::ptr::null(),
            0,
            D3D11_SDK_VERSION,
            &mut device,
            std::ptr::null_mut(),
            &mut None,
        )
        .and_some(device)
    }
}

fn create_device() -> Result<ID3D11Device> {
    let mut result = create_device_with_type(D3D_DRIVER_TYPE::D3D_DRIVER_TYPE_HARDWARE);

    if let Err(err) = &result {
        if err.code() == DXGI_ERROR_UNSUPPORTED {
            result = create_device_with_type(D3D_DRIVER_TYPE::D3D_DRIVER_TYPE_WARP);
        }
    }

    result
}

fn create_render_target(
    factory: &ID2D1Factory1,
    device: &ID3D11Device,
) -> Result<ID2D1DeviceContext> {
    let mut d2device = None;
    let mut target = None;
    unsafe {
        let d2device = factory
            .CreateDevice(device.cast::<IDXGIDevice>()?, &mut d2device)
            .and_some(d2device)?;

        let target = d2device
            .CreateDeviceContext(
                D2D1_DEVICE_CONTEXT_OPTIONS::D2D1_DEVICE_CONTEXT_OPTIONS_NONE,
                &mut target,
            )
            .and_some(target)?;
        target.SetUnitMode(D2D1_UNIT_MODE::D2D1_UNIT_MODE_DIPS);

        Ok(target)
    }
}

fn get_dxgi_factory(device: &ID3D11Device) -> Result<IDXGIFactory2> {
    let dxdevice = device.cast::<IDXGIDevice>()?;
    let mut adapter = None;
    unsafe {
        dxdevice
            .GetAdapter(&mut adapter)
            .and_some(adapter)?
            .GetParent()
    }
}

fn create_swapchain_bitmap(swapchain: &IDXGISwapChain1, target: &ID2D1DeviceContext) -> Result<()> {
    let surface: IDXGISurface = unsafe { swapchain.GetBuffer(0)? };

    let props = D2D1_BITMAP_PROPERTIES1 {
        pixelFormat: D2D1_PIXEL_FORMAT {
            format: DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_UNORM,
            alphaMode: D2D1_ALPHA_MODE::D2D1_ALPHA_MODE_IGNORE,
        },
        dpiX: 96.0,
        dpiY: 96.0,
        bitmapOptions: D2D1_BITMAP_OPTIONS::D2D1_BITMAP_OPTIONS_TARGET
            | D2D1_BITMAP_OPTIONS::D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
        colorContext: None,
    };

    let mut bitmap = None;

    unsafe {
        let bitmap = target
            .CreateBitmapFromDxgiSurface(&surface, &props, &mut bitmap)
            .and_some(bitmap)?;
        target.SetTarget(bitmap);
    };

    Ok(())
}

fn create_swapchain(device: &ID3D11Device, window: HWND) -> Result<IDXGISwapChain1> {
    let factory = get_dxgi_factory(device)?;

    let props = DXGI_SWAP_CHAIN_DESC1 {
        Format: DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 2,
        SwapEffect: DXGI_SWAP_EFFECT::DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        ..Default::default()
    };

    let mut swapchain = None;

    unsafe {
        factory.CreateSwapChainForHwnd(
            device,
            window,
            &props,
            std::ptr::null(),
            None,
            &mut swapchain,
        )
    }
    .and_some(swapchain)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongA(window, index, value as _) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongPtrA(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongA(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongPtrA(window, index)
}
