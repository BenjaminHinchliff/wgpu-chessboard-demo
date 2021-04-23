#version 450

layout(location = 0) in vec2 v_texcoord;

layout(location = 0) out vec4 f_color;

const float SIZE = 8.0;
const vec3 foreground = vec3(0.6, 0.4, 0.2);
const vec3 background = vec3(0.94, 0.85, 0.71);

float toBoardCoord(in float coord) {
    return mod(coord * SIZE, 2.0);
}

void main() {
    vec3 val =
        toBoardCoord(v_texcoord.x) <= 1.0 ^^
        toBoardCoord(v_texcoord.y) <= 1.0
        ? background : foreground;
    f_color = vec4(val, 1.0);
}
