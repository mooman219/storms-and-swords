#version 330 core

in vec4 v_Color;
in vec2 UV;
in float draw_border;

out vec4 out_color;

void main() {
    if (draw_border < 0.5) {
        if (UV.x < 0.05 || UV.x > 0.95) {
            out_color = vec4(0, 0, 0, 1);
        }
        else if (UV.y < 0.05 || UV.y > 0.95) {
            out_color = vec4(0, 0, 0, 1);
        }
        else {
            out_color = vec4(v_Color.r, v_Color.g, v_Color.b, v_Color.a);//vec4(1.0, 0.5, 0.25, 1.0);
        }
    }
    else {
            out_color = vec4(v_Color.r, v_Color.g, v_Color.b, v_Color.a);//vec4(1.0, 0.5, 0.25, 1.0);
    }
}