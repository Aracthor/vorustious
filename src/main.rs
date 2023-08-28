mod window;

fn main() {
    let mut window = window::Window::create_window(800, 600, "Vorustious");
    while !window.should_close() {
        window.clear();
        window.refresh();
        window.update_events();
    }
}
