#version 330 core

in vec2 a_Pos;
in vec3 color;
in vec2 uv;

uniform Proj {
    mat4 u_prop;
};

in float rotation;


out vec4 v_Color;
out vec2 out_uv;

void main() {
    v_Color = vec4(color, 1.0);
    out_uv = uv;
    gl_Position = u_prop * vec4(a_Pos , 0.0, 1.0);;
}