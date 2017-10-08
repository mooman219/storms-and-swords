#version 330 core

layout(location = 0)in vec2 a_Pos;
layout(location = 1)in vec3 color;
layout(location = 2)in float rotation;

uniform mat4 ortho;

out vec4 v_Color;

void main() {
    v_Color = vec4(color, 1.0);
    gl_Position = ortho * vec4(a_Pos, 0.0, 1.0);
}