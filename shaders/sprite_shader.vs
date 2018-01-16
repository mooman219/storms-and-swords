#version 400 core

layout(location = 0)in vec3 a_Pos;
layout(location = 1)in vec2 uv;

uniform mat4 ortho;


in float rotation;


out vec4 v_Color;
out vec2 out_uv;

void main() {
    out_uv = uv;
    gl_Position = ortho * vec4(a_Pos, 1.0);
}