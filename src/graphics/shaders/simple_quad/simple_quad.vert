#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform UniformBuffer {
    mat4 u_camera;
};

layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec2 a_uv;

layout(location = 2) in vec4 i_uvrect;
layout(location = 3) in vec3 i_transform0;
layout(location = 4) in vec3 i_transform1;
layout(location = 5) in float i_layer;
layout(location = 6) in int i_imagelayer;

layout(location = 0) out vec2 v_uv;
layout(location = 1) flat out int v_imagelayer;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    mat3 transform = mat3(
        i_transform0,
        i_transform1,
        vec3(0, 0, 1)
    );

    vec3 worldPos = transform * vec3(a_pos, 1.0);
    gl_Position = u_camera * vec4(worldPos.xy, i_layer, 1.0);
    v_uv = mix(i_uvrect.xy, i_uvrect.zw, a_uv);
    v_imagelayer = v_imagelayer;
}
