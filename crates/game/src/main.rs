mod instance;
// test

use kiss3d::window::Window;

fn main() {
    let mut window = Window::new("Test");

    while window.render() {}
}
