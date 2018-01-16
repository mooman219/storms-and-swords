#version 400 core

in vec2 out_uv;

out vec4 out_color;

uniform sampler2D tex;

void main() {
    out_color = vec4(texture(tex, out_uv).rgba);
    if (out_color.a < 0.2) {
        discard;
    }
    else {
        out_color.a = 1.0;
    }
}