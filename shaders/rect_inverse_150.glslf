#version 150 core

uniform int i_time;
in vec4 v_Color;
out vec4 Target0;

void main() {
    Target0 = vec4(v_Color.r, sin(i_time / 100.0), 1 - v_Color.b, 1);
}