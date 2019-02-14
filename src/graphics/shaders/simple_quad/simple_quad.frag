#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 1) uniform sampler2DArray t_sampler;

layout(location = 0) in vec2 v_uv;
layout(location = 1) flat in int v_imagelayer;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = texture(t_sampler, vec3(v_uv, v_imagelayer));
}
