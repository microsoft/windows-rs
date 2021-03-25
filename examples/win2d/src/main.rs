use bindings::Microsoft::Graphics::Canvas::CanvasDevice;

fn main() {
    let _device = CanvasDevice::new().expect("Failed to create CanvasDevice");

    println!("ok!");
}
