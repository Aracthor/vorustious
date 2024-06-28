#version 320 es

in vec2 vert_position;
in vec2 vert_texture_coord;

uniform mat4 uni_projection_matrix;

out vec2 textureCoord;

void main()
{
    gl_Position = uni_projection_matrix * vec4(vert_position, 0.0, 1.0);
    textureCoord = vert_texture_coord;
}
