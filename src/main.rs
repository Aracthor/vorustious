mod graphic;

use graphic::material::Material;

fn main() {
    let mut window = graphic::window::Window::create_window(800, 600, "Vorustious");

    let mut material = Material::create("shaders/hello_texture.vert", "shaders/hello_texture.frag");

    let texture = graphic::cube::cube_texture([0x40, 0x40, 0x40, 0xFF], [0x80, 0x80, 0x80, 0xFF]);
    material.add_texture(texture);

    let mesh = graphic::cube::cube_mesh(material);

    while !window.should_close() {
        window.clear();

        mesh.draw();

        window.refresh();
        window.update_events();
    }
}
