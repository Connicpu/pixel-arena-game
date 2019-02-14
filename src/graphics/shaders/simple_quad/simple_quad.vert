#version 450

layout(set = 0, binding = 0) uniform UniformBuffer {
    mat4 u_camera;
};

in vec2 a_pos;
in vec2 a_uv;

in vec4 i_uvrect;
in vec3 i_transform0;
in vec3 i_transform1;
in float i_layer;
in int i_imagelayer;

out vec2 v_uv;
flat out int v_imagelayer;

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
