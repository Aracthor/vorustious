use super::core::color::Color;
use super::meshes::material::Material;
use super::meshes::mesh::Mesh;
use super::opengl::texture::Texture;
use super::opengl::vertex_objects::Primitive;

pub fn cube_texture(border_color: Color, fill_color: Color) -> Texture {
    let b = border_color;
    let f = fill_color;
    let texture_pixels: Vec<Color> = [
        b,  b,  b,  b,  b,  b,  b,  b,
        b,  f,  f,  f,  f,  f,  f,  b,
        b,  f,  f,  f,  f,  f,  f,  b,
        b,  f,  f,  f,  f,  f,  f,  b,
        b,  f,  f,  f,  f,  f,  f,  b,
        b,  f,  f,  f,  f,  f,  f,  b,
        b,  f,  f,  f,  f,  f,  f,  b,
        b,  b,  b,  b,  b,  b,  b,  b,
    ].to_vec();
    Texture::create(8, 8, texture_pixels)
}

pub fn circle_texture(border_color: Color, fill_color: Color) -> Texture {
    let b = border_color;
    let f = fill_color;
    let texture_pixels: Vec<Color> = [
        b,  b,  b,  b,  b,  b,  b,  b,
        b,  b,  b,  f,  f,  b,  b,  b,
        b,  b,  f,  f,  f,  f,  b,  b,
        b,  f,  f,  f,  f,  f,  f,  b,
        b,  f,  f,  f,  f,  f,  f,  b,
        b,  b,  f,  f,  f,  f,  b,  b,
        b,  b,  b,  f,  f,  b,  b,  b,
        b,  b,  b,  b,  b,  b,  b,  b,
    ].to_vec();
    Texture::create(8, 8, texture_pixels)
}

pub fn cube_mesh(material: Material) -> Mesh {

    let positions: Vec<f32> = [
        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,

        -0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5, -0.5,  0.5,

        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5,

        0.5,  0.5,  0.5,
        0.5,  0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,

        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        -0.5, -0.5,  0.5,
        -0.5, -0.5, -0.5,

        -0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
    ].to_vec();

    let tex_coords: Vec<f32> = [
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,

        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,

        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        1.0, 0.0,
        0.0, 0.0,
        0.0, 1.0,

        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        1.0, 0.0,
        0.0, 0.0,
        0.0, 1.0,
        ].to_vec();

    let mut mesh = Mesh::create(Primitive::Triangles, false, material);
    mesh.set_positions_3d(&positions);
    mesh.set_texture_coords(&tex_coords);
    mesh
}