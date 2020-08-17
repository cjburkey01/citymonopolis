#version 330

in vec3 vert_col_frag;

layout (location = 0) out vec4 frag_color;

void main() {
    frag_color = vec4(vert_col_frag, 1.0);
}
