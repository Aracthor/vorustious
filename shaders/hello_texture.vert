#version 320 es

in vec3 vert_position;
in vec2 vert_texture_coords;

uniform mat4 uni_projection_matrix;
uniform mat4 uni_view_matrix;

out vec2 texture_coord;

void main()
{
    gl_Position = uni_projection_matrix * uni_view_matrix * vec4(vert_position, 1.0);
    texture_coord = vert_texture_coords;
}
