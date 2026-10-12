use std::time::Duration;
use windows_reactor::*;

struct Fixture {
    phase: u8,
    subscribed: bool,
    counts: [u32; 3],
    previous: [u32; 3],
    scheme: Option<(u8, ColorScheme)>,
    size: Option<(u8, WindowSize)>,
    placement: Option<(u8, WindowPlacement)>,
    _timeout: ComponentTimer,
    delay: ComponentTimer,
}

enum Message {
    Scheme(u8, ColorScheme),
    Size(u8, WindowSize),
    Placement(u8, WindowPlacement),
    Step,
    Subscribe(bool),
    Timeout,
}

#[derive(Clone, PartialEq)]
struct ObserverInput {
    scheme: Callback<ColorScheme>,
    size: Callback<WindowSize>,
    placement: Callback<WindowPlacement>,
}

struct ObserverChild;

impl Component for ObserverChild {
    type Input = ObserverInput;
    type Message = ();

    fn create(_: &ObserverInput, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &ObserverInput, context: &mut ViewContext<Self>) -> View {
        context.on_color_scheme(input.scheme.clone());
        context.on_window_size(input.size.clone());
        context.on_window_placement(input.placement.clone());
        TextBlock::new().text("Child window observer").into()
    }
}

impl Component for Fixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        Self {
            phase: 0,
            subscribed: true,
            counts: [0; 3],
            previous: [0; 3],
            scheme: None,
            size: None,
            placement: None,
            _timeout: context.set_timeout(Duration::from_secs(10), Message::Timeout),
            delay: context.set_timeout(Duration::from_millis(200), Message::Step),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Scheme(phase, scheme) => {
                self.counts[0] += 1;
                self.scheme = Some((phase, scheme));
                if self.phase == 0 {
                    assert_eq!(self.counts[0], 1, "render replayed the initial scheme");
                }
            }
            Message::Size(phase, size) => {
                self.counts[1] += 1;
                self.size = Some((phase, size));
            }
            Message::Placement(phase, placement) => {
                self.counts[2] += 1;
                self.placement = Some((phase, placement));
            }
            Message::Timeout => panic!("window observer fixture stalled in phase {}", self.phase),
            Message::Subscribe(subscribed) => self.subscribed = subscribed,
            Message::Step => {
                match self.phase {
                    0 => {
                        assert_eq!(self.counts[0], 1);
                        assert_eq!(self.scheme, Some((0, ColorScheme::Dark)));
                        assert!(self.counts[1] > 0);
                        assert_eq!(self.counts[2], 1);
                    }
                    1 | 3 | 4 | 6 | 8 | 11 | 14 | 16 => {
                        assert_eq!(
                            self.counts, self.previous,
                            "unchanged or removed observers fired in phase {}",
                            self.phase,
                        );
                    }
                    2 | 7 => {
                        assert_eq!(self.counts[0], self.previous[0] + 1);
                        assert_eq!(self.scheme, Some((self.phase, ColorScheme::Light)));
                        assert!(self.counts[1] > self.previous[1]);
                        assert!(self.counts[2] > self.previous[2]);
                        assert_eq!(self.size.unwrap().0, self.phase);
                        assert_eq!(self.placement.unwrap().0, self.phase);
                    }
                    5 => {
                        assert_eq!(self.counts[0], self.previous[0] + 1);
                        assert_eq!(self.scheme, Some((5, ColorScheme::Dark)));
                        assert_eq!(self.counts[1], self.previous[1]);
                        assert_eq!(self.counts[2], self.previous[2] + 1);
                        assert_eq!(self.placement.unwrap().0, 5);
                    }
                    9 => {
                        assert_eq!(self.counts[0], self.previous[0]);
                        assert!(self.counts[1] > self.previous[1]);
                        assert!(self.counts[2] > self.previous[2]);
                        assert_eq!(self.size.unwrap().0, 9);
                        assert_eq!(self.placement.unwrap().0, 9);
                    }
                    10 => {
                        assert_eq!(self.counts[0], self.previous[0] + 1);
                        assert_eq!(self.scheme, Some((10, ColorScheme::Light)));
                        assert_eq!(self.counts[1..], self.previous[1..]);
                    }
                    12 | 13 | 15 => {
                        assert_eq!(
                            self.counts[0],
                            self.previous[0] + 1,
                            "new observer lifetime did not report the current scheme",
                        );
                        assert_eq!(self.scheme, Some((self.phase, ColorScheme::Light)));
                        assert_eq!(self.counts[1], self.previous[1]);
                        assert_eq!(self.counts[2], self.previous[2] + 1);
                        assert_eq!(self.placement.unwrap().0, self.phase);
                    }
                    _ => unreachable!(),
                }
                if self.phase == 16 {
                    assert!(context.close_window());
                    return;
                }
                self.previous = self.counts;
                self.phase += 1;
                if self.phase == 12 {
                    let sender = context.sender();
                    assert!(sender.send(Message::Subscribe(false)));
                    assert!(sender.send(Message::Subscribe(true)));
                }
                self.delay = context.set_timeout(Duration::from_millis(200), Message::Step);
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Window observer fixture");
        let phase = self.phase;
        if phase != 4 && phase < 13 && self.subscribed {
            // Each render captures a new mapping, including renders caused by the observers.
            if !matches!(phase, 8 | 9) {
                context.on_color_scheme(
                    context.callback(move |scheme| Message::Scheme(phase, scheme)),
                );
            }
            context.on_window_size(context.callback(move |size| Message::Size(phase, size)));
            context.on_window_placement(
                context.callback(move |placement| Message::Placement(phase, placement)),
            );
        }
        let (theme, width) = match phase {
            0 | 1 => (WindowTheme::Dark, 480.0),
            2 | 3 => (WindowTheme::Light, 520.0),
            4..=6 => (WindowTheme::Dark, 560.0),
            7 | 8 => (WindowTheme::Light, 600.0),
            _ => (WindowTheme::Light, 640.0),
        };
        context.window_visuals(WindowVisuals::new().theme(theme).client_size(width, 320.0));
        if phase >= 13 {
            return StackPanel::new()
                .keyed_children([keyed(
                    if phase < 15 { "first" } else { "second" },
                    View::component::<ObserverChild>(ObserverInput {
                        scheme: context.callback(move |scheme| Message::Scheme(phase, scheme)),
                        size: context.callback(move |size| Message::Size(phase, size)),
                        placement: context
                            .callback(move |placement| Message::Placement(phase, placement)),
                    }),
                )])
                .into();
        }
        TextBlock::new().text("Window observer lifecycle").into()
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(20));
        eprintln!("window observer fixture timed out");
        std::process::exit(1);
    });
    App::run_component::<Fixture>(())
}
