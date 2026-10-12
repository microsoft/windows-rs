use std::time::Duration;
use windows::Win32::dwmapi::DwmFlush;
use windows::Win32::wingdi::{CLR_INVALID, GetPixel};
use windows::Win32::winuser::*;
use windows::Win32::{POINT, RECT};
use windows_reactor::*;

fn sample(window: &WindowHandle<'_>) -> (u32, u32, WindowSize) {
    let hwnd = window.as_raw().cast();
    unsafe {
        assert!(
            SetWindowPos(
                hwnd,
                Some(HWND_TOPMOST),
                100,
                100,
                0,
                0,
                (SWP_NOSIZE | SWP_NOACTIVATE) as u32
            )
            .as_bool()
        );
        DwmFlush().ok().unwrap();
        let mut rect = RECT::default();
        assert!(GetClientRect(hwnd, &mut rect).as_bool());
        let dpi = f64::from(GetDpiForWindow(hwnd));
        let mut inside = POINT {
            x: (60.0 * dpi / 96.0) as i32,
            y: (70.0 * dpi / 96.0) as i32,
        };
        let mut outside = POINT {
            x: rect.right - 30,
            y: rect.bottom - 30,
        };
        assert!(ClientToScreen(hwnd, &mut inside).as_bool());
        assert!(ClientToScreen(hwnd, &mut outside).as_bool());
        let dc = GetDC(None);
        assert!(!dc.is_null());
        let colors = (
            GetPixel(dc, inside.x, inside.y),
            GetPixel(dc, outside.x, outside.y),
        );
        assert_ne!(ReleaseDC(None, dc), 0);
        assert_ne!(colors.0, CLR_INVALID);
        assert_ne!(colors.1, CLR_INVALID);
        (
            colors.0,
            colors.1,
            WindowSize {
                width: f64::from(rect.right) * 96.0 / dpi,
                height: f64::from(rect.bottom) * 96.0 / dpi,
            },
        )
    }
}

fn background(theme: WindowTheme) -> u32 {
    unsafe {
        let mut contrast = HIGHCONTRASTW {
            cbSize: size_of::<HIGHCONTRASTW>() as u32,
            ..Default::default()
        };
        assert!(
            SystemParametersInfoW(
                SPI_GETHIGHCONTRAST as u32,
                contrast.cbSize,
                (&mut contrast as *mut HIGHCONTRASTW).cast(),
                0,
            )
            .as_bool()
        );
        if contrast.dwFlags & HCF_HIGHCONTRASTON as u32 != 0 {
            return GetSysColor(COLOR_BTNFACE);
        }
    }
    match theme {
        WindowTheme::Light => 0x00ff_ffff,
        WindowTheme::Dark => 0,
        WindowTheme::System => unreachable!(),
    }
}

struct Fixture {
    phase: u8,
    scheme: Option<ColorScheme>,
    size: Option<WindowSize>,
    delay: ComponentTimer,
}

enum Message {
    Step,
    Pixels(u32, u32, WindowSize),
    Scheme(ColorScheme),
    Size(WindowSize),
    PolicyDone,
}

impl Fixture {
    fn case(&self) -> (WindowTheme, WindowBackdrop, bool, bool) {
        use WindowBackdrop::*;
        use WindowTheme::*;
        match self.phase {
            0 | 2 | 4 | 6 => (Light, None, false, false),
            1 => (Dark, None, false, false),
            3 => (Light, Mica, false, false),
            5 => (Light, MicaAlt, false, false),
            7 => (Light, Acrylic, false, false),
            8 => (Dark, None, false, false),
            9 => (Light, None, true, false),
            10 => (Dark, Mica, true, false),
            11 => (Light, None, true, true),
            12 => (Dark, None, true, true),
            _ => (Dark, None, false, false),
        }
    }
}

impl Component for Fixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        Self {
            phase: 0,
            scheme: None,
            size: None,
            delay: context.set_timeout(Duration::from_millis(300), Message::Step),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Scheme(scheme) => self.scheme = Some(scheme),
            Message::Size(size) => self.size = Some(size),
            Message::Step => assert!(context.run_window(|window| {
                let (inside, outside, size) = sample(&window);
                Message::Pixels(inside, outside, size)
            })),
            Message::Pixels(inside, outside, client) => {
                let (theme, backdrop, green, small) = self.case();
                assert_eq!(
                    self.scheme,
                    Some(if theme == WindowTheme::Light {
                        ColorScheme::Light
                    } else {
                        ColorScheme::Dark
                    }),
                    "wrong scheme in phase {}",
                    self.phase,
                );
                // Size subscriptions do not replay layout completed before registration.
                if self.phase > 0 {
                    let size = self.size.unwrap();
                    assert!(
                        (size.width - client.width).abs() < 0.01,
                        "root width changed in phase {}: {size:?} vs {client:?}",
                        self.phase
                    );
                    assert!(
                        // WinUI excludes the top non-client border from its root layout.
                        (size.height - client.height).abs() <= 1.0,
                        "root height changed in phase {}: {size:?} vs {client:?}",
                        self.phase
                    );
                }
                if green {
                    assert_eq!(
                        inside, 0x0000_8000,
                        "application background was overwritten"
                    );
                }
                if backdrop == WindowBackdrop::None {
                    let expected = if green && !small {
                        0x0000_8000
                    } else {
                        background(theme)
                    };
                    assert_eq!(outside, expected, "wrong fallback in phase {}", self.phase);
                } else if !green && background(WindowTheme::Light) != background(WindowTheme::Dark)
                {
                    assert_ne!(outside, background(theme), "fallback covered the backdrop");
                }
                if self.phase == 13 {
                    self.phase = 14;
                    let sender = context.sender();
                    assert!(
                        context.open_window_with_policy::<PolicyFixture>(
                            (
                                WindowTheme::Light,
                                Callback::new(move |()| assert!(sender.send(Message::PolicyDone))),
                            ),
                            WindowPolicy::new()
                                .theme(WindowTheme::Light)
                                .client_size(480.0, 320.0),
                        )
                    );
                } else {
                    self.phase += 1;
                    self.delay = context.set_timeout(Duration::from_millis(300), Message::Step);
                }
            }
            Message::PolicyDone if self.phase == 14 => {
                self.phase = 15;
                let sender = context.sender();
                assert!(
                    context.open_window_with_policy::<PolicyFixture>(
                        (
                            WindowTheme::Dark,
                            Callback::new(move |()| assert!(sender.send(Message::PolicyDone))),
                        ),
                        WindowPolicy::new()
                            .theme(WindowTheme::Dark)
                            .client_size(480.0, 320.0),
                    )
                );
            }
            Message::PolicyDone => {
                assert_eq!(self.phase, 15);
                assert!(context.close_window());
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let (theme, backdrop, green, small) = self.case();
        context.on_color_scheme(context.callback(Message::Scheme));
        context.on_window_size(context.callback(Message::Size));
        context.window_visuals(
            WindowVisuals::new()
                .theme(theme)
                .backdrop(backdrop)
                .client_size(if self.phase == 0 { 480.0 } else { 500.0 }, 320.0),
        );
        let mut root = Border::new().content(context.window_frame(
            "Window theme fixture",
            TextBlock::new().text("Window theme fallback").margin(16.0),
        ));
        if green {
            root = root.background(Color::rgb(0, 128, 0));
        }
        if small {
            root = root
                .width(120.0)
                .height(80.0)
                .horizontal_alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Top);
        }
        root.into()
    }
}

struct PolicyFixture {
    theme: WindowTheme,
    done: Callback<()>,
    _delay: ComponentTimer,
}

enum PolicyMessage {
    Check,
    Pixels(u32),
}

impl Component for PolicyFixture {
    type Input = (WindowTheme, Callback<()>);
    type Message = PolicyMessage;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            theme: input.0,
            done: input.1.clone(),
            _delay: context.set_timeout(Duration::from_millis(300), PolicyMessage::Check),
        }
    }

    fn update(&mut self, message: PolicyMessage, context: &ComponentContext<Self>) {
        match message {
            PolicyMessage::Check => {
                assert!(context.run_window(|window| { PolicyMessage::Pixels(sample(&window).1) }));
            }
            PolicyMessage::Pixels(pixel) => {
                assert_eq!(pixel, background(self.theme));
                assert!(context.close_window());
                self.done.call(());
            }
        }
    }

    fn view(&self, _: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Window policy theme fixture");
        TextBlock::new().text("Window policy theme").into()
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(20));
        eprintln!("Window theme fixture timed out");
        std::process::exit(1);
    });
    App::run_component::<Fixture>(())
}
