#version 320 es

precision mediump float;

uniform vec4 uni_color;

out vec4 frag_color;

void main()
{
    frag_color = uni_color;
}
