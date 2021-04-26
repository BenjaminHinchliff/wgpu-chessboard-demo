#version 450

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_texcoord;

layout(location = 2) in vec4 a_piece_pos_0;
layout(location = 3) in vec4 a_piece_pos_1;
layout(location = 4) in vec4 a_piece_pos_2;
layout(location = 5) in vec4 a_piece_pos_3;
layout(location = 6) in uvec2 a_piece_type;


layout(location = 0) out vec2 v_texcoord;
layout(location = 1) out flat uvec2 v_piece_type;

void main() {
    mat4 model_matrix = mat4(
        a_piece_pos_0,
        a_piece_pos_1,
        a_piece_pos_2,
        a_piece_pos_3
    );
    v_texcoord = a_texcoord;
    v_piece_type = a_piece_type;
    gl_Position = model_matrix * vec4(a_position, 0.0, 1.0);
}
