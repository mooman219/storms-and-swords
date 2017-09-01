#version 150 core

in vec4 v_Color;
out vec4 Target0;

void main() {
    Target0 = vec4(v_Color.r ,v_Color.g, v_Color.b, 1);
}