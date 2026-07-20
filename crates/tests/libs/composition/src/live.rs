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
    let v = c.create_sprite_visual();

    v.set_offset(3.0, 4.0, 5.0);
    let o = v.offset();
    assert_eq!((o.x, o.y, o.z), (3.0, 4.0, 5.0));

    v.set_size(120.0, 80.0);
    let s = v.size();
    assert_eq!((s.x, s.y), (120.0, 80.0));

    v.set_opacity(0.25);
    assert_eq!(v.opacity(), 0.25);

    v.set_visible(false);
    assert!(!v.is_visible());
    v.set_visible(true);
    assert!(v.is_visible());

    // Exercises the IVisual2 cast path (RelativeSizeAdjustment) and setters that
    // have no getter — a failure here surfaces as a panic.
    v.set_scale(Vector3::new(2.0, 2.0, 1.0));
    v.set_center_point(Vector3::new(60.0, 40.0, 0.0));
    v.set_anchor_point(Vector2::new(0.5, 0.5));
    v.set_border_mode(BorderMode::Soft);
    v.set_relative_size_adjustment(Vector2::new(1.0, 1.0));
}

#[test]
fn container_children_insert_and_remove() {
    let c = compositor();
    let root = c.create_container_visual();
    let a = c.create_sprite_visual();
    let b = c.create_sprite_visual();

    let children = root.children();
    assert_eq!(children.count(), 0);

    children.insert_at_top(&a);
    children.insert_at_bottom(&b);
    assert_eq!(children.count(), 2);

    // A sprite visual is itself a container: multi-step Deref lets it pass here.
    assert_eq!(a.parent().children().count(), 2);

    children.remove(&a);
    assert_eq!(children.count(), 1);
    children.remove_all();
    assert_eq!(children.count(), 0);
}

#[test]
fn color_brush_round_trips() {
    let c = compositor();
    let brush = c.create_color_brush(Color::rgb(10, 20, 30));
    assert_eq!(brush.color(), Color::rgb(10, 20, 30));

    brush.set_color(Color::rgba(1, 2, 3, 4));
    assert_eq!(brush.color(), Color::rgba(1, 2, 3, 4));

    let visual = c.create_sprite_visual();
    visual.set_brush(&brush);
}

#[test]
fn nine_grid_brush_builds() {
    let c = compositor();
    let source = c.create_color_brush(Color::rgb(0, 0, 0));
    let nine = c.create_nine_grid_brush();
    nine.set_insets(4.0, 4.0, 4.0, 4.0);
    nine.set_center_hollow(true);
    nine.set_source(&source);

    let visual = c.create_sprite_visual();
    visual.set_brush(&nine);
}

#[test]
fn shape_visual_hierarchy_builds() {
    let c = compositor();
    let shape_visual = c.create_shape_visual();
    let container = c.create_container_shape();
    let geometry = c.create_ellipse_geometry();
    geometry.set_radius(Vector2::new(8.0, 8.0));

    let sprite = c.create_sprite_shape(&geometry);
    let fill = c.create_color_brush(Color::rgb(255, 0, 0));
    sprite.set_fill_brush(&fill);
    sprite.set_offset(Vector2::new(1.0, 2.0));

    container.shapes().append(&sprite);
    shape_visual.shapes().append(&container);
}

#[test]
fn key_frame_animation_starts_on_visual() {
    let c = compositor();
    let animation = c.create_vector3_key_frame_animation();
    animation.insert_key_frame(0.0, Vector3::new(0.0, 0.0, 0.0));
    animation.insert_key_frame(1.0, Vector3::new(2.0, 2.0, 1.0));
    animation.set_duration(Duration::from_millis(300));
    animation.set_delay(Duration::from_millis(50));
    animation.set_iteration_count(3);

    let visual = c.create_sprite_visual();
    visual.set_center_point(Vector3::new(0.0, 0.0, 0.0));
    visual.start_animation("Scale", &animation);
}

#[test]
fn scoped_batch_ends() {
    let c = compositor();
    let batch = c.create_scoped_batch(BatchKind::Animation);

    let animation = c.create_vector3_key_frame_animation();
    animation.insert_key_frame(1.0, Vector3::new(1.0, 1.0, 1.0));
    animation.set_duration(Duration::from_millis(100));

    let visual = c.create_sprite_visual();
    visual.start_animation("Offset", &animation);
    batch.end();
}

#[test]
fn visual_reports_its_compositor() {
    let c = compositor();
    let visual = c.create_sprite_visual();
    // Round-trips through ICompositionObject::Compositor and builds another object.
    let from_visual = visual.compositor();
    from_visual.create_container_visual();
}
