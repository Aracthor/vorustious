#version 320 es

in vec3 vert_position;
in vec4 vert_color;

uniform mat4 uni_model_matrix;
uniform mat4 uni_projection_view_matrix;

out vec4 color;

void main()
{
    gl_Position = uni_projection_view_matrix * uni_model_matrix * vec4(vert_position, 1.0);
    color = vert_color;
}
