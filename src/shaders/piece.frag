#version 450

layout(location = 0) in vec2 v_texcoord;
layout(location = 1) in flat uvec2 v_piece_type;

layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

const vec2 PIECES_SIZE = vec2(6.0, 2.0);

void main() {
    vec2 piece_texcoord = v_texcoord / PIECES_SIZE;
    vec2 offset = v_piece_type / PIECES_SIZE;
    f_color = texture(sampler2D(t_diffuse, s_diffuse), offset + piece_texcoord);
}
