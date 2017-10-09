#version 400 core

in vec4 v_Color;
in vec2 out_uv;


out vec4 out_color;

void main() {
    out_color = texture(tex, uv) + v_Color;
}