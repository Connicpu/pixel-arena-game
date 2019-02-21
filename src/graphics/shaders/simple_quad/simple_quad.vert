#version 400

uniform Camera {
    mat4 u_camera;
};

in vec2 a_pos;
in vec2 a_uv;

in vec4 i_uvrect;
in vec2 i_transform0;
in vec2 i_transform1;
in vec2 i_transform2;
in float i_layer;
in uint i_imagelayer;

out vec2 v_uv;
flat out int v_imagelayer;

void main() {
    mat3 transform = mat3(
        vec3(i_transform0, 0),
        vec3(i_transform1, 0),
        vec3(i_transform2, 1)
    );

    vec3 worldPos = transform * vec3(a_pos, 1);
    gl_Position = u_camera * vec4(worldPos.xy, i_layer, 1.0);
    v_uv = mix(i_uvrect.xy, i_uvrect.zw, a_uv);
    v_imagelayer = v_imagelayer;
}
