#![windows_subsystem = "windows"]

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;

use windows_reactor::{
    Application, CancellationToken, Resource, Window, button, component, fragment, stack_panel,
    text_block,
};

fn load_resource(cancel: CancellationToken, key: u32) -> windows_core::Result<String> {
    let repetitions = if key == 1 { 32 } else { 6 };
    for _ in 0..repetitions {
        if cancel.is_cancelled() {
            return Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004004_u32 as i32),
                "resource load cancelled",
            ));
        }
        thread::sleep(Duration::from_millis(25));
    }
    if key == 2 {
        Err(windows_core::Error::new(
            windows_core::HRESULT(0x80004005_u32 as i32),
            format!("failure {key}"),
        ))
    } else {
        Ok(format!("value {key}"))
    }
}

fn main() -> windows_core::Result<()> {
    let effects = Arc::new(AtomicU32::new(0));
    let cleanups = Arc::new(AtomicU32::new(0));
    let root = component(move |cx| {
        let generation = cx.use_state(|| 0u32);
        let current_generation = generation.value();
        let show_effect = cx.use_state(|| true);
        let refresh = cx.use_state(|| false);
        let resource_key = cx.use_state(|| 0u32);
        let current_resource_key = resource_key.value();
        let resource = cx.use_resource(current_resource_key, load_resource);

        let effect = if show_effect.value() {
            let effects = Arc::clone(&effects);
            let cleanups = Arc::clone(&cleanups);
            component(move |cx| {
                let effects = Arc::clone(&effects);
                let cleanups = Arc::clone(&cleanups);
                cx.use_effect_with_cleanup(current_generation, move || {
                    effects.fetch_add(1, Ordering::Relaxed);
                    move || {
                        cleanups.fetch_add(1, Ordering::Relaxed);
                    }
                });
                text_block(format!("Effect mounted: {current_generation}"))
            })
            .key(1)
        } else {
            fragment([])
        };
        let resource_status = match resource {
            Resource::Loading => "Resource: loading".to_string(),
            Resource::Ready(value) => format!("Resource: ready {value}"),
            Resource::Failed(_) => format!("Resource: error {current_resource_key}"),
        };
        let current_refresh = refresh.value();

        let content = stack_panel([
            effect,
            text_block(format!("Effects: {}", effects.load(Ordering::Relaxed))),
            text_block(format!("Cleanups: {}", cleanups.load(Ordering::Relaxed))),
            button("Change effect dependency", move || {
                generation.update(|value| *value += 1);
            }),
            button("Remove effect", move || {
                show_effect.set(false);
            }),
            button("Refresh hook status", move || {
                refresh.set(!current_refresh);
            }),
            text_block(format!("Resource key: {current_resource_key}")),
            text_block(resource_status),
            button("Advance resource", move || {
                resource_key.update(|value| *value += 1);
            }),
        ]);
        Application::new(
            [Window::new("windows-reactor matched hooks", content, || {})
                .client_size(480.0, 520.0)
                .build()
                .key(0)],
        )
        .build()
    });
    windows_reactor::run_reactor_winui_app(root)
}
