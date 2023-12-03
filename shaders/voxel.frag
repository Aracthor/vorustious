#version 320 es

precision mediump float;

in vec2 texture_coord;

uniform sampler2D diffuse_texture;
uniform sampler2D damage_texture;
uniform float uni_alpha;

out vec4 frag_color;

vec3 lerp(vec3 a, vec3 b, float t)
{
    return (1.0 - t) * a + t * b;
}

void main()
{
    vec4 damage_color = texture(damage_texture, texture_coord);
    vec3 diffuse_color = texture(diffuse_texture, texture_coord).rgb;
    frag_color.rgb = lerp(diffuse_color, damage_color.rgb, damage_color.a);
    frag_color.a = uni_alpha;
}
