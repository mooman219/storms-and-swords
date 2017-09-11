#version 150 core

in vec2 a_Pos;
in vec3 color;
in float rotation;


out vec4 v_Color;

void main() {
    v_Color = vec4(color, 0.0);
    gl_Position = vec4(a_Pos , 1.0, 1.0);;
}