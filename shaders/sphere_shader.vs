#version 150 core

in vec2 a_Pos;
in vec3 color;
in vec2 uv;
in float rotation;


out vec4 v_Color;
out vec2 out_uv;

void main() {
    v_Color = vec4(color, 0.0);
    out_uv = uv;
    gl_Position = vec4(a_Pos , 1.0, 1.0);;
}