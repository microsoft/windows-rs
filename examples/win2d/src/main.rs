use bindings::microsoft::graphics::canvas::CanvasDevice;

fn main() {
    let _device = CanvasDevice::new().expect("Failed to create CanvasDevice");

    println!("ok!");
}
