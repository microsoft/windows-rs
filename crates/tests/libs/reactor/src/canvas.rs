use super::*;
use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use windows_canvas::{Canvas, ColorF, GpuDevice, Invalidator};
use windows_core::{IUnknown, Interface, implement, interface};

#[expect(non_snake_case)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "\\canvas_bindings.rs"));
}

pub(super) struct CanvasFixture {
    surface: Option<View>,
    generation: u32,
    resized: bool,
    frame: String,
    dropped: Vec<u32>,
    devices: Arc<DeviceCounts>,
    device_status: String,
    invalidator: Invalidator,
    lose_device: Rc<Cell<bool>>,
    repaint: Rc<Repaint>,
    repaint_status: String,
    draw_errors: u32,
}

#[derive(Clone)]
pub(super) enum Message {
    Resize,
    Remove,
    Attach,
    Frame(String),
    Dropped(u32),
    LoseDevice,
    Probe,
    Invalidate(bool),
    Repainted(String),
    FailDraw,
    DrawError(IntegrationError),
}

#[derive(Default)]
struct Repaint {
    revision: Cell<u32>,
    again: Cell<bool>,
    fail: Cell<bool>,
}

#[derive(Clone, Copy, Default)]
pub(super) enum DrawFailure {
    #[default]
    None,
    Native,
    Callback,
    Device,
}

#[derive(Clone, Copy, Default)]
pub(super) enum AfterSubmission {
    #[default]
    None,
    Invalidate,
    Remove,
}

#[derive(Default)]
struct SubmittedFrames {
    requested: Cell<u32>,
    attempt: Cell<u32>,
    pending: Cell<Option<(u32, u32, f32)>>,
    current: Cell<Option<(u32, u32, f32)>>,
    failed_attempt: Cell<Option<u32>>,
    failure: Cell<DrawFailure>,
    after: Cell<AfterSubmission>,
}

pub(super) struct SubmittedInputFixture {
    surface: Option<View>,
    frames: Rc<SubmittedFrames>,
    invalidator: Invalidator,
    submitted: Option<u32>,
    pressed: Option<(u32, &'static str)>,
    presses: u32,
    errors: u32,
    expected_error: Option<IntegrationError>,
    released: bool,
}

#[derive(Clone)]
pub(super) enum SubmittedMessage {
    Stage,
    Draw(DrawFailure, AfterSubmission),
    Submitted(u32),
    Pressed(u32, &'static str),
    Error(IntegrationError),
    Remove,
    Released,
}

impl Component for SubmittedInputFixture {
    type Input = ();
    type Message = SubmittedMessage;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        let frames = Rc::new(SubmittedFrames::default());
        let invalidator = Invalidator::new();
        let lifetime = Rc::new(DrawLifetime {
            generation: 0,
            dropped: context.callback(|_| SubmittedMessage::Released),
        });
        let draw_lifetime = Rc::clone(&lifetime);
        let drawing = Rc::clone(&frames);
        let completed = Rc::clone(&frames);
        let redraw = invalidator.clone();
        let submitted = context.callback(SubmittedMessage::Submitted);
        let remove = context.message(SubmittedMessage::Remove);
        let error = context.callback(SubmittedMessage::Error);
        let canvas: View = Canvas::invalidated(&invalidator, move |frame| {
            let _keep_alive = &draw_lifetime;
            let revision = drawing.requested.get();
            let attempt = drawing.attempt.get() + 1;
            drawing.attempt.set(attempt);
            drawing.pending.set(Some((attempt, revision, frame.height)));
            frame.clear(ColorF::BLUE);
            let brush = frame.create_solid_brush(ColorF::RED)?;
            let y = if revision.is_multiple_of(2) {
                0.0
            } else {
                frame.height / 2.0
            };
            frame.fill_rect(
                &windows_canvas::Rect::from_xywh(0.0, y, frame.width, frame.height / 2.0),
                &brush,
            );
            let failure = drawing.failure.take();
            if !matches!(failure, DrawFailure::None) {
                drawing.failed_attempt.set(Some(attempt));
            }
            match failure {
                DrawFailure::None => Ok(()),
                DrawFailure::Native => {
                    unsafe {
                        frame
                            .as_interface::<bindings::ID2D1DeviceContext>()
                            .PopAxisAlignedClip();
                    }
                    Ok(())
                }
                DrawFailure::Callback => Err(windows_core::Error::from_hresult(
                    windows_core::HRESULT(0x8000_4004_u32 as i32),
                )),
                DrawFailure::Device => Err(windows_canvas::device_lost_error()),
            }
        })
        .device_factory(GpuDevice::new_warp)
        .on_error(move |value| error.call(value))
        .on_submitted(move || {
            let _keep_alive = &lifetime;
            let frame = completed.pending.take().unwrap();
            assert_ne!(Some(frame.0), completed.failed_attempt.get());
            completed.current.set(Some(frame));
            submitted.call(frame.1);
            match completed.after.take() {
                AfterSubmission::None => {}
                AfterSubmission::Invalidate => {
                    completed.requested.set(frame.1 + 1);
                    redraw.invalidate();
                }
                AfterSubmission::Remove => {
                    redraw.invalidate();
                    remove.call(());
                }
            }
        })
        .into();
        let current = Rc::clone(&frames);
        let pressed = context.callback(|(revision, row)| SubmittedMessage::Pressed(revision, row));
        let surface = Border::new()
            .width(240.0)
            .height(120.0)
            .on_pointer_pressed(RoutedCallback::new(move |pointer: PointerEventInfo| {
                let (_, revision, height) = current.current.get().unwrap();
                let top = pointer.y < f64::from(height / 2.0);
                let row = if top == revision.is_multiple_of(2) {
                    "red"
                } else {
                    "blue"
                };
                pressed.call((revision, row));
                true
            }))
            .content(
                Grid::new().children((
                    canvas,
                    TextBlock::new()
                        .automation_id("canvas-input")
                        .text("Two canvas rows"),
                )),
            )
            .into();
        Self {
            surface: Some(surface),
            frames,
            invalidator,
            submitted: None,
            pressed: None,
            presses: 0,
            errors: 0,
            expected_error: None,
            released: false,
        }
    }

    fn update(&mut self, message: Self::Message, _: &ComponentContext<Self>) {
        match message {
            SubmittedMessage::Stage => self.frames.requested.set(self.frames.requested.get() + 1),
            SubmittedMessage::Draw(failure, after) => {
                self.expected_error = match failure {
                    DrawFailure::Native => Some(IntegrationError::Native(
                        bindings::D2DERR_PUSH_POP_UNBALANCED.0,
                    )),
                    DrawFailure::Callback => Some(IntegrationError::Native(0x8000_4004_u32 as i32)),
                    DrawFailure::None | DrawFailure::Device => None,
                };
                self.frames.failure.set(failure);
                self.frames.after.set(after);
                self.invalidator.invalidate();
            }
            SubmittedMessage::Submitted(revision) => self.submitted = Some(revision),
            SubmittedMessage::Pressed(revision, row) => {
                self.presses += 1;
                self.pressed = Some((revision, row));
            }
            SubmittedMessage::Error(error) => {
                assert_eq!(Some(error), self.expected_error.take());
                self.errors += 1;
            }
            SubmittedMessage::Remove => self.surface = None,
            SubmittedMessage::Released => self.released = true,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: canvas-submitted-input");
        context.window_visuals(WindowVisuals::new().client_size(600.0, 700.0));
        let draw = |id, label, failure, after| {
            Button::new()
                .automation_id(id)
                .content(label)
                .on_click(context.message(SubmittedMessage::Draw(failure, after)))
        };
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                Button::new()
                    .automation_id("stage")
                    .content("Stage next layout")
                    .on_click(context.message(SubmittedMessage::Stage)),
                draw(
                    "submit",
                    "Submit layout",
                    DrawFailure::None,
                    AfterSubmission::None,
                ),
                draw(
                    "fail-native",
                    "Fail EndDraw",
                    DrawFailure::Native,
                    AfterSubmission::None,
                ),
                draw(
                    "fail-callback",
                    "Fail callback",
                    DrawFailure::Callback,
                    AfterSubmission::None,
                ),
                draw(
                    "lose-device",
                    "Return device loss",
                    DrawFailure::Device,
                    AfterSubmission::None,
                ),
                draw(
                    "again",
                    "Invalidate from submission",
                    DrawFailure::None,
                    AfterSubmission::Invalidate,
                ),
                draw(
                    "remove",
                    "Remove from submission",
                    DrawFailure::None,
                    AfterSubmission::Remove,
                ),
                StackPanel::new()
                    .keyed_children(self.surface.clone().map(|surface| keyed("canvas", surface))),
                TextBlock::new()
                    .automation_id("requested")
                    .text(format!("Requested {}", self.frames.requested.get())),
                TextBlock::new()
                    .automation_id("submitted")
                    .text(format!("Submitted {:?}", self.submitted)),
                TextBlock::new()
                    .automation_id("pressed")
                    .text(format!("Pressed {} {:?}", self.presses, self.pressed)),
                TextBlock::new()
                    .automation_id("errors")
                    .text(format!("Errors {}", self.errors)),
                TextBlock::new()
                    .automation_id("released")
                    .text(format!("Released {}", self.released)),
            ))
            .into()
    }
}

#[derive(Default)]
struct DeviceCounts {
    created: AtomicU32,
    released: AtomicU32,
}

#[interface("8c0f69d8-f481-4509-9b1e-ec0b434b4088")]
unsafe trait IDeviceLifetime: IUnknown {}

#[implement(IDeviceLifetime)]
struct DeviceLifetime {
    counts: Arc<DeviceCounts>,
    completion: ComponentCompletion<Message>,
}

impl IDeviceLifetime_Impl for DeviceLifetime_Impl {}

impl Drop for DeviceLifetime {
    fn drop(&mut self) {
        self.counts.released.fetch_add(1, Ordering::SeqCst);
        _ = self.completion.clone().complete(Message::Probe);
    }
}

struct DrawLifetime {
    generation: u32,
    dropped: Callback<u32>,
}

impl Drop for DrawLifetime {
    fn drop(&mut self) {
        self.dropped.call(self.generation);
    }
}

fn surface(
    generation: u32,
    devices: &Arc<DeviceCounts>,
    invalidator: &Invalidator,
    lose_device: &Rc<Cell<bool>>,
    repaint: &Rc<Repaint>,
    context: &ComponentContext<CanvasFixture>,
) -> View {
    let lifetime = DrawLifetime {
        generation,
        dropped: context.callback(Message::Dropped),
    };
    let observed = Cell::new(None);
    let drawn = context.callback(Message::Frame);
    let draw_devices = Arc::clone(devices);
    let lose_device = Rc::clone(lose_device);
    let repaint = Rc::clone(repaint);
    let redraw = invalidator.clone();
    let repainted = context.callback(Message::Repainted);
    let failed = context.callback(Message::DrawError);
    let counts = Cell::new((0, 0));
    let demand = *context.input();
    let draw = move |frame: &windows_canvas::DrawContext<'_>| {
        let _keep_alive = &lifetime;
        frame.clear(ColorF::CORNFLOWER_BLUE);
        if lose_device.replace(false) {
            return Err(windows_core::Error::from_hresult(windows_core::HRESULT(
                0x887A_0005_u32 as i32,
            )));
        }
        if repaint.fail.replace(false) {
            unsafe {
                frame
                    .as_interface::<bindings::ID2D1DeviceContext>()
                    .PopAxisAlignedClip();
            }
            return Err(windows_core::Error::from_hresult(windows_core::HRESULT(
                0x8000_4004_u32 as i32,
            )));
        }
        let size = (frame.width, frame.height);
        if observed.replace(Some(size)) != Some(size) || frame.device_changed() {
            drawn.call(format!(
                "Canvas {generation}: {:.0} x {:.0}; changed {}; device {}",
                frame.width,
                frame.height,
                frame.device_changed(),
                draw_devices.created.load(Ordering::SeqCst),
            ));
        }
        if demand {
            let revision = repaint.revision.get();
            let (previous, count) = counts.get();
            let count = if revision == previous { count + 1 } else { 1 };
            counts.set((revision, count));
            repainted.call(format!("Revision {revision}; draws {count}"));
            if repaint.again.replace(false) {
                redraw.invalidate();
            }
        }
        Ok(())
    };
    let canvas = if *context.input() {
        Canvas::invalidated(invalidator, draw)
    } else {
        Canvas::animated(draw)
    };
    let devices = Arc::clone(devices);
    let completion = context.completion();
    canvas
        .on_error(move |error| failed.call(error))
        .device_factory(move || {
            let device = GpuDevice::new_warp()?;
            let native: bindings::ID3D11Device = device
                .d3d_device()
                .as_interface::<bindings::ID3D11Device>()
                .clone();
            let lifetime: IDeviceLifetime = DeviceLifetime {
                counts: Arc::clone(&devices),
                completion: completion.clone(),
            }
            .into();
            // Native ownership of this marker ends when the D3D device is destroyed.
            unsafe {
                native
                    .SetPrivateDataInterface(&IDeviceLifetime::IID, &lifetime)
                    .ok()?;
            }
            devices.created.fetch_add(1, Ordering::SeqCst);
            _ = completion.clone().complete(Message::Probe);
            Ok(device)
        })
        .into()
}

impl Component for CanvasFixture {
    type Input = bool;
    type Message = Message;

    fn create(_: &bool, context: &ComponentContext<Self>) -> Self {
        let devices = Arc::default();
        let invalidator = Invalidator::new();
        let lose_device = Rc::new(Cell::new(false));
        let repaint = Rc::default();
        Self {
            surface: Some(surface(
                0,
                &devices,
                &invalidator,
                &lose_device,
                &repaint,
                context,
            )),
            generation: 0,
            resized: false,
            frame: String::new(),
            dropped: Vec::new(),
            devices,
            device_status: String::new(),
            invalidator,
            lose_device,
            repaint,
            repaint_status: String::new(),
            draw_errors: 0,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Resize => self.resized = true,
            Message::Remove => {
                self.invalidator.invalidate();
                self.surface = None;
            }
            Message::Attach => {
                self.generation += 1;
                self.resized = false;
                self.surface = Some(surface(
                    self.generation,
                    &self.devices,
                    &self.invalidator,
                    &self.lose_device,
                    &self.repaint,
                    context,
                ));
            }
            Message::Frame(frame) => self.frame = frame,
            Message::Dropped(generation) => self.dropped.push(generation),
            Message::LoseDevice => {
                self.lose_device.set(true);
                self.invalidator.invalidate();
            }
            Message::Probe => {
                self.device_status = format!(
                    "Devices created {}; released {}",
                    self.devices.created.load(Ordering::SeqCst),
                    self.devices.released.load(Ordering::SeqCst),
                );
            }
            Message::Invalidate(again) => {
                self.repaint.revision.set(self.repaint.revision.get() + 1);
                self.repaint.again.set(again);
                for _ in 0..3 {
                    self.invalidator.invalidate();
                }
            }
            Message::Repainted(status) => self.repaint_status = status,
            Message::FailDraw => {
                self.repaint.fail.set(true);
                self.invalidator.invalidate();
            }
            Message::DrawError(error) => {
                assert_eq!(
                    error,
                    IntegrationError::Native(bindings::D2DERR_PUSH_POP_UNBALANCED.0)
                );
                self.draw_errors += 1;
            }
        }
    }

    fn view(&self, demand: &bool, context: &mut ViewContext<Self>) -> View {
        let surface = self.surface.as_ref().map(|surface| {
            keyed(
                self.generation,
                Border::new()
                    .width(if self.resized { 320.0 } else { 240.0 })
                    .height(if self.resized { 200.0 } else { 160.0 })
                    .content(surface.clone()),
            )
        });
        context.window_title(if *demand {
            "Reactor UI fixture: canvas-demand"
        } else {
            "Reactor UI fixture: canvas"
        });
        context.window_visuals(WindowVisuals::new().client_size(600.0, 700.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                Button::new()
                    .automation_id("resize")
                    .content("Resize canvas")
                    .on_click(context.message(Message::Resize)),
                Button::new()
                    .automation_id("remove")
                    .content("Remove canvas")
                    .on_click(context.message(Message::Remove)),
                Button::new()
                    .automation_id("attach")
                    .content("Attach canvas")
                    .on_click(context.message(Message::Attach)),
                Button::new()
                    .automation_id("lose-device")
                    .content("Inject device loss")
                    .on_click(context.message(Message::LoseDevice)),
                Button::new()
                    .automation_id("fail-draw")
                    .content("Fail drawing and callback")
                    .on_click(context.message(Message::FailDraw)),
                Button::new()
                    .automation_id("invalidate")
                    .content("Invalidate three times")
                    .on_click(context.message(Message::Invalidate(false))),
                Button::new()
                    .automation_id("invalidate-again")
                    .content("Invalidate while drawing")
                    .on_click(context.message(Message::Invalidate(true))),
                StackPanel::new().keyed_children(surface),
                TextBlock::new().automation_id("frame").text(&self.frame),
                TextBlock::new()
                    .automation_id("dropped")
                    .text(format!("Dropped {:?}", self.dropped)),
                TextBlock::new()
                    .automation_id("devices")
                    .text(&self.device_status),
                TextBlock::new()
                    .automation_id("repaint")
                    .text(&self.repaint_status),
                TextBlock::new()
                    .automation_id("draw-errors")
                    .text(format!("Draw errors {}", self.draw_errors)),
            ))
            .into()
    }
}

/// A window whose view is the canvas component itself. It exits after drawing a frame.
pub(super) struct CanvasRootFixture;

impl Application for CanvasRootFixture {
    type Input = ();
    type Message = ();

    fn create(_: &(), context: &ApplicationContext<Self>) -> Self {
        context.show_window("main");
        Self
    }

    fn update(&mut self, _: (), context: &ApplicationContext<Self>) {
        context.exit();
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        ApplicationView::new().window::<CanvasRootWindow>("main", context.message(()))
    }
}

struct CanvasRootWindow;

impl Component for CanvasRootWindow {
    type Input = Callback<()>;
    type Message = ();

    fn create(_: &Callback<()>, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, drawn: &Callback<()>, context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: canvas-root");
        let drawn = drawn.clone();
        Canvas::animated(move |frame| {
            frame.clear(ColorF::CORNFLOWER_BLUE);
            drawn.call(());
            Ok(())
        })
        .device_factory(GpuDevice::new_warp)
        .into()
    }
}
