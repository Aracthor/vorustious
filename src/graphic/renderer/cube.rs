use super::super::core::color::Color;
use super::super::meshes::material::Material;
use super::super::meshes::mesh::Mesh;
use super::super::opengl::texture::Texture;
use super::super::opengl::vertex_objects::Primitive;
use crate::maths::matrix::Mat4f;
use crate::voxels::voxel::TextureType;

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

pub fn cube_mesh(alpha: f32) -> Mesh {

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

    let material = {
        let mut material = Material::create("shaders/voxel.vert", "shaders/voxel.frag");

        material.add_instance_data_buffer("instance_position", 3);
        material.add_instance_data_buffer("instance_texture_index", 1);
        material.add_instance_data_buffer("instance_damage", 1);
        material.add_uniform_mat4("uni_model_matrix", Mat4f::identity());

        let hull_texture = cube_texture(Color::new(0x40, 0x40, 0x40, 0xFF), Color::new(0x80, 0x80, 0x80, 0xFF));
        material.add_texture(&format!("voxel_texture[{}]", TextureType::LightHull as i32), hull_texture);
        let hull_texture = cube_texture(Color::new(0x20, 0x20, 0x20, 0xFF), Color::new(0x30, 0x30, 0x30, 0xFF));
        material.add_texture(&format!("voxel_texture[{}]", TextureType::DarkHull as i32), hull_texture);
        let core_texture = cube_texture(Color::new(0x80, 0x80, 0x80, 0xFF), Color::new(0x80, 0x80, 0xFF, 0xFF));
        material.add_texture(&format!("voxel_texture[{}]", TextureType::Core as i32), core_texture);
        let canon_texture = circle_texture(Color::new(0x80, 0x80, 0x80, 0xFF), Color::new(0x20, 0x20, 0x20, 0xFF));
        material.add_texture(&format!("voxel_texture[{}]", TextureType::Canon as i32), canon_texture);

        let damage_texture = {
            let b = Color::black();
            let t = Color::transparent();
            let texture_pixels = [
                t,  b,  t,  t,  t,  t,  t,  b,
                t,  b,  t,  t,  t,  b,  b,  t,
                t,  b,  t,  t,  b,  t,  t,  b,
                t,  t,  b,  t,  b,  b,  b,  b,
                t,  t,  b,  b,  b,  t,  t,  t,
                t,  b,  t,  b,  t,  b,  t,  b,
                b,  t,  t,  t,  t,  b,  t,  b,
                t,  t,  t,  t,  t,  t,  b,  b,
            ].to_vec();
            Texture::create(8, 8, texture_pixels)
        };
        material.add_texture("damage_texture", damage_texture);

        material.add_uniform_f32("uni_alpha", alpha);
        material
    };

    let mut mesh = Mesh::create(Primitive::Triangles, false, material);
    mesh.set_positions_3d(&positions);
    mesh.set_texture_coords(&tex_coords);
    mesh
}