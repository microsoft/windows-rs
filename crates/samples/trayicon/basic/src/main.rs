use windows_trayicon::{Menu, TrayIcon, TrayIconEvent};

#[derive(Clone, Copy)]
enum Command {
    Activate,
    Exit,
}

fn main() -> windows_trayicon::Result<()> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\..\\..\\reactor\\icon\\icon.ico"
    );
    let commands = std::rc::Rc::new(std::cell::RefCell::new(std::collections::VecDeque::new()));
    let callback_commands = std::rc::Rc::clone(&commands);
    let started = std::time::Instant::now();
    let mut icon = TrayIcon::new(path)
        .tooltip("Select to update; use the context menu to exit")
        .menu(Menu::new().item(1, "Exit"))
        .on_event(move |event| match event {
            TrayIconEvent::Activate { .. } => {
                println!(
                    "{:>6} ms: activation request",
                    started.elapsed().as_millis()
                );
                callback_commands.borrow_mut().push_back(Command::Activate);
            }
            TrayIconEvent::Unavailable => {
                println!(
                    "{:>6} ms: the Windows Shell could not restore the icon",
                    started.elapsed().as_millis()
                );
                callback_commands.borrow_mut().push_back(Command::Exit);
            }
            TrayIconEvent::MenuItem { id: 1 } => {
                println!("{:>6} ms: Exit selected", started.elapsed().as_millis());
                callback_commands.borrow_mut().push_back(Command::Exit);
            }
            _ => {}
        })
        .build()?;
    let mut activations = 0;
    windows_window::run_with(move || {
        loop {
            let Some(command) = commands.borrow_mut().pop_front() else {
                break;
            };
            match command {
                Command::Activate => {
                    activations += 1;
                    icon.set_icon(path)?;
                    icon.set_tooltip(Some(&format!("Activation {activations}")))?;
                    println!("           processed activation {activations}");
                }
                Command::Exit => windows_window::quit(),
            }
        }
        Ok(false)
    })
}
