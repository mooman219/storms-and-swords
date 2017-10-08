#version 330 core

layout(location = 0)in vec2 a_Pos;
layout(location = 1)in vec3 color;
layout(location = 2)in vec2 uv;

uniform mat4 ortho;


in float rotation;


out vec4 v_Color;
out vec2 out_uv;

void main() {
    v_Color = vec4(color, 1.0);
    out_uv = uv;
    gl_Position = ortho * vec4(a_Pos, 0.0, 1.0);//u_prop * vec4(a_Pos , 0.0, 1.0);;
}