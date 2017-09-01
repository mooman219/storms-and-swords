#version 150 core

in vec2 a_Pos;
in vec3 a_Color;

uniform Transform {
    mat4 u_Transform;
    mat4 u_Scale;
    mat4 u_Rotation;
};

out vec4 v_Color;

void main() {
    v_Color = vec4(a_Color, 0.0);
    gl_Position = (u_Scale * u_Transform) * vec4(a_Pos, 0.0, 1.0);
}