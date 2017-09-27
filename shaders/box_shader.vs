#version 330 core

in vec2 a_Pos;
in vec3 color;
in float rotation;

uniform Pro {
    mat4 u_prop;
};

out vec4 v_Color;

void main() {
    v_Color = vec4(color, 1.0);
    gl_Position = u_prop * vec4(a_Pos , 0.0, 1.0);
}