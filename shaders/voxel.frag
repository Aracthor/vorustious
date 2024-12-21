#version 320 es

precision mediump float;

in vec3 normal;
in vec2 texture_coord;
in flat int texture_index;
in flat float damage;

uniform sampler2D voxel_texture[4];
uniform sampler2D damage_texture;
uniform float uni_alpha;

// TODO uniforms ?
const vec3 light_direction = normalize(vec3(1.0, 2.0, -3.0));
const float light_power = 0.5;
const float ambient_power = 0.5;

out vec4 frag_color;

vec3 lerp(vec3 a, vec3 b, float t)
{
    return (1.0 - t) * a + t * b;
}

float get_light_power()
{
    // Assuming normal is normalized.
    float cos_angle = dot(normal, -light_direction);
    float dir_light_power = max(cos_angle, 0.f) * light_power;
    return min(ambient_power + dir_light_power, 1.f);
}

void main()
{
    frag_color.rgb = texture(voxel_texture[texture_index], texture_coord).rgb;
    if (damage > 0.0)
    {
        vec4 damage_color = texture(damage_texture, texture_coord);
        frag_color.rgb = lerp(frag_color.rgb, damage_color.rgb, damage_color.a * damage);
    }
    frag_color.rgb *= get_light_power();
    frag_color.a = uni_alpha;
}
