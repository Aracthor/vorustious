#version 320 es

in vec3 vert_position;
in vec2 vert_texture_coords;

out vec2 texture_coord;

void main()
{
    gl_Position = vec4(vert_position, 1.0);
    texture_coord = vert_texture_coords;
}
