use super::material::Material;
use super::mesh::Mesh;
use super::opengl::texture::Texture;
use super::opengl::vertex_objects::Primitive;

pub fn cube_texture(border_color: [u8; 4], fill_color: [u8; 4]) -> Texture {
    let b_r = border_color[0];
    let b_g = border_color[1];
    let b_b = border_color[2];
    let b_a = border_color[3];
    let f_r = fill_color[0];
    let f_g = fill_color[1];
    let f_b = fill_color[2];
    let f_a = fill_color[3];
    let texture_pixels: Vec<u8> = [
        b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,
        b_r, b_g, b_b, b_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  b_r, b_g, b_b, b_a,
        b_r, b_g, b_b, b_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  b_r, b_g, b_b, b_a,
        b_r, b_g, b_b, b_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  b_r, b_g, b_b, b_a,
        b_r, b_g, b_b, b_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  b_r, b_g, b_b, b_a,
        b_r, b_g, b_b, b_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  b_r, b_g, b_b, b_a,
        b_r, b_g, b_b, b_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  f_r, f_g, f_b, f_a,  b_r, b_g, b_b, b_a,
        b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,  b_r, b_g, b_b, b_a,
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
        1.0, 1.0,
        1.0, 0.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,
        ].to_vec();

    Mesh::create(positions, tex_coords, Primitive::Triangles, material)
}