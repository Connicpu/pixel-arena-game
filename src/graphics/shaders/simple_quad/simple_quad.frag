#version 450
#extension GL_ARB_separate_shader_objects : enable

uniform sampler2DArray tex;

in vec2 v_uv;
flat in int v_imagelayer;

out vec4 f_color;

void main() {
    f_color = texture(tex, vec3(v_uv, v_imagelayer));
}