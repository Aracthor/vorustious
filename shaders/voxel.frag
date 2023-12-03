#version 320 es

precision mediump float;

in vec2 texture_coord;
in flat int texture_index;
in flat float damage;

uniform sampler2D voxel_texture[2];
uniform sampler2D damage_texture;
uniform float uni_alpha;

out vec4 frag_color;

vec3 lerp(vec3 a, vec3 b, float t)
{
    return (1.0 - t) * a + t * b;
}

void main()
{
    frag_color.rgb = texture(voxel_texture[texture_index], texture_coord).rgb;
    if (damage > 0.0)
    {
        vec4 damage_color = texture(damage_texture, texture_coord);
        frag_color.rgb = lerp(frag_color.rgb, damage_color.rgb, damage_color.a * damage);
    }
    frag_color.a = uni_alpha;
}
