//! Sequence two transitions on a storyboard: rise to a peak, then fall back.

use animation_samples::*;

fn main() -> Result<()> {
    init_com();

    let manager = Manager::new()?;
    let library = TransitionLibrary::new()?;
    let variable = manager.create_variable(0.0)?;

    let storyboard = manager.create_storyboard()?;

    let rise = library.accelerate_decelerate(1.0, 180.0, 0.3, 0.7)?;
    let keyframe = storyboard.add_transition(&variable, &rise)?;

    let fall = library.accelerate_decelerate(1.0, 0.0, 0.3, 0.7)?;
    storyboard.add_transition_at_keyframe(&variable, &fall, keyframe)?;

    storyboard.schedule(0.0)?;

    for step in 0..=20 {
        let time = f64::from(step) / 10.0;
        manager.update(time)?;
        let value = variable.value()?;
        println!("t={time:.1}s  value={value:6.2}  {}", bar(value, 180.0));
    }

    Ok(())
}
