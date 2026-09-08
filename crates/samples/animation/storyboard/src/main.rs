use windows_animation::*;

fn main() -> Result<()> {
    windows_core::init_mta()?;

    let manager = Manager::new()?;
    let library = TransitionLibrary::new()?;

    let direct = manager.create_variable(0.0)?;
    let transition = library.accelerate_decelerate(1.0, 100.0, 0.3, 0.7)?;
    manager.schedule_transition(&direct, &transition, 0.0)?;

    let sequenced = manager.create_variable(0.0)?;
    let storyboard = manager.create_storyboard()?;

    let rise = library.accelerate_decelerate(1.0, 180.0, 0.3, 0.7)?;
    let keyframe = storyboard.add_transition(&sequenced, &rise)?;

    let fall = library.accelerate_decelerate(1.0, 0.0, 0.3, 0.7)?;
    storyboard.add_transition_at_keyframe(&sequenced, &fall, keyframe)?;

    storyboard.schedule(0.0)?;

    for step in 0..=20 {
        let time = f64::from(step) / 10.0;
        manager.update(time)?;
        let direct = direct.value()?;
        let sequenced = sequenced.value()?;
        println!("t={time:.1}s  direct={direct:6.2}  storyboard={sequenced:6.2}");
    }

    Ok(())
}
