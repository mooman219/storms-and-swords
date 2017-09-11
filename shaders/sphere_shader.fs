#version 150 core

in vec4 v_Color;
in vec2 out_uv;

out vec4 Target0;

void main() {
    
    float x_diff = out_uv.x - 0.5f;
    float y_diff = out_uv.y - 0.5f;
    float total = (x_diff * x_diff)  + (y_diff * y_diff);
    float final_height = sqrt(total);
    if(final_height <= 0.5) {
      Target0 = v_Color;
    }
    else {
        Target0 = vec4(0.0, 0.0, 0.0, 0.0);
    }
}