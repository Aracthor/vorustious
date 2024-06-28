#version 320 es

precision mediump float;

in vec2 textureCoord;

uniform sampler2D uni_texture;

out vec4 frag_color;

void main()
{
    frag_color = texture(uni_texture, textureCoord);
}
