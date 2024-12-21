#version 320 es

in vec3 vert_position;
in vec2 vert_texture_coord;
in vec3 instance_position;
in int instance_texture_index;
in float instance_damage;

uniform mat4 uni_model_matrix;
uniform mat4 uni_projection_view_matrix;

out vec2 texture_coord;
out flat int texture_index;
out flat float damage;

void main()
{
    gl_Position = uni_projection_view_matrix * uni_model_matrix * vec4(vert_position + instance_position, 1.0);
    texture_coord = vert_texture_coord;
    texture_index = instance_texture_index;
    damage = instance_damage;
}
