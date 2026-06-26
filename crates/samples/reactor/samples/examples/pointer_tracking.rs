#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (pos, set_pos) = cx.use_state(None::<(f64, f64)>);
    let (inside, set_inside) = cx.use_state(false);

    let label = match (inside, pos) {
        (true, Some((x, y))) => format!("Tracking at ({x:.0}, {y:.0})"),
        (true, None) => "Pointer entered".to_string(),
        (false, _) => "Move the pointer into the box".to_string(),
    };

    let on_enter = cx.use_callback((), {
        let set_inside = set_inside.clone();
        let set_pos = set_pos.clone();
        move |info: PointerEventInfo| {
            set_inside.call(true);
            set_pos.call(Some((info.x, info.y)));
        }
    });
    let on_move = cx.use_callback((), {
        let set_pos = set_pos.clone();
        move |info: PointerEventInfo| set_pos.call(Some((info.x, info.y)))
    });
    let on_exit = cx.use_callback((), move |()| {
        set_inside.call(false);
        set_pos.call(None);
    });

    let fill = if inside {
        Color::rgb(40, 160, 90)
    } else {
        Color::rgb(40, 120, 200)
    };

    vstack((
        TitleBar::new("windows_reactor — pointer tracking"),
        text_block(label)
            .font_size(20.0)
            .automation_id("pointer-label"),
        border(text_block("Move the pointer over me").foreground(Color::rgb(255, 255, 255)))
            .background(fill)
            .padding(Thickness::uniform(40.0))
            .width(360.0)
            .height(240.0)
            .on_pointer_entered(on_enter)
            .on_pointer_moved(on_move)
            .on_pointer_exited(on_exit),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Pointer Tracking", app)
}
