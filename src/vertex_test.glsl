#version 330

layout (location = 0) in vec3 vert_pos;
layout (location = 1) in vec3 vert_col;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 object;

out vec3 vert_col_frag;

void main() {
    gl_Position = projection * view * object * vec4(vert_pos, 1.0);

    vert_col_frag = vert_col;
}
