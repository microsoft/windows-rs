//! Headless tests over a live `Compositor`.
//!
//! Each test creates a dispatcher queue on its own test thread, then builds and
//! inspects composition objects. No window or message pump is involved, so these
//! run in CI. Getters read back through the same COM objects the setters wrote,
//! validating that the wrapper method routes to the correct versioned interface.

use std::time::Duration;
use windows_composition::*;

fn compositor() -> Compositor {
    let queue = DispatcherQueueController::create_on_current_thread().unwrap();
    let compositor = Compositor::new().unwrap();
    // The compositor needs the thread's queue to stay alive; leak it for the
    // remainder of the (short-lived) test process rather than thread it through
    // every caller.
    std::mem::forget(queue);
    compositor
}

#[test]
fn visual_property_round_trips() {
    let c = compositor();
    let v = c.create_sprite_visual().unwrap();

    v.set_offset(3.0, 4.0, 5.0).unwrap();
    let o = v.offset().unwrap();
    assert_eq!((o.x, o.y, o.z), (3.0, 4.0, 5.0));

    v.set_size(120.0, 80.0).unwrap();
    let s = v.size().unwrap();
    assert_eq!((s.x, s.y), (120.0, 80.0));

    v.set_opacity(0.25).unwrap();
    assert_eq!(v.opacity().unwrap(), 0.25);

    v.set_visible(false).unwrap();
    assert!(!v.is_visible().unwrap());
    v.set_visible(true).unwrap();
    assert!(v.is_visible().unwrap());

    // Exercises the IVisual2 cast path (RelativeSizeAdjustment) and setters that
    // have no getter — a failure here surfaces as an HRESULT error.
    v.set_scale(Vector3::new(2.0, 2.0, 1.0)).unwrap();
    v.set_center_point(Vector3::new(60.0, 40.0, 0.0)).unwrap();
    v.set_anchor_point(Vector2::new(0.5, 0.5)).unwrap();
    v.set_border_mode(BorderMode::Soft).unwrap();
    v.set_relative_size_adjustment(Vector2::new(1.0, 1.0))
        .unwrap();
}

#[test]
fn container_children_insert_and_remove() {
    let c = compositor();
    let root = c.create_container_visual().unwrap();
    let a = c.create_sprite_visual().unwrap();
    let b = c.create_sprite_visual().unwrap();

    let children = root.children().unwrap();
    assert_eq!(children.count().unwrap(), 0);

    children.insert_at_top(&a).unwrap();
    children.insert_at_bottom(&b).unwrap();
    assert_eq!(children.count().unwrap(), 2);

    // A sprite visual is itself a container: multi-step Deref lets it pass here.
    assert_eq!(a.parent().unwrap().children().unwrap().count().unwrap(), 2);

    children.remove(&a).unwrap();
    assert_eq!(children.count().unwrap(), 1);
    children.remove_all().unwrap();
    assert_eq!(children.count().unwrap(), 0);
}

#[test]
fn color_brush_round_trips() {
    let c = compositor();
    let brush = c.create_color_brush(Color::rgb(10, 20, 30)).unwrap();
    assert_eq!(brush.color().unwrap(), Color::rgb(10, 20, 30));

    brush.set_color(Color::rgba(1, 2, 3, 4)).unwrap();
    assert_eq!(brush.color().unwrap(), Color::rgba(1, 2, 3, 4));

    let visual = c.create_sprite_visual().unwrap();
    visual.set_brush(&brush).unwrap();
}

#[test]
fn nine_grid_brush_builds() {
    let c = compositor();
    let source = c.create_color_brush(Color::rgb(0, 0, 0)).unwrap();
    let nine = c.create_nine_grid_brush().unwrap();
    nine.set_insets(4.0, 4.0, 4.0, 4.0).unwrap();
    nine.set_center_hollow(true).unwrap();
    nine.set_source(&source).unwrap();

    let visual = c.create_sprite_visual().unwrap();
    visual.set_brush(&nine).unwrap();
}

#[test]
fn shape_visual_hierarchy_builds() {
    let c = compositor();
    let shape_visual = c.create_shape_visual().unwrap();
    let container = c.create_container_shape().unwrap();
    let geometry = c.create_ellipse_geometry().unwrap();
    geometry.set_radius(Vector2::new(8.0, 8.0)).unwrap();

    let sprite = c.create_sprite_shape(&geometry).unwrap();
    let fill = c.create_color_brush(Color::rgb(255, 0, 0)).unwrap();
    sprite.set_fill_brush(&fill).unwrap();
    sprite.set_offset(Vector2::new(1.0, 2.0)).unwrap();

    container.shapes().unwrap().append(&sprite).unwrap();
    shape_visual.shapes().unwrap().append(&container).unwrap();
}

#[test]
fn key_frame_animation_starts_on_visual() {
    let c = compositor();
    let animation = c.create_vector3_key_frame_animation().unwrap();
    animation
        .insert_key_frame(0.0, Vector3::new(0.0, 0.0, 0.0))
        .unwrap();
    animation
        .insert_key_frame(1.0, Vector3::new(2.0, 2.0, 1.0))
        .unwrap();
    animation.set_duration(Duration::from_millis(300)).unwrap();
    animation.set_delay(Duration::from_millis(50)).unwrap();
    animation.set_iteration_count(3).unwrap();

    let visual = c.create_sprite_visual().unwrap();
    visual
        .set_center_point(Vector3::new(0.0, 0.0, 0.0))
        .unwrap();
    visual.start_animation("Scale", &animation).unwrap();
}

#[test]
fn scoped_batch_ends() {
    let c = compositor();
    let batch = c.create_scoped_batch(BatchKind::Animation).unwrap();

    let animation = c.create_vector3_key_frame_animation().unwrap();
    animation
        .insert_key_frame(1.0, Vector3::new(1.0, 1.0, 1.0))
        .unwrap();
    animation.set_duration(Duration::from_millis(100)).unwrap();

    let visual = c.create_sprite_visual().unwrap();
    visual.start_animation("Offset", &animation).unwrap();
    batch.end().unwrap();
}

#[test]
fn visual_reports_its_compositor() {
    let c = compositor();
    let visual = c.create_sprite_visual().unwrap();
    // Round-trips through ICompositionObject::Compositor and builds another object.
    let from_visual = visual.compositor().unwrap();
    from_visual.create_container_visual().unwrap();
}
