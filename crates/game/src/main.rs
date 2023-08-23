mod instance;
// test
use kiss3d::light::Light;
use kiss3d::nalgebra::{UnitQuaternion, Vector3};
use kiss3d::window::Window;

fn main() {
    let mut window = Window::new("Test");

    while window.render() {}
}
