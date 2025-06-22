#version 150 core

in vec3 a_pos;
in vec2 a_tex_coord;
out vec2 v_tex_coord;

uniform locals {
	mat4 u_transform;
};

void main() {
    v_tex_coord = a_tex_coord;
    gl_Position = u_transform * vec4(a_pos, 1.0);
    gl_ClipDistance[0] = 1.0;
}
