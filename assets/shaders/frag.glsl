#version 150 core

in vec2 v_tex_coord;
out vec4 main_window_target;
uniform sampler2D tex_sampler;

void main() {
    main_window_target = texture(tex_sampler, v_tex_coord);
}
