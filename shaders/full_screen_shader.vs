#version 330 core
layout(location = 0) in vec3 vertex_pos_modelspace;


out vec2 UV;

void main() {
    gl_Position = vec4(vertex_pos_modelspace, 1);
    UV = (vertex_pos_modelspace.xy + vec2(1, 1)) / 2.0;
}