#version 320 es

precision mediump float;

in vec2 texture_coord;

uniform sampler2D diffuse_texture;
uniform float uni_alpha;

out vec4 frag_color;

void main()
{
    frag_color = texture(diffuse_texture, texture_coord);
    frag_color.a *= uni_alpha;
}
