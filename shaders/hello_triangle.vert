#version 320 es

in vec3 vertPosition;

void main()
{
    gl_Position = vec4(vertPosition, 1.0);
}
