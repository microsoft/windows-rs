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
}

#[derive(Default)]
struct Repaint {
    revision: Cell<u32>,
    again: Cell<bool>,
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
        .device_factory(move || {
            let device = GpuDevice::new_warp()?;
            let native: bindings::ID3D11Device = device.d3d_device().cast()?;
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
