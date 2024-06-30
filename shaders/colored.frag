#version 320 es

precision mediump float;

in vec4 color;

out vec4 frag_color;

void main()
{
    frag_color = color;
}
