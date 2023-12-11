#version 320 es

in vec3 vert_position;

uniform mat4 uni_model_matrix;
uniform mat4 uni_projection_view_matrix;

void main()
{
    gl_Position = uni_projection_view_matrix * uni_model_matrix * vec4(vert_position, 1.0);
}
