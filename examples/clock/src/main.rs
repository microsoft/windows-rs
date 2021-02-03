use bindings::{
    windows::foundation::numerics::*, windows::win32::direct2d::*, windows::win32::direct3d11::*,
    windows::win32::dxgi::*, windows::win32::gdi::*, windows::win32::menus_and_resources::*,
    windows::win32::system_services::*, windows::win32::ui_animation::*, windows::win32::upnp::*,
    windows::win32::windows_and_messaging::*, windows::win32::windows_programming::*, windows::*,
};

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

        let second = (time.w_second as f32 + time.w_milliseconds as f32 / 1000.0) * 6.0;
        let minute = time.w_minute as f32 * 6.0 + second / 60.0;
        let hour = (time.w_hour % 12) as f32 * 30.0 + minute / 12.0;

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
        let dxfactory = create_dxfactory()?;
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
            if error.0 == DXGI_STATUS_OCCLUDED as u32 {
                unsafe {
                    self.dxfactory
                        .RegisterOcclusionStatusWindow(
                            self.handle,
                            WM_USER as u32,
                            &mut self.occlusion,
                        )
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

    fn present(&self, sync: u32, flags: u32) -> ErrorCode {
        unsafe { self.swapchain.as_ref().unwrap().Present(sync, flags) }
    }

    fn draw(&self, target: &ID2D1DeviceContext) -> Result<()> {
        let clock = self.clock.as_ref().unwrap();
        let shadow = self.shadow.as_ref().unwrap();

        unsafe {
            self.manager
                .Update(get_time(self.frequency)?, std::ptr::null_mut())
                .ok()?;

            target.Clear(&DXGI_RGBA {
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

        let mut size = D2D_SIZE_F::default();
        unsafe { target.GetSize(&mut size) };

        let radius = size.width.min(size.height).max(200.0) / 2.0 - 50.0;
        let translation = Matrix3x2::translation(size.width / 2.0, size.height / 2.0);
        unsafe { target.SetTransform(&translation) };

        let ellipse = D2D1_ELLIPSE {
            point: D2D_POINT_2F::default(),
            radiusx: radius,
            radiusy: radius,
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
            target.SetTransform(
                &(Matrix3x2::rotation(angles.second, Vector2::default()) * &translation),
            );

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

            target.SetTransform(
                &(Matrix3x2::rotation(angles.minute, Vector2::default()) * &translation),
            );

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

            target.SetTransform(
                &(Matrix3x2::rotation(angles.hour, Vector2::default()) * &translation),
            );

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
        let mut size_f = D2D_SIZE_F::default();
        unsafe { target.GetSize(&mut size_f) };

        let size_u = D2D_SIZE_U {
            width: (size_f.width * self.dpi / 96.0) as u32,
            height: (size_f.height * self.dpi / 96.0) as u32,
        };

        let properties = D2D1_BITMAP_PROPERTIES1 {
            pixel_format: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_UNORM,
                alpha_mode: D2D1_ALPHA_MODE::D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpix: self.dpi,
            dpiy: self.dpi,
            bitmap_options: D2D1_BITMAP_OPTIONS::D2D1_BITMAP_OPTIONS_TARGET,
            color_context: None,
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
            match message as i32 {
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
            let instance = HINSTANCE(GetModuleHandleA(std::ptr::null()));
            debug_assert!(instance.0 != 0);
            let class_name = b"Sample\0";
            let window_title = b"Clock\0";

            let wc = WNDCLASSA {
                h_cursor: LoadCursorA(HINSTANCE(0), IDC_ARROW as *const i8),
                h_instance: instance,
                lpsz_class_name: class_name.as_ptr() as *mut u8 as *mut i8,
                style: (CS_HREDRAW | CS_VREDRAW) as u32,
                lpfn_wnd_proc: Some(Self::wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExA(
                0,
                class_name.as_ptr() as *const i8,
                window_title.as_ptr() as *const i8,
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                HWND(0),
                HMENU(0),
                instance,
                self as *mut _ as _,
            );

            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);
            let mut message = MSG::default();

            loop {
                if self.visible {
                    self.render()?;

                    while PeekMessageA(&mut message, HWND(0), 0, 0, PM_REMOVE as u32).into() {
                        if message.message == WM_QUIT as u32 {
                            return Ok(());
                        }
                        DispatchMessageA(&message);
                    }
                } else {
                    GetMessageA(&mut message, HWND(0), 0, 0);
                    if message.message == WM_QUIT as u32 {
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
            if message == WM_NCCREATE as u32 {
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lp_create_params as *mut Self;
                (*this).handle = window;
                SetWindowLongPtrA(window, GWLP_USERDATA, this as _);
            } else {
                let this = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut Self;

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
    let color = DXGI_RGBA {
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
            .CreateEffect(&"C67EA361-1863-4e69-89DB-695D3E9A5B6B".into(), &mut shadow)
            .and_some(shadow)?;

        shadow.SetInput(0, clock, TRUE);
        Ok(shadow)
    }
}

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debug_level = D2D1_DEBUG_LEVEL::D2D1_DEBUG_LEVEL_INFORMATION;
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

fn create_dxfactory() -> Result<IDXGIFactory2> {
    unsafe {
        let mut dxfactory: Option<IDXGIFactory2> = None;
        CreateDXGIFactory1(&IDXGIFactory2::IID, dxfactory.set_abi()).and_some(dxfactory)
    }
}

fn create_style(factory: &ID2D1Factory1) -> Result<ID2D1StrokeStyle> {
    let props = D2D1_STROKE_STYLE_PROPERTIES {
        start_cap: D2D1_CAP_STYLE::D2D1_CAP_STYLE_ROUND,
        end_cap: D2D1_CAP_STYLE::D2D1_CAP_STYLE_TRIANGLE,
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
    // TODO: need fix for https://github.com/microsoft/win32metadata/issues/165
    let mut flags = D3D11_CREATE_DEVICE_FLAG::D3D11_CREATE_DEVICE_BGRA_SUPPORT.0 as u32;

    if cfg!(debug_assertions) {
        flags |= D3D11_CREATE_DEVICE_FLAG::D3D11_CREATE_DEVICE_DEBUG.0 as u32;
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
            D3D11_SDK_VERSION as u32,
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
        if err.code().0 == DXGI_ERROR_UNSUPPORTED as u32 {
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
        let adapter = dxdevice.GetAdapter(&mut adapter).and_some(adapter)?;
        let mut parent = None;

        adapter
            .GetParent(&IDXGIFactory2::IID, parent.set_abi())
            .and_some(parent)
    }
}

fn create_swapchain_bitmap(swapchain: &IDXGISwapChain1, target: &ID2D1DeviceContext) -> Result<()> {
    let mut surface = None;

    let surface: IDXGISurface = unsafe {
        swapchain
            .GetBuffer(0, &IDXGISurface::IID, surface.set_abi())
            .and_some(surface)?
    };

    let props = D2D1_BITMAP_PROPERTIES1 {
        pixel_format: D2D1_PIXEL_FORMAT {
            format: DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_UNORM,
            alpha_mode: D2D1_ALPHA_MODE::D2D1_ALPHA_MODE_IGNORE,
        },
        dpix: 96.0,
        dpiy: 96.0,
        bitmap_options: D2D1_BITMAP_OPTIONS::D2D1_BITMAP_OPTIONS_TARGET
            | D2D1_BITMAP_OPTIONS::D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
        color_context: None,
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
        format: DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_UNORM,
        sample_desc: DXGI_SAMPLE_DESC {
            count: 1,
            quality: 0,
        },
        buffer_usage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        buffer_count: 2,
        swap_effect: DXGI_SWAP_EFFECT::DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
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

// TODO: workaround for https://github.com/microsoft/win32metadata/issues/142
#[link(name = "user32")]
extern "system" {
    fn SetWindowLongPtrA(window: HWND, index: i32, value: isize) -> isize;

    fn GetWindowLongPtrA(window: HWND, index: i32) -> isize;
}
