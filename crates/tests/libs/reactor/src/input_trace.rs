use bindings::*;
use std::{cell::RefCell, rc::Rc};
use windows_core::{EventRevoker, Interface, Ref};
use windows_reactor::{Callback, WindowSize};

#[allow(dead_code, nonstandard_style, clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "\\input_trace_bindings.rs"));
}

pub struct InputTrace {
    events: Rc<RefCell<Vec<EventRevoker>>>,
}

impl InputTrace {
    pub fn new() -> Self {
        Self {
            events: Rc::default(),
        }
    }

    pub fn on_size(&self) -> Callback<WindowSize> {
        let weak = Rc::downgrade(&self.events);
        Callback::new(move |_| {
            if let Some(events) = weak.upgrade() {
                let mut events = events.borrow_mut();
                if events.is_empty() {
                    *events = subscribe().unwrap();
                }
            }
        })
    }
}

pub(super) fn subscribe() -> windows_core::Result<Vec<EventRevoker>> {
    let compositor = CompositionTarget::GetCompositorForCurrentThread()?;
    let islands = ContentIsland::FindAllForCompositor(&compositor)?;
    assert!(!islands.is_empty(), "input trace found no content islands");
    let mut events = Vec::new();
    for island in islands.iter() {
        let island = island.as_ref().unwrap();
        state(island, "install")?;
        let id = island.Id()?;
        events.push(island.StateChanged(|sender, _| {
            state(sender.unwrap(), "changed").unwrap();
        })?);
        let activation = InputActivationListener::GetForIsland(island)?;
        eprintln!(
            "native island activation: id {id:?}, state {:?}, tick {}",
            activation.State()?,
            unsafe { GetTickCount() },
        );
        events.push(activation.InputActivationChanged(move |sender, _| {
            eprintln!(
                "native island activation changed: id {id:?}, state {:?}, tick {}",
                sender.unwrap().State().unwrap(),
                unsafe { GetTickCount() },
            );
        })?);
        let source = InputPointerSource::GetForIsland(island)?;
        let handler = move |name| {
            move |_: Ref<InputPointerSource>, args: Ref<PointerEventArgs>| {
                let args = args.unwrap();
                let point = args.CurrentPoint().unwrap();
                let position = point.Position().unwrap();
                let properties = point.Properties().unwrap();
                eprintln!(
                    "native island input: id {id:?}, {name}, point ({}, {}), pointer {}, \
                     delta {}, horizontal {}, modifiers {:?}, handled {}, timestamp {}, tick {}",
                    position.x,
                    position.y,
                    point.PointerId().unwrap(),
                    properties.MouseWheelDelta().unwrap(),
                    properties.IsHorizontalMouseWheel().unwrap(),
                    args.KeyModifiers().unwrap(),
                    args.Handled().unwrap(),
                    point.Timestamp().unwrap(),
                    unsafe { GetTickCount() },
                );
            }
        };
        events.push(source.PointerEntered(handler("entered"))?);
        events.push(source.PointerExited(handler("exited"))?);
        events.push(source.PointerMoved(handler("moved"))?);
        events.push(source.PointerPressed(handler("pressed"))?);
        events.push(source.PointerReleased(handler("released"))?);
        events.push(source.PointerWheelChanged(handler("wheel"))?);
        eprintln!("native island input: id {id:?}, subscribed at {}", unsafe {
            GetTickCount()
        },);
    }
    Ok(events)
}

fn state(island: &ContentIsland, phase: &str) -> windows_core::Result<()> {
    eprintln!(
        "native island {phase}: id {:?}, connected {}, island enabled {}, visible {}, \
         site enabled {}, visible {}, processes pointer input {}, tick {}",
        island.Id()?,
        island.IsConnected()?,
        island.IsIslandEnabled()?,
        island.IsIslandVisible()?,
        island.IsSiteEnabled()?,
        island.IsSiteVisible()?,
        island.cast::<IContentIsland2>()?.ProcessesPointerInput()?,
        unsafe { GetTickCount() },
    );
    Ok(())
}
