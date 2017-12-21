#version 330 core

layout(location = 0)in vec2 a_Pos;
layout(location = 1)in vec3 color;
layout(location = 2)in float rotation;
layout(location = 3)in vec2 uv;
layout(location = 4)in float border;

uniform mat4 ortho;
 
out vec4 v_Color;
out vec2 UV;
out float draw_border;

void main() {
    UV = uv;
    draw_border = border;
    v_Color = vec4(color.r, color.g, color.b, 1.0);
    gl_Position = ortho * vec4(a_Pos, 0.0, 1.0);
}